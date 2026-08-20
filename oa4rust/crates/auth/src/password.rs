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
// Java SSO uses 3DES with a 16-byte (EDE2) or 24-byte (EDE3) key.
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
