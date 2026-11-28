# R636a 回收站类型筛选值域错配甄别（验证型零源码改）

## 结论

判 P3（低）：回收站类型筛选确属语义失效（已删除恒空、待恢复恒全、统计卡恒 0、每行来源标签恒错），但默认「全部类型」选项完整可用，查看/恢复/永久删除/清空全部核心操作不阻断；后端归属正确、无越权无写库无数据泄露；与 R634c SearchApp 值域错配同族同构，据实降为 P3，不硬凑 P2/P1。

## 事实链（真父 9bd06a53a，源码零改）

- 后端 oa4rust/backend/crates/file_assemble_control/src/lib.rs（sha256 见文末）：recycle_list :3990-4056，SQL :3997-3999 按 person = $1 取本人已删除行（归属正确）。映射 :4003-4049 返回键仅 id/name/person/referenceType/extension/length/mimeType/deletedAt，全程无 deleted 布尔键、无 deleteTime 键。
- 前端 oa4rust/frontend/apps/desktop/src/views/RecycleApp.vue（sha256 见文末）：
  - :22-26 下拉三选项 all/deleted/active；:98 typeFilter 默认 all；:138-139 api.get(/api/recycle/list) 后 allItems = r.data 无字段补全。
  - :103-104 itemType(item) 返回 item.deleted ? deleted : active —— 判定依赖后端根本不返回的 deleted 字段。
  - :112 matchType = typeFilter === all || itemType(item) === typeFilter；:108-115 items computed 据此过滤。
  - :15 统计卡 allItems.filter(i=>i.deleted).length；:61 来源列 itemType(item)===deleted ? 已删除项目 : 待恢复项目。
- 根因：写入侧值域（deletedAt 字符串）与过滤/展示侧值域（deleted 布尔）无交集，属单一值域错配，同时污染筛选、统计、标签三处展示。
- 路由可达：main.ts path recycle 动态导入；后端 routes.rs .route(/api/recycle/list, get(recycle_list))，非死代码。

## 真测试坐实（vitest 5.0.5 + jsdom + ts.transpileModule/new Function 真实执行，5/5 通过）

临时探针 R636a.recycleTypeFilter.contract.test.ts（验证型取证后已删除，不入库）：用 ts.transpileModule 擦除 RecycleApp.vue 脚本类型，按大括号配对提取 itemType 并用 new Function 真实执行；match/统计/来源列表达式均以 new Function 真实求值，喂入后端形状样本对象（仅 deletedAt），非静态字符串断言。
1. ① itemType 真实执行：后端样本恒返回 active；显式 deleted=true 才返回 deleted。
2. ② matchType 真实执行：deleted 恒 false、active 恒 true、all 恒 true。
3. ③ items 过滤真实执行：deleted 恒 0 项、active 与 all 均 3 项（全量通过）。
4. ④ 统计卡真实执行：allItems.filter(i=>i.deleted).length 恒 0。
5. ⑤ 来源列真实执行：每行恒返回 待恢复项目。
真跑命令：node_modules/.bin/vitest run frontend/apps/desktop/src/views/R636a.recycleTypeFilter.contract.test.ts --no-coverage，结果 1 passed / 5 tests passed / EXIT 0。

## 三分据实（判级理由）

- R634c SearchApp 值域错配判 P3：分类 tab 失效但全部结果 tab 正常、可绕过。本案同构：默认 all 正常显示全部回收项，核心操作不阻断。
- R596b 状态值域错配判 P2、R599/600 DocumentApp 筛选失效判 P2：本案据实下调，因默认值 all 完全可用、无身份/越权/写库/泄露，仅展示层语义降级，且根因单一。
- 不判 P1：不阻断主流程、无数据安全影响、无数据丢失（恢复/永久删除端点独立于本筛选）。
- 判 P3：纯前端值域映射不一致导致筛选/统计/标签语义错误，确定性可复现、默认值可绕过、影响有限。

## 行为现状与可复现步骤

1. 以任意账号进入回收站，列表返回 N 条本人已删除项（后端仅给 deletedAt）。
2. 统计卡「已删除」显示 0；每行「来源」列全部显示 待恢复项目。
3. 下拉选「已删除」→ 列表恒空（显示「没有符合当前筛选条件的回收项目」）；选「待恢复」→ 显示全部 N 条（与「全部类型」等同）。
4. 恢复、永久删除、批量与清空全部正常，用户仍可完成所有管理操作。

## SHA（分列，均非真值）

- frontendSourceSha = c5f0e49985e12532fdb4d240e3c182ab9b6a5b3554101c25e69ff94f6428c479
- backendSourceSha  = 39824e39ef2b77ef3a0d3dc63a8f31975f45d5e7558342e6f333bff9cb4d3a44
- runnerSha（临时探针全文 SHA-256）= 39275321013cf0ce12acc60798395547e4e2e607c45ea017dabaf350961bc4bf
- docSha = 见任务报告（本文件全文 SHA-256；自引用悖论，不在正文内固化）
- parent = 9bd06a53a422f85feecc80839f1576d1fe3df9d4

## 据实披露

- 零源码改：本轮仅新增本审计文档；临时探针取证后删除，未改任何前端/后端/测试文件。
- 未跑浏览器 GUI 人工点验；结论基于源码逐行复核 + 表达式真实执行探针。
- desktop 包 tsc --noEmit 不解析 .vue（非 vue-tsc），类型层面无法捕获该值域错配，属既有盲区。
- 文档仅含占位符与哈希，无口令、真域名/IP、个人路径等真值；LEAK 扫描 0。
- 建议（不在本甄别范围）：前端 itemType/统计/来源列改用 item.deletedAt 判定，或后端映射补 deleted 布尔字段统一值域。
