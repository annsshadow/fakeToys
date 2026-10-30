// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    client_ip, is_auth_exempt, is_auth_rate_limited, requires_admin, SecurityState,
};
use crate::error::AppError;
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
pub async fn is_admin(pool: &Pool, person_unique: &str) -> bool {
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
        registry.register_prefix("/api/authentication", PermissionLevel::Public);
        registry.register_prefix("/api/authentication/captcha", PermissionLevel::Public);
        registry.register_prefix("/api/authentication/oauth", PermissionLevel::Public);
        registry.register_prefix("/api/authentication/code", PermissionLevel::Public);
        registry.register_prefix("/api/authentication/refresh", PermissionLevel::Public);
        // 新增认证端点：显式覆盖，避免继承 /api/authentication 的 Public 权限
        registry.register_prefix(
            "/api/authentication/two_factor",
            PermissionLevel::Authenticated,
        );
        registry.register_exact(
            "/api/authentication/safe/logout",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix(
            "/api/authentication/check/token",
            PermissionLevel::Authenticated,
        );
        registry.register_exact("/api/authentication/switchuser", PermissionLevel::Admin);
        registry.register_exact(
            "/api/authentication/switchuser/mockputtopost",
            PermissionLevel::Admin,
        );
        registry.register_prefix("/api/authentication/sso", PermissionLevel::Authenticated);
        registry.register_prefix("/api/reset", PermissionLevel::Public);
        registry.register_prefix("/api/secret/check", PermissionLevel::Public);
        registry.register_prefix("/api/secret/set", PermissionLevel::Public);
        // 四轮 §三.7 双锁一致性修复：这两条已在 `AUTH_EXEMPT_PATHS` 里显式列为匿名前置
        // 端点（登录验证码校验、注册），但注册表没有对应 Public 条目，被兜底的
        // `/api`→Authenticated 判成需会话，而 auth 层豁免又不插 Session ⇒ 实测对任何人
        // 都 401（`tests/exempt_authorize_parity.rs` 匿名臂）。注册表补齐设计意图。
        registry.register_exact("/api/secret/captcha/verify", PermissionLevel::Public);
        registry.register_exact("/api/person/regist", PermissionLevel::Public);
        // organization_assemble_authentication: 认证前置端点（登录模式、验证码密钥、登出）
        registry.register_prefix(
            "/api/organization/assemble/authentication/authentication",
            PermissionLevel::Public,
        );
        // program_center: 认证查询
        registry.register_prefix(
            "/api/program_center/authentication",
            PermissionLevel::Public,
        );
        // 自服务端点：改密和头像，登录用户即可操作
        registry.register_prefix("/api/person/password", PermissionLevel::Authenticated);
        // GET icon is public per R8 (头像端点无权限也可访问)
        registry.register_prefix("/api/person/icon", PermissionLevel::Public);
        // ── o2server 契约路径别名（2026-10-24 R1，OA-P0-1 收口）────────────
        // 与 AUTH_EXEMPT_PATHS 的别名镜像严格配对（双锁一致性：豁免 ⟺ Public）。
        // 旧路径公开/自服务的形态，别名必须同权——少一条就是「豁免了但
        // 注册表不 Public」的 401 死态，多一条（如把 empower 族标 Public）
        // 就是匿名可写。
        registry.register_prefix(
            "/api/organization/assemble/personal/reset",
            PermissionLevel::Public,
        );
        registry.register_exact(
            "/api/organization/assemble/personal/regist",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/organization/assemble/personal/custom",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/organization/assemble/personal/definition",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/organization/assemble/personal/icon",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/organization/assemble/personal/person/icon",
            PermissionLevel::Public,
        );
        // 契约别名的兜底档位（2026-10-24 R1）：镜像旧径 /api/person 的 catch-all
        // （下方 line ~390 同款）。**不能省**：check_permission 对注册表未命中的
        // 路径（None）早退 Authenticated 且**不 consult requires_admin**——少了这条，
        // 别名下的 empower/person/signature 等写操作就绕过了 admin 门禁
        // （旧径命中 /api/person 兜底，写要 admin；别名不要 = 安全弱化）。
        // 上面的 Public 条目均长于本前缀，最长前缀优先下不受影响。
        registry.register_prefix(
            "/api/organization/assemble/personal",
            PermissionLevel::Authenticated,
        );
        // 新注册的管理后台端点（AI、CMS、file 等扩展模块）：Authenticated 级别，
        // 写操作保护由 requires_admin 函数独立处理（仅 person/unit/role/group 需要 admin）
        registry.register_prefix("/api/ai", PermissionLevel::Authenticated);
        registry.register_prefix("/api/cms", PermissionLevel::Authenticated);
        registry.register_prefix("/api/correlation", PermissionLevel::Authenticated);
        registry.register_prefix("/api/file", PermissionLevel::Authenticated);
        registry.register_prefix("/api/program_center", PermissionLevel::Authenticated);
        registry.register_prefix("/api/query", PermissionLevel::Authenticated);
        registry.register_prefix("/api/bbs", PermissionLevel::Authenticated);
        registry.register_prefix("/api/meeting", PermissionLevel::Authenticated);
        registry.register_prefix("/api/message", PermissionLevel::Authenticated);
        registry.register_prefix("/api/processplatform", PermissionLevel::Authenticated);
        registry.register_prefix("/api/portal", PermissionLevel::Authenticated);
        // express batch query endpoints: no authentication required (R24)
        registry.register_prefix("/api/express/person", PermissionLevel::Public);
        registry.register_prefix("/api/express/unit", PermissionLevel::Public);
        registry.register_prefix("/api/express/identity", PermissionLevel::Public);
        registry.register_prefix("/api/express/group", PermissionLevel::Public);
        registry.register_prefix("/api/express/role", PermissionLevel::Public);
        // 与 AUTH_EXEMPT_PATHS 对齐：列表查询端点无需认证（R401J200 消除）
        // 注意：带 {param} 的路径段需逐项注册前缀，以覆盖 get_permission 的 prefix 匹配
        registry.register_prefix("/api/person/list", PermissionLevel::Public);
        registry.register_prefix("/api/person/detail", PermissionLevel::Public);
        registry.register_prefix("/api/person/has/role", PermissionLevel::Public);
        registry.register_prefix("/api/person/custom", PermissionLevel::Public);
        registry.register_prefix("/api/person/definition", PermissionLevel::Public);
        registry.register_prefix("/api/group/list", PermissionLevel::Public);
        registry.register_prefix("/api/group/has/role", PermissionLevel::Public);
        registry.register_prefix("/api/role/list", PermissionLevel::Public);
        registry.register_prefix("/api/unit/list", PermissionLevel::Public);
        registry.register_prefix("/api/unit/check", PermissionLevel::Public);
        registry.register_prefix("/api/unit/identity", PermissionLevel::Public);
        // 批量查询 / 汇总端点：o2server 侧无需认证（R401J200 消除）
        registry.register_prefix("/api/document/batch", PermissionLevel::Public);
        registry.register_prefix(
            "/api/file/assemble/control/file/referencetype",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/meeting/assemble/control/config",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/attendance/assemble/control/dingding",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/attendance/assemble/control/qywx",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/attendance/assemble/control/statistic",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/attendance/assemble/control/attendancestatisticalcycle",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/application/list",
            PermissionLevel::Public,
        );
        // 破坏性写（batch delete attachments）：o2server 对齐虽列 Public，但 Rust 侧
        // handler 以 require_admin 门禁，故降为 Authenticated（登录用户 + admin 校验）。
        registry.register_prefix(
            "/api/program_center/adminlogin",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix("/api/program_center/applications", PermissionLevel::Public);
        registry.register_prefix(
            "/api/program_center/center/applications",
            PermissionLevel::Public,
        );
        registry.register_prefix("/api/queryview/morelikethis", PermissionLevel::Public);
        registry.register_prefix("/api/queryview/search", PermissionLevel::Public);
        registry.register_prefix("/api/script/list/manager", PermissionLevel::Public);
        // 企微扫码登录回调（B6）：一次性 code 由企微 OAuth 侧验证，必须先于认证可达。
        // 豁免表已含该路径，这里补 Public 注册，消除「豁免 ∧ ¬Public」的惰性死态。
        registry.register_exact(
            "/api/qiyeweixin/update/person/detail/{code}",
            PermissionLevel::Public,
        );
        // processplatform assemble/surface 查询端点：o2server 侧无需认证（消除 R401/R500）
        registry.register_prefix(
            "/api/processplatform/assemble/surface/application/list",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/control/workorworkcompleted",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/readrecord/list",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/record/list",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/sign/list",
            PermissionLevel::Public,
        );
        // surface touch/*（超期/超期通过/催办 控制表时间戳写）：handler 要求登录会话，
        // 故从 Public 降为 Authenticated。
        registry.register_prefix(
            "/api/processplatform/assemble/surface/touch/expire",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/touch/passexpired",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/touch/touchdetained",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix(
            "/api/processplatform/assemble/surface/work/count",
            PermissionLevel::Public,
        );
        // service/processing touch/mergeitem（去重删 x_attachment 破坏性批作业）：
        // handler 以 require_admin 门禁，降为 Authenticated。
        registry.register_prefix(
            "/api/processplatform/service/processing/touch/mergeitem",
            PermissionLevel::Authenticated,
        );
        // program_center 查询端点
        registry.register_prefix("/api/program_center/datastructure", PermissionLevel::Public);
        registry.register_prefix(
            "/api/program_center/dingding/get/callback",
            PermissionLevel::Public,
        );
        // jest clear cache（写 x_program_sync_log）：handler 以 require_admin 门禁。
        registry.register_prefix(
            "/api/program_center/jest/clear/cache",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix(
            "/api/program_center/market/cloud/unit/is/vip",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/program_center/market/installed/version",
            PermissionLevel::Public,
        );
        // 剩余 program_center 查询/管理端点
        // appstyle：图片读/擦除/上传全部仅管理控制台消费（无未认证读），
        // 移出豁免后由 auth 层要求会话，写操作在 handler 内 require_admin。
        registry.register_prefix(
            "/api/program_center/appstyle",
            PermissionLevel::Authenticated,
        );
        // cachedispatch / ai chat delete：破坏性写，handler 已 require_admin / require_owner，
        // 降为 Authenticated（需登录会话）。
        registry.register_prefix(
            "/api/program_center/cachedispatch",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix("/api/ai/chat/delete", PermissionLevel::Authenticated);
        registry.register_prefix(
            "/api/calendar/assemble/control/calendar/follow",
            PermissionLevel::Public,
        );
        registry.register_prefix("/api/person/nick/name", PermissionLevel::Public);
        registry.register_prefix("/api/unit/list/all", PermissionLevel::Public);
        registry.register_prefix("/api/person/list/all", PermissionLevel::Public);
        // attendance statistic 端点
        registry.register_prefix(
            "/api/attendance/assemble/control/dingdingstatistic",
            PermissionLevel::Public,
        );
        registry.register_prefix(
            "/api/attendance/assemble/control/qywxstatistic",
            PermissionLevel::Public,
        );
        // 现有管理端点（person/unit/role/group）：注册 Authenticated 保持向后兼容
        registry.register_prefix("/api/person", PermissionLevel::Authenticated);
        registry.register_prefix("/api/unit", PermissionLevel::Authenticated);
        registry.register_prefix("/api/role", PermissionLevel::Authenticated);
        registry.register_prefix("/api/group", PermissionLevel::Authenticated);
        // 组织核心实体写操作端点
        registry.register_prefix("/api/organization/person", PermissionLevel::Authenticated);
        registry.register_prefix("/api/organization/group", PermissionLevel::Authenticated);
        registry.register_prefix(
            "/api/organization/definition",
            PermissionLevel::Authenticated,
        );
        registry.register_prefix("/api/organization/identity", PermissionLevel::Authenticated);
        registry.register_prefix("/api/organization/custom", PermissionLevel::Authenticated);
        registry.register_prefix("/api/organization/bind", PermissionLevel::Authenticated);
        // 通用 o2server 端点兜底
        registry.register_prefix("/api", PermissionLevel::Authenticated);
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

/// 路径在权限注册表上是否命中 `Public`（四轮 §三.7 的「第二把锁」）。
///
/// 供 `auth_middleware` 做双锁一致性判定：豁免表早退时不插 Session，而 authorize
/// 只在 Public 时放行，所以「豁免 ∧ ¬Public」= 对所有用户（含合法会话）401 死态。
pub(crate) fn is_registry_public(path: &str) -> bool {
    matches!(
        permission_registry().get_permission(path),
        Some(PermissionLevel::Public)
    )
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
///
/// 比较统一做 `trim().to_lowercase()` 归一化（与 attendance 的 `normalize_key` 口径一致）。
/// 修复历史缺陷：原先 `session.person_unique == owner_id` 是原始字符串比较，当身份标识存在
/// 大小写/空白差异时，本人操作会被误判为越权而拒绝，且归一化输入与原始会话比较口径不一致
/// 会在 IDOR 校验中产生歧义。归一化后同一身份的大小写变体视为同一人，既修复本人误拒，
/// 也不引入越权（不同身份归一化后仍不相等）。
pub async fn require_owner(pool: &Pool, session: &Session, owner_id: &str) -> Result<(), AppError> {
    if is_admin(pool, &session.person_unique).await {
        return Ok(());
    }
    if session.person_unique.trim().to_lowercase() == owner_id.trim().to_lowercase() {
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
        PermissionLevel::Forbidden => error_response(
            StatusCode::FORBIDDEN,
            format!("forbidden: insufficient permissions for {}", path),
        ),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// rate_limit_middleware
//
// 速率限制中间件：认证接口（认证 + 密码重置 + 系统初始化 + express 枚举面）
// 默认 30 次/分钟/IP，普通接口默认 300 次/分钟/IP（可用
// OA4RUST_AUTH_RATE_LIMIT / OA4RUST_GENERAL_RATE_LIMIT 环境变量覆盖）。
// 超限返回 429。
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

    // 认证与普通请求必须用**独立的桶**：两者共用 `ip` 会让普通流量吃掉登录预算
    // （实测 30 次/分钟的认证上限被 15 个页面请求 + 15 次登录打满，登录提前 429）。
    let (key, max_attempts) = if is_auth_rate_limited(&path) {
        (format!("auth:{ip}"), auth_rate_limit())
    } else {
        (format!("general:{ip}"), general_rate_limit())
    };

    if state
        .rate_limiter
        .check_rate_limit(&key, max_attempts, RATE_LIMIT_WINDOW_MINUTES)
        .await
        .is_err()
    {
        return error_response(StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded");
    }

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_registry_default_uses_defaults() {
        // `Default` 委托 `with_defaults()`（与单例 permission_registry 同源）。
        let d = PermissionRegistry::default();
        assert_eq!(
            d.get_permission("/api/authentication"),
            Some(PermissionLevel::Public)
        );
        assert_eq!(
            d.get_permission("/api/person"),
            Some(PermissionLevel::Authenticated)
        );
    }

    #[test]
    fn registry_matches_exact_before_prefix() {
        let mut reg = PermissionRegistry::new();
        reg.register_prefix("/api/x", PermissionLevel::Authenticated);
        reg.register_exact("/api/x/y", PermissionLevel::Admin);
        // 精确注册必须压过任何前缀（switchuser 类端点的权限覆盖依赖此规则）。
        assert_eq!(reg.get_permission("/api/x/y"), Some(PermissionLevel::Admin));
        assert_eq!(
            reg.get_permission("/api/x/z"),
            Some(PermissionLevel::Authenticated)
        );
    }

    #[test]
    fn registry_picks_longest_prefix() {
        let mut reg = PermissionRegistry::new();
        reg.register_prefix("/api", PermissionLevel::Authenticated);
        reg.register_prefix("/api/deep", PermissionLevel::Public);
        assert_eq!(
            reg.get_permission("/api/deep/r"),
            Some(PermissionLevel::Public)
        );
        assert_eq!(
            reg.get_permission("/api/other"),
            Some(PermissionLevel::Authenticated)
        );
    }

    #[test]
    fn registry_string_prefix_semantics() {
        // get_permission 用字符串 starts_with（非段边界匹配）：
        // 前缀 "/api/person" 也命中 "/api/persons"。测试钉死该语义，
        // 注册端点时必须注册到段边界，避免短段被长段意外放行。
        let mut reg = PermissionRegistry::new();
        reg.register_prefix("/api/person", PermissionLevel::Authenticated);
        assert_eq!(
            reg.get_permission("/api/persons"),
            Some(PermissionLevel::Authenticated)
        );
    }

    #[test]
    fn registry_unregistered_paths_return_none() {
        let reg = PermissionRegistry::new();
        assert_eq!(reg.get_permission("/anything"), None);
    }

    #[test]
    fn defaults_grant_public_to_health_and_authentication_prefix() {
        let reg = PermissionRegistry::with_defaults();
        assert_eq!(reg.get_permission("/health"), Some(PermissionLevel::Public));
        assert_eq!(
            reg.get_permission("/api/authentication/login"),
            Some(PermissionLevel::Public)
        );
    }

    #[test]
    fn defaults_longer_prefix_overrides_authentication_public() {
        let reg = PermissionRegistry::with_defaults();
        // /api/authentication 是 Public，但 two_factor 段被更长前缀提升为需登录。
        assert_eq!(
            reg.get_permission("/api/authentication/two_factor/status"),
            Some(PermissionLevel::Authenticated)
        );
        // switchuser 精确注册为 Admin，压过 /api/authentication 的 Public 前缀。
        assert_eq!(
            reg.get_permission("/api/authentication/switchuser"),
            Some(PermissionLevel::Admin)
        );
    }

    #[test]
    fn defaults_person_password_requires_auth_but_icon_is_public() {
        let reg = PermissionRegistry::with_defaults();
        assert_eq!(
            reg.get_permission("/api/person/password/change"),
            Some(PermissionLevel::Authenticated)
        );
        // R8：头像端点公开（更长前缀 /api/person/icon 压过 /api/person）。
        assert_eq!(
            reg.get_permission("/api/person/icon/u-1"),
            Some(PermissionLevel::Public)
        );
    }

    #[test]
    fn defaults_module_prefixes_are_authenticated_with_api_fallback() {
        let reg = PermissionRegistry::with_defaults();
        for path in [
            "/api/ai/chat",
            "/api/cms/article/list",
            "/api/message/assemble/communicate/im/msg",
            "/api/processplatform/assemble/surface/task/list",
        ] {
            assert_eq!(
                reg.get_permission(path),
                Some(PermissionLevel::Authenticated),
                "path {path}"
            );
        }
        // 未注册任何前缀的模块路径落到 /api 兜底（需登录）。
        assert_eq!(
            reg.get_permission("/api/unknown-module/x"),
            Some(PermissionLevel::Authenticated)
        );
    }

    #[test]
    fn defaults_express_and_batch_query_prefixes_are_public() {
        let reg = PermissionRegistry::with_defaults();
        assert_eq!(
            reg.get_permission("/api/express/person/list"),
            Some(PermissionLevel::Public)
        );
        assert_eq!(
            reg.get_permission("/api/document/batch"),
            Some(PermissionLevel::Public)
        );
    }

    #[test]
    fn admin_cache_roundtrip() {
        let mut cache = AdminCache::default();
        assert_eq!(cache.get("u-1"), None);
        cache.set("u-1".to_string(), true);
        cache.set("u-2".to_string(), false);
        assert_eq!(cache.get("u-1"), Some(true));
        assert_eq!(cache.get("u-2"), Some(false));
        // 同一请求内重复查 admin 命中缓存，避免重复 DB 查询。
        assert_eq!(cache.get("u-1"), Some(true));
    }
}

#[cfg(test)]
mod security_audit_registry_tests {
    use super::*;

    /// 破坏性写端点在权限注册表中不得再是 Public（2026-10 安全审计）：
    /// 它们落到模块级 Authenticated 前缀，配合 handler 内 admin/owner 门禁。
    #[test]
    fn destructive_write_paths_are_not_public() {
        let reg = PermissionRegistry::with_defaults();
        let paths = [
            "/api/processplatform/service/processing/touch/mergeitem",
            "/api/processplatform/assemble/surface/touch/expire",
            "/api/processplatform/assemble/surface/touch/passexpired",
            "/api/processplatform/assemble/surface/touch/touchdetained",
            "/api/program_center/jest/clear/cache/source",
            "/api/program_center/cachedispatch",
            "/api/program_center/appstyle/image/application/top/erase",
            "/api/program_center/adminlogin",
            "/api/processplatform/assemble/surface/attachment/batch/delete/manage",
            "/api/ai/chat/delete/some-clue",
        ];
        for path in paths {
            assert_ne!(
                reg.get_permission(path),
                Some(PermissionLevel::Public),
                "破坏性写端点 {path} 不得注册为 Public"
            );
        }
    }

    /// 只读端点保持 Public（o2server 对齐），防止过度收紧破坏未登录读。
    #[test]
    fn readonly_paths_remain_public() {
        let reg = PermissionRegistry::with_defaults();
        for path in [
            "/api/person/list/all",
            "/api/program_center/datastructure",
            "/health",
        ] {
            assert_eq!(
                reg.get_permission(path),
                Some(PermissionLevel::Public),
                "只读端点 {path} 应保持 Public"
            );
        }
    }

    // ── 2026-10-29 覆盖率轮：角色/组/admin 判定的真库路径（空库 fail-closed + fixture 命中）──
    #[cfg(test)]
    mod role_group_tests {
        use super::*;
        use crate::testing::is_db_available;

        #[tokio::test]
        async fn role_group_admin_checks_fail_closed_then_hit_with_fixtures() {
            if !is_db_available().await {
                eprintln!("SKIP: no database available (set DATABASE_URL)");
                return;
            }
            let pool = crate::testing::test_pool();
            let client = pool.get().await.unwrap();

            // 清场 + 空库判定：未知 person 全部 fail-closed 为 false
            for sql in [
                "DELETE FROM auth_person_role WHERE person_id LIKE 'cov-rg%'",
                "DELETE FROM auth_person_group WHERE person_id LIKE 'cov-rg%'",
                "DELETE FROM auth_role WHERE id LIKE 'cov-rg%'",
                "DELETE FROM auth_group WHERE id LIKE 'cov-rg%'",
                "DELETE FROM auth_person WHERE id LIKE 'cov-rg%'",
            ] {
                client.execute(sql, &[]).await.unwrap();
            }
            assert!(!is_admin(&pool, "cov-rg-nobody").await);
            assert!(!person_has_role(&pool, "cov-rg-nobody", "cov-rg-role").await);
            assert!(!person_has_group(&pool, "cov-rg-nobody", "cov-rg-group").await);

            // fixture：人 + 角色（未删未禁）+ 组（未删未禁）
            client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('cov-rg-p1', 'cov-rg-p1', 'cov', 'x')",
                &[],
            )
            .await
            .unwrap();
            client
                .execute(
                    "INSERT INTO auth_role (id, name) VALUES ('cov-rg-role', 'cov-rg-admin')",
                    &[],
                )
                .await
                .unwrap();
            client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('cov-rg-p1', 'cov-rg-role', '')",
                &[],
            )
            .await
            .unwrap();
            client
                .execute(
                    "INSERT INTO auth_group (id, name) VALUES ('cov-rg-group', 'cov-rg-group')",
                    &[],
                )
                .await
                .unwrap();
            client
            .execute(
                "INSERT INTO auth_person_group (person_id, group_id) VALUES ('cov-rg-p1', 'cov-rg-group')",
                &[],
            )
            .await
            .unwrap();

            assert!(person_has_role(&pool, "cov-rg-p1", "cov-rg-admin").await);
            assert!(person_has_group(&pool, "cov-rg-p1", "cov-rg-group").await);
            // 未挂 admin 角色的普通人不因 fixture 而变成 admin
            assert!(!is_admin(&pool, "cov-rg-p1").await);

            // 角色禁用后 fail-closed：disable=true 的角色不再命中
            client
                .execute(
                    "UPDATE auth_role SET disable = true WHERE id = 'cov-rg-role'",
                    &[],
                )
                .await
                .unwrap();
            assert!(!person_has_role(&pool, "cov-rg-p1", "cov-rg-admin").await);

            for sql in [
                "DELETE FROM auth_person_role WHERE person_id LIKE 'cov-rg%'",
                "DELETE FROM auth_person_group WHERE person_id LIKE 'cov-rg%'",
                "DELETE FROM auth_role WHERE id LIKE 'cov-rg%'",
                "DELETE FROM auth_group WHERE id LIKE 'cov-rg%'",
                "DELETE FROM auth_person WHERE id LIKE 'cov-rg%'",
            ] {
                client.execute(sql, &[]).await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn require_owner_admin_path_bypasses_ownership() {
        use crate::testing::is_db_available;
        if !is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        let pool = crate::testing::test_pool();
        let client = pool.get().await.unwrap();
        for sql in [
            "DELETE FROM auth_person_role WHERE person_id LIKE 'cov-adm%'",
            "DELETE FROM auth_person WHERE id LIKE 'cov-adm%'",
        ] {
            client.execute(sql, &[]).await.unwrap();
        }
        // admin fixture：人 + 名为 admin 的角色（is_admin 的判定键是角色名）；
        // 若库里已有同名角色则沿用（INSERT 冲突时忽略，靠下面的 SELECT 取 id）
        client
            .execute(
                "INSERT INTO auth_role (id, name) VALUES ('cov-adm-role', 'admin') ON CONFLICT (name) DO NOTHING",
                &[],
            )
            .await
            .ok();
        client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('cov-adm-p1', 'cov-adm-p1', 'adm', 'x')",
                &[],
            )
            .await
            .unwrap();
        let role_id: String = client
            .query_one(
                "SELECT id FROM auth_role WHERE name = 'admin' ORDER BY id LIMIT 1",
                &[],
            )
            .await
            .unwrap()
            .get("id");
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('cov-adm-p1', $1, '')",
                &[&role_id],
            )
            .await
            .unwrap();

        let admin = crate::session::Session {
            token: "cov-adm-tok".to_string(),
            person_unique: "cov-adm-p1".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(1),
        };
        // admin 对「别人的行」也放行（IDOR 门禁的管理员旁路）
        require_owner(&pool, &admin, "some-other-owner")
            .await
            .unwrap();

        for sql in [
            "DELETE FROM auth_person_role WHERE person_id LIKE 'cov-adm%'",
            "DELETE FROM auth_person WHERE id LIKE 'cov-adm%'",
        ] {
            client.execute(sql, &[]).await.unwrap();
        }
    }
}
