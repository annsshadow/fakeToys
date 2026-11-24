# R18 设计终审报告 · admin 运营看板反应伤害贡献分析

> 终审人：design-reviewer-wb（独立复核，不采信任何成员自述）
> 任务：t156 / attempt 2 / attempt_id=2cf57ac1-cd12-43c6-be0f-2aa7f007d672
> 对象：t152 规范 + t153 后端聚合 + t154 admin 双视图 + t155 复查报告
> 日期：2026-11-24
> 测试环境说明：E2E 为 agent-browser 真实浏览器自动化（非物理真机），页面/接口/数据库为本机回环 QA 环境真实运行。

## verdict = pass

六项门禁全部达标（六项全 PASS），未发现任何新的真实缺陷。终审方式：亲读 SPEC/契约/渲染测试/后端实现/前端实现/复查报告/DOM 探针日志，亲算截图 SHA256，亲读决定性截图，独立复跑测试。

---

## 门禁① SPEC 契约一致 —— PASS

- 规范 `docs/laiyipao/specs/round18-reaction-damage-impact/SPEC.md`（99 行）六节齐全：§0 真实缺口（stats.go:72-76 ReactionCount 只有 reaction/count；DashboardView.vue:142-162 只画次数柱；damage 数据已在 reaction_timeline 未被聚合）、§1 后端 API 契约（GET /admin/reaction-damage-impact，window_days=30 + window_from/to + excluded_old_battles + items 五字段）、§2 固定选择器与逐字文案、§3 AI 四判据、§4 诚实红线、§5 契约分组、§6 三端不回退。
- 契约测试 `laiyipao/admin/tests/R18ReactionDamageImpact.contract.test.ts`（90 行 9 守卫）逐项钉死字段/选择器/逐字文案/诚实红线/AI 四判据/三端不回退基线。
- 独立复跑：`vitest run tests/R18ReactionDamageImpact.contract.test.ts tests/R18ReactionDamageImpact.render.test.ts` → 2 files / 19 tests ALL PASSED，exit 0。

## 门禁② 后端伤害聚合诚实 —— PASS

亲读 `laiyipao/server/internal/service/reaction_damage_impact.go`（185 行）逐项确认：

- **权威次数不被冒充**：`AdminReactionDamageImpact` 的 count 来自独立查询 `SELECT reaction, SUM(count) FROM dashboard_reaction_counts GROUP BY reaction`，注入 `authCounts[key]`；与 stats.go:72 ReactionCount / Dashboard.ReactionUsage 同源。真 API 返回 count=306/507/306 与复查报告 SQL 独立核验的物化视图数值一致，证明未用 events（53/41/46）冒充。
- **旧战报如实排除**：独立 COUNT 查询统计窗口内 `reaction_timeline IS NULL OR schema_version IS DISTINCT FROM 'reaction-timeline-v1' OR events IS NULL OR jsonb_array_length(events)=0` 的战报数，回传 `excluded_old_battles=101`，不假装全量、不从 reactions_used 反推补洞。
- **稳健性**：SQL 先按 created_at 窗口（走索引）过滤再 `jsonb_array_elements` 展开；`validReactionKeys()` 过滤未知 key；`finalizeReactionDamageImpact` 中 SUM=0 时占比全置 0 不产 NaN；`items := make([]..., 0, len)` 保证空结果为 `[]` 非 null；damage_share_pct 用 `math.Round(x*1000)/10` 确定性一位小数。
- **零污染**：不改 stats.go ReactionCount 与次数分布查询；不涉及结算、score、loot、星级、replay_hash。
- 独立复跑 `go test ./internal/service/... -run 'R18|ReactionDamageImpact' -count=1` → ok，exit 0（3 个纯逻辑测试全绿；DB E2E 在本机无管理库 DSN 环境由真 HTTP 实测补证，属诚实 SKIP 非缺陷）。

## 门禁③ admin 双视图 + AI 四判据 + 确定性格式化 —— PASS

亲读 `laiyipao/admin/src/views/DashboardView.vue`（R18 相关命中 23 处）：

- 固定选择器齐全：`.r18-damage-impact`(role=region, :403) / `.r18-view-count` + `.r18-view-damage`(双视图 aria-pressed, :407-408) / `.r18-window-note`(:412) / `.r18-excluded-note`(:413) / `.r18-damage-row`(data-r18-reaction, :418) / `.r18-ai-note`(role=note, :426) / `.r18-honesty-note`(:432)。默认次数视图，不改变既有体验。
- 服务端取名：`labelOfReaction(reactionLabel.value, key)`(:113)，不硬编码中文名；未知 key 兜底原始 key。
- 确定性格式化：`fmtImpactNumber`(整数 toFixed(0)、非整数 toFixed(digits))、`pct`(有限数 toFixed(1)、非有限数 '—')，无裸插值长浮点 / NaN。
- AI 四判据齐备：可见汉字「AI」(:403/:426)、真实数据结论（高频低伤 :100 / 低频高伤 :103 / 最高贡献 :109 均由 count 与 damage_share_pct 真算）、依据行逐字「依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 dashboard_reaction_counts 为准」、可执行 CTA（前往战报列表验真 →，指向 /admin/battles）。空数据走空态不编造置信度。
- 诚实说明逐字「伤害贡献为辅助分析；触发次数以物化视图为准，不参与结算与 replay_hash」；空数据逐字「近 30 天无逐次反应伤害记录」；excluded=0 也如实显示。
- 渲染测试 `R18ReactionDamageImpact.render.test.ts`（10 用例）覆盖降序、格式化、空态、AI 四判据、排除标注、次数图不回退，全绿。

## 门禁④ 真浏览器决定性证据 —— PASS

- **截图 SHA256 亲算**（sha256sum，两两不同，与复查报告一致）：
  - `r18-01-damage-top.png` = `c20354846ec70cdecda66f41d1e190cdef3d55ad53654b24d5812a4e36f16581`
  - `r18-03-count-view.png` = `7e38ff48ca6228303d2c2e71427ef0636a82e720fde0f7dedc8af7b399a988b3`
- **亲读 r18-01**：非登录页（数据看板），AI·反应伤害贡献洞察卡（近 30 天）+「伤害贡献」视图高亮，窗口逐字「统计窗口：近 30 天（2026-10-25 至 2026-11-24）」+排除「已排除 101 条无逐次记录的旧战报（不计入伤害）」，3 行真实数据（蒸汽爆发 46.1%/12175/306、过热 28.4%/7488/507、超导 25.5%/6746/306），AI 洞察真实结论+依据行，顶栏脱敏标签（API 127.0.0.1:8080，回环非真值）。
- **亲读 r18-03**：非登录页，同卡「次数」视图高亮，证明原次数视图保留不回退。
- **DOM 探针日志**（`e2e-final2.log`，140 行）决定性字段：`loginPage=false`、`r18=true`、`rows`=3 行真实数据、`windowNote`/`excluded`/`honesty` 逐字命中、`ai` 含 AI 四判据全文；`probe.pageErrors=[] / consoleErrors=[] / apiFailures=[] / failedRequests=[]`（全 0）；`impactResponses[0].status=200`，响应体逐字段与 SQL 独立核验一致。探针模板 `probe.tpl.js` 只做 token 占位符 `__R18_TOKEN__` 注入与只读响应 clone，不篡改任何字段。
- 覆盖边界（不判 FAIL）：r18-03 视口内次数柱状图本体未入镜，由渲染测试「次数图与权威空态保留不回退」用例覆盖；`r18-02-damage-scroll.png` 为滚动辅助截图，未列为决定性证据。

## 门禁⑤ 零真值 + R1-R17 不回退 —— PASS

- **零真值**：含真实 guest_token 的 `probe.js` 已删除；`.gitignore`（亲读 :end）已将 `round18-reaction-damage-impact/data/`（含 qa-before.dump）、`auth-init.js`、`mint-probe.sh`、`probe.js` 全部忽略。SPEC/报告/E2E 脚本正文只含仓库相对路径、回环 127.0.0.1 与占位符，无域名/IP/JWT/口令/个人路径真值。
- **令牌锚点**：`tokens.css:12-14` 三锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 原样保留，无 `--r18-` 新增视觉变量（grep 亲验）。
- **次数图不回退**：DashboardView.vue 仍含 `reactionRef`(:38)/`reactionChart`(:41)/`x.count`(:209)/「尚无反应触发记录」(:399)/`reactionRows`(:130)/`labelOfReaction`(:113)；次数图 series.data 仍取 `items.map(x => x.count)`。
- **顶栏脱敏**：`AdminLayout.vue:178` 调 `maskApiBase(...)`，`apiLabel.ts:23` 规则在。
- **两步确认**：`ElMessageBox.confirm` 在 6 个视图（ContentView/DefensesView/EconomyView/LevelsView/RecomputesView/UsersView）保留。
- **引擎侧**：25% AoE（`engine.ts:113-115` splash=reactionDamage>>2=25%）与逐次反应采集（`reactionTimelineEvents` :607 / `reactionTimeline()` :772 / push :1759-1760）均在，H5 本轮零改动口径成立。

## 门禁⑥ 新真实缺陷裁决 —— PASS（无新缺陷）

- 复查报告低优先级观察项终审复算：两张决定性截图 SHA256 经本人亲算两两不同（不存在 R16 曾出现的「两图同 sha」笔误）；次数柱状图未入 r18-03 视口属截图覆盖边界非功能缺陷（已由渲染测试覆盖）。
- 六门禁证据链自洽：契约→后端聚合→admin 渲染→真浏览器 DOM→零真值/不回退，无相互矛盾。未发现任何阻断或中低优先级真实缺陷。

---

## 终审结论

R18「admin 运营看板反应伤害贡献分析」六项设计门禁全部 PASS，verdict=**pass**。权威次数与辅助伤害分离、旧战报诚实排除、空数据不编造、零真值、R1-R17 不回退均有决定性证据支撑。无新真实缺陷，无需 needs_revision，可进入后续收官流程。
