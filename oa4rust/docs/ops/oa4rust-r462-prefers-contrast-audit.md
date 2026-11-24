# R462 prefers-contrast: more 模式下 AI 风格对比度可达性甄别

> 甄别轮次：t321（Kind=work，只读甄别，不改任何入库源码/契约/台账）  
> 真父：`f37c9178ccb1131519fcc4a63ebf8ef175605f7f`（台账 §457 已 push）  
> 证据目录：`oa4rust/test-results/r462-prefers-contrast/`（42 PNG + `report.json`）  
> 来源标注：Playwright Chromium headless `emulateMedia({contrast:"more"})` 与 baseline `no-preference` 同页横比，PC 1440x900 DPR1 / H5 390x844 DPR2 isMobile/hasTouch；**NOT a physical device，非物理真机**

## 1. 审计范围与空白维度坐实

全库 `prefers-contrast` 命中 **0**（真实空白维度）。Windows/macOS「提高对比度」无障碍偏好与 forced-colors 不同：forced-colors 是系统强制替换调色板，prefers-contrast 保留色值而请求提升对比。本轮实证该偏好下 AI 风格对比度可达性。

已覆盖维度严禁重复（本轮只做附带记录，不重复判缺陷）：

- `prefers-reduced-motion` 守卫：`packages/ui/src/theme/dark.css` + `reducedMotion.contract.test.ts`；
- `:focus-visible` 焦点环：同上契约；
- 触摸目标 44px：H5 `App.vue:163-166` + `mobile/design-system.contract.test.ts`，PC `modern.css:142` + `tokens.css:104`；
- 骨架屏：R453/R456 已穷尽；
- 深浅色 WCAG 对比度：R458 判 0 缺陷；forced-colors：R461 判 0 缺陷。

## 2. 实测方法与样本

E2E 脚本（gitignored）：`oa4rust/test-results/r462-e2e.cjs`、`r462-e2e-run.sh`；凭据经 ssh 运行时从受保护凭据文件读取注入环境变量，域名真值只在 gitignored runner，不入库。

同一页面、同一主题、同一登录态下依次 `emulateMedia({contrast:"no-preference"})` 采 baseline + 截图，再 `emulateMedia({contrast:"more"})` 采 more + 截图，逐字段横比 `color` / `bgColor` / `bgImage` / `clip` / `fill` / `border` / `backdrop`，并记录半透明色样本（如 `rgba(15,23,42,0.08)`）数量。CDP 先 `DOM.enable` + `CSS.enable` 再 `CSS.getMatchedStylesForNode`。

样本：PC 8 页（dashboard/org/file/bbs/calendar/meeting/attendance/common）× 双主题 = 16 组；H5 5 页（index/mine/message/calendar/collect）锁浅色。每组 baseline + more 各 1 张截图，共 **42 张**。

实测结果：`R462_DONE pc=16 h5=5 cdpPC=8 cdpH5=5 errors=0`，`ERRORS=[]`，**0 pageerror**。

## 3. 决定性证据：媒体查询激活但渲染零变化（规范预期）

| 证据维度 | 结果 |
|---|---|
| 媒体查询激活 | more 态 21/21 组 `matchMedia("(prefers-contrast: more)").matches=true`；baseline 态 21/21 为 false；`less` 两态均 false，`forced-colors` 两态均 false |
| computedStyle 横比 | 9 类角色 × 7 字段 × 前三元素，baseline→more **差异组数 0/21**（逐字段零差异） |
| 半透明色样本 | 每页 baseline 与 more 的半透明样本数完全相同（如 dashboard-light 8 vs 8、org-dark 0 vs 0），未见对比增强请求导致透明度改变 |
| 截图像素横比 | 21 组 baseline/more 截图 sha256 **逐一完全一致**（`PIXEL_IDENTICAL=21 PIXEL_DIFF=0`），像素级零变化 |
| CDP 根因 | 8 PC 页 + 5 H5 页匹配规则共 **149 条，含 `prefers-contrast` media 规则 0 条**；命中规则均为常规 `.ai-gradient-text{background:var(--ai-gradient-text);color:rgba(0,0,0,0);clip:text}` + scoped 规则，无 `@media (prefers-contrast: more)` 分支 |

**判定**：Chromium headless 对 `prefers-contrast: more` 的原生支持有限——`emulateMedia` 正确设置了媒体查询状态，但浏览器未据此改变任何渲染（无 media query 实现、引擎也不自动增强对比）。这符合任务背景中「无可见变化若因浏览器不改渲染属规范预期非缺陷」的明确口径。本项目无 `@media (prefers-contrast)` 实现，既无因对比增强导致的退化，也无刻意适配需求。

## 4. 逐角色可读性（more 态，与 baseline 像素一致）

因 more 与 baseline 像素完全相同，下表即为 contrast:more 下实际呈现（经 3 张关键图目视确认）：

| 角色 | 呈现 | 判定 |
|---|---|---|
| 渐变文字 `.ai-gradient-text` | common 浅色「公共组件库」蓝紫渐变、meeting 深色「会议管理」青紫渐变清晰（R460 修复后 color=transparent+clip:text 生效） | 可读，非缺陷 |
| 玻璃卡 `.ai-panel`/`.glass-card` | 浅色半透明背景 + 边框 + 阴影，深色卡片边框完整，内容可辨 | 可读，非缺陷 |
| 焦点环 | 系统默认焦点可见（R461 已记录系统蓝环，本轮附带不重复） | 不退化 |
| 品牌按钮 `.ai-brand-fill` | H5 工作台靛蓝→青渐变统计卡白字「8 待办审批」「1 今日出勤」清晰 | 可读，非缺陷 |
| 半透明色 `rgba(15,23,42,0.08)` 骨架/标签 | 样本数与 baseline 一致，未因对比增强请求改变；R458 已判 WCAG 0 缺陷，本轮不重复 | 不退化 |

## 5. 截图 sha256（命令行 `sha256sum *.png | sort` 原始值；21 组 baseline/more 逐一完全相同）

下表每组 sha 同时适用于 `-baseline.png` 与 `-more.png`（两者 sha256 完全相同）：

| 页面组 | sha256（baseline == more） |
|---|---|
| `pc-dashboard-light` | `02229d24ce1c7cc5e7e5d60f470806bf455f9fe5e82f7fedf1b76b68e36dcc21` |
| `pc-org-light` | `2398bcaa36981017b1b92cd25e0f23964a037de794d14facd61261ca81f4bcf0` |
| `pc-common-light` | `33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073` |
| `pc-meeting-light` | `41e50eb557ff8c1edee28386ccd7cdcf37b255b6bfb0386d3840bff3fe896b31` |
| `pc-file-light` | `4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf` |
| `pc-calendar-light` | `58bc1b43aaa049014fc1cf3a00bac5955f94de87df49724b8479c208f205c783` |
| `pc-attendance-dark` | `620c4d352f45d358c7a8f309c678b36afe97168a36dd19b2bf8135a383da6b8f` |
| `pc-meeting-dark` | `66e6c90af2d05f18241cbe9f899dcf600d7edf004aac676e70a82bb242153a93` |
| `pc-bbs-dark` | `73989e84056b2b3fbf1876be08fd93c0c7f4e66365aaf5ca14d076fecb21d786` |
| `pc-attendance-light` | `7571f6534bfd99ead4c0102dc015db3a20c3d3802f24d5065b9b7cf71d8e0011` |
| `pc-bbs-light` | `7f3838288cbc34c335f454f3757c279defaaac9ed5ff6d8402dc1a9eb72334ae` |
| `pc-dashboard-dark` | `7fd99d455261b8d6a8f07a7ffd5ddaac7b440652e14aaa337c93bcff6342f330` |
| `pc-file-dark` | `94f4985914fc20808246a883da8c61e2039f00414e8696cbcdc9b2096adaf47c` |
| `h5-calendar` | `95af2b84a08af75e603a927b7e757e5cf132f6a848c2cd26b82b9fd1e6f6f1fc` |
| `pc-org-dark` | `9e08677b5bcd43ddb7bcbca0a02f7b9187e4002bab9bcaeff1d57c8c841c8764` |
| `pc-common-dark` | `afe79b29a25873e32fabc99ddcb7729ec6aab8faffcec317d88c0adfaca33424` |
| `h5-index` | `c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96` |
| `h5-collect` | `cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6` |
| `h5-mine` | `cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385` |
| `pc-calendar-dark` | `db2179ae8f542e1a349db9829c54e723ff8be4ce928a522f3da6b776950a4930` |
| `h5-message` | `170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4` |

共 21 组 × 2 = 42 张截图，全部像素一致。

## 6. 三分判定

- **可达真缺陷：0**。媒体查询 21/21 激活但 computedStyle 零差异、截图像素 21/21 完全一致、CDP 149 条规则 0 条 contrast media——「无可见变化」源于浏览器原生支持有限且项目无 contrast media 实现，属规范预期，非缺陷；
- **不可达 L1：0**（本轮无死代码/折叠区样本）；
- **正常-规范预期若干**：渐变文字/玻璃卡/焦点环/品牌按钮/半透明标签在 contrast:more 下与 baseline 像素一致且可读，未出现对比增强引起的退化。

据实判 0，严禁臆造。已覆盖维度（motion/focus/44px/骨架/R458 WCAG/R461 forced-colors）不重复判缺陷。

## 7. 真值纪律

server 域名 / 主机 / 端口 / 口令真值一律不入库，以 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；凭据仅在 gitignored runner 运行时经环境变量注入，真域名不回显。截图来源如实标注为 Chromium headless 非物理真机，未冒充物理设备。
