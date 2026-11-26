# R18 复查报告：后端伤害聚合 + admin 双视图真浏览器实测

- 任务：t155 / attempt 2（attempt_id: 0226a818-2aff-45ff-a5ff-6b4428bf4d16）
- 复查人：qa-e2e-wb（只读复查，未改动业务源码，未执行 git）
- 整体判定：**PASS**，四项验收全部通过，未发现阻断缺陷。
- 测试环境说明：本轮为 agent-browser 真实浏览器自动化实测，非物理真机；页面、接口与数据库均为本机 QA 环境真实运行。

## 一、后端聚合 API：test-first 与权威数据核验

- 规范 `docs/laiyipao/specs/round18-reaction-damage-impact/SPEC.md`（99 行）确认：`GET /api/v1/admin/reaction-damage-impact`，窗口 30 天，字段 `window_days/window_from/window_to/excluded_old_battles/items`，单条 `reaction/total_damage/damage_share_pct/events/count`；`count` 必须取 `dashboard_reaction_counts` 权威次数，不得以 `events` 冒充；旧战报（无 timeline / 空 events / schema 不符）计入 `excluded_old_battles`。
- RED 基线：`git show HEAD:laiyipao/admin/src/views/DashboardView.vue | grep -c r18-` = 0；`R18ReactionDamageImpact.render.test.ts` 与 `laiyipao/server/internal/service/reaction_damage_impact.go` 在 HEAD 中不存在。
- 后端实现见 `laiyipao/server/internal/service/reaction_damage_impact.go:39` `AggregateReactionDamageImpact(...)` 与 `laiyipao/server/internal/httpapi/routes.go:335` 路由。
- `cd laiyipao/server && go test ./internal/service/... ./internal/httpapi/... -run "R18|ReactionDamageImpact"`：3 个纯逻辑测试 PASS；2 个 DB E2E 因本机无管理库 DSN 诚实 SKIP，改由下面真实 HTTP 实测补证；`gofmt -l` 为空，`go vet` exit 0。
- 真实接口（管理员登录取 token 后）返回 200：

| reaction | total_damage | damage_share_pct | events | count（权威） |
|---|---:|---:|---:|---:|
| steam_burst | 12175 | 46.1% | 53 | 306 |
| overheat | 7488 | 28.4% | 41 | 507 |
| superconduct | 6746 | 25.5% | 46 | 306 |

NaN
- 独立 SQL 核验脚本 `verify-agg.sql`（仅占位/回环，不含真值）：全部 30 天聚合逐字段与 API 一致；`battle_records` 中 378 按反应手算 steam_burst 5239/23、superconduct 4472/27、overheat 3438/19（合计 13149，API 为全库多局汇总故不同）；339 空 schema/events=0、341 v1/events=71、378 v1/events=69；排除旧战报数 101 与 API 一致；`dashboard_reaction_counts` 数值 overheat=507/superconduct=306/steam_burst=306 与 API `count` 一致，证明未用 events 冒充权威次数。占比手算 12175/(12175+7488+6746)=46.1% 与 API 一致。

## 二、admin 双视图：契约、渲染与类型检查

- 契约测试 `laiyipao/admin/tests/R18ReactionDamageImpact.contract.test.ts`（9 守卫）与渲染测试 `R18ReactionDamageImpact.render.test.ts`（10 用例）：`cd laiyipao/admin && npx vitest run --config vitest.config.ts tests/R18ReactionDamageImpact.contract.test.ts tests/R18ReactionDamageImpact.render.test.ts` => 2 files / 19 tests ALL PASSED。
- admin 全量 `npx vitest run --config vitest.config.ts` => 65 files / 741 tests ALL PASSED；`npx vue-tsc --noEmit` exit 0。
- 次数视图（原有能力）保留：契约含 `stats.go ReactionCount`、`reactionRef/reactionChart/x.count/尚无反应触发记录/labelOfReaction` 锚点；令牌锚点 `tokens.css` 的 v1/v3-r3/r6 保留且无 `--r18-`；测试全绿即证。
- 源文件中 R18 选择器命中 34 处（`laiyipao/admin/src/views/DashboardView.vue:403` 的 `<section class="r18-damage-impact" role="region">` 等），非口头声明。

## 三、真浏览器实测（agent-browser，非物理真机）

- 本机 QA 环境：后端 API `:8080`、admin Vite `:5175` 真实启动；新建 session `r18f2`，viewport 1440x900，打开 `http://127.0.0.1:5175/laiyipao-admin/#/admin/dashboard`（非登录页）。QA 快照 `data/qa-before.dump`（sha256 `93d5e431ce07b6dc3af79613fc7c3cd896b749b8bb6f56d6c88523765355ad69`，已忽略入库）。
- 初始默认「次数」视图：`countPressed=true`、`damagePressed=false`、`r18=true`、`rows=[]`，未发生能力回退。
- 点击 `.r18-view-damage` 后 DOM 探针：`damagePressed=true`，3 行 `[steam_burst: 蒸汽爆发 46.1% 12175 306]`、`[overheat: 过热 28.4% 7488 507]`、`[superconduct: 超导 25.5% 6746 306]`；窗口文案逐字「统计窗口：近 30 天（2026-10-25 至 2026-11-24）」，排除文案逐字「已排除 101 条无逐次记录的旧战报（不计入伤害）」，诚实红线逐字「伤害贡献为辅助分析；触发次数以物化视图为准，不参与结算与 replay_hash」。
- AI 洞察卡真实渲染：「AI · 反应伤害洞察 伤害贡献最高的是 蒸汽爆发（46.1%），未见明显高频低伤或低频高伤失衡。依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 dashboard_reaction_counts 为准 前往战报列表验真 →」，AI 四判据（汉字 AI、真实数据结论、依据行、可执行 CTA）齐全。
- 探针 `window.__R18_PROBE__`：接口一条 200 且响应体与 SQL 核验逐字段一致；`pageErrors`、`consoleErrors`、`apiFailures`、`failedRequests` 全为空数组（对应 pe/ce/af/fr = 0）。
- 切回 `.r18-view-count` 得到次数视图截图，双视图切换闭环成立。

### 决定性截图（均已亲读，非登录页）

| 文件 | sha256 | 说明 |
|---|---|---|
| screenshots/r18-01-damage-top.png | c20354846ec70cdecda66f41d1e190cdef3d55ad53654b24d5812a4e36f16581 | 伤害贡献视图 3 行真实数据 + AI 洞察；「伤害贡献」高亮 |
| screenshots/r18-03-count-view.png | 7e38ff48ca6228303d2c2e71427ef0636a82e720fde0f7dedc8af7b399a988b3 | 同一卡片切回次数视图；「次数」高亮，证明原视图保留 |

## 四、诚实红线与真值清理

- 红线核对：伤害贡献为辅助线索，不改 `stats.go` 的 `ReactionCount`、不参与结算与 `replay_hash`、H5 零改动；API `count` 来自权威物化视图；旧战报诚实排除并展示数量；空数据不会产出 NaN。以上由契约 9 守卫 + 渲染 10 用例 + API 实测共同证明。
- 零真值：含真 token 的临时 `probe.js` 已删除；`mint-probe.sh`、`data/`、`auth-init.js`、`probe.js` 均写入 `.gitignore`；报告与证据正文不含 JWT、口令、真实域名/IP 或个人路径（本地回环端口仅用于说明测试环境）。
- QA 数据：收尾执行 `pg_restore` 从 `data/qa-before.dump` 还原，随后确认接口健康；本轮未提交任何 git 变更。

## 五、结论

四项验收全部达标。R18 后端聚合与 admin 双视图是可信闭环：权威次数与辅助伤害分离、旧战报诚实排除、DOM 真数据、零浏览器错误、截图各异且亲读通过。**判定 PASS**。
