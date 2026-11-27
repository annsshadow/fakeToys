# R607a 考勤明细接收端点归属校验修复审计

## 结论

R606a 审计判定的身份盲 IDOR 属实：`attendancedetail_recive` / `attendancedetail_reciveSingle`
两端点直接 `require_session(session)?` 丢弃会话，UPDATE 仅按 `id` 绑定，任意登录用户可凭他人明细 id 置 `received=true`。
本次修复在两端点取事件前加入与 `mobile_recive` 一致的归属门禁（会话 + 按 id 查 `person_id` + `require_owner`），
UPDATE 语句与返回语义零改。裁决 P1：越权写入他人考勤状态，非越权读/非越权删除。

## 修复范围

- 文件：`oa4rust/backend/crates/attendance_assemble_control/src/lib.rs`（仅 `attendancedetail_recive`/`attendancedetail_reciveSingle` 两端点）
- 端点1 `attendancedetail_recive`（约 :1992）：`let sess = require_session(session)?`；acquire 后、UPDATE 前按 `id` 查 `x_attendance_detail.person_id`，
  命中且非空则 `shared::middleware::require_owner(&pool, &sess, &person_id).await?`（本人/admin 放行，他人 403）；无会话 401。
- 端点2 `attendancedetail_reciveSingle`（约 :2036）：同构门禁。
- 记录不存在或 `person_id` 为空：跳过归属门，维持原 `n==0` 返回 `received=0`，不泄露存在性。
- `mobile_recive` / `mobile_id` / `detail_person_scope` / `owner_scope` / `start_check` / 其他端点 / `routes.rs` / `Cargo.toml` / migrations / 其他 crate 零改。

## 契约与验证

- 新增契约：`oa4rust/backend/crates/attendance_assemble_control/tests/r607a_attendance_recive_owner_fix.rs`（不在原始写域，作为验证门禁据实披露）。
- 真 PG（端口占位 `<TEST_HOST>:<PORT>`）scratch 库独立运行；`attendance_assemble_control_router(pool)` + `tower::ServiceExt::oneshot` + `Extension(test_session(person))`。
- 3 用例：alice 凭 bob id 调两端点→403 且 bob.received 不变；裸请求→401；alice 本人 200 且 received=1；admin 代操作 bob→200；不存在 id→200/received=0；源码守卫确认 UPDATE SQL 未变。
- RED：以真父 `lib.rs` 覆盖后定点 EXIT101（越权仍 200、source_guard 失败）；GREEN：修复版 EXIT0 3passed。
- 三 verify 全 EXIT0：V1 定点 3passed；V2 `cargo test -p attendance_assemble_control` 全量；V3 包级 `cargo fmt --check -p attendance_assemble_control` 空 + `cargo clippy -p attendance_assemble_control -- -D warnings`。
- 格式化副作用仅新增契约文件；`src/lib.rs` sha 在 `cargo fmt -p` 前后均为 `2fa0d41a...`。workspace 级 `cargo fmt --check` 存在大量真父既有差异（bbs/meeting/message/realtime/calendar 等），非本任务引入。

## 证据指纹（不含文档自身 sha）

- 真父 lib.rs sha256：`81b243bf5ecde12e9559593d04caa69672e254084a6d2cfba11abd95bc347128`
- 修复 lib.rs sha256：`2fa0d41af986e80322debdb30ec15219696910a78132aa5ebb21831132ccf047`
- 契约 sha256：`efba66a63f841b9ee5df6a4d09dcc081637c3707e62c452cfe6398c271f808c7`
- runnerSha（sha256 of compact JSON metadata）：`654d009a2e7fe333b268672da964a75ee75bba1b461b7644cef0fbd18903c3b6`
- 日志 sha256：RED `34caeff0...`、GREEN `b8e334cb...`、V1 `4996f687...`、V2 `2ad7987e...`、fmt `e3b0c442...`、clippy `a7e33517...`

## 披露

- 一次性迁移测试 `tests/r607a_migrate_once.rs` 用于初始化基础库，验证后已删除；未跑浏览器 GUI，全部断言由真 PG + HTTP oneshot + 源码守卫完成。
- 无真值入库：连接串用 `<TEST_HOST>:<PORT>` 占位，日志路径不落盘。
