# R385 PC CalendarApp.vue 写操作链失败信封甄别

- 轮次：R385（t209）
- 执行者：qa-e2e（agent-2），只读审计，未改任何源码
- 环境：Playwright Chromium headless 1440×900，针对部署测试服的真实页面流程；**非物理真机**，截图均来自该 headless 浏览器
- 测试域名与凭据：仅从环境变量/本地凭据脚本读取；本文全部使用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，0 真值入库
- 源码路径：均为仓库相对路径；无本机绝对路径

## 一、结论（三分判定）

**0 个用户可达真缺陷**（本轮与 R366/R368/R370/R373/R377 等批次不同， CalendarApp 后端写端点用非 2xx 报错，误导不成立）。

- L1 潜在隐患（非本轮缺陷、不判 P1）：`frontend/apps/desktop/src/views/CalendarApp.vue` 的 `calWriteCE`（:283-343）、`finishEvent`（:494-508）、`removeEvent`（:510-526）等写函数在 `await api` 后仍有裸 `toast.success` 不读响应 `type`；若后端未来改成 200+type:error 口径，会复现同型误报。当前真后端统一 400/404，SDK 抛错进 catch 走 `toast.error`，故用户不可达。
- 取证限制：破坏性真状态翻转/真删未授权未执行，采用假 ID 探针 + 只注入列表行、写响应完全走真后端不拦截的决定性对照替代；死代码分支未逐一 UI 触发；R311 历史测试日历为既有数据。
- 产品语义差异：`calendar` crate 用 `AppError::BadRequest/NotFound` 非 2xx 报错，与 o2server 兼容式 `Ok(Json(ActionResult::error(..)))`（HTTP200+type:error）口径不同，属既有实现差异，非本轮引入。

## 二、真 UI 入口甄别（区分死代码）

文件：`frontend/apps/desktop/src/views/CalendarApp.vue`（820 行），路由 `frontend/apps/desktop/src/main.ts:82-84` path=calendar、name=CalendarApp。

- 有真实 @click/@submit 写入口：模板 :57-70 `calCreateOpen` 表单 → :58 `@submit.prevent=calWriteCE(calCreate)`；`createEvent`（:467-490，模板 :130 @submit.prevent）；`finishEvent`（:494-508，模板 :148 @click，confirmMsg 守卫 :497）；`removeEvent`（:510-526，模板 :149 @click，confirmMsg 守卫）。DOM 审计 35 按钮，`calendar-create-entry`=1、`calendar-scope-tabs`=1；事件表单/管理面板需先选中日历才出现。
- 死代码（全文件仅定义处，无 @click/defineExpose/window，按 R364/R365/R372 口径不判缺陷）：`calWrite`（:223-254）、`calMore`（:257-280）、`loadCalCoreEntities`（:345）、`loadCalSettings`（:190），以及 `calWriteCE` 的 calUpdate/calRemove/eventCreate/eventUpdate/eventRemove 分支（需 prompt 输入、无真入口）。
- 只读探测（非写）：`loadCalendars`（:352-379，失败 describeListError「日历列表加载失败」）、`selectCalendar`（:385）、`loadCalMeta`（:396）、`loadCalEvents`（:434，失败「日历事件加载失败」）。

## 三、真后端契约（真浏览器假 ID 探针，不臆造）

后端 `backend/crates/calendar/src/lib.rs`：`calendar_create`（:269-315）缺字段→`AppError::BadRequest`(400)、成功 `ActionResult::success(item)`；`event_create`（:437-489）缺字段→400；`event_update`（:493-560）假 ID→`AppError::NotFound`(404)、有 require_owner；`event_remove`/`calendar_update`/`calendar_remove` 同 404 口径。`backend/crates/calendar/src/routes.rs` :24/:27/:28 为对应 POST 路由。

真浏览器探针（evidence.json probe / realResps）5 端点全部非 2xx：

| 探针 | HTTP | type | message |
|---|---|---|---|
| calCreateMissing（缺 name） | 400 | error | bad request: name is required |
| eventCreateMissing（缺 calendarId） | 400 | error | bad request: calendarId is required |
| eventUpdateFake | 404 | error | not found |
| eventRemoveFake | 404 | error | not found |
| calUpdateFake | 404 | error | not found |

SDK `frontend/packages/sdk/src/api.ts`：:82-88 `parseJsonEnvelope` 只 JSON.parse 不查 type；:275-296 仅非 2xx 抛错（400/404→ApiError 进 catch）。因此 400/404 会进各写函数 catch → `toast.error`，UI 不会误报成功。

## 四、E2E 实测（3 轮，1440×900 headless 非物理真机，0 pageerror）

证据：`test-results/r385-pc-calendar-write/`（gitignored），含 evidence.json、脚本 r385-pc-calendar-write.cjs / -v2.cjs、截图 cal-dom-audit.png、cal-probe-done.png、cal-injected.png、cal-finish-cancel.png、cal-finish-decisive.png、cal-stub-finish-toast.png、cal-stub-create-event-toast.png、cal-stub-create-cal-toast.png、cal-http500-toast.png、cal-fail-state.png、cal-empty-state.png。

1. **假 ID 探针**：上表 5 端点非 2xx，坐实后端口径。
2. **决定性对照（排除注入伪造）**：只注入日历列表 1 行 `R385-INJ-*` + 事件 1 行，写响应完全走真后端不拦截。点「结束」→ confirmMsg DOM 弹窗点取消 → writes 5→5 零写；再点「结束」→确认 → 真 POST `/api/calendar/event/update`（假 ID）→ HTTP404 `not found` → UI toast「✗ 更新事件失败：请求的资源不存在或已被删除」（截图 cal-finish-decisive.png 肉眼确认：红 toast 更新事件失败 + 注入日历/事件可见）。真响应体由脚本全局 realResps 捕获（s=404, type=error, not found）。
3. **人造注入对照（不判缺陷）**：注入 200+type:error stub：stubFinish→「✓ 事件已结束」、stubCreateEvent→「✓ 事件已创建」、stubCreateCal→「✓ 日历已创建」，确属裸 toast 误报形状，但真后端不产生该信封（见第三节），故不判用户可达缺陷。截图 cal-stub-finish-toast.png 肉眼确认绿 toast「事件已结束」。
4. **非 2xx 与失败/空态**：注入 HTTP500 → 「✗ 更新事件失败: HTTP 500: Internal Server Error」（正确错误）；列表 500 → 「日历列表加载失败：HTTP 500: Internal Server Error」+ 重试按钮；列表 200+空 data → chips=0 仅表头，失败与空态可区分。
5. **0 pageerror**：全程 pageErrors=[]。

## 五、最小修复方向（仅建议，本轮不改）

若将来后端口径变化或需防御性加固：对齐 R363 方案 B（`frontend/apps/desktop/src/views/MeetingApp.vue:634-640` meetingWriteSucceeded，type 判定为主 + 权威 bool/id 附加校验），在 `calWriteCE` / `finishEvent` / `removeEvent` 等函数 `await api` 后先判 `type=success` 且权威返回字段有效，否则 `toast.error(r?.message || 操作未生效)`；当前真后端非 2xx 口径下该加固非必需。

## 六、覆盖甄别

R311/R385 等既有日历文档为入口/视觉/插槽方向，从未专项甄别写操作链失败信封 UI 反馈，本轮首次专测为有效增量。