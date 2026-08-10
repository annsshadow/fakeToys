use axum::extract::Extension;
use axum::Json;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;

#[derive(Debug, Deserialize)]
pub struct CheckTokenRequest {
    pub token: String,
}

/// POST /jaxrs/authentication/check/token —— Token 校验
///
/// 允许外部系统验证 OA token 有效性。
/// 权限级别：Authenticated（需携带有效 token 才能校验，防止会话枚举）。
pub async fn check_token(
    session_manager: Extension<SessionManager>,
    Json(req): Json<CheckTokenRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = req.token.trim().to_string();
    if token.is_empty() {
        return Ok(Json(ActionResult::error("token is required")));
    }

    match session_manager.validate_session(&token).await {
        Some(_session) => Ok(Json(ActionResult::success(serde_json::json!({
            "authenticated": true,
        })))),
        None => Ok(Json(ActionResult::error("invalid token"))),
    }
}
