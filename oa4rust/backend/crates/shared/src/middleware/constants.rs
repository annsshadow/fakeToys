// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// ──────────────────────────────────────────────────────────────────────────────
// 认证与速率限制豁免路径
//
// R12：健康检查端点及认证前置端点豁免认证，按精确路径匹配。
// 使用 `{param}` 段通配参数化端点（如验证码尺寸、凭证、OAuth code）。
// /api/reset/code|check|set 为 personal crate 的自服务密码重置流程，
// 必须在认证前置阶段可用，因此一并豁免。
// ──────────────────────────────────────────────────────────────────────────────
pub const AUTH_EXEMPT_PATHS: &[&str] = &[
    "/health",
    // 登录 / 登出 / 当前用户（契约路径 + 现有自造路径）
    "/api/authentication",
    "/api/authentication/login",
    "/api/authentication/logout",
    "/api/authentication/who",
    // organization_assemble_authentication 认证别名（o2server 行为对齐）
    "/api/organization/assemble/authentication/authentication",
    "/api/organization/assemble/authentication/authentication/mode",
    "/api/organization/assemble/authentication/authentication/captchaRSAPublicKey",
    "/api/organization/assemble/authentication/authentication/mockdeletetoget",
    "/api/organization/assemble/authentication/authentication/safe/logout",
    // 认证别名验证码（R8 不可解析族复核，2026-10-30）：委托 auth::code / auth::code_send，
    // 与 /api/authentication/code 同属认证前置，必须匿名可达（发送验证码）。
    "/api/organization/assemble/authentication/authentication/code",
    "/api/organization/assemble/authentication/authentication/code/credential/{credential}",
    // program_center 认证别名
    "/api/program_center/authentication",
    // 验证码
    "/api/authentication/captcha",
    "/api/authentication/captcha/width/{width}/height/{height}",
    "/api/authentication/code",
    "/api/authentication/code/credential/{credential}",
    // 绑定 / 扫码登录
    "/api/authentication/bind",
    "/api/authentication/bind/meta/{meta}",
    // 企微扫码登录回调：code 换 userid 后创建/更新人员并签发会话（A4 批定性为
    // 认证前置族，必须匿名可达；一次性 code 由企微 OAuth 侧验证，等价外部门）。
    "/api/qiyeweixin/update/person/detail/{code}",
    // OAuth 授权与回调
    "/api/authentication/oauth",
    "/api/authentication/oauth/list",
    "/api/authentication/oauth/qywx/config",
    "/api/authentication/oauth/dingding/config",
    "/api/authentication/oauth/name/{name}",
    "/api/authentication/oauth/login/qywx/code/{code}",
    "/api/authentication/oauth/login/dingding/code/{code}",
    "/api/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}",
    "/api/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}",
    // 会话刷新
    "/api/authentication/refresh",
    // 验证码校验
    "/api/secret/captcha/verify",
    // 密码重置流程
    "/api/reset/check/credential/{credential}",
    "/api/reset/check/password/{password}",
    "/api/reset/code/credential/{credential}",
    "/api/reset",
    "/api/reset/password/anonymous",
    // personal crate 自服务密码重置流程（认证前置阶段）
    "/api/reset/code",
    "/api/reset/check",
    "/api/reset/set",
    // ── o2server 契约路径别名豁免镜像（2026-10-24 R1，OA-P0-1 收口）────────
    // /api/organization/assemble/personal/* 是 Java 契约基路径，Rust 侧由
    // personal / empower / personal_extend 三个 crate 的 contract alias 路由
    // 原样委托同一 handler（旧路径 /api/person/*、/api/reset/* 行为零变化）。
    // 以下每条都是旧路径豁免状态的镜像：旧路径匿名可达，别名必须同权，
    // 否则契约客户端（匿名注册/重置/自定义字段读取）会平白吃 401。
    // 双锁一致性（豁免 ⟺ 注册表 Public）见 rbac.rs 同批补齐的 6 条前缀。
    // ⚠️ 不含 custom/{name}/mockdeletetoget 与 definition/{name}/mockdeletetoget：
    // 这两条旧径本身就没进豁免表（要登录），别名跟着要登录——给别名加豁免
    // 会让它比旧径更宽松，那是弱化不是对齐。
    "/api/organization/assemble/personal/reset",
    "/api/organization/assemble/personal/reset/check/credential/{credential}",
    "/api/organization/assemble/personal/reset/code/credential/{credential}",
    "/api/organization/assemble/personal/reset/password/anonymous",
    "/api/organization/assemble/personal/regist",
    "/api/organization/assemble/personal/custom/{name}",
    "/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}",
    "/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}/mockputtopost",
    "/api/organization/assemble/personal/definition/{name}",
    "/api/organization/assemble/personal/definition/{name}/mockputtopost",
    "/api/organization/assemble/personal/icon/{person}",
    "/api/organization/assemble/personal/person/icon",
    "/api/organization/assemble/personal/person/icon/mockputtopost",
    // 系统初始化（仅系统未初始化时；见 system_uninitialized）
    "/api/secret/check",
    "/api/secret/set",
    "/api/secret/set/cancel",
    // 注册（匿名访问）
    "/api/person/regist",
    // group queries
    "/api/group/has/role",
    "/api/group/list",
    "/api/group/list/group/sub/direct",
    "/api/group/list/group/sub/direct/object",
    "/api/group/list/group/sub/nested",
    "/api/group/list/group/sub/nested/object",
    "/api/group/list/group/sup/direct",
    "/api/group/list/group/sup/direct/object",
    "/api/group/list/group/sup/nested",
    "/api/group/list/group/sup/nested/object",
    "/api/group/list/group/tree",
    "/api/group/list/identity",
    "/api/group/list/identity/object",
    "/api/group/list/object",
    "/api/group/list/person",
    "/api/group/list/person/object",
    // person queries
    "/api/person/custom/{name}",
    "/api/person/custom/manager/person/{person}/name/{name}",
    "/api/person/custom/manager/person/{person}/name/{name}/mockputtopost",
    "/api/person/definition/{name}",
    "/api/person/definition/{name}/mockputtopost",
    "/api/person/detail/{flag}",
    "/api/person/has/role",
    "/api/person/list",
    "/api/person/list/group",
    "/api/person/list/group/object",
    "/api/person/list/identity",
    "/api/person/list/identity/object",
    "/api/person/list/login/after",
    "/api/person/list/login/after/object",
    "/api/person/list/login/recent",
    "/api/person/list/login/recent/object",
    "/api/person/list/object",
    "/api/person/list/pair/identity",
    "/api/person/list/person/sub/direct",
    "/api/person/list/person/sub/direct/object",
    "/api/person/list/person/sub/nested",
    "/api/person/list/person/sub/nested/object",
    "/api/person/list/person/sup/direct",
    "/api/person/list/person/sup/direct/object",
    "/api/person/list/person/sup/nested",
    "/api/person/list/person/sup/nested/object",
    "/api/person/list/role",
    "/api/person/list/role/object",
    "/api/person/list/unit/sub/direct",
    "/api/person/list/unit/sub/direct/like",
    "/api/person/list/unit/sub/direct/like/object",
    "/api/person/list/unit/sub/direct/object",
    "/api/person/list/unit/sub/nested",
    "/api/person/list/unit/sub/nested/like",
    "/api/person/list/unit/sub/nested/like/object",
    "/api/person/list/unit/sub/nested/object",
    // role queries
    "/api/role/list",
    "/api/role/list/object",
    "/api/role/list/person",
    "/api/role/list/person/object",
    // unit queries
    "/api/unit/check/unit/has/identity",
    "/api/unit/check/unit/has/person",
    "/api/unit/check/unit/has/unit",
    "/api/unit/identity/level",
    "/api/unit/identity/level/object",
    "/api/unit/identity/type",
    "/api/unit/identity/type/object",
    "/api/unit/list/identity",
    "/api/unit/list/identity/object",
    "/api/unit/list/identity/sup/nested",
    "/api/unit/list/identity/sup/nested/object",
    "/api/unit/list/level",
    "/api/unit/list/level/name/object",
    "/api/unit/list/level/object",
    "/api/unit/list/object",
    "/api/unit/list/person",
    "/api/unit/list/person/object",
    "/api/unit/list/person/sup/nested",
    "/api/unit/list/person/sup/nested/object",
    "/api/unit/list/types",
    "/api/unit/list/types/object",
    "/api/unit/list/unitduty",
    "/api/unit/list/unitduty/object",
    // R401J200 豁免扩展：只读查询端点
    // attendance 只读查询（无需认证）
    "/api/attendance/assemble/control/attendanceworkplancalendar",
    "/api/attendance/assemble/control/attendanceworkplancalendar/list",
    "/api/attendance/assemble/control/attendanceconfig",
    "/api/attendance/assemble/control/attendanceconfig/list",
    "/api/attendance/assemble/control/attendancestatistical",
    "/api/attendance/assemble/control/attendancestatistical/total",
    // processplatform 只读查询（工作统计、列表）
    "/api/processplatform/assemble/surface/work/count",
    "/api/processplatform/assemble/surface/work/count/application/{applicationFlag}",
    "/api/processplatform/assemble/surface/work/count/process/{processFlag}",
    "/api/processplatform/assemble/surface/work/list",
    "/api/processplatform/assemble/surface/work/list/identity/{identity}",
    "/api/processplatform/assemble/surface/work/list/person/{person}",
    "/api/processplatform/assemble/surface/work/list/unit/{unit}",
    "/api/processplatform/assemble/surface/work/list/application/{applicationFlag}",
    "/api/processplatform/assemble/surface/work/list/process/{processFlag}",
    // bbs 只读查询（论坛、帖子）
    "/api/bbs/assemble/control/bbsforum",
    "/api/bbs/assemble/control/bbsforum/list",
    "/api/bbs/assemble/control/bbsforum/{id}",
    "/api/bbs/assemble/control/bbstopic",
    "/api/bbs/assemble/control/bbstopic/list",
    "/api/bbs/assemble/control/bbstopic/{id}",
    // meeting 只读查询（会议列表、详情）
    "/api/meeting/assemble/control/meeting/list",
    "/api/meeting/assemble/control/meeting/coming",
    "/api/meeting/assemble/control/meeting/coming/list",
    "/api/meeting/assemble/control/meeting/calendar",
    "/api/meeting/assemble/control/meeting/calendar/list",
    // message 只读查询（消息列表）
    "/api/message/assemble/communicate/message",
    "/api/message/assemble/communicate/message/list",
    "/api/message/assemble/communicate/message/{id}",
    "/api/message_assemble_communicate/api/message",
    "/api/message_assemble_communicate/api/message/list",
    "/api/message_assemble_communicate/api/message/{id}",
    // file 只读查询（文件信息）
    "/api/file/assemble/control/fileinfo",
    "/api/file/assemble/control/fileinfo/list",
    "/api/file/assemble/control/fileinfo/{id}",
    // query 只读查询（查询设计、视图）
    "/api/queryview",
    "/api/queryview/{id}",
    "/api/queryview/table/list/{id}",
    "/api/queryview/stat/list",
    // cms 只读查询（内容管理）
    "/api/cms/assemble/control/cmsdocument",
    "/api/cms/assemble/control/cmsdocument/list",
    "/api/cms/assemble/control/cmsdocument/{id}",
    "/api/cms/assemble/control/cmsview",
    "/api/cms/assemble/control/cmsview/list",
    // general 只读查询
    "/api/general/assemble/control/invoice",
    "/api/general/assemble/control/invoice/list",
    "/api/general/assemble/control/invoice/{id}",
    // neural 只读查询
    "/api/neural",
    "/api/neural/list",
    "/api/neural/{id}",
    // ── plan006 U5 扩展：R401J200 豁免追加 ──
    // processplatform 扩展：附件/文件/可用流程/可用身份/字典
    "/api/processplatform/assemble/surface/attachment/list/task/{taskId}",
    "/api/processplatform/assemble/surface/work/list/application/{applicationFlag}/process/{processFlag}",
    "/api/processplatform/assemble/surface/work/list/process/{processFlag}/person/{person}",
    "/api/processplatform/assemble/surface/available/work/identity",
    "/api/processplatform/assemble/surface/available/work/person",
    "/api/processplatform/assemble/surface/available/work/unit",
    "/api/processplatform/assemble/surface/available/work/application/{applicationFlag}",
    "/api/processplatform/assemble/surface/available/work/process/{processFlag}",
    "/api/processplatform/assemble/surface/work/list/activity/{activityId}",
    "/api/processplatform/assemble/surface/application/list",
    "/api/processplatform/assemble/surface/process/list/person/{person}",
    "/api/processplatform/assemble/surface/process/list/unit/{unit}",
    "/api/processplatform/assemble/surface/task/list/identity/{identity}",
    "/api/processplatform/assemble/surface/task/list/person/{person}",
    "/api/processplatform/assemble/surface/task/list/unit/{unit}",
    "/api/processplatform/assemble/surface/task/list/activity/{activityId}",
    "/api/processplatform/assemble/surface/task/list/all",
    "/api/processplatform/assemble/surface/workorworkcompleted/list",
    "/api/processplatform/assemble/surface/workorworkcompleted/list/identity/{identity}",
    "/api/processplatform/assemble/surface/workorworkcompleted/list/person/{person}",
    "/api/processplatform/assemble/surface/workorworkcompleted/list/unit/{unit}",
    "/api/processplatform/assemble/surface/workorworkcompleted/list/application/{applicationFlag}",
    "/api/processplatform/assemble/surface/workorworkcompleted/list/process/{processFlag}",
    "/api/processplatform/assemble/surface/archive/list/identity/{identity}",
    "/api/processplatform/assemble/surface/archive/list/person/{person}",
    "/api/processplatform/assemble/surface/archive/list/unit/{unit}",
    "/api/processplatform/assemble/surface/archive/list/application/{applicationFlag}",
    "/api/processplatform/assemble/surface/archive/list/process/{processFlag}",
    // cms 只读查询扩展
    "/api/cms/assemble/control/cmsapplication",
    "/api/cms/assemble/control/cmsapplication/list",
    "/api/cms/assemble/control/cmsapplication/{id}",
    "/api/cms/assemble/control/cmscategory",
    "/api/cms/assemble/control/cmscategory/list",
    "/api/cms/assemble/control/cmscategory/{id}",
    "/api/cms/assemble/control/cmsdocumentcategory/list",
    "/api/cms/assemble/control/cmspagedocument/view/{viewId}/category/{categoryId}",
    "/api/cms/assemble/control/cmspagedocument/view/{viewId}",
    // file 只读查询扩展
    "/api/file/assemble/control/file/list/folder/{folderId}",
    "/api/file/assemble/control/file/list/person/{personId}",
    "/api/file/assemble/control/file/list/unit/{unitId}",
    "/api/file/assemble/control/file/list/recent",
    "/api/file/assemble/control/file/list/type/{type}",
    "/api/file/assemble/control/folder/list/{parentId}",
    "/api/file/assemble/control/folder/list/root",
    "/api/file/assemble/control/file/reference/list/{referenceId}",
    // queryview 只读查询扩展
    "/api/queryview/list/query/{queryFlag}",
    "/api/queryview/list/application/{applicationFlag}",
    "/api/queryview/view/list",
    "/api/queryview/view/list/application/{applicationFlag}",
    "/api/queryview/table/list/{id}/list/{listId}/prev/{count}",
    "/api/queryview/table/list/{id}/list/{listId}/next/{count}",
    "/api/queryview/search/list/{viewFlag}/query/{queryFlag}",
    // calendar 只读查询
    "/api/calendar/assemble/control/event",
    "/api/calendar/assemble/control/event/list",
    "/api/calendar/assemble/control/event/{id}",
    "/api/calendar/assemble/control/event/list/person/{personId}",
    "/api/calendar/assemble/control/event/list/unit/{unitId}",
    "/api/calendar/assemble/control/event/list/range/{start}/{end}",
    "/api/calendar/assemble/control/calendar",
    "/api/calendar/assemble/control/calendar/list",
    "/api/calendar/assemble/control/calendar/{id}",
    "/api/calendar/assemble/control/calendar/person/{personId}",
    "/api/calendar/assemble/control/period/list",
    "/api/calendar/assemble/control/period/list/person/{personId}",
    "/api/calendar/assemble/control/period/list/unit/{unitId}",
    "/api/calendar/assemble/control/period/list/application/{applicationFlag}",
    // meeting 只读查询扩展
    "/api/meeting/assemble/control/meeting/{id}/list",
    "/api/meeting/assemble/control/meeting/list/person/{personId}",
    "/api/meeting/assemble/control/meeting/list/unit/{unitId}",
    "/api/meeting/assemble/control/meeting/list/room/{roomId}",
    "/api/meeting/assemble/control/meeting/room",
    "/api/meeting/assemble/control/meeting/room/list",
    "/api/meeting/assemble/control/meeting/room/{id}",
    "/api/meeting/assemble/control/mroom/list",
    "/api/meeting/assemble/control/mroom/list/identity/{identity}",
    // bbs 只读查询扩展
    "/api/bbs/assemble/control/bbsforum/{id}/topic/list",
    "/api/bbs/assemble/control/bbstopic/{id}/reply/list",
    "/api/bbs/assemble/control/bbstopic/list/forum/{forumId}",
    "/api/bbs/assemble/control/bbstopic/list/person/{personId}",
    "/api/bbs/assemble/control/bbstopic/list/recent",
    "/api/bbs/assemble/control/bbsreply/list/topic/{topicId}",
    "/api/bbs/assemble/control/bbsreply/list/person/{personId}",
    "/api/bbs/assemble/control/bbsreply/list/recent",
    // message 只读查询扩展
    "/api/message/assemble/communicate/message/list/person/{personId}",
    "/api/message/assemble/communicate/message/list/sender/{senderId}",
    "/api/message/assemble/communicate/message/list/type/{type}",
    "/api/message/assemble/communicate/message/list/recent",
    "/api/message/assemble/communicate/message/list/unread",
    "/api/message/assemble/communicate/instant/list",
    "/api/message/assemble/communicate/instant/list/person/{personId}",
    "/api/message/assemble/communicate/instant/list/sender/{senderId}",
    "/api/message/assemble/communicate/instant/list/recent",
    "/api/message/assemble/communicate/instant/list/unread",
    "/api/message/assemble/communicate/mass/list",
    "/api/message/assemble/communicate/mass/list/person/{personId}",
    "/api/message/assemble/communicate/mass/list/sender/{senderId}",
    "/api/message/assemble/communicate/mass/list/type/{type}",
    "/api/message/assemble/communicate/mass/list/recent",
    // program_center 只读查询
    "/api/program_center/applications",
    "/api/program_center/applications/list",
    // NOTE: /api/program_center/application/{id} intentionally NOT listed here —
    // the {id} wildcard matches any segment (e.g. "create"), incorrectly exempting
    // write endpoints. Use the permission registry (rbac.rs) which has a proper
    // prefix entry for program_center (Authenticated) instead.
    "/api/program_center/center/applications",
    "/api/program_center/center/applications/list",
    "/api/program_center/application/{id}/deploy",
    "/api/program_center/application/{id}/deploy/list",
    "/api/program_center/deploy/list",
    "/api/program_center/deploy/list/application/{applicationFlag}",
    "/api/program_center/deploy/list/person/{personId}",
    "/api/program_center/deploy/list/unit/{unitId}",
    "/api/program_center/deploy/list/application/{applicationFlag}/person/{personId}",
    "/api/program_center/deploy/list/application/{applicationFlag}/unit/{unitId}",
    "/api/program_center/session/list/application/{applicationFlag}",
    "/api/program_center/session/list/person/{personId}",
    "/api/program_center/session/list/unit/{unitId}",
    "/api/program_center/session/list/all",
    // general 只读查询扩展
    "/api/general/assemble/control/invoice/list/person/{personId}",
    "/api/general/assemble/control/invoice/list/unit/{unitId}",
    "/api/general/assemble/control/invoice/list/application/{applicationFlag}",
    "/api/general/assemble/control/securityclearance/list",
    "/api/general/assemble/control/securityclearance/list/person/{personId}",
    "/api/general/assemble/control/securityclearance/list/unit/{unitId}",
    "/api/general/assemble/control/securityclearance/list/type/{type}",
];

// 认证类端点（计入 10 次/分钟/IP 的认证限流）。
// 精确限流仅限"明文凭证尝试"：登录契约路径（POST /api/authentication）
// 与其自造别名（/api/authentication/login）；以及密码重置与初始化端点（前缀）。
pub const AUTH_RATE_LIMIT_EXACT: &[&str] = &[
    "/api/authentication",
    "/api/authentication/login",
    // 四轮 §三.7：注册与验证码校验显式公开后必须限流，否则匿名侧变成
    // 无上限批量建号 / 验证码暴力枚举（rate_limit_middleware 对「豁免且未列限流」直接跳过）。
    "/api/person/regist",
    // 契约别名同限流（2026-10-24 R1）：匿名面等权，防批量建号/验证码枚举
    "/api/organization/assemble/personal/regist",
    "/api/secret/captcha/verify",
];
pub const AUTH_RATE_LIMIT_PREFIXES: &[&str] = &[
    "/api/authentication/code",
    "/api/authentication/two",        // 双因素登录
    "/api/authentication/safe",       // 安全注销
    "/api/authentication/switchuser", // 用户切换
    "/api/person/regist/code",        // 注册验证码发送
    // 认证别名验证码同限流（2026-10-30）：委托 auth::code，匿名可达必须防枚举
    "/api/organization/assemble/authentication/authentication/code",
    // 契约别名同限流（2026-10-24 R1）
    "/api/organization/assemble/personal/regist/code",
    "/api/organization/assemble/personal/reset",
    "/api/reset",
    "/api/secret/check",
    "/api/secret/set",
    "/api/secret/cancel",
    // express 批量查询：无认证但需速率限制防枚举
    "/api/express/person",
    "/api/express/unit",
    "/api/express/identity",
    "/api/express/group",
    "/api/express/role",
];

// 系统初始化端点：仅当系统未初始化（auth_person 无任何未删除的未锁定用户）时豁免认证
// 四轮 §三.7 补齐销毁侧：`path_matches` 按段数相等匹配，`/api/secret/set` 不吞
// `/api/secret/set/cancel`，所以少列一条就等于让「删掉初始化密钥」这一 GET 永久匿名可达，
// 只剩 handler 体内 `has_person` 一道锁（全库用户锁定/软删的维护态下那道锁也会失效）。
pub const SECRET_INIT_PATHS: &[&str] = &[
    "/api/secret/check",
    "/api/secret/set",
    "/api/secret/set/cancel",
];

// ── 速率限制（轮72）────────────────────────────────────────────────────
// 历史上 AUTH/GENERAL 均为 10000/分钟（等于不设防），与注释「10 次/分钟」不符，
// 登录暴力破解防护实际失效。现为真实限额：认证端点 30 次/分钟/IP（登录有验证码
// 兜底，E2E/联调多次登录仍有余量），普通端点 300 次/分钟/IP。可用环境变量覆盖
// （压测/应急放开），未设置或非法值时用默认。
pub fn auth_rate_limit() -> i32 {
    env_var_or("OA4RUST_AUTH_RATE_LIMIT", 30)
}

pub fn general_rate_limit() -> i32 {
    env_var_or("OA4RUST_GENERAL_RATE_LIMIT", 300)
}

fn env_var_or(name: &str, default: i32) -> i32 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&v| v > 0)
        .unwrap_or(default)
}

pub const RATE_LIMIT_WINDOW_MINUTES: i64 = 1;

/// Deprecated: these prefixes no longer require admin writes.
/// Kept for reference during migration period.
#[allow(dead_code)]
pub const ADMIN_WRITE_DEPRECATED_PREFIXES: &[&str] = &[
    "/api/ai",
    "/api/cms",
    "/api/correlation",
    "/api/file",
    "/api/program_center",
    "/api/query",
    "/api/bbs",
    "/api/meeting",
    "/api/message",
    "/api/processplatform",
    "/api/portal",
];

pub const ADMIN_WRITE_PREFIXES: &[&str] = &[
    "/api/person",
    "/api/unit",
    "/api/role",
    "/api/group",
    "/api/organization/person",
    "/api/organization/group",
    // o2server 契约路径别名（2026-10-24 R1）：/api/organization/assemble/personal/*
    // 的写操作与旧路径 /api/person/* 同权——旧径写要 admin，别名也要，
    // 否则契约客户端可绕过 admin 门禁（自服务的改密/头像两支由
    // requires_admin 内的例外条款同权豁免，见 security.rs）。
    "/api/organization/assemble/personal",
];

// plan006 U5 前缀豁免：覆盖带动态路径段的只读查询端点
// 与 PERMISSION_REGISTRY Public 前缀对齐，确保 auth_middleware 豁免生效
pub const AUTH_EXEMPT_PREFIXES: &[&str] = &[
    "/api/processplatform/assemble/surface/application/list",
    "/api/processplatform/assemble/surface/control/workorworkcompleted",
    "/api/processplatform/assemble/surface/readrecord/list",
    "/api/processplatform/assemble/surface/record/list",
    "/api/processplatform/assemble/surface/sign/list",
    "/api/processplatform/assemble/surface/work/count",
    "/api/program_center/datastructure",
    "/api/program_center/dingding/get/callback",
    "/api/program_center/market/cloud/unit/is/vip",
    "/api/program_center/market/installed/version",
    "/api/person/list/all",
    "/api/unit/list/all",
    "/api/person/nick/name",
    "/api/calendar/assemble/control/calendar/follow",
    "/api/attendance/assemble/control/dingdingstatistic",
    "/api/attendance/assemble/control/qywxstatistic",
    "/api/meeting/assemble/control/config/system",
    "/api/person/icon",
];

// ── 豁免排除列表（A5 批，2026-10-29）─────────────────────────────────────────
// 通配段碰撞：下面这些路径会被上方含 `{param}` 的豁免条目「本意给只读详情的
// {id}」错误地匿名放行（如 `/api/general/.../invoice/{id}` 把 `/invoice/create`
// 的 create 当成 id 段）。它们要么是写路由（体内已有门，但中间件层应先拦住），
// 要么是应该走注册表 Public 判定而非通配豁免的只读端点。逐条精确点名排除，
// 不让通配豁免成为第二条匿名入口。
pub const AUTH_EXEMPT_BLOCKED_PATHS: &[&str] = &[
    // general invoice：写路由，豁免只应覆盖 invoice/{id} 详情读
    "/api/general/assemble/control/invoice/create",
    "/api/general/assemble/control/invoice/upload",
    // meeting：写路由，豁免只应覆盖 meeting/{id} 详情读
    "/api/meeting/assemble/control/meeting/create",
    // queryview：写方法但只读的查询端点，匿名可达只应来自 rbac Public 注册，
    // 不应靠 `/api/queryview/{id}` 的通配碰撞
    "/api/queryview/search",
    "/api/queryview/morelikethis",
    // queryview/list：B6 惰性豁免条目——注册表 Authenticated 要求登录，
    // 删除显式豁免后仍被 `{id}` 通配覆盖，精确阻断使「豁免失效」与注册表一致。
    "/api/queryview/list",
];
