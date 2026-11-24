# R17 · 验真页逐次反应序列对照（reaction-timeline-v1 辅助线索）契约与设计规范

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改业务代码）。
> 依据（2026-11-24 真代码审计）：验真页只认 replay_hash，而 replay_hash 只封构筑快照 S 段；R15 已落库的逐次反应时间线在验真链路完全未被消费，玩家无法用「第几波/哪个敌人/什么反应/多少伤害」辅助自查。

## 0. 真实缺口与依据（带行号）

1. 验真权威口径：`laiyipao/server/internal/domain/battle_collections.go:16` 注释 `ReplayHash string → replay_hash 列`；`:45` `maxReplayHashLen`、`:313-315` hash 格式校验。服务端**不重算事件流**，replay_hash 只覆盖既有构筑快照段。
2. 验真页只消费 replay_hash：`laiyipao/miniapp/src/pages/verify/verify.vue:18-31` R5 AI 复核提示、`:106-158` 本地重放卡（`:123` verdict-box 按 `outcome.matched`，`:132` 裁定「一致 — 分数可复现」/「不一致 — 该分数无法被复现」）、`:160-176` 服务端裁定卡（`result.matched` → 验真通过/失败）。全页无 `reaction_timeline`、无「逐次反应序列对照」。
3. 引擎侧已具备确定性序列但被丢弃：`laiyipao/miniapp/src/game/replay.ts:302` `replay(info, deps): ReplayOutcome`，`:331` `new BattleEngine`，`:353` `runToEnd`，`:355` 只取 `engine.replayHash()`，`:356-366` 返回 `{ computedHash, matched, stats }`——**未取** `engine.reactionTimeline()`。
4. 确定性已被证明：`laiyipao/miniapp/src/styles/r15ReactionTimeline.test.ts:149` `expect(a.reactionTimeline()).toEqual(b.reactionTimeline())`；引擎 `engine.ts:63-72` 定义 `ReactionTimelineEvent`（wave/tick/reaction/enemy_uid/damage，`enemy_uid` 为 snake_case wire 契约），`:74-75` 上限 200，`:772` `reactionTimeline()`，`:777` `reactionTimelineTruncated()`。
5. 服务端 ReplayInfo 缺字段：`laiyipao/server/internal/service/economy.go:1202-1222` `ReplayInfo` 结构（battle_id/level_id/seed/level/expected_hash/created_at/build/card_picks）无 `reaction_timeline`；而 `battle_records` 已有 `reaction_timeline` JSONB 列（R15 migration 00031），`economy.go:1377` 的 `BattleDetail` 已消费同列（`:1398` SELECT）。H5 `replay.ts:33-59` 的 `ReplayInfo` 接口同样缺该字段。

本规范补齐：把 R15 已落库的逐次序列桥接到验真页，做**辅助对照线索**，权威裁定仍唯一依赖 replay_hash。

## 1. 字段契约（端到端新增可选字段）

```
GET /api/v1/battle/:id/replay   （服务端 ReplayInfo 新增可选字段）
// "reaction_timeline": { "schema_version": "reaction-timeline-v1", "events": [...], "truncated": false, "total": N }
// 旧记录 / 字段缺失 / 结构异常 → events=[] + schema_version=reaction-timeline-v1（诚实空态，不报错、不编造）
```

- 单条事件严格五字段（与 R15 完全一致，顺序固定）：`wave`(int,≥1) / `tick`(int,≥0) / `reaction`(string key，合法反应集合) / `enemy_uid`(int，snake_case wire) / `damage`(int)。
- `truncated=true` 表示超过引擎上限 200 被尾部截断（保留最早 200 条）；H5 必须显示「已截断，仅显示前 200 条」，不得静默吞掉。
- **后端 test-first**：先加契约测试断言 `ReplayInfo` JSON 含 `reaction_timeline` 且空态填入法字段，再改 `economy.go:1202-1222` 从 `battle_records.reaction_timeline` 读取；旧行/无列返回 schema+events=[]。**不重算、不改 replay_hash、不改验真结论**。
- **H5 引擎旁路**：`ReplayOutcome` 新增 `reactionTimeline: ReactionTimelineEvent[]`（与 `reactionTimelineTruncated`，可选）；`replay()` 在 `runToEnd` 后同帧取 `engine.reactionTimeline()`，不改 `computedHash` / `matched` 任何计算。

## 2. 验真页展示契约（固定选择器，供 E2E 与契约测试锚定）

在 `verify.vue`「本地重放卡」之后、「服务端裁定卡」之前（建议 `:158` 附近）新增模块：

| 用途 | 选择器 | 语义 | 约束 |
| --- | --- | --- | --- |
| 容器 | `.r17-reaction-diff` | 逐次反应序列对照卡 | `role="region"`；标题含可见汉字「AI」；与权威裁定视觉分区 |
| 对照摘要 | `.r17-diff-summary` | 一致条数/差异条数 | 逐字前缀「一致 N 条 · 差异 M 条」，N/M 从确定性 diff 得出 |
| 逐行 | `.r17-diff-row` | 单条对照 | 每行带 `[data-r17-index="<i>"]`（i 为 0 基序号），仅差异行渲染 |
| 首处差异 | `.r17-first-diff` | 首处差异定位 | 逐字前缀「首处差异：第 {i+1} 条」，无差异时不渲染 |
| 空态 | `.r17-diff-empty` | 本地或服务端序列缺失 | 逐字文案「该战报无逐次反应记录，暂无法对照（旧版本结算）」 |
| 诚实说明 | `.r17-diff-note` | 定位说明 | 逐字「辅助对照线索，非验真判据；权威裁定仍以 replay_hash 为准」 |

- **行格式**：`第 {wave} 波 · {tick} tick · {反应中文名} · #{enemy_uid} · {damage}伤害`（本地值在前，服务端值在后，各带 `本地/``服务端` 前缀）；反应中文名用 H5 既有 `REACTIONS[key].name`（`elements.ts:128`），未知 key 兜底原始 key，**不硬编码中文名 map**。
- **diff 算法口径（确定性，可测试）**：取 `outcome.reactionTimeline` 与 `info.reaction_timeline.events`；按 index 逐条比较五字段（`wave/tick/reaction/enemy_uid/damage` 全等才是一致）；长度不等时多出的部分逐条计差异；仅展示差异行，一致数 = `min(本地长, 服务端长) - 差异数`，差异数 = 五字段不等的 index 数 + `abs(lenA-lenB)`。序号 i 为 0 基，展示时 `i+1`。
- **字段校验**：`schema_version !== 'reaction-timeline-v1'` 的事件序列按空态处理；非法事件（wave 非正整数/tick 非负整数/reaction 空/enemy_uid 非整数/damage 非有限数）丢弃后再 diff；`truncated` 仅认 `=== true`；不本地推算、不补零、不伪造 uid。
- **数值格式化**：damage 经确定性格式化（沿用 H5 `toFixed(1)` 惯例），禁止裸插值长浮点；wave/enemy_uid/tick 为整数直接插值。
- **与权威卡隔离**：本卡绝不放 `verdict-box`、不写「验真通过/失败」、不复用 `outcome.matched` / `result.matched` 作为本卡结论色。

## 3. AI 四判据（延续 verify.vue R5 复核卡口径，`:22-31`）

1. **可见汉字「AI」**：标题形如「AI · 逐次反应序列对照」，让玩家一眼识别这是辅助分析视图。
2. **真实数据结论**：摘要只陈述 diff 真实结果「一致 N 条 · 差异 M 条」；N/M 必须从确定性算法得出；无数据走空态，不做结论。
3. **依据行**：逐字 `依据：本地重放 reaction_timeline.events 与服务端 reaction_timeline.events 逐条比对，辅助对照线索`；点明两端字段名。
4. **可执行语义**：提供「查看权威裁定 ↑」CTA 指向同页服务端裁定卡（`:160-176`），不新造路由、不冒充裁定按钮。

- 空态逐字：`该战报无逐次反应记录，暂无法对照（旧版本结算）`。
- 截断态逐字：`已截断，仅显示前 200 条`（与 R15/R16 完全一致）。

## 4. 诚实红线（不可协商）

- **不参与、不覆盖 replay_hash**：reaction_timeline **不参与也不覆盖 replay_hash** 计算；replay_hash 仍只覆盖既有构筑快照段，服务端不重算事件流。
- **辅助对照线索、非权威验真判据**：时间线一致性**绝不冒充验真通过/失败**；权威裁定仍是 `replay_hash matched`（`outcome.matched` 与 `result.matched`）。diff 仅回答「本地与服务端记录的逐次叙事是否一致」，一致不代表分数可复现，不一致也不代表作弊。
- **不改结算**：timeline 不影响 score / 星级 / loot，服务端不把它当权威结算输入；验真 SQL/写入（`compareAndRecord`）口径不变。
- **旧记录诚实空态**：字段缺失 / `events=[]` / 结构异常 / schema 不符一律显示逐字空态，不抛错、不显示 `undefined` / `NaN`；**不得反推编造**事件。
- **零真值**：本规范与实现不得写入任何域名/IP/JWT/口令/本机路径真值；API 地址只用回环/占位符。

## 5. test-first 契约测试设计

新增 `laiyipao/miniapp/src/styles/r17VerifyReactionDiff.contract.test.ts`（miniapp vitest `include: ['src/**/*.test.ts']`，`vitest.config.ts:66`，environment=node，故测试落 `src/styles/`）。测试为**规范/基线守卫**：读 SPEC.md 断言条款齐全，读三端源码断言既有基线不回退；实施者删改规范或破坏锚点即变红。分组：

1. 契约主干：SPEC 含 `reaction-timeline-v1`、`reaction_timeline`、五字段、`200`、`truncated`、`schema_version`、`enemy_uid`。
2. 选择器与文案：SPEC 与源码含 `.r17-reaction-diff` / `.r17-diff-summary` / `.r17-diff-row` / `[data-r17-index]` / `.r17-first-diff` / `.r17-diff-empty` / `.r17-diff-note`，含逐字空态与 note 文案，且与 `.r15-` 区分。
3. diff 算法：SPEC 含五字段全等、`abs(lenA-lenB)`、0 基序号、`i+1`、`min`、`wave/tick`。
4. 诚实红线：SPEC 含 `不参与也不覆盖 replay_hash`、`辅助对照线索`、`非权威验真判据`、`权威裁定仍是`、`replay_hash matched`、`绝不冒充验真通过/失败`、`反推编造`。
5. AI 四判据与不回退：SPEC 含 `AI`、`真实数据结论`、`依据行`、`可执行`；`verify.vue` 仍含 `outcome.matched` / `result.matched` / 「验真通过」/「验真失败」；`replay.ts` 仍含 `engine.replayHash()` 且 `matched` 计算不变；`engine.ts` 仍含 `reactionTimeline()` 与 `enemy_uid`；`tokens.css` 三锚点不变、无 `--r17-` 新视觉变量。

运行：`npx vitest run --config vitest.config.ts src/styles/r17VerifyReactionDiff.contract.test.ts --reporter=verbose`（在 `laiyipao/miniapp` 下）。

## 6. 三端分工与不回退

- **backend**（t147）：`economy.go:1202-1222` `ReplayInfo` 增 `reaction_timeline`，从 `battle_records` 列读取、空态填 schema+events=[]；test-first；不改 replay_hash / 验真 SQL。
- **engine/H5**（t148）：`replay.ts` `ReplayInfo` 接口与 `ReplayOutcome` 增字段，`replay()` 旁路取 `engine.reactionTimeline()`；`verify.vue` 按 §2 选择器渲染对照卡；不碰 `outcome.matched` / `result.matched` 权威裁定。
- **不回退**：R1–R16 诚实文案、25% AoE、顶栏脱敏、R5 验真复核卡与三步步骤条、R15 `.r15-` 选择器与 R16 admin `.r16-` 选择器均不变；令牌锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`（`tokens.css:14-16`）不变；零新增视觉令牌。

> 公开仓库约束：本规范与配套测试不含任何域名/IP/JWT/口令/本机路径真值；所有路径均为仓库相对路径。