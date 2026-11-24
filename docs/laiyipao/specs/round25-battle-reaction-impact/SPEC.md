# R25 规范：单局战报反应伤害主力聚合卡

## 0. 缺口依据（亲读源码带行号）

- `laiyipao/miniapp/src/pages/battles/battles.vue:659-677`：`reactionTimeline` computed 已把 `detail.reaction_timeline.events[]` 逐条过滤校验（`wave`≥1 整数、`tick`≥0 整数、`reaction` 非空串、`enemy_uid` 整数、`damage` 有限数），按 wave/tick 排序后映射为带 `reactionName` 的展示行，渲染于 `.r15-reaction-timeline`（模板 `:234-245`）。
- 缺口：逐次时间线只做「第几波对谁触发」的流水，玩家看不到「本局哪个反应是伤害主力、占比多少」的聚合洞察。R15 逐条展示、R18（admin 跨局）/R19（玩家跨局）均不覆盖单局伤害主力下沉。
- 数据现成：`events[].damage` 与权威 `detail.reactions_used`（`Record<string, number>`，`client.ts:1324`），纯前端即可派生，无需新增后端/迁移/API。
- 反应名来源：`battles.vue:499-502` 的 `reactionNameMap` computed 取 `store.config?.reactions`（`/config.reactions`）的 key→name 映射；兜底原始 key，沿用引擎真相（fire+ice=steam_burst、ice+lightning=superconduct、fire+lightning=overheat）。

## 1. 纯前端派生（不落库、不进哈希）

- 新增 computed `reactionImpact`：只消费 `reactionTimeline.value`（已是校验后的干净事件）与 `detail.value?.reactions_used`。不做任何网络请求、不写引擎状态、不碰 `step()`/`replayHash()`/`score`/`loot`/结算。
- 按 reaction key 分组聚合：`total_damage = Σ events[].damage`（仅有限数参与），`events = 该组事件条数`，`count = reactions_used[key]`（**权威触发次数，严禁用 events 条数冒充**；缺值取 0 并如实标注）。
- `damage_share_pct = total_damage / Σ(全部组 total_damage) × 100`，`toFixed(1)` 展示。全部 `total_damage` 之和为 0 或负时占比一律归零（全部 0.0），**不产 NaN / Infinity / -0**。
- 排序：`total_damage` 降序（主力在前）；同伤害按 reaction key 字典序稳定排序。每行展示 `reactionName`（`reactionNameMap` 取名，兜底原始 key）。
- 与 R19 口径一致但维度不同：R19 是近 30 天跨局，R25 是单局；count 权威来源同为 `reactions_used`，events 仅为时间线事件条数，二者不可混用。

## 2. 诚实空态与边界

- 旧战报无 `reaction_timeline` 字段 / `events` 非数组 / 空数组 / 过滤后无有效行 → `reactionImpact` 为空数组，渲染 `.r25-impact-empty`，逐字文案「本局无逐次反应伤害记录」。
- 即使 `reactions_used` 有值但无 timeline 伤害行，也不反推波次/敌人/伤害、不编造主力行（与 R15 诚实红线一致）。
- `reaction_timeline.truncated=true` 时如实提示「已截断，仅统计前 200 条」（沿用 R15 口径），不把截断数据冒充全量。
- 展示性归因：events 条数与 damage 聚合仅供复盘参考，不参与结算、不改变 replay_hash，权威以 `replay_hash` 为准。

## 3. 选择器契约（H5 binding）

- 容器 `.r25-reaction-impact`：单局战报详情展开卡，紧跟 R15 逐次时间线块之后。
- 行 `.r25-impact-row`，每行带 `[data-r25-reaction]` 属性（值为原始 reaction key）：展示反应名、伤害总计、占比、权威次数、事件条数。
- 空态 `.r25-impact-empty`：逐字文案「本局无逐次反应伤害记录」。
- AI 注释 `.r25-ai-note`：逐字依据行「依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准」。

## 4. AI 四判据

- ① 可见汉字「AI」标签。
- ② 真实数据结论：基于本局实际聚合，示例「AI：本局蒸汽爆发造成 62.5% 反应伤害，是伤害主力」。
- ③ 依据行：逐字「依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准」。
- ④ 可执行 CTA：例如「下一局优先凑齐该元素组合」。

## 5. 诚实红线（逐字守卫）

- 「辅助展示非伤害权威,权威以 replay_hash 为准」。
- 该卡为只读派生展示：不改结算/replay_hash/score/loot；count 取权威 `reactions_used`，绝不用 events 条数冒充触发次数；无 timeline 不编造。

## 6. 契约测试设计（test-first）

- 文件：`laiyipao/miniapp/src/pages/battles/battles_r25_impact.contract.test.ts`。
- 纯参考实现 `aggregateReactionImpact(events, reactionsUsed, nameMap)` 按 §1 口径断言：多反应分组 total_damage/events/count 正确、占比求和约 100%、排序降序、SUM=0 不产 NaN、count 取权威值而非 events 条数、旧战报空数组走空态、reactionNameMap 取名兜底。
- 源码文本守卫：battles.vue 含五类选择器与两条逐字文案、沿用 `reactionTimeline`/`reactionNameMap`/`reactions_used`；tokens.css 三锚点 v1/v3-r3/r6 保留且无 `--r25-`；SPEC 含红线/依据行/AI 四判据关键词。
- 零真值：SPEC 与测试不得含域名/IP/JWT/口令/个人绝对路径。

## 7. 三端不回退与分工

- 令牌锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 保留，禁用 `--r25-` 新令牌。
- R15 逐次时间线块、R13/R14 反应建议、R18 admin 端点、R19 玩家跨局端点与既有契约不改；本规范只新增 R25 单局聚合卡契约。
- 分工：t194 只交付本 SPEC 与契约测试；H5 实施由后续任务完成；本轮零后端改动。