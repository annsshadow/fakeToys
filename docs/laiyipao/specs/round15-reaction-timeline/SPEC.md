# R15 · 战报反应时间线（reaction-timeline-v1）契约与设计规范

> 状态：设计拍板 / test-first 契约已钉（RED 3 failed / 1 passed 已验证）。
> 依据（2026-11-23 真代码审计）：引擎每反应发 per-event 事件，但战报详情只有聚合 map，无法回答「哪一波/哪个敌人/触发什么反应/造成多少伤害」。

## 0. 真实缺口

- `laiyipao/miniapp/src/game/engine.ts:1725-1746`：每次反应 `this.emit({ type: 'reaction', reaction, x, y, damage, resisted, existingElement, incomingElement })` + `this.record(this.tick, 'reaction', e.uid, reactionIndex(res.reaction))` —— **事件流里逐次信息是存在的**。
- `laiyipao/server/internal/service/adminbattledetail.go:56-57`：`elements_used` / `reactions_used` 只是聚合 map（`map[string]int64`），`myBattleDetail`（`routes.go:887`，路由 `routes.go:196` 的 `GET /me/battles/:id`）同样只回聚合。战报详情无法回答逐次归因。
- `laiyipao/server/internal/domain/battle_collections.go:19`：客户端可控集合字段本无界，一个 10MB 的 map 就能把战报行撑到不可用——新增字段必须自带上限，不能重蹈覆辙。

## 1. 字段契约（服务端战报详情新增可选字段）

```
GET /api/v1/me/battles/:id  （玩家战报详情）
GET /api/v1/admin/battles/:id  （运营战报详情抽屉）
// 两者均新增可选字段 reaction_timeline；schema_version = reaction-timeline-v1
```

单条事件五字段（严格类型）：wave(int) / tick(int) / reaction(string key) / enemy_uid(int) / damage(int)。

| 字段 | 类型 | 含义 | 来源 |
| --- | --- | --- | --- |
| wave | int | 第几波（≥1） | 引擎 record 时的当前波次 |
| tick | int | 该波内 tick（≥0） | 引擎 `this.tick` |
| reaction | string key | 反应 key | 引擎 `res.reaction`，必须属于 `AllReactionSpecs` 合法集合 |
| enemy_uid | int | 触发该反应的敌人 uid | 引擎 `e.uid`（事件 record 的 uid） |
| damage | int | 该次反应伤害 | 引擎 `res.reactionDamage` |

外层：`"reaction_timeline": { "schema_version": "reaction-timeline-v1", "events": [ ... ], "truncated": false, "total": 37 }`。`total` 为事件实际总数，`events` 为截断后数组；`truncated=true` 表示超过上限被截断，H5 必须显示「已截断，仅显示前 200 条」提示，不得悄悄吞掉。

### 1.1 上限与 DB 红线

- 最多 200 条；超出部分**从尾部截断**（保留最早的 200 条，确保「哪一波先发生」可读），并置 `truncated=true`。
- 依据 `battle_collections.go:19` 的 10MB 红线：单条事件五字段均为定长标量，200 条序列化体积远低于阈值；但**不得**因为「小」就取消上限——无界字段正是 `elements_used` 当年的教训。
- `reaction` 写入前用 `AllReactionSpecs` 合法集合校验（`knownReactions`），未知 key 丢弃而非入库，避免污染运营「反应分布」维度。

## 2. 诚实红线（不可协商）

- **展示性归因**：`reaction_timeline` 来自本局回放事件记录，**仅作展示性归因**；它是「发生了什么」的叙事，不是结算凭证。
- **不参与 replay_hash**：timeline **不参与 replay_hash** 计算；replay_hash 仍只覆盖既有构筑快照段，服务端不重算事件流。
- **不影响结算**：timeline **不影响 score**，不影响 loot，不影响**星级**；服务端**不把它当权威结算输入**，不读它做数值裁决。
- **旧战报诚实空态**：旧战报无字段 / 空数组时显示「该战报无逐次反应记录（旧版本结算）」。这是**可选字段**：缺失不是错误，是历史版本没记录；**不得编造**事件、不得拿聚合 map 反推逐次明细填充。
- 前端不得把 timeline 当作「AI 精准预测/结算依据」宣传；它只回答「本局第几波对哪个敌人触发了什么反应、造成多少伤害」。

## 3. H5 展示契约（选择器固定，供 E2E 锚定）

- 容器：`.r15-reaction-timeline` —— 战报详情抽屉内新增模块，标题「反应时间线」。
- 逐条：`.r15-timeline-row`，每条带 `[data-r15-wave="<wave>"]` 属性，按 wave/tick 升序；条文展示「第 {wave} 波 · {reaction} · #{enemy_uid} · {damage} 伤害」。
- 空态：`.r15-timeline-empty`，文案逐字为「该战报无逐次反应记录（旧版本结算）」。
- 截断态：`.r15-truncated-note`，文案「已截断，仅显示前 200 条」；`truncated=false` 时不渲染。
- 数值来自服务端 timeline 事件，前端不本地推算、不补零、不伪造 uid。

## 4. 三端分工与并行消费

- backend：在 `myBattleDetail` / `AdminBattleDetail` 响应里新增 `reaction_timeline` 可选字段；从结算时落库的事件记录读取（结算写入侧需同步保留逐次事件，但不在本规范范围），上限 200 + 截断标记 + 合法 key 校验；空战报返回字段缺失或空数组，不编造。
- h5-ui：在战报详情按 §3 选择器渲染容器/行/空态/截断态；无字段走诚实空态文案。
- qa-e2e：E2E 用 `.r15-reaction-timeline` / `.r15-timeline-row` / `[data-r15-wave]` / `.r15-timeline-empty` 锚定，断言空战报显示逐字文案、有数据战报行数 ≤200 且升序。

## 5. 契约测试与不回退

`laiyipao/miniapp/src/styles/r15ReactionTimeline.contract.test.ts` 四组：① schema/五字段/上限200/truncated；② 诚实红线（展示性归因、不进 replay_hash、不改 score/loot/星级、非权威结算输入、旧战报空态、不得编造、可选字段、AllReactionSpecs）；③ 选择器 + 引擎 per-event 事实；④ 令牌锚点不回退。

不回退：R1–R14 诚实文案、25% AoE、顶栏脱敏、R14 建议 CTA 与 R7 两步确认不变；令牌 v1/v3-r3/r6 锚点不变，零新增视觉令牌。

> 公开仓库约束：本规范不含任何域名/IP/token/口令/本机路径真值。
