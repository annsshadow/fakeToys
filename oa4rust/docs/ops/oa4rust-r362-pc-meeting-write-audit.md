# R362 PC 会议管理写操作链甄别审计（MeetingApp.vue）

> 审计轮次：R362（t174）。范围：PC /oa4rust/app/meeting 会议管理页 MeetingApp.vue 写操作链。
> 性质：只读甄别，不修改任何源码/契约/后端/SDK、不修复、不部署。
> 浏览器口径：Playwright Chromium headless，1440x900，对部署测试服实测；截图与 evidence 标注 Playwright Chromium headless against deployed test server; not a physical device，非物理真机。
> 环境口径：测试服域名/host/凭据仅运行期从 gitignored 脚本读取，本文档一律用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，0 真值。
> 判定纪律：①真缺陷＝真 UI 可复现＋真请求/响应＋源码定位；②取证限制＝破坏性真写未授权不执行；③产品语义差异＝设计取舍。
> 与既有轮次关系：R359 判 H5 meeting 五写端点误报成功 P1、R360 已修（判 type 加 flag）；R361 判 PC process 审批写链 0 缺陷（后端 SELECT FOR UPDATE 假 ID 直接 404）。本轮聚焦 PC meeting 写链失败信封，从未专项甄别。

## 一、结论摘要

| 编号 | 判定 | 严重度 | 一句话 |
| --- | --- | --- | --- |
| D1 | 真缺陷 | P1 | 会议写操作链对后端 200+type:success+bool:false 及 200+type:error 一律弹成功提示：假 ID 经 owner_gate 放行真实可达，accept/completed/modify 三条 UI 实测误报成功 |
| L1 | 取证限制 | — | 真会议状态翻转/真改时间属破坏性写未授权不执行；失败信封经假 ID 真探针加 route 注入对照取证 |
| P1 | 产品语义差异 | — | markMeetingCompleted 用响应 completed 布尔就地落态（列表端点 SELECT 不含 status 列），但 toast 仍无条件成功，属状态/提示两套口径的设计 |
| G1 | 回归观察（正常） | — | 取消守卫 0 写请求、403/500 正确错误提示、失败/空态文案可区分、0 pageerror |

## 二、D1【真缺陷·P1】写操作链失败信封误报成功

### 2.1 后端真契约（假 ID 探针，非注入）

页面内 fetch 用不存在 ID（R362-NOSUCH-时间戳）实测状态翻转端点，全部 200 + type:success + bool:false：

- POST meeting/{fake}/accept → 200 响应 data.accepted=false、type=success。
- POST meeting/{fake}/reject → 200 data.rejected=false、type=success。
- POST meeting/{fake}/confirm/allow → 200 data.confirmed=false、type=success。
- POST meeting/{fake}/manual/completed → 200 data.completed=false、type=success。
- POST meeting/{fake}/modify/starttime → 500（该端点对缺失 ID 另有校验，未产生 200+bool:false 形状）。

机理坐实（backend/crates/meeting_assemble_control/src/lib.rs）：

- meeting_id_accept:2660 的 UPDATE x_meeting SET status=accepted WHERE id=$1（:2672）不查 rows affected，:2681 一律 ActionResult::success 且 accepted=Bool(result > 0)；u2_meeting_owner_gate:2667 对不存在行放行，与 R359 H5 同源。
- meeting_id_manual_completed:2876 的 UPDATE status=completed（:2890）同样不查受影响行，:2899 completed=Bool(result > 0)。
- meeting_id_modify_starttime:2946 的 UPDATE（:2976）后 :2990 modified=Bool(result > 0)。
- 同族 reject:2996 / confirm_allow:2790 / confirm_deny:2815 与 R359 已坐实口径一致。

### 2.2 前端消费链（await 后无条件成功）

frontend/apps/desktop/src/views/MeetingApp.vue：

- joinMeeting:643-652：await api.post(meeting/{id}/accept) 后 toast.info(已加入会议) 加 loadMeetings，不读 data.accepted。
- markMeetingCompleted:685-697：await api.post(manual/completed) 后 meetingCompletedFromResponse(r) 仅用于就地落态（meetingStatus.ts:27-32 读 data.completed === true），随后 toast.success(已标记完成) 无条件。
- modifyMeetingTime:663-684：await api.post(modify/starttime) 加可选 modify/completedtime 后 toast.success(会议时间已更新)，不读 data.modified。
- approveMeeting:634-642 / cancelMeeting:624-633 / updateMeeting:613-623 / removeBuilding:553-562 / removeRoom:601-610 / addBuilding:540-552 / addRoom:585-600 同型：await 后仅 catch 报错，无响应体业务标志判定。
- SDK frontend/packages/sdk/src/api.ts:275-296：非 2xx 才抛错（403 转 PermissionError 没有权限执行该操作、其余转 httpStatusError）；2xx 仅 parseJsonEnvelope 等于 resp.json()（:82-88），不检查 type:error，故 200+type:error 也被当成功返回。

### 2.3 UI 实测（route 注入对照，1440x900 headless）

注入列表 1 行 R362注入待办（id R362-INJECT-1），卡片 7 按钮 加入/参会人/邀请/邀请管理/改时间/标记完成/签到码（截图 meeting-injected.png 肉眼坐实）。逐项点击并抓 .oa4-toast：

| 操作 | 注入响应 | UI toast 实测 | 判定 |
| --- | --- | --- | --- |
| 加入 | 200 success accepted:false | 已加入会议 | 误报成功 |
| 加入 | 200 type:error message=业务失败 | 已加入会议 | 误报成功 |
| 标记完成 | 200 success completed:false | 已标记完成（状态仍显示未开始） | 误报成功 |
| 改时间 | 200 success modified:false | 会议时间已更新 | 误报成功 |
| 加入 | 403 type:error | 加入失败: : 没有权限执行该操作 | 正确 |
| 加入 | 500 type:error | 加入失败: : HTTP 500: Internal Server Error | 正确 |

markMeetingCompleted 的双重矛盾尤其明确：completed:false 时 meetingCompletedFromResponse 返回 false、卡片状态保持未开始，但 toast 仍弹已标记完成，同一操作状态与提示相反。

### 2.4 最小修复方向（本轮不改码）

对齐 R360 方案 B：状态翻转端点消费响应体，判 type 为 success 且对应布尔（accepted/rejected/confirmed/denied/completed/modified）为 true 才弹成功并刷新；否则提示 message 或操作未生效请刷新后重试。modifyMeetingTime 需读 data.modified；markMeetingCompleted 已有 completed 布尔，复用同一判定即可消除状态与提示矛盾。注：SDK 2xx+type:error 不抛错是全局口径，本轮仅在业务写链处按 type 判定，不改 SDK 通用行为。

## 三、L1 取证限制 / P1 产品语义 / G1 回归观察

- L1：真状态翻转/真改时间属破坏性写，未授权不执行；失败信封由真实假 ID 探针加 route 注入对照取证，未对真实有效会议做状态篡改。
- P1 产品语义：meetingStatus.ts:4-8 注释明确列表端点 SELECT 不含 status 列，标记完成须消费响应布尔就地落态；本轮坐实其落态正确但 toast 未跟随，属状态与提示分离的设计，不单列缺陷（已并入 D1 修复方向）。
- G1 通过项：① confirm 取消守卫——点标记完成弹自定义确认框（#oa4-confirm-overlay，confirmMsg toast.ts:182）点取消后写请求增量 0；② 非 2xx——403 提示没有权限执行该操作、500 提示 HTTP 500 Internal Server Error 正确；③ 失败态 会议列表加载失败：HTTP 500 加重试，空态 暂无会议点击新建会议预订第一场，二者可区分；④ pageErrors 空数组，0 pageerror。

## 四、覆盖甄别与证据

- docs/ops 既有命中：r326-pc-meeting-audit.md（首次全模块覆盖，判 D1 搜索筛选 P2 已另修、L1 楼栋无种子数据、P1 无删除入口、G1 R309 修复生效）、r326-meeting-search-fix.md、r304-view-contract-sync.md:30、r300-b3.md:4；均非写操作链失败信封专项，本轮首次真浏览器甄别为有效增量。
- 证据（gitignored）：test-results/r362-pc-meeting-write/evidence.json 与截图 meeting-initial.png / meeting-injected.png / meeting-join-boolfalse.png / meeting-completed-false.png / meeting-fail-state.png / meeting-empty-state.png；脚本 test-results/r362-pc-meeting-write.cjs。
- 预算：本轮 1 轮 E2E（11 次写请求含假 ID 探针 5 加 UI 注入对照 6），0 pageerror；截图 1440x900 headless 非物理真机。

