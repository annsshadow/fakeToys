use axum::body::Body;
use axum::extract::{ConnectInfo, State};
use axum::http::{header, HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::Pool;
use std::env;
use std::net::SocketAddr;
use std::sync::OnceLock;
use tower_http::cors::CorsLayer;
use tracing::warn;

use super::constants::*;
use crate::error::AppError;
use crate::rate_limit::RateLimiter;
use crate::response::error_response;
use crate::session::{Session, SessionManager};

// ──────────────────────────────────────────────────────────────────────────────
// CORS 中间件
//
// 允许 o2web 前端跨域访问，支持凭据（Authorization/Cookie）。
// 仅允许 GET/POST/HEAD/OPTIONS 方法，允许 Authorization/Content-Type 头。
// ──────────────────────────────────────────────────────────────────────────────
pub fn cors_middleware() -> CorsLayer {
    use tower_http::cors::AllowOrigin;

    let origins: Vec<String> = env::var("CORS_ALLOW_ORIGIN")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let allow_origin = if origins.is_empty() {
        AllowOrigin::exact(
            "http://localhost:3000"
                .parse::<HeaderValue>()
                .expect("default origin parse"),
        )
    } else {
        let header_values: Vec<HeaderValue> = origins
            .into_iter()
            .filter_map(|o| o.parse::<HeaderValue>().ok())
            .collect();
        if header_values.len() == 1 {
            AllowOrigin::exact(header_values.into_iter().next().unwrap())
        } else {
            AllowOrigin::list(header_values)
        }
    };

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::HEAD,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true)
}

// ──────────────────────────────────────────────────────────────────────────────
// SecurityState
//
// 认证 / 授权 / 限流中间件共享的运行时状态。由 main.rs 构造单一实例，
// 通过 from_fn_with_state 注入，避免认证、限流与角色检查状态分裂。
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct SecurityState {
    pub session_manager: SessionManager,
    pub rate_limiter: RateLimiter,
    pub pool: Pool,
}

/// 路径段匹配：`{param}` 段通配任意单个段，其余段精确匹配
pub(crate) fn path_matches(path: &str, pattern: &str) -> bool {
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

pub(crate) fn is_auth_exempt(path: &str) -> bool {
    AUTH_EXEMPT_PATHS.iter().any(|pattern| path_matches(path, pattern))
}

pub(crate) fn is_auth_rate_limited(path: &str) -> bool {
    AUTH_RATE_LIMIT_EXACT.iter().any(|p| path == *p)
        || AUTH_RATE_LIMIT_PREFIXES.iter().any(|prefix| path.starts_with(prefix))
}

/// 路径段前缀匹配：`path` 等于 `prefix` 或 `prefix + "/"` 开头。
/// 带段边界，避免把 /jaxrs/personal/* 误判为 /jaxrs/person*。
pub(crate) fn path_starts_with_segment(path: &str, prefix: &str) -> bool {
    path == prefix || path.starts_with(&format!("{}/", prefix))
}

/// 角色授权判定：person/unit/role/group 及扩展模块的写操作（POST/PUT/DELETE）
/// 需要 admin 角色。自服务端点（改密、头像）豁免。
pub(crate) fn requires_admin(method: &axum::http::Method, path: &str) -> bool {
    if matches!(method.as_str(), "POST" | "PUT" | "DELETE") {
        if path_starts_with_segment(path, "/jaxrs/person/password")
            || path_starts_with_segment(path, "/jaxrs/person/icon")
        {
            return false;
        }
        ADMIN_WRITE_PREFIXES
            .iter()
            .any(|prefix| path_starts_with_segment(path, prefix))
    } else {
        false
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 可信代理配置
//
// client_ip 的提取模型：仅信任来自可信代理白名单（环境变量 TRUSTED_PROXY_IPS，
// 逗号分隔的 IP 列表，默认空 = 仅信任本地回环 127.0.0.1 / ::1，即本地 nginx）
// 的 X-Forwarded-For 第一个 IP；其余情况回退 socket 地址（ConnectInfo）。
// ──────────────────────────────────────────────────────────────────────────────
fn trusted_proxy_ips() -> &'static Vec<String> {
    static TRUSTED: OnceLock<Vec<String>> = OnceLock::new();
    TRUSTED.get_or_init(|| {
        let raw = env::var("TRUSTED_PROXY_IPS").unwrap_or_default();
        let mut ips: Vec<String> = raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if ips.is_empty() {
            ips.push("127.0.0.1".to_string());
            ips.push("::1".to_string());
        }
        ips
    })
}

fn first_xff_ip(request: &Request<Body>) -> Option<String> {
    let xff = request.headers().get("x-forwarded-for")?.to_str().ok()?;
    xff.split(',').next().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// 从请求提取客户端 IP：可信代理来源的 X-Forwarded-For 第一个值，否则 socket 地址。
/// 无 ConnectInfo 时（如单元测试）回退 127.0.0.1。
pub(crate) fn client_ip(request: &Request<Body>) -> String {
    let socket_ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|info| info.ip().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    if trusted_proxy_ips().iter().any(|p| *p == socket_ip) {
        if let Some(ip) = first_xff_ip(request) {
            return ip;
        }
    }

    socket_ip
}

// ──────────────────────────────────────────────────────────────────────────────
// trace_middleware
//
// 请求追踪中间件，记录每个请求的 HTTP 方法和 URI，
// 并在服务端返回 5xx 错误时额外输出 warning 级别日志，便于生产环境排查。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn trace_middleware(request: Request<Body>, next: Next) -> Response {
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
// security_headers_middleware
//
// 应用层安全响应头：X-Content-Type-Options、X-Frame-Options、Cache-Control: no-store。
// FORCE_HTTPS=true 时执行 HTTP→HTTPS 跳转（依据 nginx 透传的 X-Forwarded-Proto）。
// HSTS 由 nginx 终止层下发，Rust 侧不设置，避免在回环直连场景误伤。
// ──────────────────────────────────────────────────────────────────────────────
fn force_https() -> bool {
    static FORCE: OnceLock<bool> = OnceLock::new();
    *FORCE.get_or_init(|| {
        env::var("FORCE_HTTPS")
            .map(|v| matches!(v.as_str(), "true" | "1"))
            .unwrap_or(false)
    })
}

fn https_redirect(request: &Request<Body>) -> Option<Response> {
    let proto = request
        .headers()
        .get("x-forwarded-proto")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("https");
    if proto.eq_ignore_ascii_case("https") {
        return None;
    }

    let host = request.headers().get(header::HOST)?.to_str().ok()?;
    let path = request
        .uri()
        .path_and_query()
        .map(|p| p.as_str())
        .unwrap_or("/");
    let location = format!("https://{}{}", host, path);

    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::TEMPORARY_REDIRECT;
    response
        .headers_mut()
        .insert(header::LOCATION, HeaderValue::from_str(&location).ok()?);
    Some(response)
}

pub async fn security_headers_middleware(request: Request<Body>, next: Next) -> Response {
    if force_https() {
        if let Some(response) = https_redirect(&request) {
            return response;
        }
    }

    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    response
        .headers_mut()
        .insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
        .headers_mut()
        .insert(header::REFERRER_POLICY, HeaderValue::from_static("strict-origin-when-cross-origin"));
    response
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
