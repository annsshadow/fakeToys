# R24 规范：战斗实时元素层与反应触发预判 HUD

## 0. 缺口依据（亲读源码）

- `laiyipao/miniapp/src/game/engine.ts:435-437`：`BattleConfig.enemies: Map<number, EnemyDef>` 是关卡静态敌种表；运行时 `this.enemies` 是活动实体数组（`:1032 this.enemies.push(e)`、`:1147/:1228/:2062 enemies:this.enemies`）。
- `laiyipao/miniapp/src/game/types.ts:153-171`：活动敌人 `Enemy.uid/defId/name/category/hp/maxHp/shield/stacks: Map<Element,bigint>/applyElement(e,add,cap)`；元素层数是 `Enemy.stacks`，按元素键存 bigint。
- `laiyipao/miniapp/src/game/engine.ts:2028`：关卡 `element_cap` 与局内 buff 合成当前元素层上限，HUD 展示层数时只读取、不修改。
- 战斗进行中玩家看不到敌人身上元素层数，也看不到「再补某元素可触发什么反应」；此前只有选卡/备战态的事前预判（R22/R23），缺临场信息。

## 1. 只读快照（引擎侧）

- 新增只读快照方法 `elementLayerHud()`，返回当前场上存活敌人的浅拷贝快照数组（不暴露内部 Map/Enemy 引用）：每项含 `uid`、`defId`、`name`、`hp`、`maxHp`、`stacks`（`Record<Element, number>`，仅列层数>0 的元素）。
- 照抄 R15 只读范例 `laiyipao/miniapp/src/game/engine.ts:771-779` 的 `reactionTimeline(): ReactionTimelineEvent[] { return this.reactionTimelineEvents.slice() }`：返回切片/副本，不写任何参与 replay_hash 的字段。
- `laiyipao/miniapp/src/game/engine.ts:868-879` 的 `replayHash()` prefix 由 `L|A(attack/crit/critMultiplier/reactionMult/elementCap/reactionTier/elementCoef)|S(skills)|X` 段构成。R24 快照不得触碰 `step()`、`replayHash()`、`buildSnapshotSegment()`、`score`、结算与伤害；新增代码只能出现在只读方法中。
- 契约测试以源码文本守卫：引擎必须出现 `elementLayerHud` 方法名，且方法体用 `slice()` / `map()` 等复制，不得出现对这些字段的赋值。

## 2. 派生算法（H5 侧，纯函数）

- 输入：只读快照数组 + `laiyipao/miniapp/src/utils/reactionPairs.ts:15-25` 的 `REACTION_PAIRS`（由 `lookupReaction` 对 ELEMENT_ORDER 五元素两两穷举反查派生，纯静态数据不落库不进哈希）+ `ELEMENT_NAME`（焰/冰/电/毒/动能，`elements.ts:21-27`）。
- 对每个敌人，把 `stacks` 中层数>0 的元素视为已附着元素集合 E。
- `hintsForEnemy(E)`：对每个 `pair=[existing,incoming]`，若 `existing ∈ E` 且 `incoming ∉ E`，则调用 `lookupReaction(existing,incoming)` 得到反应 key，收集去重。同一反应可由多个 pair 命中时合并为一条提示。数据结构 `hint = {reaction, existing, incoming}`。
- 关键真相（引擎 `elements.ts:290-332`）：`fire+ice=steam_burst`（蒸汽爆发）、`ice+lightning=superconduct`（超导）、`fire+lightning=overheat`（过热）。已附着焰、再补冰 → 蒸汽爆发，不是超导；超导要求已附着冰再补电（或已附着电再补冰）。规范与示例一律以此为准，不得沿用错误草稿。
- `lookupReaction(existing,incoming)` 在 `elements.ts:328` 有 `if(!existing||!incoming||existing===incoming)return null`：同元素不触发；E 为空时 `hintsForEnemy([])` 返回空数组，走诚实空态，不编造提示。
- E 含全部五元素时无 `incoming ∉ E`，提示为空，走诚实空态。
- 血量可选展示：`hp/maxHp` 用 `Number(...)` 后 `toFixed(1)` 展示（跟随 admin/H5 数值惯例），但不得据血量推算伤害或结算。

## 3. 选择器契约（H5 binding）

- 容器 `.r24-element-hud`：战斗页 tick 后刷新的只读 HUD。
- 敌人行 `.r24-enemy-row`，每行带 `[data-r24-uid]` 属性（值为敌人 uid）：显示敌种名与当前元素层数。
- 反应提示 `.r24-reaction-hint`：每条提示显示「已附着 X + 再补 Y → Z 反应」。
- 空态 `.r24-hud-empty`：场上无敌人或无可提示反应时显示，逐字文案「当前场上无可预判的元素反应」。
- AI 注释 `.r24-ai-note`：逐字依据行「依据：敌人 stacks 元素层 × REACTION_PAIRS(源自引擎 lookupReaction) 派生；只读快照不进 replay_hash」。
- H5 必须在 tick 后刷新响应式数据：`battle.vue` 的 engine ref 见 `:612`，渲染模式 `renderMode` 见 `:638`，降级 tick `startFallbackLoop`/`setInterval(fallbackTick,...)` 见 `:972-978`；`engine.step()` 后调用只读快照并更新 ref（raf 与 fallback 两条路径都要刷新）。

## 4. AI 四判据

- ① 可见汉字「AI」标签。
- ② 真实数据结论：基于实际敌人元素层派生，示例「AI 提示：该敌人已附着焰，再补电可触发过热」。
- ③ 依据行：逐字「依据：敌人 stacks 元素层 × REACTION_PAIRS(源自引擎 lookupReaction) 派生；只读快照不进 replay_hash」。
- ④ 可执行 CTA：例如「去释放对应元素技能」。

## 5. 诚实红线（逐字守卫）

- 「辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准」。
- HUD 是只读派生：不得写回引擎状态、不改 step/replay_hash/score/结算/伤害；`lookupReaction`/`REACTION_PAIRS` 为静态派生，不落库不进哈希。
- 空态如实展示，不得编造「再补某元素就能触发」的假建议；敌人离开/死亡后快照自然消失，不缓存旧数据冒充当前状态。

## 6. 契约测试设计（test-first）

- 文件：`laiyipao/miniapp/src/pages/battle/battle_r24_element_hud.contract.test.ts`。
- 以纯参考实现 `hintsForEnemy` 按 SPEC §2 口径断言：`hintsForEnemy(['fire'])` 含 `steam_burst`（补 ice）与 `overheat`（补 lightning）但不含 `superconduct`；`hintsForEnemy(['ice'])` 含 `superconduct`（补 lightning）；`hintsForEnemy([])` 为空；`hintsForEnemy(ELEMENT_ORDER)` 为空。
- 源码文本守卫：引擎含 `elementLayerHud` 与 `slice()` 只读副本；battle.vue 含上述五类选择器与两条逐字文案；tokens.css 三锚点保留且无 `--r24-`；SPEC 含红线/依据行/四判据关键词。
- 零真值：SPEC 与测试不得含域名/IP/JWT/口令/个人绝对路径。

## 7. 三端不回退与分工

- 令牌锚点 `laiyipao/miniapp/src/styles/tokens.css:14-16` 的 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 保留，禁用 `--r24-` 新令牌。
- R22 选卡预判、R23 备战覆盖、R15-R23 既有契约与测试不改；本规范只新增 R24 只读 HUD 契约。
- 分工：t188 只交付本 SPEC 与契约测试；引擎只读方法/H5 渲染由后续实施任务完成。