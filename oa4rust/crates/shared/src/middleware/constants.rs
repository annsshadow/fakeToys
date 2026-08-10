// ──────────────────────────────────────────────────────────────────────────────
// 认证与速率限制豁免路径
//
// R12：健康检查端点及认证前置端点豁免认证，按精确路径匹配。
// 使用 `{param}` 段通配参数化端点（如验证码尺寸、凭证、OAuth code）。
// /jaxrs/reset/code|check|set 为 personal crate 的自服务密码重置流程，
// 必须在认证前置阶段可用，因此一并豁免。
// ──────────────────────────────────────────────────────────────────────────────
pub const AUTH_EXEMPT_PATHS: &[&str] = &[
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
pub const AUTH_RATE_LIMIT_EXACT: &[&str] = &["/jaxrs/authentication", "/jaxrs/authentication/login"];
pub const AUTH_RATE_LIMIT_PREFIXES: &[&str] = &[
    "/jaxrs/authentication/code",
    "/jaxrs/reset",
    "/jaxrs/secret/check",
    "/jaxrs/secret/set",
    "/jaxrs/secret/cancel",
];

// 系统初始化端点：仅当系统未初始化（auth_person 无任何未删除的未锁定用户）时豁免认证
pub const SECRET_INIT_PATHS: &[&str] = &["/jaxrs/secret/check", "/jaxrs/secret/set"];

pub const AUTH_RATE_LIMIT: i32 = 10;
pub const GENERAL_RATE_LIMIT: i32 = 100;
pub const RATE_LIMIT_WINDOW_MINUTES: i64 = 1;

/// Deprecated: these prefixes no longer require admin writes.
/// Kept for reference during migration period.
#[allow(dead_code)]
pub const ADMIN_WRITE_DEPRECATED_PREFIXES: &[&str] = &[
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

pub const ADMIN_WRITE_PREFIXES: &[&str] = &[
    "/jaxrs/person",
    "/jaxrs/unit",
    "/jaxrs/role",
    "/jaxrs/group",
    "/jaxrs/organization/person",
    "/jaxrs/organization/group",
];
