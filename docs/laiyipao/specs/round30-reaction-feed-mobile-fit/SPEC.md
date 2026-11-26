# R30 规范：窄屏反应行盒模型修复（round30-reaction-feed-mobile-fit）

> 目标：修复 R28/R29 连续审计登记的 390×844 既有基线缺陷：`.reaction-line` right=396 超出 390 视口 6px、`.reaction-feed` scrollWidth=386 > clientWidth=369。用最小 CSS 盒模型修正让长反应名/长归因在 feed 内换行，不隐藏、不截断任何信息。只动 `battle.vue` 移动端一条 CSS 规则，不改引擎/结算/后端/tokens，R24/R26/R27/R28/R29 全部共存，CTA 仍真实可点。

## 0 缺口依据（亲读带行号；不可编造）

- `docs/laiyipao/audits/round28-live-reaction-attribution/REPORT.md:83-91`：390 视口实测 feed right=380 / scrollWidth=386 / clientWidth=369；3 条 `.reaction-line` right=**396**（超视口 6px）、width=386、max-width=none；`.reaction-cue` right=341 未溢出；判定为 HEAD 基线既有问题，非 R28 引入，但必须单独修复。
- `laiyipao/miniapp/src/pages/battle/battle.vue:2280` 移动端媒体查询 `@media (max-width: 430px)`；:2300-2308 `.reaction-feed{gap:4rpx}`、`.reaction-line{flex-wrap:wrap;row-gap:2rpx;padding:4rpx 10rpx;width:100%}`——`width:100%` + 左右 padding + 1rpx 边框，且**无 `box-sizing:border-box`**（基线桌面 :2330-2339 同样无），拉伸出容器边界。
- `battle.vue:2323-2328` `.reaction-feed{display:flex;flex-direction:column;align-items:flex-start}`；`align-items:flex-start` 让子项按自身宽度而非拉伸宽度布局，`width:100%` 又强制内容盒等于容器宽 → 含 padding/border 后实际外宽超出。
- `battle.vue:2330-2339` 桌面 `.reaction-line{display:flex;flex-direction:row;gap:12rpx;padding:6rpx 14rpx;border:1rpx solid}`；:2346-2351 `.reaction-line-main` 同为 row flex；:2358-2363 `.reaction-cue{max-width:620rpx}` 且移动端仅改为 max-width:100%。
- `battle.vue:61-70` 模板：`.reaction-line` 包含 `.reaction-line-main`（reaction-name + damage）、`.r28-attribution`（桌面/全局 :2788 `white-space:nowrap`）、`.reaction-cue`，三者同一行 flex；移动端虽 `flex-wrap:wrap`，但 `.reaction-line-main` 内部不换行、桌面 `.r28-attribution` 的 nowrap 在移动端未覆盖、`.reaction-cue` 长文本，均未受最小宽度约束。
- `battle.vue:2787-2793` R28 `.r28-attribution` nowrap、`.r28-feed-ai` max-width:100%、`.r28-feed-cta` pointer-events:auto；:2794-2814 R29 抽屉 z-index:40；:2401-2404 R26 brief max-height:160rpx；:2762-2786 R27 容器紧凑。以上必须保持不回退。
- 根因裁决：内容盒模型溢出（width:100% + padding/border 无 border-box）叠加 flex 子项 min-width:auto 无法收缩。修复只在移动端媒体查询内补齐 `.reaction-line` 三条盒模型属性与各子元素约束，桌面/全局规则原样保留，不碰 JS/HTML。

## 1 CSS 修复契约（battle.vue 移动端媒体查询内）

- 在 `@media (max-width: 430px)` 块的 `.reaction-line` 规则中，保留既有 `flex-wrap:wrap; row-gap:2rpx; padding:4rpx 10rpx; width:100%`，并新增且逐字包含：
  - `box-sizing: border-box`：width:100% 含 padding 与 border，外宽严格等于 feed 内容宽；
  - `max-width: 100%`：即使内容极长也不越过父容器；
  - `min-width: 0`：允许作为 flex 子项收缩（覆盖 flex item 默认 min-width:auto）。
- 必须新增 `.reaction-line-main { min-width: 0; flex-wrap: wrap; }`（移动端块内，紧邻现有 `.reaction-line-main{gap:8rpx}`）：长 `{{ r.pair }} → {{ r.name }}` 与伤害可在子元素间换行，不撑破父行。
- 归因换行的层级裁决（消除原表述矛盾）：**桌面/全局 `.r28-attribution`（battle.vue:2788）的 `white-space: nowrap` 必须原样保留**，不修改、不删除；**仅在移动端媒体查询内新增 `.r28-attribution { white-space: normal; max-width: 100%; overflow-wrap: anywhere; }`**，用 `normal` 显式覆盖桌面 `nowrap`，让极长归因（含长英文/无空格串）可折行而不撑破行盒。禁止只在移动端写 max-width/overflow-wrap 却忘记覆盖 nowrap（nowrap 仍会撑破）；不 display:none、不 text-overflow:ellipsis 隐藏信息。
- `.reaction-cue` 保留既有 `max-width:100%`（移动端 :2316-2319），补 `overflow-wrap: anywhere` 以约束长 cue；模板 `.reaction-feed` / `.reaction-line` / `.r28-attribution` / `.r28-feed-cta` 结构一律不改。
- 移动端 `.reaction-name` / `.reaction-damage` 必须补齐 `min-width: 0` 与 `overflow-wrap: anywhere`：现有移动端 :2312-2315 只有 `font-size: 20rpx`（两条选择器共用一个规则），flex 子项默认 `min-width:auto` 时，长英文/无空格反应名或伤害数字串仍会撑破 `.reaction-line-main`；在移动端块内为两者加约束（可在共用规则或各自规则中），桌面基线 :2341-2356 不改。
- 禁止项：不得用 `overflow:hidden`、`text-overflow:ellipsis`、`display:none`、`white-space:nowrap` 追加到 `.reaction-line` 或子元素来掩盖溢出（唯一例外是桌面/全局既有 `.r28-attribution` nowrap 原样保留、移动端用 `white-space: normal` 覆盖）；不得改 `.hud-row` 的 `flex-wrap:wrap`（R9/R27 既有换行）；不得删除或重命名任何 R24/R26/R27/R28/R29 选择器与文案；不得改 tokens、不得加 `--r30-`。

## 2 390×844 QA 几何与证据（RED→GREEN 门禁）

- RED（修前）：契约测试对当前 battle.vue 断言缺失修复属性必然失败；命令见 §4。
- GREEN 判据（真浏览器 390×844，战斗 WAIT 态有 ≥3 条反应行时量测）：
  - 每条 `.reaction-line` 的 `getBoundingClientRect().right <= 390`，且 `width <= feed.clientWidth + 1`；
  - `.reaction-feed.scrollWidth <= .reaction-feed.clientWidth + 1`（feedOverflow=false）；
  - `document.documentElement.scrollWidth <= 390`（docOverflow=false）；
  - 长反应名/长归因行文本完整可读（不隐藏/不截断）；
  - R24 `.r24-element-hud`、R26 `.r26-wave-brief`、R27 `.r27-wave-progress`、R28 `.r28-attribution`/`.r28-feed-ai`、R29 `.r29-timeline-overlay` 均存在且布局不回退；
  - 真实点击 `.r28-feed-cta` 仍可打开完整反应时间线抽屉（R29），`.r28-feed-ai` pointer-events:none、CTA pointer-events:auto 不变。
- 截图/几何证据要求：在 `docs/laiyipao/audits/round30-reaction-feed-mobile-fit/` 下保存 390×844 战斗态截图与 `measure-eval.js` 输出的 JSON（feed rect + 每行 rect + scrollWidth/clientWidth），截图亲读确认非登录页/非结算页遮挡；sha 与审计记录写入后续实施报告。

## 3 AI 四判据与诚实红线

- 本轮为纯 CSS 布局修复，不新增 AI 文案；R28 AI 结论「依据：R15 reactionTimeline 逐次事件（wave/enemy_uid/reaction/damage）按事件发射顺序配对；截断或不匹配如实标注归因不可用」与红线「辅助归因非权威判据，权威以 replay_hash 为准」必须原样保留。
- 不修改任何战斗逻辑、伤害、结算、replay_hash、loot、score；不新增/改后端 API；不改 H5 数据流。
- 修复结论如实标注「基线既有问题修复」，不得宣称 R28 引入或把几何改善冒充玩法改动。

## 4 契约测试与验证命令（test-first）

- 契约测试 `laiyipao/miniapp/src/pages/battle/battle_r30_reaction_feed_mobile_fit.contract.test.ts` 直接读 `battle.vue`/SPEC/tokens 源码做静态守卫；当前源码无修复属性 → RED（exit 1，断言失败）。
- RED 命令（workdir=laiyipao/miniapp）：`set -o pipefail; npx vitest run --config vitest.config.ts src/pages/battle/battle_r30_reaction_feed_mobile_fit.contract.test.ts --reporter=verbose`。期望：测试失败，报告缺失 `box-sizing: border-box` / `max-width: 100%` / `min-width: 0` 等守卫。
- GREEN 命令（实施后同一命令）：全部断言通过 exit 0；再按 §2 真浏览器量测并留截图/几何证据。
- 契约测试须断言：SPEC 存在且含逐字修复属性与几何判据；battle.vue 移动端媒体查询内 `.reaction-line` 三条属性齐备；`.reaction-line-main`/`.r28-attribution`（含移动端 `white-space: normal` 覆盖桌面 nowrap）/`.reaction-name`/`.reaction-damage`/`.reaction-cue` 约束齐备；桌面/全局 `.r28-attribution` 的 nowrap 不得被改写；禁止隐藏/截断负断言；R9/R24/R26/R27/R28/R29/R29 CTA 与 tokens 锚点不回退。当前源码缺失这些修复属性 → RED，失败原因只能为缺失修复属性。

## 5 三端分工与不回退

- 本任务仅写本 SPEC + 契约测试；实施任务只改 `battle.vue` 移动端 CSS，不改 `engine.ts`/`step`/`replay_hash`/`score`/`settle`/`loot`，不改 server/admin，不动 R1–R29 任何既有行为与文案。
- R30 不新增令牌变量；tokens.css 三锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 原样保留，禁止 `--r30-`。
- t232 修订仅改本 SPEC 与契约测试：把「保留 nowrap 又允许折行」的矛盾表述改为「桌面/全局 nowrap 保留、移动端 white-space: normal 显式覆盖」，并补移动端 `.reaction-name`/`.reaction-damage` 的 min-width:0 + overflow-wrap:anywhere 约束；不改 battle.vue，RED 原因仍只能是缺失修复属性。