# R589 UsersView 最近战报结果列英文直出跨页一致性甄别（验证型，判 1 项 P2）

## 结论
判 1 项可达真缺陷 P2。真浏览器坐实：同一 result=win/lose 战斗，后台用户管理 UsersView「最近战报」结果列经 `{{ row.result }}` 直出英文 win/lose；而同为中文受众的战报页 BattlesView 列表显示 `胜/负`、详情显示 `通关/失败`，玩家端 me 页显示 `胜利/失败`。同一字段、同源数据、中文表头，三处口径不一致，运营与玩家均可见。零源码改。

## 立项性质与争议披露（Fail Loud）
验证型甄别，跨页一致性方向。据实披露：R587b/R588 针对 order.status 字段（不同字段），R586 次候选 B 提及 BattlesView 文案漂移但未专项甄别；台账与逐行 grep 确认 UsersView 本实例从未专项甄别，无重复。严禁因「同族维度已覆盖」预判 P2 或判 0，全部结论以真浏览器实测为准。次候选 BattlesView 列表胜/负 vs 详情通关/失败属同字段内口径差异，已作为对照据实记录，不硬凑第二项缺陷。

## 范围与入口
目标锚点：laiyipao/admin/src/views/UsersView.vue:678 `最近战报` 区块、:679 el-table `:data=detail.recent_battles`、:682 el-table-column `prop="result" label="结果"` 无 template 无映射英文直出（全文件 prop="result" 仅此一处）。入口：路由 `/admin/users?user_id=<N>`（router/index.ts:22 子路由 users），:457-467 onMounted 带 query user_id 经 openDetail 自动开详情，零深层造数。对照：BattlesView.vue:394-395 列表 `胜/负`、:473-474 详情 `通关/失败`；miniapp me.vue:381 `胜利/失败`。同源链：service/adminuserdetail.go:243 AdminListBattles、:381 out.RecentBattles=battles；service/game.go:663-667 resultStr 只产 win/lose。服务地址以 `<TEST_HOST>:<PORT>` 占位，真值不入库。

## 方法（真浏览器 Playwright + stub）
自起 admin Vite（curl 验当前树 HTTP 200），viewport 1440x900 DPR1；page.addInitScript 预置测试令牌绕过路由守卫，stub 仅拦 `**/api/v1/**` 造一名有 win+lose 同源战报的用户：`/admin/users`、`/admin/users/<id>`（含 recent_battles）、`/admin/battles`、`/admin/battles/<id>`、`/admin/reactions`。主探针直达 users?user_id 开详情，补充探针用独立 context 专测 /admin/battles 列表+详情（避开 keep-alive 缓存串页）。runnerSha=sha256(JSON.stringify(meta))，文件字节 sha256 另算，两者分列。证据落 gitignored 的 oa4rust/test-results/r589/（据实披露）。

## 决定性证据（同一字段三页面实测对照）
| 页面 | 锚点 | result=win 显示 | result=lose 显示 | 语种 |
| --- | --- | --- | --- | --- |
| 数据源 | game.go:663-667 | win | lose | 英文枚举原值 |
| 用户管理最近战报 | UsersView.vue:682 | win | lose | 英文直出（缺陷） |
| 战报列表 | BattlesView.vue:394-395 | 胜 | 负 | 中文 |
| 战报详情 | BattlesView.vue:473-474 | 通关 · 3 星 | 失败 · 0 星 | 中文 |
| 玩家端最近战报 | me.vue:381 | 胜利 | 失败 | 中文 |
主报告 cases=3 fatals=[] pe=[]：usersview 组 resultCell texts=[win, lose]、sectionTitle 含「最近战报」、scoreCell=[12345,6789]，截图名实相符；positive 组中文表头 [ID/昵称/状态/最高关/战力/金币/钻石/验真/最后登录/操作]。补充报告 cases=2 fatals=[] pe=[]：battlesList 组 rows 显示「胜」「负」、标题「战斗记录与验真」；battlesDetail 组 detailResults=[["通关 · 3 星"],["失败 · 0 星"]]。3 图 1440x900 逐张目视名实相符。

## 三分裁决
可达 P2（成立）：同一 result 字段、同源函数产出的 win/lose，在 UsersView 直出英文，而 BattlesView 与玩家端均译为中文，同为中文界面受众可见的口径不一致。不可达 L1（排除）：stub 注入后三页面均成功渲染，users?user_id 直达零造数壁垒。正常 0（排除）：逐条去伪抗辩均不成立（见下节），据实判 P2 不硬凑也不回避。

## 去伪（Fail Loud）
①整页英文设计抗辩：排除——UsersView 同页中文表头（ID/昵称/状态/最高关/战力/金币/钻石/验真/最后登录/操作）+ 抽屉内中文标签（构筑/最近战报/验真记录/分数/击杀/漏怪/时间），仅结果列英文直出属局部疏漏。②语境简写抗辩：排除——win/lose 是未翻译枚举原值，非「胜/负」的中文简写，页面其余结果语义位均有中文译法。③不同字段抗辩：排除——同源函数同字段 result，UsersView 与 BattlesView recent_battles/battles 共用 AdminListBattles/resultStr 产出。④i18n 按 Audience 分流抗辩：排除——同 admin 子项目内 BattlesView 与 UsersView 受众相同却口径不同，非合理分流。⑤计数类抗辩：排除——DefensesView wins/loses 是防线战绩计数，与本次 result 枚举呈现无关，不混入。admin 全项目 grep 无 result→中文映射函数（仅 BattlesView 模板内三元表达式），据实披露。

## 正面对照与交互（非死代码）
正面对照：BattlesView 同页同字段 result 经模板映射真实显示胜/负与通关/失败，证明 AX/渲染通道正常，UsersView 英文直出是缺映射而非数据缺失。交互：users?user_id=<N> 直达自动开详情（onMounted openDetail）真实渲染两条战报；点击 BattlesView「详情」按钮真实打开抽屉并按 result 切换通关/失败文案，非死代码。0 pageerror。

## 最小修复方向
在 UsersView.vue:682 结果列加 template 映射，对齐 me.vue 权威口径 win→胜/lose→负（或与管理后台详情统一为通关/失败，需产品定夺文风），value 保留英文枚举不改后端契约。业务零改、仅 admin 呈现层。本轮不改源码，修复另立契约。

## Fail Loud 汇总
主探针首版因长正则写入转义出错（SyntaxError: Unexpected token if）未采信，改无反斜杠字符串判断重写；次版 require 路径错误（MODULE_NOT_FOUND）未采信，改 `require("@playwright/test")` 从根解析。主探针 battles 组因 keep-alive 缓存串页致截图与 usersview 同 sha（混合 DOM），该组数据未采信，另起独立 context 补充探针专测 /admin/battles 坐实列表胜/负与详情通关/失败。全部 fatal 与串页现象均 Fail Loud 披露。

## 证据 sha 表
runnerSha = sha256(JSON.stringify(meta)) 为紧凑 JSON sha；文件 sha256sum 为字节哈希，两者分列不可混用。
- runnerSha：主报告 `9eef59145549db054f509145662de04af0ab0405a962b937933123615f51e69d`；补充报告 `df1607acda4e84238ec27ba5698f6f6aaa5c57a55237a7206ad43dd667e55423`。
- 报告文件：r589-report.json `756c160cc41e542cf6431b1d3ca5f9c6268f11963f03f303463cbb080a8aedfb`；r589-battles-report.json `456c3e2b8addbdeb7eb4e640a45d4c84fd98c8374abde6ad1a8650cdd065d05d`。
- 截图（1440x900，逐张目视）：r589-usersview.png `adadb8b2015a43b7a8259fd02120401fcf1a8639b71970476ee5ece677f4cdb8` / r589-battles-list2.png `a1200e6ab45943c68cd6aa3c81e68deb8f58b01c27c50849f87fcd526d94f97b` / r589-battles-detail2.png `82a1cdfd6097620cf4d05c125cd6cbada8a7ab0d3ee4b9b6e5e736428e82d53d`。串页未采信：r589-battles-list.png 同 usersview sha、r589-battles-detail.png `3fb472ab01797171b30f37181a23a8be698ffc764cad90c9ab1d17bd04766986`。
- 探针脚本：r589-e2e.cjs `4a0dfcbd219bb1ab66e9964407a5d338ed9d25d3b602f9c79587ff3c954d4aa1`；r589-battles.cjs `c22b3c0459ead3f9e97f6d179bfa8c8b5e261abaa2a6bb5bcfae64f9c1755882`。
- 源码：UsersView.vue `d95643ef7f084c2cc3ce5f582fc62701f449c249a869057644575fe4076ad67e` / BattlesView.vue `19b1a4226d889bbe68e0de71e91ec2b01546a9e7a29a252c602aab39de39c16d` / me.vue `303441acbb251a0bb8daa82e294a202a7f2960e7a1fa02edfbc163ab26f980c8` / adminuserdetail.go `50ac1c0db161ce9c5dddb0d0e7c6734f3c7b543843829485692eacf5b59711e7` / game.go `530c04342a1cd375fb0efb3c1b5504063725b81dbd5d8c8eaf8244a9ebedc424`。
- 脚本与报告留 gitignored 证据目录 oa4rust/test-results/r589/，据实披露不入仓。