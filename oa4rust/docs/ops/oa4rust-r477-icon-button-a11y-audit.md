# R477 WCAG 4.1.2 纯图标按钮无障碍名称甄别

## 1. 概况
- 轮次/任务：t345 [R477]，设计总设计师只读甄别，唯一交付本文档；不改任何入库源码/契约/台账。
- 真父任务：c92e30c56a4169417a1fd0c31799726b31d9e147；分支 opt1000/oa4rust-pc-ai-refactor。
- 审计目标：PC 端纯符号按钮（✕/×/🗑/+/↩/✏ 等无中文文本）是否具备可被辅助技术正确朗读的 accessibleName（WCAG 4.1.2 Name, Role, Value，A 级）。
- 取证环境：Playwright Chromium headless，视口 1440x900 DPR1，浅色主题，非物理真机（模拟浏览器）。
- 纪律：严禁仅凭源码无 aria-label 预判；一律以 CDP Accessibility.getPartialAXTree 实测 accessibleName 为准；带 emoji+中文文字的按钮（如 导出）有可见文字不算候选。

## 2. 取证方法
- 登录：真实 UI 流程填写用户名/密码并提交，waitForURL 进入 /app/ 后逐页导航。
- 覆盖 10 个路由：process-designer、form-designer、form、config-designer、im、file、bbs、calendar、process、recycle。
- 每页：DOM.querySelectorAll 枚举全部 button，过滤可见元素；CDP DOM.enable + Accessibility.enable，DOM.getDocument(depth:-1) → DOM.querySelectorAll(button) → Accessibility.getPartialAXTree(nodeId, fetchRelatives:false) 逐按钮取 name/role/ignored；同步采 DOM text/title/aria-label/aria-labelledby/aria-hidden/display/visible。
- 分类正则：纯符号集 ✕×✖✗🗑➕+-−↩↪⊞→←✓✏👁🔄📤📊 与空白；含 Unicode 汉字判为「有可见文字」。
- 结果：R477_DONE pages=10 errors=0，0 pageerror；额外 detail 探针 items=3（CDP DOM/AX name 来源坐实）。

## 3. 逐页 AXTree 统计表

| 页面 | 可见按钮 | 有合法名称 | 符号名(仅符号) | 空名称 | axIgnored |
|---|---|---|---|---|---|
| process-designer | 34 | 31 | 3 | 0 | 0 |
| form-designer | 10 | 10 | 0 | 0 | 0 |
| form | 54 | 39 | 15 | 0 | 0 |
| config-designer | 42 | 16 | 26 | 0 | 0 |
| im | 5 | 5 | 0 | 0 | 0 |
| file | 18 | 16 | 2 | 0 | 0 |
| bbs | 11 | 11 | 0 | 0 | 0 |
| calendar | 20 | 19 | 0 | 1 | 0 |
| process | 17 | 17 | 0 | 0 | 0 |
| recycle | 10 | 10 | 0 | 0 | 0 |
| 合计 | 221 | 174 | 46 | 1 | 0 |

- 受影响合计 47 个按钮：46 个 accessibleName 仅为符号，1 个 accessibleName 为空。
- form-designer 实测 10 个按钮全部有合法名称；静态源码中的符号按钮在本次实测视口下未暴露为无名称（受 aria-label 保护或未渲染），据实记录、不臆造缺陷。

## 4. 决定性方法学发现：符号内容遮蔽 title
- 按 W3C 可访问名称计算规范，button 的 name from content 优先于 title 属性。实测证实：凡按钮内含可见符号（✏/🗑/↩/↪），即使 title 写了中文语义（如 title=编辑、title=删除），AXTree 的 accessibleName 仍是符号本身，而非 title 的中文。
- 这意味着一类更隐蔽的缺陷：开发者已提供中文 title 作为语义兜底，但被符号内容遮蔽，屏幕阅读器仍朗读符号，中文意图落空。
- 两类成因同样后果：A 类无 title 无 aria（form 15 个、calendar 1 个空），name 为符号或空；B 类有 title 但被遮蔽（process-designer 3、config-designer 26、file 2 共 31 个），name 仍为符号。

## 5. CDP 详情证据（detail.json）
- calendar 索引 15：outerHTML 为 cal-chip 按钮，仅带 style border-color 主色、无文本无 title；DOM 文本空、before/after content=none、宽高 4x4px；AX role=button、name 空、ignored=false、nameFrom 空。源码 CalendarApp.vue 第 35-45 行 v-for 渲染 cal-chip，按钮文本取 c.name，测试数据存在 name 为空的日历项，导致按钮可访问名为空。
- config-designer 索引 15：outerHTML 为 si-btn 按钮、title=编辑、文本 ✏；AX role=button、name=✏、ignored=false。源码 ConfigDesignerApp.vue 第 66-67 行 title=编辑/删除 + 符号内容。
- form 索引 10：outerHTML 为 btn-sm btn-del 按钮、无 title、文本 🗑；AX role=button、name=🗑、ignored=false。源码 FormApp.vue 第 33 行 deleteForm 删除按钮。
- 其余符号按钮源码定位：ProcessDesigner.vue 第 12/13/20 行（撤销↩/重做↪/清空🗑，均带 title 但 AX name 为符号）；ConfigDesignerApp.vue 第 130 行（历史项删除🗑，无 title）；FileManager.vue 第 64/85 行（删除🗑，带 title）。

## 6. 三分判定
- 可达真缺陷 1 项（P2）：47 个纯图标按钮的 accessibleName 仅为符号或为空，无法向辅助技术可靠传达按钮语义（WCAG 4.1.2）。其中 31 个已提供中文 title 但被符号内容遮蔽（本可通过 aria-label 避免），16 个无 title/aria 或数据驱动空名。定 P2 依据：影响屏幕阅读器用户操作与理解，不阻断主流视觉用户功能，且为低成本可修复的标签缺失。
- 不可达 L1：0。10 页按钮全部可见、axIgnored=0，无 aria-hidden/display:none 排除项；本次 10 页均不在 R306C 硬排除页清单（PortalScriptDesigner/PortalWidgetDesigner/ServiceInvoke/FindDesigner/PdfViewer/Common）内。
- 正常规范预期：174 个按钮有合法可访问名（aria-label 或可见中文文本），含带 emoji+中文的按钮（如 导出、保存），未误判为候选。
- 据实判定，不臆造、不凑轮次；缺陷清单与源码行号、CDP name 来源、截图三方证据一致。

## 7. 截图 sha256（命令行 sha256sum 原始值，与 report.json 一致）

- process-designer-icon-a11y.png：1f4a124f765404b974003e5179eb145cf835dd5f3132dd36e12ca3eb38414610
- form-designer-icon-a11y.png：c1586ab9feff86ffddfbf5c4c52502d897b00fc4c7ab0a64e76122c0e6b0011c
- form-icon-a11y.png：577b73c3de9a05e9ab28d20d1551e3de8af93c0f607d2b23c904997db8c24f80
- config-designer-icon-a11y.png：5605435ed5fc32e592d8d141330682c47a64e65cb25046a0d66b0079b8109380
- im-icon-a11y.png：4305f68b1b168138bead0dc7d973c0af31aed7bcb79ebe6e8b338c1a6cbdc24a
- file-icon-a11y.png：066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f
- bbs-icon-a11y.png：888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105
- calendar-icon-a11y.png：8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad
- process-icon-a11y.png：1cdfa352dc73baedcc374b29e31413d3ca4c4c876b068c3af0dd2ebb53762a72
- recycle-icon-a11y.png：8456a5267ac759247176081ae925fc0a4248fe6fc32637152a1f43d0e2475472

## 8. 证据目录与真值纪律
- 证据目录（gitignored）：oa4rust/test-results/r477-icon-button-a11y/，含 10 张 PNG、report.json、analysis.txt、detail.json；脚本 r477-e2e.cjs、r477-analyze.cjs、r477-probe.cjs、r477-detail.cjs 及 runner 位于 oa4rust/test-results/（.gitignore 拦截）。
- 截图来源：Playwright Chromium headless 模拟浏览器，非物理真机；已在证据 source 字段如实标注。
- 真值纪律：测试域名/主机用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；口令经 SSH 运行时从受控凭据源提取注入进程环境，不落仓库、不在文档回显；本文档无真实域名、IP、端口、口令或个人路径。