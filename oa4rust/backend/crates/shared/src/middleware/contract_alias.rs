// o2server 契约路径别名登记表（2026-10-24 R1，OA-P0-1 收口）
//
// `/api/organization/assemble/personal/*` 是 Java 契约
// （scripts/all_java_handlers.json 的 x_organization_assemble_personal，76 条）
// 的基路径；Rust 前端一直调 `/api/person/*` 与 `/api/reset/*`。
// 个人/empower/personal_extend 三个 crate 把契约路径**原样委托同一 handler**
// （路由层），本表是那份委托的权威登记，同时是中间件四张表
// （constants.rs 豁免/限流/admin、rbac.rs 权限注册表）镜像覆盖的判据来源。
//
// 两个测试消费本表：
//   - 本 crate 的 contract_alias_parity_tests（security.rs 尾部）：中间件判定 parity
//   - oa4rust 的 tests/contract_alias_parity.rs：全栈 oneshot 状态码 parity
//
// 由 Temp/r1_gen_aliases.py 从契约 JSON + 已注册路由生成，勿手改。
// 契约 76 条 − 3 条有意不映射 = 73 对：
//   - GET regist|reset/check/password/{password}（2 条）：口令入 URL 的旧形态，
//     Rust 侧已按「十类功能1 安全重设计」改 POST body，别名回 GET 是安全回退；
//   - POST custom/{name}/mockputtopost（1 条）：与 organization_assemble_personal
//     既有路由 custom/{id}/mockputtopost 同形，双注册会 axum panic。

/// (别名方法, 别名路径, 旧方法, 旧路径)
///
/// 两个方法都记：契约 PUT 常落在旧径的 POST（mockputtopost 约定）上，
/// 中间件的 admin 写判定按**各自真实方法**做。
pub const CONTRACT_ALIAS_PAIRS: &[(&str, &str, &str, &str)] = &[
    ("GET", "/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}", "GET", "/api/person/custom/manager/person/{person}/name/{name}"),
    ("PUT", "/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}", "PUT", "/api/person/custom/manager/person/{person}/name/{name}"),
    ("POST", "/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}/mockputtopost", "POST", "/api/person/custom/manager/person/{person}/name/{name}/mockputtopost"),
    ("DELETE", "/api/organization/assemble/personal/custom/{name}", "DELETE", "/api/person/custom/{name}"),
    ("GET", "/api/organization/assemble/personal/custom/{name}", "GET", "/api/person/custom/{name}"),
    ("PUT", "/api/organization/assemble/personal/custom/{name}", "PUT", "/api/person/custom/{name}"),
    ("GET", "/api/organization/assemble/personal/custom/{name}/mockdeletetoget", "GET", "/api/person/custom/{name}/mockdeletetoget"),
    ("DELETE", "/api/organization/assemble/personal/definition/{name}", "DELETE", "/api/person/definition/{name}"),
    ("GET", "/api/organization/assemble/personal/definition/{name}", "GET", "/api/person/definition/{name}"),
    ("PUT", "/api/organization/assemble/personal/definition/{name}", "PUT", "/api/person/definition/{name}"),
    ("GET", "/api/organization/assemble/personal/definition/{name}/mockdeletetoget", "GET", "/api/person/definition/{name}/mockdeletetoget"),
    ("POST", "/api/organization/assemble/personal/definition/{name}/mockputtopost", "POST", "/api/person/definition/{name}/mockputtopost"),
    ("GET", "/api/organization/assemble/personal/empower/list/person/{flag}", "GET", "/api/person/empower/list/person/{flag}"),
    ("GET", "/api/organization/assemble/personal/empower/list/{id}/next/{count}", "GET", "/api/person/empower/list/{id}/next/{count}"),
    ("GET", "/api/organization/assemble/personal/empower/list/{id}/prev/{count}", "GET", "/api/person/empower/list/{id}/prev/{count}"),
    ("GET", "/api/organization/assemble/personal/empower/manager/{id}/mockdeletetoget", "GET", "/api/person/empower/manager/{id}/mockdeletetoget"),
    ("POST", "/api/organization/assemble/personal/empower/manager/{id}/mockputtopost", "POST", "/api/person/empower/manager/{id}/mockputtopost"),
    ("GET", "/api/organization/assemble/personal/empower/{id}/mockdeletetoget", "GET", "/api/person/empower/{id}/mockdeletetoget"),
    ("POST", "/api/organization/assemble/personal/empower/{id}/mockputtopost", "POST", "/api/person/empower/{id}/mockputtopost"),
    ("POST", "/api/organization/assemble/personal/empowerlog/list/currentperson/paging/{page}/size/{size}", "POST", "/api/person/empowerlog/list/currentperson/paging/{page}/size/{size}"),
    ("POST", "/api/organization/assemble/personal/empowerlog/list/to/currentperson/paging/{page}/size/{size}", "POST", "/api/person/empowerlog/list/to/currentperson/paging/{page}/size/{size}"),
    ("GET", "/api/organization/assemble/personal/empowerlog/list/{id}/next/{count}", "GET", "/api/person/empowerlog/list/{id}/next/{count}"),
    ("GET", "/api/organization/assemble/personal/empowerlog/list/{id}/prev/{count}", "GET", "/api/person/empowerlog/list/{id}/prev/{count}"),
    ("POST", "/api/organization/assemble/personal/empowerlog/manager/list/paging/{page}/size/{size}", "POST", "/api/person/empowerlog/manager/list/paging/{page}/size/{size}"),
    ("DELETE", "/api/organization/assemble/personal/empowerlog/{id}", "DELETE", "/api/person/empowerlog/{id}"),
    ("GET", "/api/organization/assemble/personal/empowerlog/{id}/mockdeletetoget", "GET", "/api/person/empowerlog/{id}/mockdeletetoget"),
    ("GET", "/api/organization/assemble/personal/exmail", "GET", "/api/person/exmail"),
    ("POST", "/api/organization/assemble/personal/exmail", "POST", "/api/person/exmail"),
    ("GET", "/api/organization/assemble/personal/exmail/list/title/passive", "GET", "/api/person/exmail/list/title/passive"),
    ("GET", "/api/organization/assemble/personal/exmail/new/count", "GET", "/api/person/exmail/new/count"),
    ("GET", "/api/organization/assemble/personal/exmail/new/count/passive", "GET", "/api/person/exmail/new/count/passive"),
    ("GET", "/api/organization/assemble/personal/exmail/sso", "GET", "/api/person/exmail/sso"),
    ("GET", "/api/organization/assemble/personal/icon/{person}", "GET", "/api/person/icon/{person}"),
    ("PUT", "/api/organization/assemble/personal/password", "PUT", "/api/person/password"),
    ("POST", "/api/organization/assemble/personal/password/mockputtopost", "POST", "/api/person/password/mockputtopost"),
    ("GET", "/api/organization/assemble/personal/person", "GET", "/api/person"),
    ("PUT", "/api/organization/assemble/personal/person", "PUT", "/api/person"),
    ("POST", "/api/organization/assemble/personal/person/icon", "POST", "/api/person/icon"),
    ("POST", "/api/organization/assemble/personal/person/icon/mockputtopost", "POST", "/api/person/icon/mockputtopost"),
    ("POST", "/api/organization/assemble/personal/person/mockputtopost", "POST", "/api/person/mockputtopost"),
    ("PUT", "/api/organization/assemble/personal/person/password", "PUT", "/api/person/password"),
    ("POST", "/api/organization/assemble/personal/person/password/mockputtopost", "POST", "/api/person/password/mockputtopost"),
    ("POST", "/api/organization/assemble/personal/regist", "POST", "/api/person/regist"),
    ("GET", "/api/organization/assemble/personal/regist/captcha/width/{width}/height/{height}", "GET", "/api/person/regist/captcha/width/{width}/height/{height}"),
    ("GET", "/api/organization/assemble/personal/regist/check/mobile/{mobile}", "GET", "/api/person/regist/check/mobile/{mobile}"),
    ("GET", "/api/organization/assemble/personal/regist/check/name/{name}", "GET", "/api/person/regist/check/name/{name}"),
    ("GET", "/api/organization/assemble/personal/regist/code/mobile/{mobile}", "GET", "/api/person/regist/code/mobile/{mobile}"),
    ("GET", "/api/organization/assemble/personal/regist/mode", "GET", "/api/person/regist/mode"),
    ("PUT", "/api/organization/assemble/personal/reset", "PUT", "/api/reset"),
    ("GET", "/api/organization/assemble/personal/reset/check/credential/{credential}", "GET", "/api/reset/check/credential/{credential}"),
    ("GET", "/api/organization/assemble/personal/reset/code/credential/{credential}", "GET", "/api/reset/code/credential/{credential}"),
    ("POST", "/api/organization/assemble/personal/reset/mockputtopost", "POST", "/api/reset/mockputtopost"),
    ("POST", "/api/organization/assemble/personal/reset/password/anonymous", "POST", "/api/reset/password/anonymous"),
    ("GET", "/api/organization/assemble/personal/signature/delete/{id}", "GET", "/api/person/signature/delete/{id}"),
    ("GET", "/api/organization/assemble/personal/signature/list", "GET", "/api/person/signature/list"),
    ("GET", "/api/organization/assemble/personal/signature/list/person/{flag}", "GET", "/api/person/signature/list/person/{flag}"),
    ("POST", "/api/organization/assemble/personal/signature/upload", "POST", "/api/person/signature/upload"),
    ("POST", "/api/organization/assemble/personal/empower", "POST", "/api/person/empower"),
    ("GET", "/api/organization/assemble/personal/empower/list/currentperson", "GET", "/api/person/empower/list/currentperson"),
    ("GET", "/api/organization/assemble/personal/empower/list/currentperson/enable", "GET", "/api/person/empower/list/currentperson/enable"),
    ("GET", "/api/organization/assemble/personal/empower/list/to", "GET", "/api/person/empower/list/to"),
    ("GET", "/api/organization/assemble/personal/empower/list/to/enable", "GET", "/api/person/empower/list/to/enable"),
    ("POST", "/api/organization/assemble/personal/empower/manager", "POST", "/api/person/empower/manager"),
    ("POST", "/api/organization/assemble/personal/empower/manager/list/paging/{page}/size/{size}", "POST", "/api/person/empower/manager/list/paging/{page}/size/{size}"),
    ("DELETE", "/api/organization/assemble/personal/empower/manager/{id}", "DELETE", "/api/person/empower/manager/{id}"),
    ("PUT", "/api/organization/assemble/personal/empower/manager/{id}", "PUT", "/api/person/empower/manager/{id}"),
    ("DELETE", "/api/organization/assemble/personal/empower/{id}", "DELETE", "/api/person/empower/{id}"),
    ("GET", "/api/organization/assemble/personal/empower/{id}", "GET", "/api/person/empower/{id}"),
    ("PUT", "/api/organization/assemble/personal/empower/{id}", "PUT", "/api/person/empower/{id}"),
    ("GET", "/api/organization/assemble/personal/empower/{id}/disable", "GET", "/api/person/empower/{id}/disable"),
    ("GET", "/api/organization/assemble/personal/empower/{id}/enable", "GET", "/api/person/empower/{id}/enable"),
    ("GET", "/api/organization/assemble/personal/person/icon", "GET", "/api/person/icon"),
    ("PUT", "/api/organization/assemble/personal/person/icon", "PUT", "/api/person/icon"),
];
