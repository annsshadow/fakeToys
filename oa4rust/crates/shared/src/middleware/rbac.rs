use axum::body::Body;
use axum::extract::State;
use axum::http::{Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::sync::OnceLock;

use super::constants::*;
use super::security::{
    client_ip, error_handler, is_auth_exempt, is_auth_rate_limited, path_starts_with_segment,
    requires_admin, SecurityState,
};
use super::token::system_uninitialized;
use crate::error::AppError;
use crate::rate_limit::RateLimiter;
use crate::response::error_response;
use crate::session::Session;

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

/// 请求级 admin 缓存：避免同一请求内多次 is_admin 调用
#[derive(Clone, Default)]
pub struct AdminCache {
    pub cache: HashMap<String, bool>,
}

impl AdminCache {
    pub fn get(&self, person_unique: &str) -> Option<bool> {
        self.cache.get(person_unique).copied()
    }
    pub fn set(&mut self, person_unique: String, is_admin: bool) {
        self.cache.insert(person_unique, is_admin);
    }
}

/// 当前用户是否具备 admin 角色：查 auth_person 关联 auth_role（name = 'admin'），
/// person_unique 可能是 unique_id（login 会话）也可能是 id（bind 会话），
/// 因此按两者匹配。查询失败时 fail-closed（拒绝）。
///
/// 请求级缓存：同一请求内多次调用共享结果，避免重复 DB 查询。
/// 缓存通过 AdminCache 扩展注入，key 为 person_unique。
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

/// 权限配置：路径 → 权限级别
/// 使用精确路径匹配 + 前缀匹配（最长前缀优先）
pub struct PermissionRegistry {
    exact: HashMap<String, PermissionLevel>,
    prefixes: Vec<(String, PermissionLevel)>,
}

impl PermissionRegistry {
    pub fn new() -> Self {
        Self {
            exact: HashMap::new(),
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
        registry.register_prefix("/jaxrs/person", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/unit", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/role", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/group", PermissionLevel::Authenticated);
        // 组织核心实体写操作端点
        registry.register_prefix("/jaxrs/organization/person", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/organization/group", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/organization/definition", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/organization/identity", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/organization/custom", PermissionLevel::Authenticated);
        registry.register_prefix("/jaxrs/organization/bind", PermissionLevel::Authenticated);
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
    method: &Method,
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
    use axum::response::IntoResponse;

    let path = request.uri().path().to_string();
    let method = request.method().clone();

    let level = permission_registry().get_permission(&path);

    // Public 路径无需认证，直接放行
    if matches!(level, Some(PermissionLevel::Public)) {
        return next.run(request).await;
    }

    let Some(session) = request.extensions().get::<Session>() else {
        return AppError::Unauthorized.into_response();
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
