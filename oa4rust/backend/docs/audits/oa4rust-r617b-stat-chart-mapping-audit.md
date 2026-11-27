# R617b QueryManagerDeep 统计分析值域错配甄别（t231）

- 任务: t231 / 轮次: R617b / 类型: verification（零源码改）
- 甄别对象: QueryManagerDeep.vue 统计分析图表映射
- 派发父提交: 3b4d35d4e（当前 HEAD 4c8bf54d1f 的祖先，中间仅 R617a 审计提交，写域不冲突）
- 裁决: P2（功能可见错误；非安全、无数据破坏）
- runnerSha: ac5f08ae14b67e2056708d3fef9df99d589fc26aa9727f99ae74be3d689a3fac
- 说明: runnerSha = sha256(JSON.stringify(meta)) 紧凑 JSON；文件 sha256 由 sha256sum 独立计算，本文档不含自身 sha。

## 一、甄别目标
后端 stat/do 返回嵌套 envelope，前端将其当作扁平「标签:数值」映射，据实判断是否值域错配及危害等级。

## 二、源码锚点（相对路径:行号，sha256）
- backend/crates/query_assemble_designer/src/u2_closures.rs:2493-2580 stat_do；:2540-2543 聚合 SQL（参数绑定，无注入面）；:2549-2567 行对象 {dimension,rowCount,metricTotal}；:2569-2579 返回 ActionResult.data = {id,dimension,metric,data:[...]}。
  sha256=347ffd64362239cb1a2d9697b80e95fd914434b3f7ce7679963d0bf5c8b7bc33
- frontend/apps/desktop/src/views/QueryManagerDeep.vue:889 statResult.value = r?.data ?? null；:488-490 statChartData = Object.entries(statResult.value).map(([name,value])=>({name,value}))；:185-192 ECharts 消费 statChartData。
  sha256=f356b2c85a187b4be7b6c2f9fd7caacfa493a1f1991c77d85639cba3b8aa0b1e
- frontend/packages/sdk/src/api.ts:82-86 parseJsonEnvelope 返回整个 JSON body，故 r?.data 即 ActionResult.data（嵌套 envelope），非行数组。
- 兄弟正确对照 frontend/apps/desktop/src/views/QueryStatementDesigner.vue:1300 直接 (r?.data ?? []).map 映射行集；:1438 chartData 按维度聚合行集。
  sha256=8c00655cac1b532fe182d16ae251c3f1d7421e13fc1722763079fcf7a97be7f7
- 路由 frontend/apps/desktop/src/main.ts:237-238 path query-manager-deep 动态 import 可达。

## 三、行为证据（vitest 真执行）
- 探针 tests/r617b_probe.test.ts（@vitest-environment jsdom，4 用例），执行: node node_modules/vitest/vitest.mjs run tests/r617b_probe.test.ts --reporter=verbose。
- 结果: EXIT 0，1 文件通过 / 4 用例全过（vitest v5.0.5）。
- 输出文本 sha256=d9e36dc5e00d782a9d650a07f695a9563d84c4f555a9f9b7afd9c62cf205900c（日志本体占位 <PROBE_LOG_PATH>，未入库）。
- 坐实事实:
  1) 后端 envelope Object.keys = [id,dimension,metric,data]；真实行在 data[]（华北/华南，rowCount/metricTotal）。
  2) 前端 Object.entries 映射产出 4 行，name = [id,dimension,metric,data]，长度 4；真实统计行华北/华南均未进入图表数据。
  3) value 类型错配: id/dimension/metric 为字符串，data 为数组，ECharts 期望数值 name/value 行。
  4) 兄弟正确写法对同一 envelope 产出 name=华北/value=3450 与 name=华南/value=2200 两行，行数等于 data.length。
- 零源码改: git diff --stat HEAD -- 上述四个源码锚点为空；探针用后已删除。

## 四、三分判级
- P2（成立）: 功能可见错误——选统计分析且有数据时，图表显示 4 条元数据行而非真实统计行，用户得到错误图表；影响面限于该分析视图，需主动触发。无数据破坏、无越权、不写库、不崩溃、无敏感数据泄漏。
- P0/P1（排除）: 非远程代码执行、非权限绕过、非数据泄露或篡改；后端聚合正确，仅前端消费层值域错配，危害不达高等级。
- L1/0（排除）: 缺陷真实可复现且功能可见错误，非低优先级文案/样式问题，不可归类为无关或两极端点。

## 五、据实披露
- 测试锁定: frontend/apps/desktop/src/views/QueryManagerDeep.stat-chart.test.ts:28 把错映射固化为期望（sha256=466fc66abc526aa2593ab3a5838b21496b0cca058d2bfa6d8e97f13fac368716）；修缺陷时须同步更新该测试，否则 GREEN 会掩盖回归。
- 未跑浏览器 GUI: 本甄别基于 vitest + jsdom 行为探针与源码逐行复核，未在真实浏览器渲染 ECharts 验证视觉呈现；结论限定于数据流与映射逻辑。
- 探针为一次性甄别工具，执行后删除，不入库；本文档为唯一交付物。
- 所有哈希为内容指纹，不含任何密钥、口令、真实域名/IP、个人路径等真值。

## 六、复现口径
- 前置: oa4rust 工作区已安装 vitest（oa4rust/node_modules/vitest）。
- 复现: 在 oa4rust 目录执行 node node_modules/vitest/vitest.mjs run；jsdom 探针按第二节锚点复刻映射。
- 期望: 后端 envelope 的 data[] 行应作为图表行集（name=dimension, value=metricTotal），而非把顶层元数据键当行。
