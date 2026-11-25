# R511 WCAG 2.1.4 Character Key Shortcuts 甄别报告

## 1. 依据与判据

- 准则：WCAG 2.1 AA 2.1.4 Character Key Shortcuts —— 若快捷键仅用单字符（字母/标点/数字）实现，必须满足下列至少一条：可关闭（turn off）、可重映射（remap）、或仅在对应组件获得焦点时生效（active only on focus）。
- 本轮唯一对象：流程设计器全局 document 级单字符快捷键 h / g。
- 反预判铁律：静态锚点只是线索，缺陷成立必须真浏览器坐实「在文本输入控件内打字实际误触发」；业务守卫（multiSelected>=2、groupedNodes>=2）不等同 WCAG 关闭机制；带修饰键快捷键不属本维度。
- 三分判定：可达真缺陷（单键无守卫 + 无关闭/重映射入口 + 实证误触发）/ 不可达 L1（路径未构造成功、据实不判）/ 正常规范预期（带守卫或带修饰键的对照）。

## 2. 基线与来源

- 分支：opt1000/oa4rust-pc-ai-refactor；真父 76b2cedbcf751b92f509f8258230e763eba11ffb（台账 §505）；HEAD=UP，AB=0/0。
- 来源声明：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题，**非物理真机**；CDP DOM + Accessibility.getPartialAXTree + capture/bubble 双阶段键盘事件探针。
- 测试服真域名/主机仅存在于 gitignored 本地 runner（占位 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`），未入仓。口令由 runner 从测试服运行时读取并注入环境变量，未回显。
- 源码 sha256：ProcessDesigner.vue = 623261c1d106e2f295b0ded417a410bfbbbbec7d1d83b549d0bb9bb3136cdb70；IMChat.vue = 3fa954a5ead62873d5da672a59102ef2fd9dc6c9d7b53020faa6bb50b67eec74。

## 3. 静态锚点（线索，须实测确认）

- ProcessDesigner.vue:6276-6329 `onDocumentKeyDown`，:6333 `document.addEventListener('keydown', onDocumentKeyDown)` 全局注册。
- :6298-6302 Delete/Backspace 分支**有** activeElement 守卫（tagName 非 INPUT/TEXTAREA 才 deleteSelected）——正面对照。
- :6309 `if (e.key === 'h' && !e.ctrlKey)` → `e.preventDefault(); showHelpModal = !showHelpModal` —— **无** activeElement 守卫。
- :6313 `if (e.key === 'g' && !e.ctrlKey && multiSelected.size>=2)` → `e.preventDefault(); createGroup()` —— 无 activeElement 守卫，但有业务守卫；createGroup (:4835-4836) 另有 `groupedNodes.size<2` 守卫。
- :28 / :1003-1030 帮助面板仅展示快捷键清单（⌨️ 帮助），grep 全文件无关闭/重映射开关。
- :8393 `handleShortcut` 定义、:8390 `toggleShortcutHelp` 定义，但 grep 全文件仅此定义命中；模板 keydown 仅 :112（canvas delete/ctrl.d）与 :592-595（minimap），无 handleShortcut 绑定 —— 判为未注册死代码（据 grep 与模板实证，不预判）。
- IMChat.vue:96-97 Ctrl+Enter 发送 / Shift+Enter 换行 —— 带修饰键，正面对照。

## 4. 真浏览器程序化证据（三轮）

### 4.1 主轮（report.json，双主题）

- 路由 /oa4rust/app/admin/process-designer/，画布就绪、0 pageerror、0 fatal。
- 聚焦搜索框 `.sb-search input` 后连续按 h：hSteps helpVisible = [true, false, true]（每次切换），afterH.helpVisible=true，AXTree `heading/⌨️ 快捷键` 与 `button/✕`。
- capture/bubble 探针 evH：3 次按键 active=INPUT(.sb-input)，prevented=**true**；同轮 g 分支 evG prevented=**false**（multiSelected<2 未进入分支），searchValue 由空变 ggg —— 反证 h 被 preventDefault 阻断输入而 g 未阻断。
- 设置入口扫描：buttonCount=35，shortcutRelated=[「⌨️ 帮助」]，checkboxCount=0。

### 4.2 补采（supplement.json，双主题）

- 真实打开「新建流程」弹窗，聚焦描述 textarea `.fta` 连续键入 h：beforeT.activeTag=TEXTAREA、descValue=''；tSteps 三次 descValue 恒为 ''（字符未录入），helpVisible=[true,false,true]。
- 双阶段探针 probeT：cap 记录首击焦点 tag=TEXTAREA(.fta)、后续因帮助弹层抢焦变 BUTTON(.btn-close)；bub 三次 prevented=**true**。坐实 h 在真实文本域内误触发帮助且阻止录入。
- Delete 正面对照：先点击载入含 3 节点流程，聚焦搜索框按 Delete，nodeCount 3→3，未删节点 —— 同函数 Delete 分支守卫有效。
- g 多选尝试：Shift 点选 2 节点后聚焦搜索框按 ggg，beforeG/afterG.groupLabelCount 恒 0、zoneCount=3，未生成分组（createGroup 依赖 groupedNodes，纯按 g 未满足守卫）—— 据实不判 g 为可达。

### 4.3 第三轮（round3.json，双主题）

- 点开「⚙️系统设置」面板：panelText 仅「基础配置/安全设置/通知配置/关于」四组（系统名称、Logo、时区、语言），bodyHasShortcut=**false**，shortcutControls=**[]**，inputCount=11。坐实无单字符快捷键关闭/重映射入口。
- 据实披露：本轮 clean-h 复测片段焦点落在 `MAIN.content`（cap tag=MAIN、prevented=false、desc 恒空、helpVisible=false），**焦点未进入 textarea，采样无效，不作 h 判定依据**；h 判定仅依据 4.1 与 4.2 两轮有效证据。

## 5. 三分判定

- **可达真缺陷 1 项（P2）**：ProcessDesigner.vue:6309 `h` 单键全局 document 级监听、无 activeElement 守卫，在文本输入控件（搜索框、描述 textarea）内打字实际误触发帮助弹层并 preventDefault 阻断字符录入；系统设置与帮助面板均无可关闭/可重映射入口。违反 2.1.4 三条例外均不满足。双主题实证一致。P2 边界据实：误触发帮助属干扰性、非数据破坏，不夸大。
  - 修复建议：在 :6309 h 分支（及同构 g 分支）前置与 Delete 相同的守卫，如 `document.activeElement?.tagName==='INPUT' || ==='TEXTAREA' || isContentEditable` 时跳过 e.preventDefault 与 toggle；或在帮助面板/系统设置提供单键快捷键的关闭或重映射开关。复用现有 activeElement 守卫模式，零业务语义变更。
- **不可达/未实证 L1 2 类**：① g 单键 createGroup 路径 —— groupedNodes 业务守卫未满足（groupLabelCount 恒 0），三轮未实证触发分组，据实不判缺陷；② 第三轮 clean-h 片段焦点未入 textarea，采样无效据实排除。
- **正常规范预期若干**：Delete/Backspace 分支 activeElement 守卫有效（nodeCount 3→3）；Ctrl+A/Z/Y/D/=/0 均带修饰键；Escape 取消选择；IMChat.vue:96-97 Ctrl+Enter/Shift+Enter 带修饰键；handleShortcut 未注册死代码不生效。
- 反证边界：全库多页面快捷键未穷举，本轮仅实采流程设计器与 IMChat，其余不预判。

## 6. 证据清单（sha256 命令行核验）

| 文件 | sha256 |
| --- | --- |
| report.json | cc80ec47009ebab5ee447462a1f364f8c2cbf0d2d030bf406ff6d947026d8f9f |
| supplement.json | 5af883e73a8290a7ef3bb40ae5a70eb91e2f581734989bd79e5c2d0e94cde724 |
| round3.json | c0aa8adaace3d7765e0377c174d7bae27dd8d78c832307ac076d5e1d2f586db6 |
| pc-pd-h-light-r511.png | 8634b5b53f74378a2936569eb57b222ceee335af6a51af19dd82f0108a056344 |
| pc-pd-h-dark-r511.png | 7fa1e315c8948c75fd88e8b3f7e4b53fc79f872429f57d541c216c9d1614eee9 |
| pc-pd-g-light-r511.png | b2bb7f8eb585dad4d6e7f28251b3fe27269b2d6642fdf244e1e8511d4b66b41c |
| pc-pd-g-dark-r511.png | 0c0cf6eef3fd0d25d239d66d7e998053adefc5fecc7d9d9008910b2f158f67fb |
| pc-pd-help-light-r511.png | e94278b431a2987c44d3d4abc7ea494fc43cc9e1e3c0e5dcc42b5c9415f5e1a1 |
| pc-pd-help-dark-r511.png | 6007bac979ca42b7243154ed3e6e8a3eb94adda7471a490f02b1cfded9a0e8e4 |
| sup-textarea-h-light-r511.png | 7a1c35e04323b3427bdfd5a2f86295be9c0b1d2175181689760bf51b4ce89fc4 |
| sup-textarea-h-dark-r511.png | 6063b4410123bf4fc37f4e46d7b1209b047cc50bf802bae0aba4335c1490e019 |
| sup-multi-g-light-r511.png | 72466804f77d89ff7e00f95333136dee562ac1c1e3bac5f743b4ed41ec5a0286 |
| sup-multi-g-dark-r511.png | 3790825e966c525e4992b82f60042565074a278a99cbd74d011e7c972db12bf4 |
| r3-settings-light-r511.png | 4ab413f8eca8d74c08cb2d3666788e91d3745234eb5263c56d4e49a6f915390c |
| r3-settings-dark-r511.png | 1e0117d83ecf94f2940db3e49827528c8d8ca9f4d0efe9ab949bf1d1ca92d798 |
| r3-clean-h-light-r511.png | 7d03e0d646596a468d5009c8b34a7010ddba999dda18e5d7c5fa00c9e937099d |
| r3-clean-h-dark-r511.png | f872252f4679170a15b89e024f964315386ef50d4536ddc993e4b327ec689ee3 |

截图目视（1440x900 非物理真机）：pc-pd-h-light 显示「⌨️ 快捷键」帮助浮层覆盖流程设计器；pc-pd-g-light 显示搜索框内 ggg 与空画布（无分组）；pc-pd-help-light 显示帮助浮层含 G 键分组条目；r3-settings-light 显示系统设置四分组无快捷键项。

## 7. 脱敏边界

- 全文仅占位 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`，0 真值域名、IP、主机、SSH 端口、口令、私钥。
- 证据目录 oa4rust/test-results/r511-* 已 gitignore，仅文档入仓。

## 8. 结论

本轮据实识别 1 项可达真缺陷（P2，h 单字符快捷键无输入焦点守卫且无关闭/重映射，双主题实证），2 类不可达/未实证，其余为带守卫或带修饰键的正常规范预期。本轮为只读甄别，不改源码；修复建议已给出，待后续修复轮 RED→GREEN 验证。≥1000 轮持续迭代，非终态。