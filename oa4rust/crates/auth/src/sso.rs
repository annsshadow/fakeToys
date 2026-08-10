use axum::{
    extract::Extension,
    Json,
};
use base64::Engine;
use chrono::{Duration, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;

use crate::password::{des3_decrypt_ede2, des3_encrypt_ede2};

// ──────────────────────────────────────────────────────────────────────────────
// sso — 单点登录（3DES EDE2 加密 token）
//
// SSO token 格式：base64url(3DES_EDE2_encrypt(credential#timestamp_millis))
// 有效期：5 分钟（防止重放攻击）
//
// 端点：
//   GET  /jaxrs/authentication/sso/client/{client}/token/{token}  — 解密 token 并登录
//   POST /jaxrs/authentication/sso                                 — 从请求体解密 token
//   POST /jaxrs/authentication/sso/encrypt                         — 加密辅助
// ──────────────────────────────────────────────────────────────────────────────

const SSO_TOKEN_TTL_MINUTES: i64 = 5;

#[derive(Debug, Deserialize)]
pub struct SsoLoginRequest {
    pub client: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct SsoEncryptRequest {
    pub client: String,
    pub key: String,
    pub credential: String,
}

#[derive(Debug, Serialize)]
pub struct SsoLoginResponse {
    pub token: String,
    pub person: SsoPersonInfo,
}

#[derive(Debug, Serialize)]
pub struct SsoPersonInfo {
    pub unique: String,
    pub name: String,
}

/// POST /jaxrs/authentication/sso
pub async fn sso_post_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Json(req): Json<SsoLoginRequest>,
) -> Result<Json<ActionResult<SsoLoginResponse>>, AppError> {
    if req.client.is_empty() || req.token.is_empty() {
        return Ok(Json(ActionResult::error("client and token are required")));
    }
    let decrypted = decrypt_sso_token(&req.token, &req.client)?;
    let (credential, timestamp_str) = parse_sso_payload(&decrypted)?;
    validate_sso_timestamp(&timestamp_str)?;
    create_sso_session(&pool, &session_manager, &credential).await
}

/// POST /jaxrs/authentication/sso/encrypt
pub async fn sso_encrypt(Json(req): Json<SsoEncryptRequest>) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.client.is_empty() || req.key.is_empty() || req.credential.is_empty() {
        return Ok(Json(ActionResult::error("client, key, and credential are required")));
    }
    let now = Utc::now();
    let payload = format!("{}#{}", req.credential, now.timestamp_millis());
    let encrypted = des3_encrypt_ede2(&payload, &req.key)
        .map_err(|_| AppError::BadRequest("invalid key length".to_string()))?;
    let token = base64::engine::general_purpose::URL_SAFE.encode(&encrypted);
    Ok(Json(ActionResult::success(serde_json::json!({
        "client": req.client,
        "token": token,
        "timestamp": now.timestamp_millis(),
    }))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ──────────────────────────────────────────────────────────────────────────────

fn decrypt_sso_token(token: &str, key: &str) -> Result<Vec<u8>, AppError> {
    let decoded = base64::engine::general_purpose::URL_SAFE
        .decode(token)
        .map_err(|_| AppError::BadRequest("invalid token encoding".to_string()))?;
    des3_decrypt_ede2(&decoded, key)
        .map_err(|_| AppError::BadRequest("decryption failed".to_string()))
}

fn parse_sso_payload(decrypted: &[u8]) -> Result<(String, String), AppError> {
    let payload = String::from_utf8(decrypted.to_vec())
        .map_err(|_| AppError::BadRequest("invalid token content".to_string()))?;
    let mut parts = payload.rsplitn(2, '#');
    let timestamp_str = parts.next().ok_or_else(|| AppError::BadRequest("invalid token format".to_string()))?;
    let credential = parts.next().ok_or_else(|| AppError::BadRequest("invalid token format".to_string()))?;
    Ok((credential.to_string(), timestamp_str.to_string()))
}

fn validate_sso_timestamp(timestamp_str: &str) -> Result<(), AppError> {
    let timestamp: i64 = timestamp_str
        .parse()
        .map_err(|_| AppError::BadRequest("invalid timestamp".to_string()))?;
    let now_ms = Utc::now().timestamp_millis();
    if now_ms - timestamp > Duration::minutes(SSO_TOKEN_TTL_MINUTES).num_milliseconds() {
        return Err(AppError::BadRequest("token expired".to_string()));
    }
    Ok(())
}

async fn create_sso_session(
    pool: &Pool,
    session_manager: &SessionManager,
    credential: &str,
) -> Result<Json<ActionResult<SsoLoginResponse>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT unique_id, name FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&credential],
        )
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");

    if person_unique.is_empty() {
        return Ok(Json(ActionResult::error("user not found")));
    }

    let token = uuid::Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await;

    Ok(Json(ActionResult::success(SsoLoginResponse {
        token: session.token,
        person: SsoPersonInfo {
            unique: person_unique,
            name: person_name,
        },
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sso_timestamp_recent_passes() {
        let now_ms = Utc::now().timestamp_millis();
        assert!(validate_sso_timestamp(&now_ms.to_string()).is_ok());
    }

    #[test]
    fn validate_sso_timestamp_expired_fails() {
        let expired_ms = Utc::now().timestamp_millis() - Duration::minutes(10).num_milliseconds();
        assert!(validate_sso_timestamp(&expired_ms.to_string()).is_err());
    }

    #[test]
    fn validate_sso_timestamp_invalid_format_fails() {
        assert!(validate_sso_timestamp("not-a-timestamp").is_err());
    }
}
