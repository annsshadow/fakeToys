use axum::{
    extract::{Extension, Json},
    http::HeaderMap,
    Json as AxumJson,
};
use serde::Deserialize;
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;

/// POST /jaxrs/authentication/switchuser — 用户切换（管理员）
///
/// 管理员临时切换为其他用户身份操作。
/// 原管理员 session 保持有效，返回新 token 用作目标用户身份。
/// 权限：仅 admin 可调用。
#[derive(Debug, Deserialize)]
pub struct SwitchUserRequest {
    pub credential: String,
}

pub async fn switch_user(
    pool: Extension<deadpool_postgres::Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(req): Json<SwitchUserRequest>,
) -> Result< AxumJson<ActionResult<serde_json::Value>>, AppError> {
    // 验证当前用户是 admin
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager.validate_session(&token).await.ok_or(AppError::Unauthorized)?;

    if !shared::middleware::is_admin(&pool, &session.person_unique).await {
        return Ok(AxumJson(ActionResult::error("forbidden")));
    }

    // 查找目标用户
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT unique_id, name FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::BadRequest("user not found".to_string()))?;

    let target_unique: String = row.get("unique_id");
    let target_name: String = row.get("name");

    // 为目标用户签发新 session
    let new_token = uuid::Uuid::new_v4().to_string();
    let new_session = session_manager.create_session(target_unique.clone(), new_token.clone()).await;

    Ok(AxumJson(ActionResult::success(serde_json::json!({
        "token": new_session.token,
        "person": {
            "unique": target_unique,
            "name": target_name,
        },
    }))))
}
