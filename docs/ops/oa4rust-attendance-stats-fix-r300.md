# R300 PC 考勤统计硬编码修复与真浏览器复验（t31）

- 来源声明：本文结论来自测试服真浏览器（Playwright Chromium headless，1440×900）真实 UI 登录与页面操作，截图为浏览器自动化实测，非物理真机、非静态假数据；测试服域名统一用 `<YOUR_TEST_DOMAIN>` 占位，口令在运行时从测试服凭据文件读取，不入库、不在本文回显。
- 关联：`docs/ops/oa4rust-defect-audit-r300.md` P0-2「PC 考勤统计 45/42/3/2 硬编码」；A2 逐页审计 `docs/ops/oa4rust-page-audit-r300.md`。
- 范围：仅 PC `AttendanceApp.vue` 统计卡；明细接口、业务逻辑、H5、后端均未改。

## 1. 缺陷与根因
- 现象：R300 基线 PC `/app/attendance` 统计卡固定显示 应出勤 45 / 实际出勤 42 / 迟到 3 / 请假 2，而同月明细列表为「暂无考勤记录」，统计与明细数据源不一致（A2/A3 已记录）。
- 根因定位：`oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue:329` 原 `const stats = computed(() => [...恒定值 45/42/3/2...])` 为前端硬编码常量；明细走真实接口 `GET /oa4rust-api/api/attendance/assemble/control/attendancedetail/filter/list?month=&page=&size=20`（:340-342），取 `r.data?.list`（:344）。二者天然不一致，属 UI 假数据缺陷而非偶发接口故障。

## 2. Test-first RED → GREEN
- 契约测试：`oa4rust/frontend/apps/desktop/src/views/AttendanceApp.stats.contract.test.ts`。用 `ts.transpileModule` 从 `<script setup>` 转译、`new Function` 真实执行 `deriveAttendanceStats`；`extractFn` 按大括号配对提取函数源码。5 条用例：①无硬编码 45/42/3/2；②存在可执行 `deriveAttendanceStats` 且 `stats` computed 由 `records` 派生；③空明细四项归零；④非空明细按「人员×日期」去重同口径（正常/迟到/请假）；⑤业务明细接口与 `data-testid="attendance-stats"` 保留。
- RED：首次运行 4 failed / 1 passed（`AssertionError: expected undefined to be 2` 等），证明原实现不合格；测试数据里同人同日两条被正确去重，已改为不同日期（2026-11-01 / 2026-11-02）并期望应出勤=3。
- GREEN 实现：新增纯函数 `deriveAttendanceStats(rows: R[]): Array<{label:string;value:number;color:string}>`，`keyOf=(r)=>personName/name/id + '|' + date.slice(0,10)`，`Map` 去重；`actual` = 有 `checkInTime/checkOutTime` 或 `status` 为 '1'/'2'；`late` = `isLate===true || status==='2'`；`leave` = `status==='3'`；`const stats = computed(() => deriveAttendanceStats(records.value))`。队友既有副标题改动（「更多维护操作可展开高级操作」）完整保留。
- GREEN 验证：`vitest run src/views/AttendanceApp` — 2 文件 / 9 测试全绿（exit 0）。

## 3. 静态验证与部署
- `pnpm --filter @oa4rust/desktop typecheck` EXIT=0；`pnpm --filter @oa4rust/desktop build` EXIT=0（新产物 `oa4rust/frontend/dist/web/assets/AttendanceApp-P_fpj-hd.js`、`index-BM6UG1wT.js`）。
- 原子部署测试服（脚本 `oa4rust/test-results/r31x-deploy.sh`，gitignored）：`tar czf web-r31x.tgz -C oa4rust/frontend/dist web` → ssh 备份 `web.bak-r31x-20261121-0350` → scp 至临时目录 → 远程解包到 `web.incoming-$STAMP` → `mv` 原子替换 → `systemctl restart oa4rust`。STAMP=r31x-20261121-0350、TAR_BYTES=817149、BACKUP_OK、SWAP_OK；远程 `systemctl is-active oa4rust=active`；远程 `web/index.html` hash=`index-BM6UG1wT.js`、`AttendanceApp-P_fpj-hd.js` 已上线。

## 4. 测试服真浏览器复验
- 脚本：`oa4rust/test-results/r31x-attendance-verify.cjs`（gitignored）。真 UI 登录（占位符 `请输入用户名`/`请输入密码`，凭据运行时从测试服读取）→ `/app/attendance` → 抓 UI 统计卡与同口径明细比对 → 切空月份（2020-01）验证归零 → 截图 + evidence。
- 结果：`VERIFY {"match":true,"zeroOk":true,"apiBadCount":0,"apiBad":[],"pass":true} MONTH 2026-11 EXPECTED [0,0,0,0] UI [0,0,0,0]`。当前月明细实为 0 条，统计卡 0/0/0/0 与明细同口径；空月份同样归零；无异常接口响应。
- 证据：`oa4rust/test-results/r31x-attendance-verify/evidence.json`、`attendance-current.png`（1440×900，159428B）、`attendance-empty-month.png`（1440×900，143435B）。目视确认统计卡 0/0/0/0、月份切换生效、无报错条。截图为自动化浏览器实测，非物理真机。

## 5. 结论与后续
- 硬编码统计已消除：统计卡由当月真实明细派生，空明细归零，与列表同口径；接口与业务分区未改。
- 后续轮次若测试服出现真实明细，应以同一口径复核统计值；本文不声称项目完成，后续按队长评审继续迭代（≥1000 轮）。
