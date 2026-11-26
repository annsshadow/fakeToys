# R28 规范：实时反应 feed 波次/敌人归因（round28-live-reaction-attribution）

> 目标：增强 R9 现有 `.reaction-feed`（不是新起重复卡片），在 390×844 下每条反应清楚显示「第N波 · #enemy_uid」；无法可靠匹配时诚实显示「归因不可用」。纯只读派生，不改引擎权威。

## 0 缺口依据（亲读源码带行号）

- `laiyipao/miniapp/src/game/engine.ts:1801-1836`：每次反应除 emit R9 事件外，R15 旁路 push 一条 `ReactionTimelineEvent {wave:this.waveIndex+1, tick:this.tick, reaction:res.reaction, enemy_uid:e.uid, damage:Number(res.reactionDamage)}`；:1806 `length < REACTION_TIMELINE_LIMIT(200)` 才写入，:1814-1816 超出置 `didTruncateReactionTimeline`。
- `engine.ts:63-75` `ReactionTimelineEvent` 五字段（wave≥1 / tick≥0 / reaction / enemy_uid≥1 / damage）与上限 200；:782-785 `reactionTimeline()` 返回 `.slice()` 副本；:823-826 `reactionTimelineTruncated()` 返回截断标记。**两者都是只读旁路，不进 replay_hash、不影响结算**。
- `laiyipao/miniapp/src/pages/battle/battle.vue:61-70` R9 `.reaction-feed` 只显示 pair→name、damage、`AI cue`，无波次/敌人；:571-573 `ReactionLog {id,key,name,pair,cue,damage,resisted}`；:1034-1064 `noteReactions(events)` 取 `slice(-3)` 生成 `fresh`、`reverse()` 后 `slice(0, 3)`，仅 3 条上限。
- `battle.test.ts:1268-1350` 钉死 R9 现有 name/pair/cue/damage/抗性/「元素组合未上报」行为，必须原样保留。
- 结论：波次/敌人信息在 R15 timeline 里已存在但 feed 未消费；新增纯函数把本次 reaction 事件与 timeline 逐条配对即可补齐，禁止用当前 `waveIndex` 伪造归因。

## 1 纯函数模块 liveReactionFeed.ts（新增）

- 路径：`laiyipao/miniapp/src/pages/battle/liveReactionFeed.ts`；具名导出 `attributeLiveReactions(events, timeline, truncated)`、`attributionText(row)`、`buildLiveFeedAi(result)`，并导出常量 `ATTRIBUTION_UNAVAILABLE`、`LIVE_FEED_BASIS`、`LIVE_FEED_HONESTY`。
- 入参 events：本次 step 的 reaction 事件数组（每项至少 `{reaction, damage}`，damage 允许 bigint/number）；timeline：`ReactionTimelineEvent[]` 或 null/undefined；truncated：`reactionTimelineTruncated()` 布尔值。**签名不含 waveIndex/currentWave**：归因只来自 timeline 记录，绝不拿当前波次冒充。
- 返回 `LiveFeedResult {rows: ReactionAttribution[], matchedCount, unavailableCount}`；每条 `ReactionAttribution {reaction, damage:number, wave:number|null, enemy_uid:number|null, available:boolean}`，rows 与输入 events 顺序一一对应。

## 2 匹配算法（顺序 + reaction + damage + 合法性 + 游标消费）

- 截断优先：`truncated === true` 时 timeline 尾部已丢弃，而 feed 展示的正是最新事件 → 全部 rows `available=false`、`wave=null`、`enemy_uid=null`（诚实不可用），不得从残留记录拼凑。
- 空 timeline（null/undefined/非数组/长度 0）→ 全部不可用。
- 顺序匹配：以 timeline **尾部为游标**、按 events **发射顺序逆序消费**（最新事件配最新记录）：对每个事件，从当前游标向前找最后一条 `reaction` 相同且 `Number(damage)` 严格相等的记录；**一旦 reaction+damage 命中就立即把游标移到该索引−1（消费该记录）**，随后才做合法性校验。即使该记录 wave/enemy_uid 非法而使本行不可用，游标也已被消费，绝不能被后一个相同 reaction+damage 事件重复占用。这样「多事件同 reaction 同 damage」仍能一一对应、不串位；绝不用正序从头配（会把最新事件错配到最早记录）。
- 不匹配（reaction 不同或 damage 不等或游标已耗尽）→ 该行不可用；不为凑数放宽 damage 容差、不按 reaction 单独兜底。
- 合法性校验（在游标消费之后执行，不改变已消费的游标）：命中记录的 `wave` 必须是整数且 ≥1，`enemy_uid` 必须是整数且 ≥1，`reaction` 非空字符串，`damage` 有限数；任一非法 → 该行不可用（不输出伪造/越界数字），覆盖「非法 wave/enemy_uid」。
- 全部数值来自 timeline 记录；`available=true` 时 wave/enemy_uid 必为合法整数，否则 null。
- 输入顺序约定：函数接收**本次 step 的反应事件正序数组**（即 battle.vue `noteReactions` 中 `reactions.slice(-3)` 之后、`.reverse()` 之前的顺序），`rows` 与该数组下标一一对应；battle.vue 把归因文本附加到同序 `fresh` 上后再一起 `reverse()` 渲染，确保归因随事件同行、不串位。
- **非法游标消费用例（必须可 RED）**：timeline 尾部是最新记录且 wave=0（非法）、较早记录 wave=1/enemy_uid=5（合法），events 为两个相同 reaction+damage → 最新事件配非法记录（不可用、wave/uid 均为 null），较早事件配合法记录（wave=1、uid=5）。若实现「遇非法就跳过、不消费游标」，两个事件会抢同一条合法记录，此用例必失败。

## 3 展示契约（增强现有 feed，不另起卡片）

- 保留 R9 现有 `.reaction-feed` / `.reaction-line` / `.reaction-line-main` / `.reaction-name` / `.reaction-damage` / `.reaction-cue` 及 `AI {cue}`、name/pair/damage/resisted、`slice(0, 3)` 三条上限（battle.test.ts:1268-1350 不回退）。
- 每条反应行内新增文字 `.r28-attribution`：可用时逐字格式「第N波 · #enemy_uid」（数字来自归因），不可用时逐字「归因不可用」。
- `.r28-attribution` 带 `[data-r28-wave]`（可用时为 wave，否则 `unknown`）与 `[data-r28-enemy]`（可用时为 enemy_uid，否则 `unknown`）属性，供 E2E 锚定。
- 在 `.reaction-feed` 内三行之后追加**一行**紧凑 AI 归因说明 `.r28-feed-ai`（不新建独立卡片），必须同时包含四要素：① 可见汉字「AI」；② 逐字依据行（下行）；③ 逐字红线（下行）；④ CTA 逐字「查看完整反应时间线 →」，且该 CTA 必须是 `.r28-feed-ai` 内的子元素 `.r28-feed-cta`，绑定 `@click.stop="openHelp"`（`.stop` 阻止冒泡到画布）。
- 依据行逐字：`依据：R15 reactionTimeline 逐次事件（wave/enemy_uid/reaction/damage）按事件发射顺序配对；截断或不匹配如实标注归因不可用`。
- 红线逐字：`辅助归因非权威判据，权威以 replay_hash 为准`。
- CSS：`.r28-attribution` 行内紧凑，必须 `white-space: nowrap` 且 `font-size ≤ 20rpx`；`.r28-feed-ai` 必须 `max-width: 100%`、`pointer-events: none` 且**允许换行**（不得 `white-space: nowrap`），说明/红线不拦截画布点击；`.r28-feed-cta` 必须显式 `pointer-events: auto`（父级 `.r28-feed-ai` 与 `.overlay-top` 均为 `pointer-events: none`，子元素不覆盖为 auto 则 CTA 不可点击，R24/R27 同款做法）；CTA 另带 `@click.stop="openHelp"` 阻止冒泡；沿用 `.overlay-top` 的 `pointer-events: none`（battle.vue:1618/1996）；不挤压防线/波次/击杀/星级/钳分/R24/R26/R27。

## 4 AI 四判据

- ① 可见汉字「AI」（`.r28-feed-ai` 与既有 `.reaction-cue` 的 `AI`）。
- ② 真实数据结论：`buildLiveFeedAi(result).conclusion` 必须来自真实 `matchedCount/unavailableCount`（例「已归因 2/3 条最近反应」「最近无反应记录可归因」），不得写死数字。
- ③ 依据行：`LIVE_FEED_BASIS`（§3 逐字）。
- ④ CTA：逐字「查看完整反应时间线 →」且绑定 `@click.stop="openHelp"`（见 §3）；CTA 元素 `.r28-feed-cta` 必须 `pointer-events: auto` 才可在 `.overlay-top` 下点击。

## 5 诚实红线（不可协商）

- `辅助归因非权威判据，权威以 replay_hash 为准`。
- 纯只读：不改 engine emit/record/step/replay_hash/score/settle/loot；timeline 与截断标记均来自现有只读旁路 API。
- 截断、空 timeline、不匹配、非法 wave/enemy_uid 一律走 `归因不可用`，绝不编造波次/敌人、绝不拿当前 waveIndex 顶替。

## 6 契约测试设计（test-first，真实模块导入 + RED）

- 文件：`laiyipao/miniapp/src/pages/battle/battle_r28_live_reaction_attribution.contract.test.ts`。
- **必须从 `./liveReactionFeed` 导入真实函数**（禁止内嵌参考实现，t204 教训）；覆盖：多事件同 reaction 同 damage 顺序匹配、不匹配、空 timeline、truncated 截断不可用、非法 wave/enemy_uid、非法游标消费（最新非法/较早合法/两同事件）、attributionText 格式、buildLiveFeedAi 真实计数。
- 源码守卫（t215 补强）：battle.vue 含 R28 选择器/数据属性/逐字文案；对 `attributeLiveReactions(` / `attributionText(` / `buildLiveFeedAi(` 用**调用正则**断言真实调用（仅 import 不算）；`.r28-feed-ai` 四要素与 CTA 逐字「查看完整反应时间线 →」及 `@click.stop="openHelp"` 钉死；CTA 子元素 `.r28-feed-cta` 必须 `pointer-events: auto` 且 `@click.stop="openHelp"`（可点击性门禁）；CSS 精确守卫 `.r28-attribution` 的 nowrap + font-size≤20rpx、`.r28-feed-ai` 的 max-width:100% + pointer-events:none + 可换行、`.r28-feed-cta` 的 pointer-events:auto；保留 R9 三条上限与字段；engine.ts 含只读 API 与上限 200 且未被新模块反向写入；tokens 三锚点保留且无 `--r28-`。
- **RED 门禁**：模块不存在时 vitest 退出码 1（`Cannot find module ./liveReactionFeed`、`ERR_MODULE_NOT_FOUND`、Test Files 1 failed / Tests no tests），须保留逐字失败证据；t214 只交付 RED，不建业务实现。
- 零真值：SPEC 与测试不含域名/IP/JWT/口令/个人绝对路径。

## 7 三端不回退与分工

- 令牌锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 保留，禁用 `--r28-`。
- R9 feed、R15 timeline、R24 元素层、R26 下一波简报、R27 波次推进共存不回退；零后端改动。
- t214 交付本 SPEC 与契约测试（RED）；t215 补强非法游标消费用例、真实调用正则、AI 说明区块四要素与 CTA、CSS 精确守卫；t216 补强 CTA 可点击性：`.r28-feed-ai` 保持 `pointer-events: none`、`.r28-feed-cta` 显式 `pointer-events: auto` 且 `@click.stop="openHelp"`（R24/R27 同款做法，防 `.overlay-top` 拦截与冒泡）；纯函数模块与 battle.vue 渲染由后续实施任务完成。
