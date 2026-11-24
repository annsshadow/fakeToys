# R19 复查报告：玩家伤害贡献面板（GET /me/reaction-damage-impact + H5 双视图）

- 任务：t161「R19复查：玩家伤害贡献面板真浏览器实测」，attempt_id=2fb61deb-e011-4a34-b97d-707eeb668b55。
- 判定：**PASS（五项验收全部通过，无缺陷）**。
- 方式：独立复查，未采信实施者自述；未改业务源码；未执行 git。
- 浏览器：agent-browser 自动化浏览器（**非物理真机**），视口 390×844；每步均采 DOM 探针与截图并亲读。
- 规范口径：docs/laiyipao/specs/round19-player-reaction-impact/SPEC.md（105 行）。

## 一、验收项与证据（按顺序）

### ① GET /me/reaction-damage-impact 只返回当前玩家数据（跨用户隔离）
- HTTP 实测（认证后 GET，脚本 docs/laiyipao/audits/round19-player-reaction-impact/verify-isolation.sh，结果落 gitignore 保护的 data/http-impact-*.json）：
  - QA 用户 10（有战报）：HTTP 200，window_days=30，window_from=2026-10-25，window_to=2026-11-24，excluded_old_battles=5，items=3 行。
  - QA 用户 11（无战报）：HTTP 200，excluded=0，items=[]（非 null）。
  - QA 用户 1（2 场旧战报、reactions_used={}）：HTTP 200，excluded=2，items=[]。
- SQL 独立核验（docker exec lyp-qa-pg psql）：用户 10 权威次数 = overheat 228 / steam_burst 203 / superconduct 191；与用户 10 端点返回的 count 逐字段一致。用户 1/11 端点不含用户 10 的任何伤害或次数，跨用户隔离成立。
- SQL 排除数核验：用户 10 近 30 天共 7 场，schema 不符/空 events 的旧战报 5 场，与端点 excluded=5 一致。

### ② count 口径与权威字段一致（非 events、非全平台视图）
- 源码复核：laiyipao/server/internal/service/me_reaction_damage_impact.go 的权威次数取自 battle_records.reactions_used（WHERE user_id=当前用户），伤害取自 reaction_timeline.events；handler laiyipao/server/internal/httpapi/handlers_r19.go 从 requireUser 的 user_id 取用户，不接受前端传入 user_id。
- 手算占比：总伤害 = 12175+7488+6746 = 26409；12175/26409 = 46.1%、7488/26409 = 28.4%、6746/26409 = 25.5%，与端点 damage_share_pct 逐项一致。
- 契约 11 守卫（laiyipao/miniapp/tests/R19ReactionDamageImpact.contract.test.ts）钉死「count 取 reactions_used，不用 events / dashboard_reaction_counts」口径，实跑 11/11 PASS。
- 服务端 R19 测试：cd laiyipao/server && go test ./internal/service/... ./internal/httpapi/... -run R19 => 4 条纯逻辑 PASS；3 条 DB E2E 因本机无管理库 DSN 诚实 SKIP，已由上述 HTTP 实测补足。

### ③ 旧战报与空态诚实
- 用户 1（旧战报 timeline=[]、reactions_used={}）：端点 items=[] 且 excluded=2，UI 无编造数据。
- 用户 11（新玩家）：端点 items=[] 且 excluded=0；H5 空态逐字显示「近 30 天无逐次反应伤害记录」，rows=0，AI 结论「暂无可用数据」，依据行与诚实红线仍在。
- 旧战报无 timeline 不计入伤害但 excluded 自增；UI 用「权威 N 次 · 事件 N 条」区分次数与伤害，分母差异措辞不混淆。

### ④ H5 reactions.vue 双视图 + AI 卡四判据 + 诚实红线真实渲染
- 契约测试 11/11 PASS；目标渲染测试 laiyipao/miniapp/src/pages/reactions/reactions_r19_impact.test.ts 4/4 PASS；miniapp 全量 vitest 194 files passed（1 skipped）/1790 tests passed（2 skipped）。
- 真浏览器（QA 用户 10）：
  - 次数视图：active=["次数"]，3 行「蒸汽爆发 权威 203 次 · 事件 53 条 / 过热 权威 228 次 · 事件 41 条 / 超导 权威 191 次 · 事件 46 条」，窗口「2026-10-25 ~ 2026-11-24（30 天）」，「已排除 5 条无逐次记录的旧战报」逐字。
  - 点击「伤害」视图：active=["伤害"]，3 行「蒸汽爆发 占比 46.1% · 总伤害 12175.0 / 过热 占比 28.4% · 总伤害 7488.0 / 超导 占比 25.5% · 总伤害 6746.0」。
  - AI 四判据：可见「AI · 我的反应伤害洞察」；真实结论「过热 触发 228 次但伤害贡献仅 28.4%，伤害主力是 蒸汽爆发（46.1%），疑似高频低伤」；依据行「依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准」逐字；可执行 CTA「查看权威结算 →」。
  - 诚实红线逐字「辅助分析非权威结算依据，权威请看 replay_hash」；依据行/CTA/诚实行均在卡片内。
  - 嗅探到 2 条 GET /me/reaction-damage-impact 响应，status=200，items 与 SQL 权威次数一致。
- 诚实边界：本卡为辅助分析，不参与结算与 replay_hash，权威仍以 replay_hash/权威次数为准；三端不回退由契约守卫覆盖（tokens.css 锚点 v1/v3-r3/r6 无 --r19-，admin 零改动，理论视图符号保留）。

### ⑤ 探针无报错、截图 sha 各异、亲读非登录页
- DOM 探针（window.__R19_PROBE__，document-start 注入并劫持 uni.request）：用户 10 有数据态与用户 11 空态均为 pageErrors=0 / consoleErrors=0 / apiFailures=0 / failedRequests=0；loginPage=false；location.hash=#/pages/reactions/reactions，title=反应图鉴。
- 截图（docs/laiyipao/audits/round19-player-reaction-impact/screenshots/，4 张均亲读确认为反应图鉴页、非登录页，sha256 两两不同）：
  - r19-01-count-view.png = b16088dbeab45f1aa53549aa820c221682c7f250875ae3cb4007e8520bd5a92b（390×844，次数视图，权威 203/228/191 次 + AI 洞察卡）
  - r19-02-damage-view.png = a34068c6a2d63aa427bb8548bdda234e423a8499b67a3933bf2de6b1f219dff7（390×844，伤害视图，占比 46.1%/28.4%/25.5% + 总伤害）
  - r19-03-damage-tall.png = f5d3e584fed9761a0d7d434d44731301505ffe4408d25114e197c0a81b084685（390×1100，同页长视口）
  - r19-04-empty-user11.png = 744a45cf5d666ca5edad237a7473c8eaa8d6f270086a9f4d9012e2c5f884a117（390×844，用户 11 空态逐字）

## 二、真值与清理
- 审计目录 ops_grep 精确模式（JWT / Bearer / guest_token 真值 / 个人绝对路径 / 连接串 / 私钥路径）扫描：0 命中。
- 含真 token 的审计注入脚本 auth-init.js 与临时测试配置 vitest.r19qa.config.ts 于收尾删除；QA 快照 data/qa-before.dump（sha256=af0058dee925c24dc636447f244252c1cc47f09810d60afa1ad1c6161af845b0）与 data/http-impact-*.json 由 .gitignore 规则 docs/laiyipao/audits/round19-player-reaction-impact/data/ 排除入库；auth-init.js 亦已加入忽略。
- QA 数据还原：docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < data/qa-before.dump 完成，exit=0，战报数 103 恢复。

## 三、命令清单（复现）
- cd laiyipao/miniapp && npx vitest run --config vitest.r19qa.config.ts --reporter=dot => 11/11 PASS。
- cd laiyipao/miniapp && npx vitest run --config vitest.config.ts --reporter=dot => 194 files /1790 tests PASS（1/2 skipped）。
- cd laiyipao/server && go test ./internal/service/... ./internal/httpapi/... -run R19 => 纯逻辑 PASS，DB E2E SKIP（无管理库 DSN）。
- bash docs/laiyipao/audits/round19-player-reaction-impact/verify-isolation.sh => 三用户 HTTP 隔离 + SQL 权威次数核验。
- bash docs/laiyipao/audits/round19-player-reaction-impact/e2e-h5.sh / e2e-empty.sh => 真浏览器双视图/空态 DOM 探针与截图。

## 四、结论
R19 五项验收全部 PASS，无缺陷。玩家维度端点与 H5 面板均按 SPEC 实现，跨用户隔离、权威次数口径、旧战报诚实空态、AI 四判据与诚实红线全部实测成立；未修改业务源码，未执行 git。
