// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::body::Body;
use axum::extract::ConnectInfo;
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
use crate::config::AuthConfig;
use crate::error::AppError;
use crate::rate_limit::RateLimiter;
use crate::session::SessionManager;

// ──────────────────────────────────────────────────────────────────────────────
// CORS 中间件
//
// 允许 o2web 前端跨域访问，支持凭据（Authorization/Cookie）。
// 允许常用读写方法，允许 Authorization/Content-Type 头。
// ──────────────────────────────────────────────────────────────────────────────
pub fn cors_middleware() -> CorsLayer {
    let config = AuthConfig::from_env().expect("invalid browser authentication configuration");
    cors_middleware_for_origin(&config.public_origin)
}

pub fn cors_middleware_for_origin(public_origin: &str) -> CorsLayer {
    use tower_http::cors::AllowOrigin;

    // 除配置的 public_origin 外，放行其回环镜像（localhost ⇄ 127.0.0.1）：
    // 同源访问经 127.0.0.1 时浏览器同样携带 Origin，精确单源白名单会把它
    // 当跨域拒绝（实测 POST 同源 403）。
    let mut origins = vec![public_origin
        .parse::<HeaderValue>()
        .expect("validated APP_PUBLIC_ORIGIN must be a valid header value")];
    if let Ok(url) = url::Url::parse(public_origin) {
        let port = url.port();
        if let Some(host) = url.host_str() {
            let mirror_host = match host {
                "localhost" => Some("127.0.0.1"),
                "127.0.0.1" => Some("localhost"),
                _ => None,
            };
            if let Some(mirror) = mirror_host {
                let mirror_origin = match port {
                    Some(port) => format!("{}://{}:{}", url.scheme(), mirror, port),
                    None => format!("{}://{}", url.scheme(), mirror),
                };
                if let Ok(value) = mirror_origin.parse::<HeaderValue>() {
                    origins.push(value);
                }
            }
        }
    }
    let allow_origin = AllowOrigin::list(origins);

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
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
    // 精确路径匹配（带 {param} 通配）
    AUTH_EXEMPT_PATHS.iter().any(|pattern| path_matches(path, pattern))
        // 前缀匹配（覆盖带动态路径段的子路径）
        || AUTH_EXEMPT_PREFIXES.iter().any(|prefix| path_starts_with_segment(path, prefix))
}

pub(crate) fn is_auth_rate_limited(path: &str) -> bool {
    AUTH_RATE_LIMIT_EXACT.contains(&path)
        || AUTH_RATE_LIMIT_PREFIXES
            .iter()
            .any(|prefix| path_starts_with_segment(path, prefix))
}

/// 路径段前缀匹配：`path` 等于 `prefix` 或 `prefix + "/"` 开头。
/// 带段边界，避免把 /api/personal/* 误判为 /api/person*。
pub(crate) fn path_starts_with_segment(path: &str, prefix: &str) -> bool {
    path == prefix || path.starts_with(&format!("{}/", prefix))
}

/// 角色授权判定：person/unit/role/group 及扩展模块的写操作（POST/PUT/DELETE）
/// 需要 admin 角色。自服务端点（改密、头像）豁免。
pub(crate) fn requires_admin(method: &axum::http::Method, path: &str) -> bool {
    if matches!(method.as_str(), "POST" | "PUT" | "DELETE") {
        if path_starts_with_segment(path, "/api/person/password")
            || path_starts_with_segment(path, "/api/person/icon")
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
    xff.split(',')
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 从请求提取客户端 IP：可信代理来源的 X-Forwarded-For 第一个值，否则 socket 地址。
/// 无 ConnectInfo 时（如单元测试）回退 127.0.0.1。
pub(crate) fn client_ip(request: &Request<Body>) -> String {
    let socket_ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|info| info.ip().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    if trusted_proxy_ips().contains(&socket_ip) {
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
        warn!(
            ?method,
            ?uri,
            status = response.status().as_u16(),
            "server error"
        );
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

/// FORCE_HTTPS 下重定向的 host：生产固定用 APP_PUBLIC_ORIGIN 的 host。
/// 直接拼请求 Host 头会允许攻击者经 `Host: evil.com` 制造 307 开放重定向
///（受害者浏览器被送往攻击者站点）；单源部署下固定配置源即正确行为。
/// 开发环境（未配置或 http 源）回退请求 Host 保持本地直连可用。
fn force_https_host() -> Option<String> {
    static HOST: OnceLock<Option<String>> = OnceLock::new();
    HOST.get_or_init(|| {
        std::env::var("APP_PUBLIC_ORIGIN")
            .ok()
            .and_then(|v| url::Url::parse(&v).ok())
            .filter(|u| u.scheme() == "https")
            .and_then(|u| u.host_str().map(|h| h.to_string()))
    })
    .clone()
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

    let host = match force_https_host() {
        Some(h) => h,
        None => request
            .headers()
            .get(header::HOST)?
            .to_str()
            .ok()?
            .to_string(),
    };
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
    response.headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    response
        .headers_mut()
        .insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    // Strict Content-Security-Policy (enforced, not report-only).
    // - script-src allows 'unsafe-eval' for ECharts/CodeMirror runtime compilation.
    // - style-src allows 'unsafe-inline' for Vue dynamic styles and Chart.js injection.
    // - connect-src allows same-origin XHR/fetch plus dev WebSocket on loopback.
    //   生产 https 部署下同域 wss 由 'self' 覆盖；前端 realtime 仅连同域
    //   （sdk/websocket.ts: window.location.host），无需 `wss://*` 通配——
    //   通配会给任何脚本打开到任意 wss 端点的 WS 外泄通道。
    // - frame-src/object-src blocked; base-uri/form-action restricted.
    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_str(
            "default-src 'self'; \
             script-src 'self' 'unsafe-eval'; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: blob:; \
             connect-src 'self' ws://localhost:* ws://127.0.0.1:*; \
             font-src 'self'; \
             object-src 'none'; \
             frame-src 'none'; \
             base-uri 'self'; \
             form-action 'self'; \
             upgrade-insecure-requests",
        )
        .unwrap(),
    );

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

// ──────────────────────────────────────────────────────────────────────────────
// 安全审计回归测试（2026-10）
//
// 钉死两类事实，防止破坏性写端点被误改回匿名可达：
// 1. 破坏性写端点（批量删附件/清缓存/登出/头像资源删除/控制表写）必须要求会话；
// 2. 只读批量查询端点保持匿名可达（o2server 行为对齐 R401J200）。
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod security_audit_tests {
    use super::is_auth_exempt;

    /// 这些端点会写库（DELETE/UPDATE/INSERT 全表或批处理维护），
    /// 必须要求登录会话，不得匿名可达。若任一被改回豁免即测试失败。
    #[test]
    fn destructive_write_endpoints_require_session() {
        let destructive = [
            // 批量维护作业：删重复附件 / 控制表时间戳写
            "/api/processplatform/service/processing/touch/mergeitem",
            "/api/processplatform/assemble/surface/touch/expire",
            "/api/processplatform/assemble/surface/touch/passexpired",
            "/api/processplatform/assemble/surface/touch/touchdetained",
            // program_center 运维写：清缓存 / 缓存分发 / 头像资源全删 / 注销
            "/api/program_center/jest/clear/cache/source",
            "/api/program_center/cachedispatch",
            "/api/program_center/appstyle/image/application/top/erase",
            "/api/program_center/adminlogin",
            // 批量删附件（管理）
            "/api/processplatform/assemble/surface/attachment/batch/delete/manage",
            // AI 线索删除（owner 门禁）
            "/api/ai/chat/delete/some-clue",
            // 破坏性维护作业/写端点（2026-10-13 二轮复核：原 o2server 对齐豁免误放，
            // handler 均为 admin 门禁，移出豁免后未认证 401 而非 500）
            "/api/attendance/assemble/control/dingding/all",
            "/api/attendance/assemble/control/qywx/all",
            "/api/attendance/assemble/control/statistic/do",
            "/api/attendance/assemble/control/attendancestatisticalcycle",
            "/api/document/batch/d-1",
            "/api/file/assemble/control/file/referencetype/t1/reference/r1",
            "/api/meeting/assemble/control/config",
            "/api/script/list/manager",
        ];
        for path in destructive {
            assert!(
                !is_auth_exempt(path),
                "破坏性写端点 {path} 不得匿名可达（须要求会话）"
            );
        }
    }

    /// 这些是 o2server 对齐的只读批量查询，保持匿名可达（前端未登录态使用）。
    #[test]
    fn readonly_batch_queries_stay_exempt() {
        let readonly = [
            "/api/person/list/all",
            "/api/unit/list/all",
            "/api/person/nick/name",
            "/api/calendar/assemble/control/calendar/follow",
            "/api/attendance/assemble/control/dingdingstatistic",
            "/api/meeting/assemble/control/config/system",
            "/api/person/icon/u-1",
            "/health",
        ];
        for path in readonly {
            assert!(
                is_auth_exempt(path),
                "只读端点 {path} 应保持匿名可达（o2server 对齐）"
            );
        }
    }
}
