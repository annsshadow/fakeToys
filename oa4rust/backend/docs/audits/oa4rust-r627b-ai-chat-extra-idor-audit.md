# R627b ai chat_write_completion_extra 身份盲写 IDOR 甄别

## 一、结论 / 真父 / 零源码改
- 结论：**可达真缺陷 1 项（P2）**——`chat_write_completion_extra` 仅有会话门、无归属校验，任意已认证用户凭他人 chat id 即可覆写其 `extra` 扩展数据并真实落库。
- 真父：`02e1cca3b9b8ae1b08132b13144deb3ea41f94d6`（HEAD=真父）。
- 本次零源码改：仅新增本文档；oneshot 夹具为临时测试文件，真测后从源码树删除（git 零残留）。

## 二、立项范围与判级口径
- 范围：`ai_assemble_control` 写端点 `chat_write_completion_extra` 的归属校验缺失。
- 判 P2 而非 P1：需已认证会话、且须知道目标 chat id；写对象是扩展字段 `extra`（非正文 `content`、非删除/提权）；影响为他人对话扩展数据被篡改，非跨用户正文泄露或系统级破坏。
- 据实不夸大：不降 L1（落库真实发生、行为可证），不升 P1（有会话门 + 需知 id + 仅扩展字段）。

## 三、机制锚点（oa4rust/backend/crates/ai_assemble_control/src/lib.rs）
- `:2700` `chat_write_completion_extra(pool, session: Option<Extension<Session>>, req: Option<Json<Value>>)`。
- `:2715` `require_session(session)?` —— 只验证「有会话」，随即丢弃会话身份，全程不再用于归属比较。
- `:2723` SQL `UPDATE x_ai_chat SET extra=$2::text::jsonb WHERE id=$1 AND deleted_at IS NULL` —— 仅按 chat id 定位，无 owner/user_id 谓词。
- `:2729` `updated==0 → AppError::NotFound`；`:2733-2736` 无 `AI_API_KEY` 时 `gateway=simulated`，但写已落库（非假成功）。
- 路由 `routes.rs:93-94` POST `/api/ai_assemble_control/chat/write/completion/extra`；`oa4rust/backend/src/lib.rs:154` `.merge` 真实挂载。

## 四、兄弟铁证（同 crate）
- `chat_delete_clue_id` `:2629-2661`：先 `:2639` `SELECT user_id FROM x_ai_conversation WHERE id=$1`，再 `:2659` `shared::middleware::require_owner(&pool,&session,&user_id)`，他人冒名 → Forbidden。说明本 crate 已有可用归属门，仅本写端点漏用。
- `require_owner` `shared/src/middleware/rbac.rs:582-590`：admin 放行 / 本人（trim+lowercase 归一）放行 / 其余 Forbidden。

## 五、决定性实测（独立 PG oneshot，真调 handler）
临时容器独占实例（占位 `<TEST_HOST>:<PORT>`，不入库真值）；`DATABASE_URL` 由环境变量注入；fresh_scratch 建库 + 全量迁移；测试后 drop_scratch + 删夹具。
| 臂 | 输入 | 结果 | 判定 |
| --- | --- | --- | --- |
| 1 跨属主写 | alice 凭 bob chat id 写 `{attacker:alice}` | success；`bob_extra={"attacker":"alice"}` 真实落库；gateway=simulated | **IDOR 坐实** |
| 2 本人写 | bob 写 bob chat | success，落库 `self=bob` | 正常 |
| 3 admin 跨人写 | admin 写 alice chat | success，落库 `by=admin` | 正常（管理员放行） |
| 4 匿名调 handler | 无 session | `AppError::Unauthorized`（401） | 正常（会话门） |
| 5 不存在 id | alice 写不存在 chat | `AppError::NotFound`（404） | 正常 |
| 兄弟对照 | alice 冒名删 bob conversation | `AppError::Forbidden`（403） | 正常（归属门未变） |
| app 层裸 401 | 无凭据 POST 路由（auth_middleware） | HTTP 401 | 正常 |
- `cargo test -p ai_assemble_control --test r627b_ai_chat_extra_idor_audit -- --nocapture`：`2 passed / 0 failed`，exit 0。
- 首轮失败已修复并据实记录：seed 用 `$5::jsonb` 绑定 String 报 `WrongType { postgres: Jsonb, rust: String }`，改 `$5::text::jsonb` 后通过（仅测试夹具问题，与缺陷无关）。

## 六、三分裁决
- **可达真缺陷：1 项 P2** —— 跨属主覆写 `extra` 落库（臂 1）。
- **不可达 L1：0 项。**
- **正常项**：会话门（臂 4 + app 层 401）、404 分支（臂 5）、本人/admin 放行（臂 2/3）、兄弟删除归属门（对照 403）。
- 前端消费据实披露：本轮在 `oa4rust/frontend` 未定位到该端点的直接调用点；后端路由与挂载属实，缺陷以后端 handler 为准，不因前端未定位而否认落库事实。

## 七、Fail Loud / 旧审计披露
- 旧审计 `2026-10-14-security-audit-round4.md:1049` 仅把本端点记为「会话档」（`require_session` 后置、无会话 401），从未审视归属；`:1051` 明记「owner/admin 收紧待 §五」。本甄别据实补上归属维度，不推翻其会话档结论。
- 全部实测执行完成，无跳过；未设的环境依赖（`AI_API_KEY`）已验证为未配置，gateway=sig 分支据实记录为 simulated。

## 八、SHA 清单（runnerSha 与文件字节 sha 分列）
- runnerSha（`sha256(JSON.stringify(meta))`）：`558d2c84fc84180191ad5cefa4cfe1d7ee6628e3f3f42f18835308a77edcddcd`
- 夹具源码字节 sha（源码树与证据副本一致）：`b2dcf70541b6593c09082067b0745df7aecb2184c197e8f6cfb1ca4814bdb141`
- oneshot.log 字节 sha：`6fde4d4a43458c252383ec101c5e33e732de1c2a743f7ffa01661baa00805602`
- meta.json 字节 sha：`db82ca7fa9263dbd64a845dd4ecc78aea51b6f03705f0395046cce4867821f89`
- 证据目录（gitignored）：`oa4rust/test-results/r627b-ai-extra/`

## 九、修复方向（不在本次范围）
在 handler 内事务化：先按 chat id 查 `x_ai_chat.conversation_id`（或 creator），再查 `x_ai_conversation.user_id`，`require_owner(&pool,&session,&user_id)` 通过后才执行 UPDATE；归属查不到时 fail-closed（仅 admin）。参照兄弟 `chat_delete_clue_id` 的归属门实现。
