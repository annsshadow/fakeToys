// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use cipher::{Block, BlockDecryptMut, BlockEncrypt, KeyInit};
use des::{Des, TdesEde2};

/// bcrypt 哈希存储前缀，用于区分 MD5/DES 旧格式
pub const BCRYPT_PREFIX: &str = "{bcrypt}";

/// bcrypt 工作因子（cost），10 在安全性与性能间取得平衡
pub const BCRYPT_COST: u32 = 10;

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
    match bcrypt::hash(plain, BCRYPT_COST) {
        Ok(hash) => format!("{BCRYPT_PREFIX}{hash}"),
        Err(_) => format!("{:x}", md5::compute(plain.as_bytes())),
    }
}

pub fn verify_password(plain: &str, stored: &str, key: &str, _encrypt_type: Option<&str>) -> bool {
    if let Some(bcrypt_hash) = stored.strip_prefix(BCRYPT_PREFIX) {
        if let Some(cached) = get_cached(bcrypt_hash, plain) {
            return cached;
        }
        let result = bcrypt::verify(plain, bcrypt_hash).unwrap_or(false);
        set_cached(bcrypt_hash, plain, result);
        return result;
    }

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

/// 3DES EDE2 加密（16 字节 key）
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
        assert!(verify_password("s3cret!", &h, "", None));
        // bcrypt 分支不读 key（key 只用于遗留 DES 路径）。
        assert!(verify_password("s3cret!", &h, "ignored-key", None));
        // 错误明文不得通过。
        assert!(!verify_password("nope", &h, "", None));
    }

    #[test]
    fn verify_password_accepts_legacy_md5_hex() {
        // 存量数据是 MD5 十六进制：必须继续可登录（双算法兼容是登录可用性底线）。
        let legacy = format!("{:x}", md5::compute(b"oldpass"));
        assert!(verify_password("oldpass", &legacy, "k", None));
        assert!(!verify_password("newpass", &legacy, "k", None));
    }

    #[test]
    fn verify_password_accepts_legacy_des_urlencoded_base64() {
        // 存量 DES 格式：base64url(url-encoded) 比对，必须精确复现 o2server 算法。
        let key = "secreckey"; // >= 8 字节
        let binding = URL_SAFE.encode(des_encrypt("despass", key).unwrap());
        let stored = urlencoding::encode(&binding);
        assert!(verify_password("despass", &stored, key, None));
        assert!(!verify_password("despass", &stored, "anotherkey8", None));
        // 空 key 走不到 DES 分支：仅 MD5 匹配可过，否则 false。
        assert!(!verify_password("despass", &stored, "", None));
    }

    #[test]
    fn verify_password_unknown_formats_reject() {
        assert!(!verify_password("x", "not-a-real-hash", "", None));
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
        assert!(verify_password("legacy", &upgraded, "", None));
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
}
