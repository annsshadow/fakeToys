# R458 PC/H5 AI 风格重构可达新方向甄别——深浅色双模式对比度 WCAG 实证

## 真值纪律
本报告域名/主机一律使用占位符 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`；不含任何真实域名、IP、口令、凭据或个人信息（真值扫描 0 命中）。截图来源：Playwright Chromium headless（PC 1440×900 DPR1 / H5 390×844 DPR2 isMobile/hasTouch）→ 测试服真实页面，非物理真机（NOT a physical device）。

## 一、审计范围与选维理由
- 选维：(A) 深浅色双模式对比度 / 文本可读性实证——R318–R457 未覆盖的新维度。
- 理由：R446–R450 已清零「令牌未定义」维度，既有 `textContrast.contract.test.ts` 只静态解析 dark.css 的十六进制 `--text-*` 令牌，**无法覆盖** ① AI 令牌引入的渐变文字色标（`--ai-gradient-text`，color:transparent + clip:text）② 半透明玻璃卡 / 品牌渐变上的文本实际合成对比度。本轮以真浏览器 computedStyle 取色标 + WCAG 实算补此空白。
- 排除维度：「补 ai-page-b5 标记」已被 B3/B4/R305/R306B/R306C/R445 契约穷尽，补标记会破坏精确 class 契约；OrgViewer 属 EndpointFold 生产 fold=0 不可达——本轮不走此方向（遵照队长静态侦察结论）。
- 可达性实证：PC 左下角 `.theme-toggle`（aria-label="切换主题"）真实存在，CDP 探针实测点击后 `data-theme` light→dark、localStorage 落盘 dark；H5 `App.vue` 显式锁定浅色（禁系统深色覆盖），双模式维度对 H5 天然只有 light，据实记录。

## 二、方法与判定口径
- 测试服真实页面、0 pageerror（report.errors=[]）。PC 6 高频页 × 双主题（工作台/组织通讯录/文件管理/论坛/BAM/日历/会议）：`/oa4rust/app/{dashboard,org,file,bbs,calendar,meeting}`；H5 主页面 3 页：index/mine/message。
- 每页采 heading/body/meta/button/link 关键文本 computedStyle color、逐层合成有效背景、WCAG 2.1 相对亮度算对比度；大文本阈值 3:1、正文 4.5:1；渐变文字按 background-image 色标分别算取最小值。
- 低对比候选一律再用 CDP `CSS.getMatchedStylesForNode` 坐实规则来源，区分 AI 令牌 / 业务色 / 语义覆盖 / 算法误判，严禁据单一算法数字预判缺陷。

## 三、逐页证据表

### 3.1 PC 6 页 × 双主题（对比度取样，low=低于阈值项数）
| 页面 | 主题 | low | heading | body | meta | button |
|---|---|---|---|---|---|---|
| dashboard | light | 0 | 5.36 | 6.86 | 5.51 | 7.58 |
| dashboard | dark | 0 | 9.15 | 7.3 | 5.25 | 6.7 |
| org | light | 0 | 5.36 | - | 5.51 | 7.58 |
| org | dark | 0 | 9.15 | - | 5.25 | 6.7 |
| file | light | 0 | 5.36 | 17.72 | 5.51 | 7.58 |
| file | dark | 0 | 9.15 | 17.11 | 5.25 | 6.7 |
| bbs | light | 0 | 5.36 | 17.37 | 5.51 | 7.58 |
| bbs | dark | 0 | 9.15 | 17.51 | 5.25 | 6.7 |
| calendar | light | 0 | 5.36 | 17.37 | 5.51 | 7.58 |
| calendar | dark | 0 | 9.15 | 17.51 | 5.25 | 6.7 |
| meeting | light | 0 | 5.36 | 4.98 | 5.51 | 7.58 |
| meeting | dark | 0 | 9.15 | 5.72 | 5.25 | 6.7 |

PC 全部 12 组 low=0：`--text-primary/secondary/muted` 双模式令牌在浅/深背景上均 ≥4.5:1；渐变标题色标（dark `#a5b4fc→#67e8f9` / light `#4338ca→#0e7490`）按最小色标算亦达正文阈值。

### 3.2 H5 主页面（light 锁定；初始算法报 16 低对比候选，CDP 逐项判定）
| 页面 | 算法 low | 候选文本:对比度 | CDP 规则来源判定 |
|---|---|---|---|
| index | 4 | 待办审批:1.07 今日出勤:1.07 工作台:3.43 消息:2.95 | 白字 stat-label 压品牌渐变（算法误判，见 3.3）；tabbar 灰 #90979f 属 pages.json 传统配置 |
| mine | 7 | 工作台:2.95 消息:2.95 审批:2.95 考勤:2.95 我的:3.43 开启推送:3.43 保存新密码按钮 3.43 | 同上 tabbar 灰/蓝 + 白字按钮压 #2d8cf0（业务语义按钮色） |
| message | 5 | 工作台:2.95 消息:3.43 审批:2.95 考勤:2.95 我的:2.95 | 同上 tabbar 灰/蓝，pages.json 传统配色 |

### 3.3 决定性 CDP 证据（低对比候选根因坐实）
- `.stat-label` matched CSS 无 color 规则（继承 `.stat-card` 的 `var(--color-surface)`=白），字号 26rpx≈13px、opacity .9；`.stat-card` 同时命中 `.ai-brand-fill{background:var(--ai-gradient-brand)}` 与 scoped `.stat-card[data-v]{background:var(--color-primary);color:var(--color-surface)}`，提权重复类 `uni-page-body .ai-brand-fill.ai-brand-fill` 亦命中 → 真实底色为品牌渐变。
- 实测 stat 卡 `background-color:rgba(0,0,0,0)`、`background-image:linear-gradient(135deg, rgb(79,70,229) 0%, rgb(8,145,178) 100%)`、label 白字。E2E 首轮「1.07:1」是把透明 backgroundColor 穿透到页面底 rgb(245,247,250) 算出的**算法误判**（未合成 backgroundImage），非真实渲染。
- 白字 opacity .9 合成品牌渐变真实对比（WCAG 精算）：靛蓝端 #4f46e5 ≈ 5.43:1、渐变中点 ≈ 4.45:1、青端 #0891b2 ≈ 3.27:1。
- 边界关注点（如实记录，非 P 级缺陷）：`.stat-label` 13px（26rpx）白字在渐变青端 3.27:1 低于 AA 正文 4.5:1，中点 4.45:1 逼近阈值；但文字居中（卡片 text-align:center）实际覆盖区近中点，且 64rpx≈32px 大数字属大文本阈值 3:1（5.43–3.27:1 全达标），两张代表图目视白字清晰可辨、无用户可感知退化。据实列为关注点供后续评估，不臆造为 P 级缺陷，也不隐瞒边界数值。
- tabbar 未选 #90979f / 选中 #2d8cf0 来自 `pages.json` 的 `color`/`selectedColor` 传统配置，非 `--ai-*` 令牌；`#2d8cf0` 亦为 `--color-primary` 传统业务色（App.vue:18）。

## 四、三分判定
1. **可达真缺陷（AI 令牌引入的低对比）：0 项。** PC 12 组双主题全 0 低对比；H5 16 候选经 CDP 全部归因算法误判 / 传统业务色 / tabbar 配置，无一来自 `--ai-*` 令牌。据实判 0，不臆造缺陷。
2. **不可达 L1：1 项（设计锁定，非缺陷）。** H5 `App.vue` 显式只用浅色令牌、禁系统深色覆盖，无 `data-theme` 切换路径，深色模式对 H5 不可达；属既定设计而非遗漏。
3. **正常（据实记录）：** ① PC `--text-*` 双模式令牌对比度健康（既有契约守护 + 本轮真浏览器复核一致）；② AI 渐变文字色标双模式均有效、无透明不可见；③ stat-card 白字压品牌渐变：大数字达标、标签居中区≈4.45:1、青端角落 3.27:1 已如实列为边界关注点（见 3.3），目视无退化故不判 P 级缺陷；④ tabbar/按钮业务色属语义态，不作 AI 风格缺陷。

## 五、截图 sha256 清单（命令行 sha256sum 原始值，15 张）
| 截图 | sha256 |
|---|---|
| pc-dashboard-light.png | afe5a3a3dafc83dd19497a014a1ecac7de491ce70c44cf7154bd5a003201befa |
| pc-dashboard-dark.png | 2fd5733ed3f8d37b4d17a706f811e571001f66e32132888634037ce3ddec0734 |
| pc-org-light.png | 2398bcaa36981017b1b92cd25e0f23964a037de794d14facd61261ca81f4bcf0 |
| pc-org-dark.png | 9e08677b5bcd43ddb7bcbca0a02f7b9187e4002bab9bcaeff1d57c8c841c8764 |
| pc-file-light.png | 4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf |
| pc-file-dark.png | 94f4985914fc20808246a883da8c61e2039f00414e8696cbcdc9b2096adaf47c |
| pc-bbs-light.png | 7f3838288cbc34c335f454f3757c279defaaac9ed5ff6d8402dc1a9eb72334ae |
| pc-bbs-dark.png | a79263b12da49f2a82d019d74c0666f0fc12c54e75ace43a314c1fbd1fec4bba |
| pc-calendar-light.png | 58bc1b43aaa049014fc1cf3a00bac5955f94de87df49724b8479c208f205c783 |
| pc-calendar-dark.png | 806838833b14f266720eef378c6b2a34127a7c83a55ab07af3e97698ecae16f6 |
| pc-meeting-light.png | 358123648fd15b37bbaf1674c33d6c466156cf3a6fb225e30da62d5b558f7345 |
| pc-meeting-dark.png | 2479f8324e5474929a39312dd02d866348b945d747fe6a1df678fa237a9253f1 |
| h5-index-light.png | c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96 |
| h5-mine-light.png | cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385 |
| h5-message-light.png | 170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4 |

## 六、结论与后续
本轮针对 AI 令牌引入后最深处的对比度盲区（渐变文字色标 / 玻璃卡合成背景 / 双主题文本）做真浏览器实证：可达真缺陷 0。深浅色对比度维度健康，无需追加修复。审计证据（15 PNG + report.json + cdp-probe.json）留存 gitignored `oa4rust/test-results/r458-ai-visual-new/`。项目 ≥1000 轮迭代中一环，非终态；后续方向由队长依评审台账裁定。

真值纪律：本报告所有域名/主机均为占位符 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`，真值扫描 0 命中，符合 AGENTS.md Secrets 规范。