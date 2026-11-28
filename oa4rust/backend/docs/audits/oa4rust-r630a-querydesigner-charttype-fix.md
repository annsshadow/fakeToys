# R630a QueryDesigner chartType 四形态真修复审计

## 结论
判级 P2（功能缺陷已修复，非安全漏洞）。QueryDesigner 图表弹窗的 chartType 下拉提供 bar/line/pie/area 四形态，但 renderChart 从不消费 chartType.value，模板硬编码 chart-bars 柱状条独占渲染，用户切换形态无任何效果。本次接入兄弟组件 EChartsView 使四形态真实生效。

## 真父基线
- 真父 HEAD: 2db38d58d8cc2bbba66bd1ed1a602bc9a3015c46
- 缺陷源: oa4rust/frontend/apps/desktop/src/views/QueryDesigner.vue（真父 sha256 6924c3fad0ea1a0ccc0bb603e195fd64d601d928f6429e26bd9d378fd0240f29，858 行）
- 缺陷锚点: :190 四形态 select v-model=chartType；:720 chartType ref 声明；:727-746 renderChart 零消费 chartType.value；:195-202 模板硬编码 chart-bars。
- 路由可达: main.ts 查询设计器路由登记（历史甄别已坐实）。

## 修复内容（写域精确三文件，零外溢）
1. QueryDesigner.vue：仅改图表弹窗模板与图表脚本段，import EChartsView 兄弟组件；模板 chart-bars 硬编码替换为 EChartsView 绑定（:data=chartEchartsData / :columns=chartEchartsColumns / :chart-type=chartTypeResolved / dimension=name / metric=value）；新增 chartEchartsColumns、chartEchartsData、chartTypeResolved 三个 computed 真正消费 chartType.value。
2. QueryDesigner.charttype.test.ts（新增源码契约）：5 用例，真父 RED 4 失败、修复后 GREEN 5 通过。
3. 本文档。
- EChartsView.vue / QueryManagerDeep.vue / main.ts / 路由 / Cargo / migrations / 后端：零改。

## area 形态映射（据实披露）
EChartsView 的 chart-type prop 只接受 bar/pie/line/table，无独立 area 分支。本次将 area 映射为 line：EChartsView line 分支已含 areaStyle（opacity 0.1）面积填充，视觉语义等价面积图。映射在 chartTypeResolved computed 中以 chartType.value === 'area' ? 'line' : chartType.value 实现，源码可读可审，非隐藏行为。

## RED → GREEN 证据
- RED（真父，修复前）：vitest 定点 5 用例中 4 失败（缺 EChartsView import / 无 chartType.value 读取 / 无 area→line 映射 / 旧 chart-bars 仍在），EXIT=1。
- GREEN（修复后）：同一契约 5 用例全通过，EXIT=0。
- 契约测试用 resolve(import.meta.dirname, 'QueryDesigner.vue') 读源码字符串断言，规避 cwd 漂移；测的是「接线是否存在」，非组件运行时渲染。

## 三验证（V1/V2/V3）
- V1 定点契约：cd oa4rust && node node_modules/vitest/vitest.mjs run frontend/apps/desktop/src/views/QueryDesigner.charttype.test.ts → 5 passed，EXIT=0。
- V2 包级 typecheck：cd oa4rust && pnpm --filter @oa4rust/desktop typecheck → EXIT=0。据实披露：desktop 包 typecheck 为 tsc --noEmit，不解析 .vue 单文件组件模板，Vue 模板内类型/prop 正确性不由本命令覆盖。
- V3 格式：cd oa4rust && node_modules/.bin/biome check <Vue> <测试> → EXIT=0（首轮因导入排序与字符串拼接报 3 错，已手动修正为排序 import + 模板字面量 + EChartsView 加 noUnusedImports ignore，重跑全绿）。

## 未覆盖 / 据实披露
- 未跑浏览器 GUI 人工点验四形态渲染效果；结论基于源码契约 + typecheck + biome 三层。
- 契约测试断言的是源码接线模式（import/prop/chartType.value 消费/映射），非 ECharts 实际绘图输出。
- EChartsView 内部 bar/pie/line 分支由兄弟组件 QueryManagerDeep 既有使用覆盖，本次未改其内部。
- 本案无网络/凭据交互，测试夹具与本文档均 0 真值；无 TEST_HOST/口令/私钥需占位。

## SHA 分列（runnerSha 不含自身，文件 sha256 由 git 内容确定）
- runnerSha(紧凑 JSON meta): 87740783f4871df2d8c4370c40015da52e9950073c76c4d8491c8386a9652fac
- sourceSha(真父 QueryDesigner.vue): 6924c3fad0ea1a0ccc0bb603e195fd64d601d928f6429e26bd9d378fd0240f29
- redLog sha256: e5d00f45446a86861163de1a00110abcc9b70678a35dcbb1754ba65fb21d6aa7
- greenLog sha256: 3fcbe6c6051eda0534fd479df375a592eed272e2a38421be058f31bc653b0cc5
- typecheckLog sha256: 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92
- biomeLog sha256: a7c662c71a99e9d26e6b19d44e907b7a72664486be8ae361df047927de509b73
- 日志路径均为 oa4rust/test-results/r630a-*.log（本地未入库）。

## 复现命令
cd oa4rust
node node_modules/vitest/vitest.mjs run frontend/apps/desktop/src/views/QueryDesigner.charttype.test.ts --reporter=verbose
pnpm --filter @oa4rust/desktop typecheck
node_modules/.bin/biome check frontend/apps/desktop/src/views/QueryDesigner.vue frontend/apps/desktop/src/views/QueryDesigner.charttype.test.ts