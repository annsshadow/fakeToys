# R561 laiyipao/miniapp 我的战报页裸 @click 伪按钮键盘可达性甄别（只读）

## 结论
判定：可达真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。battles.vue「我的战报」页 6 类裸 @click 控件键盘完全不可达、辅助技术语义不可读。属新实例，不因同 SC 维度已被 oa4rust H5 覆盖而判 0，亦非据静态预判。

## 范围与入口
- 目标：`laiyipao/miniapp/src/pages/battles/battles.vue`（sha256 `84d51fa7eba2d7cbd78d27c3e0cd4d68b535dc8232e7ad9f7bdb90416e63ac20`）。
- 6 类候选：:9-18 三个 filter-chip（全部/胜利/失败）；:120 空态「去关卡页挑一关开打 →」（零造数决定性样本）；:124-129 列表行 battle-row.tap；:174 复制回放哈希 copy-hash-btn；:177 去验真此局 verify-btn；:184 加载更多 load-more。全部为 `<view>/<text @click>`，无 role/tabindex/aria/@keydown。
- 入口：H5 hash 路由 `#/pages/battles/battles` 直达；onMounted(:576-579) 未登录先 store.login() 再 reload()，探针以预置令牌跳过 guestLogin，空态链接零造数可达。
- 正面对照：同页 :45 `<svg aria-label=战绩分数趋势>`；全项目 a11y 以 svg aria-label 为主，无 role/tabindex 的 button 范式控件。

## 方法
- 自起 `pnpm dev:h5 --port <PORT>`（真值不入库，报告 base 写 `<TEST_HOST>:<PORT>`）；Playwright chromium headless 390x844 DPR2，stub 仅拦 `**/api/v1/**`。
- 决定性判据：DOM.getAttributes 取原生节点 role/tabindex/aria；DOM.focus 后读 document.activeElement；Accessibility.getPartialAXTree + getFullAXTree 按 backendDOMNodeId 映射读 role/name，过滤 focused=true 且非 RootWebArea 节点。严禁探宿主 wrapper（R535 假阳性教训）。
- 6 组：light/dark × empty/withData/error，fatals=0、pageErrors=0。

## 决定性证据
- 原始 DOM：6 类候选 role/tabindex/aria 全 null；partial/full AX role=generic、name=null、ignored=false。
- 焦点：DOM.focus 后 activeElement=BODY、isBody=true、isTarget=false，getFullAXTree 中 focused 非根节点 axFocused=[]，无辅助技术可读焦点。
- 干净 Tab 30 次：6 类候选命中全 0，distinct=[`BODY:`]，全程停在 BODY。
- 注入对照：注入 role=button + tabindex=0 + aria-label=R561-S-* 后，AX role=button、name 可读，Tab 30 次命中 10~15 次（empty 链接 15，filter/row 各 10），证检测器有效且缺陷可修复。
- 正面对照：svg[aria-label=战绩分数趋势] AX role=image、name=战绩分数趋势（无 tabindex 故不进 Tab），证 AX 通道正常、非探针失效。
- 鼠标对照非死代码：空态点 :120 → `/pages/stage/stage` changed=true；withData 点行 → detail=true、copyBtn=1、verifyBtn=1。
- Fail Loud 据实：copyToast=false、verifyNav.changed=false（dispatchEvent(MouseEvent tap) 未走 uni-app 事件链，仅旁证不据此判缺陷；detail=true 已坐实主链可达）。error 态 stub HTTP500 回落空态链接，与 empty 同形。

## 等价入口与排除
- 全页无 input/@confirm、无原生 button 参与候选、无其他键盘等价入口；uni-app H5 把 view 编译为 UNI-VIEW，注入后 Tab 命中节点即 UNI-VIEW/UNI-TEXT 本身（非 wrapper 假阳性）。
- 空态无行/加载更多不另计；已修 oa4rust H5 各页与 PC 覆盖实例不重复立案。

## 最小修复方向
逐页显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler，@click 不动、业务零改（类比 R556/R560）。

## 证据（gitignored：oa4rust/test-results/r561/，借用已忽略目录存放 laiyipao 证据，据实披露）
| 项 | sha256 |
| --- | --- |
| 主报告 r561-report.json | 594d44ecf4400833b0d4de550f081b83ec4c95c197fa96515d40874f0bb6d6e2 |
| 补充报告 r561-supplement-report.json | 03aba36bd7806b2306ba9f0ec9054abb54e19ae9149265de5096d3f0d8602644 |
| 截图 withData（light=dark 同 sha） | d676149e9222189edd1030273ce1c4f307e8841ab3ccbd1df2104baadeb4faec |
| 截图 empty（light=dark 同 sha） | f4ec562a0f7e2c7a7754e2a63eda13cc3a7b50a6236cda977cf09c054f72e5c5 |
| 截图 error（light=dark 同 sha） | db71bf4b1bdd04414917c68edddefb0f927a203baadca702a4d8f8c9d0dfabc5 |
| 源码 pages.json | cf3d8495926a41b81c5fb168f0ed69e032f364277d7730aa4d73fda5ea8c8191 |
| 源码 store/game.ts | 2055efb8ac15fb7cc3f01173448049e546233e5bcf2221e961e45e8f059090a6 |
| 源码 api/client.ts | cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2 |
| 基线 HEAD | 6eedb85853d156a01ef1396c9c645a007ab9dd6f |

截图 light/dark 同 sha：miniapp H5 无主题切换机制。三张截图均经 read_image 目视有效（战报页含筛选/统计/趋势/列表/详情/复制/验真/加载更多；空态与错误态链接正常渲染）。