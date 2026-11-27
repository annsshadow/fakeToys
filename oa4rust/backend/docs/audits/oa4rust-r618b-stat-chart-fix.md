# R618b QueryManagerDeep 统计分析映射真修复审计

## 结论
- 判定：P2（值域错配，统计行从未进图表）已修复；非 P0，L1-0。
- 真父：221ecbb210832ca973360419d98ebffe7ebc8270；写域严格 4 文件（2 M + 2 A）。

## 锚点与根因
- 后端 query_assemble_designer/src/u2_closures.rs:2549-2579：stat/do 返回包络 data 为 {id,dimension,metric,data:[{dimension,rowCount,metricTotal}]}。
- 前端真父 views/QueryManagerDeep.vue:484 statResult 类型错，:488-490 Object.entries(statResult.value) 把 4 个顶层元数据键映射成图表行，真实统计行从未进图表。
- 模板 :185-197 EChartsView 与 v-for(statResult) 同用错数据；stat-chart.test.ts:28 把错映射文本写死为期望，固化缺陷。

## 修复方案
- 新增 StatRow/StatResponse 接口承载嵌套响应，statResult 改装正确类型。
- 新增 mapStatRows(resp) 直接映射 resp.data[] 为 name/value 对象（name=row.dimension，value=Number(row.rowCount)||0）；statChartData 由 computed 调用它。
- 主数值选 rowCount：后端该列为 i64 计数、metricTotal 为可选字符串，EChartsView :93 亦用 Number(r[metric])，语义一致。
- 表格模式改遍历 statResult.data，展示 dimension / rowCount / metricTotal；stat-result 判据加 data.length 检查。
- statChartColumns 保持 name/value，模板 dimension=name、metric=value，与 EChartsView props 契约吻合（组件未改）。

## 测试与 RED→GREEN
- 真父版本（vue 未改）跑两测试：2 files failed / 7 failed+2 passed，PIPE_EXIT=1，坐实缺陷。
- 修复后定点 vitest：2 files / 9 tests 全绿 EXIT0。
- 新契约用真实后端形状（华北 rowCount 120、华南 rowCount 80）断言映射产出 2 行、name/value 正确、不含顶层元数据行、null/空 data 安全。
- 原 stat-chart.test.ts 错映射断言 :28 已改为新映射断言，不再固化缺陷。

## 验证（三 verify 全 EXIT0）
- V1 定点 vitest 9/9 EXIT0。
- V2 typecheck（tsc --noEmit）EXIT0；据实披露：tsc 不检查 .vue 单文件组件内部类型，vue 侧正确性由 V1 组件源码契约覆盖。
- V3 biome check 三个交付文件 EXIT0（仅 2 info + 1 warning，无 error）。
- 全量 vitest：283 files passed / 2262 tests passed；2 个既有基线 suite 失败（tabStateA11y 无 test suite、TaskCompleted.R314 需 backend cwd 下 lib.rs），与历史记录一致，非本轮引入，据实披露。

## Fail Loud 与边界
- 未跑 GUI 浏览器验证：以 vitest + 组件源码契约替代，据实披露。
- V3 为达 EXIT0 修复了 3 处纯格式问题（原 613/663/743 行，同文件写域内，零语义变化）；真父版 biome 整文件即 EXIT1，属既有基线，已格式化并披露。
- 后端 u2_closures.rs 返回结构不动，EChartsView.vue props 契约吻合不改，其他视图/组件未触碰。

## sha256（文件值与 runnerSha 分列）
- QueryManagerDeep.vue: 9270c8534098fa0283198716ba8da0097ff352a27c2dd8069fe57f91e65ce327
- QueryManagerDeep.stat-chart.test.ts: a48050e5528fc041f965cbd32db622867567288746dda0f3b0f6a12a29ea17be
- QueryManagerDeep.statChartData.contract.test.ts: c46337352c70b5340081ccfdc867a74206b7a4f4da53cda3bc63d60bdcb467d3
- runnerSha=sha256(JSON.stringify(meta)): 78364effd786072ad14844d8166d404a6025a13b0e1fd01018828dbd913073a1
- 日志 sha256: RED=79ae1d95583fed94bad88e1b0c9e40c280b34b8bfae53ac67f57b626b5edef34; V1=e17cc004678890aafdff2369e5bde58738db6c9000ac45d9380c39db828aa595; V2=8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92; V3=87bb7ebccf372133d050f004582c0866d9a08c32f7b49450bcd36e8fc66d1d6e; full=f5d511e7d4ed45203b77349daa90b6c6eacaf95774e2d06739d0a74d00c88f68
- 本审计文档 0 真值（无密钥/口令/域名/IP/个人路径）。
