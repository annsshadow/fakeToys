# R88 规范：battles 战绩趋势图图例修正（RED）

- 轮次：R88 规范设计（承接 R88-B 侦察候选 R88-B-1）
- 执行：same-route-lead（R41 规范设计）
- 基线 HEAD：6a18e7e9b89f7bfa9327239534f125dc47eebb93
- 性质：纯规范 + 契约（test-first RED）；不改业务源码（battles.vue 留待实施轮）；零真值、不写 git、不碰令牌三锚点。

## §0 背景与决策

R88-B 侦察（docs/laiyipao/recon/ROUND88_B_DATA_CHART_READABILITY_RECON.md）列出 1 个四条件全中候选 R88-B-1：battles.vue G70 战绩趋势图图例（battles.vue:74-77，文案在 :75）不完整且符号误导。队长读源码坐实并裁定立规范轮。

与需产品评定的分叉不同，这是纯展示层图例文字与图形编码的客观错位：模板 :58 注释自称「分数走势 + 胜负色点，趋势一眼可见」，图例 :75 却只讲两条折线、用实心圆点符号 ● 标注折线；图上真正的圆点是胜负点，绿胜红负色点完全没有解释。

旧图例逐字：battles.vue:75 「● 分数（蓝）　● 漏怪率（红）　越低越好」。三重问题：

1. 无绿胜红负色点解释；
2. 实心圆点符号 ● 用来标注折线（折线非圆点，与真正圆点错位）；
3. 红点（负）与红虚线（漏怪率）同为 var(--bad) 异义，「越低越好」字面误导分数（分数实际越高越好）。

## §1 权威口径（图上实际三种视觉编码，队长读源码坐实，勿自造）

- 蓝实线 polyline.trend-line = 分数（CSS var(--ice)，越高越好）：battles.vue:65 / CSS :1045-1050。
- 红虚线 polyline.leak-line = 漏怪率（CSS var(--bad)，stroke-dasharray 4 3，越低越好）：battles.vue:66 / CSS :1051。
- 圆点 circle = 每局胜负：trend-win 绿 var(--ok)=胜 / trend-lose 红 var(--bad)=负：battles.vue:67-72（win：it.result==='win' 见 :411）/ CSS :1053-1054。

同仓权威范式：battle.vue:180-255 战场图例遮罩逐项解释每个视觉元素；battles.vue:1061-1069 R19 绿胜红负语义已固化。图例须逐项覆盖三种编码，不得只列两条折线。

## §2 改点（实施轮执行，本规范轮不改业务源码）

唯一改点：battles.vue:75 一行图例文字。图例文案须同时满足四条：

1. 解释圆点颜色语义（绿=胜 / 红=负）；
2. 线型符号与折线对应（用线型符号如 ━ / ┄ 标注分数实线与漏怪率虚线，不再用实心圆点 ● 标注折线）；
3. 「越高越好」绑分数、「越低越好」绑漏怪率（不误导）；
4. 色彩词（蓝 / 红 / 绿）与 CSS 实际一致。

建议文案方向（实施轮可微调措辞，但须覆盖四条语义）：

> ━ 分数（蓝·越高越好）　┄ 漏怪率（红虚线·越低越好）　● 绿胜 ● 红负

## §3 不变量（契约逐字守卫，严禁改动）

- 三种图形编码结构不变：trend-line / leak-line polyline、trend-win / trend-lose circle、:411 win：it.result==='win'。
- CSS :1045-1054 不变：trend-line var(--ice)、leak-line var(--bad) dasharray 4 3、trend-win var(--ok)、trend-lose var(--bad)。
- 模板 :58 注释、v-if trend.count>=2、svg aria-label=战绩分数趋势、:62 均分文案不变。
- 令牌三锚点不动（tokens.css:14-16）：--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6。
- 零令牌改动（不碰 tokens.css / common.css）；不改任何 computed / API / data；仅图例文字一行。

## §4 契约清单

契约文件：laiyipao/miniapp/src/styles/r88TrendLegend.contract.test.ts，严格参照 r81SkillCollectionProgress.contract.test.ts 范式——fs.readFileSync 真实读 battles.vue（禁内嵌源码副本），linesOf 按行断言，legendBlock 从 trend-legend 容器起截取 4 行。共 6 个 it：

- it1 图例解释胜负色点：图例行须含颜色词 + 胜/负语义（绿+胜、红+负），不得仍是旧文案。
- it2 图例标注分数越高越好、漏怪率越低越好：两词均出现且分别就近绑定分数/漏怪率（正则就近断言）。
- it3 图例线型符号与折线对应：旧错误模式 ● 分数 / ● 漏怪率 不存在，改用线型符号（━/┄/─/—）。
- it4 不变量：三种图形编码结构与 :411 result==='win' 保留。
- it5 不变量：CSS 三编码颜色（var(--ice) / var(--bad) dasharray / var(--ok)）与 v-if / aria-label 保留。
- it6 令牌三锚点保留 + 零 --r88- 令牌 + 写域边界（write-scope 白名单：battles.vue / tokens.css）。

vue-tsc 严格模式注意：无未使用函数（TS6133）、find 结果为 undefined 时做非空守卫（TS2345）——本契约 legendIdx 用 findIndex + 条件截取规避。

## §5 RED 结果

命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r88TrendLegend.contract.test.ts --reporter=verbose

结果：exit RC=1（符合 RED 预期），Test Files 1 failed (1)，Tests 3 failed | 3 passed (6)：

- 失败（缺口断言，符合预期）：it1「图例须解释圆点胜负语义：含胜」expected 旧文案 to contain 胜；it2「图例须标注分数越高越好」expected 旧文案 to contain 越高越好；it3 线型符号断言失败（旧文案用 ● 无 ━/┄）。
- 通过（不变量，符合预期）：it4 三种图形编码结构保留、it5 CSS 三编码颜色与模板守卫保留、it6 令牌三锚点与写域边界保留。

vue-tsc：pnpm exec vue-tsc --noEmit，RC=0，契约本身无 TS 错误（无 TS6133 / TS2345）。

## §6 诚实边界

1. 本轮纯规范 + RED 契约，未改 battles.vue 一行业务源码；实施轮改图例文案后须重跑本契约至 6/6 GREEN。
2. 契约 legendBlock 截取 trend-legend 容器起 4 行，前提实施轮「仅图例文字一行」——若实施轮改为多行图例，须同步扩展截取行数（属契约维护，非本轮范围）。
3. 图例具体措辞规范轮只给方向建议（§2），实施轮可微调，但须过 it1-it3 四条语义断言。
4. 未触碰 QA 库（无 PostgreSQL，PSQL_MISSING）；无真值入仓；报告与契约均为 UNTRACKED。
5. hygiene 边界：本轮写域仅 SPEC.md 与本契约两个 UNTRACKED 文件，battles.vue 与令牌三锚点零改动（HEAD 未变、diff-check RC0、auth 四路径 ABSENT）。走查发现工作树存在一处预存的 tracked 改动 `.gitignore`（timestamp 2026-11-26 14:44:33，早于本任务，内容为 round88-trend-legend 审计路径的 ignore 规则），非本轮产出、不在本轮写域，未触碰未回滚，如实披露供队长裁定。