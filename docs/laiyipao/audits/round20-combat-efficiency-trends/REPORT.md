# R20 复查报告：三端战斗效率趋势闭环真浏览器实测

- 任务：t168「R20复查：三端战斗效率趋势闭环真浏览器实测」，attempt_id=77896c0d-427e-42e0-a4ab-43e4dc62081f。
- 判定：**PASS（六项验收全部通过，无缺陷）**。
- 方式：独立复查，未采信实施者自述；未改业务源码；未执行 git。
- 浏览器：agent-browser 自动化浏览器（**非物理真机**）；H5 视口 390×844，admin 视口 1440×900；每步均采 DOM 探针与截图并亲读。
- 规范口径：docs/laiyipao/specs/round20-combat-efficiency-trends/SPEC.md（106 行）。

## 一、验收项与证据（按顺序）

### ① H5 battles.vue 效率卡：有战报用户真实数据 + 无战报用户空态
- 有数据态（QA 用户 10，7 场战报）：真浏览器打开 http://127.0.0.1:5174/#/pages/battles/battles，DOM 探针 .r20-efficiency-trend[role=region] 存在；窗口「2026-10-25 ~ 2026-11-24（近 30 天）」；汇总行逐字「命中率 61.3% / 平均峰值热量 98.7 / 场均漏怪 0.0 / 场均时长 87278.6ms」；趋势条 2 行 data-r20-bucket=2026-11-22（72.8% · 3 场）、2026-11-23（54.2% · 4 场），按 bucket 升序。
- 嗅探 GET /api/v1/me/battle-efficiency?window_days=30 status=200，字段 battles=7/shots=1032/hits=633/accuracy_pct=61.3/avg_heat_max=98.7/leaked=0/avg_duration_ms=87278.6 与 SQL 独立核验逐字段一致。
- 空态（QA 用户 11，无战报）：逐字「近 30 天无战斗记录」，窗口如实保留，趋势条 0 行、AI 卡不渲染（ai=false），响应 battles=0/buckets=[]/ai={}，不编造效率值。
- 探针：两态 pageErrors/consoleErrors/apiFailures/failedRequests 全为 0；loginPage=false，title=我的战报。

### ② admin Dashboard 全平台效率卡真实数据
- 真浏览器登录 admin 打开 http://127.0.0.1:5175/laiyipao-admin/#/admin/dashboard，.r20-efficiency-trend[role=region] 存在。
- 汇总视图：窗口「统计窗口：近 30 天（2026-10-25 至 2026-11-24）」；汇总行「场次 103 / 命中率 55.0% / 平均峰值热量 27.4 / 场均漏怪 1.1 / 场均时长 32.9s」。
- 点击「趋势」后：趋势条 data-r20-bucket=2026-11-22（73.1%）、2026-11-23（54.1%），按 bucket 升序；汇总行隐藏，双视图切换生效。
- 嗅探 GET /api/v1/admin/battle-efficiency status=200，battles=103/shots=10922/hits=6009/accuracy=55.0/avg_heat=27.4/leaked=1.1/avg_duration=32857.3 与全平台 SQL 独立核验（103 场、10922 shots、6009 hits）逐字段一致。
- 探针 pe/ce/af/fr 全 0，loginPage=false，截图亲读为数据看板非登录页。

### ③ 后端 me/admin 端点逐字段与 SQL 一致（SUM=0 归零、空态、窗口）
- me 端点跨用户隔离 HTTP 实测（verify-isolation.sh，结果落 gitignore 保护的 data/http-efficiency-*.json）：
  - 用户 10：200，30 天窗口 2026-10-25 至 2026-11-24，7 场/1032 shots/633 hits，accuracy 61.3%；分桶 SQL 独立核验 2026-11-22(3 场/394/287)、2026-11-23(4 场/638/346) 与端点 buckets 逐字段一致。
  - 用户 11：200，battles=0、shots=0、hits=0、所有均值 0、buckets=[]、ai_insight={}，诚实空态，不产 NaN。
  - 用户 1：200，battles=2 但 shots=0/hits=0 → accuracy_pct=0、avg_* 全 0（SUM=0 分母归零），buckets=[2026-11-23: 2 场/0/0/0]，无 Infinity/NaN。
  - 非法参数 window_days=31 → HTTP 400「window_days 只接受 30」。
- 隔离性：用户 1/11 的响应不含用户 10 的任何场次/命中/热量；me 端点从会话 user_id 取用户（源码 laiyipao/server/internal/service/battle_efficiency.go queryBattleEfficiency 中 userID>0 追加 AND user_id=$3），不接受前端 user_id。
- admin 端点：全平台同口径聚合（userID=0 无过滤），浏览器实测响应与全平台 SQL 一致。
- 后端纯逻辑测试 TestAggregateBattleEfficiencyBucketsAndZeroDivision / TestAggregateBattleEfficiencyEmpty / TestAggregateBattleEfficiencyAIInsightFourCriteria 三条 PASS；3 条 DB E2E 因本机无管理库 DSN 诚实 SKIP，已由上述 HTTP+SQL 实测补证。

### ④ AI 四判据 + 诚实红线文案
- H5：.r20-ai-note 内可见「AI · 战斗效率诊断」；真实结论「近 30 天命中率 61.3%，场均漏怪 0.0，平均峰值热量 98.7；命中率呈下降趋势，建议优先提升命中与清漏效率。」；逐字依据行「依据：数据来自全量battle_records近30天；近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准」；可执行 CTA「去战斗→」（点击跳关卡页）；诚实行逐字「辅助诊断非权威结算依据，权威请看 replay_hash」。
- admin：可见「AI · 战斗效率诊断」；真实结论「近 30 天命中率 55.0%，峰值热量呈下降趋势，建议关注 heat 管理。」；逐字依据行「依据：近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准」；CTA「查看战报 →」；诚实行逐字「效率趋势为辅助诊断；不参与结算、score、星级、loot，也不覆盖 replay_hash」。
- 红线：效率为辅助诊断，不参与结算/score/星级/loot，不覆盖 replay_hash；空态与 SUM=0 均如实归零。

### ⑤ 探针 pe/ce/af/fr 全 0
- H5 有数据态与空态、admin 汇总与趋势视图：window.__R20_PROBE__ / __R20_ADMIN_PROBE__ 的 pageErrors、consoleErrors、apiFailures、failedRequests 全部为 0；无 4xx/5xx 失败请求（window_days=31 的 400 为主动参数校验测试，不计入正常流程）。

### ⑥ 截图 sha 各异 + 亲读非登录页 + 非物理真机标注
- 目录 docs/laiyipao/audits/round20-combat-efficiency-trends/screenshots/，6 张 PNG 均亲读确认为战报页/数据看板（非登录页），sha256 两两不同：
  - r20-01-h5-efficiency.png = 6bd832f9b5be2df6aa21e3a8674d219ea9f086ed3a711f930f1c1ce587b90c64（390×844，H5 战报页顶部 KPI）
  - r20-02-h5-scroll.png = 829bac239e97b60e2063d6a160edcaa44d4be579a421f6bca19609f7ed338546（390×844，H5 战报页滚动态）
  - r20-03-h5-card.png = efa5ac86e85bd849e2c4a3e7f092e79ba3c4a827efbb7643b8d92ea756e2496b（390×844，H5 效率趋势卡完整可见：命中率 61.3%、两条趋势、AI 卡、诚实行）
  - r20-04-admin-summary.png = 36e9e213a9dce6376af6a94628df07f7625262f0799ad24e15374fddd91418cf（1258×588，admin 汇总视图：场次 103/命中率 55.0%/AI 卡）
  - r20-05-admin-trend.png = d177d7b6679d7cb249b2f7149cc932927dd89dfa0181f607f113778a0d9b1bf6（1258×588，admin 趋势视图：两条按日桶 73.1%/54.1%）
  - r20-06-h5-empty.png = 4741b0c1570276e0bb76e6c864dc0d08fd864f694923f29df7e35b151b48e838（1258×588，用户 11 空态「近 30 天无战斗记录」）
- 浏览器为 agent-browser 自动化浏览器，非物理真机，已在判定中如实标注。

## 二、测试与 RED 基线
- 契约/渲染：admin R20CombatEfficiency.contract.test.ts + R20BattleEfficiency.render.test.ts = 14/14 PASS；miniapp battles_r20_efficiency.contract.test.ts + battles_r20_efficiency.test.ts = 9/9 PASS。
- 全量：admin 67 files/755 tests PASS；miniapp 196 files passed（1 skipped）/1799 tests passed（2 skipped）。
- RED 基线：git show HEAD 的 DashboardView.vue 无 r20-（grep -c=0）、battles.vue 无 r20-efficiency（=0）、battle_efficiency.go 与 R20BattleEfficiency.render.test.ts 均 NOT_IN_HEAD，证实为新增能力而非既有代码自证。

## 三、真值与清理
- 审计目录 ops_grep 精确模式（JWT / Bearer / guest_token 真值 / 个人绝对路径 / 带口令连接串）扫描：0 命中。
- 含真 token 的审计文件 auth-init.js、admin-probe.js 于收尾删除；.gitignore 已追加 docs/laiyipao/audits/round20-combat-efficiency-trends/{data/,auth-init.js,admin-probe.js} 三条忽略。
- QA 数据还原：docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < data/qa-before.dump（快照 sha256=5a0591c0a53f54633efe7b8ede4cfb276e0146412d6863bbce49c8bde1f0c7f8），exit=0。

## 四、命令清单（复现）
- bash docs/laiyipao/audits/round20-combat-efficiency-trends/verify-isolation.sh → 三用户 HTTP 隔离 + SQL 聚合核验 + window_days=31 的 400。
- bash docs/laiyipao/audits/round20-combat-efficiency-trends/e2e-h5.sh（agent-browser，session lyp-r20-e2e）→ H5 有数据态 DOM 探针与截图。
- bash docs/laiyipao/audits/round20-combat-efficiency-trends/e2e-admin.sh（agent-browser，session r20-admin）→ admin 汇总/趋势双视图探针与截图。
- cd laiyipao/admin && npx vitest run --config vitest.config.ts → 755 tests PASS。
- cd laiyipao/miniapp && npx vitest run --config vitest.config.ts → 1799 tests PASS（2 skipped）。
- cd laiyipao/server && go test ./internal/service/... -run TestAggregateBattleEfficiency → 3 条 PASS。

## 五、结论
R20 六项验收全部 PASS，无缺陷。me/admin 端点与 H5/admin 双端效率卡均按 SPEC 落地：跨用户隔离、30 天窗口、SUM=0 归零、诚实空态、AI 四判据与诚实红线全部真浏览器与 HTTP/SQL 双重核验成立；未修改业务源码，未执行 git。
