# R641b AppInfoApp 列字段与统计卡值域错配甄别审计（t336）

## 结论
判定 2 项真实可达前端契约错配缺陷（同页同根因：后端 x_cms_appinfo 行键集与前端读取键集不一致）：
- A（P2）：appInfoColumns 的 name/appFlag/appType 三列键在后端行中均不存在，AdminTable 走 `row[col.key] ?? '—'` 回退，三列恒显“—”，应用名称/Flag/类型三项核心列表信息全部失效；操作列（cell-actions 具名插槽）仍可用，故不判 P3，但非权限/写库问题故不升 P1。
- B（P3）：统计卡按 `i.status !== 'disabled'` / `=== 'disabled'` 过滤，后端无 status 列只有 enabled BOOLEAN，“已启用”恒等于总计、“已禁用”恒为 0，属展示误导，无功能阻断。

## 立项性质与范围
- 验证型甄别：零源码改动，唯一交付物为本审计文档；探针与日志置于本地 gitignored 证据目录，临时探针用后即删。
- 基线 HEAD=25eb3d69c7f2c49c393d3d098c13906095b18bc3（工作开始时所处 HEAD；工作期间并发推进属正常）。
- 台账去重：docs/ops/current-state.md:2170（R217 搜索框 a11y）、:4348（R308 统计卡 loading 标签修正）、:4861-4870（R332/R333 默认插槽静默丢弃，修的是 cell-actions 插槽）、:5154-5159（R396/R397 写入口）均不涉及“列键集/统计 status 键值域错配”，本缺陷独立成立。

## 机制锚点（逐行核实）
### A 三列值域错配
- 前端 oa4rust/frontend/apps/desktop/src/views/AppInfoApp.vue:64-69 appInfoColumns 定义键 name/appFlag/appType/actions；模板 :27 传入 AdminTable，仅 :28-34 有 cell-actions 一个具名插槽，name/appFlag/appType 三列无具名插槽。
- 渲染组件 oa4rust/frontend/apps/desktop/src/components/admin/AdminTable.vue:35-37 单元格回退 `{{ row[col.key] ?? '—' }}`；无具名插槽即读该表达式。
- 数据取得 AppInfoApp.vue:102-105 api.get('/api/appinfo/list/all')，(r.data ?? []) as Item[] 直接赋值 items，无 snake→camel 重命名；:105 搜索读 (a.name ?? '') 亦印证期望 name 键。
- SDK oa4rust/frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 仅 await resp.json() as ApiResponse<T>，无字段转换。
- 后端 oa4rust/backend/crates/cms_assemble_control/src/lib.rs:896-900 appinfo_list_all → list_from_table_filtered_legacy(..., "x_cms_appinfo", "deleted_at IS NULL", &[])。
- 同文件 :401 list_from_table_filtered_inner，:441 SELECT * FROM {}{}{}，:459 rows.iter().map(row_to_json) 整行序列化。
- oa4rust/backend/crates/shared/src/response.rs:30-33 row_to_json 以 col.name() 原样为 JSON 键，无别名映射。
- 表结构 oa4rust/backend/migrations/023_create_cms_assemble_tables.sql:5-15 x_cms_appinfo 列 id/app_type/alias/icon/enabled/manager/creator/create_time/deleted_at；064_cms_u2_columns.sql:13-14 加 config。键集为 {id,app_type,alias,icon,enabled,manager,creator,create_time,deleted_at,config}，无 name/appFlag/appType → 三列恒 undefined → 恒“—”。
- 路由 cms_assemble_control/src/routes.rs:415 .route("/api/appinfo/list/all", get(appinfo_list_all))；前端 main.ts:152-154 path appinfo 普通登录可达，无需 admin。
### B 统计卡 status 值域错配
- AppInfoApp.vue:90-95 stats computed：总计=items.length；已启用=filter(i.status !== 'disabled')；已禁用=filter(i.status === 'disabled')。
- 后端行无 status 键（见上键集），只有 enabled BOOLEAN → i.status 恒 undefined：undefined !== 'disabled' 恒 true → 已启用恒=总计；undefined === 'disabled' 恒 false → 已禁用恒=0。
- docs/ops/current-state.md:4348 证明统计卡是 R308 已完成修标签的业务承诺，非装饰。

## 方法（真测试）
- 探针 oa4rust/test-results/t336/AppInfoApp.columnsStats.probe.test.ts（gitignored，已不留在 views）：vitest + ts.transpileModule + new Function 真实执行——从 vue 源码切片 appInfoColumns 定义与 stats computed 并执行；从 AdminTable.vue:37 提取 `{{ row[col.key] ?? '—' }}` 表达式 new Function 逐列求值。
- 6 用例：①真实后端形状行 {app_type,alias,enabled} 三列经回退恒“—”；②对照带 name/appFlag/appType 行三列正确显示“内容管理/CMS/cms”；③真实行统计卡已启用恒=总计、已禁用恒=0；④对照带 status 行统计卡正确得 2/1；⑤源码直取 list/all 且 doSearch 段无 app_type/status 映射（证无重命名）；⑥反证把列键改为 alias/app_type/enabled 后数据可见（证数据存在仅键错配）。
- 真测试 vitest run 6/6 passed EXIT 0（先 RED 3 failed 因探针匹配行取错，修正后 GREEN，非业务结论变化）。

## 决定性实测
| 观测 | 结果 |
| --- | --- |
| 三列键 name/appFlag/appType 在后端行存在? | 否 |
| 真实行三列渲染值 | — / — / — |
| 对照行三列渲染值 | 内容管理 / CMS / cms |
| 真实行 已启用/已禁用/总计 | 3 / 0 / 3 |
| 对照行 已启用/已禁用 | 2 / 1 |
| 数据本身是否可得（改键后） | alias=内容管理, app_type=cms, enabled=true |
| /api/appinfo/list/all 路由 | routes.rs:415 已注册，普通登录可达 |

## Fail Loud / 边界
- 判级说明：A 三列核心信息失效但操作列可用、未越权/未写库/未泄露凭据 → P2 不升 P1、有操作列故不降 P3；B 统计误导无功能阻断 → P3。
- 未跑物理 GUI 浏览器（本轮为静态源码 + 真表达式执行甄别）；tsc --noEmit 不解析 .vue 模板，属盲区据实披露。
- 生效前提：普通登录用户访问 /app/admin/appinfo 即可触发，无需 admin。
- 无凭据/真域名/个人环境真值入文档；sha 分列见下。

## SHA256 分列（口径）
- docSha=4278ffb2b2df10e33c0d7c7df8ecdfc66e50405496c1ed1944baad141d4605e5（本审计文档正文：各行按 LF 连接、去掉 docSha 行、去除尾部换行后的 sha256，可复算）
- runnerSha=b1af59a56710edd819825454b509d1a21f9f9e8eeed0522e9842252e73b19dff（探针脚本全文 sha256）
- 探针日志 oa4rust/test-results/t336/r641b-probe.log sha256=18b7e40362e50d3a70486625a43b2ec8d04d68961392daf8831146e2e3daced6
- 后端/前端源码与迁移为已核实锚点，本轮零改动。

## 修复方向（建议，不实施）
- A：把 appInfoColumns 三键对齐后端真实键（alias→应用名称、app_type→类型）；appFlag 后端无对应列，需产品确认语义（或对应 enabled、或 config 内字段）后决定映射或删列；或后端在该端点增 SELECT 别名 name/appFlag/appType。
- B：统计卡改按 enabled BOOLEAN 过滤（i.enabled !== false / === false），与后端列对齐。
