use axum::{
    extract::Extension,
    http::HeaderMap,
    Json,
};
use chrono::Utc;
use serde_json::Value;
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;

/// POST /jaxrs/authentication/safe/logout —— 安全注销
///
/// 使当前用户所有 session 全部过期（批量注销）。
/// 从 Authorization header 提取 token，验证后批量移除该用户所有会话。
/// 同时写入 TokenThreshold 实体记录当前时间戳，用于多实例场景下的 token 失效广播。
pub async fn safe_logout(
    pool: Extension<deadpool_postgres::Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(_payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;

    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    // 写入 TokenThreshold 记录当前时间戳
    let now = Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let _ = client
        .execute(
            "INSERT INTO auth_token_threshold (person_unique, threshold_time, created_at) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (person_unique) DO UPDATE SET threshold_time = $2, updated_at = $3",
            &[&session.person_unique, &now, &now],
        )
        .await;

    // 移除该用户所有 session
    session_manager
        .remove_sessions_by_person(&session.person_unique)
        .await;

    // 多实例广播：通知所有实例使早于阈值的 session 失效
    session_manager
        .broadcast_logout(&session.person_unique)
        .await;

    Ok(Json(ActionResult::success(serde_json::json!({
        "message": "all sessions invalidated"
    }))))
}
