# R20 设计终审报告 · 六门禁独立复核

- 任务：t169「R20终审：六门禁独立复核」（Attempt 2），attempt_id=2998c40a-534c-4876-bfb1-5e1a7024302f。
- 复核人：design-reviewer-wb（设计终审 / workbuddy 路由）。
- 判定：**verdict = pass**（六门禁全部通过，无需返工的结构性缺陷）。
- 复核方式：独立复核，未采信实施者/复查者自述；亲读 SPEC、后端源码、H5/admin 卡片源码、契约与渲染测试、隔离 JSON、真实截图并亲算 SHA；独立复跑三端测试与真值守卫。
- 浏览器证据口径：agent-browser 自动化浏览器（**非物理真机**），H5 视口 390×844、admin 视口 1258×588；已在复查报告中如实标注。

## 门禁① SPEC 契约一致 + 测试绿

- SPEC：`docs/laiyipao/specs/round20-combat-efficiency-trends/SPEC.md`（106 行）亲读——§1 后端字段契约（window_days=30 固定、window_from/to UTC、battles/shots/hits/accuracy_pct/avg_heat_max/leaked_per_battle/avg_duration_ms/buckets/ai_insight）、§2 展示选择器与逐字空态、§3 AI 四判据、§4 诚实红线、§5 test-first、§6 三端分工与不回退。
- 契约与渲染测试亲读：admin `tests/R20CombatEfficiency.contract.test.ts`、`tests/R20BattleEfficiency.render.test.ts`；miniapp `src/pages/battles/battles_r20_efficiency.contract.test.ts`、`src/pages/battles/battles_r20_efficiency.test.ts`；后端 `internal/service/battle_efficiency_test.go`、`internal/httpapi/routes_e2e_r20_battle_efficiency_test.go`。
- 独立复跑（本人执行，均 exit 0）：admin 2 文件 14/14 PASS；miniapp 2 文件 9/9 PASS；后端 `go test ./internal/service/... -run TestAggregateBattleEfficiency -count=1` PASS（3 条纯逻辑用例）；DB E2E 3 条因本机无管理库 DSN 按仓库约定 SKIP，已由 HTTP+SQL 实测补证。
- 判定：**PASS**。

## 门禁② 后端聚合诚实（隔离 / 归零 / 空态 / 窗口 / 不产 NaN）

- 源码亲读 `laiyipao/server/internal/service/battle_efficiency.go`：`AggregateBattleEfficiency` 纯逻辑聚合，battles/shots/hits/heat/leaked/duration 逐桶累加；`r20AccuracyPct`（shots=0→0）、`r20Average`（count=0→0）math.Round 保留一位；sum 为 0 时全部均值归零，无 NaN/Inf；buckets `make(...,0,...)` 空数组非 null；`window_from/to` 由 `now.UTC()` 确定性推导；AI 结论由 buckets 与真实字段确定性推导，battles=0 时 ai_insight 空对象。
- SQL 形状亲读（`queryBattleEfficiency`）：`WHERE created_at >= $1 AND created_at < $2`，仅当 `userID > 0` 追加 `AND user_id = $3`；me 端点由 `userIDFrom(c)` 取会话身份（handlers_r20.go:31），不接受前端传参；admin 全平台无过滤。`battleEfficiencyWindow` 按 UTC 午夜计算边界，窗口 30 天。
- 参数校验亲读：`battleEfficiencyWindowDays` 只接受 30（缺省 30、重复/非法 400）。
- 隔离 JSON 亲读：`data/http-efficiency-10.json`（7 场/1032 shots/633 hits/61.3%/两桶 72.8%·54.2%）、`http-efficiency-11.json`（全零空态、buckets=[]、ai={}）、`http-efficiency-1.json`（2 场但 shots=0→所有均值 0、无 NaN）。三用户互不串数据。
- 判定：**PASS**。

## 门禁③ H5 + admin AI 四判据 + 诚实文案 + 真浏览器决定性证据

- H5 源码亲读（`battles.vue`）：`.r20-efficiency-trend[role=region]` 容器（:122）、。r20-efficiency-row 汇总（:128）、。r20-trend-bar[data-r20-bucket] 趋势条（:137）、空态逐字「近 30 天无战斗记录」（:154）、加载失败空态（:159）；`r20Ai` computed（:911-923）由真实字段与 buckets 推导结论；AI 四判据齐备（可见汉字「AI · 战斗效率诊断」/真实结论/逐字依据行/CTA「去战斗→」）；诚实行逐字「辅助诊断非权威结算依据，权威请看 replay_hash」。
- admin 源码亲读（`DashboardView.vue`）：`section.r20-efficiency-trend[role=region]`（:492-529）汇总/趋势双视图、`effNum`（非有限数→「—」）、`effBuckets` 按 bucket 升序、空态逐字、AI 四判据齐备、诚实行逐字「效率趋势为辅助诊断；不参与结算、score、星级、loot，也不覆盖 replay_hash」；失败降级 try-catch 不阻塞权威看板。
- 截图 SHA 亲算（sha256sum，六张与复查报告逐字一致且两两不同）：r20-01=6bd832f9…、r20-02=829bac23…、r20-03=efa5ac86…、r20-04=36e9e213…、r20-05=d177d7b6…、r20-06=4741b0c1…。
- 截图亲读：r20-03（390×844）命中率 61.3%、两趋势桶、AI 四判据、诚实行、窗口，非登录页；r20-05（1258×588）admin 趋势视图 55.0%、两桶 73.1%/54.1%、AI 与诚实行，非登录页；r20-06（1258×588）用户 11 空态逐字「近 30 天无战斗记录」，非登录页。
- DOM 探针：复查报告记录 H5/admin 有数据态与空态 window.__R20_PROBE__/__R20_ADMIN_PROBE__ 的 pe/ce/af/fr 全 0（本终审以截图+源码+隔离 JSON 独立交叉验证，未重跑 E2E）。
- 判定：**PASS**。

## 门禁④ 零真值 + R1-R19 不回退

- 零真值：审计目录含真 token 的 `auth-init.js` / `admin-probe.js` 已删，仅留 `__LYP_TOKEN__` 占位模板与 127.0.0.1 回环；`git check-ignore -v` 确认 `.gitignore:200-202` 三条 R20 规则覆盖 `data/qa-before.dump`、`auth-init.js`、`admin-probe.js`；本人 `go test ./internal/domain/... -run 'TestNoRealSecrets|TestNoInfra' -count=1` exit 0；审计目录正则扫描无 JWT 真值/带口令 DSN/个人绝对路径命中。
- 令牌锚点亲读：admin `tokens.css:12-14` 与 miniapp `tokens.css:14-16` 均为 `lyp-tokens-v1` / `lyp-tokens-v3-r3` / `lyp-tokens-r6`，无 `--r20-` 新令牌；契约测试「三端不回退」分组断言 R18 /admin/reaction-damage-impact、R19 /me/reaction-damage-impact、R15-R17 时间线生态、25% AoE、顶栏脱敏均保留。
- 判定：**PASS**。

## 门禁⑤ 真实缺陷清零（含队长指定的单位展示差异裁定）

- 队长关注点：H5 场均时长显示「87278.6ms」，admin 显示「32.9s」——同一 `avg_duration_ms` 字段、两种展示单位。
- 独立核查：后端统一输出 `avg_duration_ms`（毫秒，见 battle_efficiency.go:62、SPEC §1 字段表）；H5 `r20Duration`（battles.vue:910）=毫秒直接 toFixed(1)+模板固定「ms」，admin（DashboardView.vue:508）=毫秒/1000+「s」。87278.6ms 与 32.9s 分属不同用户群与不同统计口径（玩家个人 vs 全平台），数值本身均正确。
- **裁定：不构成需修复的真实体验缺陷。** 理由：① SPEC 仅要求 toFixed(1) 确定性格式化，未规定展示单位，两端均符合契约；② 两端为不同界面、不同数据样本，非同一数字的两种错误表达；③ 单位在各自界面均有明确后缀（ms/s），不存在歧义或误读；④ 数据权威值与 replay_hash 未受影响。
- 建议项（非阻塞、不驱动返工）：若追求跨端观感统一，可在后续轮次由规范侧明确展示单位（例如统一为秒），并同步契约测试与文案；本轮不因此判 needs_revision。
- 其余：三图亲读无登录页、无编造数值、无 NaN/undefined；未发现其他真实缺陷。
- 判定：**PASS**。

## 门禁⑥ 三端测试全绿

- 本人独立复跑：admin 14/14、miniapp 9/9、后端 service 聚合 3 条、domain 真值守卫 exit 0（详见门禁①/④）。复查报告记录的全量 admin 755 tests / miniapp 1799 tests 亦为绿。
- 判定：**PASS**。

## 最终结论

R20 六门禁全部 PASS，verdict=pass。SPEC 契约、后端聚合诚实性（隔离/归零/空态/窗口）、H5 与 admin 双端 AI 四判据与诚实文案、真浏览器决定性证据（六图 SHA 各异并亲读非空）、零真值与 R1-R19 不回退、三端测试全绿均独立复核成立。队长指定的 H5/admin 时长单位差异经裁定为展示口径差异、非真实缺陷，记为非阻塞建议项。无需下游返工。

> 说明：本报告为独立设计终审，不替代物理真机验证；浏览器证据为 agent-browser 自动化（非物理真机），已如实标注。
