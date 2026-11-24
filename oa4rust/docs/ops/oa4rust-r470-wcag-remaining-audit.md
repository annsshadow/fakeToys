# R470 WCAG 剩余维度甄别（真浏览器实证）

- 轮次：R470，任务 t332，attempt b607396b-398e-4cbd-b2c0-541abd6b1166
- 日期：2026-11-24
- 性质：只读甄别，不改动任何入库源码/契约/文档（本文件为唯一交付）
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD f6971c423d9df585fa1aee79664ffed743d4ea8f

## 1. 概况与方法

历史 R458–R468 已穷尽对比度系列、forced-colors、prefers-contrast、文字缩放、颜色独立性、悬停内容、动画暂停、时间限制、Reflow 等维度。本轮系统侦察 WCAG 2.1 AA 尚未专门覆盖的剩余维度，只挑有**真实代码信号 + 可真浏览器实证**者，不强凑。

测试环境：Playwright Chromium headless，**非物理真机**。
- PC：1440×900 DPR1，双主题（light/dark），8 路由 × 2 主题 = 16 组
- H5：390×844 DPR2 isMobile/hasTouch，5 路由，锁浅色
- 合计 21 组，0 pageerror（R470_DONE groups=21 shots=21 errors=0）
- 额外：1.4.12 用户样式覆盖实证 3 页（dashboard/bbs/meeting）

证据目录（gitignored，.gitignore:28 拦截）：oa4rust/test-results/r470-wcag-remaining/（21 张主截图 + 3 张间距覆盖截图 + report.json + spacing-report.json + analysis.txt + CDP 采样）。

## 2. 维度择选依据

| 候选维度 | 静态代码信号 | 是否择选 | 理由 |
|---|---|---|---|
| 2.4.2 Page Titles | main.ts:565-572 router.afterEach 同步 document.title，86 条路由 meta.title，含契约钉死 | 择选 | 信号强、可逐页实证 title 文本 |
| 4.1.1 Parsing | ProcessDesigner.vue:4640/7448 两处 id="arr"（SVG defs marker） | 择选 | 有具体重复 id 疑点，可 DOM 级核验 |
| 1.3.2 Meaningful Sequence | IMChat.vue:17 role=list + :18 role=listitem，多处 ul/li | 择选 | 有语义标记，可核 DOM 顺序/语义 |
| 1.4.12 Text Spacing | QueryStatementDesigner.vue:2728 leading-tight(1.25)、:2732 tracking-tight(-0.025em) | 择选 | 有覆盖工具类疑虑，可做用户样式注入实证 |
| 3.2.4 Consistent Identification | 无专门信号；按钮文本/图标跨页一致性无可自动判定锚点 | 不择选 | 无可靠自动实证信号，据实不强凑、不臆造 |
| 1.4.5 Images of Text | 无图片化文字信号（标题均 CSS 渐变文字，已在 R454/R459 覆盖） | 不择选 | 已覆盖且无新信号 |

## 3. 逐维度证据

### 3.1 WCAG 2.4.2 Page Titles — 正常（0 缺陷）

源码：oa4rust/frontend/apps/desktop/src/main.ts:565-572 router.afterEach 在导航完成后取 to.meta.title，按「页面名 + 空格 + 中点 + 空格 + OA4Rust」格式写入 document.title（无 title 时写品牌名 OA4Rust），随后调用 routeAnnouncer.announce(to)。86 条路由均声明 meta.title。

实测 21/21 组 title 全部正确：
- PC 16 组：格式统一为「页面名 · OA4Rust」，含品牌 + 期望页名（工作台/组织架构/文件管理/论坛/日历/会议管理/考勤管理/公共组件库），双主题一致。
- H5 5 页：navigationBarTitleText 正确（工作台/我的/消息/日历/我的收藏），H5 uni-app 无站点品牌后缀属平台惯例，非缺陷。
- hasExpect 21/21 = true。

判定：**正常-规范预期**。标题随路由同步、含页名、多标签可辨识。

### 3.2 WCAG 4.1.1 Parsing — 正常（0 缺陷；1 项不可达 L1）

静态两处疑点：oa4rust/frontend/apps/desktop/src/views/ProcessDesigner.vue:4640 与 :7448 各有「defs 内 marker id=arr」。精读上下文确认：这是**两个不同 SVG 导出函数**各自生成的**独立 SVG 文档字符串**（各自完整的 svg 根元素），并非注入当前 DOM 的同一棵树；两份导出产物不会同时存在于一个文档。

真浏览器 DOM 级核验（遍历 document.querySelectorAll('[id]') 建映射统计重复）：21/21 组 dupIds=[]，无任何重复 id。

判定：**正常**（DOM 无重复 id）；两处 SVG marker id=arr 属**不可达 L1**（导出产物不注入当前文档，不构成解析冲突）。

### 3.3 WCAG 1.3.2 Meaningful Sequence — 正常（0 缺陷）

源码：oa4rust/frontend/apps/desktop/src/views/IMChat.vue:17 有 div.conversation-list 带 role=list，:18 有 div 项带 role=listitem，列表语义完整。

实测：各页 ul/li 与 role=list/listitem 的 DOM 直接子元素数与视觉项数一致（如 dashboard 5/5、org 10/10、bbs 17/17、meeting 3/3），序列 DOM 顺序与视觉顺序一致，无 role 与结构冲突。

判定：**正常-规范预期**。

### 3.4 WCAG 1.4.12 Text Spacing — 正常（0 缺陷）

静态疑虑：QueryStatementDesigner.vue:2728 有工具类 leading-tight（line-height:1.25）、:2732 有 tracking-tight（letter-spacing:-0.025em），需核实基线是否被压到不可读、以及用户覆盖样式是否导致内容丢失。

基线实测（21 组 p/.desc/.subtitle/.hint 样本）：line-height 全部 ≥1.5 倍字号（18px/12px=1.5、19.5px/13px=1.5、21px/14px=1.5、16.5px/11px=1.5），letter-spacing=normal，word-spacing=0px。基线即满足 1.4.12 推荐值。

用户样式覆盖实证（WCAG 1.4.12 最小要求）：对 dashboard/bbs/meeting 三页注入通配选择器规则（line-height:1.5 / letter-spacing:0.12em / word-spacing:0.16em，均 !important）并对 p/.desc/.subtitle/.hint 注入 margin-bottom:2em !important。

结果（3/3 页完全一致）：
- 文本内容保持：beforeLen==afterLen（dashboard 674/674、bbs 702/702、meeting 653/653），去空白后字符串全等 → textPreserved=true，无信息丢失。
- 无新增裁切：clipsBefore=0 → clipsAfter=0。
- 无横向溢出：docW==cliW==1440。
- computedStyle 生效：lh=24px、ls=1.92px、ws=2.56px（按字号缩放后达到最小倍数）。
- CDP CSS.getMatchedStylesForNode 坐实注入规则命中：通配规则 line-height:1.5 !important / letter-spacing:0.12em !important / word-spacing:0.16em !important，以及 p 等选择器规则 margin-bottom:2em !important 均在前（优先级高于 scoped 原规则）。
- 目视（spacing-dashboard-r470.png / spacing-bbs-r470.png）：字距略增但布局完整、文字清晰、无重叠无裁切，名实相符。

判定：**正常-规范预期**。基线达标且用户覆盖后内容完整保留。

## 4. 三分判定汇总

| 维度 | 可达真缺陷 | 不可达 L1 | 正常-规范预期 |
|---|---|---|---|
| 2.4.2 Page Titles | 0 | 0 | 1（21/21 title 正确） |
| 4.1.1 Parsing | 0 | 1（SVG 导出 marker id=arr 不注入 DOM） | 1（DOM 无重复 id） |
| 1.3.2 Meaningful Sequence | 0 | 0 | 1（语义与序列一致） |
| 1.4.12 Text Spacing | 0 | 0 | 1（基线达标 + 覆盖实证内容保留） |
| 3.2.4 Consistent Identification | — | — | 候选近穷尽，无可靠自动实证信号，据实不择不臆造 |

**总判定：可达真缺陷 0；不可达 L1 1；正常-规范预期若干。据实判 0，不臆造。**

建议：WCAG 2.1 AA 可自动实证的剩余空白维度已近穷尽（本轮 4 维 + 历史 R458–R468 系列）。后续轮次宜转回 AI 风格重构主线（视觉深化 / 交互打磨），WCAG 类仅在有新代码信号或真实用户反馈时再启。

## 5. 截图 sha256（命令行原始值，24 张）

    pc-dashboard-light-r470.png  e2d09055e2f5f4f12019e3c7c10b73e94c497fbe19092cc150c5367268f224b0
    pc-org-light-r470.png  e6660e9702b990795f1c5a852f7505011642073e3c429ec7ce6b63727dfb1e7b
    pc-file-light-r470.png  066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f
    pc-bbs-light-r470.png  888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105
    pc-calendar-light-r470.png  8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad
    pc-meeting-light-r470.png  94c4840fbd266eeaae6c3287abe09f1d0e3b7e874f6e35ef8a358d73fd891b26
    pc-attendance-light-r470.png  678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b
    pc-common-light-r470.png  33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073
    pc-dashboard-dark-r470.png  d1597b125c1efaffcfd53cdedbce8f79d5889329fcccfbfa14793b248ebeaa76
    pc-org-dark-r470.png  887da0a8cbdb5e02ad1b6aebbbaa7b849f087bd14426b2a46b6ef05040f11738
    pc-file-dark-r470.png  d7ebe750e0bf26efbca0896ab619ae7dae03e2af697ff92616fa43b1c35b9875
    pc-bbs-dark-r470.png  6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c
    pc-calendar-dark-r470.png  0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b
    pc-meeting-dark-r470.png  4f8279150d32c028e5165c2b0636c59d76dfcda93e561bf3dae0af6a347c6b46
    pc-attendance-dark-r470.png  85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7
    pc-common-dark-r470.png  afe79b29a25873e32fabc99ddcb7729ec6aab8faffcec317d88c0adfaca33424
    h5-index-r470.png  c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96
    h5-mine-r470.png  cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385
    h5-message-r470.png  170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4
    h5-calendar-r470.png  95af2b84a08af75e603a927b7e757e5cf132f6a848c2cd26b82b9fd1e6f6f1fc
    h5-collect-r470.png  cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6
    spacing-dashboard-r470.png  f8d422003dcbedf3b93cf8f52131a4908f9e6d7efd4039c08e7386205b1009c9
    spacing-bbs-r470.png  e7f79064362242191ff175a06cbb81d1ca20e0674a1292f5ec5ca0705e40edb1
    spacing-meeting-r470.png  153d206a0a0f5d9a695cf6d76d871fdd52c2c55d522e9d8d7b4394766a75c179

## 6. 真值纪律

本文所有服务器地址、账号、端口均以占位符表示（<TEST_HOST>），无真域名/IP/口令/个人路径入库。凭据与 OA_BASE_URL 真值仅存在于 gitignored 的 test-results 脚本中，运行时从受控凭据源提取后注入进程环境，不回显、不入库。截图来源如实标注为 Playwright Chromium headless 非物理真机。本文不含任何真实环境定位信息。
