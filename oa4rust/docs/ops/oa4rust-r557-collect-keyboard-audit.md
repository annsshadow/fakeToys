# R557 H5 我的收藏页伪按钮键盘可达性甄别（只读）

## 结论

可达真缺陷 1 项，P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。
`oa4rust/mobile/src/pages/collect/collect.vue` 6 类交互控件均为 `<view @tap>`，无 role/tabindex/@keydown；CDP 实证 DOM 三属性全 null、AX role=generic/name=null、焦点无法进入且 Tab 序列零命中。键盘用户无法操作收藏项、删除、重试、去浏览与刷新。

## 源码锚点（HEAD=131da6e2c，只读未改）

- :86 `.retry-btn @tap=load` 错误态重试
- :93 `.empty-guide-link @tap=goBrowse` 空态去浏览内容（常驻，新用户零造数）
- :94 `.empty-guide-link @tap=load` 空态刷新（常驻）
- :100 `.retry-btn @tap=load` 有数据错误横幅重试
- :102 `.item v-for @tap=openUrl(r)` 收藏列表项
- :107 `.item-del @tap.stop=removeRow(r)` 列表项删除（.stop 须保留）
- 入口：pages.json tabBar「我的」→ mine.vue:32-34 goCollect → navigateTo /pages/collect/collect，一跳可达。handler load(:23)/openUrl(:36)/removeRow(:44)/goBrowse(:72) 均真实定义。
- 源码 sha256：`e627b885361751205cd2fa401d643e530da2dcf7f7db967e0c7f839e1282b46c`。

## 决定性 CDP 证据（自起 uni H5，Playwright 390x844 DPR2，光/暗双主题）

判据：`document.activeElement` + `Accessibility.getFullAXTree` 中 `properties.focused=true` 的非 root 节点 AX role/name；严禁探宿主 wrapper（R535 假阳性教训）。

- 6 类锚点 DOM `role`/`tabindex`/`aria-label` 全 null；partial+full AX `role=generic`、`name=null`。
- 原生 `DOM.focus()` 后 `activeElement.tagName=BODY`、`isBody=true`、`axFocused=[]`（首锚点干净测量）；顺序测量中 :107/:94 焦点显示 UNI-VIEW 系前一锚点已注入 tabindex=0 的探针假象，原始不可达由未注入的干净 Tab 探针统一坐实。
- 干净 Tab 探针：8 组（light/dark × withData/error/empty/errorBanner）全部 `body=30`、`BUTTON=0`、retry/emptyLink/item/itemDel 命中全 0、`distinct=[BODY:]`。
- 注入对照：5 类锚点注入 role=button+tabindex=0+aria-label 后，AX 变 `role=button`、name 可读，Tab 命中 10~15 次、Enter 生效（证原始不可达是属性缺失而非探针失效）。
- 跨页正面对照：calendar.vue:295 `role=button` 但 tabindex=null → Tab 0（伪可达）；:296 `tabindex=0` → Tab 命中（完整可达），证方法能抓伪可达。
- 鼠标对照证非死代码：点 :93 → `#/pages/browse/browse`（changed=true）；点 :102 → toast「链接已复制」=true；点 :107 → 列表 3→2（removed=1，双主题）。
- errorBanner 态经导航往返（index→collect 二次 load）坐实 retry=1、错误文案「收藏加载失败：HTTP 500」，:100 非死代码。
- 空态 :93/:94 常驻呈现（emptyLinks=2）无需造数；collect 全页无 input/@confirm/@keydown 等价键盘入口（grep 仅命中 6 个 @tap）。
- 四份探针 `fatals=0`、`pageErrors=0`。

## 反预判与去伪

1. 同缺陷类已被 R544/R546/R548/R550/R552/R554/R556 覆盖不据此判 0：SC 维度≠实例，本页为新实例。
2. 不仅凭 `<view @tap>` 静态预判 P2，全部结论经真浏览器 CDP 坐实。
3. 宿主 wrapper（UNI-VIEW/AX generic focusable=false）不代判，只取 focused 非 root 节点。
4. 注入后非零 Tab 命中不混为原始可达，原始态以未注入探针为准。
5. goBrowse 语义：功能为导航跳转，语义宜 link；但本轮缺陷核心是键盘完全不可达且无 role，属 SC2.1.1 普适失效，最小修复按按钮口径补齐即可覆盖。
6. 已修 index.vue:159 论坛/mine 入口与 calendar:296 不重复立案；本页无原生 button 混入需排除。

## 最小修复方向（仅写入文档，未实施）

逐页显式补 `role=button`+`tabindex=0`+动态 `aria-label`+`@keydown.enter/.space.prevent` 复用原 handler，@tap 不动，业务零改；:107 保留 `@tap.stop`。

## 证据 SHA-256（入库仅 sha，真值不入库）

| 文件 | sha256 |
| --- | --- |
| r557-report.json | `b8381f2afb5099e6efe0905c6a3493217430efc1c0232b98c5dc7fe22a6a3315` |
| r557-detail-report.json | `d3931a2f6953bbad5dd4ba2500c30601a75f571f90ef9d88c47a753370cf008e` |
| r557-tab-report.json | `f80a1df35980adfb478895a0f0763e28509d5e77b13aa1faba34aefafc186748` |
| r557-del-report.json | `d6d53e3dabb56264a52e565bf89d2d706e12b60e19ab7480a6e83c9c0cb579f9` |
| withData 截图(light=dark) | `5c6ffc2f441c6ca7426cb031bdc9a889c4097d0e91f0621868222915abd73550` |
| error 截图(light=dark) | `fa7b7c9f7232320c7f9a279192bfc8537e6b4c74c47449f2927b7dd061de5a8e` |
| empty 截图(light=dark) | `6f3f54c04044982d4a62865c5685668b5c45594353731eb53d2a398a3bbc4fd9` |
| errorBanner 截图(light=dark) | `c93f5a864fdfad567def4292e4f9aa4f07210c72245483ce4d88dd796191acb5` |

证据原件位于 gitignored `oa4rust/test-results/r557-*`，运行 base 用 `<TEST_HOST>:<PORT>` 占位，0 真值入库。