# R18 · admin 运营看板反应伤害贡献分析（reaction-timeline-v1 聚合）契约与设计规范

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改业务代码）。
> 依据（2026-11-24 真代码审计）：R15 已把逐次反应（含真实伤害）落到 battle_records.reaction_timeline，R16/R17 已用于战报展示与验真 diff；唯一未消费的环节是运营看板的聚合分析——次数分布只看得到「触发多少次」，看不到「打出多少伤害」。

## 0. 真实缺口与依据（带行号）

1. 后端只统计次数：laiyipao/server/internal/service/stats.go:72-76 的 `ReactionCount{Reaction,Count}` 只有 `reaction` / `count` 两字段；:284-331 的「反应使用分布」段读 `dashboard_reaction_counts` 物化视图（:298-303 SELECT `reaction, SUM(count)`，:325-329 按 `domain.AllReactionSpecs()` 逐键填 `ReactionCount`）。这是次数权威，基于 `reactions_used` map，不含伤害。
2. admin 看板只画次数柱：laiyipao/admin/src/views/DashboardView.vue:142-162 的 `reactionChart` 用 `series.data = items.map(x => x.count)`；:78-82 `reactionRows` 与 :87 `reactionDesc` 也只按 `count` 排序；模板 :323-342 反应分布卡只有「触发次数」表头与「尚无反应触发记录」空态。运营无法识别「高频低伤 vs 低频高伤」的反应失衡。
3. 真实伤害数据已存在但未被聚合：`reaction_timeline` 每条事件含 `damage`（见 R15 SPEC §1 / `engine.ts:63-72` 的 `ReactionTimelineEvent{wave,tick,reaction,enemy_uid,damage}`），R16 战报展示、R17 验真 diff 已消费，唯独没有看板聚合。
4. H5 侧已采集完毕、本轮无需改动：laiyipao/miniapp/src/game/engine.ts:772 的 `reactionTimeline()` 已在结算期落库，H5 不新增采集逻辑；R18 是纯后端聚合 + admin 展示，H5 本轮零改动（见 §6）。

本规范补齐：新增后端聚合 API，admin 反应分布卡加「次数/伤害贡献」双视图 + 真实数据驱动 AI 洞察；次数权威不变，伤害贡献定位辅助分析维度。

## 1. 后端聚合 API 契约（新增，test-first）

```
GET /admin/reaction-damage-impact   （新增；管理员鉴权，沿用 /admin/* 中间件）
```

响应（严格字段，snake_case）：

| 字段 | 类型 | 含义 | 真源 |
| --- | --- | --- | --- |
| `window_days` | int | 统计窗口天数，固定 `30` | 常量，响应显式标注 |
| `window_from` / `window_to` | string | 窗口起止日期（UTC 日期，`YYYY-MM-DD`） | 服务端按 `now - 30d` 确定性计算 |
| `excluded_old_battles` | int | 窗口内无 reaction_timeline 的旧战报数（不计入伤害聚合） | 从 battle_records 统计，如实标注 |
| `items` | array | 每反应一行 | 见下 |

`items` 单行五字段（逐字对齐任务书：`{reaction,total_damage,damage_share_pct,events,count}`）：

| 字段 | 类型 | 含义 | 真源 |
| --- | --- | --- | --- |
| `reaction` | string | 反应 key | `domain.AllReactionSpecs()` 合法集合 |
| `total_damage` | int64 | 窗口内该反应 `damage` 之和 | battle_records.reaction_timeline JSONB 展开 `events` 求和（仅窗口内、仅合法 key） |
| `damage_share_pct` | float64 | 伤害贡献占比 = `total_damage / SUM(total_damage) * 100` | 服务端计算并 `toFixed(1)` 等价精度下发；`SUM=0` 时全为 `0`，不产生 `NaN` |
| `events` | int64 | 窗口内该反应时间线事件条数 | 同上展开计数 |
| `count` | int64 | 权威触发次数 | 必须来自 `dashboard_reaction_counts`（与 `Dashboard.ReactionUsage` 同口径），不得用 `events` 冒充 |

- 窗口口径：固定近 `30` 天，按 `battle_records.created_at` 过滤；窗口内 `reaction_timeline` 缺失 / `events=[]` / schema 不符的战报不计入伤害聚合，但 `excluded_old_battles` 自增，响应显式回传，避免运营把「少了旧战报」误读为「伤害变低」。
- 上限与稳健：JSONB 展开限定窗口后再聚合；非法 key 丢弃；`total_damage` 溢出可用 `COALESCE` 与 int64 承接；空结果 `items` 为 `[]`（不是 `null`）。
- test-first：先加后端契约测试断言响应五字段、`window_days=30`、`excluded_old_battles` 回传、`count` 取自 `dashboard_reaction_counts` 而非 `events`，再写聚合 SQL/API；不改 `stats.go:72-76` 的 `ReactionCount` 结构与次数图查询。

## 2. admin 展示契约（固定选择器，供 E2E 与契约测试锚定）

在 DashboardView.vue 反应分布卡（:323-342）内新增「次数/伤害贡献」双视图切换与 AI 洞察，次数图原样保留：

| 用途 | 选择器 | 语义 | 约束 |
| --- | --- | --- | --- |
| 容器 | `.r18-damage-impact` | 伤害贡献分析卡（嵌在反应分布卡内） | role=region；与权威次数图同卡但视觉分区 |
| 视图切换 | `.r18-view-toggle` | 次数 / 伤害贡献 双视图 | 两个选项 `.r18-view-count` / `.r18-view-damage`；默认次数视图（不改变既有体验） |
| 伤害行 | `.r18-damage-row` | 单反应伤害贡献 | 每行带 `[data-r18-reaction='<key>']`，按 `damage_share_pct` 降序 |
| 窗口标注 | `.r18-window-note` | 统计窗口 | 逐字「统计窗口：近 30 天（{window_from} 至 {window_to}）」 |
| 排除标注 | `.r18-excluded-note` | 旧战报排除数 | 逐字「已排除 {excluded_old_battles} 条无逐次记录的旧战报（不计入伤害）」；为 0 时也要如实显示 0 |
| AI 洞察 | `.r18-ai-note` | 辅助分析结论 | role=note；标题含可见汉字「AI」 |
| 诚实说明 | `.r18-honesty-note` | 口径定位 | 逐字「伤害贡献为辅助分析；触发次数以物化视图为准，不参与结算与 replay_hash」 |

- 次数视图：沿用现有 `reactionRef` / `reactionChart` / `reactionRows` / `reactionDesc` / 「尚无反应触发记录」空态，不得删除或替换（:142-162/:78-82/:324-341 保留）。
- 伤害视图：`series` 用 `damage_share_pct`，`yAxis` 用 `labelOf(reaction)`（服务端取名，/reactions 的 `labelOfReaction`，兜底原始 key，不得硬编码中文名）；数值经确定性格式化（沿用 admin `toFixed(1)` 惯例），禁止裸插值长浮点 / `NaN`。
- 空态：`items=[]` 时伤害视图显示逐字「近 30 天无逐次反应伤害记录」，不编造、不显示 `undefined` / `NaN`；窗口/排除标注仍需显示。
- 疑问定位：`excluded_old_battles > 0` 时排除标注必须可见，不能静默隐藏旧战报影响。

## 3. AI 四判据（延续 admin 既有洞察卡口径，utils/aiInsight.ts:14-22 的 `AiInsight`）

在 `.r18-ai-note` 内同时具备四要素：

1. 可见汉字「AI」：标题形如「AI · 反应伤害洞察」。
2. 真实数据结论：从 `items` 确定性推导，例如「overheat 触发占比 X% 但伤害贡献仅 Y%，疑似高频低伤」——X/Y 必须由 `count` / `damage_share_pct` 真实计算；无数据走空态，不做结论、不编造置信度。
3. 依据行：逐字「依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 dashboard_reaction_counts 为准」。
4. 可执行 CTA：指向同页反应分布卡（次数视图）或 /admin/battles，不新造路由、不冒充权威裁定。

## 4. 诚实红线（不可协商）

- 辅助分析、非权威：伤害贡献是辅助分析维度；次数权威仍以物化视图 `dashboard_reaction_counts` 为准，`count` 不得用 `events` 或聚合伤害冒充，AI 结论不得宣称伤害占比是官方数值平衡结论。
- 不参与结算 / replay_hash：伤害贡献不参与任何结算、score、星级、loot，也不参与或覆盖 `replay_hash`；服务端不读它做数值裁决，验真结论口径不变。
- 旧战报如实标注：无时间线旧战报不计入伤害聚合，但必须在 `excluded_old_battles` 如实标注排除数，不得假装全量、不得拿 `reactions_used` map 反推伤害补洞。
- H5 本轮零改动：数据（逐次 damage）已由 R15 在结算期采集落库，本轮只做后端聚合 + admin 展示；不改 H5 采集、不改验真 diff（R17 口径保持）。
- 零真值：本规范与实现不得写入任何域名/IP/JWT/口令/本机路径真值；API 地址只用回环/占位符。

## 5. test-first 契约测试设计

新增 laiyipao/admin/tests/R18ReactionDamageImpact.contract.test.ts（admin vitest include: tests/**/*.test.ts，vitest.config.ts:55，jsdom + setup 垫片齐备）。测试为规范/基线守卫：读 SPEC.md 断言条款齐全，读三端源码断言既有基线不回退；实施者删改规范或破坏锚点即变红。分组：

1. 字段契约：SPEC 含 `reaction-damage-impact`、`reaction` / `total_damage` / `damage_share_pct` / `events` / `count`、`window_days` / `window_from` / `window_to` / `excluded_old_battles`、`30`。
2. 选择器与文案：SPEC 含 `.r18-damage-impact` / `.r18-view-toggle` / `.r18-view-count` / `.r18-view-damage` / `.r18-damage-row` / `[data-r18-reaction]` / `.r18-window-note` / `.r18-excluded-note` / `.r18-ai-note` / `.r18-honesty-note`，含逐字窗口/排除/诚实文案。
3. 诚实红线：SPEC 含 `dashboard_reaction_counts`、`不参与`、`replay_hash`、`不得假装全量`、`反推`、`H5 本轮零改动`、`零真值`。
4. AI 四判据：SPEC 含 `AI`、`真实数据结论`、`依据行`、`可执行`。
5. 三端不回退：stats.go 仍含 `type ReactionCount struct` 与 json:reaction_usage、DashboardView.vue 仍含 `reactionRef` / `reactionChart` / `x.count` /「尚无反应触发记录」、`labelOfReaction`；tokens.css 三锚点（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）不变且无 --r18- 视觉变量。

运行：在 laiyipao/admin 下执行 npx vitest run --config vitest.config.ts tests/R18ReactionDamageImpact.contract.test.ts --reporter=verbose。

## 6. 三端分工与不回退

- backend（后续实施）：新增 GET /admin/reaction-damage-impact，从 battle_records.reaction_timeline 聚合近 30 天伤害、从 dashboard_reaction_counts 取权威次数，回传窗口与排除数；test-first；不改 stats.go:72-76 的 ReactionCount 与次数分布查询。
- admin（后续实施）：DashboardView.vue 反应分布卡内加双视图切换 + AI 洞察；次数图、reactionRows/reactionDesc、空态、labelOfReaction 全部保留。
- h5：本轮零改动——逐次 damage 已由 R15 在结算期采集落库（engine.ts:772），数据链完整；R16 战报展示、R17 验真 diff 口径不变。
- 不回退：R1–R17 诚实文案、25% AoE、顶栏脱敏、R5 AI 洞察卡、令牌锚点 v1/v3-r3/r6（tokens.css:12-14）不变；零新增视觉令牌。

> 公开仓库约束：本规范与配套测试不含任何域名/IP/JWT/口令/本机路径真值；所有路径均为仓库相对路径。