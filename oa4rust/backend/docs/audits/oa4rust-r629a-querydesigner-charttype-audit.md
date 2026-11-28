# R629a QueryDesigner chartType 四选一恒柱状图零消费甄别审计（只读）

## 1. 结论
判级 P2（据实，不夸大 P1）：QueryDesigner 图表面板提供 bar/line/pie/area 四选一，但选择任何一项均不改变渲染结果——永远输出柱状图数据，属"控件存在、语义零消费"的可达 UI 缺陷。非 P0/P1：不影响数据落库、接口安全或权限，仅本地呈现层语义失真；不适用 P3：选择控件明确可见且用户预期有差异，不是纯文案瑕疵。

## 2. 锚点与根因
- `oa4rust/frontend/apps/desktop/src/views/QueryDesigner.vue`：模板 `:190` `<select v-model="chartType">` 四选项 bar/line/pie/area；`:193` 按钮 `@click="renderChart()"`；`:195-200` 结果区硬编码 `class="chart-bar"` 与 `v-for="(d,di) in chartData"`，按 `d.h` 柱高渲染，无任何按类型分支。
- 脚本 `:720` `const chartType = ref("bar")` 仅声明；`:727-746` `renderChart()` 读 `resultData/chartX/chartY` 聚合写 `chartData/chartStats/chartRendered`，全程未读取 `chartType.value`。全文件 `chartType` 仅 `:190`、`:720` 两处命中，无第三处读取点。
- 根因：v-model 绑定了 ref，但渲染函数与模板都未消费该状态，四选一退化为装饰性控件。
- 可达性：`oa4rust/frontend/apps/desktop/src/main.ts:230-232` 注册 `query-designer` 路由并加载该组件；`:520` 重定向至 `/app/admin/query-designer`，入口真实可达。
- 兄弟对照：`oa4rust/frontend/apps/desktop/src/components/EChartsView.vue:53` 定义 `chartType` prop，`:78` 取 `type`，`:80/97/113` 分别按 bar/pie/line 真分支渲染，`:129` 对 line 应用 `areaStyle`——同仓组件正确消费类型，凸显目标组件缺失分支。
- 台账 `docs/ops/current-state.md` 对 chartType 四选一零消费无专项命中，`:5668` 据实立项 R629 方向。

## 3. vitest + jsdom 真执行证据
- 环境：`oa4rust` 下 `./node_modules/.bin/vitest run r629a_probe.test.ts --reporter=verbose`（vitest 5.0.5 + jsdom 30.0.1）。
- 结果：Test Files 1 passed (1)；Tests 5 passed (5)；EXIT=0（真跑，非自述）。
- 用例1：源码 `chartType` 命中数恰为 2，且仅 v-model 绑定与 ref 声明——坐实无读取点。
- 用例2：`renderChart` 函数体不含 `chartType`——坐实渲染函数零消费。
- 用例3：图表模板段硬编码 `class="chart-bar"`，不含 `chartType|chart-line|chart-pie|v-else-if`——坐实无类型分支。
- 用例4：源自源码 `:727-746` 逐行转写的纯 JS 等价实现，bar/line/pie/area 四值产出逐字节相同（含 label/value/h 聚合与 stats）。
- 用例5：jsdom 按四值生成 `.chart-bar` DOM，`innerHTML` 完全相同，柱数一致——坐实 DOM 层零差异。
- Fail Loud：动态两用例曾因探针 TS 类型剥离正则不完整（`new Function` 报 Missing initializer）与双重 JSON 断言写法失败，均为探针自身问题，修正后全绿；被测源码零改动。探针为临时.oneshot，验证后删除，不入库。

## 4. 三分据实
- 可达真缺陷 P2：控件可见、路由可达、四选一恒柱状图，用户选择无效果。
- 非数据破坏：仅本地图表呈现层，`resultData` 与后端查询不受影响，无落库/接口/权限副作用。
- 不适用 P3：语义明确（四类型选择），非纯文案/色值瑕疵，故不降为 P3。

## 5. 修复方向（未实施，零源码改）
在 `renderChart()` 读取 `chartType.value` 并按 `bar/line/pie/area` 分支渲染（可复用 `EChartsView.vue` 的 `chartTypeMap`/`describeChart` 口径），模板去掉硬编码 `chart-bar` 改为按类型渲染；`des` 包 `typecheck` 为 `tsc --noEmit` 不查 `.vue`，建议补 `vue-tsc` 或针对性契约防回归。

## 6. SHA 与范围声明（分列）
- 真父 HEAD：`26a2c002b2ac824ec773658d6d5c6321f4141e7a`；审计时工作树 HEAD 已被队友前移至 `a94ffa51a073b83fde9d9cc55dfb9d1f720d3e2a`，`git merge-base --is-ancestor` 确认真父仍为祖先（ANC=0），据实披露、不回滚。
- runnerSha（对本次 meta JSON 的 sha256，非文件哈希）：`420486252a15cf4e1a434ef9d8b23631e591574780f0df5dfc107e15dc2843d5`。
- 审计对象文件 sha256（QueryDesigner.vue，只读未改）：`6924c3fad0ea1a0ccc0bb603e195fd64d601d928f6429e26bd9d378fd0240f29`（40520 bytes / 858 行）。
- 本次零源码改、零测试入库、零配置改；唯一入库物为本审计文档。文档 0 真值：无域名/IP/口令/私钥/个人路径/真实端口。