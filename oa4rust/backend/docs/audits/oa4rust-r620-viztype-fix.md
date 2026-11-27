# R620b QueryStatementDesigner vizType 三形态死配置修复审计

## 结论
P2 真修复。vizType 下拉框（`bar`/`line`/`pie`）是真死配置：真父只声明 `vizType = ref('bar')` 与三个选项，模板固定渲染原生柱，renderChart 聚合后 vizType 全程零消费；用户切到 line/pie 图形态不变。非安全、不越权、不写库、不崩溃，局部功能错误，故 P2；排除 P0/P1/L1/0。

## 锚点与证据
- `oa4rust/frontend/apps/desktop/src/views/QueryStatementDesigner.vue:591`：三选项 vizType 选择器。
- 同文件 :596-602（真父）固定 `v-for="(d,di) in vizBars"` 原生柱，无 vizType 分支。
- 同文件 :1775 `const vizType = ref('bar')`；:1782-1802 renderChart 只写 vizBars/vizStats，vizType 零消费。
- 兄弟 `oa4rust/frontend/apps/desktop/src/views/QueryManagerDeep.vue:185-192` 与 :442：EChartsView 正确接入范例（`:data/:columns/:chart-type/:dimension=name/:metric=value`）。
- `oa4rust/frontend/apps/desktop/src/components/EChartsView.vue:50-56`：props 支持 `bar|pie|line|table`；:78-130 三形态真实 setOption 分支。
- 路由 `oa4rust/frontend/apps/desktop/src/main.ts` 含 query-statement-designer，页面可达。

## 修复（写域精确 3 文件，零外溢）
1. 导入 `EChartsView from '../components/EChartsView.vue'`（按 biome 排序置入组件导入区）。
2. 新增 `const vizChartColumns = ['name', 'value']` 与 `const vizChartData = computed(() => vizBars.value.map((d) => ({ name: d.label, value: d.value })))`。vizXAxis/vizYAxis/renderChart 聚合逻辑、vizStats/exportVizData CSV 零改。
3. 模板按 vizType 分支：`v-if="vizType === 'bar'"` 保留原原生柱（含无障碍标题/颜色/高度），`v-else` 渲染 `<EChartsView :data="vizChartData" :columns="vizChartColumns" :chart-type="vizType" dimension="name" metric="value" />`，line/pie 真正切换图表形态。

## RED → GREEN（vitest 真执行）
- 契约：`oa4rust/frontend/apps/desktop/src/views/queryStatementDesigner.viztype.test.ts`（断言 vizType 选择器驱动 EChartsView、三形态分支、computed 映射、bar 原生柱保留）。
- 真父源码 sha256=8c00655cac1b532fe182d16ae251c3f1d7421e13fc1722763079fcf7a97be7f7 覆盖下 RED：EXIT=1，1 failed（`<EChartsView` 缺失）；日志 sha256=ed3480fefea66e2df12e72782976d42443a8256afd3a8172df8007e5755bad28（<PROBE_LOG_PATH>）。
- 修复版 sha256=17989aa2eee988ade442076940762f1f5cf5c21a313af6add81e8f9022b02999 下 GREEN：EXIT=0，1 passed；日志 sha256=a55ead14ab4744af3a8a64de157c596fc639d5ea86e71e1f3811c225254b48ee（<PROBE_LOG_PATH>）。

## 三 verify（据实）
- V1 定点 `pnpm exec vitest run src/views/queryStatementDesigner.viztype.test.ts`：EXIT=0，1 passed。
- V2 `pnpm typecheck`：`tsc --noEmit` EXIT=0，但不解析 `.vue`，vizType 死配置属其盲区，据实披露；`pnpm exec vitest run` 全量 2263 tests passed、2 个无关既有文件失败（`tabStateA11y.contract.test.mjs` / `TaskCompleted.R314.contract.test.ts`，后者因路径 `backend/crates/...` 在 desktop cwd 下不存在），均非本次写域，据实披露未静默跳过。
- V3 `pnpm exec biome check .` 基线含大量既有错误；精确 `pnpm exec biome check src/views/QueryStatementDesigner.vue src/views/queryStatementDesigner.viztype.test.ts` 仅剩 1 个既有错误 `lint/complexity/useOptionalChain`（真父备份 1063 行 `if (!item || !item.id) return`，非本任务引入，未越界改写），本次新增代码零格式/lint 错误。

## 审计标识
runnerSha=daad4d74fbaaa09d6a5ca9195ba6540a2256812556d087c957ec2882620fdcd9（sha256(JSON.stringify(meta)) 紧凑 JSON，task=t241/round=R620b/verdict=P2-fixed/parent/head/sourceParentSha/sourceFixedSha/contractSha/redLogSha/greenLogSha/anchor）。
本文档 sha256 由提交前 `sha256sum` 单独落账，文档正文不含自身 sha；日志真值一律以 <PROBE_LOG_PATH> 占位，0 真值。

## 边界与披露
- 兄弟 QueryManagerDeep.vue 及 QueryManagerDeep.stat-chart.test.ts、其它视图、后端 .rs、路由、Cargo/migrations 零改。
- 未跑浏览器 GUI；行为验证基于 vitest 契约与源码锚点，据实披露。
- 当前 GitHub 出口不通；本地 commit 落地后 Fail Loud 报告队长待补推。
