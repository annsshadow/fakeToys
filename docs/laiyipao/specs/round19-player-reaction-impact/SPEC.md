# R19 · 玩家反应伤害贡献面板（reaction-timeline-v1 个人聚合）契约与设计规范

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改业务代码）。
> 依据（2026-11-24 真代码审计）：R15 已把逐次反应的真实伤害落到 `battle_records.reaction_timeline.events.damage`（带 user_id），R18 已做 admin 侧全平台聚合；但玩家侧 `reactions.vue` 只展示「理论可达反应」（构筑能打出哪些），玩家看不到自己真实打出的反应伤害贡献——高频低伤 vs 低频高伤完全盲区。

## 0. 真实缺口与依据（带行号）

1. 玩家反应页只有理论构筑视图：laiyipao/miniapp/src/pages/reactions/reactions.vue:8-26 的 KPI 是 `myReactions.length / allReactions.length` 与覆盖率，:85-88 是「我的构筑能触发的反应」，:101-115 是元素覆盖矩阵；:254 起 `<script setup>` 的 `reachableReactions` / `lookupReaction` 全部来自配置表，没有任何来自战报的真实伤害数据。
2. 伤害数据已在库但未对玩家开放：R15 的 `reaction_timeline.events[].damage` 自结算期写入 battle_records（见 engine.ts:772 的 `reactionTimeline()`），R16 战报详情、R17 验真 diff、R18 admin 聚合均已消费；**唯一没有面向玩家本人的伤害贡献视图**。
3. admin 聚合口径不能直接复用：R18 的 `GET /admin/reaction-damage-impact` 是全平台视图，次数权威取全平台物化视图 `dashboard_reaction_counts`（stats.go:298-303），无 user_id 维度；玩家侧必须新增按当前 user_id 过滤的端点，且**同一份权威次数语义**要换真源（见 §1 count 口径裁决）。
4. 权威次数列已存在：laiyipao/server/internal/domain/battle_collections.go:13/20/32/36/214-216 的注释与校验坐实 `ReactionsUsed map[string]int → reactions_used 列`（结算期每触发一次 +1、键名按合法反应集合校验），是玩家个人触发次数的权威真源。

本规范补齐：新增玩家鉴权聚合 API（严格按 user_id 隔离），H5 反应作战台加「我的反应伤害贡献」卡（次数/伤害双视图 + AI 四判据 + 诚实空态）。伤害贡献定位辅助分析，权威结算与验真口径一律不动。

## 1. 后端 API 契约（新增，test-first）

```
GET /me/reaction-damage-impact   （新增；玩家鉴权，沿用 requireUser 中间件）
```

**鉴权硬约束**：从会话取 `user_id`（与 requireUser / c.Locals("user_id") 一致），所有 SQL 必须带 `WHERE user_id = 当前登录用户`；**绝不返回他人数据**，不接受前端传入的 user_id 参数，不因参数注入越权。test-first 必须包含「两个用户互不串数据」用例。

响应（严格字段，snake_case）：

| 字段 | 类型 | 含义 | 真源 |
| --- | --- | --- | --- |
| `window_days` | int | 统计窗口天数，固定 `30` | 常量，响应显式标注 |
| `window_from` / `window_to` | string | 窗口起止日期（UTC 日期，`YYYY-MM-DD`） | 服务端按 `now - 30d` 确定性计算 |
| `excluded_old_battles` | int | 窗口内无 timeline / `events=[]` / schema 不符的旧战报数 | 从当前用户的 battle_records 统计，如实标注 |
| `items` | array | 每反应一行 | 见下；空结果为 `[]`，不是 `null` |

`items` 单行五字段（逐字对齐任务书：`{reaction,total_damage,damage_share_pct,events,count}`）：

| 字段 | 类型 | 含义 | 真源 |
| --- | --- | --- | --- |
| `reaction` | string | 反应 key | `domain.AllReactionSpecs()` 合法集合 |
| `total_damage` | int64 | 窗口内该反应的真实累计伤害 | 当前用户 battle_records.reaction_timeline JSONB 展开 `events` 求和（仅窗口内、仅合法 key、仅 schema=v1） |
| `damage_share_pct` | float64 | 伤害贡献占比 = `total_damage / SUM(total_damage) * 100` | 服务端计算并 `toFixed(1)` 等价精度下发；`SUM=0` 时全为 `0`，不产生 `NaN` |
| `events` | int64 | 窗口内该反应的时间线事件条数 | 同上展开计数（与权威 count 语义不同，不得冒充） |
| `count` | int64 | **权威触发次数** | 当前用户窗口内 `battle_records.reactions_used` 按 reaction key 累加（见下方裁决） |

### count 口径裁决（为什么不是 events，也不是 dashboard_reaction_counts）

- **不用 `events`**：`reaction_timeline.events` 是 R15 的展示性逐次归因，条数只在有 timeline 的战报里存在；旧战报无 timeline 但 `reactions_used` 仍有次数，用 events 冒充 count 会系统性低估触发次数，且两个字段语义不同（事件条数 ≠ 结算触发次数）。
- **不用 `dashboard_reaction_counts`**：该物化视图（迁移 00025，stats.go:298-303）是**全平台** GROUP BY 结果，只有 reaction 维度、无 user_id，无法回答「我打了多少次」。玩家个人面板必须从逐场 battle_records 的 `reactions_used` 按 user + 窗口累加。
- **选 `battle_records.reactions_used`**：它与看板同源（看板由它在写入时触发器累加），是**结算期权威写入**的次数 map（battle_collections.go:13/32/36/214-216 校验合法键），语义与 R18 的 count 一致，只是过滤维度从「全平台」收窄到「当前 user_id + 30 天窗口」。
- **口径差异的诚实标注**：旧战报（无 timeline / 空 events / schema 不符）的 `reactions_used` 次数**仍计入 count**（权威次数不受展示性数据缺失影响），但其伤害不计入 `total_damage`，同时 `excluded_old_battles` 自增；UI 必须对「触发次数」与「已记录伤害」使用不同措辞，并在诚实说明中标注两者分母可能不同。

- 窗口口径：固定近 `30` 天，按 `battle_records.created_at` 过滤；服务端确定性计算，不信任客户端时间。
- 稳健性：JSONB 展开限定当前用户 + 窗口后再聚合；非法 key 丢弃；`COALESCE` 处理 null；int64 承接伤害；空结果 `items=[]`。
- test-first：先加后端契约测试断言①两用户数据隔离②响应五字段③`count` 取自 `reactions_used` 而非 `events`④旧战报 `excluded_old_battles` 自增且次数仍入 count⑤`SUM=0` 占比归零无 NaN，再写聚合 SQL/端点。

## 2. H5 展示契约（固定选择器，供 E2E 与契约测试锚定）

在 `reactions.vue` 的反应作战台内新增「我的反应伤害贡献」卡，置于理论构筑 KPI 区之后，理论视图全部保留：

| 用途 | 选择器 | 语义 | 约束 |
| --- | --- | --- | --- |
| 容器 | `.r19-my-reaction-impact` | 玩家伤害贡献卡 | 与理论构筑视图并存，不替换 |
| 视图切换 | `.r19-view-toggle` | 次数 / 伤害 双视图 | 默认次数视图；伤害视图用 `damage_share_pct` |
| 数据行 | `.r19-rdi-row` | 单反应贡献行 | 每行带 `[data-r19-reaction='<key>']`，伤害视图按占比降序 |
| 空态 | `.r19-rdi-empty` | 无记录 / 加载失败 | 逐字「近 30 天无逐次反应伤害记录」，不编造 |
| AI 洞察 | `.r19-ai-note` | 辅助分析结论 | 含可见汉字「AI」、真实数据结论、依据行、CTA |

- 次数视图：`count` 来自权威字段，沿用页面既有取名函数 `reactionName(key)`（:370-371）；**不得**把 `events` 当成触发次数展示。
- 伤害视图：`damage_share_pct` 经 `toFixed(1)` 确定性格式化，禁止裸插值长浮点 / `NaN`；`SUM=0` 显示 0%。
- AI 结论必须来自当前响应真实数据（例：「overheat 触发 X 次但伤害贡献仅 Y%，疑似高频低伤」），无数据走空态，不编造。
- 诚实说明逐字：「辅助分析非权威结算依据，权威请看 replay_hash」；`window_from` / `window_to` / `excluded_old_battles` 必须展示，排除数为 0 也如实显示 0。

## 3. AI 四判据（延续页面既有 R3/R6/R14 AI 模式）

在 `.r19-ai-note` 内同时具备四要素：

1. 可见汉字「AI」：标题形如「AI · 我的反应伤害洞察」。
2. 真实数据结论：由 `items` 的 `count` / `damage_share_pct` 确定性推导，禁止硬编码反应名结论。
3. 依据行：逐字「依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准」。
4. 可执行 CTA：指向本页次数视图或战报列表，不新造路由、不冒充权威裁定。

## 4. 诚实红线（不可协商）

- 辅助分析非权威：伤害贡献只是辅助洞察；**权威结算以 replay_hash 为准**，本端点不参与也不覆盖 replay_hash、score、星级、loot。
- 不泄露他人数据：严格 `WHERE user_id = 当前登录用户`，不接前端 user_id 参数；跨用户数据隔离是 test-first 必测项。
- 次数口径诚实：`count` 取 `reactions_used`（权威结算写入），`events` 是时间线事件条数，二者分母可能不同且旧战报次数仍入 count；不得混用、不得用 events 冒充。
- 旧战报如实标注：无 timeline / 空 events / schema 不符的战报计入 `excluded_old_battles`，UI 明示「已排除 N 条无逐次记录的旧战报」，不得假装全量、不得反推伤害补洞。
- 空结果诚实：`items=[]` 走逐字空态，不显示 `undefined` / `NaN`，不以理论可达反应替代真实记录。
- 零真值：规范与实现不得写入任何域名/IP/JWT/口令/本机路径真值；API 地址只用回环或占位符。

## 5. test-first 契约测试设计

新增 laiyipao/miniapp/tests/R19ReactionDamageImpact.contract.test.ts（规范/基线守卫：读 SPEC 断言条款齐全，读三端源码断言既有基线不回退，实施者删改规范或破坏锚点即变红）。分组：

1. API 与字段：SPEC 含 `GET /me/reaction-damage-impact`、`user_id` 隔离、`window_days` / `window_from` / `window_to` / `excluded_old_battles` / `items`、五字段与 `window_days=30`。
2. count 口径裁决：SPEC 含 `reactions_used`、`不得冒充`、`dashboard_reaction_counts`、`分母可能不同`、`旧战报` 次数仍入 count。
3. 选择器与逐字文案：SPEC 含 `.r19-my-reaction-impact` / `.r19-view-toggle` / `.r19-rdi-row` / `[data-r19-reaction]` / `.r19-rdi-empty` / `.r19-ai-note`，含两处逐字文案。
4. AI 四判据与诚实红线：SPEC 含 `可见汉字「AI」`、`真实数据结论`、`依据行`、`可执行`、`辅助分析非权威结算依据，权威请看 replay_hash`、`不泄露他人数据`、`零真值`。
5. 三端不回退：miniapp tokens.css 三锚点 v1/v3-r3/r6 不变且无 `--r19-`；battle_collections.go 保留 `reactions_used` / `replay_hash` 与合法键校验；economy.go 保留 `ReplayInfo` 与 `parseReplayReactionTimeline`；admin/src/api/index.ts 保留 `/admin/reaction-damage-impact`；reactions.vue 保留 `myReactions` / `reachableReactions` / `reactionName` 理论视图符号。

## 6. 三端分工与不回退

- backend（后续实施）：新增 GET /me/reaction-damage-impact，按 user_id + 30 天窗口从 reaction_timeline 聚合伤害、从 reactions_used 聚合权威次数，回传窗口与排除数；test-first；不改已有端点。
- h5（后续实施）：reactions.vue 加贡献卡与双视图/AI 洞察；理论构筑 KPI、覆盖矩阵、R3/R6/R14 AI 组件与取名函数全部保留。
- admin：本轮零改动——R18 端点 `GET /admin/reaction-damage-impact`（admin/src/api/index.ts:202）与看板次数图不动。
- 不回退：R1–R18 诚实文案、25% AoE、顶栏脱敏、令牌锚点 v1/v3-r3/r6（miniapp tokens.css:14-16）不变；零新增视觉令牌。

> 公开仓库约束：本规范与配套测试不含任何域名/IP/JWT/口令/本机路径真值；所有路径均为仓库相对路径。