use axum::{extract::Extension, extract::Path, Json};
use base64::Engine;
use chrono::{Duration, Utc};
use deadpool_postgres::Pool;
use serde::Serialize;
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;

use crate::password::des3_decrypt_ede2;

// ──────────────────────────────────────────────────────────────────────────────
// andfx — 移动办公 MOA SSO（Android/iOS）
//
// token 格式：base64url(3DES_EDE2_encrypt(credential#timestamp_millis))
// 有效期：5 分钟（与 SSO 保持一致）
//
// 端点：
//   GET /jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}
// ──────────────────────────────────────────────────────────────────────────────

const ANDFX_TOKEN_TTL_MINUTES: i64 = 5;

#[derive(Debug, Serialize)]
pub struct AndfxLoginResponse {
    pub token: String,
    pub person: AndfxPersonInfo,
}

#[derive(Debug, Serialize)]
pub struct AndfxPersonInfo {
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub icon: Option<String>,
}

/// GET /jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}
pub async fn andfx_moa_sso(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path((token, enter_id)): Path<(String, String)>,
) -> Result<Json<ActionResult<AndfxLoginResponse>>, AppError> {
    if token.is_empty() || enter_id.is_empty() {
        return Ok(Json(ActionResult::error("token and enterId are required")));
    }

    let key = std::env::var("ANDFX_KEY").map_err(|_| AppError::Internal)?;
    let expected_enter_id = std::env::var("ANDFX_ENTER_ID").map_err(|_| AppError::Internal)?;

    if enter_id != expected_enter_id {
        return Ok(Json(ActionResult::error("invalid enterId")));
    }

    let decrypted = decrypt_andfx_token(&token, &key)?;
    let (credential, timestamp_str) = parse_andfx_payload(&decrypted)?;
    validate_andfx_timestamp(&timestamp_str)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let unique_id = format!("andfx_{credential}");

    let row = client
        .query_opt(
            "SELECT unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let (person_unique, person_name, person_mobile, person_email, person_icon) = match row {
        Some(r) => (
            r.get::<_, String>("unique_id"),
            r.get::<_, String>("name"),
            r.get::<_, Option<String>>("mobile"),
            r.get::<_, Option<String>>("email"),
            r.get::<_, Option<String>>("icon"),
        ),
        None => {
            return Ok(Json(ActionResult::error("user not found")));
        }
    };

    let token_val = uuid::Uuid::new_v4().to_string();
    let session = session_manager
        .create_session(person_unique.clone(), token_val.clone())
        .await;

    Ok(Json(ActionResult::success(AndfxLoginResponse {
        token: session.token,
        person: AndfxPersonInfo {
            unique: person_unique,
            name: person_name,
            mobile: person_mobile,
            email: person_email,
            icon: person_icon,
        },
    })))
}

// ──────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ──────────────────────────────────────────────────────────────────────────────

fn decrypt_andfx_token(token: &str, key: &str) -> Result<Vec<u8>, AppError> {
    let decoded = base64::engine::general_purpose::URL_SAFE
        .decode(token)
        .map_err(|_| AppError::BadRequest("invalid token encoding".to_string()))?;
    des3_decrypt_ede2(&decoded, key)
        .map_err(|_| AppError::BadRequest("decryption failed".to_string()))
}

fn parse_andfx_payload(decrypted: &[u8]) -> Result<(String, String), AppError> {
    let payload = String::from_utf8(decrypted.to_vec())
        .map_err(|_| AppError::BadRequest("invalid token content".to_string()))?;
    let mut parts = payload.rsplitn(2, '#');
    let timestamp_str = parts
        .next()
        .ok_or_else(|| AppError::BadRequest("invalid token format".to_string()))?;
    let credential = parts
        .next()
        .ok_or_else(|| AppError::BadRequest("invalid token format".to_string()))?;
    Ok((credential.to_string(), timestamp_str.to_string()))
}

fn validate_andfx_timestamp(timestamp_str: &str) -> Result<(), AppError> {
    let timestamp: i64 = timestamp_str
        .parse()
        .map_err(|_| AppError::BadRequest("invalid timestamp".to_string()))?;
    let now_ms = Utc::now().timestamp_millis();
    if now_ms - timestamp > Duration::minutes(ANDFX_TOKEN_TTL_MINUTES).num_milliseconds() {
        return Err(AppError::BadRequest("token expired".to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_andfx_timestamp_recent_passes() {
        let now_ms = Utc::now().timestamp_millis();
        assert!(validate_andfx_timestamp(&now_ms.to_string()).is_ok());
    }

    #[test]
    fn validate_andfx_timestamp_expired_fails() {
        let expired_ms = Utc::now().timestamp_millis() - Duration::minutes(10).num_milliseconds();
        assert!(validate_andfx_timestamp(&expired_ms.to_string()).is_err());
    }

    #[test]
    fn validate_andfx_timestamp_invalid_format_fails() {
        assert!(validate_andfx_timestamp("not-a-timestamp").is_err());
    }
}
