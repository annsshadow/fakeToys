// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use cipher::{Block, BlockDecryptMut, BlockEncrypt, KeyInit};
use des::{Des, TdesEde2};
use sha2::Digest;
use aes_gcm::aead::{Aead, OsRng, rand_core::RngCore};
use aes_gcm::{Aes256Gcm, Key, Nonce};

/// bcrypt 哈希存储前缀，用于区分 MD5/DES 旧格式
pub const BCRYPT_PREFIX: &str = "{bcrypt}";

/// bcrypt 工作因子（cost），10 在安全性与性能间取得平衡
pub const BCRYPT_COST: u32 = 10;

/// AES-GCM-256 SSO 密钥标识前缀，用于从 DB 查表选择密钥。
/// o2server 3DES-ECB 仅作历史解密兼容，新加密一律走 AES-GCM。
pub const AES_GCM_PREFIX: &str = "{aesgcm256}";

/// 密码验证结果短期缓存（避免对同一 (hash, plain) 重复做 bcrypt）
type VerifyCache = std::collections::HashMap<String, bool>;
static VERIFY_CACHE: std::sync::OnceLock<std::sync::Mutex<VerifyCache>> =
    std::sync::OnceLock::new();

fn cache_key(bcrypt_hash: &str, plain: &str) -> String {
    format!("{}:{}", bcrypt_hash, plain)
}

fn get_cached(bcrypt_hash: &str, plain: &str) -> Option<bool> {
    let cache = VERIFY_CACHE.get_or_init(|| std::sync::Mutex::new(VerifyCache::new()));
    cache
        .lock()
        .ok()
        .and_then(|c| c.get(&cache_key(bcrypt_hash, plain)).copied())
}

fn set_cached(bcrypt_hash: &str, plain: &str, result: bool) {
    if let Some(cache) = VERIFY_CACHE.get() {
        if let Ok(mut c) = cache.lock() {
            c.insert(cache_key(bcrypt_hash, plain), result);
        }
    }
}

/// 按双算法兼容方案生成密码哈希（新写入统一 bcrypt，兼容既有 MD5/DES 校验）
pub fn hash_password(plain: &str) -> String {
    // bcrypt::hash 仅在 cost 越界时失败；BCRYPT_COST 为常量 10 恒合法，此路实为不可达。
    // 失败时 fail-closed panic，杜绝静默降级到可彩虹表破解的无盐 MD5（旧实现的安全隐患）。
    let hash = bcrypt::hash(plain, BCRYPT_COST)
        .expect("bcrypt hashing failed with a constant valid cost; refusing to downgrade to MD5");
    format!("{BCRYPT_PREFIX}{hash}")
}

/// 遗留 MD5/DES 密码格式兼容开关。
///
/// o2server 迁移历史沉淀中部分存量账号密码以无盐 MD5 或 DES 加密存储。为支持一次性
/// 数据迁移，这些旧格式在登录后通过 `rehash_password` 自动升级为 bcrypt。在不开启
/// 兼容开关时，所有非 bcrypt 格式均被拒绝（fail-closed），避免彩虹表/离线暴力破解。
///
/// 由调用方（登录/SSO handler 与 `main.rs` 启动留痕）读取 `AUTH_LEGACY_COMPAT` 一次，
/// 并以 `legacy_compat` 布尔参数显式传入 `verify_password` / `sso_decrypt_payload`；
/// 测试可直接传布尔值，无全局可变状态、无并发测试竞争。
pub fn legacy_compat_from_env() -> bool {
    std::env::var("AUTH_LEGACY_COMPAT").as_deref().unwrap_or("") == "true"
}

pub fn verify_password(plain: &str, stored: &str, key: &str, legacy_compat: bool, _encrypt_type: Option<&str>) -> bool {
    if let Some(bcrypt_hash) = stored.strip_prefix(BCRYPT_PREFIX) {
        if let Some(cached) = get_cached(bcrypt_hash, plain) {
            return cached;
        }
        let result = bcrypt::verify(plain, bcrypt_hash).unwrap_or(false);
        set_cached(bcrypt_hash, plain, result);
        return result;
    }

    // 非 bcrypt 哈希：只有显式开启 legacy_compat 时才接受 MD5/DES 旧格式。
    // 默认关闭即 fail-closed，杜绝无盐 MD5 离线破解风险。
    if !legacy_compat {
        return false;
    }

    // 遗留 MD5：无盐，不抗彩虹表，仅作一次性迁移期临时接受。
    let md5_hash = format!("{:x}", md5::compute(plain.as_bytes()));
    if md5_hash == stored {
        return true;
    }

    if key.is_empty() {
        return false;
    }

    if let Ok(encrypted) = des_encrypt(plain, key) {
        let base64 = URL_SAFE.encode(encrypted);
        let url_encoded = urlencoding::encode(&base64);
        if url_encoded == stored {
            return true;
        }
    }

    false
}

/// 检查密码哈希是否需要升级（MD5/DES -> bcrypt）
pub fn needs_rehash(stored: &str) -> bool {
    !stored.starts_with(BCRYPT_PREFIX)
}

/// 将密码重新哈希为 bcrypt（用于登录成功后自动升级）
pub fn rehash_password(plain: &str) -> String {
    hash_password(plain)
}

pub(crate) fn des_encrypt(plain: &str, key: &str) -> Result<Vec<u8>, ()> {
    if key.len() < 8 {
        return Err(());
    }

    let key_bytes = &key.as_bytes()[..8];
    let des = Des::new_from_slice(key_bytes).map_err(|_| ())?;

    let data = plain.as_bytes();
    let mut padded = data.to_vec();
    while padded.len() % 8 != 0 {
        padded.push(0);
    }

    let mut encrypted = Vec::with_capacity(padded.len());
    for chunk in padded.chunks_exact(8) {
        let mut block = Block::<Des>::default();
        block.copy_from_slice(chunk);
        des.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    Ok(encrypted)
}

// ──────────────────────────────────────────────────────────────────────────────
// 3DES EDE (Encrypt-Decrypt-Encrypt) for SSO token encryption
//
// o2server SSO uses 3DES with a 16-byte (EDE2) or 24-byte (EDE3) key.
// Token format: base64(3DES_encrypt(credential#timestamp))
// ──────────────────────────────────────────────────────────────────────────────

/// 3DES EDE2 加密（16 字节 key）。
/// 安全审计（2026-10）：生产 SSO 加密已迁移至 AES-GCM-256；此函数仅用于
/// 测试生成历史 3DES token（回归 `des3_decrypt_ede2` 回退路径），非生产路径。
#[cfg(test)]
pub(crate) fn des3_encrypt_ede2(plain: &str, key: &str) -> Result<Vec<u8>, ()> {
    if key.len() < 16 {
        return Err(());
    }
    let key_bytes = &key.as_bytes()[..16];
    let cipher = TdesEde2::new_from_slice(key_bytes).map_err(|_| ())?;

    let data = plain.as_bytes();
    let mut padded = data.to_vec();
    while padded.len() % 8 != 0 {
        padded.push(0);
    }

    let mut encrypted = Vec::with_capacity(padded.len());
    for chunk in padded.chunks_exact(8) {
        let mut block = Block::<TdesEde2>::default();
        block.copy_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    Ok(encrypted)
}

/// 3DES EDE2 解密（16 字节 key）
pub(crate) fn des3_decrypt_ede2(encrypted: &[u8], key: &str) -> Result<Vec<u8>, ()> {
    if key.len() < 16 {
        return Err(());
    }
    let key_bytes = &key.as_bytes()[..16];
    let mut cipher = TdesEde2::new_from_slice(key_bytes).map_err(|_| ())?;

    let mut decrypted = Vec::with_capacity(encrypted.len());
    for chunk in encrypted.chunks_exact(8) {
        let mut block = Block::<TdesEde2>::default();
        block.copy_from_slice(chunk);
        cipher.decrypt_block_mut(&mut block);
        decrypted.extend_from_slice(&block);
    }

    // 去除尾部 0 填充
    let len = decrypted.len();
    let trimmed = &decrypted[..len - decrypted.iter().rev().take_while(|&&b| b == 0).count()];
    Ok(trimmed.to_vec())
}

// ──────────────────────────────────────────────────────────────────────────────
// AES-GCM-256 SSO token encryption (replaces DES/3DES-ECB for new tokens)
//
// o2server 3DES EDE2 (ECB, zero-pad) 仅作历史解密兼容，安全审计（2026-10）：
// 新生成的 SSO/andes token 一律走 AES-GCM-256，附带随机 nonce + 认证标签，
// 避免 ECB 模式的模式泄露与 3DES 56 位密钥暴力破解风险。
//
// 令牌格式：base64url(nonce(12) || ciphertext+tag)
// ──────────────────────────────────────────────────────────────────────────────

/// 从配置 key 派生 AES-256 密钥。配置 key 不足 32 字节时，用 SHA-256 拉伸到 32 字节，
/// 保证密钥长度固定且确定（只要 key 值相同即得相同密钥）。
fn aes256_key_from_config(key: &str) -> [u8; 32] {
    if key.len() >= 32 {
        let mut k = [0u8; 32];
        k.copy_from_slice(&key.as_bytes()[..32]);
        k
    } else {
        let h = sha2::Sha256::digest(key.as_bytes());
        let mut k = [0u8; 32];
        k.copy_from_slice(&h);
        k
    }
}

/// 新格式 SSO token 的版本前缀（解码后 raw 首字节序列）。
/// 带前缀 → 一律按 AES-GCM 处理；无前缀 → 历史 3DES-ECB（仅 legacy_compat 开启时接受）。
/// 前缀作为显式格式判别，不依赖密文尾字节等不可靠启发式。
pub const SSO_TOKEN_V2_PREFIX: &[u8] = b"v2.";

/// AES-GCM-256 加密（新 SSO token）。
/// 返回 base64url("v2." || nonce(12) || ciphertext+tag)。
pub fn sso_aes_encrypt(plain: &str, key: &str) -> Result<String, ()> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&aes256_key_from_config(key)));
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plain.as_bytes()).map_err(|_| ())?;
    let mut out = Vec::with_capacity(3 + 12 + ciphertext.len());
    out.extend_from_slice(SSO_TOKEN_V2_PREFIX);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(URL_SAFE.encode(&out))
}

/// AES-GCM-256 解密（新 SSO token，须带 "v2." 前缀）。
pub fn sso_aes_decrypt(token: &str, key: &str) -> Result<Vec<u8>, ()> {
    let raw = URL_SAFE.decode(token).map_err(|_| ())?;
    let body = raw.strip_prefix(SSO_TOKEN_V2_PREFIX).ok_or(())?;
    if body.len() < 12 + 16 {
        return Err(());
    }
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&aes256_key_from_config(key)));
    let nonce = Nonce::from_slice(&body[..12]);
    let plaintext = cipher.decrypt(nonce, &body[12..]).map_err(|_| ())?;
    Ok(plaintext)
}

/// 统一 SSO token 解码（格式自描述）：
/// - raw 以 "v2." 前缀开头 → AES-GCM-256（现代，带认证标签，防篡改）。
/// - 无前缀 → 历史 3DES-ECB，仅当 `legacy_compat=true` 时接受（默认关闭）。
///
/// 安全审计（2026-10）：前缀显式区分两种格式，避免 AES-GCM/3DES 之间靠尾字节
/// 等启发式判断造成的误判与「以 3DES 旧 token 冒充」的攻击面。
pub fn sso_decrypt_payload(token: &str, key: &str, legacy_compat: bool) -> Result<Vec<u8>, ()> {
    let raw = URL_SAFE.decode(token).map_err(|_| ())?;
    if raw.starts_with(SSO_TOKEN_V2_PREFIX) {
        return sso_aes_decrypt(token, key);
    }
    if !legacy_compat {
        return Err(());
    }
    des3_decrypt_ede2(&raw, key)
}

/// 统一 SSO token 加密（生产路径）：仅使用 AES-GCM。
pub fn sso_encrypt_payload(plain: &str, key: &str) -> Result<String, ()> {
    sso_aes_encrypt(plain, key)
}

// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password_produces_bcrypt_prefixed_hash() {
        let h = hash_password("hunter2");
        assert!(h.starts_with(BCRYPT_PREFIX));
        // bcrypt 输出形如 $2b$10$...，总长 60，前缀 8 → 68。
        assert_eq!(h.len(), BCRYPT_PREFIX.len() + 60);
    }

    #[test]
    fn verify_password_bcrypt_roundtrip() {
        let h = hash_password("s3cret!");
        assert!(verify_password("s3cret!", &h, "", false, None));
        // bcrypt 分支不读 key（key 只用于遗留 DES 路径）。
        assert!(verify_password("s3cret!", &h, "ignored-key", false, None));
        // 错误明文不得通过。
        assert!(!verify_password("nope", &h, "", false, None));
    }

    #[test]
    fn verify_password_rejects_legacy_md5_when_compat_disabled() {
        // 安全审计（2026-10）：legacy_compat=false 时，无盐 MD5 必须被拒绝（fail-closed）。
        let legacy = format!("{:x}", md5::compute(b"oldpass"));
        assert!(!verify_password("oldpass", &legacy, "k", false, None));
    }

    #[test]
    fn verify_password_accepts_legacy_md5_when_compat_enabled() {
        // 一次性迁移期显式开启兼容开关（legacy_compat=true）后，旧 MD5 格式才被接受。
        let legacy = format!("{:x}", md5::compute(b"oldpass"));
        assert!(verify_password("oldpass", &legacy, "k", true, None));
        assert!(!verify_password("newpass", &legacy, "k", true, None));
    }

    #[test]
    fn verify_password_accepts_legacy_des_when_compat_enabled() {
        // 一次性迁移期显式开启兼容开关（legacy_compat=true）后，旧 DES 格式才被接受。
        let key = "secreckey"; // >= 8 字节
        let binding = URL_SAFE.encode(des_encrypt("despass", key).unwrap());
        let stored = urlencoding::encode(&binding);
        assert!(verify_password("despass", &stored, key, true, None));
        assert!(!verify_password("despass", &stored, "anotherkey8", true, None));
        // 空 key 走不到 DES 分支；关闭兼容开关时旧格式一律拒绝。
        assert!(!verify_password("despass", &stored, "", true, None));
        assert!(!verify_password("despass", &stored, key, false, None));
    }

    #[test]
    fn verify_password_unknown_formats_reject() {
        assert!(!verify_password("x", "not-a-real-hash", "", false, None));
        // 即便开启兼容开关，未知格式仍拒绝（非 MD5/DES 形态）。
        assert!(!verify_password("x", "not-a-real-hash", "", true, None));
    }

    #[test]
    fn needs_rehash_flags_non_bcrypt_formats() {
        assert!(!needs_rehash(&hash_password("p")));
        assert!(needs_rehash(&format!("{:x}", md5::compute(b"p"))));
        assert!(needs_rehash("legacy-des-value"));
    }

    #[test]
    fn rehash_password_upgrades_to_bcrypt() {
        let upgraded = rehash_password("legacy");
        assert!(!needs_rehash(&upgraded));
        assert!(verify_password("legacy", &upgraded, "", false, None));
    }

    #[test]
    fn des_encrypt_pads_to_block_boundary_and_rejects_short_keys() {
        assert_eq!(des_encrypt("abcdefgh", "12345678").unwrap().len(), 8);
        assert_eq!(des_encrypt("abcdefghi", "12345678").unwrap().len(), 16);
        assert!(des_encrypt("x", "short").is_err()); // < 8 字节 key
    }

    #[test]
    fn des3_ede2_roundtrip() {
        let key = "1234567890abcdef"; // 16 字节 EDE2
        let ct = des3_encrypt_ede2("token#1700000000", key).unwrap();
        assert_eq!(ct.len(), 16); // 16 字节明文 + 0 填充 = 2 个块
        let pt = des3_decrypt_ede2(&ct, key).unwrap();
        assert_eq!(pt, "token#1700000000".as_bytes());
        // 短 key 必须拒绝（o2server 3DES 密钥 ≥ 16 字节）。
        assert!(des3_encrypt_ede2("x", "short").is_err());
        assert!(des3_decrypt_ede2(&[0u8; 8], "short").is_err());
        // 密文非 8 字节倍数：尾部不足块被忽略（chunks_exact 语义，不 panic）。
        let ct2 = des3_encrypt_ede2("abcdefghi", key).unwrap(); // 9 字节 → 补 8 = 16 字节密文
        assert_eq!(ct2.len(), 16);
        let partial = des3_decrypt_ede2(&ct2[..12], key).unwrap();
        assert_eq!(partial.len(), 8); // 仅前一个完整块被解密
    }

    #[test]
    fn sso_aes_gcm_roundtrip_and_auth() {
        let key = "0123456789abcdef0123456789abcdef"; // 32 字节
        let token = sso_encrypt_payload("u123#1700000000", key).unwrap();
        // 新 token 带 "v2." 前缀，必顶层 AES-GCM 解码成功（与 legacy_compat 开关无关）。
        let pt = sso_decrypt_payload(&token, key, false).unwrap();
        assert_eq!(pt, b"u123#1700000000");
        let pt = sso_decrypt_payload(&token, key, true).unwrap();
        assert_eq!(pt, b"u123#1700000000");

        // 错误密钥：AES-GCM 认证标签校验失败 → Err（即使 legacy_compat 开启，
        // 带 "v2." 前缀的 token 也绝不会被 3DES 路径误接）。
        let wrong_key = "ffffffffffffffffffffffffffffffff";
        assert!(sso_decrypt_payload(&token, wrong_key, true).is_err());
        assert!(sso_decrypt_payload(&token, wrong_key, false).is_err());

        // AES-GCM 路径独立于 legacy 开关：关掉也能正常解密新 token。
        assert!(sso_aes_decrypt(&token, key).is_ok());
    }

    #[test]
    fn sso_legacy_3des_rejected_without_compat_flag() {
        // 旧 3DES-ECB token（无 "v2." 前缀）：仅在 legacy_compat=true 时才接受。
        let key = "0123456789abcdef";
        let legacy_ct = des3_encrypt_ede2("u123#1700000000", key).unwrap();
        let legacy_token = URL_SAFE.encode(&legacy_ct);

        assert!(
            !URL_SAFE.decode(&legacy_token).unwrap().starts_with(SSO_TOKEN_V2_PREFIX),
            "回归前提：旧 3DES token 必不带 v2. 前缀"
        );
        assert!(sso_decrypt_payload(&legacy_token, key, false).is_err(),
            "未开启兼容开关时旧 3DES token 必须拒绝");

        let pt = sso_decrypt_payload(&legacy_token, key, true).unwrap();
        assert_eq!(pt, b"u123#1700000000");
    }
}
