# R20 · 战斗效率趋势分析三端契约（battle_records 权威全量聚合）

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改业务代码）。
> 依据（2026-11-24 真代码审计）：battle_records 已存 shots/hits/heat_max/leaked/duration_ms/kills 全字段，但 H5 命中率效率卡只聚合已展开的局（本地 details 缓存），样本残缺；服务端无全量聚合/时间趋势/AI 洞察，admin 也无全平台战斗效率视角。R15–R19 是反应时间线生态，本轮为独立主题，不依赖 reaction_timeline。

## 0. 真实缺口与依据（带行号）

1. H5 效率卡样本残缺：laiyipao/miniapp/src/pages/battles/battles.vue:491-503 的 G132 `efficiencyStats` computed 用 `const ds = Object.values(details.value)`，shots/hits 只累加已展开局的 details 缓存，未展开的战报被漏掉；没有按日/周趋势、没有服务端全量口径。
2. 权威字段已在库但未全量聚合：laiyipao/server/migrations/00004_battle.sql:19-44 的 battle_records 含 :32 `duration_ms` / :33 `shots` / :34 `hits` / :28 `kills` / :29 `leaked` / :36 `heat_max` / :43 `created_at`；:45 有 `idx_battle_records_user (user_id, created_at DESC)` 索引，具备按 user + 时间窗口聚合的条件。
3. 服务端无效率聚合端点：laiyipao/admin/src/api/index.ts:193-203 只有 `fetchDashboard()` 与 R18 的 `fetchReactionDamageImpact()`（/admin/reaction-damage-impact，保留）；无 battle-efficiency 端点。
4. 三端现有锚点必须保留：admin/miniapp 的 tokens.css 三锚点 v1/v3-r3/r6（admin:12-14 / miniapp:14-16）；R18 admin 反应伤害端点、R19 玩家伤害贡献端点、R15–R17 时间线生态均不动。

本规范补齐：新增玩家/管理员两端战斗效率聚合 API（同口径），H5 与 admin 各加效率趋势卡（含 AI 四判据）。效率定位辅助诊断，权威结算与 replay_hash 一律不动。

## 1. 后端 API 契约（新增，test-first）

```
GET /api/v1/me/battle-efficiency?window_days=30   （新增；玩家鉴权，严格 WHERE user_id = 当前登录用户）
GET /api/v1/admin/battle-efficiency               （新增；管理员鉴权，全平台同口径）
```

鉴权硬约束（me 端点）：从会话取 `user_id`（与 requireUser / c.Locals("user_id") 一致），所有 SQL 必须带 `WHERE user_id = 当前登录用户`；绝不返回他人数据，不接受前端传入的 user_id 参数。test-first 必须包含「两个用户互不串数据」用例。

窗口参数：`window_days` 查询参数仅接受 30（本轮固定窗口）；非法值回 400 或按 30 处理须在 SPEC 固定为「只接受 30，其他值 400」，服务端按 UTC now 确定性计算 window_from/window_to，不信任客户端时间。

响应（严格字段，snake_case；两端同结构）：

| 字段 | 类型 | 含义 | 真源 |
| --- | --- | --- | --- |
| `window_days` | int | 统计窗口天数，固定 `30` | 常量，响应显式标注 |
| `window_from` / `window_to` | string | 窗口起止日期（UTC，YYYY-MM-DD） | 服务端按 now-30d 确定性计算 |
| `battles` | int64 | 窗口内 battle_records 行数（me 端点按 user_id 过滤） | count(*) from battle_records where created_at in window |
| `shots` | int64 | 窗口内 shots 之和 | SUM(shots) |
| `hits` | int64 | 窗口内 hits 之和 | SUM(hits) |
| `accuracy_pct` | float64 | 命中率 `hits / shots * 100` | 服务端计算；`shots=0` 时全为 `0`，不产生 NaN |
| `avg_heat_max` | float64 | 平均峰值热量 `SUM(heat_max)/battles` | battles=0 时为 0，不除零 |
| `leaked_per_battle` | float64 | 平均每局漏怪 `SUM(leaked)/battles` | battles=0 时为 0，不除零 |
| `avg_duration_ms` | float64 | 平均战斗时长 `SUM(duration_ms)/battles` | battles=0 时为 0，不除零 |
| `buckets` | array | 按日（或周）时间桶 | 见下 |
| `ai_insight` | object | 辅助诊断结论 | 见 §3，无数据为空态对象 |

buckets 单行字段（逐字钉死）：`{bucket, battles, shots, hits, accuracy_pct}`。bucket 为 YYYY-MM-DD（按日）或 YYYY-Www（按周），本轮固定按日；空窗口 `buckets=[]`（不是 null）。

count 口径裁决：battles 权威就是 battle_records 行数（30 天窗口，me 端点额外 WHERE user_id），不依赖 reaction_timeline、不依赖 details 缓存、不依赖 stats.go 物化视图；accuracy = hits/shots；所有除法在分母为 0 时归零，禁止 NaN/Infinity。空结果 `items` 概念在本端点表现为 `battles=0, buckets=[], ai_insight 空态`，诚实返回而非编造。

test-first：先加后端契约测试断言①me 两用户数据隔离②响应全部字段③`window_days=30`④`battles` 来自 battle_records 行数⑤`shots=0` 与 `battles=0` 时不产 NaN⑥buckets 按日有序⑦admin 与 me 同口径（同一聚合函数仅 user_id 过滤不同），再写 SQL/端点。

## 2. H5 / admin 展示契约（固定选择器，供 E2E 与契约测试锚定）

在 battles.vue 命中率效率卡（:491-503）附近新增效率趋势卡（G132 原卡保留，不删除）；admin DashboardView.vue 新增全平台效率卡。两端共用下列语义选择器（r20 前缀）：

| 用途 | 选择器 | 语义 | 约束 |
| --- | --- | --- | --- |
| 容器 | `.r20-efficiency-trend` | 效率趋势卡 | role=region；与 G132 卡并存，不替换 |
| 汇总行 | `.r20-efficiency-row` | 窗口汇总（场次/命中率/热量/漏怪/时长） | 数值来自响应字段，toFixed(1) 格式化 |
| 空态 | `.r20-efficiency-empty` | 无战斗记录 | 逐字「近 30 天无战斗记录」 |
| 趋势条 | `.r20-trend-bar[data-r20-bucket]` | 按日效率桶 | data-r20-bucket 为 YYYY-MM-DD，按时间正序 |
| AI 洞察 | `.r20-ai-note` | 辅助诊断结论 | 含可见汉字「AI」、真实数据结论、依据行、CTA |

说明：本轮非反应主题，[data-r20-reaction] 不需要（禁止把它写进契约）。

- H5 数据获取：新增 fetchMyBattleEfficiency()，从 me 端点读全量聚合，替代 G132 只依赖 details 缓存的残缺样本；G132 efficiencyStats 保留作为即时本地反馈，不删除、不改语义。
- admin 数据获取：新增 fetchAdminBattleEfficiency()，从 admin 端点读全平台聚合；DashboardView 既有反应分布卡/R18 伤害贡献卡不动。
- 数值格式：accuracy_pct / avg_heat_max / leaked_per_battle / avg_duration_ms 经 `toFixed(1)` 确定性格式化（沿用 admin BattlesView.vue:125/284 与 H5 惯例），禁止裸插值长浮点 / NaN。
- 空态：`battles=0` 时显示逐字「近 30 天无战斗记录」，不显示 undefined/NaN；window_from / window_to 仍需展示，如实标注窗口。
- 趋势条：buckets 按 bucket 升序渲染，`data-r20-bucket` 与响应逐字一致；无桶时走空态而非空图。

## 3. AI 四判据（延续页面既有 AI 洞察模式）

在 `.r20-ai-note` 内同时具备四要素：

1. 可见汉字「AI」：标题形如「AI · 战斗效率诊断」。
2. 真实数据结论：由响应确定性推导，例如「近 30 天命中率 X%，峰值热量呈上升趋势，建议关注 heat 管理」——X 由 `hits/shots` 真实计算，趋势由 buckets 真实比较；无数据走空态，不做结论、不编造置信度。
3. 依据行：逐字「依据：近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准」。
4. 可执行 CTA：H5 指向战斗入口或关卡页，admin 指向关卡管理或战报列表；不新造路由、不冒充权威裁定。

## 4. 诚实红线（不可协商）

- 辅助诊断非权威：效率趋势是辅助诊断，不参与结算、score、星级、loot，也不参与或覆盖 replay_hash；权威结算与验真仍以 replay_hash 为准。
- 空态诚实：`battles=0` 走逐字「近 30 天无战斗记录」，不编造效率值；`shots=0` / 分母为 0 一律归零不产 NaN。
- 旧数据如实：响应 `window_days` 与 window_from/window_to 必须展示，明示统计窗口；不全量历史、不假装覆盖窗口外数据。
- 样本口径诚实：battles 权威为 battle_records 行数，不依赖 details 缓存、不依赖 reaction_timeline；G132 本地即时卡保留但不冒充全量。
- 隔离：me 端点严格按 user_id 过滤，不返回他人数据；admin 端点仅管理员可见。
- 零真值：规范与实现不得写入任何域名/IP/JWT/口令/本机路径真值；API 地址只用回环或占位符。

## 5. test-first 契约测试设计

新增两份规范/基线守卫测试（读 SPEC 断言条款齐全，读源码断言既有基线不回退；实施者删改规范或破坏锚点即变红）：

1. laiyipao/admin/tests/R20CombatEfficiency.contract.test.ts（admin vitest include: tests/**/*.test.ts，vitest.config.ts:55，jsdom + setup 垫片齐备）。specPath=join(__dirname, '../../../docs/laiyipao/specs/round20-combat-efficiency-trends/SPEC.md')；srcRoot=join(__dirname,'..')；readSrc(p)=readFileSync(join(srcRoot,p),'utf-8')，前端源码带 src/ 前缀，后端用 ../server/... 相对路径。分组：API 字段契约 / 选择器文案 / 诚实红线 / AI 四判据 / 三端不回退（admin tokens 三锚点、DashboardView 反应卡与 fetchReactionDamageImpact 保留、无 --r20- 新令牌）。
2. laiyipao/miniapp/src/pages/battles/battles_r20_efficiency.contract.test.ts（miniapp vitest include: src/**/*.test.ts，vitest.config.ts:66）。specPath=join(__dirname,'../../../docs/laiyipao/specs/round20-combat-efficiency-trends/SPEC.md')；readSrc(p)=readFileSync(join(__dirname,'../..',p),'utf-8') 或按相对层级定位到 laiyipao/miniapp。分组：字段与选择器 / 逐字文案 / AI 四判据 / 三端不回退（battles.vue 保留 G132 efficiencyStats 与 reactionRef 无关符号、miniapp tokens 三锚点无 --r20-）。

运行命令：
```
cd laiyipao/admin && npx vitest run --config vitest.config.ts tests/R20CombatEfficiency.contract.test.ts --reporter=verbose
cd laiyipao/miniapp && npx vitest run --config vitest.config.ts src/pages/battles/battles_r20_efficiency.contract.test.ts --reporter=verbose
```

## 6. 三端分工与不回退

- backend（后续实施）：新增 GET /api/v1/me/battle-efficiency 与 GET /api/v1/admin/battle-efficiency，同一聚合函数（me 额外 WHERE user_id），30 天窗口按 battle_records.created_at；test-first；不改任何现有端点/表结构。
- h5（后续实施）：battles.vue 加效率趋势卡 + fetchMyBattleEfficiency，G132 本地卡保留；反应页/R15–R19 时间线生态不动。
- admin（后续实施）：DashboardView 加全平台效率卡 + fetchAdminBattleEfficiency；反应分布卡、R18 伤害贡献卡、fetchDashboard 不动。
- 不回退：R1–R19 诚实文案、25% AoE、顶栏脱敏、AI 洞察卡、令牌锚点 v1/v3-r3/r6（admin tokens.css:12-14 / miniapp tokens.css:14-16）、R18 /admin/reaction-damage-impact、R19 /me/reaction-damage-impact、R15–R17 时间线均不变；零新增视觉令牌。

> 公开仓库约束：本规范与配套测试不含任何域名/IP/JWT/口令/本机路径真值；所有路径均为仓库相对路径。