# R541 H5 消息页伪按钮键盘可达性甄别（WCAG 2.1 SC 2.1.1 A 关联 SC 4.1.2 A）

- 轮次：R541（audit，t47），真父 `878cbc5a39193389c4c79dd635845998a0b5a20a`（台账 §531，已 push），attempt `c478a7fd-1234-44c7-95a4-2063bf3ea78d`。
- 性质：只读甄别，不改源码、不提交源码。属 R539/R540 已修工作台的**同缺陷类不同页面实例**（维度覆盖 ≠ 实例覆盖，先例 R509/R515/R532）。R540 Fail Loud 已声明全局增强器越界未采用，故其余页伪按钮为真实遗留。
- 环境：自起 uni H5（`http://127.0.0.1:<PORT>/oa4rust-h5/`，<TEST_HOST> 占位），Playwright chromium headless，390×844 DPR2；认证/会话列表数据以 stub 注入，0 真值。H5 App.vue 无主题机制，light/dark 截图 sha 相同。
- 取证口径：`DOM.querySelectorAll` + `DOM.getAttributes` + `Accessibility.getPartialAXTree` + `Accessibility.getFullAXTree`（按 backendDOMNodeId 映射比对 partial↔full），`document.activeElement` 逐 Tab 采样；**不探宿主 wrapper**（R535 教训），注入对照证明 AX 通道有效。

## 一、结论（三分判定）

**可达真缺陷 1 项 P2**：message 页打开会话列表项（`:102`）与错误/空态控件（`:89`/`:95`/`:100`）为 `<view @tap>` 伪按钮，DOM 无 `role`/`tabindex`/`aria-label`，CDP AX `role=generic name=null`，键盘 Tab20 次无一命中——键盘用户无法打开任何会话、也无法重试/刷新，读屏用户无法识别为按钮。归一族（SC 2.1.1 Keyboard A 关联 SC 4.1.2 Name Role Value A）。

- 非 L1（不可达）：stub 造数坐实列表态 2 条会话真实可达、空态与错误态均为真实可触发状态（非死代码、非空态伪装）。
- 非正常：注入 `role=button tabindex=0 aria-label` 后同节点 AX 变 `button`/`focusable=true`/name=注入值并稳定进 Tab10 次，证明是真实缺陷而非探针假阳性；同工程 `calendar.vue:295` 正面对照按钮 AX 名可读。
- 0 缺陷据实判 0、有缺陷据实判缺陷，未臆造。

## 二、源码锚点（逐行核实）

| 文件:行 | 代码 | 判定 |
|---|---|---|
| `message.vue:102` | `<view v-for="(row, i) in rows" :key="i" class="item o2-list-item" @tap="openConversation(row)">` | 打开会话唯一入口，伪按钮，缺陷 |
| `message.vue:89` | `<view class="retry-btn" @tap="load">重试</view>`（错误态 tip） | 伪按钮，缺陷 |
| `message.vue:100` | `<view class="retry-btn" @tap="load">重试</view>`（列表内 error-banner） | 伪按钮，缺陷 |
| `message.vue:95` | `<view class="empty-guide-link" @tap="load">刷新会话</view>`（空态） | 伪按钮，缺陷 |
| `message.vue:73-78` | `openConversation(row)` → `uni.navigateTo({ url: /pages/chat/chat?id=... })` | 业务 handler，键盘层无入口 |
| `message.vue:82-107` | 模板：无搜索框，grep `搜索|search|keyword|filter|input` = 0 命中 | 无等价键盘打开会话入口 |
| `calendar.vue:295` | `role=button aria-label=新建日历` | 正面对照，AX 名可读 |
| `main.ts:16-82` | `installH5NativeInputNameA11y` 仅表单名增强，无伪按钮补救 | 全局无兜底 |

## 三、决定性 CDP 证据（6 组 = light/dark × 列表/空态/错误，fatals=0 pageErrors=0）

| 状态 | 宿主 class | DOM role/tabindex/aria | AX partial.role / full.role | AX name | Tab20 命中 | 注入后 |
|---|---|---|---|---|---|---|
| 列表(2 条) | `item o2-list-item` | null / null / null | generic / generic | null | 0（全 BODY） | button / focusable=true / R541-injected-control，进 Tab10 次 |
| 空态 | `empty-guide-link` | null / null / null | generic / generic | null | 0（全 BODY） | 同上 |
| 错误态 | `retry-btn` | null / null / null | generic / generic | null | 0（全 BODY） | 同上 |

- partial↔full 的 backendDOMNodeId 映射一致（`fullMatched=true`），排除单一树假象。
- `focusedFull=[]`：getFullAXTree 无 focused 非 root 节点，与 Tab 全 BODY 一致——键盘焦点从未进入这三类控件。
- 注入对照：运行时 `setAttribute(role=button/tabindex=0/aria-label)` 后，同节点 partial/full AX 均 `role=button name=R541-injected-control props=[focusable=true]`，证明 AX 通道正常、原缺陷真实。
- 正面对照：`calendar.vue:295` 编译后 DOM `role=button aria-label=新建日历`，CDP 命中 `role=button`（可读名），证明页面级 AX 播报链路有效。
- 等价入口核查：message 页无搜索/过滤输入框（grep 0 命中），亦无其它键盘路径可打开会话或重试；tabBar 消息页入口仅为页面导航，不构成对页面内控件的豁免。

## 四、证据清单（sha256）

| 项 | 文件 | sha256 |
|---|---|---|
| 源码 | `mobile/src/pages/message/message.vue` | `1c2a09d421e6fa243215f5a4e7469cad12eb1da883cb1de671c868c068ac9153` |
| 源码 | `mobile/src/main.ts` | `b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b` |
| 源码 | `mobile/src/pages/calendar/calendar.vue`（正面对照） | `75c2c2bdbbb1f1e78b5259749e278158164105fcb8512f94208f2977983d4838` |
| 源码 | `mobile/src/pages.json`（tabBar 核查） | `fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466` |
| 报告 | `test-results/r541/r541-report.json` | `d7b79007ca6559ea452df78a633ff6b950c3e39e071a63dae61bf2386ad42f6c` |
| 截图 | `r541-withData-light.png` / `r541-withData-dark.png` | `cdbc5d083a003f87ea837b78954d9bab79a8ada0bfc25315960d3aa25bd88db5` |
| 截图 | `r541-empty-light.png` / `r541-empty-dark.png` | `adab0d203a0784c8a818a88775e3d835ff66c55fb789cfa505338ca6cd30262c` |
| 截图 | `r541-error-light.png` / `r541-error-dark.png` | `c62594187c814ee77196a147a6593fcbbb1d9c79579264aec3cb23418fabe485` |
| 探针 | `test-results/r541-e2e.cjs` / `r541-parse2.cjs` | gitignored，运行时脚本 |

证据全部位于 gitignored 的 `oa4rust/test-results/r541-*`（`.gitignore:28`），未入库。

## 五、覆盖甄别与最小修复方向

- 与 R534-R538（表单控件可访问名）为不同维度：本轮为非表单点击元素键盘可达性 + 语义角色。
- 与 R539/R540（工作台 index.vue）为同缺陷类不同页面实例，符合"维度覆盖不等于实例覆盖"先例，据实立案。
- 去伪：不探 UNI-VIEW 宿主 wrapper（generic 假阳性），以 DOM 原生节点 + AX partial/full + Tab 实测为准；注入对照排除"AX 通道失效"辩解；stub 造数排除空态/L1 辩解。
- 最小修复方向（供后续 repair 任务，**本轮不改**）：对 `:102`/`:89`/`:95`/`:100` 四个 `<view @tap>` 追加 `role="button" tabindex="0" :aria-label`（会话项用 `conversationTitle(row)` 动态名），并补 `@keydown.enter`/`@keydown.space.prevent` 复用 `openConversation`/`load`；业务 handler 零改、原 `@tap` 鼠标行为保留。与 R540 逐页显式补口径一致（全局增强器越界已 Fail Loud 排除）。

（文档脱敏：真域名/IP/凭据/个人路径均未入库，统一用 <TEST_HOST> / <PORT> 占位。）