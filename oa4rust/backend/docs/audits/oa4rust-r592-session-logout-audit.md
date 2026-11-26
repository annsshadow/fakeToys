# R592 safe_logout PG 回填臂会话失效一致性甄别报告

## 结论

判 **L1（故障注入才复现，非确证 P2）**：正常路径下 `auth::safe_logout` 后旧 token（t1/t2）均被正确拒绝，DB 中该 person 会话行清零（`revived=false, dbrows=0`）；仅在人为故障注入臂（`DELETE FROM auth_session` 被触发器阻断）下旧 token 复活（`revived=true, dbrows=1`）。按派发契约，仅故障注入臂复活须据实判 L1，不得包装成确证 P2；正常路径未复现缺陷，故判 L1。

## 立项性质与争议披露

验证型甄别（后端 PG 回填臂会话失效一致性）。争议点：历史注释称 safe_logout 曾因 `threshold_time` 按字符串绑定报 WrongType 且被 `let _` 吞错，导致"全端登出使旧 token 失效从未真正生效"。本轮不采信注释与静态推断，以真测试坐实当前代码在正常路径下的实际行为，严格区分正常路径与故障注入臂，避免把故障注入结果包装成确证 P2。

## 范围与入口

- 写域：仅新增本文档；零源码改。证据置于 gitignored `oa4rust/test-results/r592/`（`.gitignore:28` 覆盖）。
- 入口：`auth::safe_logout`（`crates/auth/src/safe_logout.rs:17-57`）经路由 `POST /api/authentication/safe/logout` 以 `Bearer` 令牌调用。
- 关键源码：`SessionManager::validate_session`（`crates/shared/src/session.rs:359-445`）、`remove_sessions_by_person`（`:482-508`）、`broadcast_logout`（`:514-553`）、`u2::safe_logout_get`（`crates/organization_assemble_authentication/src/u2.rs:192-228`）。
- 建表依据：`migrations/001_create_auth_tables.sql`、`migrations/017_create_token_threshold.sql`。

## 机制（源码逐行）

1. `safe_logout` 提取 token → `validate_session` → 写 `auth_token_threshold(person_unique, threshold_time=now NaiveDateTime, created_at)` `ON CONFLICT DO UPDATE`（`:33-42`，错误被 `let _` 吞）→ `remove_sessions_by_person` → `broadcast_logout` → 返回 success。
2. `validate_session`：内存命中未过期经 `person_is_disabled` 闸放行；内存未命中先查 negative_cache（TTL 5s），再查 PG `SELECT ... FROM auth_session WHERE token=$1 AND expires_at>NOW()`（signed token），命中则回填内存。★**DB 回填路径完全不查 `auth_token_threshold`**；`check_token_threshold`（`:277-310`）仅在 `create_session` 登录时调用。
3. `remove_sessions_by_person`：先内存批量删该 person token，再 `DELETE FROM auth_session WHERE person_id=$1`（失败仅 `tracing::warn` 静默跳过）。`broadcast_logout` 读 threshold_time 仅清内存中 `created_at < threshold` 的会话，为单实例本地清理。
4. 推论：登出失效依赖"内存删除 + DB 删除"两处同时成功；二者任一失败且回调经 DB 回填即可复活。正常路径二者均成功，故旧 token 被拒。

## 方法（真测试，非静态推断）

- 起临时 PG 容器（postgres:16-alpine），建三张必要表 `auth_person` / `auth_session` / `auth_token_threshold` 及索引。连接参数由环境变量 `DATABASE_URL` 注入（真值不入库，本报告以 `<TEST_HOST>:<PORT>` 占位）。
- 写真集成测试 `crates/auth/tests/r592_session_logout_audit.rs`（2 个 `#[tokio::test]`，全部经 pub API：`crate::router` / `SessionManager::with_pool`），测试源码用后即删，工作树保持零源码改。
- 命令：`cargo test -p auth --test r592_session_logout_audit -- --nocapture` → `2 passed; 0 failed`。
- 两臂：①正常路径——create t1/t2 → 断言前置 valid → 调 safe_logout(Bearer <TEST_TOKEN>) → validate t1/t2 + COUNT(person) → 清理。②故障注入——建 `BEFORE DELETE` 触发器对目标 person 抛异常阻断 DELETE → create t1 → 调 safe_logout → validate + COUNT → 删触发器 + 清理。

## 决定性两臂数值

| 臂 | HTTP | revived_t1 | revived_t2 | DB 会话行 | 判定 |
|---|---|---|---|---|---|
| 正常路径 | 200 OK | false | false | 0 | 旧 token 被正确拒绝 |
| 故障注入（DELETE 失败） | 200 OK | true | （单 token） | 1 | 旧 token 复活，非确证 P2 |

两臂响应体均为 `success` / `all sessions invalidated`，即登出接口在 DELETE 失败时仍返回成功（`let _` / `tracing::warn` 吞错），失败对调用方不可见——这是故障注入臂复活的直接原因，但属异常环境条件，判 L1。

## 三分裁决

- **可达 P2（排除）**：正常路径未复现缺陷，`revived=false, dbrows=0`。
- **正常 0（排除）**：故障注入臂确能复活（`revived=true, dbrows=1`），非完全无风险。
- **不可达 L1（采纳）**：仅人为故障注入臂复现，正常路径安全，按派发契约据实判 L1。

## Fail Loud

- 临时 PG 与测试夹具为本地可复现环境；连接真值、测试令牌均未写入仓库，仅以占位符表达。
- 测试源码为一次性甄别夹具，落盘运行后已删除，工作树 `git status` 除无关 untracked 外无改动。
- 本结论仅覆盖单实例、PG 可查场景；多实例/threshold scanner（60s tick）跨节点时序未纳入本轮，据实披露为未覆盖。

## sha256 证据表

runner 紧凑 JSON（`sha256(JSON.stringify(meta))`）与文件字节哈希（`sha256sum`）分列，不可混用：

- runnerSha(R592 meta) = 149a0839fec110a6ca7d93f2aa7629be89cee8bec5c4d9255aac3936939a640b
- 证据日志 `oa4rust/test-results/r592/r592-test.log`（字节）= be2321e3d044ab57a9e7340483b6351ef7c8a3acb327d8e88da41150ac87d544
- 测试夹具备份 `oa4rust/test-results/r592/r592-audit-test.rs`（字节）= 5ffac483432e7e61c1687a7851e817e57ab38488f88821bbe8cb35a463b82527
- 源文件 `crates/auth/src/safe_logout.rs` = ca094c209c9831641e2513b1d8956c8ca0af46f0fc1de5c57fd5d27a075a37fb
- 源文件 `crates/shared/src/session.rs` = 9a6f5be4caa481da9331c9a9b1eb3db72c54b051ce92f4435c2971bd2a3247f5
- 源文件 `crates/organization_assemble_authentication/src/u2.rs` = fbea4445aa7425acec8a838b75c4efbfe333ae0171cadef7b27aa4f62cbbc78a
- 源文件 `migrations/001_create_auth_tables.sql` = 5e967256118ad210f8c1a35dcc6d244691783d1fd138d018761eddafe0e81fe2
- 源文件 `migrations/017_create_token_threshold.sql` = 2566a4abdf79f49af287d3b81dac2cd1e528b005f081eda33dc8077fb07ff701

## 观察（非本轮修复项）

登出在 DELETE 失败时仍返回 success 且不向调用方暴露失败，是可改进的可观测性缺口；但正常路径会话失效成立，本轮判 L1，不另立修复契约。
