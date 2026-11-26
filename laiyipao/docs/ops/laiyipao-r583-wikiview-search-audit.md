# R583 WikiView 搜索框 placeholder 补名去伪甄别（验证型，判 0 收官）

## 结论
判 0（正常，无缺陷）：WikiView 搜索框原生 `<input type=search placeholder=...>` 无 label/aria-label，但真浏览器 CDP 实证其可访问名为 placeholder 全文「搜索关卡 / 敌人 / 技能 / 装备 / 反应 / 专精（回车跳第一条）」——非空。按 WCAG 4.1.2 机械判据（name 非空即通过）据实判 0 收官，与 R577 任务 ID Input 经 placeholder 补名判 0 先例一致，严禁硬凑 P2。空态与输入文本后 name 均稳定可读。

## 立项性质与争议披露（Fail Loud）
验证型甄别。诚实披露：所谓「laiyipao/admin 覆盖空白」前提不成立——台账 47 处命中，chartA11y（§97.3）、BattlesView 星级（§90.2）、全局焦点环（§59.1）已修。本候选为单一实例，维度与 oa4rust R212/R214/R217/R235 搜索框可访问名同族，属跨子项目新实例而非新维度；本轮判 0 亦为有效实测收官。

## 范围与入口
目标：`laiyipao/admin/src/views/WikiView.vue`:660-670` 搜索框（v-model=searchText、type=search、class=search-input、placeholder 见上、@focus/@input/@keydown.enter.prevent=pickFirst/@keydown.esc/@blur），全页无 `<label for>` 指向、无 id、无 aria-label/aria-labelledby。入口零跳 `#/wiki`（router/index.ts:34，壳外全宽），AdminLayout 侧栏「玩家玩法文档站」直达；AUTH_REQUIRED 正则 `/^\/admin(?:\/(?!login)(.*))?$/` 不覆盖 /wiki，但守卫 :75 无令牌仍跳登录，故探针预置测试令牌绕过（/wiki 不走 :79 令牌验真分支）。load() 用公开 `/config`（auth:false），fetchDashboardSoft 401 预期吞掉不跳登录。搜索框页顶空态常驻，零操作可验。零源码改。

## 方法（真浏览器 CDP）
Playwright chromium headless，viewport 1440x900 DPR1；DOM.enable + Accessibility.enable；querySelectorAll → describeNode({nodeId}) 取 backendNodeId → getPartialAXTree({backendNodeId,fetchRelatives:false}) 与 getFullAXTree 按 backendDOMNodeId 对齐；DOM.focus 后读 activeElement。stub 仅拦 `**/api/v1/**` 造 config（1 关卡 R583-test-level）+ admin/dashboard 401；服务地址以 `<TEST_HOST>:<PORT>` 占位，真值不入库。证据落 gitignored 的 `oa4rust/test-results/r583/`（据实披露）。

## 决定性证据（判 0 依据）
- 空态：partial 与 full AX 均 `role=searchbox`、`name=搜索关卡 / 敌人 / 技能 / 装备 / 反应 / 专精（回车跳第一条）`（=placeholder 全文）、ignored=false；DOM attrs 仅 class/type/placeholder，无 aria/label 属性。
- 动态维度：输入「R583-test-level」后（placeholder 视觉淡出），partial/full AX name 仍为同一 placeholder 全文——可访问名稳定可读，未因输入而丢失；据实披露 Blink 对 `<input type=search>` 的 placeholder 回退为 accessible name 的机制。
- 焦点身份：DOM.focus 后 activeElement.tagName=INPUT、type=search、isBody=false、aria=null，焦点坐落原生 INPUT 本身而非宿主 wrapper（R535 去伪，无宿主假阳性）。
- 页面渲染：空态显示「玩家玩法文档站」标题+搜索框+侧栏目录+正文表格；输入态显示命中面板「关卡 / R583-test-level / 第 1 章 · 关卡 1」。3 图 1440x900 逐张目视名实相符。

## 正面对照与去伪（Fail Loud）
- 注入对照（决定性）：注入 `aria-label=R583-inj-search` 后，partial/full AX name 实时变为 R583-inj-search，证明 AX 通道正常、name 完全由可访问名属性驱动——反证空态 name 来自 placeholder 回退而非通道失效。
- 正面对照：同页 `aside.wiki-nav[aria-label=目录]` partial/full AX `role=complementary`、`name=目录`、ignored=false，证全局 AX 通道正常。
- 去伪边界：placeholder 作名在理想实践上并非最优 label，但 WCAG 4.1.2 机械判据以 name 非空为准，R577 有 placeholder 的 antd Input 已据同判据判 0；本例据实判 0，不因「维度已覆盖」或「无显式 label」硬凑 P2。

## 交互对照（非死代码）
输入「R583-test-level」→ 命中面板真实渲染（panelBefore: panel=true、hits=1、text 含关卡与章节）；回车 → hash 变为 `#/wiki?tab=levels&focus=level:1`、搜索框文本保留、高亮行数 focusRows=1，坐实 pickFirst 深链跳转与高亮生效。键盘 Tab 30 次未直接命中搜索框（hits=0、distinct=[]）——据实披露：本轮为可访问名维度，键盘 Tab 顺序不在裁决范围；输入与回车键盘交互均实测生效。

## Fail Loud 汇总（探针三次致命与特性披露）
首版探针 fatal「waitForSelector input.search-input 超时」——根因无令牌被守卫重定向空页，预置测试令牌修复；第二版 fatal「Element is not focusable」——根因正面对照 aside 不可 focus 却走 DOM.focus，跳过其 focus 修复；第三版 fatal 前 stub 模式 `**/api/**` 误伤源码模块 `/src/api/client.ts`（路径含 /api/）返回空对象致应用未挂载，收窄为 `**/api/v1/**` 修复；最终 cases=3 fatals=0 pe=[0,0,0]。CDP axFocused 未置位（axFocused=[]、isTarget=false）系 Vue/浏览器特性，与 R579/R581/R582 同类，不影响 name 裁决，据实披露不采信该字段判可达。所有 fatal 均 Fail Loud 未采信。

## 证据 sha 表
runnerSha = sha256(JSON.stringify(report))；文件 sha256sum 为字节哈希，两者分列不可混用。
- runnerSha：`d41f5ae6c5f63921da5f7e7372e62308917a2a8c32b735bd5da9ef933190e081`。
- 报告文件：r583-report.json `82b6247f73ce4eade71261441d277011af169813ab6ff85063129b320613a81e`。
- 截图（1440x900，已逐张目视）：empty `3ac1d49961f77a01c175a17ce590ddc4a946e78f8a32f316888ae6c482681ff4` / typed `81b7178460b19bec90c48ef3b616aec64f4835d93e983ea2eeedb448f62d3f05` / injected `3ac1d49961f77a01c175a17ce590ddc4a946e78f8a32f316888ae6c482681ff4`（empty 与 injected 同 sha，注入 aria-label 不改视觉，符合预期）。
- 源码：WikiView.vue `62f1dfbef9b0d610aeac064adfce75099dc18b54e18b38693497e2e6bfedb30c` / router/index.ts `f98c4971b6832adb2e5f60d16d22a8e6d796619ff89dc2811f518d6f9e06a793`。
- 探针脚本：r583-e2e.cjs（最终版，含预置令牌+收窄stub+aside跳过focus）；诊断脚本 r583-diag.cjs / r583-diag2.cjs 仅用于定位 fatal，未采信为证据。
