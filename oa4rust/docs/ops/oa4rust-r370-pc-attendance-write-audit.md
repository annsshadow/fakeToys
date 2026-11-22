# R370 PC AttendanceApp.vue 写操作链失败信封甄别（只读审计）

- 任务：t187 / R370，attempt 3de3b775-2060-4c35-8506-5192bd1c4919。
- 性质：**只读甄别**，不改源码、契约、后端、SDK。
- 测试环境：Playwright Chromium headless，**1440x900 非物理真机**；测试服域名与凭据运行期注入，文档仅用占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`。
- 证据目录：`oa4rust/test-results/r370-pc-attendance-write/`（gitignored，不入库），含 evidence.json、evidence-v2/v3/v4.json 与截图 att-dom-audit.png、att-injected-appeal.png、att-decisive-toasts.png、att-cancel-guard.png、att-fail-state.png、att-empty-state.png 等（后两张为首轮失效注入截图，已由 evidence-v4.json 补测替代）。

## 一、判定：1 个 P1 真缺陷

**缺陷**：考勤管理页多处有真实 @click 入口的写函数在后端返回 HTTP 200 + 业务失败信封（type:error）时，无条件弹出成功 toast，用户误以为操作已生效。与 R366（程序中心）、R368（文件管理）同型。

- 申诉管理行内写函数：`auditAppealV2`（AttendanceApp.vue:412-422→:418 toast 已审核申诉）、`checkAppeal`（:423-433→:429 已标记检查）、`updateAppealStatus`（:434-446→:442 已更新申诉状态）、`archiveAppeal`（:447-458→:454 已归档），均 await 后不读响应 type。
- 批量写函数群：`attV2Write`（:639-682→:678）、`attCoreEntity`（:787-817→:813）、`attWrite3`（:819-889→:885）、`attV2Ops`（:891-934→:930）、`attAppealWrite`（:1088-1111→:1093/1101/1106）；独立写 :1002-1011→1007、:1012-1024→1019、:1025-1035→1031、:1048-1058→1054、:1112-1120→1116、:1060-1086→1072/1081；明细行内 `analyseOne`（:1744-1751→1747）、`archiveOne`（:1752-1759→1755）、`deleteDetail`（:1760-1770→1764）。全部 await 后裸 toast.success。
- 三全满足：①真 UI 可复现（真实点击注入行的审核/检查/改状态按钮）；②真请求响应（真后端 POST attendanceappealInfo/audit|check|appeal → 200 + type:error），决定性对照**不拦截写路由**；③源码定位（await 后无条件 toast.success）。

**最小修复方向（本轮不改码）**：对齐 R363 方案 B——新增写响应成功判定 `writeSucceeded(resp, flagKey)`，判 `type === success` 且权威 bool 严格为 true 才成功，否则 `toast.error(resp?.message ?? 操作未生效)` 并中止，各写函数 await 后先判再提示。参考实现 `oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:634-640` 的 `meetingWriteSucceeded`。

## 二、源码与契约证据

- 后端 `attendanceappealInfo_audit`（`oa4rust/backend/crates/attendance_assemble_control/src/lib.rs:715-754`）：UPDATE 影响行数 result==0 → `return Ok(Json(ActionResult::error(attendance appeal not found)))`（:742-744），**HTTP 200 + type:error**。同模块 error 共 49 处。
- `delete_admin_record`（:4652-4677）、`delete_owned_record`（:4680-4711）：删除行数为 0 或行不存在 → ActionResult::error（attendance admin not found / record not found），同样 HTTP 200。
- `ActionResult::error`（`oa4rust/backend/crates/shared/src/response.rs:188-200`）：设 type=error、message、size=-1、prompt=ExceptionEntityNotExist，但 HTTP 状态仍 200。
- SDK（`oa4rust/frontend/packages/sdk/src/api.ts:82-88` parseJsonEnvelope 仅解析 JSON 不检查 type；:275-296 仅 `!resp.ok` 抛错），2xx 的 type:error 原样 resolve，故 await 不抛错、裸 toast.success 误报。
- 例外：`core_entity` 的 record_delete（`oa4rust/backend/crates/attendance_core_entity/src/lib.rs:287-313`）无行返回 ownership_error → 实测 HTTP 404，前端 catch 正确报错，不属本缺陷。

## 三、覆盖甄别与 fail-loud 更正

- docs/ops 命中 r324-pc-attendance-audit.md（D1 明细解包 P1、D2 toggleAttType error 误报 P2）与 r324-pc-attendance-fix.md（:23-24 已修 D2）、r355/r356 H5 考勤。R324 D2 仅覆盖 `toggleAttType` 单函数且已修复，从未覆盖本轮批量写函数与行内写函数；本轮首次专测为有效增量。
- 对比组 `toggleAttType`（:459-475）仍保持 R324 修复后的 `if type:error → toast.error`，实测未回归。
- 首轮 S4 失败态注入（evidence.json）经 v3 精准探针确认**未生效**：注入命中 `[500,500]` 但 section 仍显示暂无请假申请——根因是申诉列表 useQuery 带 `staleTime:120000`（:378-380），reload 命中缓存不重取。fail-loud 如实记录，改由 v4 用写后 `loadAppeals()` 路径验证，不虚报通过。

## 四、E2E 实测（1440x900 headless，非物理真机）

1. 首轮（job bash-c0a0f1c64caa42cc，evidence.json）：DOM 50 按钮、展开高级操作后 86；真后端假 ID 探针 9 端点，8 个 HTTP 200 + type:error（audit/check/status/archive/workflowAppeal/adminDelete/mobileDetailDelete/appealAuditMutation，message 为 attendance appeal not found / attendance admin not found / record not found），coreRecordDelete=404。
2. 决定性对照（同轮 S2，**不拦截写路由**）：只注入申诉列表 1 行假 ID，真实点击审核/检查/改状态 → 真 POST attendanceappealInfo/audit（200 error）、/check（200 error）、/appeal/{id}（200 error）→ UI 仍弹「✓已审核申诉」「✓已标记检查」「✓已更新申诉状态」；3 操作全 clicked、writesDelta 均为 1，0 pageerror。
3. 取消守卫（同轮 S3）：点归档 → confirmMsg 自定义 DOM 弹窗（#oa4-confirm-overlay）→ 点取消 → writesBefore=12 / writesAfter=12，零写。
4. v2/v3（evidence-v2/v3.json）：确认申诉列表 useQuery staleTime 缓存导致 reload 注入失效（v3 hits=[500,500] 但无失败提示），定位为取证工具问题非产品缺陷。
5. v4（evidence-v4.json，失败态补测）：注入 1 行后切申诉列表为 500，真实点击审核触发写后 `loadAppeals()`（:1358/:1363-1371）→ section 显示「请假申请加载失败：HTTP 500: Internal Server Error」+「重试」，与空态「暂无请假申请」可区分；同帧 toast 仍为误报的「✓已审核申诉」，进一步印证缺陷。0 pageerror。

## 五、三分判定

- **真缺陷（1 个 P1）**：申诉管理行内及批量写函数在 200+type:error 下误报成功，三全满足，最小修复方向见第一节。影响面按源码归为行内写、批量写、独立写三群，代表性真 UI 覆盖审核/检查/改状态，其余同源同构。
- **取证限制**：破坏性真状态翻转/真删未授权未执行（用假 ID + 只注入列表行、写响应走真后端）；高门槛 prompt/批量输入分支按代表性覆盖未逐一 UI 触发；全量写函数按源码归类而非逐行点击。
- **产品语义差异**：ActionResult::error 用 HTTP 200 是 o2server 兼容既有设计（非本轮改）；coreRecordDelete 走 404 与其他考勤端点 200 口径不同属既有实现差异，前端已正确报错；confirmMsg 为自定义 DOM 弹窗非原生 dialog。
- **通过项**：非 2xx（404/500）正确 toast.error；confirmMsg 取消守卫 0 写；失败态与空态可区分（v4 坐实）；0 pageerror；R324 已修 toggleAttType 未回归。

## 六、交付

- 本轮只读甄别，未改任何源码/契约/后端/SDK；唯一新增文档即本文。真缺陷已留证并给出最小修复方向，交由后续修复轮处理；项目按 ≥1000 轮持续迭代，本轮不代表完成。
