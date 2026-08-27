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
    // 注册（匿名访问）
    "/jaxrs/person/regist",
    // 行为对比测试：Java 侧无需认证/管理权限的只读查询端点（Rust 应豁免）
    // attendance assemble control
    "/jaxrs/attendance/assemble/control/dingding/all",
    "/jaxrs/attendance/assemble/control/qywx/all",
    "/jaxrs/attendance/assemble/control/statistic/do",
    "/jaxrs/attendance/assemble/control/attendancestatisticalcycle",
    // document / file
    "/jaxrs/document/batch/{id}",
    "/jaxrs/file/assemble/control/file/referencetype/{referenceType}/reference/{reference}",
    // group queries
    "/jaxrs/group/has/role",
    "/jaxrs/group/list",
    "/jaxrs/group/list/group/sub/direct",
    "/jaxrs/group/list/group/sub/direct/object",
    "/jaxrs/group/list/group/sub/nested",
    "/jaxrs/group/list/group/sub/nested/object",
    "/jaxrs/group/list/group/sup/direct",
    "/jaxrs/group/list/group/sup/direct/object",
    "/jaxrs/group/list/group/sup/nested",
    "/jaxrs/group/list/group/sup/nested/object",
    "/jaxrs/group/list/group/tree",
    "/jaxrs/group/list/identity",
    "/jaxrs/group/list/identity/object",
    "/jaxrs/group/list/object",
    "/jaxrs/group/list/person",
    "/jaxrs/group/list/person/object",
    // meeting
    "/jaxrs/meeting/assemble/control/config",
    // person queries
    "/jaxrs/person/custom/{name}",
    "/jaxrs/person/custom/manager/person/{person}/name/{name}",
    "/jaxrs/person/custom/manager/person/{person}/name/{name}/mockputtopost",
    "/jaxrs/person/definition/{name}",
    "/jaxrs/person/definition/{name}/mockputtopost",
    "/jaxrs/person/detail/{flag}",
    "/jaxrs/person/has/role",
    "/jaxrs/person/list",
    "/jaxrs/person/list/group",
    "/jaxrs/person/list/group/object",
    "/jaxrs/person/list/identity",
    "/jaxrs/person/list/identity/object",
    "/jaxrs/person/list/login/after",
    "/jaxrs/person/list/login/after/object",
    "/jaxrs/person/list/login/recent",
    "/jaxrs/person/list/login/recent/object",
    "/jaxrs/person/list/object",
    "/jaxrs/person/list/pair/identity",
    "/jaxrs/person/list/person/sub/direct",
    "/jaxrs/person/list/person/sub/direct/object",
    "/jaxrs/person/list/person/sub/nested",
    "/jaxrs/person/list/person/sub/nested/object",
    "/jaxrs/person/list/person/sup/direct",
    "/jaxrs/person/list/person/sup/direct/object",
    "/jaxrs/person/list/person/sup/nested",
    "/jaxrs/person/list/person/sup/nested/object",
    "/jaxrs/person/list/role",
    "/jaxrs/person/list/role/object",
    "/jaxrs/person/list/unit/sub/direct",
    "/jaxrs/person/list/unit/sub/direct/like",
    "/jaxrs/person/list/unit/sub/direct/like/object",
    "/jaxrs/person/list/unit/sub/direct/object",
    "/jaxrs/person/list/unit/sub/nested",
    "/jaxrs/person/list/unit/sub/nested/like",
    "/jaxrs/person/list/unit/sub/nested/like/object",
    "/jaxrs/person/list/unit/sub/nested/object",
    // processplatform
    "/jaxrs/processplatform/assemble/surface/attachment/batch/delete/manage",
    // program_center
    "/jaxrs/program_center/adminlogin",
    "/jaxrs/program_center/appstyle",
    "/jaxrs/program_center/cachedispatch",
    // role queries
    "/jaxrs/role/list",
    "/jaxrs/role/list/object",
    "/jaxrs/role/list/person",
    "/jaxrs/role/list/person/object",
    // script
    "/jaxrs/script/list/manager",
    // unit queries
    "/jaxrs/unit/check/unit/has/identity",
    "/jaxrs/unit/check/unit/has/person",
    "/jaxrs/unit/check/unit/has/unit",
    "/jaxrs/unit/identity/level",
    "/jaxrs/unit/identity/level/object",
    "/jaxrs/unit/identity/type",
    "/jaxrs/unit/identity/type/object",
    "/jaxrs/unit/list/identity",
    "/jaxrs/unit/list/identity/object",
    "/jaxrs/unit/list/identity/sup/nested",
    "/jaxrs/unit/list/identity/sup/nested/object",
    "/jaxrs/unit/list/level",
    "/jaxrs/unit/list/level/name/object",
    "/jaxrs/unit/list/level/object",
    "/jaxrs/unit/list/object",
    "/jaxrs/unit/list/person",
    "/jaxrs/unit/list/person/object",
    "/jaxrs/unit/list/person/sup/nested",
    "/jaxrs/unit/list/person/sup/nested/object",
    "/jaxrs/unit/list/types",
    "/jaxrs/unit/list/types/object",
    "/jaxrs/unit/list/unitduty",
    "/jaxrs/unit/list/unitduty/object",
];

// 认证类端点（计入 10 次/分钟/IP 的认证限流）。
// 精确限流仅限"明文凭证尝试"：登录契约路径（POST /jaxrs/authentication）
// 与其自造别名（/jaxrs/authentication/login）；以及密码重置与初始化端点（前缀）。
pub const AUTH_RATE_LIMIT_EXACT: &[&str] = &["/jaxrs/authentication", "/jaxrs/authentication/login"];
pub const AUTH_RATE_LIMIT_PREFIXES: &[&str] = &[
    "/jaxrs/authentication/code",
    "/jaxrs/authentication/two",     // 双因素登录
    "/jaxrs/authentication/safe",    // 安全注销
    "/jaxrs/authentication/switchuser", // 用户切换
    "/jaxrs/person/regist/code",    // 注册验证码发送
    "/jaxrs/reset",
    "/jaxrs/secret/check",
    "/jaxrs/secret/set",
    "/jaxrs/secret/cancel",
    // express 批量查询：无认证但需速率限制防枚举
    "/jaxrs/express/person",
    "/jaxrs/express/unit",
    "/jaxrs/express/identity",
    "/jaxrs/express/group",
    "/jaxrs/express/role",
];

// 系统初始化端点：仅当系统未初始化（auth_person 无任何未删除的未锁定用户）时豁免认证
pub const SECRET_INIT_PATHS: &[&str] = &["/jaxrs/secret/check", "/jaxrs/secret/set"];

pub const AUTH_RATE_LIMIT: i32 = 10000;
pub const GENERAL_RATE_LIMIT: i32 = 10000;
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
