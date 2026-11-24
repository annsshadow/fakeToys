# R21 设计终审报告：玩家卡点教练（跨局个人关卡诊断）

- 任务：t175「R21终审：六门禁设计复审」，attempt_id=1d71fdb6-5f99-4c20-ae7e-054fcbd85c6f。
- 角色：design-reviewer-wb（设计终审 / workbuddy 路由）。
- 判定：**verdict = pass**（六门禁全部通过，无需返工的结构性缺陷）。
- 方式：独立终审，不采信实施/复查成员自述；亲读规范、三端源码、证据 JSON、截图，独立复跑测试与 SHA256、gitignore 生效核验。
- 浏览器证据形态：agent-browser 自动化浏览器（**非物理真机**），视口 390×844，本地 127.0.0.1 回环，已如实标注。
- 队长特别关注五点已逐项独立核实：①严格 WHERE user_id 跨用户隔离 ②stuck=连续失败≥2 遇 win 即停 ③分母0归零无 NaN ④空态 coach 不编造结论 ⑤R15–R20 不回退。

---

## 门禁① SPEC 落地 + 契约一致 + 测试绿 — PASS

- 亲读 `docs/laiyipao/specs/round21-player-level-coach/SPEC.md`（105 行）：§0 真实缺口（/me/stars 只有历史最佳星级、/diagnose 是单局即时、battle_records 已具跨局聚合字段）、§1 后端 API 契约（GET /api/v1/me/level-coach?window_days=30，八字段、严格 WHERE user_id、window_days 仅 30、stuck 阈值与遇 win 即停、分母归零、空 items=[]）、§2 H5 选择器与逐字文案、§3 coach 结构与 AI 四判据、§4 诚实红线、§5 test-first、§6 三端分工不回退——条款齐全。
- 契约测试独立复跑（本人执行）：
  - miniapp `stage_r21_level_coach.contract.test.ts`（6）+ `stage_r21_coach.test.ts`（5）= **11/11 通过**，exit=0。
  - admin `R21LevelCoach.contract.test.ts` = **8/8 通过**，exit=0。
  - 两份契约均逐字钉死端点/八字段/选择器/空态/诚实红线/AI 四判据/令牌 v1-v3-r3-r6 锚点且无 --r21-。
- 结论：SPEC 与契约、实现三方一致。

## 门禁② 后端聚合诚实（user_id 隔离 + 归零 + 空态 + 阈值）— PASS

- 亲读 `laiyipao/server/internal/service/level_coach.go`（213 行）：
  - **严格 user_id 隔离**：`MeLevelCoach(ctx, userID)` 两条 SQL（聚合 + ROW_NUMBER 最近结果）均 `WHERE user_id = $1`；handler `handlers_r21.go` 用 `userIDFrom(c)` 从会话取 user_id，不接受前端传参。routes.go:136 `me.Get("/level-coach", s.meLevelCoach)` 挂在鉴权组。
  - **stuck 遇 win 即停**：`levelCoachStuck` 遍历 newest-first 结果序列，遇非 lose 即 break，头数 `>= domain.StuckThreshold(=2)` 返回 true；SQL 用 `ROW_NUMBER() OVER (PARTITION BY level_id ORDER BY created_at DESC, id DESC)` 取每关最近 2 局，与 progression.go 的 countConsecutiveLosses 语义一致。
  - **分母0归零无 NaN**：`levelCoachAverage` 在 count<=0 时返回 0；空 `AggregateLevelCoach(nil)` 返回非 nil 空 items（JSON 为 []）；`deriveLevelCoachAdvice` 对 attempts<=0 返回空 coach（不编造结论）。
  - UTC 窗口确定性：`levelCoachWindow` 取 now.UTC() 当日 00:00，from=当日-30d、to=次日 00:00；响应 window_days/window_from/window_to 如实展示。
  - 八字段 snake_case 钉死，按 attempts 降序（同次数按 level_id 升序）稳定排序。
- 独立复跑后端纯逻辑测试：`go test ./internal/service/... -run TestAggregateLevelCoach -count=1` → **ok，exit=0**（含 FieldsAndStuck、EmptyAndZeroDivision 两条，覆盖遇 win 即停/连续失败/分母归零/空 items/确定性 coach/八字段 JSON）。
- 三用户隔离 JSON 亲读（`data/http-coach-*.json`）：用户10 三关（level1/5/40 各 attempts=3/3/1、wins=3/3/1、stuck=false）；用户100 仅 level40（attempts=3/wins=0/best_wave=7/avg_wave=6.3/last_result=lose/stuck=true）；用户11 items=[]。互不串数据。参数守卫实测 window_days=7 与重复 30 → 400、无鉴权 → 401（复查报告 + 端点测试双证）。

## 门禁③ H5 AI 四判据 + 诚实文案 — PASS

- 亲读 `laiyipao/miniapp/src/pages/stage/stage.vue`（模板 :342-373、computed :411-437）：
  - 容器 `.r21-level-coach[role=region]` 新增于诊断卡旁，星级/`fetchMyStars`/`diagnose`/`d.stuck` 全部保留（ coexist 不替换）。
  - 逐行 `.r21-coach-row[data-r21-level]` 按 attempts 降序，展示尝试/胜/最好波次/平均波次（`r21Wave` 用 Number.isFinite 兜底，非有限数→'0.0'，不渲染 NaN/裸长浮点）；stuck 行显示「卡点」标注。
  - AI 四判据齐全：可见汉字「AI · 卡点教练」/ 真实数据结论（`coachConclusion` 由 top.attempts/wins/best_wave 确定性拼接）/ 逐字依据行「依据：近 30 天 battle_records 的 level_id/result/wave_reached 聚合；连续失利沿用 StuckThreshold=2」/ CTA「去补强 →」（页内切关不新造路由）。
  - 诚实红线逐字：「本卡为辅助诊断，不参与结算、score、星级、loot；权威结果以 replay_hash 为准。」
  - 空态逐字「近 30 天无关卡尝试记录」；无数据/请求失败均走空态，不显示 undefined/NaN；`coachConclusion` 空态为「近30天暂无足够记录生成教练建议。」不编造结论。
- `client.ts:1423` `fetchMyLevelCoach`（GET /me/level-coach?window_days=...，带 token 的 request）；`fetchLevelCoach` 别名兼容。

## 门禁④ 真浏览器决定性证据（SHA 各异 + 亲读）— PASS

- 四张截图本人**亲读**（1258×588，均为选关页/教练卡特写，非登录页）：
  - `r21-data.png`（选关页顶部，可见 AI 规则引擎卡与关卡网格）。
  - `r21-data-card.png`（个人突破档案特写：第40关卡点、尝试3胜0最好7波平均6.3波、AI建议、依据行、CTA、诚实行）。
  - `r21-empty.png`（用户11选关页）。
  - `r21-empty-card.png`（空态特写：逐字「近 30 天无关卡尝试记录」+ AI 空态「近30天暂无足够记录生成教练建议。」+ 诚实行）。
- `sha256sum` 独立复算四图两两不同，前 8 位与队长亲读一致：d36c4f16 / 65e67d6c / b142cc03 / 063c9b4d。
- DOM 探针 `dom-data.json` / `dom-empty.json` / `scroll-*.json` 亲读：有数据态 rowsCount=1（level=40）、空态 rowsCount=0、aiNote 四判据与诚实行文本齐全、hasUndefined=false、hasNaN=false；两态 pageErrors/consoleErrors/apiFailures/failedRequests 全 []（探针 pe/ce/af/fr 全 0）、loginPage=false。
- auth-init.tpl.js 亲读：token 用 `__LYP_TOKEN__` 占位符，运行时由 mint-token.sh 注入；探针只做 localStorage 注入与错误/响应收集，不篡改任何业务字段。

## 门禁⑤ 零真值 + R15–R20 不回退 — PASS

- **零真值**：含真 token 的 `auth-init.js` 已删除（glob 确认仅存模板）；真值守卫 `go test ./internal/domain/... -run 'TestNoRealSecrets|TestNoInfra' -count=1` → **ok，exit=0**；`git check-ignore -v` 确认 `.gitignore:307` data/ 与 `:308` auth-init.js 规则实际生效（qa-before.dump、http-coach-10.json、auth-init.js 均被忽略）；审计目录脚本仅含占位符与回环 127.0.0.1，无硬编码真 token。
- **令牌锚点不回退**：miniapp/admin tokens.css 均保留 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`；全库 CSS 无 `--r21-` 新增令牌。
- **端点不回退**：routes.go 保留 `/reaction-damage-impact`（R18/R19）、`/battle-efficiency`（R20）、`/stars`，新增 `/level-coach` 并列；admin 侧 reaction-damage-impact/battle-efficiency 端点保留；stage.vue 星级与单局诊断视图保留。

## 门禁⑥ 无新真实缺陷 + 三端测试全绿 — PASS

- 三端独立复跑全绿：后端纯逻辑 2 条 ok；miniapp 11/11；admin 8/8；真值守卫 ok。复查记录另载 miniapp 全量 1810 passed、admin 全量 763 passed（本轮未重复跑全量，契约+渲染与真值守卫已独立复跑）。
- 队长关注五点逐项结论：①跨用户隔离由两条带 user_id 的 SQL + 会话取身份 + 三用户 JSON 实证成立 ②stuck 逻辑与单测（关卡1 lose-win-lose-lose→不 stuck；关卡2 lose×3→stuck）双证成立 ③分母0归零单测 + 空 items JSON 成立 ④空态 coach 空对象 + 诚实文案，不编造成立 ⑤R15–R20 端点/令牌/基线不回退成立。
- 未发现任何阻塞性、高、中等级真实缺陷。

---

## 非阻塞观察项（不驱动返工）

1. **审计文档 SHA 抄录笔误（低）**：`docs/laiyipao/audits/round21-player-level-coach/REPORT.md` 第 37 行把 `r21-data-card.png` 完整 SHA256 抄为 `65e67d6c0c5e80973e2551e10180e50c1c0ab37532f4361bfd93e289ae57d2f25ab`，本人亲算实测为 `65e67d6c0c5e80973e255168153a7ad599d27bb89b5a3a2fe9ea687910eb9301`（前 8 位与队长亲读一致）。图片本身真实存在、内容正确、四图 SHA 各异且非登录页，判定不影响门禁④实质结论；属审计文档文本抄录瑕疵，建议后续轮次顺手更正，不作为本轮 needs_revision 依据。
2. 浏览器证据为 agent-browser 自动化、本地回环环境，非物理真机；已如实标注。

## 最终结论

R21「玩家卡点教练」六门禁全部 PASS，verdict = **pass**。SPEC 契约与三端实现一致，后端聚合在 user_id 隔离/连续失败阈值/分母归零/空态诚实上均成立，H5 AI 四判据与诚实红线到位，真浏览器证据 SHA 各异且亲读有效，零真值与 R15–R20 不回退确认，三端测试全绿。无阻塞缺陷，进入下一轮无需返工。
