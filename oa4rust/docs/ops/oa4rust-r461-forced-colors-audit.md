# R461 forced-colors 高对比度 AI 视觉实证甄别

> 甄别轮次：t320（Kind=work，只读甄别，不改码）  
> 真父：`f22a90176c8e952780019b3294d2f0d6da9befe1`  
> 证据目录：`oa4rust/test-results/r461-forced-colors/`（21 PNG + `report.json`）  
> 来源标注：Playwright Chromium headless + `emulateMedia({forcedColors:"active"})` 真实页面；PC 1440x900 DPR1 / H5 390x844 DPR2 isMobile/hasTouch；**NOT a physical device，非物理真机**

## 1. 审计范围与维度声明

全库 `forced-colors|forcedColors|ForcedColors` 命中 **0**，属未覆盖空白维度。本轮以 Windows 高对比度 forced-colors active 实证 AI 风格在强制颜色模式下的可读性，重点：

- PC `.ai-gradient-text`（`color:transparent` + `background-clip:text`）标题/Logo 是否可读；
- 玻璃卡 `.ai-panel` / `.glass-card` 边框与内容可辨性；
- 焦点环、骨架屏、品牌按钮、H5 品牌渐变卡；
- CDP `CSS.getMatchedStylesForNode` 坐实命中规则；
- 三分判定仅以「真实不可读退化」判可达真缺陷，0 缺陷据实判 0。

已覆盖维度严禁重复：`prefers-reduced-motion` 与 `:focus-visible` 已在 `dark.css` + 契约钉死，本轮焦点环系统蓝可见仅作附带记录；骨架屏已在 R453/R456 穷尽，本轮 0 命中非缺陷。

### 静态高风险消费面

- PC `.ai-gradient-text` 共 40 命中 / 21 页（CalendarApp、BamApp、CmsDictDesignerApp、BBSForum、CmsIndexApp、ConfigDesignerApp、ControlPanelApp、Dashboard、DeploymentApp、FileManager、LogViewerApp、MeetingApp、NoteApp、OrgDirectoryApp、PortalScriptDesignerApp、ProcessManagerApp、ProcessTaskCenterApp、ProcessWork、RecycleApp、RoleManager、ThreeMemberApp）；
- `.glass-card|ai-panel` 共 392 命中；H5 `.ai-brand-fill` 25 命中；PC `ai-shimmer` 45 命中；
- token 定义 `oa4rust/frontend/packages/ui/src/theme/ai-tokens.css:109-112`：`.ai-gradient-text{background:var(--ai-gradient-text);-webkit-background-clip:text;background-clip:text}`。

## 2. 实测方法与样本

E2E 脚本（gitignored）：`oa4rust/test-results/r461-e2e.cjs`、`r461-e2e-run.sh`；凭据经 ssh 运行时从受保护凭据文件读取注入环境变量，域名真值只在 gitignored runner，不入库。

登录范式：PC `input[placeholder*="用户名"]` + `getByRole(/登/)`；H5 `input[type=text]` / `input[type=password]` + `uni-button` filter `/登/`。

**关键实现坑**：Playwright `newContext` 不支持 `forcedColors` 选项，改用 `page.emulateMedia({forcedColors:"active"})`（PC/H5 各一处）。

采样字段：`color` / `bgColor` / `bgImage` / `clip` / `fill` / `border` / `backdrop` / `forcedColorAdjust` / `disabled` / `text`；CDP 先 `DOM.enable` + `CSS.enable` 再 `getMatchedStylesForNode`。

实测结果：`R461_DONE pc=16 h5=5 cdpPC=8 cdpH5=5 errors=0`，`ERRORS=[]`；21/21 组 `forcedColors=true` 坐实。PC 8 页（dashboard/org/file/bbs/calendar/meeting/attendance/common）× 双主题 16 组；H5 5 页（index/mine/message/calendar/collect）锁浅色。

## 3. 决定性发现：渐变文字系统色补偿（勿误判）

forced-colors 下所有 `.ai-gradient-text` 元素 computedStyle 报：`color=rgb(0,0,0)`、`bgImage=none`、`clip=text`、`fill=rgba(0,0,0,0)` —— 单看声明像是「透明文字 + 无渐变 = 不可读」。**但 4 张关键截图目视证实文字全部清晰可读（系统黑色）**："早安，admin" / "公共组件库" / "会议管理" / OA4RUST logo 均清晰。

浏览器按 forced-colors 规范用系统色 CanvasText 补偿渲染：虽 computedStyle 的 `color` 与 `-webkit-text-fill-color` 报黑/透明，实际字形由系统色绘制。判**规范预期非缺陷**，严禁凭 computedStyle 误判为 P 级。

CDP 坐实（`r461-cdp-analyze.cjs`）：common / meeting / dashboard 的 `.ai-gradient-text` 命中规则：

```css
.ai-gradient-text { background: var(--ai-gradient-text); color: rgba(0, 0, 0, 0); -webkit-background-clip: text; background-clip: text; }
.logo-text[data-v-c6e2f9bf] { /* 侧栏 Logo */ }
```

`adminTitle` 命中 h1 UA 规则 + `.ai-gradient-text`；forced-colors 层覆盖为系统色渲染。

## 4. 逐角色实测摘要

| 角色 | PC 采样 | H5 采样 | 判定 |
|---|---|---|---|
| 渐变文字 `.ai-gradient-text` | color=rgb(0,0,0) / bgImage=none / clip=text / fill=transparent，截图清晰黑字 | gradient-text 零消费 | 规范预期，非缺陷 |
| 玻璃卡 `.ai-panel`/`.glass-card` | border=1px solid rgb(0,0,0)（系统色），bgImage=none；org/bbs backdrop=blur(12px)，dashboard/file backdrop=none | — | 内容/控件边框可辨，规范预期 |
| 焦点环 `focusTarget` | tag=A，border=1px solid rgb(0,0,159)，color=rgb(0,0,159)（系统高对比蓝） | — | 焦点可见，附带确认 `:focus-visible` 不退化 |
| 骨架 `skeleton` | 全页 0 命中（本轮未 route 延迟触发） | — | R453/R456 已穷尽，非缺陷 |
| 品牌 `.ai-brand-fill` | — | index/mine/calendar/collect bg=none / border=0px none，文字图标可读（stat "8"/"1" 清晰）；message 无该元素 | 系统色替换渐变，规范预期 |
| 按钮 | — | mine「开启推送」「保存新密码」文字黑色可读 | 规范预期 |

## 5. 截图 sha256（命令行 `sha256sum *.png | sort` 原始值）

| 截图 | sha256 |
|---|---|
| `pc-meeting-light-forced.png` | `1e862f0f07d8f2975a2dc2ecc07dcf5ada2f41461a998a2d4d0fb45169dae2e7` |
| `pc-attendance-dark-forced.png` | `398c296afcd6db66818fb48f1739e87c5766547d66a593f46ec9c823e77c1c2c` |
| `pc-bbs-light-forced.png` | `3bfcb824aac48e519921a544376d8d940c8c8c98f40cfcf7cb1704045e61d096` |
| `pc-dashboard-light-forced.png` | `3f1a989e8f4922af26984fcfb51ba3da46fb1d9aa2def0b85f8b728c77057235` |
| `pc-file-light-forced.png` | `46a62315549f76283568483cbb523629767c5b82bc64caecf2a225b0f79be5c3` |
| `h5-calendar-forced.png` | `4cf2f818f74a254b12c1500aa3483dd86c839c7491e902f17d86d6a9092504fd` |
| `pc-bbs-dark-forced.png` | `5499e686c32399f41e04266130aa966141048a5a25cdfb173ce00ece11d3e962` |
| `pc-calendar-light-forced.png` | `5ffa47e765e5e5074687f62edf3e6efaea50ba436a48c9dc27fd435b9b087d10` |
| `pc-dashboard-dark-forced.png` | `6bf3dc6b9c917c7876d6187285b2c619c5b9b4efd1e0f8e7ccf20ea8abecc051` |
| `h5-mine-forced.png` | `7214438ccb35fd3cdc2a0c2f04eb4e39a89e42da2cf4dfddc7116605c000c556` |
| `pc-attendance-light-forced.png` | `76bad1b1a181952d028431814337013138c601f5ff473547afb9bc9911e1d3b4` |
| `pc-org-light-forced.png` | `78abb4302f6d7fbd673a6f1a20aa69a0b803e9376f6e9ee896c7cd9b83565b7c` |
| `pc-file-dark-forced.png` | `917698c09a96c7425e5fc97d784a3373f6cc8033fc55878c4789729aa8f6c3af` |
| `pc-common-light-forced.png` | `9c52d95576402919cd600f98b256a1b10268c023f2f5d4097ebf1fe1acf1fd24` |
| `pc-org-dark-forced.png` | `b8671e2f15a77518050e3a39f775d49988d3fe4274a80c5d7f6ab181b10e4801` |
| `h5-collect-forced.png` | `bcec6cf609a64aae44d340a668c688bfcb01c8eb78bd23dd2e98c01d390f424e` |
| `h5-index-forced.png` | `c320fa7da770333b56ef465e605123db34e5e9b772862672895117071a463070` |
| `pc-common-dark-forced.png` | `d4277b7da8621cb793735fa7eefd7134999fc0e4dbfc9c69b59f50650093f141` |
| `pc-calendar-dark-forced.png` | `e1790706dee1256bc370d27808687717fa807dc019708723fd11377d8562f606` |
| `h5-message-forced.png` | `e80553e45cd61179cf94e35cf6cc459e6aa7ee8c7a3ddd88e320be72d8d5a5d8` |
| `pc-meeting-dark-forced.png` | `f8ba7dfeb48d40572a3546133797b366af14acc48a31b48d08a229e9b3125669` |

## 6. 三分判定

- **可达真缺陷：0**。所有渐变文字、玻璃卡、品牌按钮、焦点环在 forced-colors active 下真实可读，属规范预期；
- **不可达 L1：0**（本轮无死代码 / 折叠区样本）；
- **正常-规范预期若干**：渐变文字系统色补偿、玻璃系统色边框、品牌渐变替换、焦点系统蓝环、H5 品牌卡文字可读、骨架 0 命中（已由 R453/R456 覆盖）。

据实判 0，不臆造缺陷。附带确认 `prefers-reduced-motion` 与 `:focus-visible` 由 `dark.css` + 契约覆盖，本轮焦点环系统蓝可见不退化。

## 7. 真值纪律

server 域名 / 主机 / 端口 / 口令真值一律不入库，以 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；凭据仅在 gitignored runner 运行时经环境变量注入。截图来源如实标注为 headless 非物理真机，未冒充物理设备。
