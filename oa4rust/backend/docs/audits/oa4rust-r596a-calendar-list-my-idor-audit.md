# R596-A 日历 list/my 身份盲 IDOR 验证型甄别（2026-11-27）

## 结论
判 1 项可达真缺陷 **P1**：calendar_list_my 身份盲，未认证调用即可批量读取全部私有日历（含 createor/description）；两个实现实例均受影响且前端主路径可达。零源码改。

## 立项性质与范围
- 验证型甄别，禁止静态预判；以临时 PostgreSQL seed 两个不同 person 私有日历真测坐实。
- 目标端点：GET /api/calendar/calendar/list/my（calendar crate）。
- 第二实例：GET /api/calendar/core/entity/calendar/list/my（calendar_core_entity crate）。
- 真父 HEAD=5b6bb7fbd72058f54cab7ffa80d31ecc74ee2bd1；工作树零源码改。

## 源码锚点（逐行核实）
- crates/calendar/src/lib.rs:176-178：pub async fn calendar_list_my(pool: Extension<Pool>) —— 仅 pool，无 session/身份参数。
- 同文件:183-185：SELECT ... FROM CAL_CALENDAR WHERE status='OPEN' ORDER BY create_time DESC，绑定 &[]，无 createor/target/is_public 过滤。
- 同文件:193-212：非 UNIT 行全部 push 进 my_calendars，不做归属判定。
- 兄弟对照 crates/calendar_assemble_control/src/u2.rs:164-176：Extension<Session> + WHERE status='OPEN' AND (createor=$1 OR target=$1) 绑定 session.person_unique —— 正确过滤存在，证明本轮为真实缺陷而非设计意图。
- 第二实例 crates/calendar_core_entity/src/lib.rs:211-244：Entity::find().filter(Status=OPEN).limit(50)，同样无身份过滤。
- 主入口 crates/mcp_server/src/tool_bridge.rs:1480 注释「Auth is injected per-request by callers via request extensions」，:1485 仅 Extension(session_manager)；无全局身份过滤，list/my 自身亦未 require_session。
- 测试固化 crates/calendar/tests/is_public_null_safety.rs:88-103：断言两行（不同归属）都必须被读到，把身份盲行为写进预期。

## 决定性实测
- 临时 PG（独立容器，仅占位口令，用后即删）；建表走 shared::migrate::run_migrations 全量迁移。
- seed 两行：r596-alice（createor=alice, is_public=false, desc=alice-secret-desc）、r596-bob（createor=bob, is_public=false, desc=bob-secret-desc），均 status=OPEN、type=PERSON。
- 同一 handler 无参直调两次：
  - R596_CALL1 ids=[「r596-alice」,「r596-bob」] count=2
  - R596_CALL2 ids=[「r596-alice」,「r596-bob」] count=2
  - 两次完全相同 → identity_blind=true；单次即返回两人私有日历 → cross_person_leak=true；handler 无 session 参数且无需登录 → ungated=true。
- ALICE_ROW/BOB_ROW 均回显各自 createor 与 description 私有字段。
- cargo test -p calendar --test r596_calendar_list_my_idor_audit：1 passed 0 failed。

## 前端可达性
- apps/desktop/src/views/CalendarApp.vue:372 api.get('/api/calendar/calendar/list/my') —— 主端点前端直接调用（scope=my）。
- 同文件:351 api.get('/api/calendar/core/entity/calendar/list/my') —— 第二实例前端直接调用。
- 两实例均为用户可触达路径，非内部/运维端点。

## 三分裁决
- 可达真缺陷 P1（成立）：完全无鉴权批量读取全部私有日历（垂直+水平越权叠加），两次调用结果恒同、跨 person 互见；兄弟端点证明正确过滤存在；两实例前端均可达。取 P1 理由：无需任何身份即可枚举系统全部私有日历及 createor/description，较需登录+猜 ID 的典型 IDOR 更严重，且为批量非单条。
- 不可达 L1（排除）：真实 PG + 全量迁移下 seed 两行均被读出，非造数不可达；handler 直调与 router 挂载均坐实。
- 正常 0（排除）：非按 audience 合理分流——同库另一日历端点已实现 createor/target 归属过滤，本端点无；且测试把「两行都读到」固化为预期，属缺陷被测试锁定而非设计。
- 据实披露限制：为读端点（非写），写操作 update/remove 已有属主门（security_gate_tests.rs）；泄露字段为日历元数据（名称/描述/创建者/颜色/target），非认证凭据；未逐一实测第二实例独立 oneshot（其身份盲由逐行源码 + 前端可达坐实，未重复造数）。

## Fail Loud
- 首版夹具引入 tower::ServiceExt 编译失败（calendar crate 无 tower 依赖）：未采信，改用纯 handler 直调（更贴合「签名无身份参数」的决定性证据）。
- 首轮连本地回环端口被拒（并行团队 Docker 端口/容器名冲突，出现 role 不存在日志）：未采信，改用唯一容器名+高位端口并核验 env/角色/TCP 连通后重跑通过。
- 未做浏览器级 E2E（本轮为后端 handler 真测 + 前端源码可达核实），据此如实限定结论范围。
- 全程未见 secret/真值入库；日志/夹具为 gitignored 证据目录，夹具用后即删。

## sha256 证据（runnerSha 与文件字节哈希分列）
- runnerSha（sha256(JSON.stringify(meta))）：b21effb2fc80ed41de92e20200135330cdcedd8b63fd140c3c0c00be73aca580
- 证据日志 r596-test.log 文件字节 sha256：055cb695505a860705c7dc849c0d65a80a7bf82698fa2fd180801d333113d5e2（15 行）
- 夹具 r596_calendar_list_my_idor_audit.rs 字节 sha256：9c7d51a1227cdbdc1335299884fe76c2c908e1bf8d69a7f6888a07259669beed（用后即删）
- crates/calendar/src/lib.rs：38287351e71ec4c5ac5c2a33c9e16f9d17985eed7a30ede091ac8a5d3b3cddfc
- crates/calendar/src/routes.rs：9d112112d9704a5418ce3994399d9af126f7a2d9a5ee64b55023311de186d220
- crates/calendar_assemble_control/src/u2.rs：35055626b29b8f4347fd6d78be468e159e5e7f6283097a33dbf73deb10d3b71d
- crates/calendar_core_entity/src/lib.rs：24ec7ed9db07f3415d0e17da7c186575210f25aa37210e6368c381512d20dbbd

## 最小修复方向（不在本轮范围）
- 为 calendar_list_my 增加 Extension<Session> + require_session，SQL 追加 WHERE status='OPEN' AND (createor=$1 OR target=$1) 绑定 person_unique，对齐兄弟端点；第二实例同步；修正 is_public_null_safety.rs 规模断言为按归属过滤后预期。业务零改。
