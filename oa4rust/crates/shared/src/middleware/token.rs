use axum::body::Body;
use axum::extract::State;
use axum::http::{header, HeaderMap, Method, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::Pool;

use super::constants::*;
use super::security::{is_auth_exempt, path_matches, SecurityState};
use crate::error::AppError;

pub const SESSION_COOKIE_NAME: &str = "oa4rust_session";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authentication {
    Cookie(String),
    Bearer(String),
}

impl Authentication {
    pub fn token(&self) -> &str {
        match self {
            Self::Cookie(token) | Self::Bearer(token) => token,
        }
    }
}

/// Cookie 一旦存在就优先使用；空值或无效值也不回退 Bearer。
pub fn extract_authentication(headers: &HeaderMap) -> Option<Authentication> {
    if let Some(token) = session_cookie(headers) {
        return Some(Authentication::Cookie(token.to_string()));
    }
    let auth = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    let token = auth.strip_prefix("Bearer ")?.trim();
    (!token.is_empty()).then(|| Authentication::Bearer(token.to_string()))
}

pub fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    extract_authentication(headers).and_then(|authentication| {
        let token = authentication.token();
        (!token.is_empty()).then(|| token.to_string())
    })
}

fn session_cookie(headers: &HeaderMap) -> Option<&str> {
    for cookie in headers.get_all(header::COOKIE) {
        let Ok(cookie) = cookie.to_str() else { continue };
        for part in cookie.split(';') {
            let Some((name, value)) = part.trim().split_once('=') else { continue };
            if name.trim() == SESSION_COOKIE_NAME {
                return Some(value.trim().trim_matches('"'));
            }
        }
    }
    None
}

pub async fn csrf_middleware(
    State(state): State<SecurityState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if !matches!(*request.method(), Method::POST | Method::PUT | Method::PATCH | Method::DELETE) {
        return next.run(request).await;
    }
    if !matches!(extract_authentication(request.headers()), Some(Authentication::Cookie(_))) {
        return next.run(request).await;
    }
    let matches = request.headers().get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())
        .map(|origin| origin == state.session_manager.auth_config.public_origin)
        .unwrap_or(false);
    if !matches {
        return AppError::Forbidden.into_response();
    }
    next.run(request).await
}

/// 系统是否未初始化：查询失败时 fail-closed。
pub(crate) async fn system_uninitialized(pool: &Pool) -> bool {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return false,
    };
    match client.query(
        "SELECT 1 FROM auth_person WHERE locked = false AND deleted_at IS NULL LIMIT 1",
        &[],
    ).await {
        Ok(rows) => rows.is_empty(),
        Err(_) => false,
    }
}

pub async fn auth_middleware(
    State(state): State<SecurityState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();

    if is_auth_exempt(&path) {
        if !SECRET_INIT_PATHS.iter().any(|p| path_matches(&path, p))
            || system_uninitialized(&state.pool).await
        {
            return next.run(request).await;
        }
    }

    let Some(authentication) = extract_authentication(request.headers()) else {
        return AppError::Unauthorized.into_response();
    };
    let token = authentication.token();
    if token.is_empty() {
        return AppError::Unauthorized.into_response();
    }
    // Migration observability: count legacy Bearer credentials reaching a protected route.
    // Intentionally no token/cookie/User-Agent value is logged.
    if matches!(authentication, Authentication::Bearer(_)) {
        tracing::warn!(
            auth_compat = "bearer",
            method = %request.method().as_str(),
            path = %path,
            "bearer compatibility credential used on protected route"
        );
    }
    match state.session_manager.validate_session(token).await {
        Some(session) => {
            request.extensions_mut().insert(session);
            next.run(request).await
        }
        None => AppError::Unauthorized.into_response(),
    }
}
