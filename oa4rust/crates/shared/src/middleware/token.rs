use axum::body::Body;
use axum::extract::State;
use axum::http::{header, HeaderMap, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::Pool;

use super::constants::*;
use super::security::{is_auth_exempt, is_auth_rate_limited, path_matches, SecurityState};
use crate::error::AppError;
use crate::session::Session;

/// 从 HeaderMap 提取会话令牌：优先 Authorization: Bearer <token>，
/// 回退 Cookie 中的 `token` 字段。
pub fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    if let Some(auth) = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        if let Some(token) = auth.strip_prefix("Bearer ") {
            let token = token.trim();
            if !token.is_empty() {
                return Some(token.to_string());
            }
        }
    }

    let cookie = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in cookie.split(';') {
        if let Some(v) = part.trim().strip_prefix("token=") {
            let v = v.trim().trim_matches('"');
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// 从请求提取会话令牌：优先 Authorization: Bearer <token>，
/// 回退 Cookie 中的 `token` 字段。
pub(crate) fn extract_token(request: &Request<Body>) -> Option<String> {
    extract_token_from_headers(request.headers())
}

/// 系统是否未初始化：auth_person 中不存在任何未删除（deleted_at IS NULL）且
/// 未锁定（locked = false）的用户。查询失败时 fail-closed（按已初始化处理，
/// 要求认证），避免在系统状态未知时放开认证。
pub(crate) async fn system_uninitialized(pool: &Pool) -> bool {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return false,
    };
    match client
        .query(
            "SELECT 1 FROM auth_person WHERE locked = false AND deleted_at IS NULL LIMIT 1",
            &[],
        )
        .await
    {
        Ok(rows) => rows.is_empty(),
        Err(_) => false,
    }
}

/// 认证中间件：为所有非豁免端点验证会话令牌（Authorization: Bearer <token>
/// 或 Cookie `token`），并注入 Extension<Session>。未认证返回 401。
/// /jaxrs/secret/check|set 仅在系统未初始化时豁免。
pub async fn auth_middleware(
    State(state): State<SecurityState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {

    let path = request.uri().path().to_string();

    if is_auth_exempt(&path) {
        // 系统初始化端点仅在系统未初始化时豁免
        if !SECRET_INIT_PATHS.iter().any(|p| path_matches(&path, p))
            || system_uninitialized(&state.pool).await
        {
            return next.run(request).await;
        }
    }

    let Some(token) = extract_token(&request) else {
        return AppError::Unauthorized.into_response();
    };

    match state.session_manager.validate_session(&token).await {
        Some(session) => {
            request.extensions_mut().insert(session);
            next.run(request).await
        }
        None => AppError::Unauthorized.into_response(),
    }
}
