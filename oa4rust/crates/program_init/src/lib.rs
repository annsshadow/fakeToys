use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use base64::Engine;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::{error::AppError, response::ActionResult};

// ──────────────────────────────────────────────────────────────────────────────
// program_init — 系统初始化
//
// 初始化密钥（secret）以 AES-128-GCM 加密后持久化到 `secret_config` 表
// （migrations/007_secret_config.sql），替换原有内存状态。
//
// 加密密钥来源：
//   1. 环境变量 `SECRET_ENCRYPTION_KEY`（生产环境必须显式配置）
//   2. 未配置时回退开发默认值 "oa4rust-dev-secret-key-0123456789"
//      （仅限本地开发，禁止用于生产）
//
// "已初始化"判定（与 Java 侧一致）：auth_person 存在任意启用用户
// 或 secret_config 存在配置记录。
// ──────────────────────────────────────────────────────────────────────────────

/// 开发环境默认加密密钥（生产必须通过 SECRET_ENCRYPTION_KEY 显式配置）
pub const DEV_ENCRYPTION_KEY: &str = "oa4rust-dev-secret-key-0123456789";

/// secret_config 单行逻辑表的固定主键
const SECRET_ROW_ID: &str = "init-secret";

// --- 应用级 AES-GCM 加密 ---

#[derive(Debug, Clone)]
pub struct SecretCipher;

impl SecretCipher {
    /// 从环境变量读取加密密钥，经 md5 归一化为 16 字节（AES-128-GCM 密钥长度）。
    /// 轮换机制：更换 SECRET_ENCRYPTION_KEY 并重跑 POST /jaxrs/secret/set
    /// 即可用新密钥重写密文（密文格式 base64(nonce || ciphertext+tag)，含随机 nonce）。
    fn key() -> [u8; 16] {
        let raw = std::env::var("SECRET_ENCRYPTION_KEY")
            .unwrap_or_else(|_| DEV_ENCRYPTION_KEY.to_string());
        let digest = md5::compute(raw.as_bytes());
        let mut key = [0u8; 16];
        key.copy_from_slice(&digest.0);
        key
    }

    /// 加密明文，返回 `base64(nonce(12B) || ciphertext+tag)`
    pub fn encrypt(plain: &str) -> Result<String, AppError> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes128Gcm, Nonce,
        };

        let cipher = Aes128Gcm::new_from_slice(&Self::key()).map_err(|_| AppError::Internal)?;

        let uuid = uuid::Uuid::new_v4();
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes.copy_from_slice(&uuid.as_bytes()[..12]);

        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), plain.as_bytes())
            .map_err(|_| AppError::Internal)?;

        let mut out = nonce_bytes.to_vec();
        out.extend_from_slice(&ciphertext);
        Ok(base64::engine::general_purpose::STANDARD.encode(out))
    }

    /// 解密 `base64(nonce(12B) || ciphertext+tag)`
    pub fn decrypt(encoded: &str) -> Result<String, AppError> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes128Gcm, Nonce,
        };

        let raw = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|_| AppError::Internal)?;
        if raw.len() < 13 {
            return Err(AppError::Internal);
        }

        let (nonce_bytes, ciphertext) = raw.split_at(12);
        let cipher = Aes128Gcm::new_from_slice(&Self::key()).map_err(|_| AppError::Internal)?;
        let plain = cipher
            .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
            .map_err(|_| AppError::Internal)?;

        String::from_utf8(plain).map_err(|_| AppError::Internal)
    }
}

// --- 处理器 ---

#[derive(Debug, Deserialize)]
pub struct SetSecretRequest {
    pub secret: String,
}

/// GET /jaxrs/secret/check —— 返回系统初始化状态（从数据库读取）
pub async fn check(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_row = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM auth_person WHERE locked = false AND deleted_at IS NULL)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let has_person: bool = person_row.get("exists");

    let secret_set = client
        .query_opt("SELECT 1 FROM secret_config LIMIT 1", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let initialized = has_person || secret_set.is_some();

    Ok(Json(ActionResult::success(json!({
        "initialized": initialized,
        "hasPerson": has_person,
        "secretSet": secret_set.is_some(),
    }))))
}

/// POST /jaxrs/secret/set —— 设置初始化密钥并持久化到数据库（AES-GCM 加密存储）
pub async fn set(
    pool: Extension<Pool>,
    Json(req): Json<SetSecretRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.secret.trim().is_empty() {
        return Ok(Json(ActionResult::error("secret cannot be empty")));
    }

    let encrypted = SecretCipher::encrypt(req.secret.as_str())?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO secret_config (id, secret_encrypted, created_at, updated_at) \
             VALUES ($1, $2, NOW(), NOW()) \
             ON CONFLICT (id) DO UPDATE \
             SET secret_encrypted = EXCLUDED.secret_encrypted, updated_at = NOW()",
            &[&SECRET_ROW_ID, &encrypted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({ "set": true }))))
}

/// 清除已设置的初始化密钥（secret_config 记录删除）
pub async fn set_cancel(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM secret_config", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({ "canceled": true }))))
}

// --- 路由注册 ---

/// 构建系统初始化模块路由（main.rs 接线时传入共享数据库连接池）
pub fn program_init_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/secret/check", get(check))
        .route("/jaxrs/secret/set", post(set))
        .route("/jaxrs/secret/set/cancel", get(set_cancel))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests {
    use super::SecretCipher;
    use super::SetSecretRequest;

    #[test]
    fn test_secret_cipher_roundtrip() {
        let plain = "xadmin-initial-password";
        let encoded = SecretCipher::encrypt(plain).unwrap();
        // 密文为 base64 且非明文
        assert!(!encoded.contains(plain));
        assert_eq!(SecretCipher::decrypt(&encoded).unwrap(), plain);
    }

    #[test]
    fn test_secret_cipher_random_nonce() {
        // 相同明文两次加密得到不同密文（随机 nonce）
        let a = SecretCipher::encrypt("same").unwrap();
        let b = SecretCipher::encrypt("same").unwrap();
        assert_ne!(a, b);
        assert_eq!(SecretCipher::decrypt(&a).unwrap(), "same");
        assert_eq!(SecretCipher::decrypt(&b).unwrap(), "same");
    }

    #[test]
    fn test_secret_cipher_invalid_input() {
        assert!(SecretCipher::decrypt("not-base64!!").is_err());
    }

    #[test]
    fn test_set_secret_request_deserialize() {
        let req: SetSecretRequest = serde_json::from_str(r#"{"secret":"abc"}"#).unwrap();
        assert_eq!(req.secret, "abc");
    }
}