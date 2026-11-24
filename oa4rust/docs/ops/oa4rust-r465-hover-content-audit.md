# R465 WCAG 1.4.13 Content on Hover or Focus 悬停/焦点内容可达性甄别（t324）

> 审计类别：只读甄别（不改任何入库源码/契约/台账）；唯一交付为本文档。
> 真父提交：204107f4d8dc2851436ab8e7519eb58b53c77bda（台账§460已 push）。
> 分支：opt1000/oa4rust-pc-ai-refactor；审计时 HEAD=UP=204107f4d，AB=0 0，工作树干净。

## 1. 审计目标与空白维度确认

WCAG 1.4.13 要求：由悬停或键盘焦点触发而出现的额外内容，必须同时满足三性——①Hoverable：鼠标可移动到浮层内容上而浮层不消失；②Dismissible：浮层可见时按 Esc 可关闭且不移动键盘焦点；③Persistent：只要悬停/焦点未移除，浮层持续显示，不自动消失打断阅读。

队长静态侦察据实：全库无专门针对 WCAG 1.4.13 的测试/契约；title 提示 174 处、tooltip/popover 关键词 34 命中，属真实空白维度。已覆盖维度严禁重复：prefers-reduced-motion、focus-visible、skipLink/routeAnnouncer/modalFocus/loadingA11y/labelA11y/formErrorA11y/nonTextContrast(1.4.11)/aria-live(4.1.3)/chartA11y(1.1.1)、触摸目标 44px、响应式 @media、骨架 R453/R456、标题层级、表格语义。对比度类 A11Y 已在 R458/R461/R462/R463/R464 覆盖且判 0 缺陷，本轮为悬停/焦点浮层独立维度。

三分判定口径：①规范预期（原生 title 由 UA 渲染，或自定义浮层已满足三性 → 非缺陷）；②真实退化（自定义浮层悬停即消失/无 Esc 关闭/中途消失致内容不可读 → 可达真缺陷并标 P 级）；③不可达 L1（死代码/需特定交互不可达）。仅 ② 且用户可感知才判可达真缺陷；0 缺陷据实判 0，严禁臆造。

## 2. 真浏览器实测方法

- 引擎：Playwright Chromium headless，非物理真机（NOT a physical device）。
- PC：1440x900 DPR1，双主题 light+dark，8 路由（dashboard/attendance/bam/three-member/org/file/bbs/meeting）× 2 视觉态 = 16 组。
- H5：390x844 DPR2 isMobile/hasTouch，锁浅色，5 路由（index/attendance/process/meeting/mine）。
- 采集：逐页扫 title 属性节点与自定义浮层候选（.tooltip/.popover/[role="tooltip"]/[role="dialog"]/.dropdown 等），统计可见节点。
- 四类深度探针：①nativeTitle——hover 带 title 的按钮，比对 role=tooltip DOM 节点数变化；②palette——打开命令面板，依次采 before/打开/persist500ms/hoverItem/Escape 序列，验三性+焦点归属；③userMenu——点击 admin 下拉，验点击展开/hover 保持/Esc/外部点击关闭；④processDesigner——进工作流设计器，验 pd-tooltip 与 showTooltip 是否可达。
- 登录与凭据：PC 用 input[placeholder*="用户名"] + getByRole(/登/)；H5 用 input[type=text]/[type=password] + uni-button filter(/登/)；凭据经 ssh 在测试服运行时提取并注入环境变量，域名真值仅存在于 gitignored runner，不入库。
- 测试服：<TEST_HOST>，systemctl active，http 回环端口 HTTP=200，入口构建产物与本地 dist 一致。
- 实测汇总：R465_DONE pc=16 h5=5 probes=4 errors=0，ERRORS=[]。

证据目录（全部 gitignored）：oa4rust/test-results/r465-hover-content/，含 24 张 PNG + report.json + analysis 文本，source 标注 Playwright Chromium headless 非物理真机。

## 3. title 提示与浮层分布统计

- 静态 title 命中 174 处；真浏览器 PC 16 组统计可见 title 节点合计 78 个（每组 3~52 个不等，BBS/日历/管理页按钮密集）。
- 自定义浮层候选（.tooltip/.popover/[role="tooltip"]）在所有 PC 16 组与 H5 5 组中 customVisible 全为 0——即常规浏览下页面不存在悬停即现的自定义 tooltip DOM 节点。
- H5 五页 title 节点与自定义浮层均为 0（移动端无 hover 交互，符合预期）。

## 4. 逐浮层 Hoverable + Dismissible + Persistent 证据表

| 浮层类型 | 触发方式 | Hoverable | Dismissible(Esc) | Persistent | 判定 |
|---|---|---|---|---|---|
| 原生 title 提示（174处，如 BBS 按钮/日历事件/管理页按钮） | 鼠标 hover，UA 渲染 | 由浏览器系统控制 | 由浏览器系统控制（原生 title 不可 Esc 关闭） | 由浏览器系统控制 | 规范预期（非项目缺陷） |
| 命令面板 快速导航（role=dialog，唯一自定义浮层） | 点击触发按钮或 Ctrl/Cmd+K | hoverItem 后 paletteStillOpen=true | Escape 后 paletteGone=true，焦点归还触发按钮 | persist500ms 后 paletteStillOpen=true | 规范预期（三性全满足） |
| 用户下拉菜单（个人中心/系统设置/退出登录） | 点击 admin 头像（非 hover/focus） | hover 项后 stillOpen=true | Esc 后 stillOpen=true（无 Esc 处理），外部点击 closed=true | 保持显示至外部点击 | 规范预期（1.4.13 不适用：点击菜单非 hover/focus 内容） |
| pd-tooltip（ProcessDesigner 节点提示） | 设计为鼠标移动触发 | 不可达 | 不可达 | 不可达 | 不可达 L1（死代码） |
| ECharts 图表 tooltip / CodeMirror cm-tooltip | 第三方库内部 hover | 库自身实现 | 库自身实现 | 库自身实现 | 第三方库行为，非本库自定义实现，不判项目缺陷 |

### 4.1 原生 title 探针（native-title-hover.png）

hover 带 title="切换主题" 的按钮后：role=tooltip DOM 节点数 hover 前 0 → hover 后 0，title 属性节点 52 个。证明原生 title 的提示内容由浏览器 UA 在合成层绘制，不产生可检视的 DOM 浮层；其 hover 保持、Esc 行为、消失时机均由浏览器统一控制，不属于项目可改的浮层实现。按 WCAG 1.4.13，使用原生 title 属UA 职责的规范预期，不算项目缺陷。BBS 页 button[title]（如刷新、发起主题）同理。

### 4.2 命令面板探针（palette-open.png）

交互序列实测：before（custom=[]）→ 点击触发按钮后 paletteFound=true，面板 cls="command-palette ai-panel"、display=flex、visibility=visible、role=dialog、含搜索输入框与 15 个页面导航项（截图目视完整渲染）→ persist500ms paletteStillOpen=true → hoverItem paletteStillOpen=true → Escape paletteGone=true 且焦点标签为 BUTTON（"打开命令面板"触发按钮），焦点正确归还。CDP 匹配规则采集（cdpRules）返回空数组——面板显隐由 Vue v-if/内联状态驱动而非 CSS 规则，未发现阻断三性的 CSS 规则。

三性结论：Hoverable ✓（hover 项不消失）、Dismissible ✓（Esc 关闭且焦点归还，不丢失位置）、Persistent ✓（500ms 后与 hover 期间持续显示）。命令面板本质是键盘触发的模态对话框（role=dialog），非典型 hover content，但即便按 1.4.13 检视亦全部达标。

### 4.3 用户下拉菜单探针（user-menu-open.png）

交互序列：clickOpen opened=true（点击 admin 头像展开）→ hoverItem stillOpen=true → Escape stillOpen=true（ Esc 不关闭）→ outsideClick closed=true。截图目视下拉项（个人中心/系统设置/退出登录）完整渲染。源码 AppShell.vue:128-139 下拉 DOM 由 :429 toggleUserMenu 点击切换，:466 onDocClick 仅处理外部点击关闭，无 Esc 监听。

定性：该浮层由点击触发、非 hover/键盘焦点触发，WCAG 1.4.13 的适用对象是"hover/focus 触发的额外内容"，点击菜单属不同交互模式（由 2.1.1 键盘可达与 4.1.2 状态语义覆盖，且本菜单可由 Tab 聚焦触发按钮后用 Enter 打开），故 1.4.13 对本元素不适用；Esc 不关但外部点击关属点击菜单的常见规范实现，不判 1.4.13 缺陷。

### 4.4 pd-tooltip 死代码（ProcessDesigner）

探针结果：hasPdTooltip=false、showTooltipInvoked=false、custom=[]；CDP 查询 pd-tooltip 选择器返回空数组。源码 ProcessDesigner.vue:361-362 定义了 v-if="showTooltip" 的 pd-tooltip 模板，:8257 showNodeTooltip() 与 :8263 hideNodeTooltip() 函数仅有定义、从未被调用，showTooltip 永为 false——该浮层为不可达死代码，判不可达 L1（非可感知退化，不判 P 级）。

## 5. 原生 vs 自定义浮层根因区分

- 原生 title：174 处静态命中，UA 合成层渲染、无 DOM 节点、行为由浏览器统一控制 → 规范预期，不计项目缺陷。
- 自定义浮层：真浏览器全量扫描未发现任何 hover/focus 即现的自定义 tooltip/popover DOM 节点（customVisible 全 0）；唯一可见自定义浮层是点击触发的命令面板（role=dialog）与用户下拉，均非 hover content 或已满足三性。
- 第三方库浮层：EChartsView 图表 tooltip（仅 QueryManagerDeep.vue:185 消费）、SqlEditor/ScriptCodeEditor 的 CodeMirror cm-tooltip（QueryStatementDesigner.vue:86、ScriptWorkbench.vue:41 消费）——均为库内部 hover 行为，非本库自定义实现，本轮不判项目缺陷。

## 6. 三分判定（据实，不臆造）

- 可达真缺陷：0。全量实测未发现任何自定义 hover/focus 浮层缺失悬停桥接、无 Esc 关闭或中途消失致内容不可读的情形；原生 title 属 UA 规范预期，命令面板三性全满足，用户下拉为点击菜单（1.4.13 不适用）。
- 不可达 L1：1（pd-tooltip 死代码，showTooltip 从未置 true）。
- 正常-规范预期：若干（原生 title 174 处、命令面板、用户下拉菜单、第三方库 tooltip）。

据实判 0 缺陷，不为凑缺陷降低判据。本次未新增修复任务；该维度对本代码库的自定义 hover 浮层部分实质上不适用——仅存在原生 title（UA 职责）与点击型浮层。项目处于 ≥1000 轮迭代中一环，非终态。

## 7. 截图 sha256（命令行 sha256sum 原始值，24 张）

来源：Playwright Chromium headless，非物理真机；palette-open/native-title-hover/user-menu-open 为三性深度探针截图，其余为 PC 双主题 8 页 + H5 5 页逐页证据。

2ae9d177d8de40dda6e27821b16d501a1293defb66c1cbf14bb9ba40c596d496  h5-attendance.png
c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96  h5-index.png
5faf382dd5c4c9f3205155fe393338d87177ed09e6ec06a515aab01c9d7ab51c  h5-meeting.png
cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385  h5-mine.png
d7e32e08a56712d7637405d4af2def4db23c2b618b0ae85c71d2af7a2ff534d8  h5-process.png
98488cec8b01854add6c4fae16e8ef1a5d71757d67dd90779e4e4617706f436e  native-title-hover.png
4e4f99154516815b02d2a63d9f7ff7251afe7a7f7140e2c532bbce6b5a8717f1  palette-open.png
9cf60092816c044d1d50315662df9cbb8730e8c97322fecc306babe12a9f4f97  pc-attendance-dark.png
e43740aea4ae43b571b94a334e33176a1325107f7936c7dc09cb7e2bb4b7c145  pc-attendance-light.png
803f39ce719360f5e95c925e22c1895bb4fb8669066a947f8885f56c0caed8e4  pc-bam-dark.png
db9f61789cd8952e790619961dd01111f755f2523f3fcaa3c46cdbd49f3074b7  pc-bam-light.png
77d20bbbe7b86503dc5bfe6b54f710a0d55fced4a8cd93c90dbd4654a20563ce  pc-bbs-dark.png
497d1deb739931298caa2a5332264083cde28ddf46f4417d48409aa29ac4b251  pc-bbs-light.png
6aebeef7a4140a89d0ac91ed4ac986456d44c6017387c5e48f4b7e848c48594e  pc-calendar-dark.png
4341921be8c0f2e21c47946179de9a03d9f4b480335104100f4724de4edf6192  pc-calendar-light.png
8598325a04d50a0d7c44d9bdb82446c54796f158fd22bf9ed7208611361bd3b3  pc-dashboard-dark.png
85626b8970cc8b12346b2a5d5ecc0ca6f7ff186e787436a1c39052fabbdf4be9  pc-dashboard-light.png
94f4985914fc20808246a883da8c61e2039f00414e8696cbcdc9b2096adaf47c  pc-file-dark.png
4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf  pc-file-light.png
887da0a8cbdb5e02ad1b6aebbbaa7b849f087bd14426b2a46b6ef05040f11738  pc-org-dark.png
e6660e9702b990795f1c5a852f7505011642073e3c429ec7ce6b63727dfb1e7b  pc-org-light.png
9a5f4cff1bd6328806e387dcca7bc2296d28aaf9b68237404270b102b615fc3a  pc-three-member-dark.png
2a12c2b0649cfad3e9f115fd3c135c6f5436eed94df710750f87da9f45ad3c5f  pc-three-member-light.png
5dbf7395a78184f7ff500326ccaafca2eaab82d816367707149271aeaf43381e  user-menu-open.png

## 8. 真值纪律与交付边界

- 域名/测试主机用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，正文 0 真值；测试服探针报告中的真实 URL 仅存在于 gitignored report.json，未写入本文档。
- 凭据仅在 gitignored runner 运行时从测试服读取并注入环境变量，未写入任何入库文件。
- 截图均标注 Playwright Chromium headless 非物理真机，未冒充真机。
- 本轮只读甄别，未改任何源码/契约/台账，所有交互限登录与只读浏览（打开面板/菜单/hover 不含数据写入）。
