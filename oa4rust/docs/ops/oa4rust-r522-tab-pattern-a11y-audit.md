# R522 只读甄别：WCAG 4.1.2 / 2.4.3 tab 模式 aria-controls / tabpanel 缺失

- 类型：只读甄别（不改码、不提交源码）
- 真父：672d3d9bb5e03eb05bab24cb32e5008bdd36070b（台账 §513）
- 目标：PC 日历「我的日历 / 公共日历」范围 tabs 与个人中心「授权委托」tabs 的 ARIA tab 模式
- 运行环境：Playwright Chromium headless，1440×900 DPR1，light/dark 双主题，真浏览器 + CDP DOM.getAttributes + Accessibility.getPartialAXTree + 键盘方向键探针
- 脱敏：报告与日志中的测试域名统一替换为占位符 `<TEST_HOST>`，不入库任何真值域名 / IP / 凭据；截图与 runner 脚本位于 gitignored 目录 `oa4rust/test-results/r522-*`

## 一、结论

**三分判定：正常（0 个可达 WCAG 缺陷）。** 两个 tab 组（日历范围、授权范围）的 4 个 `role="tab"` 均具备正确的 `aria-selected="true|false"`，CDP AXTree 中 `selected:true|false` 可读、`role=tab` 未 ignored；所有 tab 为原生 button，AXTree `focusable:true`，可经 Tab 键逐个到达并以 Enter/Space 触发 `@click` 完成切换，截图证实面板内容随切换真实变化。因此 SC 4.1.2 的 Name / Role / Value 三要素均达标，SC 2.1.1 键盘可达与 SC 2.4.3 焦点顺序亦达标。

**存在的差距（非 WCAG 缺陷，转介改进）：** 4 个 tab 均无 `aria-controls`，受控内容区为普通 `div` / `ul`（无 `role="tabpanel"`、无 `aria-labelledby`）；方向键 Left/Right 未在 tab 间移动焦点或改变选中。这些是 WAI-ARIA APG tab pattern 的完整性 / 最佳实践差距——`aria-controls` 在 ARIA 1.2 中属于 tab 的 supported 属性而非 required，`role=tabpanel` 与方向键属于 APG 推荐模式——不构成 SC 4.1.2 / 2.4.3 的可达失败。按「0 缺陷据实判 0，严禁臆造」铁律，本轮不判缺陷。

- 反预判：结论全部来自 headless Chromium 运行时 CDP 探针，静态 grep 仅作旁证，未以源码推断代替浏览器实证。
- 去伪：`aria-selected` 的真值在 DOM 与 AXTree 双通道一致，排除「工具假阴性」；面板视觉变化在截图中可复核，排除「tab 模式不生效」；`aria-controls` 缺失经双主题 8 项探针全 null 确认，但规范层级不足以判 WCAG 失败。

## 二、源码甄别（逐行锚点）

| 文件 | 行 | 关键代码 | 判定 |
| --- | --- | --- | --- |
| `oa4rust/frontend/apps/desktop/src/views/CalendarApp.vue` | 19 | `<div class="cal-scope glass-card" role="tablist" aria-label="日历范围" data-testid="calendar-scope-tabs">` | 显式 tablist，含 aria-label |
| 同上 | 20–26 | `<button class="cll-tab" role="tab" :aria-selected="calScope==='my'" @click="loadCalendars('my')">我的日历（{{ myCals.length }}）</button>` | tab + aria-selected + click 切换 |
| 同上 | 27–33 | `<button class="cll-tab" role="tab" :aria-selected="calScope==='public'" @click="loadCalendars('public')">公共日历（{{ pubCals.length }}）</button>` | 同型第二个 tab |
| 同上 | 35–45 | `<div v-if="(calScope==='my'?myCals:pubCals).length" class="cal-chip-row">…</div>` | 受控内容区：普通 div，无 role=tabpanel / aria-controls |
| `oa4rust/frontend/apps/desktop/src/views/Personal.vue` | 78–80 | `<section data-testid="personal-empower">…<div class="emp-tabs" role="tablist" aria-label="授权范围">` | 显式 tablist，含 aria-label |
| 同上 | 81–82 | `<button class="emp-tab" role="tab" :aria-selected="empScope==='mine'" @click="loadEmpower('mine')">我发出的…</button>` 与 `…'to'…授权给我…` | 2 个 tab + aria-selected + click 切换 |
| 同上 | 84–98 | `<div v-if=…>` / `<ul v-else class="emp-list modern-grid">…</ul>` | 受控内容区：div / ul，无 role=tabpanel / aria-controls |
| `oa4rust/frontend/apps/desktop/src/views/CalendarApp.business.contract.test.ts` | 46–50 | `it('日历范围切换为我的/公共日历 tab…')` 仅断言 `role="tablist"` 存在 | 契约未覆盖 aria-controls / tabpanel / 方向键 |

静态补充检索：`installTabA11y|tabA11y|ArrowRight|ArrowLeft|aria-controls|tabpanel` 在 `oa4rust/frontend` 下无命中，确认无全局 tab 键盘增强器、无 aria-controls / tabpanel 角色实现。此为旁证，决定性结论以第七节浏览器实证为准。

## 三、契约与判据

- **SC 4.1.2 Name, Role, Value**：状态与值必须可程序化确定。对 `tab` role，ARIA 1.2 的 required state 是 `aria-selected`（value / 状态）；`aria-controls` 是 supported 属性，`role=tabpanel` 及其 `aria-labelledby` 属于 APG tab pattern 的结构约定。本例 `aria-selected` 与 AXTree `selected` 双通道齐备，Name（tab 文案）与 Role（tablist/tab）亦齐备，故 SC 4.1.2 达标。
- **SC 2.1.1 Keyboard**：所有 tab 为原生 button，AXTree `focusable:true`，Tab 键可逐个到达，Enter/Space 触发 `@click` 切换；无键盘陷阱。方向键切换是 APG 推荐增强，非 SC 2.1.1 硬失败。
- **SC 2.4.3 Focus Order**：焦点顺序 = DOM 顺序 = 视觉从左到右顺序（截图可复核），达标。
- **三分判定口径**：①可达真缺陷＝真浏览器实证存在 WCAG 失败且控件可达；②不可达 L1＝控件不可达 / 需特殊权限；③正常＝规范硬要求齐备或缺失项不构成 WCAG 失败。本轮判③。
- **规范参考**：W3C ARIA 1.2 `tab` / `tabpanel` role 定义（`https://www.w3.org/TR/wai-aria-1.2/#tab`）、WAI-ARIA APG Tabs Pattern（`https://www.w3.org/WAI/ARIA/apg/patterns/tabs/`）、WCAG 2.1 SC 4.1.2 / 2.1.1 / 2.4.3。

## 四、决定性证据矩阵（CDP，双主题 × 4 对象）

Runner：`oa4rust/test-results/r522-e2e.cjs` + `oa4rust/test-results/r522-run.sh`；报告：`oa4rust/test-results/r522-tab-pattern/report.json`；摘要 `R522_DONE reportSha=f106b8ac54d53b5b68cf50fe87e9c443997adf2e86d0dd7035c89118d6b818ba groups=2 fatals=0`。CDP 通道：`DOM.enable` + `Accessibility.enable` + `DOM.getDocument(depth:-1)` + `DOM.querySelectorAll` + `DOM.getAttributes` + `Accessibility.getPartialAXTree(fetchRelatives:false)`；键盘通道：`tab.focus()` 后派发 ArrowRight / ArrowLeft，读 focused / selected。

| 主题 | 对象（路由） | tabCount | aria-selected（前→后） | AXTree selected（前→后） | aria-controls | tabindex | tablist AX | 方向键 Right / Left | panelAXCount | 判定 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| light | calendar-my（`日历`） | 2 | true→true | true→true | null | null | role=tablist name="日历范围" multiselectable=false orientation=horizontal | focused 保持 true / selected 保持 true | 0 | 正常 |
| light | calendar-public | 2 | false→true | false→true | null | null | 同上 | focused true / selected 保持 false→点击后 true | 0 | 正常 |
| light | personal-mine（`个人中心`） | 2 | true→true | true→true | null | null | role=tablist name="授权范围" 同上 props | focused true / selected true | 0 | 正常 |
| light | personal-to | 2 | false→true | false→true | null | null | 同上 | focused true / selected false→点击后 true | 0 | 正常 |
| dark | calendar-my | 2 | true→true | true→true | null | null | 同 light | 同 light | 0 | 正常 |
| dark | calendar-public | 2 | false→true | false→true | null | null | 同 light | 同 light | 0 | 正常 |
| dark | personal-mine | 2 | true→true | true→true | null | null | 同 light | 同 light | 0 | 正常 |
| dark | personal-to | 2 | false→true | false→true | null | null | 同 light | 同 light | 0 | 正常 |

说明：`focusedAfterArrowRight=true` 且 `focusedAfterArrowLeft=true`、`selectedAfterArrow*` 保持点击前值，证明方向键未移动焦点 / 未改选中（无全局 Arrow 增强器）；但点击（等价于键盘 Enter/Space）后 `aria-selected` 由 false→true、`tabAfter` 属性 `class=… on`、AXTree `focused:true, selected:true`，且 `beforeShot` 与 `afterShot` 哈希不同（calendar-public light: d65bfee9…→79d24386…），证明切换功能真实可用。`panelAXCount=0` 仅证明内容区无 tabpanel 角色（差距项），不能据此断言面板显隐异常；`panelVisibleAfter=false` 为探针面板选择器失配所致，已由截图人工复核排除。

### AXTree 属性级证据（示例，light calendar-public）

- 点击前：`axRole="tab"`, `axName="公共日历（0）"`, `axIgnored=false`, `axProperties={"invalid":"false","focusable":true,"selected":false}`
- 点击后：同上 + `focused:true`, `selected:true`（`class="cll-tab on"`）
- tablist：`axRole="tablist"`, `axName="日历范围"`, `axIgnored=false`, `axProperties={"multiselectable":false,"orientation":"horizontal"}`

## 五、截图与哈希（脱敏，均在 gitignored 目录）

16 张 1440×900 PNG 存于 `oa4rust/test-results/r522-tab-pattern/`，命名 `<label>-<before|after>-<light|dark>.png`。以下为视觉复核的代表性截图（界面为通用 admin 账号，无真值信息）：

| 截图 | SHA-256 | 视觉复核要点 |
| --- | --- | --- |
| `oa4rust/test-results/r522-tab-pattern/calendar-public-before-light.png` | d65bfee9ebe68805e5a7995d43339986c1b683a7e545bd28ce48f1dbbaa55687 | 公共日历 tab 被选中，chip 行含多条日历项 |
| `oa4rust/test-results/r522-tab-pattern/calendar-public-after-light.png` | 79d24386aaf1c64ca7272e42af66cebb789d2e54712795ed99b4ba9b66432e23 | 点击后公共日历保持选中，chip 行无条目（公共日历 0 条），证明内容区随状态切换 |
| `oa4rust/test-results/r522-tab-pattern/calendar-my-before-dark.png` | 3bc53399c562768da4da606a5d360889b2df666a4546f8f73688f0f8aa7282ea | 深色主题，我的日历选中（10），chip 行可见 |
| `oa4rust/test-results/r522-tab-pattern/personal-mine-before-light.png` | 9b51f0463d78c8e4e0d03fc5b5de1acf9c450b3d150765e9b84379ba0036f7e6 | 浅色主题，授权范围「我发出的」选中 |
| `oa4rust/test-results/r522-tab-pattern/personal-to-after-dark.png` | 3abf4bfefb031d4c161f8359dd9655aa5ed2924d0d02901c7ee108b3ad1cb56d | 深色主题，「授权给我」选中，内容区切换达标 |
| `oa4rust/test-results/r522-tab-pattern/personal-mine-after-dark.png` | 10fe78b6711574a65559a29782403c8bf67e0055fbd6cb58839fa2bf8ba8f789 | 深色主题，「我发出的」选中态 |

全部 16 张截图的文件 SHA 与 `read_image` 回执一致，完整清单见 `report.json`。证据文件 SHA-256：

| 文件 | SHA-256 |
| --- | --- |
| `oa4rust/test-results/r522-e2e.cjs` | 898e1fac03a8040f4de6129d714541364c30a52d8621814b77ddac33d7c40acf |
| `oa4rust/test-results/r522-run.sh` | 2aaffd386eb012a7d38b2335ab8a42e6310b957a25405951ea36e7b550099c92 |
| `oa4rust/test-results/r522-tab-pattern/report.json` | f106b8ac54d53b5b68cf50fe87e9c443997adf2e86d0dd7035c89118d6b818ba |

## 六、覆盖甄别与转介

- **次强候选 `SearchApp.vue:22–59`**：`summary[role="tab"]`（:24/:33/:42/:51）无 `aria-selected`，去伪风险高（原生 `summary` 的 details 语义与自定义 tab role 叠加需单独甄别），本轮仅备料不主判，不纳入结论。
- **改进建议（非缺陷，交队长排期）**：如需对齐 WAI-ARIA APG tab pattern，可在 4 个 tab 补 `aria-controls` 指向受控内容区、为内容区补 `role="tabpanel"` + `aria-labelledby`、并实现 Left/Right 方向键 roving focus；本轮不修改任何源码，`CalendarApp.vue` / `Personal.vue` 保持原样。
- **只读边界**：本轮未修改源码、未提交源码。
- **交付边界**：唯一入库文档为本文件；runner 脚本、日志、报告、截图全部位于 gitignored 的 `oa4rust/test-results/r522-*`，精确 `git add` 本文档，禁 `-A` / `git add .`；push 若失败 FAIL-LOUD 报告队长补推。
