# oa4rust R357 H5 即时通讯消息分页链只读审计

> 身份：qa-e2e｜轮次：R357 / t168｜性质：只读甄别（不改源码/契约/后端/SDK）
> 测试环境：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>，测试服部署；浏览器 390x844 headless（isMobile/hasTouch，DPR2），非物理真机，截图均为真浏览器实测渲染而非真机截屏。
> 凭据：运行期经 ssh 读取，不回显、不入库；API 前缀 /oa4rust-api。

## 1. 任务与结论

甄别 H5 消息页 mobile/src/pages/chat/chat.vue 的历史分页链与轮询合并链是否存在解包失配、顺序错乱、去重失败或 hasMore 终止态缺陷。

最终判定：0 个真缺陷（通过）。首屏单层信封解包、更早消息翻页拼接顺序与去重、轮询去重、失败/空态文案可区分，均经真浏览器实测通过；0 pageerror。

## 2. 覆盖甄别

- docs/ops 全库对 chat.vue 分页链命中 0（无专项文档）。
- docs/.../current-state.md:1726 仅记 chatPolling 串行轮询生命周期配对无泄漏，非分页链专项。
- R316/R317/R322 覆盖建会话/发送/已读/会话列表，未测分页解包/去重/hasMore 终止态。
- 本轮为分页链首次专测，属有效增量。

## 3. 源码梳理

### 3.1 页面链（chat.vue，383 行）

- onLoad :140-149：conversationId 取自 options.id，拉 loadHistory()。
- loadHistory :151-179：page=1 调 loadChatHistoryPage，msgs = mergeOlderMessages([], keepConversation(result.messages))（:161），记 historyPage/hasMore（:162-163），historyReady 后 startPolling()（:168）；catch 置 historyError=describeChatError（:171）。
- loadOlder :181-199：page=historyPage+1，msgs = mergeOlderMessages(msgs, keepConversation(result.messages))（:191），更新 hasMore。
- keepConversation :62-64：按 messageConversationId 过滤跨会话串入。
- 模板 :243-258：msgs=0 且 historyError → tip-error + 重试；msgs=0 → 暂无消息；有消息时 historyError → 重试 / loadingOlder → 加载更早消息… / hasMore → 加载更早消息（可点） / tip-end → 没有更早的消息了；bubble 显示 m.content。

### 3.2 分页工具（chatHistory.ts，82 行）

- rowsOf :28-31：仅 Array.isArray(resp?.data) 才返回数组，否则 []。
- loadChatHistoryPage :33-43：page/size 夹取 ≥1 取整，hasMore = messages.length >= safeSize（:42）。
- mergeOlderMessages :49-63：按 id 去重；olderDesc 倒序遍历 prepend 到 current 顶部（旧→新正序）。
- messageWasSent :80-82：data.sent===true（R259 发送确认，属另一链）。

### 3.3 轮询合并（chatPolling.ts）

- mergeNewerMessages :117-145：按 id 去重 + messageKey（sender + 分隔符 + content，:104-109）把 local-* 占位行一对一替换为服务端回显行。
- startChatPolling :52-102：串行轮询 + 连续失败退避。

### 3.4 后端契约（backend/crates/message_assemble_communicate/src/lib.rs）

- im_msg_list_page_size_size :1792-1865：按 body.conversationId 过滤（:1802-1808，修复旧版忽略 body 返回全库），ORDER BY create_time DESC LIMIT/OFFSET，返回 ActionResult::legacy_success(Value::Array(data), count, size)（:1860-1864）。
- legacy_success 单层信封：data 为数组、type=success，另含 count/size/message/date/spent/position；失败走 AppError（非 2xx）。端点不产生 type:error 无 data 形状。
- 传输层 services/http.ts :183-208：2xx resolve(envelope)，非 2xx reject（500 → httpStatusError）。services/index.ts list() :24-26 仅类型断言不解包。

## 4. 真浏览器实测

环境：390x844 headless，isMobile/hasTouch，DPR2，非物理真机；4 轮脚本（首轮 + v2 导航失效 + v3 整页重载注入 + v4 失败态对照），0 pageerror。

| 场景 | 方法 | 结果 |
| --- | --- | --- |
| 信封探针 | 真实 API fetch | conv 列表/msg/list 均 dataIsArray=true（单层数组），msg n=20 count=20 size=200，样本含 id/conversationId/content/sender/type；rowsOf 解包正确，非 R340/R351 分组信封型 |
| 真实首屏 | 进入 R318 有消息会话 | 2 条真实消息正序、无重复无丢失、tip-end=1 |
| 空会话 | 进入无消息会话 | 暂无消息，bubble=0 |
| 注入首屏 | route 注入 page1=200 条（后端 DESC 400→201） | 200 条正序 MSG-0201→MSG-0400（mergeOlder 倒序 prepend 正确），load-older=1/tip-end=0 |
| 注入翻页 | 点加载更早，page2=100 条（200→101） | 300 条正序 MSG-0101→MSG-0400，dupCount=0，load-older=0/tip-end=1 |
| 轮询去重 | 等待 7s 一轮轮询 | 仍 300 条、dupCount=0 |
| 边界终止 | page1 满页 / page2 空 | 加载后 tip-end=1，正确终止 |
| 失败文案 | 注入 HTTP 500 | 消息加载失败：HTTP 500 + 重试（tip-error=1），与空态可区分 |
| 空态文案 | 注入 data=[] | 暂无消息（errTip=0），与失败态可区分 |

v2 曾因 uni-app SPA 内 page.goto(hash) 不触发 onLoad 重跑导致注入全落空（4 场景均显示同一真实会话），改用整页 reload() 后 v3/v4 全部命中（route hit.url 确证拦截生效）。此为取证脚本导航问题，非产品缺陷。

## 5. 三分判定

### 5.1 真缺陷（0）

无。分页链解包/顺序/去重/终止态/文案五项均通过；0 pageerror。

### 5.2 取证限制

- 真实发消息属破坏性写，本任务未授权执行；发送本地占位→服务端回显替换（messageKey）逻辑由 R317 在发送 API 层覆盖，本轮仅做静态确证，未注入服务端回显实跑一对一替换。
- 真实会话历史未达 200 条，200/100 条翻页数据为 route 注入对照；不影响解包/顺序/去重逻辑验证。
- type:error 无 data（200）注入落到暂无消息，但后端 msgHistory 不产生该形状（失败必非 2xx），不作缺陷判定。

### 5.3 产品语义差异

- chatHistory.ts:42 hasMore=messages.length>=safeSize：恰好满页的末页需多一次空请求才显示「没有更早的消息了」（C_boundary 证实），无消息丢失/重复，属边界取舍而非缺陷。
- rowsOf 对非数组 data 降级为 []：后端成功恒为 Array，该降级为传输层防御，不触发真实业务路径。

## 6. 证据清单（gitignored，未入库）

- test-results/r357-h5-chat-pagination/evidence.json + evidence-v2.json + evidence-v3.json + evidence-v4.json。
- 截图：chat-message-list.png / chat-first-screen.png / chat-v3-first.png / chat-v3-older.png / chat-v3-fail.png / chat-v4-http500.png，均 390x844 headless 非物理真机。
- 脚本：r357-h5-chat-pagination.cjs / -v2.cjs / -v3.cjs / -v4.cjs 及对应 run.sh。

## 7. 复现方法

测试服部署后，以真浏览器 390x844（isMobile/hasTouch）登录，进入即时通讯 → 选有消息会话核对首屏正序；用 route 拦截 /oa4rust-api/api/message/assemble/communicate/im/msg/list/{page}/size/{size} 注入多页验证翻页/去重/终止态与失败文案。所有域名/主机以占位符表示，真值仅运行期从环境读取。