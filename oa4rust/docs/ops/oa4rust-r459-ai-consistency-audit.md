# R459 跨页 AI 风格一致性甄别（PC 双主题 + H5 浅色）

> 证据来源：Playwright Chromium headless —— PC 1440x900 DPR1、H5 390x844 DPR2 isMobile/hasTouch，测试服真实页面。**非物理真机（NOT a physical device）**。所有数值均取自真浏览器 computedStyle / CDP CSS.getMatchedStylesForNode，不注入伪造数据。

## 0. 结论摘要（据实，不臆造）

- PC 16 个 AI 令牌（light+dark）跨 8 页 100% 一致；`.ai-gradient-text` 类元素跨 8 页×双主题表现一致。
- H5 5 页令牌在 documentElement 读取为空，属元素层解析范式（与 R451/R453 一致），元素层背景渐变正常，非缺陷。
- 发现 **1 处可达真缺陷（P2 视觉呈现不一致）**：共享组件 `AdminLayout.vue` 管理页标题渐变被同规则块内的重复 `color` 声明遮蔽。影响所有使用 AdminLayout 且页面自身无救回规则的管理页（含 common 公共组件库）；meeting / process-work 因页面自身高特异性规则救回，故同名类在两页表现不同。
- 组织/考勤页标题为业务主色 `rgb(37,99,235)`（无 `.ai-gradient-text` 类），属业务语义设计差异，非缺陷；R458 的 stat-label 青端 3.27:1 对比度边界已在该轮记录，不重复判定。

## 1. 甄别维度与方法

- 选维：R458 二选一中未执行的 B（跨页/跨主题 AI 风格一致性），避开已被 R305/R306B/R306C/R441-R457 穷尽的补标记、令牌未定义、双主题对比度等维度。
- PC 8 路由：dashboard / org / file / bbs / calendar / meeting / attendance / common，各 light + dark 两主题共 16 组；主题用 `localStorage[oa4rust_theme]` + `<html data-theme>` 切换，UI 开关 CDP 实测点击 light→dark 可落盘 savedTheme=dark。
- H5 5 页：index / mine / message / calendar / collect，App.vue 显式锁浅色（设计锁定，非缺陷）。
- 每页采集 16 个 `--ai-*` 令牌及 panel / glass / gradientText / header / h1title 五类角色的 computedStyle；对差异点用 CDP DOM.enable→CSS.enable→getMatchedStylesForNode 坐实命中规则与特异性。

## 2. 实测结果

### 2.1 令牌一致性：全一致（正常）

- 浅色：`--ai-glass-bg=#ffffffb8`、`--ai-glass-blur=blur(12px)`、`--ai-glass-bg-border=#0f172a1a`、`--ai-glass-shadow=0 8px 24px`、`--ai-gradient-surface=linear-gradient(160deg,#4f46e50f,#0891b205)`、`--ai-gradient-text=linear-gradient(120deg,#4338ca,#0e7490)`、`--ai-grid-1/2/3=8px/16px/24px`、`--ai-ease-spring=cubic-bezier(.16,1,.3,1)`、`--ai-motion-fast=.14s`，跨 8 页全 TOKEN_SAME。
- 深色：`--ai-glass-bg=#0d15258c`、`--ai-gradient-text=linear-gradient(120deg,#a5b4fc,#67e8f9)` 等跨 8 页全 TOKEN_SAME。
- `--ai-radius-lg`、`--ai-surface-gradient` 在 documentElement 为空，属预期（未在该层定义/已在历轮审计说明），非缺陷。

### 2.2 `.ai-gradient-text` 类元素（侧栏 logo 采样）：跨页一致（正常）

- 浅色：`backgroundImage=linear-gradient(120deg,rgb(67,56,202),rgb(14,116,144))`、`clip=text`、`color=rgba(0,0,0,0)`；深色：渐变色标 `rgb(165,180,252)→rgb(103,232,249)`。8 页×双主题完全一致。

### 2.3 管理页 h1 标题：发现可达真缺陷（P2）

- dashboard（`welcome-heading ai-gradient-text`）：渐变正常。
- file / bbs（`ai-gradient-text`）：渐变正常。
- calendar：自身规则救回，渐变正常。
- meeting（`admin-title ai-gradient-text`）：渐变正常——CDP 命中 `.ai-page .meeting-hero .admin-title[data-v-ef93c602]{color:rgba(0,0,0,0)}`，以更高特异性救回被业务 `.meeting-hero h1{color:var(--color-primary)}` 覆盖的 `transparent`。
- **common（`admin-title ai-gradient-text`，文本「公共组件库」）：异常——** `backgroundImage=linear-gradient(120deg,rgb(67,56,202),rgb(14,116,144))` 已生效、`clip=text` 已生效，但 `color=rgb(15,23,42)`（dark `rgb(240,246,252)`）为实心文本色，遮蔽渐变文字。与 meeting 同名类表现不一致。
- org / attendance：标题无 `.ai-gradient-text` 类，使用业务主色 `rgb(37,99,235)`，属业务语义差异，非缺陷。

### 2.4 H5（正常）

- 5 页 documentElement 16 令牌读取全空（元素层解析范式），非缺陷。
- panel 角色 `page ai-surface-gradient` 5 页全生效：`backgroundImage=linear-gradient(160deg,rgba(79,70,229,0.06),rgba(8,145,178,0.02))`。
- brand 角色 index / mine / calendar / collect 元素渐变 `linear-gradient(135deg,rgb(79,70,229),rgb(8,145,178))` 生效；message 无该元素（业务语义）。
- gradient-text 在 H5 零消费；pressable 在 index 的 transition 生效。

## 3. CDP 根因坐实

- common h1title 命中两条规则：全局 `.ai-gradient-text{color:rgba(0,0,0,0)}` 与 scoped `.admin-title.ai-gradient-text[data-v-cbfb5715]{background:var(--ai-gradient-text);color:var(--text-primary);background-clip:text}`——后者以更高特异性且实心 `color` 覆盖前者，导致渐变被遮蔽。
- meeting 同名类因页面高特异性 `.ai-page .meeting-hero .admin-title[data-v-ef93c602]{color:rgba(0,0,0,0)}` 救回，渐变正常——解释「同名类跨页表现不一致」的成因。
- 源码核对 `oa4rust/frontend/apps/desktop/src/components/admin/AdminLayout.vue:195-204`：`.admin-title.ai-gradient-text` 规则块内存在**两条** `color` 声明——`:199 color:transparent` 被 `:203 color:var(--text-primary)` 覆盖（后者胜出）。当前构建产物 `index-BMFkaD73.css` 中该规则仅为 `color:var(--text-primary)`，与源码最终计算值一致，**非部署时序问题**。
- 该缺陷位于共享布局组件，影响所有使用 `<AdminLayout>` 渲染标题、且页面自身未提供更高特异性救回规则的管理页；本轮 8 页采样中 common 是典型暴露页，meeting / process-work 因自身救回规则不受影响。

## 4. 三分判定

| 项 | 判定 | 依据 |
| --- | --- | --- |
| 16 令牌跨页/跨主题一致性 | 正常 | 16 组×8 页全 TOKEN_SAME |
| `.ai-gradient-text` 侧栏 logo 跨页一致性 | 正常 | 8 页×双主题 computedStyle 全一致 |
| H5 令牌 documentElement 为空 | 正常（元素层解析范式） | 元素层 bgImage 渐变实测生效 |
| org/attendance 标题业务主色 | 业务语义差异（非缺陷） | 无 ai-gradient-text 类，色来自设计 |
| meeting/process-work 同名类渐变正常 | 正常（页面救回） | CDP 更高特异性规则 |
| **common 标题渐变遮蔽** | **可达真缺陷 P2** | 源码重复 color 声明 + CDP 规则 + 截图目视 |
| R458 stat-label 青端 3.27:1 | 已记录边界，非本轮新缺陷 | R458 文档已载 |

## 5. 证据与截图 sha256

- 证据目录：`oa4rust/test-results/r459-ai-consistency/`（21 PNG + report.json + analysis.txt，gitignored）。
- 报告：R459_DONE pc=16 h5=5 cdpPC=8 cdpH5=5，errors=0。
- 截图来源：Playwright Chromium headless PC 1440x900 DPR1 / H5 390x844 DPR2 isMobile/hasTouch 测试服真实页面，**非物理真机**。
- 逐图 sha256（命令行原始值）：

| 截图 | sha256 |
| --- | --- |
| pc-dashboard-light.png | 212f39655ca7c39a9425522e7ac7a13e0c80cab14ac2a9335713f808d9420b68 |
| pc-dashboard-dark.png | 7b6a043779b03f6370b9e0b5b70aa5e28c2e8e7b1986fbc29e2867de5112b7e3 |
| pc-org-light.png | 2398bcaa36981017b1b92cd25e0f23964a037de794d14facd61261ca81f4bcf0 |
| pc-org-dark.png | 9e08677b5bcd43ddb7bcbca0a02f7b9187e4002bab9bcaeff1d57c8c841c8764 |
| pc-file-light.png | 4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf |
| pc-file-dark.png | 94f4985914fc20808246a883da8c61e2039f00414e8696cbcdc9b2096adaf47c |
| pc-bbs-light.png | 7f3838288cbc34c335f454f3757c279defaaac9ed5ff6d8402dc1a9eb72334ae |
| pc-bbs-dark.png | 73989e84056b2b3fbf1876be08fd93c0c7f4e66365aaf5ca14d076fecb21d786 |
| pc-calendar-light.png | 58bc1b43aaa049014fc1cf3a00bac5955f94de87df49724b8479c208f205c783 |
| pc-calendar-dark.png | db2179ae8f542e1a349db9829c54e723ff8be4ce928a522f3da6b776950a4930 |
| pc-meeting-light.png | 358123648fd15b37bbaf1674c33d6c466156cf3a6fb225e30da62d5b558f7345 |
| pc-meeting-dark.png | 2479f8324e5474929a39312dd02d866348b945d747fe6a1df678fa237a9253f1 |
| pc-attendance-light.png | 7571f6534bfd99ead4c0102dc015db3a20c3d3802f24d5065b9b7cf71d8e0011 |
| pc-attendance-dark.png | 620c4d352f45d358c7a8f309c678b36afe97168a36dd19b2bf8135a383da6b8f |
| pc-common-light.png | 94b83b13f2dd1b5a3ae267a21adfae19710551ed3d7cb75657a3c0240ce3f4f0 |
| pc-common-dark.png | 49f84ddb651c533489d8e7e457c2ea236f920478d91bbc23ddc16b07d7c886f0 |
| h5-index-light.png | c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96 |
| h5-mine-light.png | cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385 |
| h5-message-light.png | 170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4 |
| h5-calendar-light.png | 95af2b84a08af75e603a927b7e757e5cf132f6a848c2cd26b82b9fd1e6f6f1fc |
| h5-collect-light.png | cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6 |

## 6. 后续建议（本轮只读不改码）

- 修复 AdminLayout.vue:203 的重复 `color: var(--text-primary);`，或改为在需要实心色的页面用独立类覆盖，恢复 `.admin-title.ai-gradient-text` 的渐变文字呈现。修复后需同步更新管理区契约与真浏览器复验。
- 建议后续任务按 test-first RED→GREEN 落地，契约捕获「同规则块内不得有后置 color 覆盖 transparent」这一意图。

## 7. 真值纪律

- 测试域名/主机：`<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；凭据经 ssh 运行时从环境注入，未入库；无密钥、口令、真实域名/IP/个人路径。
- 截图均为 headless 浏览器真实页面，已标注非物理真机，未冒充真机。
