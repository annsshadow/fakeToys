# R26 规范：选卡阶段下一波敌情简报（round26-next-wave-brief）

> 目标：在选卡决策点告诉玩家「下一波来什么、多少只、有什么威胁」，让选卡从盲选变成有依据的战术决策。全部为只读派生展示，不改引擎/结算/replay_hash。

## 0 缺口与依据（亲读源码带行号，非编造）

- `laiyipao/miniapp/src/pages/battle/battle.vue:227-291` 选卡遮罩 `.sheet` 只含卡面（:238-281）与 R22 协同预判（:264-280 / :284-290），玩家看不到即将面对的下一波敌情。
- `laiyipao/miniapp/src/game/engine.ts:2180-2205` `checkWaveEnd()` 清波后取 `next = this.waveIndex + 1`（:2186），`next >= waves.length` 走 finish（:2187-2204），否则 `phase = 'card_select'`（:2205）——即进入选卡时下一波索引必定为 `waveIndex + 1` 且合法；`beginWave()` 在 :933 写 `this.waveIndex = index`、:938 取 `this.cfg.level.waves[index]`。
- `engine.ts:710-714` 构建时遍历 `cfg.level.waves`，每波 `w.spawns` 的 item 为 `{ enemy_id, count, interval, delay }`（见 `laiyipao/miniapp/src/test/fixtures.ts:28` 与多处测试夹具）；波次结构为 `{ wave_index, spawns: [...] }`。
- 敌种名与威胁口径现成：`laiyipao/miniapp/src/store/game.ts:119-123` `enemyMap` 由 `config.enemies` 派生；`laiyipao/miniapp/src/pages/stage/stage.vue:623-630` `enemyNameMap` 取名、:638-651 `enemyThreatTag` 按 `[首领]`/`[飞行]`/`[潜地]`/`[远程]` 标注（取自 `is_boss`/`fly_height`/`burrow`/`attack_range`）。
- 因此无需新增 API / 迁移 / 后端字段：真实数据已在 `eng.cfg.level.waves` 与 `store.config.enemies`。

## 1 只读派生边界（硬约束）

- 唯一数据来源：`eng.cfg.level.waves`（下一波 `waves[waveIndex + 1]`）与 `store.enemyMap`（`Map<number, EnemyDef>`，由 `config.enemies` 派生；按敌种 id 查找取敌种名/威胁字段）。
- 简报只读：不得调用 `engine.step()` / `beginWave()` / 任何写状态方法；不得新增或改写参与 `replay_hash` 的字段；不改 `score` / 结算 / 伤害 / `loot` / 发牌逻辑 / 卡牌点击与弃牌行为。
- 派生结果只进 H5 响应式展示变量；`waveIndex` 在 `card_select` 期间不前进，简报稳定不抖动。

## 2 纯函数契约（实施任务须照此实现）

- 纯函数 `buildNextWaveBrief(waves, waveIndex, enemyDefs)`：入参 `waves` 为波次数组，`waveIndex` 为引擎当前波次下标（0 起），`enemyDefs` 为 `ReadonlyMap<number, EnemyDefLike>`（真实来源 `store.enemyMap`，按 id `get` 查找，兼容 `Readonly<Record<number, EnemyDefLike>>` 测试夹具）；返回 `{ waveIndex, rows, empty, ai }`，其中 `waveIndex` 为下一波下标 `waveIndex + 1`（0 起）。
- 下一波取 `waves[waveIndex + 1]`：按 `spawns[].enemy_id` 聚合 `count`（同 id 多组相加），每行 `{ enemy_id, name, count, threat, unknown }`；`name` 取敌种定义，缺定义时逐字 `#<id>` 且 `unknown=true`、`threat=''`（未知敌种诚实标注，不编造 category）。
- `threat` 优先级与 stage.vue 完全一致：`is_boss`→`[首领]`、`fly_height > 0`→`[飞行]`、`burrow`→`[潜地]`、`attack_range > 0`→`[远程]`，否则空串。
- 排序：`count` 降序；同数量按 `enemy_id` 升序稳定排序。
- 空态：`waves` 缺失/非数组、`waveIndex + 1` 越界、`spawns` 非数组或聚合后无行 → `empty=true`、`rows=[]`，渲染 `.r26-brief-empty` 逐字「暂无下一波敌情信息」。绝不编造敌种。
- `ai` 由真实聚合确定性推导：结论基于实际敌种/数量/威胁（如「下一波 3 种敌人共 12 只，其中 [飞行]×4 需要优先对空」）；无威胁时给朴素真实结论，不夸张。

## 3 选择器与 390×844 布局约束

- `.r26-wave-brief`：简报卡容器，置于 `.card-header` 之后、`.cards` 之前（先看敌情再选卡）。
- `.r26-brief-head`：波次标题「下一波 · 第 N 波」（N = 返回的下一波下标 + 1，1 起显示，与 UI 既有波次显示口径一致）。
- `.r26-enemy-row`：每个敌种一行，带 `[data-r26-enemy]` 属性（值为 enemy_id），展示名字、数量、威胁标签。
- `.r26-unknown-tag`：未知敌种逐字标注 `[未知敌种]`。
- `.r26-brief-empty`：空态逐字「暂无下一波敌情信息」。
- `.r26-ai-note`：AI 注释块，含可见汉字「AI」、结论、逐字依据行、CTA、诚实红线。
- 布局：简报卡 `max-height: 160rpx; overflow-y: auto`（约 83px，最多 4 行后滚动），行内省略不换行；`.cards` 的 3 张手卡保持 `min-height: 260rpx` 与 flex 横排不变，`.sheet` 整体不得因简报增长把选卡挤出 390×844 视口——手卡区与「全部不要了」按钮必须始终可见可点。`max-height` 与 `overflow-y` 由实施任务落到 CSS，契约测试钉死这两个声明。

## 4 AI 四判据

- ① 可见汉字「AI」（`.r26-ai-note` 内标题）。
- ② 真实数据结论：只基于本波实际聚合，示例「下一波 3 种敌人共 12 只，其中 [飞行]×4 需要优先对空」。
- ③ 依据行逐字：「依据：当前关卡 waves[waveIndex+1].spawns 按 enemy_id 聚合；敌种名与威胁标签来自 /config.enemies」。
- ④ 可执行 CTA：按真实威胁给「优先选对空/范围/单体技能」；无威胁时给「按伤害与元素协同选卡」。

## 5 诚实红线与不回退

- 逐字红线：「辅助简报非权威战斗数据，实际敌情以引擎波次为准；权威回放以 replay_hash 为准」。
- 未知敌种不编造名字/威胁；无下一波时空态而非猜测；简报不影响任何结算与哈希。
- 令牌锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 保留，禁用 `--r26-` 新令牌。
- R22 协同卡、R15 时间线、R18/R19/R20/R21/R24/R25 既有契约与三端数据不改。

## 6 契约测试设计（test-first，真实模块导入 + RED 门禁）

- 文件：`laiyipao/miniapp/src/pages/battle/battle_r26_next_wave_brief.contract.test.ts`。
- **禁止内嵌参考实现**：测试必须从实施模块 `laiyipao/miniapp/src/pages/battle/nextWaveBrief.ts` 具名导入 `buildNextWaveBrief`（`import { buildNextWaveBrief } from './nextWaveBrief'`）。测试文件内不得出现任何同名本地函数或算法复刻——内嵌实现会让「功能完全缺失」时仍全绿，丧失 test-first 意义。
- 聚合口径断言（全部调用真实导入的函数）：按 enemy_id 聚合同组相加、count 降序 + 同数量 enemy_id 升序、威胁优先级首领>飞行>潜地>远程、未知敌种 `#id` + `unknown=true` + 空威胁、五类空态（无 waves/越界/空 spawns/全无效行/非整数 id）、AI 结论真实来自聚合。
- **真实源码守卫（不只测 SPEC 文本）**：直接读取 `battle.vue` 源码断言六类选择器 `r26-wave-brief`/`r26-brief-head`/`r26-enemy-row`/`data-r26-enemy`/`r26-brief-empty`/`r26-ai-note` 均已落地，逐字空态文案 `[未知敌种]` 与 §5 红线已落地，CSS 含 `max-height: 160rpx` 与 `overflow-y: auto`，且模板 import/调用 `buildNextWaveBrief`（`./nextWaveBrief`）；`engine.ts` 含 `checkWaveEnd`/`const next = this.waveIndex + 1`/`card_select`/`cfg.level.waves` 依据；battle.vue 保留 `.sheet`/`showCards`/`hand-card`/`r22-synergy-card` 锚点；tokens.css 三锚点保留且无 `--r26-`。
- **RED 门禁**：实施模块 `nextWaveBrief.ts` 尚不存在时，测试文件因模块解析失败而整体加载失败（RED），须保留该 RED 运行输出作为 test-first 证据；模块落地且 battle.vue 接入后再跑 GREEN。t204 只交付 RED，不得创建业务实现模块。
- 零真值：SPEC 与测试不含域名/IP/JWT/口令/个人绝对路径。

## 7 三端分工

- t199 交付本 SPEC 与契约测试（初版）；t204 补强为真实模块导入 + 真实源码守卫并跑出 RED 证据，不改 BATTLE 业务实现。
- H5 实施（`nextWaveBrief.ts` 纯函数 + battle.vue computed/模板/CSS）由后续任务完成；本轮零后端改动。