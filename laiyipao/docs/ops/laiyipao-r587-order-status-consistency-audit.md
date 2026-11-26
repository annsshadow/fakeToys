# R587b 支付订单状态枚举跨页一致性甄别（验证型，判 1 项 P2）

## 结论
判 1 项可达真缺陷 P2（跨页文案口径不一致）。真浏览器坐实：同一订单号 R587-1001（枚举源 status=paid）在后台 EconomyView 状态列直出英文 paid、筛选器 label 全英文 pending/paid/delivered/refunded；而同一 admin 子项目的 DashboardView 看板用中文「待支付」、玩家端 miniapp me 页用权威中文映射显示「已支付 · R587-1001」。同一 StatusCode 三处文案口径不一致，运营与玩家均可见。零源码改，本轮甄别立档。

## 立项性质与争议披露（Fail Loud）
验证型甄别（跨页一致性新方向首轮）。立项依据台账 5583-5584 行（R586 侦察）；台账第 1194 行仅历史负结果提及「订单状态映射…有测试/契约佐证，无 bug」——那指数据映射的测试契约正确性，本轮是 UI 文案呈现一致性，维度不同，不构成重复也不矛盾。严格真浏览器实测，严禁静态预判 P2。

## 范围与入口
目标锚点：laiyipao/admin/src/views/EconomyView.vue:451-455 筛选器 el-option label 全英文（pending/paid/delivered/refunded）、:505-509 状态列 el-tag 内容 {{ row.status }} 直出英文；laiyipao/admin/src/views/DashboardView.vue:292-294 订单 KPI 卡中文「待支付 {{ data.economy.orders_pending }}」；laiyipao/miniapp/src/pages/me/me.vue:958-964 权威中文映射 orderStatusText（pending→待支付/paid→已支付/delivered→已发货/refunded→已退款）、:446 渲染 {{ orderStatusText(o.status) }} · {{ o.order_no }}；laiyipao/server/migrations/00006_economy.sql:84 枚举源 status TEXT NOT NULL DEFAULT pending。
入口：admin 侧栏「经济与商城」→ /laiyipao-admin/#/economy、「数据看板」→ #/dashboard（路由守卫无令牌跳登录，探针预置测试令牌绕过）；miniapp「我的」→ 记录 tab → /#/pages/me/me（uni-app H5 hash 路由，必须带 #/）。服务地址以 <TEST_HOST>:<PORT> 占位，真值不入库。零源码改。

## 方法（真浏览器 Playwright + stub）
双服务：admin Vite 与 miniapp H5 各起本地 dev，viewport 1440x900 DPR1；stub 仅拦 **/api/v1/**（收窄避免误伤源码 /src/api/ 模块）造 admin/orders、admin/dashboard、/admin/reactions={reactions:[]}、/me/orders 等；预置 lyp_admin_token/lyp_token 测试令牌。runnerSha=sha256(JSON.stringify(meta))，文件字节 sha256 另算，两者分列。证据落 gitignored 的 oa4rust/test-results/r587-consistency/（据实披露）。

## 决定性证据（同一订单号三页面文案对照）
| 页面 | 锚点 | 订单 R587-1001 文案 | 语种 |
| --- | --- | --- | --- |
| 枚举源 | 00006_economy.sql:84 | status=paid | 英文枚举 |
| 后台订单页 | EconomyView.vue:505-509 | 状态列 paid | 英文直出 |
| 后台订单页筛选器 | EconomyView.vue:451-455 | pending/paid/delivered/refunded | 全英文 |
| 后台看板 | DashboardView.vue:292-294 | 待支付 2 | 中文 |
| 玩家端 | me.vue:446+958-964 | 已支付 · R587-1001 | 中文 |
主报告 cases=4 fatals=[]：economy 组 statusTagText=paid、filterOptions 四项全英文、orderNoInRow=true；dashboard 组 orderCard 含「待支付 2」、hasOrderKpi=true、backendUnreachable=false；positive 组中文表头命中（ID/订单号/玩家/状态/SKU 等）。miniapp 专项报告 orderCard=「订单记录\ngem_100 · 6.00 元\n已支付 · R587-1001\n11/27 08:00」、statusPaidCN=true、statusPaidEN=false、pe=[]。3 图 1440x900 逐张目视名实相符。

## 三分裁决
可达 P2（成立）：三页面对同一 StatusCode=paid 呈现 paid（英文原值）/待支付（看板中文聚合）/已支付（玩家端中文）三种口径，运营与玩家同屏可见，属跨页文案一致性真缺陷。不可达 L1（排除）：订单 R587-1001 经 stub 注入在三页面均成功渲染，无登录/深层造数壁垒。正常 0（排除）：若属 i18n 按 Audience 显式分流应各端自洽，但同一 admin 子项目内部 Dashboard 中文、EconomyView 英文即自相矛盾，且 admin 全项目 grep 无订单状态中文映射函数（orderStatus 仅为筛选器 ref 名，WikiView.statusText 是文档站健康度与订单无关），排除合理分流。

## 正面对照与去伪（Fail Loud）
正面对照：EconomyView 同页表格表头为中文（ID/订单号/玩家/状态/金额/渠道/创建时间），证明 admin 页面整体中文语境，状态列英文直出是局部疏漏而非整体英文设计；Dashboard 同页其他 KPI 均中文。去伪边界：看板「待支付 2」统计的是 orders_pending（待支付计数）而非本单 paid 状态，不构成与玩家端「已支付」的直接语义冲突——真正冲突在 EconomyView 状态列对 paid 直出英文原值，与玩家端已支付/看板中文语境的口径不一致，据实锁定该项。

## 交互对照（非死代码）
筛选器全英文 label 经 CDP 展开真实可见（截图 economy 面板 pending/paid/delivered/refunded）， {{ row.status }} 状态列随 stub 订单 paid 真实渲染，非死代码。看板 orders_pending=2、orders_paid=1 由 stub 数据驱动真实渲染。miniapp 记录 tab 点击后订单卡真实出现（tabCount=1、bodyHasOrderRecord=true），/me/orders 真实被请求（请求日志末段含 /api/v1/me/orders）。

## 最小修复方向
在 EconomyView.vue 增订单状态中文映射（对齐 me.vue orderStatusText 权威口径），状态列渲染 orderStatusText(row.status)、筛选器 el-option :label 改中文（value 保留英文枚举不改后端契约）；看板「待支付」文案保留（其语义为待支付计数）。业务零改、仅 admin 呈现层，与既有 i18n/契约零冲突。本轮不改源码，修复另立契约。

## Fail Loud 汇总
首版 dashboard「后端不可达：Cannot read properties of undefined (reading \u0027map\u0027)」——fetchReactionLabels stub 返回 {} 形状不对致整页 Promise.all 失败，补 /admin/reactions={reactions:[]} 修复；首版与 positive 组 pageerror「TypeError: flows.value is not iterable」系 stub 造数不全（flows 字段缺），非页面缺陷，据实披露未采信该报错为 bug；miniapp 三版迭代：v1 用 /pages/me/me（无 #/）uni-app H5 hash 路由回首页致 onShow 不触发、/me/orders 未请求，v3 改 /#/pages/me/me 后 loadOrders 仍未调用（onShow 早于 onMounted 的 login 完成、loggedIn 尚 false），v4 切走再切回触发二次 onShow（loggedIn=true）后 /me/orders 真实请求、订单卡坐实。所有 fatal 与报错均 Fail Loud 披露。

## 证据 sha 表
runnerSha = sha256(JSON.stringify(meta)) 为紧凑 JSON sha；文件 sha256sum 为字节哈希，两者分列不可混用。
- runnerSha：主报告 7b6c3745ccd08781dab8d0037ccf99acab6230bd3e425d5297409006495d05a5；miniapp 专项 593e4716f16e3cbbc175fe0866f6cb9b0c69a914ecbbf43e66f1a78bea82bd39。
- 报告文件：r587-report.json e64569519ba886afdaa0c244f2079d0efbb14f72e4173f4e86d7efd4c9fad04d；r587-mini-report.json b2d97a3ad0548b6b68125c38b926e32d6242af952353a2d705dcbf67a25933b5。
- 截图（1440x900，3 张逐张目视）：r587-admin-economy.png 981bc7d72b99d057047fcd58ee718811443f972cc4c77f75aeab132ddeb34da5 / r587-admin-dashboard.png bcd542aaf06ee91e77f034b51c7298b8442ebb0cb092fe467f1c95e6afbf7e17 / r587-miniapp-me.png 1926f5f8f11fd89adbc44a8d1ae98d0adc3fa8c451ffe9a93058c2a3906b7980；正面对照 r587-positive-chinese.png e33621215cb1a1ae48322dbcb918f5600946abb19933dfbff4a2191cee29da17（中文表头命中，sha 存档）。
- 源码：EconomyView.vue 546d5634cb0177a2483b202a99bdf3988bd8914370a7be9c4dd86c2c9ec12701 / DashboardView.vue 903a7c49753035ad6474475383cfe9947bea271736779de31a0f55fe3d6c8d29 / me.vue 303441acbb251a0bb8daa82e294a202a7f2960e7a1fa02edfbc163ab26f980c8 / 00006_economy.sql 87d507ac66c3b3e6aae07c813dd5786bcc5ba06027b43975c0ee2555e5310ad4。
- 探针脚本与报告留 gitignored 证据目录 oa4rust/test-results/r587-consistency/，据实披露不入仓。