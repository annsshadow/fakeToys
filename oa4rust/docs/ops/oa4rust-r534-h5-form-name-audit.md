# OA4Rust R534 H5 表单控件可访问名只读甄别报告

- 轮次：R534（台账 §525），真父 `1a1c1abdada121efeb1512b3b823d7d4c995579b`，基线已 push。
- 性质：**只读甄别**，不改源码、不修业务；证据以 `<TEST_HOST>` 占位域名脱敏，0 真值入库。
- 范围：`oa4rust/mobile/`（uni-app H5），未专项甄别过；`src/main.ts:9-27` 的 `createApp()` 仅 errorHandler / unhandledrejection / pinia / setApiBase，**无任何 installA11y 增强器**，与 desktop 侧装 loadingA11y / formErrorA11y / labelA11y 等完全不同，故 PC 侧已覆盖的 4.1.2 实例不覆盖 H5。
- 方法：本地 `pnpm dev:h5`（Vite 5174，hash 路由）+ Playwright Chromium headless 移动视口 390×844 DPR2，`Accessibility.getPartialAXTree` / `getFullAXTree` / `DOM.getAttributes` 取证；认证会话与列表数据 stub，控件为真实 H5 编译产物。

## 一、结论

**可达真缺陷 1 项**（按控件族与页面聚合，非按控件个数重复计）：WCAG 2.1 **SC 4.1.2 Name, Role, Value（Level A）** —— H5 日历与论坛发帖表单的 input / textarea / picker 在 AXTree 中 role 存在（textbox / generic）但 **name 全部为 null**；placeholder 与兄弟 `<text class="label">` 均未被 uni-app H5 WebView 作为可访问名。关联 **SC 3.3.2 Labels or Instructions（Level A）**：标签文本与控件仅 DOM 兄弟关系，无 `for/id` 或 `aria-label` / `aria-labelledby` 编程关联。

反预判铁律已实证：同页 `role="button" aria-label="新建日历"` 的 UNI-TEXT 在 partial 与 full AXTree 中 name 均为「新建日历」，证明 AX 通道正常、aria-label 可进可访问名；而各表单控件 name=null —— **不是「AX 读不到」的环境假象，而是真实缺名**。若 placeholder 被当作可访问名则 4.1.2 不成立，实测证伪。

## 二、源码甄别（逐行锚点）

| 锚点 | 源码事实 |
|---|---|
| `oa4rust/mobile/src/pages/calendar/calendar.vue:295` | `<text class="calendar-create-btn" role="button" aria-label="新建日历" @tap="toggleCalendarForm">+新建日历</text>` —— 同页正面对照，有 aria-label |
| `calendar.vue:301` | 新建日历表单 `<input v-model="newCalendarName" class="c-input" placeholder="日历名称" />`，包在 `v-if="showCalendarForm"`（:300）内，无 label / aria-label |
| `calendar.vue:306-312` | `<picker :range="calendars" range-key="name">` 内含 `<view class="picker">日历：…</view>`，无编程名 |
| `calendar.vue:313-316` | 事件表单 4 个 `<input>` 仅 placeholder：事件标题 / 开始 / 结束 / 地点，包在 `v-if="composing"`（:305）内 |
| `calendar.vue:68-70 / 94-99` | `toggleCalendarForm` / `toggleCompose` 翻转 ref 展开表单（可达，非死代码） |
| `oa4rust/mobile/src/pages/bbs/new.vue:82-95` | 三个 `.field` 内 `<text class="label">版块/标题/内容</text>` 与 `picker` / `input placeholder=请输入标题` / `textarea placeholder=请输入内容` 仅兄弟，无 for/id / aria-* 关联 |
| `bbs/new.vue:69-75 / 22-33` | `onLoad` 经 ensureAuthenticated + `forumList` 拉版块数据，表单真实可达 |
| `oa4rust/mobile/src/main.ts:9-27` | createApp 无 installA11y，H5 侧无全局无障碍增强器 |

源码 SHA-256：

- `calendar.vue` = `6a499c78d68e4b5d0151990175c05723cc14c4067d19098c1d96ed2a905ee921`
- `bbs/new.vue` = `b8050e58cb08b5dfcb39cf6f79688491f7983757c5725015d2a1a10d03eb74b0`
- `main.ts` = `508492d10106af605f50624a6c1cbe4db67f0bd610f7209f449e0a7e8fcd7daa`

## 三、决定性 CDP 证据（双次采集 fatals=0 / pageErrors=0）

| 探针 | 结果（light 与 dark 两次一致） |
|---|---|
| calendar 新建日历按钮 | `.calendar-create-btn`（UNI-TEXT）AX partial/full 均 `role=button name="新建日历" ignored=false`；DOM 带 `role=button aria-label=新建日历` |
| calendar 日历名称 input（:301） | `.calendar-create-form uni-input input`（INPUT）AX partial/full 均 `role=textbox name=null ignored=false focusable=true editable=plaintext`；DOM `class=uni-input-input`，无 aria-label / id |
| calendar picker（:306-312） | `.compose .picker` 为 UNI-VIEW `role=generic name=null`；`picker` 宿主为 UNI-PICKER（内部 div 结构），无可访问名 |
| calendar 事件 4 input（:313-316） | 4 个 INPUT 全部 `role=textbox name=null focusable=true`，与名称框同型 |
| bbs 版块 picker（:83） | `.field picker` = UNI-PICKER，`.picker-value` = UNI-VIEW `role=generic name=null`；兄弟 `<text class="label">版块</text>` = UNI-TEXT `role=generic name=null` |
| bbs 标题 input（:91） | `.field input`（INPUT）AX `role=textbox name=null focusable=true editable=plaintext`；无 aria-label / for 关联 |
| bbs 内容 textarea（:95） | TEXTAREA AX `role=textbox name=null focusable=true multiline=true` |
| 键盘可达性（补充探针） | calendar Tab 序列中 `uni-input-input` 反复出现（可聚焦），bbs 中 INPUT/TEXTAREA 交替可聚焦；但聚焦控件 AX name 始终 null；控件可达≠可识别 |
| 表单展开实证 | 点击 :295「+新建日历」→ `.calendar-create-form` 出现（formOpened=true）；点击 :296「+新建」→ `.compose` 出现（composeOpened=true）；bbs/new `.form-card` 存在、fields=3 |

## 四、通过项与取证限制

- 正面对照成立：H5 自身已有 aria-label 可进 AX 的样例（:295 按钮），证明缺陷是控件缺名而非 AX 采集失效。
- 主题说明：`oa4rust/mobile/src/App.vue` 未实现 `oa4rust_theme` / `data-theme` 切换机制，light/dark 两次采集渲染一致（截图 sha 相同），据实记录，不伪造暗色差异；AX 结论与主题无关。
- stub 说明：认证与 who/forum/calendar 列表用 stub 会话（避免真值入仓），仅影响数据源；控件 DOM 与可访问名为真实 Chromium 编译渲染结果，不影响判定。空态已排除（表单成功展开、列表有 stub 数据）。
- 取证限制：`getFullAXTree.properties` 在 Chromium 不带 `current` 等属性，本报告以 AX `role/name/ignored` 与 DOM 属性为决定性依据，不夸大未observable项。

## 五、三分判定

- **可达真缺陷（P2，SC 4.1.2 A / 关联 3.3.2 A）**：上述 1 项控件族缺名，真浏览器可达、非空态非死代码，AX name=null 已坐实。
- **不可达 L1**：排除——表单经点击展开、bbs 经认证+数据可达，均实际渲染。
- **正常**：排除——placeholder 与兄弟 label 未能提供可访问名（AX 实证），同页 aria-label 按钮反而可读，不能判正常。

## 六、最小修复方向（业务零改，仅补可访问名）

- calendar.vue:301 / :313-316 五个 input、:306 picker；bbs/new.vue:83 picker / :91 input / :95 textarea：加 `aria-label`（如「日历名称」「事件标题」「开始时间」「结束时间」「地点」「版块」「标题」「内容」）；或为 `<text class="label">` 与控件建立 `id` / `aria-labelledby` 关联。
- 提示：placeholder 作标签随输入消失、且多数读屏不将其计入可访问名，应显式补名；业务函数、v-model、模板结构与样式零改。

## 七、覆盖甄别

- 已覆盖维度区分：本项聚焦 H5 4.1.2/3.3.2 控件命名，与 PC 侧已覆盖的 4.1.2 实例、R523 链接目的、R524 面包屑 current、R526 嵌套交互为不同组件/不同维度，不重复计。
- H5 其余表单页未在本次范围（本轮仅 calendar / bbs-new 最强与次强候选），不臆造全域结论。

## 八、证据清单（SHA-256，证据入 gitignored `oa4rust/test-results/r534/`）

| 文件 | SHA-256 |
|---|---|
| `r534/r534-ax-report.json` | `bf29946f972e3bf48501a618368fcf9005cf9bbaa171a101a24607ad4bae5d34` |
| `r534/r534-supp-report.json`（Tab/直接 input AX） | `8ce0d28184e5852a2a27c4cc8c79a3ab2924e4544cdb025bc84a7e934c140086` |
| `r534/r534-calendar-light.png`（780×1687 / 112340B） | `9417379476c52cccd5e21cd61c554cb63991be4096ca59afaf7e7dafc23c47af` |
| `r534/r534-calendar-dark.png`（同上，H5 无主题切换） | `9417379476c52cccd5e21cd61c554cb63991be4096ca59afaf7e7dafc23c47af` |
| `r534/r534-bbs-light.png`（780×1687 / 112027B） | `f03dee35dddd696f7bedbba3fb349f4988b4ef91431db8e76cb02d5a6f8c66f0` |
| `r534/r534-bbs-dark.png`（同上） | `f03dee35dddd696f7bedbba3fb349f4988b4ef91431db8e76cb02d5a6f8c66f0` |

判定口径：可达真缺陷 1 项 P2；反预判铁律以 CDP AX name=null + 同页 aria-label 按钮可读双向实证；0 真值入库，域名占位 `<TEST_HOST>`。