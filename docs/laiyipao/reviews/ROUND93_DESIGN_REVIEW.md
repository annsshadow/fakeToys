# R93 独立终审报告：RecomputesView 骨架屏一致性

- 被审任务：t210（实施 GREEN）；规范 t209 / QA t211。
- 终审任务：t212（attempt_id=c2f05a22-c25d-4113-909a-6db046c70d6e），执行 same-route-reviewer。
- 基线 HEAD：43a252d90ee529fc31a02d02f9444278f8feada4（终审全程未变）。
- 判定：**verdict=pass**，六维全绿，无 blocker / high / medium finding。
- 取证方式：逐维独立读源码 / 契约 / 规范 / QA 报告 / 截图，并独立复跑关键门禁；不沿用 t209/t210/t211 的自报结论作为唯一依据。

## 维度 1 · test-first 真实性（三重采信，通过）

- 规范文件 docs/laiyipao/specs/round93-recompute-skeleton/SPEC.md §5 明确记载 RED：命令 `cd laiyipao/admin && pnpm exec vitest run tests/R93RecomputeSkeleton.contract.test.ts --reporter=verbose`，exit RC=1，Test Files 1 failed (1)，Tests **3 failed | 3 passed (6)**。
- 失败三项即缺口断言：it1「导入共享 PageSkeleton 组件」、it2「根节点 .page 之后插入 PageSkeleton」、it3「表格移除 v-loading=loading」——均为实施前源码确无对应结构的负向断言；通过三项 it4/5/6 为不变量（竞态守卫 / KPI 与档位不变量 / 令牌三锚点与写域）。
- 我独立逐字读契约源码 laiyipao/admin/tests/R93RecomputeSkeleton.contract.test.ts：6 个 it 结构与 SPEC §4 逐条对应；契约以 import.meta.url + dirname/join + fs.readFileSync 真实读源码（非内嵌副本），it3 同时断言 `view` 不含 `v-loading="loading"`、保留 `<div v-loading="statsLoading" class="stat-row">`，并对 `class="data-grid"` + `:data="rows"` 表格锚点单独断言无 v-loading。契约卫真实性成立。
- 三重采信：SPEC §5 文字结果 + 契约源码结构 + 队长独立复跑；RED 原始日志未留存（SPEC §5 未附日志文件）已如实标注，不夸大为「日志实存」。
- 结论：test-first 真实，RED 形状符合「缺口 fail / 不变量 pass」，通过。

## 维度 2 · 实施逐字复核（通过）

- git diff 显示业务源码仅 laiyipao/admin/src/views/RecomputesView.vue **+3/-1**（stat 显示 2 files changed / 8 insertions / 1 deletion，其中 .gitignore +5 为审计目录白名单，见维度 3）：
  1. `import PageSkeleton from "@/components/PageSkeleton.vue"`（导入区，:18）——与 BattlesView.vue:2 同构。
  2. 根节点 `<div class="page">` 之后首行插入 `<PageSkeleton v-if="loading" />`（:360）——与 BattlesView.vue:423 同构。
  3. 表格 `<el-table class="data-grid" :data="rows" ...>` 移除 `v-loading="loading"`（:616），`stat-row` 的 `v-loading="statsLoading"` 保留。
- 权威口径逐项对照：
  - SPEC §2 三改点与 diff 逐字吻合，无第四处改动。
  - 组件 laiyipao/admin/src/components/PageSkeleton.vue：`role="status" aria-busy="true" aria-label="数据加载中"` + sk-head + sk-kpis(4 卡) + sk-table(表头 + 6 行)，skeleton-bar 宽 70%/45%/25%——实施直接复用，未自造变体。
  - main.css:535 `.page > .page-skeleton ~ * { display:none!important }` 独占规则使骨架出现时隐藏同页真内容，避免 keep-alive 旧数据叠加（QA 探针 skVisibleKpi=true 坐实）。
  - AI_VISUAL_SYSTEM.md §6（docs/laiyipao/design/AI_VISUAL_SYSTEM.md）：admin 表格页显示表头 + 5–8 行占位、KPI 数字块占位，v-loading 仅用于「按钮提交中」等局部短时场景——实施把首屏加载交给骨架、只保留 stat-row 局部 v-loading，完全吻合规范定位。
- 结论：实施逐字符合 SPEC §2/§3 与权威范式，外科改动无溢出，通过。

## 维度 3 · 不变量与外科写域（通过）

- 竞态守卫与刷新语义（QA data/static-out.txt 行号坐实）：`statsLoadSeq`（:174/:176/:180/:183/:189）与 `loadSeq`（:244/:246/:256/:260/:263）双守卫在；`Promise.all([load(), loadStats(), loadMode()])`（:311）在；未改任何 computed / API / data 业务语义。
- 不变量齐在：KPI 四格 `kpi-strip`（:459）与 `summary?.totals.total/mismatched`「—」占位、`result-count` 带 `aria-live="polite"`（:487）、`switchMode`（:220/:400）、`emptyText='没有匹配的重算记录'`（:41）、`el-pagination` + `cappedTotal(total, LEDGER_PAGE_SIZE)`（:469-473）、`AiInsightCard :loading="statsLoading"`（:452）全保留。
- 令牌三锚点（tokens.css:12/13/14）：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 逐字在；未碰 tokens.css / common.css / main.css。laiyipao/admin/src 全量 grep `--r93-` 零命中（NO_MATCH）。
- 写域白名单：`git status --porcelain` 非 ?? 项仅 `.gitignore` 与 `laiyipao/admin/src/views/RecomputesView.vue`。.gitignore 改动为队长预置的 R93 审计目录 / admin-auth-init.js 白名单（+5 行），属真值不入库守卫，非业务改动。
- 结论：不变量零回退，写域精确，通过。

## 维度 4 · QA 真浏览器可信（通过）

- QA 报告 docs/laiyipao/audits/round93-recompute-skeleton/REPORT.md 记录三视口 390×844 / 768×1024 / 1280×800 的加载态 / 到达态 / stat-row 局部刷新 / 范式对照四态；三视口实测逐项一致（骨架 1 / role=status / aria-busy=true / 4 卡 / 35 bar / 7 行；gridMaskCount=0；加载态 KPI 全「—」、tableRows=0；到达态 KPI=[42,7,16.7%,enforce]、tableRows=4、resultCount「窗口 24h · 共 42 条」；hOverflow=false；isLogin=false）。刷新 mid 态仅 statMask=1，证实局部 v-loading 不触发首屏骨架重现。
- 9 张正式图 sha256 我独立用 sha256sum 复核，与 REPORT §3 逐字一致，UNIQUE 9/9：
  - r93-390-load c13b3bf7… / r93-390-arrive 8dccef20… / r93-390-pattern be1ac823…
  - r93-768-load 3ec35ce0… / r93-768-arrive c57792de… / r93-768-pattern ea7f9cd0…
  - r93-1280-load b3b608d3… / r93-1280-arrive 919d4a7f… / r93-1280-pattern 7fa260fd…
- 亲读 2 张（≥2 达标）独立确认非登录页：
  - r93-768-load.png（768×1024/48006B，sha 3ec35ce019106d9cef8245e756a3a9acb85855428c0f1ce4cad69f8fe0a5d5ae）：标题「结算重算审计」顶栏 + 左侧竖排管理导航（重算审计图标高亮）+ Ctrl K / 登出；主体为两条 sk-head 骨架、4 张 KPI 骨架卡（每卡三段 skeleton-bar）、表头 + 6 行表格骨架，无登录表单，属加载态。
  - r93-1280-arrive.png（1280×800/99905B，sha 919d4a7f0f96f2f6118acccaac3f21d4fb451d1494f280625e408117da7104de）：顶栏「结算重算审计」+ 回环 API 角标 + 登出；含重算档位条（生效中 enforce / 正在拒发不上报 + 切换 enforce/shadow/off）、AI 运营洞察卡（第 3 关不符率最高 16.7%）、四格 KPI 真值 42/7/16.7%/enforce、stat-row 统计条（生效档位/重算总数/不符数/不符率/已拒发）+ 结果计数「窗口 24h · 共 42 条」，属到达态，非登录页。
- flaky 如实披露：QA §5 记 G3 首轮 1 failed/908 passed，唯一失败 WikiEnemyMechanics > 「审计脚本自身纳入扫描且实跑 BAD_COUNT=0」，错误 `Error: Test timed out in 20000ms.`（:182，execFileSync bash 实跑 22914ms > 20s），与骨架写域无关；QA 单测定向重跑 14/14、全量重跑 909/909。我独立复跑全量 vitest（见维度 5）亦 909/909 且首轮 flaky 未复现，flaky 判定成立。
- 诚实边界：后端 8080 未启动、数据由 init-script mock 合成（42/7/16.7%/enforce、4 行），坐实的是前端骨架与数据切换而非真实后端返回；令牌为占位串仅注入内存。
- 结论：QA 证据可信且自洽，通过。

## 维度 5 · 门禁独立复跑（通过）

我在 worktree 写独立脚本 docs/laiyipao/audits/round93-recompute-skeleton/review-gates.sh 复跑关键门禁（UNTRACKED，结果 review-gates-out.txt / review-g3-out.txt）：

| 门禁 | 命令 | 复跑结果 |
| --- | --- | --- |
| RV-G1 R93 契约 | pnpm exec vitest run tests/R93RecomputeSkeleton.contract.test.ts | 6/6 passed，RC0 |
| RV-G2 骨架范式契约 | pnpm exec vitest run tests/PageSkeleton.contract.test.ts | 9/9 passed，RC0 |
| RV-G3 admin 全量 vitest | pnpm exec vitest run --reporter=dot | 82 files passed / 909 tests passed，RC0（首轮 flaky 未复现） |
| RV-G4 vue-tsc | pnpm run type-check | RC0 |
| RV-G5 build:only | pnpm run build:only | RC0 |
| G6 git diff --check | git diff --check | RC0 |
| type-check:test | pnpm exec tsc -p tsconfig.vitest.json | 队长裁定豁免：既存写域外 .ts 类型债（TS2739/TS2345/TS6133/TS2322）与本轮无关，如实标注为豁免而非 PASS |

- 结论：关键门禁全部独立复跑通过，type-check:test 豁免如实标注，通过。

## 维度 6 · 保护面零回退与诚实边界（通过）

- 骨架范式不回退：admin/src/views 全量 grep `<PageSkeleton v-if="loading" />` 命中 10 视图（Battles :423 / Content :815 / Dashboard :329 / Defenses :284 / Economy :421 / Equipment :101 / Levels :361 / Recomputes :360 / Skills :210 / Users :527），9 既有视图未被改动；`class="page" v-loading="loading"` 整页遮罩零命中。
- 排除项未触碰：WikiView（异构详情/词典型）、LoginView（按钮 loading）均不在写域（git diff 仅 2 文件），未被波及。
- 后端未起用 mock、QA 库只读（本轮终审未写库）、零真值：auth 四路径（.env/.local.keys/remote-config.env/*.pem/*.key 及 admin-auth-init.js）在 round93 审计目录 glob 零命中 ABSENT；终审未生成或回显任何密钥 / token / 私有地址真值。
- hygiene 全绿：HEAD 未变 43a252d9；非 ?? 项仅白名单 2 文件；git diff --check RC0；git status --porcelain 无其他改动。
- 结论：保护面零回退，诚实边界充分，通过。

## 观察项（low，非阻断，不影响 verdict）

1. RED 原始日志未单独留存为文件，RED 结果仅见 SPEC §5 记载与队长独立复跑；三重采信成立，但建议后续规范轮附原始日志碎片以便再复核。
2. gates-out.txt 首轮 G3_RC=1（既存 flaky 超时），QA 与我的独立复跑均未复现；属工作树大量 untracked 审计产物拖慢零真值扫描的超时波动，非本轮回归。

## 结论

六维逐维独立取证全绿，无 blocker / high / medium finding，关键门禁独立复跑通过，2 项 low 观察项非阻断。**verdict=pass**，建议队长据 t210 实施进入分批发布流程。
