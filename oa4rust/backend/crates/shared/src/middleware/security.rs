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
    // 豁免排除（A5 批）：先精确点名，命中则即使匹配下方通配豁免也不放行。
    // 防「本意给只读详情的 {param} 豁免」把写路由一并匿名放行（通配段碰撞）。
    if AUTH_EXEMPT_BLOCKED_PATHS.contains(&path) {
        return false;
    }
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
        // 自服务写豁免：旧路径两支 + o2server 契约别名同权形态（2026-10-24 R1）。
        // 别名漏豁免的后果：契约客户端改自己的密码/传头像要被 admin 权——
        // 与旧路径行为不一致（旧径这两支登录即可）。
        if path_starts_with_segment(path, "/api/person/password")
            || path_starts_with_segment(path, "/api/person/icon")
            || path_starts_with_segment(path, "/api/organization/assemble/personal/password")
            || path_starts_with_segment(path, "/api/organization/assemble/personal/person/password")
            || path_starts_with_segment(path, "/api/organization/assemble/personal/person/icon")
            || path_starts_with_segment(path, "/api/organization/assemble/personal/icon")
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
/// 从 `TRUSTED_PROXY_IPS` 原始字符串解析可信代理白名单。抽成纯函数：
/// OnceLock 缓存的是解析结果，测试可以直接喂不同输入覆盖全部分支，
/// 不必依赖进程内首次初始化的顺序（C1 覆盖率条件项）。
fn trusted_proxy_ips_from(raw: String) -> Vec<String> {
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
}

fn trusted_proxy_ips() -> &'static Vec<String> {
    static TRUSTED: OnceLock<Vec<String>> = OnceLock::new();
    TRUSTED
        .get_or_init(|| trusted_proxy_ips_from(env::var("TRUSTED_PROXY_IPS").unwrap_or_default()))
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
/// 解析 `FORCE_HTTPS` 环境变量。抽成纯函数：OnceLock 缓存解析结果，
/// 测试可直接喂不同值覆盖 true/false 两分支（C1 覆盖率条件项）。
fn force_https_from(v: Option<String>) -> bool {
    v.map(|v| matches!(v.as_str(), "true" | "1"))
        .unwrap_or(false)
}

fn force_https() -> bool {
    static FORCE: OnceLock<bool> = OnceLock::new();
    *FORCE.get_or_init(|| force_https_from(env::var("FORCE_HTTPS").ok()))
}

/// 解析 `APP_PUBLIC_ORIGIN` 得到 FORCE_HTTPS 重定向 host。抽成纯函数：
/// 测试无需设置环境变量即可覆盖 https/非 https/解析失败/缺失全部分支。
fn force_https_host_from(v: Option<String>) -> Option<String> {
    v.and_then(|v| url::Url::parse(&v).ok())
        .filter(|u| u.scheme() == "https")
        .and_then(|u| u.host_str().map(|h| h.to_string()))
}

/// FORCE_HTTPS 下重定向的 host：生产固定用 APP_PUBLIC_ORIGIN 的 host。
/// 直接拼请求 Host 头会允许攻击者经 `Host: evil.com` 制造 307 开放重定向
///（受害者浏览器被送往攻击者站点）；单源部署下固定配置源即正确行为。
/// 开发环境（未配置或 http 源）回退请求 Host 保持本地直连可用。
fn force_https_host() -> Option<String> {
    static HOST: OnceLock<Option<String>> = OnceLock::new();
    HOST.get_or_init(|| force_https_host_from(env::var("APP_PUBLIC_ORIGIN").ok()))
        .clone()
}

/// `https_redirect` 的核心逻辑，host 来源参数化：测试直接喂配置 host 或
/// `None`（回退请求 Host 头），不必依赖 `force_https_host` 的 OnceLock
/// 初始化顺序（C1 覆盖率条件项：进程隔离互斥）。
fn https_redirect_with(
    configured_host: Option<String>,
    request: &Request<Body>,
) -> Option<Response> {
    let proto = request
        .headers()
        .get("x-forwarded-proto")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("https");
    if proto.eq_ignore_ascii_case("https") {
        return None;
    }

    let host = match configured_host {
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

/// FORCE_HTTPS 下对 HTTP 请求生成 307 重定向；https 请求直接放行。
fn https_redirect(request: &Request<Body>) -> Option<Response> {
    https_redirect_with(force_https_host(), request)
}

/// 安全响应头中间件的核心逻辑。`force_https_enabled` 参数化：测试可直接覆盖
/// FORCE_HTTPS=true 的 307 分支，不依赖进程级 OnceLock 的初始化顺序（C1 条件项）。
async fn security_headers_response(
    request: Request<Body>,
    next: Next,
    force_https_enabled: bool,
) -> Response {
    if force_https_enabled {
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

pub async fn security_headers_middleware(request: Request<Body>, next: Next) -> Response {
    security_headers_response(request, next, force_https()).await
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

// o2server 契约路径别名 ↔ 旧路径 的中间件判定 parity（2026-10-24 R1）
//
// 别名路由（/api/organization/assemble/personal/*）在三个 crate 里原样委托
// 旧路径（/api/person/*、/api/reset/*）的同一 handler。路由层 parity 由
// tests/contract_alias_parity.rs（全栈 oneshot 实测）把守；本文件把守
// **中间件判定层** parity——四张表（豁免 / 限流 / 权限注册表 / admin 写）
// 任一表忘了给别名补镜像，这里就红：
//   - exempt 忘补 → 契约客户端匿名注册/重置平白 401（功能回归）
//   - admin 忘补 → 契约客户端绕过 admin 门禁（安全弱化）
//   - 多补（如给 mockdeletetoget 加豁免）→ 别名比旧径更宽松（安全弱化）
// 表由 Temp/r1_gen_aliases.py 从契约 JSON + 已注册路由生成，勿手改。

#[cfg(test)]
mod contract_alias_parity_tests {
    use super::{is_auth_exempt, is_auth_rate_limited, requires_admin};
    use crate::middleware::contract_alias::CONTRACT_ALIAS_PAIRS;
    use crate::middleware::rbac::{is_registry_public, PermissionLevel, PermissionRegistry};
    use axum::http::Method;

    /// 复刻 `check_permission` 的控制流算「有效 admin 要求」。
    ///
    /// 不能直接比 `requires_admin` 的裸输出：它对注册表未命中（None）的路径
    /// 永远不会被 consult（None 早退 Authenticated），而旧径 /api/person/* 命中
    /// catch-all Authenticated——裸比会同时漏掉「别名绕不过 admin」和
    /// 「Public 路径本就不查 admin」两种情形。有效值才是用户可见的判定。
    fn effective_admin(method: &Method, path: &str) -> bool {
        match PermissionRegistry::with_defaults().get_permission(path) {
            // Public 直接放行；None 早退 Authenticated 且不查 admin。
            Some(PermissionLevel::Public) | None => false,
            // Authenticated 才 consult requires_admin。
            Some(PermissionLevel::Authenticated) => requires_admin(method, path),
            // Admin/Role/Group/Owner/Forbidden 级不走 requires_admin 这支
            // （本表 73 对路径不涉），按「不因 admin 写门拦」记。
            _ => false,
        }
    }

    #[test]
    fn alias_middleware_decisions_match_old_paths() {
        let mut checked = 0usize;
        for (m, alias, om, old) in CONTRACT_ALIAS_PAIRS {
            let method: Method = m.parse().expect("合法 HTTP 方法");
            let old_method: Method = om.parse().expect("合法 HTTP 方法");
            assert_eq!(
                is_auth_exempt(alias),
                is_auth_exempt(old),
                "豁免判定不一致：{m} {alias} vs 旧径 {old}"
            );
            assert_eq!(
                is_auth_rate_limited(alias),
                is_auth_rate_limited(old),
                "限流判定不一致：{m} {alias} vs 旧径 {old}"
            );
            assert_eq!(
                is_registry_public(alias),
                is_registry_public(old),
                "权限注册表 Public 判定不一致：{m} {alias} vs 旧径 {old}"
            );
            assert_eq!(
                effective_admin(&method, alias),
                effective_admin(&old_method, old),
                "有效 admin 判定不一致：{m} {alias} vs 旧径 {om} {old}"
            );
            checked += 1;
        }
        assert!(checked >= 70, "测试表意外缩小：仅 {checked} 对");
    }

    #[test]
    fn no_alias_is_exempt_but_not_public() {
        // 双锁单向不变式：豁免了就必须 Public——否则「不插 Session + 要 Session」
        // = 对所有人 401 的死态（exempt_authorize_parity.rs 同款判据）。
        // 反方向（Public 但非豁免）允许存在：旧径 mockdeletetoget 家族就是，
        // 别名照旧镜像（parity 优先于洁癖），由上一个测试把守。
        for (m, alias, ..) in CONTRACT_ALIAS_PAIRS {
            if is_auth_exempt(alias) {
                assert!(
                    is_registry_public(alias),
                    "别名 {m} {alias} 豁免但注册表非 Public = 401 死态"
                );
            }
        }
    }

    #[test]
    fn org_auth_login_alias_is_rate_limited_like_canonical() {
        // 2026-10-30：org_auth 契约别名 `/api/organization/assemble/authentication/authentication`
        // 原样委托 auth::login（与契约路径 POST /api/authentication 同一 handler）。
        // 它豁免 + Public（双锁同开），若不在认证限流名单，rate_limit_middleware
        // 对「豁免且未列限流」直接跳过 ⇒ 该别名成为无上限的登录暴力破解入口。
        let alias = "/api/organization/assemble/authentication/authentication";
        let canonical = "/api/authentication";
        assert!(is_auth_exempt(alias), "登录别名必须在豁免表（契约匿名）");
        assert!(is_registry_public(alias), "登录别名必须 Public（双锁同开）");
        assert_eq!(
            is_auth_rate_limited(alias),
            is_auth_rate_limited(canonical),
            "登录别名与契约路径限流不一致：{alias} 必须与 {canonical} 同权（否则暴力破解绕行）"
        );
        assert!(is_auth_rate_limited(alias), "登录别名必须命中认证限流");
    }
}
// ── 2026-10-29 覆盖率轮：CORS 镜像分支、trace/error_handler、HTTPS 重定向 ──
#[cfg(test)]
mod cov_middleware_tests {
    use super::*;
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    #[test]
    fn cors_middleware_for_origin_builds_mirror_origins() {
        // 默认源 localhost:3000 → 回环镜像 127.0.0.1:3000 一并放行
        let _ = cors_middleware_for_origin("http://localhost:3000");
        // 反向镜像：配置 127.0.0.1 时放行 localhost
        let _ = cors_middleware_for_origin("http://127.0.0.1:3000");
        // 非回环 host：无镜像，仅单源
        let _ = cors_middleware_for_origin("https://prod.example");
        // 带端口的镜像需保留端口
        let _ = cors_middleware_for_origin("http://localhost");
    }

    #[test]
    fn cors_middleware_reads_env_config_without_panic() {
        // 非生产环境缺省 origin 为 http://localhost:3000（from_env 的缺省分支）
        let _ = cors_middleware();
    }

    #[tokio::test]
    async fn trace_middleware_warns_only_on_server_errors() {
        let app = Router::new()
            .route(
                "/boom",
                get(|| async { axum::http::StatusCode::INTERNAL_SERVER_ERROR }),
            )
            .route("/ok", get(|| async { "fine" }))
            .layer(axum::middleware::from_fn(trace_middleware));
        let boom = app
            .clone()
            .oneshot(Request::builder().uri("/boom").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(boom.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        let ok = app
            .oneshot(Request::builder().uri("/ok").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(ok.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn error_handler_delegates_to_into_response() {
        let request = Request::builder().uri("/x").body(Body::empty()).unwrap();
        let resp = error_handler(AppError::NotFound, request).await;
        assert_eq!(resp.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn https_redirect_passthrough_for_https_proto() {
        let request = Request::builder()
            .header("x-forwarded-proto", "https")
            .uri("/x")
            .body(Body::empty())
            .unwrap();
        // 用参数化入口，避免初始化 force_https_host 的 OnceLock（进程隔离互斥）
        assert!(
            https_redirect_with(None, &request).is_none(),
            "已是 https 不重定向"
        );
    }

    #[test]
    fn https_redirect_uses_configured_public_origin_host() {
        // 直接喂配置 host，不设置 APP_PUBLIC_ORIGIN（避免污染并行测试）：
        // 重定向 host 必须取配置源而非请求 Host 头（防 307 开放重定向）
        let request = Request::builder()
            .header("x-forwarded-proto", "http")
            .header(header::HOST, "evil.com")
            .uri("/login?next=/a")
            .body(Body::empty())
            .unwrap();
        let resp = https_redirect_with(Some("prod.example".to_string()), &request)
            .expect("http 请求必须被重定向");
        assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
        let loc = resp
            .headers()
            .get(header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .unwrap()
            .to_string();
        assert_eq!(
            loc, "https://prod.example/login?next=/a",
            "host 取配置源且保留 path+query"
        );
        assert!(!loc.contains("evil.com"), "绝不沿用请求 Host 头");
    }

    #[test]
    fn https_redirect_falls_back_to_request_host_without_configured_origin() {
        // 未配置 APP_PUBLIC_ORIGIN（或非 https 源）时回退请求 Host 头。
        let request = Request::builder()
            .header("x-forwarded-proto", "http")
            .header(header::HOST, "local.dev")
            .uri("/x")
            .body(Body::empty())
            .unwrap();
        let resp = https_redirect_with(None, &request).expect("http 请求必须被重定向");
        let loc = resp
            .headers()
            .get(header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .unwrap()
            .to_string();
        assert_eq!(loc, "https://local.dev/x");
    }

    #[test]
    fn force_https_from_env_branches() {
        assert!(force_https_from(Some("true".to_string())));
        assert!(force_https_from(Some("1".to_string())));
        assert!(!force_https_from(Some("false".to_string())));
        assert!(!force_https_from(Some("yes".to_string())));
        assert!(!force_https_from(None), "未配置 FORCE_HTTPS 默认不强制");
    }

    #[test]
    fn force_https_host_from_env_branches() {
        assert_eq!(
            force_https_host_from(Some("https://prod.example".to_string())),
            Some("prod.example".to_string())
        );
        assert_eq!(
            force_https_host_from(Some("http://prod.example".to_string())),
            None,
            "非 https 源不用于重定向 host"
        );
        assert_eq!(
            force_https_host_from(Some("not-a-url".to_string())),
            None,
            "解析失败的源忽略"
        );
        assert_eq!(force_https_host_from(None), None, "未配置回退请求 Host");
    }

    #[test]
    fn trusted_proxy_ips_from_env_branches() {
        assert_eq!(
            trusted_proxy_ips_from("10.0.0.1, 10.0.0.2".to_string()),
            vec!["10.0.0.1".to_string(), "10.0.0.2".to_string()],
            "显式配置的可信代理列表"
        );
        assert_eq!(
            trusted_proxy_ips_from(" 10.0.0.1 , ".to_string()),
            vec!["10.0.0.1".to_string()],
            "trim 并过滤空段"
        );
        assert_eq!(
            trusted_proxy_ips_from(String::new()),
            vec!["127.0.0.1".to_string(), "::1".to_string()],
            "未配置时默认仅信任本地回环"
        );
    }

    #[tokio::test]
    async fn security_headers_force_https_redirects_before_downstream() {
        // 参数化入口直接喂 force=true，绕过进程级 OnceLock：FORCE_HTTPS 开启时
        // 必须在下游执行前返回 307（防开放重定向：未配置源时回退请求 Host）。
        let app = Router::new()
            .route("/login", get(|| async { "downstream" }))
            .layer(axum::middleware::from_fn(|req, next| {
                security_headers_response(req, next, true)
            }));
        let resp = app
            .oneshot(
                Request::builder()
                    .header("x-forwarded-proto", "http")
                    .header(header::HOST, "evil.com")
                    .uri("/login")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
        let loc = resp
            .headers()
            .get(header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .unwrap()
            .to_string();
        assert_eq!(loc, "https://evil.com/login", "未配置源时回退请求 Host");
    }

    #[tokio::test]
    async fn security_headers_add_headers_when_https_not_forced() {
        let app = Router::new()
            .route("/x", get(|| async { "downstream" }))
            .layer(axum::middleware::from_fn(|req, next| {
                security_headers_response(req, next, false)
            }));
        let resp = app
            .oneshot(Request::builder().uri("/x").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "下游应正常执行");
        assert!(
            resp.headers().contains_key(header::X_CONTENT_TYPE_OPTIONS),
            "应带 X-Content-Type-Options"
        );
        assert!(
            resp.headers().contains_key(header::CONTENT_SECURITY_POLICY),
            "应带 CSP"
        );
    }
}
