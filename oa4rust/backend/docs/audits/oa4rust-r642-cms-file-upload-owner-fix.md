# R642 CMS file_id_upload 跨人覆盖文件正文真修复审计

## 1. 结论
判级沿用 R641a 甄别结论 P1（认证后跨人覆盖他人文件正文，可篡改内容，属越权写；非匿名可达需有效会话+已知文件 id，不预支 P0）。本轮为真修复，非新判级。
基线与真父：abdf07e5e1a6488136e823941d1fc407f9e085a5。

## 2. 锚点与根因
目标 handler：`oa4rust/backend/crates/cms_assemble_control/src/lib.rs` 的 `file_id_upload`（原 4229-4257 行）。
根因一：原 4237 行 `require_session(session)?;` 丢弃返回的 Session，仅验证会话存在，不取身份，无法判属主。
根因二：原 4251 行 upsert `INSERT INTO x_cms_file (id, name, content_base64, content_type, size) ... ON CONFLICT (id) DO UPDATE` 不写 creator 列；x_cms_file 属主不可判，任意已登录会话可覆盖任意文件正文。
兄弟门（已存在）：`u2_check_owner`（约 8580 行）返回三态 `U2Gate::{NotFound,Forbidden,Allowed}`，按 `creator` 列比对会话人 unique_id 或 admin 放行；范本 `file_u2_update`（约 9408 行）已用该门做 Forbidden→403 分流。
DDL：`migrations/023_create_cms_assemble_tables.sql` 约 57-67 行，x_cms_file 含 creator VARCHAR(255)，id 主键支持 ON CONFLICT。
路由：`routes.rs` 约 661 行 `.route(/api/file/{id}/upload, post(file_id_upload))`，经 `cms_assemble_control_router` 可达。

## 3. 修复内容（仅 lib.rs 本 handler）
1) 保留会话：把丢弃式调用改为 `let sess = require_session(session)?;`，`sess.person_unique` 供属主判定。
2) 首个 DB 写前加属主门：`match u2_check_owner(&pool, x_cms_file, creator, &id, &sess.person_unique).await?`；`U2Gate::Forbidden => return Err(AppError::Forbidden)`（403），`NotFound`/`Allowed` 放行（NotFound 放行以支持新 id 创建）。
3) INSERT 补 creator 列：`... size, creator) VALUES ($1..$6)`，末位绑定 `&sess.person_unique`；`ON CONFLICT (id) DO UPDATE` 不 SET creator，保留原属主不变。
4) SQL/回传形状不变（仍 RETURNING * → row_to_json）。
严格限定写域：out of scope 的同构裸写端点 `file_id_mockputtopost`（约 4201 行，连 session 参数都没有）本轮未改，作为 R643 线索据实披露（见第 7 节）。

## 4. 真执行证据（RED → GREEN）
测试基建：自建独占 PostgreSQL 容器，scratch 库 + `shared::migrate::run_migrations` + seed auth_person/auth_role/auth_person_role + x_cms_file 行，构造 `shared::testing::test_session(person)` 经 `Router::oneshot` 注入 Extension 直调 handler。CONNECTION 用占位：<TEST_HOST> / <PORT> / <USER> / <PASSWORD>，无真值入库。
临时契约：`tests/r642_cms_file_upload_owner_fix_contract.rs`（本轮 implementation 契约入库），1 个 #[tokio::test] `r642_file_upload_owner_gate` 五臂 + 源码守卫。
RED（修复前）：`assertion left == right failed: R642 arm1 cross-user overwrite must be 403, left: 200, right: 403`，`test result: FAILED. 0 passed; 1 failed`，EXIT=101 —— 跨人覆写成功返回 200，缺陷坐实。
GREEN（修复后）：`R642 five arms OK: cross-user 403+unchanged, owner 200+retained, new 200+creator, anon 401+no-write, source guards passed`，`test result: ok. 1 passed; 0 failed`，EXIT=0。
五臂明细：①alice 覆写 bob 文件 → 403 且 DB content/creator 未改；②bob 覆写自己 → 200 且 content 更新、creator 保留；③alice 新建 id → 200 且 creator=alice；④匿名无 session → 401 且未写库；⑤源码守卫断言保留 Session、u2_check_owner 三态门、INSERT 含 creator 且 ON CONFLICT 段不 SET creator。

## 5. 验证矩阵
V1 定点 `cargo test -p cms_assemble_control --test r642_cms_file_upload_owner_fix_contract`：1 passed / 0 failed，EXIT=0。
V2 包级 `cargo test -p cms_assemble_control`：`test result: FAILED. 314 passed; 34 failed; 4 ignored`，EXIT=101。
V3 `cargo fmt --check -p cms_assemble_control` 与 `cargo clippy -p cms_assemble_control -- -D warnings`：均 EXIT=0。
Fail Loud 披露：V2 的 34 个失败全部为写域外基线 fixture 问题——scratch 库缺表（SqlState E42P01，relation x_cms_appinfo / x_cms_categoryinfo / x_cms_data_document / auth_role does not exist），panic 点集中在 security_gate_tests_idor_b5.rs:32、security_gate_tests_read_a_b1.rs:32、read_a_b2.rs:37、tests.rs:752 等；与本次 file_id_upload（约 4229 行）修复无因果关系，未静默跳过、未标记为通过。

## 6. 行为变化
跨人覆写既有文件：修复前 200（正文被篡改）→ 修复后 403（拒绝，DB 未改）。
新建文件：修复前 creator 落 NULL → 修复后 creator 落当前会话人 person_unique。
本人覆写自己文件：维持 200，正文更新、creator 保留。
匿名请求：维持 401；管理员读门仍由 u2_check_owner 内 is_admin 放行。

## 7. 待办线索（Fail Loud）
同构裸写端点 `file_id_mockputtopost`（约 4201 行，无 session 参数）本轮 out of scope 未改，具备相同跨人/匿名覆盖风险面，移交 R643 单独甄别修复。

## 8. SHA 分列（据 meta 不可复算，分列记录）
审计对象 lib.rs sha256（修复后）：1edb1f272a3c47e6e2854f2dfdbd61a268b6a180f238d1255b9c61fe82c58c83
runnerSha（临时契约全文 sha256）：50e761d46d5eeeade0e844b8c8a7836c8d9443185feb20e79185963fb016c5f4
docSha（本文档全文 sha256）：由落盘内容计算，见任务上报值。
零真值声明：本文仅含占位符 <TEST_HOST> 等，无口令/真域名/IP/个人路径；LEAK=0。
