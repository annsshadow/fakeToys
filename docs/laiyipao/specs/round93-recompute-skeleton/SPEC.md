# R93 规范：admin RecomputesView 表格页骨架屏一致性（RED）

- 轮次：R93 规范设计（承接 R92-A 侦察待队长裁定项 §6.1）
- 执行：same-route-ui（R41 规范设计）
- 基线 HEAD：43a252d90ee529fc31a02d02f9444278f8feada4
- 性质：纯规范 + 契约（test-first RED）；不改业务源码（RecomputesView.vue 留待实施轮）；零真值、不写 git、不碰令牌三锚点。

## §0 背景与队长裁定

R92-A 侦察（docs/laiyipao/recon/ROUND92_A_ADMIN_LOADING_RECON.md）静态 + 真浏览器三视口坐实：admin 12 视图中 9 个同构表格页均已用共享 PageSkeleton.vue（role=status/aria-busy=true，4 卡 + 6 行占位，宽 70/45/25），仅 RecomputesView.vue 作为表格型审计页用局部 v-loading 灰遮罩 + KPI 的「—」占位。侦察四条件裁定③「改动面小零风险」不成立，列为待队长裁定项。队长裁定立建设性规范轮：以最小改动对齐权威骨架范式，提升多视图加载态一致性。

RecomputesView 根节点 :358 `<div class="page">` 与 9 个骨架视图同构，main.css:535-537 `.page > .page-skeleton ~ * { display: none !important; }` 独占规则可直接生效，无需改造 DOM 层级。

## §1 权威口径（勿自造）

- AI_VISUAL_SYSTEM.md:848：admin 表格页首屏加载应由通用骨架组件显示「表头 + 5–8 行占位条」；`v-loading` 仅用于「按钮提交中」等局部短时场景。
- PageSkeleton.vue（61 行）：`<div class="page-skeleton" role="status" aria-busy="true" aria-label="数据加载中">`；sk-head + sk-kpis(4 卡) + sk-table(表头 + 6 行)；skeleton-bar 宽 70%/45%/25%；shimmer 用 main.css:504-514 `.skeleton-bar`。
- 同仓范式：BattlesView.vue:2 `import PageSkeleton from "@/components/PageSkeleton.vue"`、:423 `<PageSkeleton v-if="loading" />`。
- 四态完整：Recomputes 加载 v-loading + 「—」→ 成功真实 KPI/表格 → 空态 emptyText「没有匹配的重算记录」(:584/:692) → 错误三路 ElMessage（总账失败 warning 但列表照常 :181-186、列表失败 error :258-260、档位失败 warning :212-215）。本轮只改加载态形态，不动四态语义。

## §2 改点（实施轮执行，本规范轮不改业务源码）

精确改点仅三项（均在同一文件 laiyipao/admin/src/views/RecomputesView.vue）：

1. script import 区新增 `import PageSkeleton from "@/components/PageSkeleton.vue"`（与 BattlesView.vue:2 同构）。
2. 模板根节点 :358 `<div class="page">` 之后第一行插入 `<PageSkeleton v-if="loading" />`（与 BattlesView.vue:423 同构）。main.css:535 独占规则确保骨架出现时隐藏同页其余内容，避免 keep-alive 旧数据叠加。
3. 表格 :616 `<el-table class="data-grid" v-loading="loading" :data="rows" size="small" stripe>` 移除 `v-loading="loading"`（首屏加载由骨架承担）；保留 :489 `<div v-loading="statsLoading" class="stat-row">` 作为「刷新总账」局部短时场景（符合 §6 对 v-loading 的定位）。

## §3 不变量（契约逐字守卫，严禁改动）

- 两套竞态守卫不动：`let loadSeq = 0` / `let statsLoadSeq = 0` 及 `if (seq !== loadSeq) return` / `if (seq !== statsLoadSeq) return`。
- `function reloadAll() { ... return Promise.all([load(), loadStats(), loadMode()])` 不动；`onMounted(reloadAll)` 与 `onActivated` 刷新语义不动。
- KPI 区 :452-460 `kpi-strip` 的 `summary?.totals.total`/`summary?.totals.mismatched`「—」占位、`result-count` aria-live=polite、`switchMode` 档位确认、`el-pagination`/`cappedTotal(total, LEDGER_PAGE_SIZE)`、`emptyText`「没有匹配的重算记录」、`AiInsightCard :loading="statsLoading"` 均不动。
- 令牌三锚点不动（tokens.css:14-16）：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`。
- 零令牌改动（不碰 tokens.css / common.css / main.css）；不改任何 computed / API / data / 业务语义；业务源码零 `--r93-`。

## §4 契约清单

契约文件：laiyipao/admin/tests/R93RecomputeSkeleton.contract.test.ts；参照 PageSkeleton.contract.test.ts / R4FourViews.contract.test.ts 范式——`import.meta.url` + `dirname/join` + `fs.readFileSync` 真实读源码（禁内嵌源码副本）。共 6 个 it：

- it1 导入：RecomputesView.vue 须含 `import PageSkeleton from "@/components/PageSkeleton.vue"`（允许单/双引号）。
- it2 骨架插入：根节点 `<div class="page">` 之后须出现 `<PageSkeleton v-if="loading" />`（骨架在根节点首行）。
- it3 表格 `v-loading="loading"` 已移除且 stat-row 的 `v-loading="statsLoading"` 保留。
- it4 竞态守卫与 `Promise.all([load(), loadStats(), loadMode()])` 不回退。
- it5 KPI「—」占位 / `result-count` aria-live / `modeState` / `emptyText` / `AiInsightCard` 五类不变量保留。
- it6 令牌三锚点保留 + 业务源码零 `--r93-` + 写域边界（仅 RecomputesView.vue / PageSkeleton.vue 允许被改；契约自身与 SPEC 由本规范轮新建）。

## §5 RED 结果

命令：`cd laiyipao/admin && pnpm exec vitest run tests/R93RecomputeSkeleton.contract.test.ts --reporter=verbose`

结果：exit RC=1（符合 RED 预期），Test Files 1 failed (1)，Tests 3 failed | 3 passed (6)：

- 失败（缺口断言，符合预期）：it1「导入共享 PageSkeleton 组件」（源码无 import）、it2「根节点 .page 之后插入 PageSkeleton」、it3「表格移除 v-loading=loading」（:616 仍有 v-loading）。
- 通过（不变量，符合预期）：it4 竞态守卫与 Promise.all 刷新语义保留、it5 KPI/result-count/modeState/emptyText/AiInsightCard 五类不变量保留、it6 令牌三锚点与写域边界保留。
- 另：契约自身 `pnpm exec tsc --noEmit ...` RC=0，无 TS6133/TS2345 卫生问题（吸取 t177 教训）。

## §6 诚实边界

- 本轮为纯规范 + 契约（test-first RED），未改任何业务源码；精确改点由实施轮承接并跑 GREEN。
- 真浏览器三视口加载态复验（骨架可见、mask 消除、无横溢）由后续 QA 轮承接，本轮未自行执行。
- 交付物 SPEC.md 与契约均为 UNTRACKED 新文件；零真值、不写 git、不碰令牌三锚点；RecomputesView.vue 等权威源码保持原样。
