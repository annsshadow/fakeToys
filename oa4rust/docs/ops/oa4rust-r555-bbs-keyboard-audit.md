# R555 H5 论坛页 bbs.vue 伪按钮键盘可达性甄别（只读）

- 任务：t75 / attempt 29d82db2-234f-4a12-8506-aadc498844da / 真父 c317bcd3caaf250985127d6c81645f1e47c634e6
- 范围：WCAG 2.1 SC 2.1.1（Keyboard，A）关联 SC 4.1.2（Name Role Value，A）；H5 子项目新实例 oa4rust/mobile/src/pages/bbs/bbs.vue
- 运行环境：自起 uni-app H5（<TEST_HOST>:<PORT>/oa4rust-h5/），Playwright chromium headless 移动视口 390x844 DPR2；双主题 light/dark × 四态 withData/error/empty/errorBanner 共 8 组，另 banner 补探针 2 组，共 10 组全程 fatals=0、pageErrors=0
- 结论：**可达真缺陷 1 项 P2（SC 2.1.1 A 关联 4.1.2 A）**——论坛页 4 类决定性 @tap 伪按钮（发帖 :85、错误重试 :92、有数据错误横幅重试 :101、主题列表项 :103）键盘完全不可达且 AX 语义不可读；均非死代码、无等价键盘入口。

## 一、源码逐行（sha256 bbs.vue=284f21b2f5649561295a3797120aafb2d5aee0ee772572b097c4af2a9121be78）

- :40-57 load()：listGate.begin()+loadBbsTopics(()=>bbsApi.mobileViewAll())，失败保留 previousRows 并 error（describeBbsError 前缀「主题加载失败：」）
- :59-62 onShow()：经 ensureAuthenticated 前置后 load()（登录统一前置，非造数）
- :70-74 openTopic(row)：navigateTo /pages/bbs/topic?id=…；:76-78 newTopic()：navigateTo /pages/bbs/new
- :85 <view class="new-btn ai-brand-fill" @tap=newTopic>发帖</view>（常驻 toolbar，零造数零错误态决定性样本）
- :92 <view class="retry-btn" @tap=load>重试</view>（错误态 tip-error）；:101 同类重试（有数据错误横幅 error-banner）
- :103 <view v-for class="topic o2-list-item" @tap=openTopic(row)>主题项</view>（需 1 条数据）
- 四类候选均无 role / tabindex / @keydown；全页无 input / @confirm / 原生 button；静态 grep role=button|tabindex|@keydown 零命中
- 入口 index.vue:159 论坛 cell 已武装 role=button/tabindex=0/@keydown.enter/.space.prevent，1 跳可达本页（非死入口）

## 二、决定性 CDP 证据（DOM + document.activeElement + AXTree）

| 样本 | DOM role/tabindex/aria | getPartialAXTree / getFullAXTree | 原生 focus 后 | 干净 Tab 30 次 |
|---|---|---|---|---|
| :85 发帖 | 全 null | role=generic name=null | activeElement=BODY axFocused=[] | 0 命中 |
| :92 错误重试 | 全 null | role=generic name=null | activeElement=BODY axFocused=[] | 0 命中 |
| :101 横幅重试（导航往返触发第二次 load 后坐实） | 全 null | role=generic name=null | activeElement=BODY axFocused=[] | 0 命中 |
| :103 主题项 | 全 null | role=generic name=null | activeElement=BODY axFocused=[] | 0 命中 |

- 8 组双主题×四态 distinct=["BODY:"]、body=30、BUTTON=0；newBtn/retry 命中 0。withData/errorBanner 的 topic=30 系 BODY.textContent 含「R555topic」的文本假阳性，distinct 全 BODY 证明焦点始终停在 BODY，据实澄清不计命中。
- errorBanner 态首次进入无横幅，经 index→bbs 导航往返触发 onShow 第二次 load（stub 第 2 次 HTTP 500）后：topics=3、bannerRetry=1、errorText=「主题加载失败：HTTP 500」，再探 :101 坐实上述全 null / generic / BODY / Tab 0。

## 三、去伪与对照（证明方法能抓真缺陷、非选择器假阴性）

- 注入对照：对 :85/:92/:101/:103 注入 role=button+tabindex=0+aria-label 后，partial/full AX 变 role=button name 可读、focusable=true，Tab 命中 10~15 次、Enter 生效；证明原始不可达是被测元素属性缺失而非探针失效。
- 跨页强对照：calendar.vue:295（role=button/aria-label=新建日历 但 tabindex=null）——AX role=button name=新建日历、Tab 未命中（伪可达）；calendar.vue:296（role=button/tabindex=0/aria-label=新建事件/@keydown）——AX 完整可达、Tab 命中。相邻两例证明本方法能区分「仅 AX 语义」与「完整键盘可达」，未把伪可达误判为可达。
- 鼠标对照：点 :85 → /pages/bbs/new（new=true）、点 :103 → /pages/bbs/topic?id=r555-0（topic=true），changed=true；坐实 handler 非死代码、检测器和造数有效。
- 等价入口逐项排除：本页无搜索框 / 输入框 / @confirm；:85 发帖无其它键盘入口；错误重试仅 @tap；列表项点击为唯一详情入口；入口 cell 可达不等于页内控件可达。
- 范围排除：已修 index/message/process/contacts/browse/recycle/calendar 实例、PC 侧覆盖实例不重复立案；本页无原生 button / uni-button 实例需排除。

## 四、最小修复方向（业务零改）

对 :85/:92/:101/:103 逐处显式补 role=button + tabindex=0 + 动态 aria-label（发帖/重试/主题标题）+ @keydown.enter/.space.prevent 复用原 newTopic/load/openTopic handler；保留 @tap 不动，业务函数与数据链路零改。

## 五、证据 SHA-256（证据全在 gitignored oa4rust/test-results/r555/，0 真值入库）

- 源码 bbs.vue=284f21b2f5649561295a3797120aafb2d5aee0ee772572b097c4af2a9121be78；HEAD=c317bcd3caaf250985127d6c81645f1e47c634e6
- 报告：主 609fe39d01abf785c86c225285b1b0df5865636f919a23e18a24a03863123550 / 对照 b019c5cd2eb9906a5316738d9b11a13d66ebcf37d576bc1b0498f0a86818f458 / 干净Tab cf6c207dcf680f719826bbf3b5b74cc81bba85856d9522ce06c310a219b8a43d / banner 补探针 1f47dc78aada9654851b8f25a910c576cef689742524af5a413affd006d4c1eb
- 截图（H5 无主题机制，light/dark 同 sha）：withData fa9b06e2113d4205327863b980e0eeb9aafe39eb3b0ed5df78e6d103688ab75c / error b3df9ee34a22e43077a5ca0b1e6cefd96e47c67b0f25f91518766f6b52919bf7 / empty bd749f00417d2491036bf7ec384355ab40ef070d991f4e4e548f30b1f2548e65 / banner 81928c24056e5a74c0ddf62448a87938e0e240e16a64f078b06d2464e62186fc
- Fail Loud：主探针 errorBanner 态下拉刷新未能触发横幅（before=0 after=0），已据实披露并用导航往返补探针独立坐实 :101，未静默略过；原始报告与补探针报告均保留。
