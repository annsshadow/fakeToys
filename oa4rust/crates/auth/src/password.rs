use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use cipher::{Block, BlockEncrypt, KeyInit};
use des::Des;

/// bcrypt 哈希存储前缀，用于区分 MD5/DES 旧格式
pub const BCRYPT_PREFIX: &str = "{bcrypt}";

/// 按双算法兼容方案生成密码哈希（新写入统一 bcrypt，兼容既有 MD5/DES 校验）
pub fn hash_password(plain: &str) -> String {
    let cost = bcrypt::DEFAULT_COST;
    match bcrypt::hash(plain, cost) {
        Ok(hash) => format!("{BCRYPT_PREFIX}{hash}"),
        Err(_) => format!("{:x}", md5::compute(plain.as_bytes())),
    }
}

pub fn verify_password(plain: &str, stored: &str, key: &str, _encrypt_type: Option<&str>) -> bool {
    if let Some(bcrypt_hash) = stored.strip_prefix(BCRYPT_PREFIX) {
        return bcrypt::verify(plain, bcrypt_hash).unwrap_or(false);
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
