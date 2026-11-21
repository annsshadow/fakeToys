# R324 PC 考勤管理深度功能审计（首次覆盖）

> 审计轮次：R324（t115）｜范围：PC `/oa4rust/app/attendance` 考勤管理页（`AttendanceApp.vue`，`data-testid="attendance-page"`，`h1=考勤管理`）
> 性质：只读深度功能审计，不修改任何源码/契约/配置。
> 浏览器口径：Playwright Chromium headless，1440×900，本地渲染对部署测试服实测；截图与 evidence 的 `source` 均标注 `Playwright Chromium headless against deployed test server; not a physical device`，**非物理真机**。
> 环境口径：测试服域名/host/凭据仅在运行期从 gitignored 脚本读取，本文档一律用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，0 真值。
> 判定纪律：①真缺陷=真 UI 可复现 + 真请求/响应 + 页面回显 + 源码定位；②取证限制=无种子数据/无凭据无法驱动；③产品语义差异=设计取舍。不把②③当 bug，不臆造。
> 与既有审计的关系：R301/R305 仅覆盖 H5 骨架与 P0A 视觉；R300 P1 统计卡口径已在源码 `AttendanceApp.vue:330-350`（`deriveAttendanceStats` 由真实明细派生去重）闭环。本轮聚焦该页从未深度审计的 CRUD 链，不重复已闭环项。

## 一、结论摘要

| 编号 | 判定 | 严重度 | 一句话 |
| --- | --- | --- | --- |
| D1 | 真缺陷 | P1 | 明细列表把后端“数组 data”当 `{list,total}` 读，创建成功后明细永不显示 |
| D2 | 真缺陷 | P2 | 业务错误信封 `type:"error"` 被前端当成功，弹出“已更新…成功”toast |
| R1 | 次生风险点（未独立计缺陷） | — | 新建/删除明细后不刷新列表与统计，D1 修复后需一并验证 |
| L1 | 取证限制 | — | 请假申请/申诉、外部钉钉企微同步无种子数据与凭据，无法走闭环 |
| P1 | 回归观察（正常） | — | 打卡地点、设置项、v2 考勤组新建，核心记录建/改，全程 0 pageerror |

## 二、D1【真缺陷·P1】明细列表 data 数组解包不匹配 → 有数据不渲染

**可复现路径**：登录 → `/oa4rust/app/attendance` → 展开「⚙ 高级操作」→「打卡记录」组点「➕ 新建考勤明细」→ 输入人员 admin、日期 2026-11-20、状态 normal → toast 提示“考勤明细已创建”→ 回到明细列表区，仍显示空态「暂无考勤记录，可刷新后查看最新数据」→ 点「刷新记录」仍为空。

**真请求/真响应**（Playwright 页面内 `fetch`，会话态）：
- 创建：`POST /oa4rust-api/api/attendance/assemble/control/attendancedetail` → 200 `{"data":{"created":true,"id":"c48481aa-15f4-4cbf-9232-c973e6a0f629","personId":"admin","date":"2026-11-20"},"type":"success"}`。
- 列表：`GET /oa4rust-api/api/attendance/assemble/control/attendancedetail/filter/list?month=2026-11&page=1&size=20` → 200 `{"data":[{"date":"2026-11-20","id":"c48481aa-15f4-4cbf-9232-c973e6a0f629","personId":"admin","status":"normal"}],"type":"success","count":1}`，探针判定 `Array.isArray(parsed.data) === true`、`data.length === 1`。
- 页面回显：`recordRows` 创建前 0、点「刷新记录」后仍 0；4 张统计卡全 0；`pageErrors=[]`。

**源码定位**：
- 前端 `oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue:353-370`：`:362 records.value = r.data?.list ?? []`、`:363 totalPages.value = Math.ceil((r.data?.total ?? 1) / 20)` —— 读的是 `.list`/`.total` 字段。
- 后端 `oa4rust/backend/crates/attendance_assemble_control/src/lib.rs:1296-1346`：`attendancedetail_filter_list`，`:1341-1345 return Json(ActionResult::legacy_success(Value::Array(data), count, 0))` —— `data` 本身就是数组，`count` 在信封顶层，没有 `.list`/`.total`。
- 二者字段结构不一致：`r.data?.list` 取不到值 → `records.value=[]` → 模板 `:157 v-else-if="records.length===0"` 命中空态。

**证据**：`test-results/r324-attendance-audit/decisive-evidence.json`（`rowsBefore:0`、`rowsAfterRefresh:0`、`raw.isArray:true`、`raw.dataLen:1`、`raw.count:1`）；截图 `01-initial.png`（sha256 `029c0cfdf5cde2345798fedcd3c2f13e9eae966cf09ff73edaecd4bfaf2fb307`）、`03-detail-decisive.png`（sha256 `6d9f4078f10fdb4dc248ae7c39ee171a82c0b7f26c964c1433046696478085c6`，1440×900，可见空态与全 0 统计卡）；真请求日志 `crud-evidence.json`。脚本 `test-results/r324-attendance-decisive.cjs`、`r324-attendance-crud.cjs`（均 gitignored）。

**修复方向**（不在本只读任务内实施）：在 `AttendanceApp.vue:362-363` 归一化两种形态，例如 `const d = r?.data; const list = Array.isArray(d) ? d : (d?.list ?? []); const total = Array.isArray(d) ? (r?.count ?? list.length) : (d?.total ?? 0); records.value = list; totalPages.value = Math.max(1, Math.ceil(total / 20))`。不得改后端返回结构（其它消费者依赖 `legacy_success`）。

**验证方式**：定点契约锁“数组 data 与 `{list,total}` 两种信封都能得到非空 `records`”；真浏览器建一条明细后列表立即出现该行、`recordRows>=1`、统计卡随明细变化、刷新后持久；`0 pageerror`。

## 三、D2【真缺陷·P2】业务错误信封 `type:"error"` 被误报为成功

**可复现路径**：登录 → 考勤页 → 高级操作 →「🔧 启用/禁用考勤类型」→ 输入一个不存在的考勤类型编码（如 `NOT_EXIST_TYPE`）→ 应用内确认框点「确认」→ 弹出成功 toast「已更新考勤类型启用状态」，而后端实际返回业务失败。

**真请求/真响应**（对照实验）：
- 不存在 code：`POST /api/attendance/assemble/control/attendancesetting/enable/type` → HTTP **200** 但 body `{"type":"error","message":"setting not found","prompt":"com.x.base.core.project.exception.ExceptionEntityNotExist"}`；页面 toast = `✓ 已更新考勤类型启用状态`（成功语义）。
- 真实存在 code（先经「+ 新增」建 `R324_REAL_*`）：同一端点 → HTTP 200 body `{"data":{"code":"R324_REAL_<时间戳>","enabled":true,"updated":1},"type":"success"}`（真实 code 由「+ 新增」现场生成，形如 `R324_REAL_数字时间戳`）；页面 toast 完全相同 `✓ 已更新考勤类型启用状态`。
- 说明：失败与成功在 UI 上无法区分，用户被误导以为启用/禁用已生效。

**源码定位**：
- `oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue:454-465` `toggleAttType()`：`:460 await api.post(...)` 未接收返回值也未检查信封，`:461 toast.success("已更新考勤类型启用状态")` 无条件成功。
- `oa4rust/frontend/packages/sdk/src/api.ts`：`post():303-305` 直接转 `request()`；`request():230+` 与 `parseJsonEnvelope():82-88` 仅按 HTTP 状态（401/403/404/网络）抛错，**不检查业务信封 `body.type === "error"`**。
- 全 desktop views 仅 `ProgramCenterApp.vue:1497/1511/1515/1548` 四处手动判 `r?.type === "error"`；说明这是跨页系统性模式，考勤页是可复现实证。

**证据**：`test-results/r324-attendance-audit/toggle-record-evidence.json`（error 信封 + 成功 toast 同帧）、`toggle-contrast-evidence.json`（真实 code `type:"success", updated:1` 对照）；截图 `05-toggle-record.png`（sha256 `821d33a3a9b52f835dd10d81e62e5a1957cfddc5ed3fdaef99bbc7768585116d`，1440×900）、`06-toggle-realcode.png`。脚本 `r324-attendance-toggle.cjs`、`r324-attendance-toggle-contrast.cjs`（gitignored）。

**修复方向**（最小、限 `AttendanceApp.vue`）：`const r:any = await api.post(...)`；若 `r?.type === "error"` 则 `toast.error(r?.message ?? "更新失败")` 并 return，否则 `toast.success(...)`。更彻底的方案是在 SDK 层统一把 `type:"error"` 归一为 `ApiError`，但影响面大、应另立任务评估全站调用方。

**验证方式**：契约断言“error 信封走 error 分支、success 信封走 success 分支”；真浏览器对不存在 code 显示错误 toast、真实 code 显示成功 toast，二者可区分，`0 pageerror`。

## 四、R1 次生风险点（不独立计缺陷，D1 修复后需回归）

- `AttendanceApp.vue:1002-1013 createAttDetail()`：`:1008 POST attendancedetail` 成功后仅 `:1009 toast.success`，未 `invalidateQueries(["att","recs",...])`，列表不会自动刷新。
- `AttendanceApp.vue:1749-1758 deleteDetail()`：`:1752 DELETE` 成功后仅 `:1754 records.value = records.value.filter(...)` 本地移除，未 refetch，统计卡与分页 `total` 不同步（与 R321 文件回收站刷新缺陷同构）。
- 当前因 D1 使列表恒空，上述刷新的可见后果被掩盖；修 D1 后须一并验证“建/删后列表与统计即时一致”。
- 作为对照，申诉相关 `auditAppealV2/checkAppeal/updateAppealStatus/archiveAppeal`（`:407-452`）与 R321 文件回收站修复均已在写后 `await loadAppeals()` / `loadRecycleBin()` 刷新，此处应取齐同一口径。

## 五、L1 取证限制（非缺陷）

- 请假申请/申诉闭环：`GET /api/attendance/appeal/list` 200 返回 `data:[]`，当前账号无请假/申诉种子数据；「通过/驳回/审核/检查/改状态/归档」按钮无对象可驱动，属数据缺失而非 UI 缺陷。请假列表读 `AttendanceApp.vue:371-375` 与 `:1352-1361` 均为 `(r.data ?? [])`，`data` 为数组，解包正确。
- 外部考勤同步（钉钉/企微）：`AttendanceApp.vue:1583-1611/1613-1626` 需日期区间与外部凭据，测试环境未配置，后端会如实返回错误；本轮不驱动，避免把“未配置”误判为 bug。
- 排班配置各 tab（排班/员工配置/自助假期/工作日/管理员/导入记录/统计日志）：初始 `list/all` 均 200 `data:[]`，无种子；v2 考勤组建/删可用（见 P1），其余配置建删入口需 prompt ID，无可删对象。
- 明细逐行「详情/分析/归档/删除」：因 D1 列表恒空，行按钮不可见；D1 修复后再验（属依赖前置缺陷，不单列新缺陷）。

## 六、P1 回归观察（正常项，记录口径备查）

- 页面入口与外壳：`data-testid="attendance-page"` 存在，`h1=考勤管理`；侧栏高亮「考勤」、面包屑「工作台 / 协作 / 考勤管理」；月份选择器和高级操作面板可见。
- 打卡地点：`+ 新增` 两次 prompt → `POST /workplace` 200 `created:true` → 自动 `GET list/all` 200 `data:[1]` → DOM 条目 0→1，名称/地址正确回显（脚本 `r324-attendance-crud.cjs`）。
- 设置项：`+ 新增` 三次 prompt → `POST /attendancesetting` 200 → 列表回显 `R324_CODE_* = 1`，0→1。
- v2 考勤组：切 tab 后 `POST /v2/group/list/1/size/50` 200；`+ 新建考勤组` → `POST /v2/group` 200 `created:true` → 自动 `switchMore("v2group")` 回读 0→1。
- 核心记录：`POST /core/entity/record/create` 200 success（返回 id/user_id/status）；对不存在的 id 调 `/update` 返回 404，前端正确 toast.error「操作失败：请求的资源不存在或已被删除」——说明 HTTP 层错误映射正常，与 D2 的“业务信封错误”路径不同。
- 全程 `pageErrors=[]`；`apiFailures` 仅上述预期 404 一条，其余 /attendance 端点均 200。

## 七、证据与复现资产

所有脚本与截图/JSON 落 gitignored 目录 `test-results/`，不入库：
- 脚本：`r324-attendance-audit.cjs`（基础）、`r324-attendance-crud.cjs`（地点/设置/明细）、`r324-attendance-decisive.cjs`（D1 决定性）、`r324-attendance-other.cjs`（v2 组）、`r324-attendance-toggle.cjs`（D2）、`r324-attendance-toggle-contrast.cjs`（D2 对照）。
- 目录 `test-results/r324-attendance-audit/`：`01-initial.png`、`02-after-crud.png`、`03-detail-decisive.png`、`04-other-crud.png`、`05-toggle-record.png`、`06-toggle-realcode.png`；`evidence.json`、`crud-evidence.json`、`decisive-evidence.json`、`other-crud-evidence.json`、`toggle-record-evidence.json`、`toggle-contrast-evidence.json`。
- 全部截图 1440×900，来源均为 Playwright Chromium headless 对部署测试服实测，非物理真机；证据 JSON 的 `source` 字段已逐份标注。
- 凭据：运行期经 ssh 调用 `test-results/extract-remote-creds.py` 读取，未写入脚本/截图/文档/提交，未回显。

## 八、建议后续轮次

1. 独立 implementation 任务修 D1（数组/对象双形态解包 + 建删后刷新），test-first RED→GREEN，真浏览器复验明细可见与统计同步。
2. 独立 implementation 任务修 D2（考勤页 error 信封分支）；SDK 层统一 error 归一另立评估任务，不在此处扩面。
3. 上述修复后再补一轮请假/申诉与明细行操作审计（需先备种子数据），不重复本轮已覆盖链。
