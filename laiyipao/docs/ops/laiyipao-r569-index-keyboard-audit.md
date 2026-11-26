# R569 首页 index.vue 裸 @click 伪按钮键盘可达甄别报告（WCAG 2.1 SC 2.1.1 A / 4.1.2 A）

## 一、结论
可达真缺陷 1 项，P2（SC 2.1.1 A，关联 4.1.2 A）。首页 index/index.vue 是 pages.json 第一页（启动 0 跳直达）、游客态自动 store.login()、全项目无 tabBar，是到达所有子页的唯一枢纽；9 类裸 @click 伪按钮全部无 role/tabindex/aria、AX generic/name=null、焦点停 BODY、干净 Tab 30 次候选 0 命中——键盘用户被困首页、无法离开，影响面为全项目最大。判据据实，不预判 0、不硬凑。

## 二、范围与入口
- 目标：laiyipao/miniapp/src/pages/index/index.vue（sha256 见第七节，1123 行）。
- 9 类锚点（据实 9 条 @click 行，实际 DOM 实例以浏览器为准）：
  - :40 retry 重试（v-if store.loginError 错误态）
  - :62 annAll 全部 N 条公告入口
  - :68 annItem 公告项 ×3（toggleAnn 展开）
  - :141 dimHint 空维补导 ×2
  - :162 mgRow 最值得补的一维 ×3
  - :186 actionCard 主行动卡（goStage）
  - :227 playItem 玩法宫格 ×9
  - :240 codexItem 图鉴宫格 ×6
  - :256 elemChip 元素 chip ×5
- 入口：pages.json pages[0]，0 跳直达；onMounted 自动 store.login() 游客态；全项目无 tabBar；首屏控件常驻（错误态为 retry）。
- 排除：本页 svg[aria-label=构筑五维雷达图]（:196 起）与闯关进度 svg 属原生语义控件，作正面对照不计数。

## 三、方法
真浏览器 CDP：Playwright chromium headless，移动视口 390x844 DPR2，isMobile/hasTouch；stub 仅拦 **/api/v1/**（config/announcements/wallet/me/stars + auth），预置 lyp_token 跳过 guestLogin；6 组 light/dark × withData/empty/error + 2 组功能探针（withData/error）；DOM.enable + Accessibility.enable；probe 用 querySelectorAll + DOM.describeNode 取 backendNodeId + getPartialAXTree({backendNodeId}) + getFullAXTree 按 backendDOMNodeId 映射；focusProbe 原生 DOM.focus 读 activeElement + axFocused；tabAll 按 Tab 30 次。测试基址在报告中仅以 <TEST_HOST>:<PORT> 占位，真值不入库。

## 四、决定性证据（8 组全 fatals=0 / pageErrors=0）
- DOM 三属性：9 类 raw attrs role/tabindex/aria 全 null（正常态 8 类、错误态 retry）。
- AX 语义：partial + full 均 role=generic name=null ignored=false。
- 焦点：DOM.focus 后 activeElement=BODY（isBody=true、isTarget=false）、axFocused=[]。
- 干净 Tab（未注入）30 次：全部选择器 hits=0，distinct=["BODY:"]——键盘用户确实被困首页（全项目唯一枢纽），不止语义缺失。
- 注入对照：注入 role=button+tabindex=0+aria-label+keydown 后 AX=button name=R569-inj-*，Tab 命中 annAll=1/annItem=3/dimHint=2/mgRow=3/actionCard=1/gridItem=15/elemChip=5/retry=15；distinct 含 UNI-TEXT:ann-all、UNI-VIEW:ann-item、UNI-VIEW:action-card card-hero tap（UNI-TEXT/UNI-VIEW 身份独立确认，非宿主 wrapper 假阳性）。
- 正面对照：本页 svg[aria-label=构筑五维雷达图] count=2、AX role=image name=构筑五维雷达图（withData/empty）；错误页无此 svg（count=0，符合预期）。
- 功能真命中（注入后 Enter 经 Vue @click 链）：annItem→annOpen0=true 展开；annAll→#/pages/announcements/announcements；dimHint→#/pages/bag/bag；mgRow→#/pages/equipment/equipment；actionCard→#/pages/stage/stage?level=2；playItem→#/pages/stage/stage；codexItem 与 elemChip→#/pages/reactions/reactions；retry→retryExists=false（首次 HTTP500、重试后恢复首页）。非仅焦点移动。
- 截图目视：withData 显示公告 3 条+雷达图+补导+边际收益 3 条+行动卡+玩法 9/图鉴 6 宫格+元素 5 chip；error 显示「无法连接服务器／配置拉取失败 HTTP 500／重试」；empty 无公告但其余控件常驻。

## 五、等价入口与排除
全页 grep role=|tabindex|@keydown|@confirm|@longpress|<button 零命中，仅 9 条 @click；全项目无 tabBar；首页为唯一枢纽，键盘用户无其他路径离开——不存在等价键盘入口。uni-app 编译为 UNI-TEXT/UNI-VIEW，注入后 AX 身份可读，检测器有效。

## 六、最小修复方向
逐类显式补无障碍属性，业务 handler 零改：9 类元素补 tabindex=0 + 动态 aria-label（公告项取标题、宫格取 m.label、元素取 e.name、retry 取「重试」）+ @keydown.enter / @keydown.space.prevent 复用原 handler（go / goAnnouncements / toggleAnn / goStage / retry）；导航类用 role=link（annAll、dimHint、mgRow、actionCard、playItem、codexItem、elemChip），切换类用 role=button + aria-expanded（annItem），retry 用 role=button。沿用 R561/R562/R563/R564/R565/R567 同族范式。

## 七、证据 SHA-256（runner 紧凑 JSON sha 与文件 sha256 分列）
- runner R569_DONE：c2145c4e58dc6e27a70db0e55cab7bd897921b02163d113f3b90aa7dd0223b27（cases=8 fatals=0 pe=0）
- r569-report.json（47285B）：3bf294d10e2809eca8f0317044be1667ed0064561dbfb541f57701067962a8ba
- r569-withData-light.png：8447a6e9318ca302b5e970483bd107e6105ad67127f1c698c592b90c83d0fc61
- r569-withData-dark.png：d44b25e6ad10197d6f42e1a642a7cb3b3bde319216b3033c6c0fbd2cc3add179
- r569-empty-light.png：ce78a6aa4daf988a3af1b52ff552035c4a88f09d97402b86d51ba8bd2600eee5
- r569-empty-dark.png：a164fb94b84f45f47bf568732998f369a28af6a0299dc97fa4e66b453421e8ee
- r569-error-light.png / error-dark.png（同 sha）：99430903cdd4976f62762718d8e1a9fc1890eb35fed1016dc973419be989909c
- 源码 index.vue：68608d9b8b745210c9aabe2652ee2cb7dc13fe8e0f2805acec52e82e67463107
- 源码 client.ts：cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2
- 源码 game.ts：2055efb8ac15fb7cc3f01173448049e546233e5bcf2221e961e45e8f059090a6
- 证据目录 oa4rust/test-results/r569/（根 .gitignore:28 已忽略，借用并据实披露）；零源码改。
