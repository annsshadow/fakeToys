# R588 支付订单状态中英混排一致性修复（跨页一致性方向首个修复）

## 结论
修复 EconomyView.vue 订单状态英文直出：状态列渲染中文映射、筛选器 label 中文化而 value 保留英文枚举。与玩家端 me.vue:958-964 权威口径逐字对齐（pending=待支付 / paid=已支付 / delivered=已发货 / refunded=已退款，未知兜底原值）。真浏览器坐实同一订单 R588-1001（status=paid）状态列显示「已支付」、筛选器四项全中文、选中中文项后请求仍发英文 status=paid，0 pageerror。P2（R587b 甄别 + t135/t136 终审判定）闭合。

## 范围与零改边界
写域精确 3 文件：M laiyipao/admin/src/views/EconomyView.vue（映射函数 + 2 处展示文案）、A laiyipao/admin/tests/R588.consistency.contract.test.ts、A 本文档。
零改：后端 00006_economy.sql/迁移/handler、玩家端 me.vue、他页、main.ts、筛选请求参数（仍传英文枚举）、el-tag type 判据、CURRENCY_LABEL/REASON_LABEL、样式、既有测试、其他文案。

## 方案取舍（Fail Loud）
方案 B：新增局部 orderStatusText(s: string): string 与 me.vue 权威映射逐字一致；筛选器 el-option 的 label 改中文、value 保持英文枚举（后端契约零改）；状态列 {{ row.status }} 改为 {{ orderStatusText(row.status) }}；未知枚举 backstop 返回原值。
拒绝方案 A（全局增强器 / 统一 i18n 层）：越界他页且与既有中文语境重复，与 R578/R580/R582 先例一致采用逐页显式最小改。
拒绝 htmlFor+id：需同时改 label 与 select 两面，aria 映射不涉及本缺陷（本缺陷是文案口径非可访问名），比新增映射函数面更大。

## 改动明细
- :35 后新增 orderStatusText 映射函数（12 行，含注释）。
- :451-455 四个 el-option label: pending/paid/delivered/refunded → 待支付/已支付/已发货/已退款；value 与 @change=applyOrderFilter 原样保留。
- :505-511 状态列 el-tag 内容 {{ row.status }} → {{ orderStatusText(row.status) }}；:type 判据（paid/delivered → success，其余 info）原样保留。

## 契约与 RED→GREEN
R588.consistency.contract.test.ts：7 用例 = 3 核心（映射函数逐字一致 / 状态列渲染映射非直出 / 四筛选 label 中文且 value 英文）+ 4 守卫（请求参数零改仍传英文枚举 / el-tag type 与其他文案映射零改 / loadOrders+applyOrderFilter+onExportOrdersCsv 函数体零改 / 玩家端 me.vue + Dashboard orders_pending 正面对照零改且无全局增强器）。
踩坑：初版用 fileURLToPath(new URL(...)) 读源码在 vitest 报「The URL must be of scheme file」→ 改用 node:path resolve(process.cwd(), 相对路径)，规避 ESM file URL 限制。
GREEN 修复版 EXIT0 7passed。RED 由反向 ops_edit_file 移除 3 处修复（恢复真父 sha 546d5634）→ EXIT1 3failed（核心1/2/3）| 4passed（守卫4-7）→ 恢复修复版 revision 5cb552ef 复核 GREEN。据实披露：RED 由反向编辑实现，未另建独立脚本；测试阶段 HEAD 未变。

## 三 verify（exit code 全 0）
- V1 定点：cd laiyipao/admin，npx vitest run tests/R588.consistency.contract.test.ts → 7 passed。
- V2 全量：cd laiyipao/admin，npm run test → 38 files passed / 505 tests passed，无 skip。
- V3 类型：cd laiyipao/admin，npm run type-check（vue-tsc --noEmit）→ 无错误。

## 真浏览器 E2E 决定性判据
Runner oa4rust/test-results/r588/r588-e2e.cjs，自起 admin Vite 于 <TEST_HOST>:<PORT>（本地回环 dev 端口，被占用时据实顺延，真值不入库），Playwright chromium headless，viewport 1440x900 DPR1，stub 仅拦 /api/v1/**（admin/me、admin/orders、admin/dashboard、admin/economy），预置测试令牌。
R588_DONE runnerSha=2c6c96c26def7977422cc2aa34098328812f6104ddcc9415a08ba31c6b956f88，cases=2，fatals=[]，pe=[]/[]（两 case 均 0 pageerror）。
- economy 组：orderNoInRow=true；statusTag=已支付（statusIsCN=true、statusNotEN=true）；filterOptions=[待支付, 已支付, 已发货, 已退款]（filterAllCN=true）；选中「已支付」后 requestStatusAfterFilter=paid（requestUsesEN=true，坐实 value 仍为英文枚举、后端契约零改）。
- dashboard 正面对照：runner 用 .kpi-card 选择器未命中返回 orderCard 空（据实披露为探针选择器瑕疵，与修复无关）；同次截图 r588-dashboard-positive.png 目视清楚显示订单 KPI「1 / 待支付 2」，中文正面对照成立。
- 3 图 1440x900 逐张目视名实相符：r588-economy-default.png（支付订单表状态列「已支付」）、r588-economy-filtered.png（筛选器「已支付」选中态 + 中文选项）、r588-dashboard-positive.png（订单 KPI「待支付 2」）。
服务 evidence 目录 oa4rust/test-results/r588/ 全 gitignored，不入库。

## 脱敏声明
端口/host 真值一律以 <TEST_HOST>:<PORT> 占位；测试令牌仅存在于 gitignored runner 与 stub 内存中，不入库。本文档不含自身 sha（避免自引用悖论）：写入后计算 sha 不会改变内容。runnerSha = sha256(JSON.stringify(meta)) 为紧凑 JSON sha，与下列文件字节 sha256sum 分列不可混用。

## 证据 sha 表
runnerSha（紧凑 JSON）：R588_DONE 2c6c96c26def7977422cc2aa34098328812f6104ddcc9415a08ba31c6b956f88。
报告文件：r588-report.json 30c59b4759507284511ca6b3a2d08f8d17c9cb9ddc9c9fe591ec31bee5e3d457。
源码与契约：EconomyView.vue（修复版）5cb552efad6caa2bd8dd469bc6ca0b777d913b08661f98144d8c7570a7add116；EconomyView-parent.bak.vue（真父）546d5634cb0177a2483b202a99bdf3988bd8914370a7be9c4dd86c2c9ec12701；R588.consistency.contract.test.ts 1a1e52b86f6e4c65b3501e10c91cad8a3f3cae45711b5bc99a20cb14a529edb6。
日志：/tmp/r588-green.log 34439432d556838a78d51c9619d453dca2ce53893d8a243dd448419320ef46d8；r588-red.log cec7d6e27f45b93b39c269ca2cb4940ccf6088ef0e2c8bc03993f737cd243d6a；/tmp/r588-v1.log bde652c88f176aad8ef5b8d377fd2358a5d3e019ea41967fcc7053a126f84fe2；/tmp/r588-v2.log b04cacae05e930de9d1ba7d5170a5e1b4ec5c0517302eb168b1afd92ea58ea04；/tmp/r588-v3.log 84155fa5bcb6a3e9b085eb417db166c6b194e4d6b6e5b81b78485727b5138859；r588-e2e.log cccfa773ba4e2535f2f51e1d2d8ed36e7dd9db6dfb220ad20cf981e3c65b043f。
E2E 脚本：r588-e2e.cjs 612a3347d5770eff20b31de26883f4c10fe9e892b40ba467f377880551e2b704。
截图（1440x900，3 张逐张目视）：r588-economy-default.png f0f530c4299b477ec13868c6775f5c4f3231971d82fb1ab1e803dead1c587dff；r588-economy-filtered.png 06b213039691d18e1878df727501e7227c81faf01a13c5932f24a1c13d1ee233；r588-dashboard-positive.png d472ffd581605922410571a0d69fd8ede346deb8bf9fa28f42a5eb160125acda。
备份：oa4rust/test-results/r588/EconomyView-fixed.bak.vue 5cb552efad6caa2bd8dd469bc6ca0b777d913b08661f98144d8c7570a7add116；EconomyView-parent.bak.vue 546d5634cb0177a2483b202a99bdf3988bd8914370a7be9c4dd86c2c9ec12701。
