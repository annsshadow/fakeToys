# R318 M1 消息 sender 写读同源修复

## 背景与根因
- t95 真浏览器已坐实：非 admin 会话人创建**不传 sender** 的消息时，写入落 `system`，而列表读谓词 `MESSAGE_SENDER_OWNER_PREDICATE` 以会话人 person_unique 为 owner（`sender = $3 OR admin`），导致本人列表看不到自己刚创建的消息。
- 根因单行：`oa4rust/backend/crates/message_assemble_communicate/src/lib.rs` 的 `message_custom_create` 中 `let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system")`。
- 修复：缺省 sender 改为 `unwrap_or(&sess.person_unique)`（会话人事实源），显式 `sender` 字段覆盖能力保留；读谓词未改。

## Test-first RED → GREEN
- RED 测试（新增于 `oa4rust/backend/crates/message_assemble_communicate/src/tests_generated.rs`）：
  - `r318_m1_custom_create_default_sender_is_self_visible`：非 admin 会话人建会话+成员，创建缺省 sender 消息，再用同一 actor 调 `message/list/paging`；修复前 `data.count=0`，本人不可见，FAILED。
  - `r318_m1_custom_create_explicit_sender_is_preserved`：显式 sender 覆盖仍保留原值（GREEN 前后均通过）。
- RED 命令：`cd oa4rust/backend && cargo test -p message_assemble_communicate --lib r318_m1 -- --nocapture` → 1 passed / 1 failed（缺省可见用例失败）。
- GREEN 命令：`cd oa4rust/backend && cargo test -p message_assemble_communicate --lib` → 117 passed / 0 failed；`security_gate_tests.rs` 未授权负例、`tests_generated.rs` 路由探针、显式 sender 用例均无回退。

## 测试服真浏览器复验
- 部署：测试服地址仅通过环境/SSH 别名注入，文档不写真值；编译使用 `rust:1.98-bullseye + cargo +1.98.0` 产出兼容 Linux ELF（禁 Windows PE32+），SSH 备份现有二进制后原子替换，健康检查通过；失败立即回滚。
- 复验脚本（gitignored）：`oa4rust/test-results/r318-m1-sender-verify.cjs`，以真管理员口令经 `.local.keys`（运行期读取）登录测试服 `<YOUR_TEST_DOMAIN>`，创建非 admin 账号并登录，真实 UI/API 流程创建缺省 sender 消息，再查 `message/list/paging`。
- 证据（gitignored）：`oa4rust/test-results/r318-m1-sender-fix/evidence.json` + 3 张截图，source 标注 `Playwright Chromium headless against deployed test server; not a physical device`（本地渲染，非物理真机）。
- 判定口径：非 admin 创建的缺省 sender 消息 `defaultVisible=true`（sender=本人）、`pageErrors.length=0` 为 PASS。
- 实测结果（evidence.json）：`verdict=PASS`、`defaultVisible=true`、`sender=本人 person_unique`、创建状态 200、`pageErrors=[]`、`apiFailures=[]`，非 admin 账号 `isAdmin=false`。显式 sender 消息归属他人，按读谓词不在本人列表可见，其落库保留由后端 DB 行为测试 `r318_m1_custom_create_explicit_sender_is_preserved` 覆盖。
- 后端二进制 Linux ELF GLIBC 上限 2.30 ≤ 测试服 2.34，部署健康检查通过（`BACKEND_DEPLOY_OK`）；失败自动回滚脚本已就位，本次未触发。

## 安全与脱敏
- 不含口令、证书、真实域名/IP/端口、SSH 真值；测试服地址统一用 `<YOUR_TEST_DOMAIN>` 占位，凭据仅运行期从 `.local.keys` 读取。
- 提交仅含：`lib.rs`、`tests_generated.rs`、本文档；测试脚本、二进制产物、证据目录均 gitignored，不入库。
