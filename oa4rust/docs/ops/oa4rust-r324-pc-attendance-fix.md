# R324 PC 考勤管理三处前端修复（D1 明细解包 / D2 error 信封 / R1 建删刷新）

> 轮次：R324 后续修复轮；来源：R324 PC 深度功能审计（oa4rust-r324-pc-attendance-audit.md）发现 D1/P1、D2/P2、R1 三项。
> 本文为实施记录，不代表项目完成；项目按 ≥1000 轮持续迭代。
> 测试服地址、凭据一律以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，真值不在库中；口令仅部署期从受控脚本远端读取，运行期注入，绝不入库。

## 一、缺陷与根因

### D1[P1] 明细列表恒空
页面 `frontend/apps/desktop/src/views/AttendanceApp.vue:362` 原写 `records.value = r.data?.list ?? []`，假定后端 `data` 为 `{list,total}` 对象。
后端 `backend/crates/attendance_assemble_control/src/lib.rs:1341-1345` 的 `attendancedetail_filter_list` 以 `legacy_success(Value::Array)` 返回：`data` 实为数组，count 在信封顶层。
结果：创建明细 200、count=1，但列表取 `.list` 得 undefined → `[]`，页面恒空态。

### D2[P2] error 业务信封误报成功
`toggleAttType`（原 :454-465）仅按 HTTP 状态码判定，HTTP 200 但信封 `type:"error"`（setting not found）仍弹「已更新考勤类型启用状态」成功 toast，用户被误导。

### R1 建删不刷新
新建/删除考勤明细的成功回调未触发列表与统计重载，操作后列表/统计停留在旧值（审计次生发现）。

## 二、修复（均在 AttendanceApp.vue，最小改动）

1. D1（:362-363）：`const d=r?.data; const list=Array.isArray(d)?d:(d?.list??[]); const total=Array.isArray(d)?(r?.count??list.length):(d?.total??0); records.value=list; totalPages.value=Math.max(1,Math.ceil(total/20))`。同时兼容数组信封与对象信封。
2. D2（:460）：`const r:any=await api.post(...); if(r?.type==='error'){toast.error(r?.message??'更新失败');return} toast.success(...)`。业务 error 走错误提示并中止，不再误报成功。
3. R1：`createAttDetail`（:1009 后）与 `deleteDetail`（:1754 后）成功路径各加一次 `loadData()`，列表与统计即时回刷。

## 三、test-first RED → GREEN

契约 `frontend/apps/desktop/src/views/AttendanceApp.attendanceFix.contract.test.ts`（8 it，读源码真执行代码块）：
- D1 三项：数组 data→records 非空、{list,total}→totalPages、空信封归一。
- D2 两项：error 信封走 toast.error 不走 success、success 信封走 toast.success。
- R1 两项：建/删成功回调包含 loadData()。
- 写读链一项：端点与模板未被破坏。

红线：修复前 4 failed / 4 passed（D1 数组信封与 error 分支失败）；修复后 8/8 通过。
回归：`pnpm exec vitest run` 266 files / 2147 tests 0 failed；`pnpm run typecheck` exit 0（Scope 6/7 Done）。

## 四、测试服真浏览器验证（非物理真机）

构建：桌面前端 `pnpm --filter @oa4rust/desktop build` exit 0，入口 `index-RDg_-KGE.js` / `index-BMFkaD73.css`，考勤 chunk `AttendanceApp-CUnO28mw.js`。
部署：静态产物 tar 原子替换（先备份 web.bak，再 mv 切换），远端 grep 双哈希校验通过（DEPLOY_R324_ATTENDANCE_HASH_OK）。
验证脚本：`test-results/r324-attendance-fix-check.cjs`（gitignored），Playwright Chromium headless 1440×900，登录 `<YOUR_TEST_DOMAIN>` 测试服，真实 UI 流程：
1. 展开高级操作 → 新建考勤明细（marker 日期 2026-12-25）；
2. 列表行数 1→2、应出勤统计 1→2（即时）；
3. 刷新页面后仍 2 行（持久）；
4. 删除首行 → 行数与统计即时回退到 1/1；
5. 启用/禁用考勤类型：不存在 code → 红色「setting not found」错误 toast；真实 code → 「已更新考勤类型启用状态」成功 toast。

8 项断言全部为 true：d1_listImmediate / d1_statsChanged / d1_persistAfterReload / r1_deleteImmediate / d2_errorToast / d2_successToast / noPageErrors / noApiFailures。
pageErrors=[]，apiFailures=[]。

证据（全部为 Playwright Chromium headless 对已部署测试服渲染，**非物理真机**，来源标注见 evidence.json）：
- `test-results/r324-attendance-fix/01-after-create.png`（1440×900，sha256 4d97c76be59d923896a67f68b6beb3d51fbb571384d18df048d0c1e51e58c306）：绿勾「考勤明细已创建」、应出勤=2、明细行即时出现。
- `test-results/r324-attendance-fix/02-after-delete.png`（1440×900，sha256 462783aa8f65312957d727c747a700273414f30a3d3e60138ef3864b491b03ca）：删除后行数/统计回退到 1。
- `test-results/r324-attendance-fix/03-toggle-error.png`（1440×900，sha256 8a385aa2bc3d6a071a09afef7030acbb6e2fc32b4f47139d444ab72eb80f19b3）：红色 ✗「setting not found」错误 toast，未误报成功。
- `test-results/r324-attendance-fix/04-toggle-success.png`（1440×900，sha256 db784b43d27aded427064f277af15f83f31bd50ec6849904db6bdec1f4ef33c2）：真实 code 的成功 toast。

## 五、范围与未决

本轮仅改 PC 前端 `AttendanceApp.vue`，未触后端、H5、SDK；后端 H5 审计与修复由独立任务处理。
已知取证限制（非本轮缺陷）：考勤类型列表 enable 字段切换等不在本轮三处修复范围；保持原业务逻辑不变。
