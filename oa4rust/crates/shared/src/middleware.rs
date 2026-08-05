use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tracing::warn;

use crate::error::AppError;
use crate::rate_limit::RateLimiter;
use crate::session::SessionManager;

// ──────────────────────────────────────────────────────────────────────────────
// 认证与速率限制豁免路径
//
// R12：健康检查端点及认证前置端点豁免认证，按精确路径匹配。
// 使用 `{param}` 段通配参数化端点（如验证码尺寸、凭证、OAuth code）。
// ──────────────────────────────────────────────────────────────────────────────
const AUTH_EXEMPT_PATHS: &[&str] = &[
    "/health",
    // 登录 / 登出 / 当前用户（契约路径 + 现有自造路径）
    "/jaxrs/authentication",
    "/jaxrs/authentication/login",
    "/jaxrs/authentication/logout",
    "/jaxrs/authentication/who",
    // 验证码
    "/jaxrs/authentication/captcha",
    "/jaxrs/authentication/captcha/width/{width}/height/{height}",
    "/jaxrs/authentication/code",
    "/jaxrs/authentication/code/credential/{credential}",
    // 绑定 / 扫码登录
    "/jaxrs/authentication/bind",
    "/jaxrs/authentication/bind/meta/{meta}",
    // OAuth 授权与回调
    "/jaxrs/authentication/oauth",
    "/jaxrs/authentication/oauth/list",
    "/jaxrs/authentication/oauth/qywx/config",
    "/jaxrs/authentication/oauth/dingding/config",
    "/jaxrs/authentication/oauth/name/{name}",
    "/jaxrs/authentication/oauth/login/qywx/code/{code}",
    "/jaxrs/authentication/oauth/login/dingding/code/{code}",
    "/jaxrs/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}",
    "/jaxrs/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}",
    // 会话刷新
    "/jaxrs/authentication/refresh",
    // 验证码校验
    "/jaxrs/secret/captcha/verify",
    // 密码重置流程
    "/jaxrs/reset/check/credential/{credential}",
    "/jaxrs/reset/check/password/{password}",
    "/jaxrs/reset/code/credential/{credential}",
    "/jaxrs/reset",
    "/jaxrs/reset/password/anonymous",
    // 系统初始化（仅系统未初始化时）
    "/jaxrs/secret/check",
    "/jaxrs/secret/set",
    "/jaxrs/secret/cancel",
];

// 认证类端点（计入 10 次/分钟/IP 的认证限流）：
// 认证接口 + 密码重置端点。按路径段前缀匹配。
const AUTH_RATE_LIMIT_PREFIXES: &[&str] = &["/jaxrs/authentication", "/jaxrs/reset", "/jaxrs/secret"];

const AUTH_RATE_LIMIT: i32 = 10;
const GENERAL_RATE_LIMIT: i32 = 100;
const RATE_LIMIT_WINDOW_MINUTES: i64 = 1;

/// 路径段匹配：`{param}` 段通配任意单个段，其余段精确匹配
fn path_matches(path: &str, pattern: &str) -> bool {
    let path_segs: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    let pattern_segs: Vec<&str> = pattern.trim_start_matches('/').split('/').collect();

    if path_segs.len() != pattern_segs.len() {
        return false;
    }

    path_segs
        .iter()
        .zip(pattern_segs.iter())
        .all(|(p, pat)| *pat == "{param}" || pat.starts_with('{') || p == pat)
}

fn is_auth_exempt(path: &str) -> bool {
    AUTH_EXEMPT_PATHS.iter().any(|pattern| path_matches(path, pattern))
}

fn is_auth_rate_limited(path: &str) -> bool {
    AUTH_RATE_LIMIT_PREFIXES.iter().any(|prefix| path.starts_with(prefix))
}

/// 从请求提取客户端 IP：优先 X-Forwarded-For（nginx 反代），回退 socket 地址
fn client_ip(request: &Request<Body>) -> String {
    if let Some(xff) = request.headers().get("x-forwarded-for") {
        if let Ok(xff) = xff.to_str() {
            if let Some(first) = xff.split(',').next() {
                let ip = first.trim();
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }
    request
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|info| info.ip().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

// ──────────────────────────────────────────────────────────────────────────────
// trace_middleware
//
// 请求追踪中间件，记录每个请求的 HTTP 方法和 URI，
// 并在服务端返回 5xx 错误时额外输出 warning 级别日志，便于生产环境排查。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn trace_middleware(mut request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    // 仅对服务端错误（5xx）打 warning 日志，避免正常请求污染日志
    if response.status().is_server_error() {
        warn!(?method, ?uri, status = response.status().as_u16(), "server error");
    }

    response
}

// ──────────────────────────────────────────────────────────────────────────────
// auth_middleware
//
// 认证中间件：为所有非豁免端点验证会话令牌（Authorization: Bearer <token>），
// 未认证返回 401。R12 从首个端点暴露起生效。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn auth_middleware(
    session_manager: Extension<SessionManager>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();

    if is_auth_exempt(&path) {
        return next.run(request).await;
    }

    let token = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    let authorized = match token {
        Some(token) => session_manager.validate_session(&token).await.is_some(),
        None => false,
    };

    if !authorized {
        return AppError::Unauthorized.into_response();
    }

    next.run(request).await
}

// ──────────────────────────────────────────────────────────────────────────────
// rate_limit_middleware
//
// 速率限制中间件：认证接口（认证 + 密码重置）10 次/分钟/IP，
// 普通接口 100 次/分钟/IP。超限返回 429。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn rate_limit_middleware(
    rate_limiter: Extension<RateLimiter>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();
    let ip = client_ip(&request);

    // 健康检查与豁免端点不限流
    if is_auth_exempt(&path) && !is_auth_rate_limited(&path) {
        return next.run(request).await;
    }

    let max_attempts = if is_auth_rate_limited(&path) {
        AUTH_RATE_LIMIT
    } else {
        GENERAL_RATE_LIMIT
    };

    if let Err(_) = rate_limiter
        .check_rate_limit(&ip, max_attempts, RATE_LIMIT_WINDOW_MINUTES)
        .await
    {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            axum::Json(serde_json::json!({
                "data": None::<serde_json::Value>,
                "type": "error",
                "message": "rate limit exceeded",
                "date": None::<Option<String>>,
                "spent": None::<Option<i64>>,
                "size": None::<Option<i64>>,
                "count": None::<Option<i64>>,
                "position": None::<Option<String>>,
                "prompt": None::<Option<String>>,
            })),
        )
            .into_response();
    }

    next.run(request).await
}

// ──────────────────────────────────────────────────────────────────────────────
// error_handler
//
// 全局错误处理回调，接收 Axum 层抛出的 AppError，
// 并通过 IntoResponse 将其转换为标准化的 HTTP 响应。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn error_handler(err: AppError, _request: Request<Body>) -> Response {
    err.into_response()
}
