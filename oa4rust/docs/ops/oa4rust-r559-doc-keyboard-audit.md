# R559 H5 文档页（doc.vue）伪按钮键盘可达性甄别

## 一、结论

可达真缺陷 **1 项，P2**（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name/Role/Value A）。doc.vue 内 8 类 `@tap` 伪按钮无 `role`/`tabindex`/`@keydown`，CDP 实证辅助技术语义不可读、键盘完全不可达；入口 index.vue:134 一跳直达且 handler 非死代码。同缺陷类已覆盖不判 0，不凭静态 `@tap` 预判 P2，全部经真浏览器坐实。

## 二、源码锚点（oa4rust/mobile/src/pages/doc/doc.vue）

- :194 `.tip-error .retry-btn` `@tap=load()`（附件错误态）
- :203 `.empty-guide-link.ai-pressable` `@tap=load()`（附件空态，常驻零造数，决定性样本）
- :208 `.error-banner .retry-btn` `@tap=load()`（附件非空+错误 banner）
- :211 `v-for .item` `@tap=downloadAttachment(row)`（附件项）
- :228 `.tip-error .retry-btn` `@tap=load()`（文档库错误态）
- :236 `.empty-guide-link.ai-pressable` `@tap=load()`（文档库空态，常驻零造数，决定性样本）
- :241 `.error-banner .retry-btn` `@tap=load()`（文档库非空+错误 banner）
- :244 `v-for .item` `@tap=downloadAndOpen(row)`（文档项）
- 全页 `@confirm`/`tabindex`/`role=button`/`@keydown` 静态 0 命中，无等价键盘入口；入口 index.vue:134 文档 cell 已武装 `role=button tabindex=0 @keydown.enter/.space.prevent`，一跳 `goPage` 到本页。

## 三、决定性 CDP 证据（Playwright headless 390x844 DPR2，双主题×四态 8 组，fatals=0 pageerror=0）

- DOM：`getAttributes` 逐一坐实 8 类锚点仅含 `data-v-*/class`，`role`/`tabindex`/`aria-*` 全 `null`。
- AX：`Accessibility.getPartialAXTree` 直接取节点 `role=generic name=null`（决定性；`getFullAXTree` 按 backendDOMNodeId 偶配到相邻 StaticText，仅作旁证不采信）。
- 焦点：`DOM.focus` 后 `document.activeElement` 为 BODY（`isBody=true isTarget=false`）；`getFullAXTree` 中 `focused=true` 非 RootWebArea 节点为空。
- 干净 Tab：8 组各按 Tab 30 次，`distinct=[BODY:...]`、`body=30`、`BUTTON=0`、候选 0 命中。
- 四态覆盖：withData/error/empty/errorBanner 均渲染到位；error 两区错误文案「附件加载失败：HTTP 500」「文件加载失败：HTTP 500」；banner 态经导航往返（index→doc 二次 onShow load）坐实 retry=1 且保留数据行。

## 四、反预判与去伪（去伪三件套）

- 注入对照：对 8 类锚点注入 `role=button tabindex=0 aria-label`，AX 由 `generic/null` 变 `button/可读名`，Tab 命中 10~15 次，Enter 生效 —— 证明检测器有效、缺陷可修复（inj194/203/208/211/228/236/241/244）。
- 跨页正面对照：同页外 calendar.vue:295（`.calendar-create-btn`，role=button/aria-label=新建日历但 tabindex=null）AX 语义可读却 Tab 命中 0，属“伪可达”；calendar.vue:296（`.compose-btn`，role=button/tabindex=0/aria-label=新建事件/@keydown）Tab 命中 8，属完整可达 —— 证明方法能区分伪可达与真可达，双重对照均按契约排除、不混入 8 类。
- uni-button 另一族排除：`uni-button.tool`(:174/:177) count=2、`uni-button.o2-btn`(:201/:202) empty 态 count=3，attrs class=tool/o2-btn、AX role=generic name=null；原生 `button` count=0。属 uni-app 编译层另一族，按契约排除，不因存在而漏判 view 族。
- 鼠标非死代码：`tap` 事件监听坐实 withData 点 item 触发下载（📎R559att-0/📄R559file-0）、error 点 retry「重试」、empty 点链接「刷新」、banner 含 item+retry —— `@tap` handler 真实绑定、状态可达。

## 五、最小修复方向（业务零改）

按 R544/R546/R548/R550/R552/R554/R555/R557 逐页显式补 `role=button`+`tabindex=0`+动态 `aria-label`+`@keydown.enter/.space.prevent` 复用原 handler，`@tap` 不动；:211/:244 附件/文档项用文件名动态命名。

## 六、证据 SHA-256（证据均 gitignored `oa4rust/test-results/r559-*`，未入库）

| 类别 | 文件 | SHA-256 |
| --- | --- | --- |
| 源码 | doc.vue | 3e17484e11d2757a77cc7607231cd4f6061436695dec5bee3d89bf356f890370 |
| 源码 | index.vue | 892d75341e36d115bf61b21b53aaf62d5e53b7f94c06b57b382955cd8408cd40 |
| 源码 | main.ts | b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b |
| 源码 | calendar.vue | 7e355af979b1e1fc894082264b4261d631aeb1615b3388c074d12ed1e2479994 |
| 报告 | r559-report.json | a27ca656ee6403528d9717bd9af163460db89cf1f11382863cceb96134b1ff61 |
| 报告 | r559-detail-report.json | 26727792cdf45dcaa07c67ec248564e33ebb92d8bddcbaf65ae28771a69a9ca6 |
| 报告 | r559-tab-report.json | 27f1282970ac887d4a5717ec1bb5bf5bd7f6fe1c4a121c7a2ea422803b787d65 |
| 报告 | r559-supplement-report.json | 2275388723b95936ca6187b8bb46689b9722bf1b584194f322a13b34fb840636 |
| 截图 | r559-withData-light.png | 27444fb3c451a15c817712cac23ddce568fb52932cf43a9e4e6f0e83085cd3aa |
| 截图 | r559-error-light.png | 92f0700f3b9ae9e07d116a0d8e50e3275559ca88b4ccb1d330c0912bf20ee2ea |
| 截图 | r559-empty-light.png | 0aa52eb69f23aa6e2f91d556477cc3193dda02ffaf4216e18d7f47783b797df7 |
| 截图 | r559-errorBanner-light.png | 67b3d60185c20d35754aa84f2cc27ca79e2e4c104e1767456082cdc202e9ef19 |

访问基址（仅报告占位，真值不入库）：`<TEST_HOST>:<PORT>`；H5 App.vue 无主题机制，light/dark 截图同 sha。四张截图经 read_image 目视有效。