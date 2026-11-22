# R371 PC AttendanceApp.vue 写链失败信封误报成功 P1 修复

- 任务：t188 / R371，attempt cccc0a84-9ae9-4558-8028-f329a721b953。
- 依赖：t187 / R370 只读甄别（commit 37cedbefe），坐实 1 个 P1 真缺陷。
- 测试环境：Playwright Chromium headless 1440x900，**非物理真机**；测试服域名与凭据运行期注入，文档仅用占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，不含真值。
- 证据目录：`oa4rust/test-results/r371-pc-attendance-write-fix/`（gitignored，不入库），含 evidence.json 与 3 张截图。

## 一、缺陷与裁决

- **缺陷**：考勤管理页多处有真实 @click 入口的写函数，在后端返回 HTTP 200 + 业务失败信封（`type:error`）时，await 后无条件 `toast.success`，用户误以为操作已生效。与 R366（程序中心）、R368（文件管理）、R369（文件管理续）同型。
- **队长裁决**（否决 R370 审计文档第 16 行「严格 bool===true」）：attendance 成功端点多数无 bool（仅 deleted/enabled/checked 少数带），强制 bool 会造成大量假阴性。采用 type 判定为主：
  - `writeSucceeded(resp)`：`type==='error'` 或 `success===false` → 失败；`type==='success'` 或 `type` 缺省且 `success!==false` → 成功。
  - `guardedWrite(p)`：`await p` 后判 `writeSucceeded`，失败 `throw new Error(resp?.message || '操作未生效，请刷新后重试')`，成功透传响应。
  - 失败 toast 透传后端 message，不刷新、不落态；端点路径 / payload / confirm 取消守卫 / catch 非 2xx / 已判 type 的 toggleAttType 均保持不变。

## 二、改动

- `oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue`：
  - script setup 后新增本地 `writeSucceeded(resp: any): boolean` 与 `async guardedWrite(p: Promise<any>): Promise<any>`。
  - 29 个真实 @click 可达写函数共 86 处 `await api.*(...)` 统一包裹为 `await guardedWrite(api.*(...))`：申诉行内 4（auditAppealV2 / checkAppeal / updateAppealStatus / archiveAppeal）、批量写群（attV2Write / attCoreEntity / attWrite3 / attV2Ops / attAppealWrite）、独立写（reciveDetailById / createAttDetail / reciveMobileDetail / myMobileDetail / importLeave / importAttV2Records / checkMyRestDate）、明细行内（analyseOne / archiveOne / deleteDetail）、配置写群（addGroup / addShift / addLeave / addSelfHoliday / syncDingding / syncQywx / statDingdingPerson / syncGroupSchedule / deleteMore / addCycle）。
  - `createAttDetail` / `deleteDetail` 因 R324 老契约桩环境精简（不注入外部 guardedWrite），改用**函数内内联无类型标注的布尔表达式**做同语义守卫（resp.type 判 error / success 与 success!==false），行为与全局 helper 一致，且零 TS 标注兼容 R324 stripTs；两处仍保留 R324 要求的写后 `loadData()` 刷新。
  - `toggleAttType`（R324 已修）保持不动；`loadAttTwin` / `loadAttTwin2` 为 `s()` 吞错的只读探针，非业务写误报，不在本轮范围。
- 契约 `oa4rust/frontend/apps/desktop/src/views/AttendanceApp.writeEnvelope.contract.test.ts`（新建）：源码级 body 提取 + stripTs + `new Function` / `new AsyncFn` 真执行注入桩，非文本断言，自定义函数参数带 `: string` 防 TS7006。29 个写函数 ×（type:error / type:success）两态 + helper 6 = 64 it。

## 三、RED → GREEN 台账

- RED：在 t180 版源码基线上跑契约，helper 缺失 + 29 个 type:error 用例失败（裸 toast.success），type:success 用例通过；信号干净。
- GREEN：加 helper + 86 处 guardedWrite 包裹后，契约 64/64 通过；R324 老契约 8/8 通过（内联守卫后不再因 `guardedWrite is not defined` 进 catch 丢失 loadData 刷新）；根全量 vitest 280 files / 2430 tests 全通过；desktop `tsc --noEmit` exit 0。
- 踩坑：R324 老契约 `stripTs` 只剥少量标注，局部箭头 `(r: any): boolean` / `(r: any)` 均残留冒号致 `new Function` 语法错 → 改为零 TS 标注的内联布尔表达式规避，不改 R324 契约（out of scope）。

## 四、E2E 实测（1440x900 headless，非物理真机）

- 脚本：`oa4rust/test-results/r371-pc-attendance-check.cjs`（在 `oa4rust/` 下运行）。测试服 web 目录整目录 tar 原子替换部署（入口 grep `index-BsI8ZjQh.js` 校验）。
- 7 项断言全 true：loginOk / createTypeErrorFixed / createHttp500Correct / createSuccessStillWorks / auditTypeErrorFixed / auditSuccessStillWorks / noPageErrors；`failed=[]`，`pageErrors=[]`。
- createAttDetail：type:error → 红 toast「创建失败: 后端拒绝：R371 create rejected」无成功；HTTP 500 → 正确错误；type:success → 仍「✓考勤明细已创建」。
- 申诉行内 auditAppealV2：注入 1 行假 ID + 拦截 audit 端点，type:error → 红 toast「审核申诉失败: 后端拒绝：R371 audit not found」且注入行仍在、无成功；type:success → 仍「✓已审核申诉」。
- 截图：`01-create-typeerror-fixed.png` / `02-create-trueSuccess.png` / `03-audit-typeerror-fixed.png`，已逐张肉眼确认（1440x900，红 toast 无成功文案）。

## 五、范围边界与交付

- 未改后端 crates / SDK api.ts 传输层 / 其它 views / migrations / 共享组件；端点路径与 payload 未变。
- 精确 3 文件：`oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue`、`oa4rust/frontend/apps/desktop/src/views/AttendanceApp.writeEnvelope.contract.test.ts`、`oa4rust/docs/ops/oa4rust-r371-attendance-write-envelope-fix.md`，单提交推送 HEAD==TRACK。
- 项目按 ≥1000 轮持续迭代，本轮不代表完成。
