use axum::extract::Extension;
use axum::http::HeaderMap;
use axum::Json;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;

#[derive(Debug, Deserialize)]
pub struct CheckTokenRequest {
    pub token: String,
}

/// POST /jaxrs/authentication/check/token —— Token 校验
///
/// 允许外部系统验证 OA token 有效性。
/// 权限级别：Admin（需管理员权限才能校验，防止会话枚举）。
/// 返回 token 持有者的 distinguishedName 字符串。
pub async fn check_token(
    pool: Extension<deadpool_postgres::Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(req): Json<CheckTokenRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 验证当前用户是 admin
    let header_token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let _session = session_manager
        .validate_session(&header_token)
        .await
        .ok_or(AppError::Unauthorized)?;

    if !shared::middleware::is_admin(&pool, &_session.person_unique).await {
        return Ok(Json(ActionResult::error("forbidden")));
    }

    let token = req.token.trim().to_string();
    if token.is_empty() {
        return Ok(Json(ActionResult::error("token is required")));
    }

    match session_manager.validate_session(&token).await {
        Some(session) => {
            // 查询用户 distinguishedName（这里使用 person_unique 作为 distinguishedName）
            let distinguished_name = session.person_unique.clone();
            Ok(Json(ActionResult::success(serde_json::json!({
                "distinguishedName": distinguished_name,
            }))))
        }
        None => Ok(Json(ActionResult::error("invalid token"))),
    }
}