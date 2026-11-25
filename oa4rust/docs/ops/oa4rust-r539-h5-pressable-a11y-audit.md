# R539 H5 工作台交互伪按钮键盘可达性与语义角色只读甄别

- 甄别轮次：R539（台账 §529）
- 真父提交：`b068aa786ef84b706a883fbca26c9ef69c53db5c`
- 任务 / 尝试：`t43` / `630627c6-2370-4621-9ab9-1240ef5ee26c`
- 性质：只读甄别，不改源码、不提交源码。
- 测试基线：自起 uni-app H5 开发服务 + Playwright Chromium headless，移动视口 390×844 DPR2，双主题采样；认证态、待办、公告列表均以占位 stub 注入（0 真值）。
- 访问域名：`<TEST_HOST>`（占位，真值不入库、不回显）。

## 一、结论

判定：**可达真缺陷 1 项 P2** —— WCAG 2.1 SC 2.1.1 Keyboard（A），关联 SC 4.1.2 Name, Role, Value（A）。

工作台页（`oa4rust/mobile/src/pages/index/index.vue`）的 2 个统计卡、1 条公告项、12 个宫格入口、空态「刷新公告」及公告详情关闭控件，均为 `<view @tap>` 伪按钮：CDP 实测 AX `role=generic`、`name=null`、无 `role`/`tabindex`，30 次 Tab 序列无一命中。键盘用户无法激活这些功能，读屏用户无法将其识别为可交互按钮。

决定性去伪：运行时给宫格注入 `role=button + tabindex=0 + aria-label` 后，同一节点 partial/full AX 均变为 `role=button / focusable=true / name=注入值`，并稳定进入 Tab 序列（命中 15 次）——证明 AX 通道正常、探针方法正确、缺陷真实、修复方向有效。工程内正面对照 `calendar.vue:295` 的 `calendar-create-btn` 原生带 `role=button + aria-label`，AX `name=新建日历`。

## 二、源码甄别（只读锚点）

- `index.vue:92 / :99`：`<view class="stat-card ai-brand-fill ai-pressable" @tap="goTab(...)">`（待办审批、今日出勤），仅 `@tap`，无 `role`/`tabindex`/`@keydown`。
- `index.vue:108 / :113`：`<view class="retry-btn" @tap="loadNotices(true)">`、`<view class="empty-guide-link ai-pressable" @tap="loadNotices(true)">`（公告错误/空态刷新）。
- `index.vue:119`：`<view class="notice o2-list-item" @tap="viewNotice = n">`（公告项，点击开全文）。
- `index.vue:126–171`：12 个 `<view class="cell ai-pressable" @tap="goTab/goPage(...)">`（消息/审批/文档/通讯录/考勤/发起流程/会议/日程/论坛/浏览/搜索/回收站）。
- `index.vue:178–183`：公告全文遮罩 `@tap="viewNotice = null"`、内层 `@tap.stop`、关闭 `<button class="nd-close">`（原生 button，单列观察，不混入伪按钮主判）。
- `index.vue:77–82`：`goTab(uni.switchTab) / goPage(uni.navigateTo)`，仅触摸/鼠标 tap 触发。
- `pages.json:23–34`：tabBar 仅 5 项（工作台/消息/审批/考勤/我的），为等价入口核查依据。
- 正面对照 `calendar.vue:295`：`<button role="button" aria-label="新建日历" …>`（工程现成合规写法）。

源码 sha256：index.vue = `1bf068c55695c469339a8bafe1a92ec4e6a90d9b1ae5880472614d73039afb68`；pages.json = `fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466`；calendar.vue = `75c2c2bdbbb1f1e78b5259749e278158164105fcb8512f94208f2977983d4838`。

## 三、验收契约

1. 真浏览器 headless 移动视口，CDP `getPartialAXTree` / `getFullAXTree` 查 AX 语义，`Tab` 验证 `document.activeElement`；严禁探宿主 wrapper 假阳性（R535 教训：wrapper `role=generic focusable=false` 不能代判）。
2. 逐项核查等价键盘入口（R492/R493 铁律）；tabBar 可达项须单独甄别。
3. 先确认运行时可达，死代码不报（R525 教训）；公告详情需列表非空，空态据实标注。
4. 注入对照证 AX 通道有效；正面对照 calendar 按钮。
5. 三分判定：可达真缺陷 / 不可达 L1 / 正常，逐项给 DOM/AX/键盘/截图实证，0 缺陷据实判 0。

## 四、决定性 CDP 证据矩阵（light / dark 双主题一致，fatals=0，pageErrors=0）

| 候选 | DOM 属性（role/tabindex/aria-label） | partial AX | full AX | 30×Tab 命中 |
| --- | --- | --- | --- | --- |
| 待办统计卡 :92 | null / null / null | generic, name=null | generic, name=null | 否 |
| 出勤统计卡 :99 | null / null / null | generic, name=null | generic, name=null | 否 |
| 公告项 :119 | null / null / null | generic, name=null | generic, name=null | 否 |
| 12 宫格 :126–171（逐项） | null / null / null | generic, name=null | generic, name=null | 否 |
| 空态刷新 :113（公告空态时） | null / null / null | generic, name=null | generic, name=null | 否 |
| 公告详情关闭 :183（原生 button 包裹层，观察项） | null / null / null | generic, name=null | generic, name=null | 否 |
| **注入对照**（宫格 :134 运行时注入） | button / 0 / R539-injected-control | **button, focusable=true, name=注入值** | **button, focusable=true, name=注入值** | **是（15 次）** |
| **正面对照** calendar.vue:295 | button / — / 新建日历 | button, name=新建日历 | button, name=新建日历 | — |

注入前 `tabSeqHit=[]`、注入后 `injectTabHit` 命中 15 次且 `document.activeElement` 落在 `UNI-VIEW.cell[role=button]`；证明缺陷与探针正确性均被双向坐实。

## 五、去伪与等价入口核查

- **tabBar 等价入口（R492/R493）**：pages.json 中消息、审批、考勤 3 项在 tabBar 有同名入口，但 CDP 实测 tabBar `roleTabindexNodes=[]`（无任何 role/tabindex 节点），Tab 序列 18 次 `document.activeElement` 全为 `BODY` —— 键盘层同样不可达，故这 3 项不构成豁免；其余 9 个宫格（文档/通讯录/发起流程/会议/日程/论坛/浏览/搜索/回收站）、2 个统计卡、公告项与公告详情关闭控件无任何等价键盘入口，零替代。
- **公告详情可达性**：stub 注入非空公告后 `noticeMask=true`，详情弹层真实呈现；空态另测 `empty-guide-link` 可见且同样 `role=generic name=null`。两者均非死代码。
- **遮罩排除**：`:178 @tap="viewNotice = null"` 为遮罩点击关闭，按 R492 既定排除，不单列。
- **原生 button 边界**：`nd-close` 源码为原生 `<button>`，但 uni-app 编译后 `.nd-close button` 内层未命中、包裹层 AX `role=generic`，不加 tabindex 时 Tab 不达；此项作「原生语义被 uni-app 包裹削弱」的观察项，不计入伪按钮主判，避免夸大。
- **与已覆盖维度区分**：R534–R538 只审 H5 表单控件的可访问名；本轮为**非表单点击元素**的键盘可达性与语义角色，属新维度，不与前序重复计数。
- **4.1.3 状态消息（次强候选）**：因 SC2.1.1/4.1.2 主判已确证成立，按铁律不扩报；且 `uni.showToast` 走原生通道，去伪风险高，仅备案。

## 六、截图与证据 sha256（证据均在 gitignored 的 `oa4rust/test-results/r539/`）

- 报告 `r539-report.json` = `75ea7b9ae749ce1a1c46566b068b09c5d486818a619d4af083091277b277cae1`（双主题，fatals=0）。
- 补充报告 `r539-supplement.json` = `99ae25b6daf8457897a61fb549dc29b19acc06ea33632186161e667d890c67a7`（tabBar/空态/详情）。
- 截图 `r539-index-light.png` = `19cd92fdfcdc44bfa1c9bc2a0e91e462098b7ab896f433977a4b6e1d9ab0dd9c`；`r539-index-dark.png` 同值（H5 App.vue 无主题切换机制，与 R534–R536 一致），均 780×1688、280459 bytes。
- 截图目视确认：2 统计卡、公告列表、12 宫格、底部 tabBar 均真实渲染，非死代码。

## 七、三分判定

- **可达真缺陷（P2，1 项）**：SC2.1.1 Keyboard（A）关联 SC4.1.2 Name, Role, Value（A）—— index.vue 伪按钮族（2 统计卡 + 1 公告项 + 12 宫格 + 空态刷新 + 公告详情关闭），键盘不可达且无按钮语义/名称；注入对照与正面对照双向坐实。
- **不可达 L1**：无（全部候选运行时可见可达）。
- **正常**：正面对照 calendar.vue:295 按钮（role=button、可访问名齐备）；tabBar 鼠标/触摸入口存在（键盘层缺陷已并入主判，不重复计数）。

## 八、最小修复方向（业务零改，供后续实现参考）

给 `.ai-pressable / .cell / .stat-card / .notice / .empty-guide-link` 等 `<view @tap>` 补 `role=button + tabindex=0 + aria-label` 与 `@keydown.enter / @keydown.space` 触发等价 tap；或在 H5 侧加全局增强器统一注入（R537 已有 `installH5NativeInputNameA11y` 可类比），业务函数与模板结构零改。tabBar 键盘可达性属 uni-app 框架层共性，须单列评估，不在本页修复范围。
