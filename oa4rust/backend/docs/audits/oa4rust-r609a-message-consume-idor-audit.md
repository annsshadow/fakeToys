# R609a 后端 message crate 已读标记身份盲 IDOR 写甄别

> 结论：判 P1。零源码改动，唯一产出为本文档。真父 HEAD=c78dee77692b4242dc31c20691bab0c2040edabf（git 核实一致）。
> 真父之后出现一个他组提交 c772a2242（R609b MeetingApp bookingDate UTC 审计），经 git diff 核实对本文锚点源码零改动，不属本次范围。

## 1. 立项与范围

- 目标：验证 message crate 两个已注册写端点 update_single、mark_read 是否仅凭消息裸 id 即可跨用户翻转他人消息已读标志（xconsumed）。
- 方式：起独立临时 PG（trust 认证，回环端口，真值不入库），full migration 后 scratch 库 seed alice/bob 各一条 x_msg_message 记录（xconsumer 分别为两用户，xconsumed=false），axum oneshot 直调真实 router + handler，全程零源码改动。
- In scope：crates/message/src/lib.rs、routes.rs、security_gate_tests_b73.rs；兄弟正确范式 crates/message_assemble_communicate/src/lib.rs；DDL migrations/011_normalize_schema.sql；前端可达性 apps/desktop/src/views/IMChat.vue；MCP generated_routes.rs。
- Out of scope：任何源码修复、迁移、前端改动、台账更新。

## 2. 机制锚点

- crates/message/src/lib.rs:30-34：require_session(session: Option<Extension<Session>>) -> Result<Session, AppError> 仅 session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)，只提取会话、不校验消息归属。
- update_single（:114-152）：:119 require_session(session)? 丢弃返回值只验存在；:122 SELECT xid FROM x_msg_message WHERE xid = $1 不查 xconsumer；:139 UPDATE x_msg_message SET xconsumed = true WHERE xid = $1 仅按裸 xid 盲写。
- mark_read（:191-222）：:196 同款只验存在；:199 同款裸 xid 查询；:206 同款盲写 UPDATE，两端点同族同缺陷。
- routes.rs:19 注册 POST /api/message/consume/{id}/type/{type} → update_single；:21 注册 POST /api/message/mark_read/{id} → mark_read；:23 layer(Extension(pool))。两端点均为真实可达路由。
- 兄弟正确范式 crates/message_assemble_communicate/src/lib.rs:191-225 mark_read：:196-198 无会话 401；:201-207 SELECT ... sender AS owner FROM x_message_consume WHERE id = $1；:211-212 shared::middleware::require_owner(&pool, sess, &owner)；有归属门。证明本项目已有可复用正确做法。
- DDL：migrations/029_create_missing_core_entity_tables.sql:234-242 规范列 title/consumer/type NOT NULL；migrations/011_normalize_schema.sql:42 ALTER TABLE x_msg_message ADD COLUMN IF NOT EXISTS xconsumed BOOLEAN NOT NULL DEFAULT false。consume_list:58 与 unread_count:231 均以 xconsumer 作接收人过滤，证明 xconsumer 为归属列。
- 前端可达：apps/desktop/src/views/IMChat.vue 用户可输入任意 id 触发已读接口；MCP generated_routes.rs 对外暴露对应路由。

## 3. 方法与决定性实测

| 用例 | 操作 | 结果 |
|---|---|---|
| alice_can_mark_bob_message_consumed_via_update_single | 无会话 POST consume/{bob xid} | 401，符合仅挡匿名 |
| 同上 | alice 会话 POST consume/{bob xid}/type/plain | 200 success，bob 的 xconsumed 被置 true，alice 自己消息仍 false |
| alice_can_mark_bob_message_consumed_via_mark_read | alice 会话 POST mark_read/{bob xid} | 200 success，bob 的 xconsumed 被置 true |
| sibling_mark_read_rejects_alice_for_bob_consumer | 兄弟端点 alice 冒名 bob 的 consume id | 403（无会话 401），归属门正确 |
| source_guard_idor_shape_unchanged | 源码守卫 | require_session 只验存在 + 两裸写 SQL 无归属谓词，锚点未变 |

- 决定性命令（cwd=oa4rust/backend）：DATABASE_URL 指向独立临时 PG，cargo test --test r609a_message_consume_idor_audit -- --test-threads=1。
- 结果：EXIT=0，4 passed / 0 failed / 0 ignored，finished in 29.55s。日志见 SHA 清单。

## 4. 三分裁决（据实，不硬凑）

- 判 P1：任意已登录非管理员凭目标消息裸 id 即可跨用户翻转他人消息已读标志，两端点同族、真实路由注册、前端按钮与 MCP 均可达；兄弟 crate 已有正确归属门证明完全可避免。与同族 R606a（考勤 received 盲写）、R608a（模型 status 盲写）判级一致。
- 削弱因素据实披露：① 需有效会话（require_session 挡匿名，无会话 401），非匿名越权；② 需知目标 xid，非无差别批量遍历；③ 写操作仅翻转 xconsumed 布尔标志位，可复位，非凭据泄露、非数据外泄。据此不判 P0。
- 测试锁定误判据实披露：crates/message/src/security_gate_tests_b73.rs:15-16 注释误判同族 update_single 不在已注册路由上（gauge R7 未见），但 routes.rs:19 明确注册；该审计前提为假致此缺陷长期漏网。b73 测试（:39-53 无会话 401、:55- 有效会话过门落死池 Internal）只挡匿名、不测跨用户冒名，属覆盖缺口。
- 两端点合并为同一项，不按端点数硬凑多个发现。

## 5. Fail Loud

- 全程零 SKIP、零忽略用例；4 个用例全部实际执行且通过。
- 夹具首版编译失败（常量 BOB 未使用触发 -D warnings），已删除；seed 首版缺 title/consumer/type 触发 NOT NULL（E23502），已补齐；OPS edit 保留 mtime 致 cargo 复用陈旧二进制，经 touch 强制重编译后重跑通过，均为测试脚手架问题，非产品逻辑。
- 临时 PG 容器、夹具、日志用后清理；真值（回环端口/口令）未入库，仅以占位符表述。
- 台账 docs/ops/current-state.md 检索 R609a / xconsumed / 已读越权命中 2 处，据实披露。

## 6. SHA 清单

- runnerSha（sha256(JSON.stringify(meta))）：f1415570e36f01badf4fc0436427e3954b444db93270918cad1572f026b29179
- 文件字节 sha256（与 runnerSha 分列）：
  - 夹具 tests/r609a_message_consume_idor_audit.rs：0d67b6f6df36b078d2650801edd0af6d6ae57803e6ce067d012449f2833dc223
  - 日志 test-results/r609a-run.log：91dd097a5922056059dd80f4257d2338e664878befdac2f8948eef37254e77bb
  - crates/message/src/lib.rs：7e6fe0a71e52a9a3e0d01d84d2fe265ae94e55816b378c70597dfef9d3873e6c
  - crates/message/src/routes.rs：342ec298429c2e3129fa5b05a25782a74f7b3b5bf6b921015d126c5052c21e0b
  - crates/message/src/security_gate_tests_b73.rs：7a616228ff546b2c6bbfc746434d9c0d7408bbf97931444a8c5087fce34bf225
  - crates/message_assemble_communicate/src/lib.rs：4a173cad1e424743b3ac9ee227aaef1312da5fb6be2894960b5086dff7bc2e8c
  - migrations/011_normalize_schema.sql：7d6b6cf09c80082d0b04c530a81a379a0b3a1696b849b7d95fbbd8e0a9ce449d

## 7. 修复方向

- 参照兄弟范式，在两个 handler 中先按 xid 查 xconsumer（或 creator_person）归属，再用 shared::middleware::require_owner(&pool, &sess, &owner) 校验，无归属则 403；无记录 fail-closed。
- 修正 security_gate_tests_b73.rs:15-16 的错误注释并补跨用户冒名用例，防止同类漏网。