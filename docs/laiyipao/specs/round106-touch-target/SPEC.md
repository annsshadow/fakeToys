# R106 规范：次级可点击控件触摸目标契约

- 轮次：R106（test-first RED）｜任务：t297 ｜ attempt_id：34537dac-ec80-4e06-aac1-f0881fd72bf1
- 上游：t296 R106-B 侦察（docs/laiyipao/recon/ROUND106_B_TOUCH_TARGET_RECON.md），四条件全中（焦点环一支命题不成立、不纳入）。
- 基线 HEAD：`7911e9a016f370f5e5d5609788b3089a8dc95937`（全程未变）。
- 唯一写域：`docs/laiyipao/specs/round106-touch-target/`（SPEC.md / RED-evidence.md / RED-raw.log）+ `laiyipao/miniapp/src/styles/r106TouchTarget.contract.test.ts`。

## §0 四条件裁定

- ①真实可见缺陷：成立。t296 真 Chromium 三视口实测 index 四类元素 15-34 CSS px（390），低于 WCAG 2.5.5 的 44×44 CSS px，亦低于设计系统 ≥88rpx（§91/§211/§230）。
- ②展示层可修 + 同仓权威：成立。权威 docs/laiyipao/design/AI_VISUAL_SYSTEM.md §91（CTA 触控区 ≥88rpx×88rpx）、§211（chip min-height:88rpx）、§230（按钮与节点 ≥88rpx）；WCAG 2.5.5。以 CSS min-height 即可修正。
- ③改动面小：成立。仅 index.vue 四个既有 CSS 块各加/改一行 min-height，不碰语义、令牌、既有契约断言。
- ④真浏览器坐实：成立。t296 index 三视口完整尺寸探针 + battle/stage 跨页抽查。
- 焦点环一支：命题不成立（common.css:912-921 两层全局 `:focus-visible` 兜底已覆盖全部可 Tab 元素，真浏览器逐一聚焦 outline 3px 可见），本轮不纳入候选；契约以不变量锁定该兜底不得弱化。

## §1 修复对象（实施轮逐字口径，surgical 最小改动）

四类元素在 **index.vue 各自现有 CSS 块内** 追加/改写一行 `min-height: 88rpx;`，保持既有 display/flex-direction/align-items/gap/padding/font-size/color/border 声明原样不动，不新增块、不重构：

| # | 元素 | CSS 位置（基线） | 现有声明（节选） | 实施动作 |
| --- | --- | --- | --- | --- |
| 1 | `.mg-row` | index.vue:828 | `display:flex;flex-direction:row;align-items:center;gap:12rpx;padding:10rpx 0;border-bottom:1rpx solid var(--border-soft);` | 块内追加 `min-height:88rpx;` |
| 2 | `.elem-chip` | index.vue:1147-1152 | `display:flex;flex-direction:row;align-items:center;gap:8rpx;` | 块内追加 `min-height:88rpx;` |
| 3 | `.ann-all` | index.vue:1219-1223 | `font-size:22rpx;color:var(--fire);padding:4rpx 0 4rpx 16rpx;` | 块内追加 `min-height:88rpx;` |
| 4 | `.dim-hint` | index.vue:934-938 | `font-size:22rpx;padding:0;min-height:auto;` | **将 `min-height:auto;` 改写为 `min-height:88rpx;`**（auto 与 88rpx 互斥，若仅追加会被后者覆盖或被前者覆盖，故必须替换，不是纯追加） |

说明：队长修复口径记为「四类元素各追加 min-height:88rpx」，实施时须注意第 4 项 `.dim-hint` 基线已有 `min-height:auto`（index.vue:937），必须先删除该声明再写 88rpx，否则 CSS 后者覆盖前者会失效或语义冲突。契约第 5 个缺口 it 专门锁定「不得保留 min-height:auto」。

## §2 不变量（基线即绿，实施轮不得回退）

- 四类模板键盘三件套齐备且 `@click` 与 `@keydown.enter` 指向同一 handler（ann-all:109 goAnnouncements / dim-hint:189-193 go(d.hint) / mg-row:209-217 m.hint && go(m.hint) / elem-chip:306-314 go('/pages/reactions/reactions')）。
- common.css:912-921 两层 `:focus-visible` 兜底（`:where(...):focus-visible` + 通用 `:focus-visible`）保留 `outline:3rpx solid var(--accent)` 与 `outline-offset:2rpx`。
- 令牌三锚点 tokens.css:14-16（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`）原样。
- 业务 index.vue 零 `--r106-`。
- surgical：`.mg-row` 保留 display:flex 与 padding:10rpx 0；`.elem-chip` 保留 display:flex 与 gap:8rpx；`.ann-all`/`.dim-hint` 保留 font-size:22rpx 与既有 padding。
- 既有契约 r100ClickableView / r102IndexCtaKeyboard / r105TopbarOverflow 相关断言不得弱化（本契约以键盘三件套与 surgical 断言间接守住）。

## §3 契约结构

- 文件：`laiyipao/miniapp/src/styles/r106TouchTarget.contract.test.ts`，11 it = 5 缺口 + 6 不变量。
- 真实 `fs.readFileSync` 读 `pages/index/index.vue` / `styles/common.css` / `styles/tokens.css`，禁止内嵌实现副本。
- `cssBlock(src, selector)`：以唯一 `selector + " {"` 锚点定位 CSS 声明块，锚点非唯一返回 null 令测试自责（本轮在既有块内加属性、不新增同名块，故锚点保持唯一）。
- `tagBefore(src, needle, tag)`：以唯一 class 属性串锚点向前找开标签、向后找最近的 `>`，容忍跨行，不依赖行号。
- 缺口 5 it：四类 min-height:88rpx + `.dim-hint` 不得保留 min-height:auto。
- 不变量 6 it：键盘三件套/同 handler、焦点环兜底两层、令牌三锚点、零 --r106-、surgical 保留 ×2。

## §4 RED 验证口径

- 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r106TouchTarget.contract.test.ts --reporter=verbose`（默认配置，勿指定 vite.config.ts，否则 uni 插件报 src/manifest.json ENOENT）。
- 期望 RED：exit=1，5 failed（全为真实缺口）| 6 passed（全为不变量），无 PARSE_ERROR / 模块解析错误。
- GREEN 期望（实施轮）：11/11 通过；并复跑 r100ClickableView / r102IndexCtaKeyboard / r105TopbarOverflow 全绿。
- 实测结果见 RED-evidence.md 与 RED-raw.log。

## §5 诚实边界

- 本轮为规范 + RED，未改任何业务 vue / .gitignore / 令牌 / 其他契约；4 交付全部 UNTRACKED。
- 修复口径中 `.dim-hint` 的 `min-height:auto → 88rpx` 为替换而非追加，已在 §1 与契约缺口 it 显式锁定，避免实施歧义。
- 触控目标的 CSS px 达标仍须实施轮真浏览器复测（rpx 在 1280 宽屏换算约 0.28px/rpx，88rpx≈25 CSS px 可能仍低于 44px）；本契约锁定的是设计系统 rpx 声明口径（≥88rpx），WCAG 44 CSS px 实测属实施/复查轮验收项。
- 焦点环一支命题不成立，已如实排除，未纳入候选；本轮仅以不变量守护其不回退。
