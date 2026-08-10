use axum::{
    extract::Extension,
    http::HeaderMap,
    Json,
};
use serde_json::Value;
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;

/// POST /jaxrs/authentication/safe/logout —— 安全注销
///
/// 使当前用户所有 session 全部过期（批量注销）。
/// 从 Authorization header 提取 token，验证后批量移除该用户所有会话。
pub async fn safe_logout(
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(_payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;

    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    session_manager
        .remove_sessions_by_person(&session.person_unique)
        .await;

    Ok(Json(ActionResult::success(serde_json::json!({
        "message": "all sessions invalidated"
    }))))
}
