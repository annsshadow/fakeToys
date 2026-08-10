use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;

use crate::{code_store, PersonInfo};
pub use crate::TwoFactorLoginResponse;

// ──────────────────────────────────────────────────────────────────────────────
// two_factor — 双因素登录（短信验证码第二因子）
//
// 流程：POST /jaxrs/authentication/two_factor
//   1. 验证第一因子：credential + password（复用 login 逻辑）
//   2. 验证第二因子：短信验证码（复用 CodeStore）
//   3. 签发会话 token
//
// 安全：第一因子失败时返回相同错误消息，不暴露是否验证码正确（防枚举）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TwoFactorLoginRequest {
    pub credential: String,
    pub password: String,
    pub code: String,
}

/// POST /jaxrs/authentication/two_factor —— 双因素登录
///
/// 第一因子：credential + password（复用 login handler 的密码验证逻辑）
/// 第二因子：短信验证码（复用 CodeStore）
/// 成功后签发 2 小时有效会话 token。
pub async fn two_factor_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Json(req): Json<TwoFactorLoginRequest>,
) -> Result<Json<ActionResult<TwoFactorLoginResponse>>, AppError> {
    if req.credential.is_empty() || req.password.is_empty() || req.code.is_empty() {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    // 第一因子验证：查询用户并验证密码
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, password_hash FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");
    let person_mobile: Option<String> = row.get("mobile");
    let password_hash: String = row.get("password_hash");

    // 验证密码
    let valid = crate::password::verify_password(&req.password, &password_hash, "", None);
    if !valid {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    // 第二因子验证：短信验证码
    let code_valid = code_store().verify(&req.credential, &req.code);
    if !code_valid {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    // 签发会话
    let token = uuid::Uuid::new_v4().to_string();
    let session = session_manager
        .create_session(person_unique.clone(), token.clone())
        .await;

    Ok(Json(ActionResult::success(TwoFactorLoginResponse {
        token: session.token,
        person: PersonInfo {
            unique: person_unique,
            name: person_name,
            mobile: person_mobile,
        },
    })))
}
