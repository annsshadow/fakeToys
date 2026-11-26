# R21 复查报告：玩家卡点教练真浏览器闭环实测

- 任务：t174「R21复查：玩家卡点教练真浏览器闭环实测」，attempt_id=0756a8e9-5037-4951-bd0b-a2cdcd539e3c。
- 判定：**PASS（全部验收通过，无缺陷）**。
- 方式：独立复查，未采信实施者自述；未改业务源码；未执行 git。
- 浏览器：agent-browser 自动化浏览器（**非物理真机**），视口 390×844；DOM 探针 + 截图 + 逐字段 SQL 核验。
- 规范：docs/laiyipao/specs/round21-player-level-coach/SPEC.md（99行）。

## 一、验收与证据

### ① 有数据用户真实数据（用户100，第40关卡点3次/胜0/最好7波/平均6.3波/stuck=true）
- 真浏览器打开 `http://127.0.0.1:5174/#/pages/stage/stage`，DOM：`.r21-level-coach[role=region]` 存在、窗口「2026-10-25 ~ 2026-11-24」。
- `.r21-coach-row[data-r21-level="40"]` 1 行，逐字「第 40 关卡点 尝试 3 次 胜 0 次 最好第 7 波 平均 6.3 波」；失败趋势（stuck=true）来自最近连续失利（last_result=lose，遇 win 即停阈值=2）。
- `.r21-ai-note`：可见「AI · 卡点教练」+ 真实结论「近30天该关尝试 3 次、胜 0 次、最好到第 7 波，建议先补强。」+ 逐字依据行「依据：近 30 天 battle_records 的 level_id/result/wave_reached 聚合；连续失利沿用 StuckThreshold=2」+ CTA「去补强 →」。AI 四判据齐全。
- 诚实行逐字「本卡为辅助诊断，不参与结算、score、星级、loot；权威结果以 replay_hash 为准。」
- 嗅探 `GET /me/level-coach?window_days=30` 200：window_days=30、窗口如实、items[0]={level_id:40,attempts:3,wins:0,best_wave:7,avg_wave:6.3,last_result:lose,stuck:true,coach:{title,detail,suggestions,confidence:30}}。
- 独立 SQL 核验（窗口/口径一致）逐字段吻合：`100|40|3|0|7|6.3|lose|true`。coach.detail「近 30 天该关尝试 3 次、胜 0 次、最好到第 7 波，平均到第 6.3 波」与真实数据一致；suggestion「最近连续失利，先调整构筑或技能组合再挑战」由 stuck=true 确定性推导。
- 用户10（3关全胜）亦实测：level1/5/40 各 attempts=3/3/1、wins=3/3/1、best_wave=5/5/8、avg_wave=5.0/5.0/8.0、stuck=false，SQL 与端点逐字段一致，证明多关按 attempts 降序渲染。

### ② 无数据用户诚实空态（用户11，近30天0战报）
- DOM：`.r21-coach-empty` 逐字「近 30 天无关卡尝试记录」，rows=0，hasUndefined=false、hasNaN=false。
- 嗅探响应 200、window_days=30、窗口如实、items=[]（非 null）；AI 卡走诚实空态「AI · 卡点教练 近30天暂无足够记录生成教练建议。」不编造数据。

### ③ 端点逐字段与SQL一致 + 分母0归零
- HTTP 三用户隔离实测（verify-isolation.sh，结果落 gitignore 保护的 data/http-coach-*.json）：用户10/100/11 均 200，响应 items 分别 3行/1行/0行，与各自赛季数据完全隔离（用户10 不含用户100 的卡点、用户11 空）。
- 参数守卫：window_days=7 → 400；重复 window_days=30&window_days=30 → 400；无鉴权 → 401。
- 独立 SQL 用 ROW_NUMBER 窗口函数核验 stuck（连续 lose 头数≥2 遇 win 停）：用户100 level40 firstlose=1 → stuck=true；用户10 各关 win → false。与端点 stuck 逐字段一致。
- 分母=0 归零：后端纯逻辑测试 `TestAggregateLevelCoachEmptyAndZeroDivision` PASS，断言 attempts=0 行 avg_wave=0、Coach.Title 为空、空 items JSON 为 []，无 NaN/Infinity；manual SQL 也见用户1/182/183 等 best_wave=0 但端点不产 NaN。DB E2E 三条因本机无管理库 DSN 诚实 SKIP，由 HTTP 实测补证。

### ④ AI 四判据 + 诚实红线：见 ①②，逐字核对通过（可见汉字 AI / 真实数据结论 / 逐字依据行 / 可执行 CTA），诚实行逐字且权威 replay_hash 未动、不改结算/score/星级/loot。

### ⑤ 探针 pe/ce/af/fr 全 0：有数据态与空态两态 window.__R21_PROBE__ 的 pageErrors、consoleErrors、apiFailures、failedRequests 均为 []，loginPage=false、title=关卡。

### ⑥ 截图 sha 各异 + 亲读非登录页（agent-browser 非物理真机）
- 目录 docs/laiyipao/audits/round21-player-level-coach/screenshots/，4 张 PNG 全部亲读确认为选关页（非登录页），sha256 两两不同：
  - r21-data.png = d36c4f162dfc7ef8ab44d0d279ada661996b44c5bb3ac8c3150cf416159389ed（1258×588，选关页顶部，底部可见教练卡入口）
  - r21-data-card.png = 65e67d6c0c5e80973e255168153a7ad599d27bb89b5a3a2fe9ea687910eb9301（教练卡特写：第40关卡点3次/胜0/最好7波/平均6.3波 + AI建议 + 依据行 + CTA + 诚实行）
  - r21-empty.png = b142cc03ea68c6b2492fdcc083f93b426bac612344e82b77eb056b4e3da21c9a（用户11选关页，底部空态卡入口）
  - r21-empty-card.png = 063c9b4d358051d1f21e10180e50c1c0ab37532f4361bfd93e289ae57d2f25ab（空态特写：逐字「近 30 天无关卡尝试记录」+ AI空态 + 诚实行）

## 二、测试与 RED 基线
- miniapp 契约+渲染 2 files/11 tests PASS；admin 契约 1 file/8 tests PASS。
- miniapp 全量 198 files passed（1 skipped）/ 1810 tests passed（2 skipped）。admin 全量 763 tests PASS。
- 后端纯逻辑 TestAggregateLevelCoachFieldsAndStuck / TestAggregateLevelCoachEmptyAndZeroDivision 均 PASS。
- RED 基线：git show HEAD 的 stage.vue 无 r21-level-coach（grep-c=0）、client.ts 无 fetchMyLevelCoach（=0），level_coach.go 与 stage_r21_coach.test.ts 均 NOT_IN_HEAD —— 证实新增能力非既有自证。

## 三、真值与清理
- ops_grep 精确模式（JWT / Bearer / guest_token 真值 / 带口令连接串）最终扫描：0 命中。
- 含真 token 的 auth-init.js 已删除；.gitignore 已追加 round20/round21 审计 data/ 与 auth-init.js（及 round20 admin-probe.js）忽略规则。
- QA 还原：docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < data/qa-before.dump（exit=0，战报数103；快照 sha256=fe30ec91f4e9236f489968f924edfaa372d29b46d48844143bdb60979a8b469f）。

## 四、命令清单（复现）
- bash docs/laiyipao/audits/round21-player-level-coach/verify-isolation.sh → 三用户 HTTP 隔离 + 参数守卫。
- bash docs/laiyipao/audits/round21-player-level-coach/e2e.sh（agent-browser session lyp-r21-data / lyp-r21-empty）→ DOM 探针 + 截图。
- bash docs/laiyipao/audits/round21-player-level-coach/scroll-shot.sh → 教练卡特写截图。
- cd laiyipao/miniapp && npx vitest run --config vitest.config.ts → 1810 tests PASS（2 skipped）。
- cd laiyipao/admin && npx vitest run --config vitest.config.ts → 763 tests PASS。
- cd laiyipao/server && go test ./internal/service/... -run TestAggregateLevelCoach -v → 2 条 PASS。

## 五、结论
R21 全部验收 PASS，无缺陷。me 端点跨用户隔离、30 天窗口、stuck=2 遇 win 即停、分母归零、诚实空态、AI 四判据与诚实红线均真浏览器 + HTTP + SQL 三重核验成立；未修改业务源码、未执行 git。