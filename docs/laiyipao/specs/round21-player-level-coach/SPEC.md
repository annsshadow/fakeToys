# R21 · 玩家卡点教练（跨局个人关卡诊断）契约与设计规范

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改业务代码）。
> 依据（2026-11-24 真代码审计）：GET /me/stars 只返回历史最佳星级（laiyipao/server/internal/httpapi/routes.go:152/:668-675，形状 {stars:{<id>:n}}），无尝试次数/最好波次/连败趋势；现有 GET /diagnose 是单局即时诊断（routes.go:312，stage.vue:765-766 仅在 d.stuck 时展示），玩家看不到跨局卡点规律。battle_records 已存 level_id/result/wave_reached/created_at（migrations/00004_battle.sql:19-44），具备跨局聚合的数据基础。R15–R20 生态不动，本轮独立新主题。

## 0. 真实缺口与依据（带行号）

1. 星级端点只有最好成绩：laiyipao/miniapp/src/api/client.ts:988-989 的 fetchMyStars() 请求 /me/stars，响应只有 stars map（历史最佳星级）；stage.vue:730 仅用它点亮星数，无法回答「这关打了几次、最好到第几波、是不是连败」。
2. 诊断端点是单局即时：laiyipao/server/internal/service/progression.go:375-387 的 DiagnoseFor 用 countConsecutiveLosses（:399-411 从 battle_records 按 user_id+level_id 倒序数量词 lose）算 failedTimes，:386 d.Stuck = failedTimes >= domain.StuckThreshold；domain/battle.go:923-927 钉死 StuckThreshold=2；stage.vue:765-766 只在 d.stuck 时展示卡片。它是「这一刻卡不卡」，不是「跨局卡点规律」。
3. 权威数据已在库：migrations/00004_battle.sql:19-44 的 battle_records 含 :22 level_id / :24 result（win/lose）/ :31 wave_reached / :43 created_at / :21 user_id；:45 idx_battle_records_user (user_id, created_at DESC) 索引支持按用户与时间窗口聚合。
4. 鉴权基线：server/internal/httpapi/middleware.go:407-418 的 requireUser 把 user_id 写入 c.Locals；routes.go:152 me.Get("/stars", s.myStars) 挂 me 组；新端点必须沿用 requireUser 并从会话取 user_id。

本规范补齐：新增玩家鉴权批量端点 GET /api/v1/me/level-coach，按用户+30天窗口从 battle_records 聚合每个已尝试关卡的尝试/胜场/最好波次/平均波次/最近结果/卡点标记与确定性教练建议；H5 选关页加「我的关卡教练」卡。教练定位辅助诊断，权威结算与 replay_hash 一律不动。

## 1. 后端 API 契约（新增，test-first）

```
GET /api/v1/me/level-coach?window_days=30   （新增；玩家鉴权，沿用 requireUser 中间件）
```

鉴权硬约束：从会话取 user_id（与 requireUser / c.Locals("user_id") 一致），所有 SQL 必须带 WHERE user_id = 当前登录用户；绝不返回他人数据，不接受前端传入的 user_id 参数，不因参数注入越权。test-first 必须包含「两个用户互不串数据」用例。

窗口参数：window_days 只接受 30（本轮固定窗口），非法值回 400；服务端按 UTC now 确定性计算窗口，不信任客户端时间。

响应（严格字段，snake_case），顶层 items 数组：每行逐字为 {level_id,attempts,wins,best_wave,avg_wave,last_result,stuck,coach}。空结果 items 为 []，不是 null。

| 字段 | 类型 | 含义 | 真源 |
| --- | --- | --- | --- |
| level_id | int | 关卡 ID | battle_records.level_id |
| attempts | int64 | 窗口内该关尝试局数 | 窗口内该用户该关 battle_records 行数 |
| wins | int64 | 窗口内该关胜场 | SUM(result='win') |
| best_wave | int | 窗口内最好到达波次 | MAX(wave_reached) |
| avg_wave | float64 | 平均到达波次 | SUM(wave_reached)/attempts；分母为 0 时归零，不产生 NaN |
| last_result | string | 最近一局结果（win/lose） | 按 created_at DESC, id DESC 取首行 result |
| stuck | bool | 是否卡点 | 最近连续失败次数 >= domain.StuckThreshold（=2），沿用 progression.go:386/:399-411 的连续失败口径 |
| coach | object | 辅助教练建议 | 由本行真实数据确定性推导，见 §3 |

stuck 口径裁决：沿用 domain/battle.go:927 的 StuckThreshold=2 与 progression.go:386 的 failedTimes >= domain.StuckThreshold；连续失败从该用户该关最近一局往前数，遇 win 即停（赢过说明关卡可过，之前的失败是试错不是卡住），与 countConsecutiveLosses（progression.go:399-411）语义一致。本轮「连续」在 30 天窗口内计算，窗口外的局不计入。

稳健性：所有聚合用 COALESCE 处理 null；int64 承接计数；分母为 0（无尝试局）时 avg_wave 归零、不产生 NaN/Infinity；无窗口记录走 items=[] 空态；非法 level_id 不产生行。

test-first：先加后端契约测试断言①两用户数据隔离②响应八字段③window_days=30 与非法值 400④stuck 连续失败阈值=2 且遇 win 重置⑤avg_wave 分母为 0 归零无 NaN⑥空结果 items=[]⑦coach 由真实数据确定性推导（同输入同输出、无硬编码反应名结论），再写聚合 SQL/端点；不改 /me/stars、/diagnose、stats.go 等现有端点。

## 2. H5 展示契约（固定选择器，供 E2E 与契约测试锚定）

在 stage.vue 选关页的诊断卡（:331-338）附近新增「我的关卡教练」卡；现有星级/诊断视图全部保留。

| 用途 | 选择器 | 语义 | 约束 |
| --- | --- | --- | --- |
| 容器 | .r21-level-coach | 关卡教练卡 | role=region；与星级/诊断卡并存，不替换 |
| 教练行 | .r21-coach-row | 单关教练摘要 | 每行带 [data-r21-level] 属性（值为 level_id），按 attempts 降序 |
| 空态 | .r21-coach-empty | 无尝试记录 | 逐字 近 30 天无关卡尝试记录 |
| AI 洞察 | .r21-ai-note | 辅助诊断结论 | 含可见汉字 AI、真实数据结论、依据行、CTA |

- 数据来源：新增 fetchMyLevelCoach() 调用 me 端点，替换「只看本地/只看星级」的推断；fetchMyStars()/diagnose() 保留供既有星级与单局诊断使用。
- 数值格式：avg_wave 经 toFixed(1) 确定性格式化，禁止裸插值长浮点 / NaN；stuck 行有视觉强调但不得宣称权威结论。
- 诚实空态：items=[] 时显示逐字 近 30 天无关卡尝试记录，不显示 undefined/NaN，不以历史星级冒充尝试记录。
- 窗口标注：展示 window_from/window_to 与 window_days=30，明示统计窗口。

## 3. coach 结构与 AI 四判据

coach 单行结构（逐字钉死）：{title,detail,suggestions,confidence}，全部由本行 attempts/wins/best_wave/avg_wave/stuck 确定性推导，禁止硬编码反应名或关卡名结论；无数据行不产出 coach。

在 .r21-ai-note 内同时具备四要素：

1. 可见汉字「AI」：标题形如 AI · 卡点教练。
2. 真实数据结论：由行数据确定性推导，例如 近 30 天该关尝试 X 次、胜 Y 次、最好到第 Z 波，连续失利建议先调整构筑——X/Y/Z 必须由 attempts/wins/best_wave 真实计算；无数据走空态，不做结论、不编造置信度。
3. 依据行：逐字 依据：近 30 天 battle_records 的 level_id/result/wave_reached 聚合；连续失利沿用 StuckThreshold=2。
4. 可执行 CTA：指向选关页对应关卡入口或战报列表，不新造路由、不冒充权威裁定。

## 4. 诚实红线（不可协商）

- 辅助诊断非权威结算依据，权威看 replay_hash：教练建议不参与结算、score、星级、loot，也不参与或覆盖 replay_hash；权威验真仍以 replay_hash 为准。
- 严格 WHERE user_id：me 端点只按当前登录用户过滤，不接受前端 user_id 参数；跨用户数据隔离是 test-first 必测项。
- 空态诚实：items=[] 走逐字空态 近 30 天无关卡尝试记录，不显示 undefined/NaN，不以历史星级冒充尝试记录。
- 分母诚实：分母为 0 时归零，不产生 NaN；连续失败口径沿用 domain.StuckThreshold=2，不自造阈值。
- 零真值：规范与实现不得写入任何域名/IP/JWT/口令/本机路径真值；API 地址只用回环或占位符。

## 5. test-first 契约测试设计

新增两份规范/基线守卫测试（读 SPEC 断言条款齐全，读三端源码断言既有基线不回退）：

1. laiyipao/miniapp/src/pages/stage/stage_r21_level_coach.contract.test.ts（miniapp vitest include: src/**/*.test.ts，vitest.config.ts:66）。specPath=join(__dirname, '../../../../../docs/laiyipao/specs/round21-player-level-coach/SPEC.md')；srcRoot=join(__dirname, '../..') 即 src；readSrc('pages/stage/stage.vue')/('api/client.ts')/('styles/tokens.css')。分组：字段与选择器 / 逐字文案与诚实红线 / AI 四判据 / 三端不回退（stage.vue 保留 fetchMyStars 与 diagnoseInfo.d.stuck、client.ts 保留 fetchMyStars 与 diagnose、tokens 三锚点无 --r21-）。
2. laiyipao/admin/tests/R21LevelCoach.contract.test.ts（admin vitest include: tests/**/*.test.ts，vitest.config.ts:55）。specPath=join(__dirname, '../../../docs/...')；srcRoot=join(__dirname,'..')；后端用 readSrc('../server/internal/domain/battle.go')/('../server/internal/service/progression.go')/('../server/internal/httpapi/routes.go')/('../server/migrations/00004_battle.sql')。分组：字段契约 / 选择器文案 / 诚实红线 / AI 四判据 / 三端不回退（StuckThreshold=2、countConsecutiveLosses、me.Get("/stars")、wave_reached 列、admin tokens 三锚点无 --r21-）。

运行命令：
```
cd laiyipao/miniapp && npx vitest run --config vitest.config.ts src/pages/stage/stage_r21_level_coach.contract.test.ts --reporter=verbose
cd laiyipao/admin && npx vitest run --config vitest.config.ts tests/R21LevelCoach.contract.test.ts --reporter=verbose
```

## 6. 三端分工与不回退

- backend（后续实施）：新增 GET /api/v1/me/level-coach，按 user_id + 30 天窗口从 battle_records 聚合八字段与确定性 coach；test-first；不改 /me/stars、/diagnose、stats.go。
- h5（后续实施）：stage.vue 加关卡教练卡 + fetchMyLevelCoach；星级视图、诊断卡、fetchMyStars/diagnose 全部保留。
- admin：本轮零改动；R18 /admin/reaction-damage-impact、R20 /admin/battle-efficiency、看板次数图均不动。
- 不回退：R1–R20 诚实文案、25% AoE、顶栏脱敏、AI 洞察卡、令牌锚点 v1/v3-r3/r6（admin tokens.css:12-14 / miniapp tokens.css:14-16）不变；零新增视觉令牌。

> 公开仓库约束：本规范与配套测试不含任何域名/IP/JWT/口令/本机路径真值；所有路径均为仓库相对路径。