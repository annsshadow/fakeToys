# R479 WCAG 2.4.7 焦点可见性甄别

## 1. 概况
- 轮次/任务：t349 [R479]，设计总设计师只读甄别，唯一交付本文档；不改任何入库源码/契约/台账。
- 真父任务：7e980e7ea1ef60105e5b6c0d73372d5fa7730f38；分支 opt1000/oa4rust-pc-ai-refactor。
- 审计目标：PC 端键盘 Tab 焦点指示器（WCAG 2.4.7 Focus Visible，AA 级）——静态信号 outline:none 约 138 处、focus/:focus-visible 约 31 处，但 outline:none 本身非缺陷（border-color/box-shadow 替换可能是合法焦点指示），须真浏览器 Tab 逐次聚焦实证。
- 取证环境：Playwright Chromium headless，1440x900 DPR1，浅色/深色双主题，非物理真机（模拟浏览器）。
- 纪律：严禁仅凭源码 outline:none 预判；Tab 实测焦点指示器真实可见为准；border-color 变化/box-shadow 发光属合法指示。

## 2. 取证方法
- 登录：真实 UI 流程填写用户名/密码提交进入 /app/。
- 覆盖 8 个代表性交互页双主题共 16 组：dashboard、bbs、form、config-designer、appinfo、process-designer、calendar、common（含搜索框/表单输入/按钮/侧栏导航链接）。
- 每页：标记所有可见可聚焦元素（a[href]/button/input/textarea/select/[tabindex]），逐次 keyboard.press(Tab) 最多 14 次；对 document.activeElement 采 computedStyle（outline-style/width/color/offset、border-color、box-shadow），与 blur 态基线对比；同步记 el.matches(:focus-visible)。
- 判定：focusVisible 且有可见 outline（style 非 none 且 width≥1px）或 border-color 变化或 box-shadow → VISIBLE；focusVisible 但三者全无 → INVISIBLE（可达缺陷）。
- CDP：DOM.enable + CSS.enable + getMatchedStylesForNode 取当前焦点元素的匹配 CSS 规则；截图目视 Tab 焦点环。
- 结果：R479_DONE groups=16 focusVisible=222 invisible=0 shots=16 errors=0，0 pageerror。

## 3. 逐页焦点统计（双主题16组）

| 页面 | 主题 | 可聚焦基线 | Tab聚焦次数 | focus-visible | 有outline | 不可见 | 全局焦点环命中 |
|---|---|---|---|---|---|---|---|
| 工作台 (dashboard) | light | 14 | 14 | 14 | 14 | 0 | 是 |
| 工作台 (dashboard) | dark | 14 | 14 | 14 | 14 | 0 | 是 |
| 论坛 (bbs) | light | 13 | 13 | 13 | 13 | 0 | 是 |
| 论坛 (bbs) | dark | 13 | 13 | 13 | 13 | 0 | 是 |
| 表单管理 (form) | light | 14 | 14 | 14 | 14 | 0 | 是 |
| 表单管理 (form) | dark | 14 | 14 | 14 | 14 | 0 | 是 |
| 配置设计器 (config-designer) | light | 14 | 14 | 14 | 14 | 0 | 是 |
| 配置设计器 (config-designer) | dark | 14 | 14 | 14 | 14 | 0 | 是 |
| 应用管理 (appinfo) | light | 14 | 14 | 14 | 14 | 0 | 是 |
| 应用管理 (appinfo) | dark | 14 | 14 | 14 | 14 | 0 | 是 |
| 流程设计器 (process-designer) | light | 14 | 14 | 14 | 14 | 0 | 是 |
| 流程设计器 (process-designer) | dark | 14 | 14 | 14 | 14 | 0 | 是 |
| 日历 (calendar) | light | 14 | 14 | 14 | 14 | 0 | 是 |
| 日历 (calendar) | dark | 14 | 14 | 14 | 14 | 0 | 是 |
| 公共组件库 (common) | light | 14 | 14 | 14 | 14 | 0 | 是 |
| 公共组件库 (common) | dark | 14 | 14 | 14 | 14 | 0 | 是 |
| 合计 | 双主题 | — | 222 | 222 | 222 | 0 | 16/16 |

- 全部 222 个键盘焦点均为 :focus-visible、均具备可见 outline，0 个不可见；另 6 个焦点叠加 box-shadow 发光（不额外判缺陷）。
- 焦点环颜色：浅色主题 --color-primary=rgb(37,99,235) 靛蓝系（含渲染微差 rgb(17,45,237)/rgb(7,17,237)）；深色主题 rgb(0,212,255) 青色系（含微差 rgb(86,183,255)/rgb(85,183,255)/rgb(130,167,255)）。outline-width 恒 2px、outline-offset 恒 2px。

## 4. CDP 决定性根因（焦点环从何而来）
- 全局规则（dark.css 第 284-289 行）：
  选择器 :where(a, button, input, textarea, select, [tabindex]):focus-visible
  声明 outline: 2px solid var(--color-primary) !important; outline-offset: 2px !important;
- CDP matched rules 对深色/浅色各页 admin-nav-item 等焦点元素均命中该 :where(...):focus-visible 规则，计算值为 2px solid 主题主色；该规则用 !important 覆盖各视图 scoped 的 outline:none（全库 outline:none !important 命中 0）。
- 例：appinfo 浅色 admin-nav-item「角色管理」focusVisible=true、outlineStyle=solid、outlineWidth=2px、outlineColor=rgb(37,99,235)、outlineOffset=2px、vis=VISIBLE；CDP 同时命中浏览器 UA :focus-visible 与本全局规则，后者 !important 胜出。
- !important 意图（dark.css 注释已说明）：只在键盘交互时出现，鼠标点击不触发，不干扰输入框 :focus 时用 border-color 表达聚焦的既有视觉设计——与本次实测 border-color 变化 0 但 outline 222/222 的结论一致。

## 5. 三分判定
- 可达真缺陷 0：222/222 键盘焦点有清晰可见的 2px 焦点环，双主题均consistent，截图目视可辨，无「焦点态无任何可见指示」实证。
- 不可达 L1 0：所有 Tab 落点均为可见可聚焦元素，无 aria-hidden/display:none 排除项；无 R306C 硬排除页。
- 正常-规范预期若干：outline:none 被全局 :focus-visible !important 合法覆盖；输入框 border-color :focus 属既有视觉设计；box-shadow 发光叠加 6 处合法。
- 据实判 0 不臆造，不与已覆盖维度（forced-colors R461、prefers-contrast R462、1.4.13 R465）重复。

## 6. 目视证据
- pc-dashboard-light.png：侧栏「工作台」导航项蓝色 2px 焦点环清晰；主内容完整。
- pc-appinfo-light.png：侧栏「CMS索引设计」导航项蓝色焦点环清晰；应用管理表格无崩坏。
- pc-form-dark.png：深色表单管理页侧栏「CMS索引设计」青色焦点环清晰；表单卡片完整。
- pc-bbs-light.png：论坛页侧栏「工作台」项蓝色焦点环清晰；板块列表完整。

## 7. 截图 sha256（命令行 sha256sum 原始值，与 report.json 一致）

- pc-appinfo-dark.png：0a9dc8ba62d165082f348ae80739b06edd3c456d5cd025d4505c1fdab30aa954
- pc-bbs-dark.png：170cc576271090162ae4fe8b56664a20d81e0679cfd1b3a081348f2deba93f17
- pc-appinfo-light.png：3281b8a8be95f4955bbc34e531bf1a853a11bd1da1626b2113b195746414af72
- pc-calendar-light.png：3acd7494f8f21b4ef28b42d6c2adefb6965635115c94557dfa34425c352c690b
- pc-dashboard-dark.png：4698d975016259c2a0ef7c16304381a58cbafd71f14718e2d79c21b5e8c5c42c
- pc-config-designer-light.png：551e442ff2a2a3df44dcb77a3751176bb8e23b8275741ca408a56aa6b3ed761b
- pc-common-light.png：5871c6b30da5117322778400d907ad3f0efa3e953080d3dc836e945a78bf23e1
- pc-calendar-dark.png：852fca2bf98de0cc726d30ed19399791f06f22fbac7097a1677c68a4d5208d66
- pc-dashboard-light.png：a65683c974264d882d3d2e256aa9fabfab42250dc54f95db5a80f364d978a41a
- pc-form-dark.png：b5a3be54d35c877fbb98e141573a265cdb265631407ff59ac1d61e60fddc30cb
- pc-process-designer-light.png：bc88542d9092ea74f1e2c69735942c52118d37bd0da7721350b98c2d6c90a3bc
- pc-config-designer-dark.png：d17ef29e62db91d120cf33891bcc9aa308490318ea3cb3ca4648a99cdf991949
- pc-form-light.png：e36ab37548f34d35fa285bc8c2331ceb2120935f0150e510ba315062d93b34b4
- pc-process-designer-dark.png：f100581576c161485fca377428725d53b91c8fade4928788eb4e04449c0d696e
- pc-bbs-light.png：f218e10ff947b25be9e30a27c9a3b2ed2a1091bbaa52f2b798a0065ba99c0995
- pc-common-dark.png：f522e1e59eef099876c07ba2b18175788bf4eeaff6c001dc41078816c048545c

## 8. 证据目录与真值纪律
- 证据目录（gitignored）：oa4rust/test-results/r479-focus-visible/，含 16 PNG、report.json、analysis.txt；脚本 r479-e2e.cjs、r479-analyze.cjs、r479-detail.cjs、r479-colors.cjs 及 runner 位于 oa4rust/test-results/（.gitignore 拦截）。
- 截图来源：Playwright Chromium headless 模拟浏览器，非物理真机；已在 report.source 字段如实标注。
- 真值纪律：测试域名/主机用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；凭据经 SSH 运行时从受控凭据源提取注入进程环境，不落仓库不回显；本文档无真实域名、IP、端口、口令或个人路径。