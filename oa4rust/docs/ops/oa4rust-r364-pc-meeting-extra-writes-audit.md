# R364 PC 会议管理批量写函数链甄别审计（MeetingApp.vue）

> 审计轮次：R364（t177）。范围：PC /oa4rust/app/meeting 会议管理页 meetingWrite / meetingMore / meetingMore2 三个批量写函数链。
> 性质：只读甄别，不修改任何源码/契约/后端/SDK、不修复、不部署。
> 浏览器口径：Playwright Chromium headless，1440x900，对部署测试服实测；截图与 evidence 标注 Playwright Chromium headless against deployed test server; not a physical device，非物理真机。
> 环境口径：测试服域名/host/凭据仅运行期从 gitignored 脚本读取，本文档一律用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，0 真值。
> 判定纪律：①真缺陷＝真 UI 可复现＋真请求或响应＋源码定位；②取证限制＝破坏性真写未授权不执行；③产品语义差异＝设计取舍。

## 一、结论摘要

| 编号 | 判定 | 严重度 | 一句话 |
| --- | --- | --- | --- |
| D0 | 用户可达真缺陷 | 无 | 三函数 UI 无入口（死代码），不满足真 UI 可复现，不判真缺陷 |
| L1 | 潜在隐患（非用户可达） | 提示 | 三函数对 200+success+bool:false 与 200+type:error 无条件弹成功，源码级真执行坐实 |
| P1 | 产品语义差异 | — | 三函数属历史调试端点集合，业务契约明确要求不渲染入口，属有意设计 |
| G1 | 回归观察（正常） | — | confirm 取消守卫 0 写、403 正确错误、失败/空态可区分、0 pageerror、R363 五函数入口正常 |

本轮最重要的诚实发现：任务描述以 meetingAction / meetingMore 命名的两个批量写函数，源码实际为 meetingWrite（:836-889）、meetingMore（:891-919）及其后续 meetingMore2（:921-942）。三者在当前生产构建中均无 UI 入口，属不可由普通用户触发的死代码。因此，尽管其失败信封误报成功的逻辑缺陷客观存在（源码级真执行 9/9 坐实），按真缺陷纪律（须真 UI 可复现）不判真缺陷，改列为 L1 潜在隐患，并在 §四给出最小修复方向。

## 二、源码与 UI 入口甄别（为何不判真缺陷）

### 2.1 三函数写链结构

D:/WORKSPACE/fakeToys/oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue：

- meetingWrite(:836-889)：约 20 个 op 分支，覆盖 accept/reject/confirmAllow/confirmDeny/checkin/delete/buildingEdit/buildingDelete/roomEdit/roomDelete/roomPhoto/config/coreRoomCreate/coreRoomSave/attDelete/mtgCreate/mtgCtrlCreate/sysCfgManage/attFromPp 及缺省 attachment update；:885 无条件 toast.success（会议操作已提交），:886-888 仅 catch 报 toast.error。
- meetingMore(:891-919)：addInvite/delInvite/modifyStart/modifyComplete/manualComplete/coreSave/核心会议删除；:915 无条件 toast.success。
- meetingMore2(:921-942)：ctrlDelete/meetingDelete/roomDelete/附件下载；:938 无条件 toast.success。
- 三函数 await api.post/put/delete/get 后均不读取响应 type / 布尔标志，hasHelper=false（不调用 R363 新增的 meetingWriteSucceeded）。

### 2.2 UI 入口审计（真浏览器 + 源码双向确证）

- 模板（:40-114）仅有 7 个会议卡按钮：加入/参会人/邀请/邀请管理/改时间/标记完成/签到码，全部调用 R363 已修的五个状态翻转函数，无一调用三批量函数。
- 全 desktop src 搜索：meetingWrite / meetingMore / meetingMore2 仅出现在定义处与契约测试引用，无任何 @click 调用点。
- 业务契约 MeetingApp.business.contract.test.ts:43-48 明确断言默认视图不渲染 /api/* 端点调试按钮，且 template 不含调用 meetingWrite|meetingMore 的 @click。
- 真浏览器 DOM 审计（48 个可交互元素）：htmlHasFnName=false、htmlHasApiPath=false；无 defineExpose、无 window 全局暴露，生产构建 __vueParentComponent 探针 0 命中（devtools 被剥离）。
- 结论：三函数不可经真实 UI 触达，普通用户流程无法复现其误报，故不构成用户可达真缺陷。
## 三、真浏览器 E2E 实测（1440x900 headless）

脚本：gitignored test-results/r364-pc-meeting-extra-writes.cjs，对部署测试服实测，node 运行 exit=0。

### 3.1 假 ID 端点探针（R364-NOSUCH-*）

- 200+type:success 且布尔 false（真实可达的失败形状）：manualComplete（completed:false）、ctrlDelete（deleted:false）、meetingDelete（deleted:false）。
- 200+type:success 且布尔 true（真成功，不计误报）：addInvite（added:true）、delInvite（deleted:true）——端点对不存在会议仍 INSERT 新行，属既有设计。
- 200+type:error：coreRoomSave（room not found）、coreMeetingDelete（meeting not found）。
- 404：buildingEdit / buildingDelete / roomEdit / roomDelete / attachmentUpdate / attDelete。
- 400：roomPhoto（Invalid boundary）、coreMeetingSave（invalid startTime）。
- 500：modifyStart / modifyComplete。
- 说明：因三函数无 UI 入口，本探针为源码级端点形状探测，不通过真实 UI 触发；仅用于确证后端确实产生 200+success+bool:false 与 200+type:error 形状（与 R362 会议写链同型）。

### 3.2 页面失败态与空态（真 UI 可见）

- 会议列表注入 HTTP 500：页面显示「会议列表加载失败：HTTP 500: Internal Server Error」并带重试入口。
- 空数据：显示「暂无会议」与「点击新建会议预订第一场」。
- 失败态与空态文案可区分，错误不被空态掩盖。
- 0 pageerror：整轮 E2E 无未捕获前端异常。
- 截图（gitignored）：meeting-dom-audit.png（48 交互元素 DOM 审计）、meeting-fail-state.png、meeting-empty-state.png。

## 四、L1 潜在隐患：源码级契约真执行坐实

由于三函数无 UI 入口，常规页面点击无法驱动；本轮改用从 MeetingApp.vue 源文提取三函数函数体，注入受控 api / prompt / confirmMsg / toast 桩后真执行（gitignored test-results/r364-contract.cjs），9/9 case 全部 pass，逐条坐实逻辑而非静态推断：

| case | 检查 | 结果 |
| --- | --- | --- |
| W-accept-boolFalse | POST accept 返回 200+success+accepted:false | 弹「会议操作已提交」（误报） |
| W-accept-typeError | 200+type:error | 同样弹成功（误报） |
| W-buildingEdit-boolFalse | PUT building 返回 bool:false | 弹成功（误报） |
| W-delete-cancel | confirm 取消 | 0 写请求，0 toast（守卫正确） |
| M-addInvite-boolFalse | PUT add/invite 返回 bool:false | 弹成功（误报） |
| M-manualComplete-boolFalse | GET manual/completed 返回 bool:false | 弹成功（误报） |
| M-coreDelete-cancel | confirm 取消 | 0 写请求（守卫正确） |
| M2-ctrlDelete-boolFalse | DELETE 返回 bool:false | 弹成功（误报） |
| W-accept-403 | 403 权限错 | toast.error「会议操作失败: 403 没有权限执行该操作」（正确） |

hasHelper=false 已确证：三函数均不调用 R363 新增的 meetingWriteSucceeded，不读取 type / 布尔标志。若未来任一函数被接入 UI，将在 200+success+bool:false 或 200+type:error 下复现 R362 同型 P1（误报成功、掩盖业务失败）。

## 五、三分判定明细

- 真缺陷：0 个。三函数误报逻辑客观存在但无 UI 入口，不满足真 UI 可复现的判定门槛，不判真缺陷。
- 取证限制：破坏性真状态翻转（真接受/真删除/真签到等）属写操作，未获授权不执行；失败信封形状以假 ID 探针＋源码级函数真执行替代 UI 触发，已如实标注非真实用户流程。
- 产品语义差异：三批量函数属历史调试端点集合；业务契约 MeetingApp.business.contract.test.ts 明确要求默认视图不渲染端点调试入口，属有意设计而非缺陷。
- 通过项（回归正常）：confirm 取消守卫 0 写、403 非 2xx 正确错误提示、失败/空态文案可区分、0 pageerror、R363 五函数入口工作正常。

## 六、最小修复方向（仅建议，本轮不改码）

若决定激活或保留三函数：对齐 R363 方案 B —— 引入 meetingWriteSucceeded(resp, flagKey) 统一判 type=success 且对应布尔字段为 true 才 toast.success，否则 toast.error；对 confirm 类写操作保留取消守卫 0 写。若确认永久无入口，建议直接删除三处死代码以消除潜在误报面。此项交队长与实现 agent 决策，本轮仅甄别不实施。

## 七、覆盖甄别

docs/ops 既有 r362 / r363 / r326-pc-meeting-audit 等覆盖会议卡五状态翻转函数与首次全模块审计，均未专项甄别 meetingWrite / meetingMore / meetingMore2 三批量写函数链的端点形状与 UI 可达性。本轮首次专测，构成有效增量。

## 八、证据与交付

- 证据目录（gitignored）：test-results/r364-pc-meeting-extra-writes/（evidence、截图）与 test-results/r364-pc-meeting-extra-writes.cjs、r364-contract.cjs。
- 截图口径：Playwright Chromium headless against deployed test server; not a physical device，非物理真机。
- 本文档为唯一提交文件；未改源码、契约、后端、SDK、台账或其它页面。
