# R93 RecomputesView 骨架屏一致性 — 真浏览器 QA 报告

- 任务：t211（attempt_id=8a1ee448-65f6-4491-978f-ec59ff5d2cfd），承接 t209 规范 RED / t210 实施 GREEN。
- 结论：**PASS**。三视口加载态与数据到达态均符合契约，骨架屏与共享范式一致，无横向溢出；全部门禁通过（全量 vitest 首轮一条既存超时 flaky，单测与全量重跑均全绿，详见 §5）。
- 工作树 HEAD=43a252d90ee529fc31a02d02f9444278f8feada4（未变）。
- 零真值：本轮仅用 init-script 内占位令牌（r93-qa-placeholder），未写真值文件，auth 四路径 ABSENT。

## 1. 范围与方法

- 真浏览器：agent-browser 0.41.1（Chromium），三视口 390×844 / 768×1024 / 1280×800。
- 前端：admin vite dev 5175（base=/laiyipao-admin/），页面 `#/admin/recomputes`。后端 8080 未启动，故用 init-script patch `window.fetch` 注入合成响应（如下）；令牌为占位串，不写真值。
- mock：`/admin/me` 立即 200；`/admin/recomputes/stats` 与 `/admin/recomputes` 延迟 5000ms；`/admin/recomputes/mode` 立即 200；`/admin/battles` 同样延迟用于范式对照。合成总账 total=42 / mismatched=7 / rate=16.7% / mode=enforce，列表 4 行。
- 双 open 预热（独立会话）确保 init-script 在目标 origin 生效；`eval --stdin < file` 注入探针，`screenshot <完整路径>` 落图。
- 探针字段：skeletonCount/skRole/skAriaBusy/skKpiCards/skBars/skRows/maskCount/gridMaskCount/statMaskCount/kpiValues/tableRows/resultCount/skVisibleKpi/hOverflow/isLogin/bodyLen/hits。

## 2. 三视口实测（加载态 / 到达态 / 刷新 / 范式对照）

证据文件：`data/{390,768,1280}-{load,arrive,refresh-mid,refresh-after,pattern}.txt`。

### 2.1 加载窗口（mock 延迟 5000ms 内）

三视口（390/768/1280）逐项实测值完全一致：

| 断言 | 实测值 |
| --- | --- |
| skeletonCount / skeletonVisible | 1 / 1 |
| skRole / skAriaBusy | status / true |
| skKpiCards / skBars / skRows | 4 / 35 / 7 |
| gridMaskCount（data-grid 无 v-loading 遮罩） | 0 |
| statMaskCount / maskVisible | 1 / 0（stat-row 遮罩存在但不可见，未污染首屏） |
| kpiValues | 四格全为 '—' |
| tableRows | 0 |
| skVisibleKpi | true（骨架覆盖 KPI 区） |
| hOverflow | false（scrollWidth === innerWidth，分别为 390/768/1280） |
| isLogin / title | false / 结算重算审计 |

### 2.2 数据到达态

三视口一致：skeletonCount=0；所有遮罩计数=0；kpiValues=[42, 7, 16.7%, enforce]；tableRows=4；resultCount="窗口 24h · 共 42 条"；hOverflow=false；isLogin=false。

### 2.3 stat-row 局部刷新

点击「刷新总账」后（mid 态）：statMaskCount=1 且 statMaskVisible=1，skeletonCount=0，gridMaskCount=0，KPI 仍为 [42,7,16.7%,enforce]、tableRows=4 —— 坐实只有 stat-row 局部 `v-loading` 转圈，首屏骨架不重现；after 态 statMask=0。

### 2.4 范式对照

BattlesView 三视口加载态：skeletonCount=1 / role=status / aria-busy=true / skBars=35 / skRows=7 / maskCount=0 / hOverflow=false / title=战斗记录与验真。与 RecomputesView 加载态形状一致，符合共享 `PageSkeleton` 范式。

## 3. 截图（sha256 实测）

| 文件 | sha256 |
| --- | --- |
| r93-390-load.png | c13b3bf75954828f750508c8c5768310d7ae4ea1227be4b761d4a0ce70f9a608 |
| r93-390-arrive.png | 8dccef2049455dc15c291e871c800ce5dfaf67896488606bf6667d5238e4ec63 |
| r93-390-pattern.png | be1ac823fefbc6d293635e9ed4434e6c8769f72d7df45d16b1c7a175b6904b32 |
| r93-768-load.png | 3ec35ce019106d9cef8245e756a3a9acb85855428c0f1ce4cad69f8fe0a5d5ae |
| r93-768-arrive.png | c57792de33699fe3cbde412cd73faae70bb114997add8e447a9e98281efc556a |
| r93-768-pattern.png | ea7f9cd01613cce7417233c002560d7fb2d0c34ff3309239e9c779e16031a922 |
| r93-1280-load.png | b3b608d3c5592d18900261d59b2ee65932143083e77468c8564e2671ce63d9d7 |
| r93-1280-arrive.png | 919d4a7f0f96f2f6118acccaac3f21d4fb451d1494f280625e408117da7104de |
| r93-1280-pattern.png | 7fa260fdf9c70fab7b0776458c0f2153bad8d3aa751b23d4d6ee9a2a342eeea6 |
| trial-768-load.png | 9456c6253ba7717f80da3813b38e295bc9d65ffd40a4bd72957ce90d6f1baaf2 |
| trial-768-arrive.png | c57792de33699fe3cbde412cd73faae70bb114997add8e447a9e98281efc556a |

共 11 张正式+trial 图，9 张正式图 sha 全互异（UNIQUE=9/9）。trial-768-arrive 与 r93-768-arrive 同 sha（同页同到达态，属重复证据）；trial-768-load 与 r93-768-load 不同 sha（截于不同骨架相位）。

亲读 2 张（≥2 达标）：`r93-768-load.png`（768×1024，48006B）为「结算重算审计」页加载态，可见标题、4 张 KPI 骨架卡与 6 行表格骨架、左侧管理导航，非登录页；`r93-1280-arrive.png`（1280×800，99905B）为到达态，可见 KPI 真值 42/7/16.7%/enforce、AI 运营洞察卡、stat-row 统计条与结果计数，非登录页。其余 9 张仅列 sha，未逐张亲读。

## 4. 不变量源码级与静态回归

证据：`data/static-out.txt`（RC0）。

- 令牌三锚点 `laiyipao/admin/src/styles/tokens.css:12/13/14` 逐字在：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`。
- 竞态守卫：`statsLoadSeq`（:174/:176/:180/:183/:189）与 `loadSeq`（:244/:246/:256/:260/:263）双守卫在；`Promise.all([load(), loadStats(), loadMode()])`（:311）在。
- KPI 四格（:459）/ `result-count` 带 `aria-live`（:487）/ `switchMode`（:220/:400）/ `emptyText='没有匹配的重算记录'`（:41）/ `el-pagination` + `cappedTotal`（:469-473）/ `AiInsightCard`（:452）全在。
- 骨架遮罩：`PageSkeleton` 由 `RecomputesView.vue:18` 导入、`:360` 根节点后 `v-if="loading"`；`main.css:535-537` 的 `.page > .page-skeleton ~ * {display:none!important}` 保证加载期真内容不可见；`el-table.data-grid` 已移除 `v-loading="loading"`，`stat-row` 的 `v-loading="statsLoading"` 保留。
- 业务源码零 `--r93-`（NO_MATCH）。写域仅 `.gitignore`（队长预置）与 `laiyipao/admin/src/views/RecomputesView.vue`（实施）。

## 5. 门禁

| 项 | 结果 |
| --- | --- |
| G1 R93 契约（R93RecomputeSkeleton.contract.test.ts） | 6/6 passed，RC0 |
| G2 骨架范式契约（PageSkeleton.contract.test.ts） | 9/9 passed，RC0 |
| G3 admin 全量 vitest | 首轮 1 failed / 908 passed（见下）；全量重跑 **82 files passed (82) / 909 tests passed (909)**，RC0 |
| G4 type-check（vue-tsc） | RC0 |
| G5 build:only | RC0 |
| G6 git diff --check | RC0（仅 .gitignore LF→CRLF 提示） |

**G3 首轮失败如实披露**：唯一失败为 `tests/WikiEnemyMechanics.test.ts > R33 零真值审计脚本安全守卫（t263：扫描器不得排除自身） > 审计脚本自身纳入扫描且实跑 BAD_COUNT=0`，错误逐字 `Error: Test timed out in 20000ms.`（该文件 :182，`execFileSync("bash",[resolve(REPO,SCAN)])` 实跑 22914ms > 20s 默认超时）。该测试与本轮写域（骨架屏）无关，属工作树中大量 untracked 审计产物拖慢零真值扫描导致的超时波动。定向重跑该单测 14/14 passed（目标用例 3900ms），全量重跑 909/909 passed，判定为既存 flaky 超时，非本轮回归。

队长已裁定豁免 `type-check:test`（`pnpm exec tsc -p tsconfig.vitest.json`）：既存写域外 .ts 类型债（TS2739/TS2345/TS6133/TS2322）与 R93 实施无关，本报告不将其列为门禁。

## 6. 诚实边界

- 后端 8080 未启动，所有数据由 mock fetch 合成（42/7/16.7%/enforce、4 行列表），坐实的是骨架屏与数据切换的前端行为，不是真实后端返回值。
- 令牌为占位串且仅在 init-script 注入，auth 四路径（tracked data/、admin-auth-init.js 等）实测 ABSENT；零真值入库。
- QA 库只读：users 表中 id=477 存在、id=104 无记录（历史轮次写作「104|477」并列，此处以只读实测为准）；本轮未写库。
- 截图亲读 2 张，其余仅列 sha；trial 图为额外证据，不作独立验收项。
- hOverflow 口径：`scrollWidth <= innerWidth`（逐视口 390/768/1280 相等），无横向溢出。

判定：**PASS**。