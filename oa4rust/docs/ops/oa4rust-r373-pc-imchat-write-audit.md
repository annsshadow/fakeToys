# R373 PC IMChat 写操作链误报成功甄别（只读审计）

> 证据来源：Playwright Chromium headless 对部署测试服的真实页面实测；**非物理真机**（1440x900）。测试域名与主机以 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，真值仅运行期从环境变量与测试服凭据读取，未入库。本轮只读甄别，未改源码/契约/后端/SDK。

## 一、任务与范围

甄别 `frontend/apps/desktop/src/views/IMChat.vue`（路由 `frontend/apps/desktop/src/main.ts:68`，path=im）写操作链是否存在与 R362/R366/R368/R370/R371 同型的「HTTP 200 + type:error 业务失败信封下无条件 toast.success 误报成功」真缺陷。判定口径：真缺陷须真 UI 可复现 + 真请求响应 + 源码定位三全；无真实 @click 入口的 toast 属死代码，不判缺陷（参照 R364/R365/R372）。破坏性真状态翻转/真删未授权不执行。

## 二、覆盖甄别

`docs/ops` 命中 20 处，均为视觉/可读性/死代码旁证：`oa4rust-pc-pages-r300-b2.md`、`oa4rust-r302-polish.md`、`oa4rust-r303-dr06-readability.md`、`oa4rust-r305-p0b-visual.md`（视觉与会话名脱敏）；`oa4rust-r316-deep-functional-audit.md:36` 仅记 mark_read 端点调用；`oa4rust-r317-m1-visibility-verify.md:15-16`、`oa4rust-r317-message-candidate-verify.md:13`、`oa4rust-r317-static-latent-audit.md` 已证 imMore/imMore2/imMore3/imMore4/imMsgAction/openMsgPaging 为死代码（与本轮源码甄别一致）；`oa4rust-r322-*` 为信封与未读字段修复。均从未专项甄别写链失败信封下 UI 反馈一致性，本轮为有效增量。

## 三、源码甄别：真实入口 vs 死代码

真实 @click 写相关入口（模板 1-130 行）：
- `:10` 发起会话 → `loadRoomStats`（GET 读，非写）；`:20` 会话行 `selectConversation`（内含 `markConversationRead` POST mark_read 写）；`:37` 全部已读 → `markAllRead`；`:57`/`:60` 通话按钮 → `toggleCall`（纯本地 WebRTC，无 HTTP 写）；`:95` 发送文件 → `sendFileMessage`；`:100` 发送表单 → `sendMessage` → `sendMutation`。
- `sendFileMessage`（`:424-449`）：`:440 await api.upload(...)` 后 `:441` 取 `r.data.fileUrl ?? r.data.url`，`:442` **无条件** `toast.success`（有 url 则带文件名）；仅 catch（`:444-445`）走 `toast.error`。**不读 `r.type`**。
- `sendMutation`（`:468-512`）：`onSuccess`（`:492`）只清输入并刷新、`onError`（`:504`）`toast.error`，无成功 toast，非误报型；`markConversationRead`（`:532-536`）与 `markAllRead`（`:538-549`）仅在聚合失败时 `toast.error`（`:547`），无成功 toast，非误报型。
死代码（全文件 grep 仅定义处、无 @click / defineExpose / window 挂载，不判缺陷）：`clearConversation`（`:452`，toast `:457`）、`imConvAction`（`:702`，`:727`）、`imMsgAction`（`:732`，toast `:737/741/746/751/763`）、`imMore`（`:794`，`:828`）、`imMore2`（`:834`，`:857`）、`imMore3`（`:863`，`:872`）、`imMore4`（`:878`，`:906`）、`connectRoomChannel`（`:614`）/ `disconnectRoomChannel`（`:638`）、`loadImTwin`（`:1107`，`:1114`）、`openMsgPaging`（`:785`，无打开按钮）。

## 四、后端/SDK 契约铁证

- 后端 `backend/crates/message_assemble_communicate/src/lib.rs:1908` `im_msg_upload_conversationId_type_type`（路由 `routes.rs:112`）：假 ID → `ActionResult::error(not a conversation member)`；`im_msg_clear`（`routes.rs:98`）同型；`shared/src/response.rs:188-200` 的 `error()` 设 `type=error/size=-1/prompt=ExceptionEntityNotExist` 但 **HTTP 200**（o2server 兼容既有设计），error 构造共 78 处。
- SDK `frontend/packages/sdk/src/api.ts:82-88` `parseJsonEnvelope` 仅 `await resp.json()`，不检查 `type`；`:275-296` 仅 `!resp.ok` 抛错（403 → PermissionError，其它非 2xx → httpStatusError）；2xx 的 `type:error` 原样 resolve。故 `await api.upload` 不抛错，`sendFileMessage` 落入无条件成功分支。

## 五、E2E 实测（1440x900 headless，非物理真机）

- 首轮（脚本 `test-results/r373-pc-imchat-write.cjs`）：DOM 23 按钮，调试面板 advanced-operations=0 / paging-overlay=0 / im-new-chat=1。假 ID `R373-NOSUCH-*` 10 端点探针：`uploadImage`/`msgClear`/`convRead`/`convTopSet`/`send` 均 **200 + type:error + not a conversation member**，`msgRevoke` 200 + `message not found`；`markRead` 200 + `success + markedRead:false`、`collectionAdd` 200 + `success + collected:true`（无成功 toast 故不误报）。
- **决定性对照**：只注入会话列表 1 行（假 ID），写响应完全走真后端、不拦截上传路由。点会话行 → filechooser setFiles → 真 `POST .../im/msg/upload/R373-INJECT-*/type/image` = **200 + type:error**，UI toast=「✓文件消息已发送」，`writesDelta=2`。三全满足。截图 `im-injected-conv.png` / `im-upload-toast.png` 肉眼确认。
- v2（脚本 `r373-pc-imchat-write-v2.cjs`）：再次真后端对照 `trueError` toast=「✓文件消息已发送」（复现）；注入 403 → toast=「✗文件消息发送失败: 没有权限执行该操作」、注入 500 → 「✗文件消息发送失败: HTTP 500」（非 2xx 正确错误，截图 `im-v2-403-toast.png` 目视确认）；两轮 0 pageerror。
- 会话列表 500 注入：`im-v2-fail-state.png` 会话区空白、无错误提示；空注入：`im-v2-empty-state.png` 显示「暂无会话」。二者文本可区分（源码 `IMChat.vue:310-318` 的 useQuery 无 error 状态、模板 `:31` 仅空态判断），但失败态无错误提示/重试入口，记为非阻断 UX 观察。消息列表加载失败有 `msgLoadMoreError`（`:366`）+ 重试按钮（`:72`）。

## 六、三分判定

**1 个 P1 真缺陷**：`sendFileMessage`（`:424-449`）在后端 200 + type:error 业务失败信封下无条件 `toast.success` 误报成功。三全满足：真 UI 可复现（点会话行 + 真 filechooser 上传）、真请求响应（真 200 + type:error）、源码定位（`:440-442`）。最小修复方向对齐 R363 方案 B（参考 `frontend/apps/desktop/src/views/MeetingApp.vue:634-640` `meetingWriteSucceeded`）：`await api.upload` 后判 `r.type === success`（或 type 缺省且 success 非 false）且 `fileUrl/url` 存在才成功提示，否则 `toast.error` 透传 `r.message`。
- 通过项：非 2xx（403/500）正确 `toast.error`、`markRead`/`markAllRead`/`sendMutation` 无成功 toast 不误报、死代码函数群无入口不判缺陷、0 pageerror。
- 取证限制：真状态翻转/真删未授权不执行（用假 ID + 只注入列表行走真后端）；调试函数群死代码未 UI 触发；失败态无错误提示属既有 UX，非本轮 P1。
- 产品语义差异：后端 `ActionResult::error` 用 HTTP 200 是 o2server 兼容既有设计；`markRead` 200 + `success + markedRead:false` 属后端返回口径（无成功 toast 故不误报）。

## 七、交付

证据（`test-results/r373-pc-imchat-write/`，gitignored）：`evidence.json`、`evidence-v2.json`，截图 `im-dom-audit.png` / `im-probe-done.png` / `im-injected-conv.png` / `im-upload-toast.png` / `im-v2-true-error-toast.png` / `im-v2-403-toast.png` / `im-v2-500-toast.png` / `im-v2-fail-state.png` / `im-v2-empty-state.png`。本报告精确单文档提交。