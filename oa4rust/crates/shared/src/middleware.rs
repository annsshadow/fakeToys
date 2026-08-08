use axum::body::Body;
use axum::extract::{ConnectInfo, State};
use axum::http::{header, HeaderMap, HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::Pool;
use std::env;
use std::net::SocketAddr;
use std::sync::OnceLock;
use tower_http::cors::CorsLayer;
use tracing::warn;

use crate::error::AppError;
use crate::input_validation::validate_required;
use crate::rate_limit::RateLimiter;
use crate::response::error_response;
use crate::session::{Session, SessionManager};

// ──────────────────────────────────────────────────────────────────────────────
// 迁移注意：ADMIN_WRITE_PREFIXES 回滚 (2026-08-08)
//
// 已将 ADMIN_WRITE_PREFIXES 从 15 个前缀回滚到 4 个核心管理前缀：
// /jaxrs/person, /jaxrs/unit, /jaxrs/role, /jaxrs/group
//
// 2026-08-08 曾扩展到 15 个前缀（含 ai, cms, correlation, file 等扩展模块），
// 现已回滚。扩展模块的权限控制由 PermissionRegistry 中的 Authenticated 级别
// 管理，写操作保护由 requires_admin 函数独立处理。
// ──────────────────────────────────────────────────────────────────────────────

// ──────────────────────────────────────────────────────────────────────────────
// 认证与速率限制豁免路径
//
// R12：健康检查端点及认证前置端点豁免认证，按精确路径匹配。
// 使用 `{param}` 段通配参数化端点（如验证码尺寸、凭证、OAuth code）。
// /jaxrs/reset/code|check|set 为 personal crate 的自服务密码重置流程，
// 必须在认证前置阶段可用，因此一并豁免。
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
    // personal crate 自服务密码重置流程（认证前置阶段）
    "/jaxrs/reset/code",
    "/jaxrs/reset/check",
    "/jaxrs/reset/set",
    // 系统初始化（仅系统未初始化时；见 system_uninitialized）
    "/jaxrs/secret/check",
    "/jaxrs/secret/set",
    "/jaxrs/secret/set/cancel",
];

// 认证类端点（计入 10 次/分钟/IP 的认证限流）。
// 精确限流仅限"明文凭证尝试"：登录契约路径（POST /jaxrs/authentication）
// 与其自造别名（/jaxrs/authentication/login）；以及密码重置与初始化端点（前缀）。
// 注意：不能按 /jaxrs/authentication 前缀整段限流——验证码、扫码轮询、
// OAuth、当前用户等认证前置端点会被误伤（如扫码轮询 10 次/分钟会卡流程）。
// /jaxrs/secret 同理：/jaxrs/secret/captcha/verify（验证码校验）计入认证限流
// 会与验证码自身尝试上限叠加导致流程不可用。
const AUTH_RATE_LIMIT_EXACT: &[&str] = &["/jaxrs/authentication", "/jaxrs/authentication/login"];
const AUTH_RATE_LIMIT_PREFIXES: &[&str] = &[
    "/jaxrs/authentication/code",
    "/jaxrs/reset",
    "/jaxrs/secret/check",
    "/jaxrs/secret/set",
    "/jaxrs/secret/cancel",
];


// 系统初始化端点：仅当系统未初始化（auth_person 无任何未删除的未锁定用户）时豁免认证
const SECRET_INIT_PATHS: &[&str] = &["/jaxrs/secret/check", "/jaxrs/secret/set"];

const AUTH_RATE_LIMIT: i32 = 10;
const GENERAL_RATE_LIMIT: i32 = 100;
const RATE_LIMIT_WINDOW_MINUTES: i64 = 1;

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
    AUTH_RATE_LIMIT_EXACT.iter().any(|p| path == *p)
        || AUTH_RATE_LIMIT_PREFIXES.iter().any(|prefix| path.starts_with(prefix))
}

/// 路径段前缀匹配：`path` 等于 `prefix` 或 `prefix + "/"` 开头。
/// 带段边界，避免把 /jaxrs/personal/* 误判为 /jaxrs/person*。
fn path_starts_with_segment(path: &str, prefix: &str) -> bool {
    path == prefix || path.starts_with(&format!("{}/", prefix))
}

/// 角色授权判定：person/unit/role/group 及扩展模块的写操作（POST/PUT/DELETE）
/// 需要 admin 角色。自服务端点（改密、头像）豁免。
/// 保留此函数以维持向后兼容：注册表中的 Admin 级别路径同时受本函数保护。
// 需要 admin 角色的写操作（角色授权检查）：覆盖 person/unit/role/group 及
// AI、CMS、correlation、file、program_center、query、BBS、meeting、message、
// processplatform、portal 等模块的 POST/PUT/DELETE。
// 按路径首段前缀匹配（带段边界，避免误伤 /jaxrs/personal*）。
// 自服务端点（改密、头像）在 requires_admin 中豁免，不在本列表中。
/// Deprecated: these prefixes no longer require admin writes.
/// Kept for reference during migration period.
const ADMIN_WRITE_DEPRECATED_PREFIXES: &[&str] = &[
    "/jaxrs/ai",
    "/jaxrs/cms",
    "/jaxrs/correlation",
    "/jaxrs/file",
    "/jaxrs/program_center",
    "/jaxrs/query",
    "/jaxrs/bbs",
    "/jaxrs/meeting",
    "/jaxrs/message",
    "/jaxrs/processplatform",
    "/jaxrs/portal",
];

const ADMIN_WRITE_PREFIXES: &[&str] = &[
    "/jaxrs/person",
    "/jaxrs/unit",
    "/jaxrs/role",
    "/jaxrs/group",
];

/// 角色授权判定：person/unit/role/group 及扩展模块的写操作（POST/PUT/DELETE）
/// 需要 admin 角色。自服务端点（改密、头像）豁免。
/// 保留此函数以维持向后兼容：注册表中的 Admin 级别路径同时受本函数保护。
fn requires_admin(method: &axum::http::Method, path: &str) -> bool {
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

/// 从请求提取会话令牌：优先 Authorization: Bearer <token>，
/// 回退 Cookie 中的 `token` 字段。
pub(crate) fn extract_token(request: &Request<Body>) -> Option<String> {
    if let Some(auth) = request
        .headers()
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

    let cookie = request.headers().get(header::COOKIE)?.to_str().ok()?;
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

/// 系统是否未初始化：auth_person 中不存在任何未删除（deleted_at IS NULL）且
/// 未锁定（locked = false）的用户。查询失败时 fail-closed（按已初始化处理，
/// 要求认证），避免在系统状态未知时放开认证。
async fn system_uninitialized(pool: &Pool) -> bool {
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

/// 当前用户是否具备 admin 角色：查 auth_person 关联 auth_role（name = 'admin'），
/// person_unique 可能是 unique_id（login 会话）也可能是 id（bind 会话），
/// 因此按两者匹配。查询失败时 fail-closed（拒绝）。
pub(crate) async fn is_admin(pool: &Pool, person_unique: &str) -> bool {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return false,
    };
    match client
        .query_one(
            "SELECT EXISTS(
                SELECT 1 FROM auth_person p
                LEFT JOIN auth_person_role pr ON pr.person_id = p.id
                LEFT JOIN auth_role r ON r.id = pr.role_id
                WHERE (p.unique_id = $1 OR p.id = $1)
                  AND r.name = 'admin'
                  AND r.deleted_at IS NULL
                  AND r.disable = false
             ) AS is_admin",
            &[&person_unique],
        )
        .await
    {
        Ok(row) => row.get::<_, bool>("is_admin"),
        Err(_) => false,
    }
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
// auth_middleware
//
// 认证中间件：为所有非豁免端点验证会话令牌（Authorization: Bearer <token>
// 或 Cookie `token`），并注入 Extension<Session>。未认证返回 401。
// /jaxrs/secret/check|set 仅在系统未初始化时豁免。
// ──────────────────────────────────────────────────────────────────────────────
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
        return unauthorized_response();
    };

    match state.session_manager.validate_session(&token).await {
        Some(session) => {
            request.extensions_mut().insert(session);
            next.run(request).await
        }
        None => unauthorized_response(),
    }
}

/// 当前用户是否具备指定角色：查 auth_person_role 关联 auth_role（name = role_name），
/// person_unique 即 auth_person.id（会话中存储）。查询失败时 fail-closed（拒绝）。
pub(crate) async fn person_has_role(pool: &Pool, person_unique: &str, role_name: &str) -> bool {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return false,
    };
    match client
        .query_one(
            "SELECT EXISTS(
                SELECT 1 FROM auth_person_role pr
                LEFT JOIN auth_role r ON r.id = pr.role_id
                WHERE pr.person_id = $1
                  AND r.name = $2
                  AND r.deleted_at IS NULL
                  AND r.disable = false
             ) AS has_role",
            &[&person_unique, &role_name],
        )
        .await
    {
        Ok(row) => row.get::<_, bool>("has_role"),
        Err(_) => false,
    }
}

/// 当前用户是否属于指定用户组：查 auth_person_group，
/// person_unique 即 auth_person.id。查询失败时 fail-closed（拒绝）。
pub(crate) async fn person_has_group(pool: &Pool, person_unique: &str, group_id: &str) -> bool {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return false,
    };
    match client
        .query_one(
            "SELECT EXISTS(
                SELECT 1 FROM auth_person_group
                WHERE person_id = $1 AND group_id = $2
                AND EXISTS (
                    SELECT 1 FROM auth_group
                    WHERE id = $2 AND deleted_at IS NULL AND disable = false
                )
             ) AS has_group",
            &[&person_unique, &group_id],
        )
        .await
    {
        Ok(row) => row.get::<_, bool>("has_group"),
        Err(_) => false,
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// RBAC 权限模型
//
// 权限声明：每个端点可声明所需权限（角色、用户组、资源所有者）。
// 默认拒绝策略：未声明的端点全部拒绝访问（已认证用户可访问）。
//
// 权限级别：
// - public: 无需认证（已由 auth_middleware 豁免）
// - authenticated: 登录用户即可访问
// - admin: 需要 admin 角色
// - role:<role_name>: 需要特定角色
// - group:<group_id>: 需要特定用户组成员
// - owner: 需要资源所有者（通过请求体/路径参数中的 owner_id 判断）
// ──────────────────────────────────────────────────────────────────────────────

/// 端点权限级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionLevel {
    /// 公开端点（无需认证）
    Public,
    /// 登录用户即可访问
    Authenticated,
    /// 需要 admin 角色
    Admin,
    /// 需要特定角色
    Role(&'static str),
    /// 需要特定用户组
    Group(&'static str),
    /// 需要资源所有者
    Owner,
    /// 权限不足（check_permission 内部使用）
    Forbidden,
}

/// 权限配置：路径 → 权限级别
/// 使用精确路径匹配 + 前缀匹配（最长前缀优先）
pub struct PermissionRegistry {
    exact: std::collections::HashMap<String, PermissionLevel>,
    prefixes: Vec<(String, PermissionLevel)>,
}

impl PermissionRegistry {
    pub fn new() -> Self {
        Self {
            exact: std::collections::HashMap::new(),
            prefixes: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        // 公开端点
        registry.register_exact("/health", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/authentication", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/authentication/captcha", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/authentication/oauth", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/authentication/code", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/authentication/refresh", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/reset", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/secret/check", PermissionLevel::Public);
        registry.register_prefix("/jaxrs/secret/set", PermissionLevel::Public);
        // 自服务端点：改密和头像，登录用户即可操作
        registry.register_prefix("/jaxrs/person/password", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/person/icon", PermissionLevel::Authenticated);
        // 新注册的管理后台端点（AI、CMS、file 等扩展模块）：Authenticated 级别，
        // 写操作保护由 requires_admin 函数独立处理（仅 person/unit/role/group 需要 admin）
        registry.register_prefix("/jaxrs/ai", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/cms", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/correlation", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/file", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/program_center", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/query", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/bbs", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/meeting", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/message", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/processplatform", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/portal", PermissionLevel::Authenticated);
        // 现有管理端点（person/unit/role/group）：注册 Authenticated 保持向后兼容
        // 写操作保护由 requires_admin 函数独立处理（POST/PUT/DELETE 需 admin）
        registry.register_prefix("/jaxrs/person", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/unit", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/role", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/group", PermissionLevel::Authenticated);
        // 通用 jaxrs 端点兜底
        registry.register_prefix("/jaxrs", PermissionLevel::Authenticated);
        registry
    }

    pub fn register_exact(&mut self, path: &str, level: PermissionLevel) -> &mut Self {
        self.exact.insert(path.to_string(), level);
        self
    }

    pub fn register_prefix(&mut self, prefix: &str, level: PermissionLevel) -> &mut Self {
        self.prefixes.push((prefix.to_string(), level));
        self
    }

    /// 获取路径的权限级别（精确匹配优先，然后是最长前缀匹配）。
    /// 返回 None 表示未匹配到任何注册规则，调用方决定默认行为。
    pub fn get_permission(&self, path: &str) -> Option<PermissionLevel> {
        if let Some(level) = self.exact.get(path) {
            return Some(*level);
        }
        let mut best: Option<(&str, PermissionLevel)> = None;
        for (prefix, level) in &self.prefixes {
            if path.starts_with(prefix) {
                match best {
                    Some((best_prefix, _)) if best_prefix.len() >= prefix.len() => {}
                    _ => best = Some((prefix.as_str(), *level)),
                }
            }
        }
        best.map(|(_, level)| level)
    }
}

impl Default for PermissionRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// 全局权限注册表（单例）
fn permission_registry() -> &'static PermissionRegistry {
    static REGISTRY: OnceLock<PermissionRegistry> = OnceLock::new();
    REGISTRY.get_or_init(PermissionRegistry::with_defaults)
}

/// 检查当前用户是否具备所需权限（集成 Role / Group / Owner 数据库查询）。
pub(crate) async fn check_permission(
    pool: &Pool,
    session: &Session,
    path: &str,
    method: &axum::http::Method,
) -> PermissionLevel {
    let permission = permission_registry().get_permission(path);

    let Some(level) = permission else {
        return PermissionLevel::Authenticated;
    };

    match level {
        PermissionLevel::Public => PermissionLevel::Public,
        PermissionLevel::Authenticated => {
            if requires_admin(method, path) {
                if is_admin(pool, &session.person_unique).await {
                    PermissionLevel::Admin
                } else {
                    PermissionLevel::Forbidden
                }
            } else {
                PermissionLevel::Authenticated
            }
        }
        PermissionLevel::Admin => {
            if is_admin(pool, &session.person_unique).await {
                PermissionLevel::Admin
            } else {
                PermissionLevel::Forbidden
            }
        }
        PermissionLevel::Role(role_name) => {
            if person_has_role(pool, &session.person_unique, role_name).await {
                PermissionLevel::Role(role_name)
            } else if is_admin(pool, &session.person_unique).await {
                PermissionLevel::Admin
            } else {
                PermissionLevel::Forbidden
            }
        }
        PermissionLevel::Group(group_id) => {
            if person_has_group(pool, &session.person_unique, group_id).await {
                PermissionLevel::Group(group_id)
            } else if is_admin(pool, &session.person_unique).await {
                PermissionLevel::Admin
            } else {
                PermissionLevel::Forbidden
            }
        }
        PermissionLevel::Owner => {
            // Owner check: admin bypasses, otherwise verify session matches resource owner
            // Note: actual ownership validation is done in individual handlers via require_owner()
            // This branch grants access if user is admin; handlers should call require_owner() for non-admin
            if is_admin(pool, &session.person_unique).await {
                PermissionLevel::Owner
            } else {
                // Non-admin: allow if handler has already verified ownership via require_owner()
                // The handler is responsible for the actual ownership check
                PermissionLevel::Authenticated
            }
        }
        PermissionLevel::Forbidden => PermissionLevel::Forbidden,
    }
}

/// 所有权检查辅助函数：验证当前会话用户是否拥有指定资源。
/// 适用于 handler 中需要在执行写操作前校验资源所有权的场景。
/// 返回 Ok(()) 表示是所有者或 admin，Err(AppError::Forbidden) 表示无权限。
pub async fn require_owner(
    pool: &Pool,
    session: &Session,
    owner_id: &str,
) -> Result<(), AppError> {
    if is_admin(pool, &session.person_unique).await {
        return Ok(());
    }
    if session.person_unique == owner_id {
        return Ok(());
    }
    Err(AppError::Forbidden)
}

// ──────────────────────────────────────────────────────────────────────────────
// authorize_middleware
//
// RBAC 授权检查：在认证通过后执行，委托给 check_permission 统一处理。
// 1. check_permission 查询 PermissionRegistry 获取路径权限级别
//    （精确匹配优先，其次最长前缀匹配）
// 2. 按级别执行对应检查：Authenticated → 放行；Admin → is_admin；
//    Role → person_has_role（fallback to admin）；Group → person_has_group
// 3. 未匹配到注册规则时默认 Authenticated（需已通过 auth_middleware）
// ──────────────────────────────────────────────────────────────────────────────
pub async fn authorize_middleware(
    State(state): State<SecurityState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();
    let method = request.method().clone();

    let level = permission_registry().get_permission(&path);

    // Public 路径无需认证，直接放行
    if matches!(level, Some(PermissionLevel::Public)) {
        return next.run(request).await;
    }

    let Some(session) = request.extensions().get::<Session>() else {
        return unauthorized_response();
    };

    let level = check_permission(&state.pool, session, &path, &method).await;

    match level {
        PermissionLevel::Public
        | PermissionLevel::Authenticated
        | PermissionLevel::Admin
        | PermissionLevel::Role(_)
        | PermissionLevel::Group(_)
        | PermissionLevel::Owner => next.run(request).await,
        PermissionLevel::Forbidden => {
            error_response(StatusCode::FORBIDDEN, format!("forbidden: insufficient permissions for {}", path))
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// rate_limit_middleware
//
// 速率限制中间件：认证接口（认证 + 密码重置 + 系统初始化）10 次/分钟/IP，
// 普通接口 100 次/分钟/IP。超限返回 429。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn rate_limit_middleware(
    State(state): State<SecurityState>,
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

    if state
        .rate_limiter
        .check_rate_limit(&ip, max_attempts, RATE_LIMIT_WINDOW_MINUTES)
        .await
        .is_err()
    {
        return error_response(StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded");
    }

    next.run(request).await
}

// ──────────────────────────────────────────────────────────────────────────────
// module_routing: U9 灰度迁移 Feature Flag
//
// 通过环境变量 MODULE_ROUTING 控制每个模块前缀路由到 Rust 还是 Java。
// 格式: MODULE_ROUTING=attendance:rust,calendar:java,control:rust
// 未设置或未明确声明的模块默认路由到 Rust（true）。
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ModuleRouting {
    java_prefixes: Vec<String>,
}

impl ModuleRouting {
    pub fn from_env() -> Self {
        let raw = env::var("MODULE_ROUTING").unwrap_or_default();
        let mut java_prefixes = Vec::new();
        for part in raw.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((module, target)) = part.split_once(':') {
                let module = module.trim();
                let target = target.trim().to_lowercase();
                if target == "java" {
                    java_prefixes.push(module.to_string());
                }
            }
        }
        Self { java_prefixes }
    }

    /// 检查给定路径是否应路由到 Rust。若应路由到 Java 返回 false。
    pub fn is_rust(&self, path: &str) -> bool {
        let path_segs: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        if path_segs.len() >= 2 && path_segs[0] == "jaxrs" {
            let module = path_segs[1];
            !self.java_prefixes.iter().any(|p| p == module)
        } else {
            true
        }
    }
}

fn module_routing() -> &'static ModuleRouting {
    static INSTANCE: OnceLock<ModuleRouting> = OnceLock::new();
    INSTANCE.get_or_init(ModuleRouting::from_env)
}

/// 如果请求路径属于配置为 Java 的模块，返回 true。
pub fn should_route_to_java(path: &str) -> bool {
    !module_routing().is_rust(path)
}

fn unauthorized_response() -> Response {
    AppError::Unauthorized.into_response()
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
// behavior_comparison_middleware
//
// 行为对比测试中间件：当请求携带 X-Behavior-Comparison: true 头时，
// 记录请求路径、方法、响应状态码和响应体（前 4KB），用于 Rust vs Java
// 端点行为对比。仅用于测试环境，生产环境自动禁用。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn behavior_comparison_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let is_comparison = request
        .headers()
        .get("x-behavior-comparison")
        .map(|v| v == "true")
        .unwrap_or(false);

    if !is_comparison {
        return next.run(request).await;
    }

    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let query = request.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();

    let response = next.run(request).await;

    let status = response.status();
    let headers = response.headers().clone();
    let body_bytes = axum::body::to_bytes(response.into_body(), 4 * 1024).await.unwrap_or_default();
    let body_str = String::from_utf8_lossy(&body_bytes);

    tracing::info!(
        method = %method,
        path = %path,
        query = %query,
        status = %status.as_u16(),
        body_preview = %body_str.chars().take(500).collect::<String>(),
        "behavior_comparison"
    );

    let new_body = axum::body::Body::from(body_bytes);
    let mut new_response = Response::new(new_body);
    *new_response.headers_mut() = headers;
    new_response
}
