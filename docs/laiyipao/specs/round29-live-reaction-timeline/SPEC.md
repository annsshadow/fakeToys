# R29 规范：完整反应时间线抽屉（live-reaction-timeline）

> 目标：修复 R28 CTA「查看完整反应时间线 →」实际打开战场图例（legend-overlay/openHelp）的语义错配；改为打开本局完整逐次反应时间线抽屉，纯只读派生，不改引擎权威。

## 0 缺口依据（亲读源码带行号）

- `laiyipao/miniapp/src/pages/battle/battle.vue:77`：R28 CTA `.r28-feed-cta` 仍写 `@click.stop="openHelp"`，点击后展示的是 `:177` 的 `.legend-overlay`（战场图例，`:180` 标题「战场图例」），与 CTA 文案「查看完整反应时间线 →」语义错配。
- `battle.vue:940-969`：`openHelp()/closeHelp()` 与 `helpPausedByPage` 是图例专用暂停记账；R29 不得复用/污染它，必须独立记账 `timelinePausedByDrawer`。
- `battle.vue:1603-1621`：`doPause(reason)` / `doResume()` 是唯一暂停入口；`paused` / `pausedByHide` 为 ref；`doPause` 在 `paused || settled` 时直接 return，`doResume` 恢复时清 `pausedByHide`。
- `laiyipao/miniapp/src/game/engine.ts:63-75`：`ReactionTimelineEvent {wave,tick,reaction:ReactionKey,enemy_uid,damage}`（上限 200，:75 `REACTION_TIMELINE_LIMIT`）；:782-785 `reactionTimeline()` 返回 `.slice()`；:823-826 `reactionTimelineTruncated()` 返回截断标记。两者只读旁路，不进 replay_hash、不影响结算。
- `laiyipao/miniapp/src/pages/battles/battles.vue:685-706`：R15 战报页展示口径 —— 合法性 `wave` 整数≥1、`tick` 整数≥0、`reaction` 非空字符串、`enemy_uid` 整数、`damage` 有限数；按 `wave`、`tick` 升序；反应名以 `store.config.reactions` 为准（`:525-530` reactionNameMap，未知 key 原样兜底）；空态「该战报无逐次反应记录（旧版本结算）」；截断「已截断，仅显示前 200 条」。
- `battle.vue:487` `const store = useGameStore()`；`laiyipao/miniapp/src/store/game.ts:33` `config: GameConfig`；`laiyipao/miniapp/src/api/client.ts:107-120` `GameConfig.reactions: Array<{key:string; name:string; ...}>` —— 反应名唯一权威是服务端配置。
- 结论：战斗页已采集完整时间线但无处可看；新增只读纯函数 + 抽屉即可闭环，禁止复用图例遮罩冒充时间线。

## 1 纯函数模块 reactionTimelineView.ts（新增）

- 路径：`laiyipao/miniapp/src/pages/battle/reactionTimelineView.ts`；具名导出 `buildReactionTimelineView(events, truncated, reactions)` 与常量 `R29_EMPTY_TEXT`、`R29_TRUNCATED_TEXT`、`R29_BASIS`、`R29_HONESTY`、`R29_UNKNOWN_SUFFIX`。
- 入参：`events: ReactionTimelineEvent[] | null | undefined`（来自 `eng.reactionTimeline()` 只读副本）；`truncated: boolean`；`reactions: Array<{key:string; name:string}> | null | undefined`（来自 `store.config.reactions`，服务端权威反应名）。
- 合法性过滤（逐条照抄 `battles.vue:688-694` 口径）：`wave` 整数且 ≥1；`tick` 整数且 ≥0；`reaction` 为非空字符串；`enemy_uid` 整数且 ≥1；`damage` 为有限数。任一不合格丢弃，不输出伪造数字；全程不修改传入数组。
- 排序：按 `wave` 升序，同波按 `tick` 升序；同 wave+tick 保持原输入稳定次序（不重排同刻事件）。
- 展示模型行 `{index, wave, tick, reaction, reactionName, enemy_uid, damage}`：`index` 为过滤排序后 0-based 序号；`reactionName` 从 `reactions` 的 `key` 映射取 `name`，未知 key 原样输出 key 并追加明确标记（常量 `R29_UNKNOWN_SUFFIX` = '（未知反应）'），绝不编造元素/伤害/波次。
- 返回 `{rows, empty, truncated}`，`rows` 为展示行数组；`empty = rows.length === 0`；`truncated` 原样透传供诚实标注。
- 纯只读派生：不写 engine/step/replay_hash/score/settle/loot；不触碰 `reactionTimelineEvents` 原数组；签名不含 waveIndex/currentWave。

## 2 逐字文案（诚实红线）

- `R29_EMPTY_TEXT` 逐字：「本局暂无逐次反应记录」。
- `R29_TRUNCATED_TEXT` 逐字：「已截断，仅显示前 200 条」。
- `R29_BASIS` 逐字：「依据：引擎 reactionTimeline() 只读旁路（前 200 条，不进 replay_hash）；反应名以服务端 config.reactions 为准」。
- `R29_HONESTY` 逐字：「辅助展示非权威判据，权威以 replay_hash 为准」。
- 空数组/全非法 → 只显示空态，不显示截断提示制造「有数据被截断」误导；仅 `truncated === true` 且 `rows.length > 0` 时显示截断标注。

## 3 抽屉 UI 契约（battle.vue 新增，不替代图例）

- 选择器：`.r29-timeline-overlay`（遮罩，点击空白处关闭）、`.r29-timeline-box`（内容框，阻止冒泡）、`.r29-timeline-head`（标题 + 关闭键）、`.r29-timeline-scroll`（可滚动列表）、`.r29-timeline-row`（每行带 `[data-r29-index]` / `[data-r29-wave]` / `[data-r29-enemy]` 属性）、`.r29-timeline-empty`、`.r29-timeline-truncated`。
- 标题可见且逐字含「AI · 完整反应时间线」；关闭键可见（×）；每行文本格式「第{wave}波 · {tick} · {reactionName} · #{enemy_uid} · {damage} 伤害」，数字一位小数（沿用 `toFixed(1)`），不产生 NaN/Infinity。
- 390×844：`scroll-view scroll-y` + `.r29-timeline-scroll { max-height: 70vh; overflow-y: auto; }`，长列表不溢出文档；`.r29-timeline-overlay` 显式 `z-index: 40`，高于 `.legend-overlay` 与 `.pause-overlay`（二者现为 `z-index: 30`，见 `battle.vue:1784/:1984`），也高于无 z-index（auto）的结算 `.sheet`（`battle.vue:2294`）——战斗打开时抽屉盖在暂停/图例层之上，结算后仍可查看且盖在结算 sheet 之上；标题与关闭键在各状态下均可见。`.r29-timeline-box` 显式 `pointer-events: auto`（父遮罩/顶部浮层为 none 时仍可点击）。
- 抽屉与图例在用户交互层互斥（不得同时打开）：`v-if="reactionTimelineVisible"` 与 `helpVisible` 不得同时为 true；`openReactionTimeline()` 先无条件调用 `closeHelp()`（函数声明 hoisting 可用），再按条件 `doPause()`；`openHelp()` 先无条件调用 `closeReactionTimeline()`（函数声明 hoisting 可用），再走原图例暂停逻辑。关闭函数仍只恢复自己的记账：`closeReactionTimeline()` 仅清 `timelinePausedByDrawer` 再 `doResume()`，`closeHelp()` 仅清 `helpPausedByPage` 再 `doResume()`。因 `paused` 是单布尔（`battle.vue:1561`），禁止依赖「两遮罩同时打开各自记账各自恢复」的旧假设；打开抽屉不得设置 `helpSeen`，不得把图例按钮标记为已读。

## 4 CTA 与暂停恢复边界

- 将 `battle.vue:77` 的 `.r28-feed-cta` 从 `@click.stop="openHelp"` 改为 `@click.stop="openReactionTimeline"`；文案仍逐字「查看完整反应时间线 →」；`.r28-feed-cta` 保持 `pointer-events: auto`，其余 R27/R24 指向 openHelp 的 CTA 不动。
- `openReactionTimeline()`：先构建展示模型（从 `eng.reactionTimeline()`/`reactionTimelineTruncated()`/`store.config.reactions` 派生），再置 `reactionTimelineVisible = true`；若 `!paused.value && !settled.value` 则 `doPause()` 并置 `timelinePausedByDrawer = true`（只记「由本抽屉暂停」）；已暂停或已结算只打开抽屉、不记账、更不得调用 `doResume()`。
- `closeReactionTimeline()`：置 `reactionTimelineVisible = false`；仅当 `timelinePausedByDrawer` 为真时清标记并调用 `doResume()` —— 只恢复本抽屉造成的暂停，不覆盖玩家原有暂停（手动/切后台）、倍速、图例暂停或帮助暂停。
- 已结算（`settled === true`）时仍可打开查看时间线；关闭时必须保持结算态，不得非法 resume（`doResume()` 内部已有 `!paused` 守卫，但调用方也须判 `timelinePausedByDrawer` 才调）。
- **结算态真实用户入口（t227 硬缺口）**：结算结果 `sheet` 内必须有可见可点、无需 DOM/eval 操作的 `.r29-result-timeline-cta`，文案逐字「查看完整反应时间线 →」，`@click.stop="openReactionTimeline"`，点击后 `.r29-timeline-overlay` 打开且 `z-index:40` 覆盖结果 sheet；保留战斗态 `.r28-feed-cta` 原入口，二者均为真实用户可达路径，结算 sheet 遮挡战斗 feed 时结果页入口是唯一可达路径。
- 与 `helpPausedByPage` 记账隔离且交互互斥：图例关闭只动图例的暂停记账，抽屉关闭只动抽屉的暂停记账；二者不可同时打开（见 §3），由开关函数先关对方再开自己保证。抽屉的 `openReactionTimeline()` / `closeReactionTimeline()` 函数体不得直接读写 `helpVisible` / `helpPausedByPage`（只允许调用 `closeHelp()`），也不得读写 `speed` / 调用 `setSpeed()`，倍速档位完全不受影响。图例 `openHelp()` 先无条件 `closeReactionTimeline()`（hoisting 可用），再按原逻辑写 `helpVisible`/`helpPausedByPage`。

## 5 AI 四判据

- ① 标题含可见汉字「AI」（「AI · 完整反应时间线」）。
- ② 真实数据结论：抽屉顶部结论的 **Vue 模板必须逐字为「本局显示 {{ rows.length }} 条逐次反应记录」**（双花括号 Vue 插值；本文中的 `{rows.length}` 仅为说明占位，绝不可直接写进 UI 源码，单花括号字面会被 Vue 当作普通文本）。`rows.length` 为合法过滤后的展示行数，非引擎原始事件数、非累计触发次数。空态用 `v-if` 独立分支只显示 `R29_EMPTY_TEXT`「本局暂无逐次反应记录」，非空才显示上述结论；禁止出现「本局共」这类会把过滤/截断数据夸大为全局总数的措辞。截断仅按 `R29_TRUNCATED_TEXT` 提示，不补数据、不把被截断条数计入 `rows.length`。
- ③ 依据行逐字 `R29_BASIS`。
- ④ CTA：R28 feed 的 `.r28-feed-cta` 指向 `openReactionTimeline`（可点击性由 R28 的 pointer-events:auto 保证）。
- 诚实红线逐字 `R29_HONESTY`；截断仅提示不补数据；未知反应名明确标注，不编造。

## 6 契约测试设计（test-first，真实模块导入 + RED）

- 文件：`laiyipao/miniapp/src/pages/battle/battle_r29_live_reaction_timeline.contract.test.ts`；**必须从 `./reactionTimelineView` 真实导入**（禁止内嵌参考实现，t204 教训）；模块不存在时 vitest exit=1、`ERR_MODULE_NOT_FOUND`、Test Files 1 failed / Tests no tests，保留 RED 证据。
- 纯函数覆盖：正序（wave/tick 升序）、非法记录过滤（wave<1/tick<0/空 reaction/uid<1/damage NaN）、未知反应名标注、空态、截断标注、不修改入参、稳定同刻顺序。
- 源码守卫（battle.vue/engine.ts/tokens.css）：`.r29-*` 选择器与逐字文案；CTA 改为 `openReactionTimeline` 且不再 `openHelp`；`openReactionTimeline/closeReactionTimeline` 真实调用 `doPause/doResume` 且 `timelinePausedByDrawer` 记账；互斥守卫：`openReactionTimeline` 调用 `closeHelp()`、`openHelp` 调用 `closeReactionTimeline()`，抽屉开关块不直接读写 `helpVisible`/`helpPausedByPage`/`speed`；AI 结论使用 Vue 插值逐字「本局显示 {{ rows.length }} 条逐次反应记录」且不含「本局共」，空态独立 `v-if` 分支；CSS `pointer-events:auto` + `max-height`/`overflow-y` + `.r29-timeline-overlay` 的 `z-index:40`；engine.ts 只读 API 存在且未被反向写入；tokens 三锚点 v1/v3-r3/r6 保留、无 `--r29-`；R28 CTA 文案保留。
- 零真值：SPEC 与测试不含域名/IP/JWT/口令/个人绝对路径。

## 7 三端不回退与分工

- R15 时间线、R24 元素层、R26 敌情、R27 波次、R28 归因均不删不改其权威口径；零后端改动。
- 令牌锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 保留，禁用 `--r29-`。
- t222 只交付本 SPEC 与契约测试（RED）；纯函数模块与 battle.vue 抽屉渲染/暂停记账由后续实施任务完成。t223 仅修订本 SPEC 与契约测试（互斥遮罩、暂停所有权单布尔、z-index:40、AI 结论 rows.length 口径），不写业务实现、不动 battle.vue/engine.ts。t224 再修正两处契约精度：AI 结论按 Vue 插值逐字「本局显示 {{ rows.length }} 条逐次反应记录」断言（空态独立 v-if、非空才显示），函数体改用平衡花括号 extractFunctionBody 提取以免 slice 到 EOF 误伤后续函数/CSS；仍不写业务实现。