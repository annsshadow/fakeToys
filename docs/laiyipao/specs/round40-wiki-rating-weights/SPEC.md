# R40 规范：Wiki 评分权重 rating_weights 对齐权威五键并修复测试夹具旧键名

## 0. 缺口依据（亲读带行号）
- 权威结构与默认权重：laiyipao/server/internal/domain/power.go:253-259 RatingWeights 五字段逐字 element_coverage / reaction_coverage / mastery_done / equipment_synergy / mechanic_depth，json tag 均为 snake_case；:262-269 DefaultRatingWeights 为 12 / 15 / 10 / 12 / 8。:248-251 注释明确：缺 json tag 会退化成 PascalCase，客户端拿 undefined 且不报错。
- 服务端真实消费：laiyipao/server/internal/domain/power.go:350-354 ComputeBuildRating 用五键加权求和得到 Total。service.go 公开 config 下发 rating_weights；WikiView.vue:658 已有 api.get<ConfigResp>('/config', { auth: false })，即公开 /api/v1/config，本轮不新增端点。
- 玩家端真实消费：laiyipao/miniapp/src/pages/index/index.vue:470-528 按五权重展示五维份额（权重和为 0 诚实回退），:534-550 用各维上界×权重算加权满分，:565-574 按权重×缺口排序边际收益；laiyipao/miniapp/src/test/fixtures.ts:147 使用正确五键。
- 候选识别：docs/laiyipao/recon/ROUND40_RECON.md:1-97 判定候选 A 为三合一真实缺口；公开快照 docs/laiyipao/audits/round38-wiki-recipes/data/config.json 的 rating_weights 段为上述五键与 12/15/10/12/8。示例数字仅帮助核对，差异一律以公开 config 实际下发为准，不得硬编码进 Wiki。

## 1. 范围与零改动面
- 本轮规范与 test-first 阶段只新增两个文件：本 SPEC 与 laiyipao/admin/tests/WikiRatingMechanics.test.ts；先真实 RED，不创建 laiyipao/admin/src/ratingMechanicsText.ts（由实施阶段创建），不改 WikiView.vue，不改任何业务源码。
- 禁改 engine/后端/replay/hash/score/settle/loot/tokens；禁改 power.go 权重算法与 json tag；禁止新增 --r40-* 令牌。令牌锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 保留。
- 诚实硬门禁：Wiki 展示的是公开 config 的配置基值，不是权威结算；权重是原始整数，不换算百分比或千分比；玩家端收益与边界以游戏内为准，权威回放以 replay_hash 为准。

## 2. 格式化契约（../src/ratingMechanicsText，真实 import，禁止内嵌参考实现）
- MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用同值破折号（—），禁止在本模块重新声明或写第二份字面量。所有入参 unknown；脏值统一 —，禁止输出 NaN/undefined/Infinity/[object Object]。
- RATING_KEYS 为只读五键固定顺序：element_coverage / reaction_coverage / mastery_done / equipment_synergy / mechanic_depth，逐字对齐 types.gen.ts:566-572；顺序不得按 Object.entries 漂移。
- ratingLabel(k: unknown): string 五键逐字映射：element_coverage→元素覆盖、reaction_coverage→反应链、mastery_done→专精投入、equipment_synergy→装备契合、mechanic_depth→机制深度。其它合法非空字符串逐字返回「（未知维度#key）」，不猜测、不吞字段；空字符串/全空白/非字符串/null/undefined/数字/布尔/对象/数组一律 —。
- ratingValue(v: unknown): string 只接受有限整数（含 0），直显 String(v)；null/undefined/小数/负数/NaN/±Infinity/字符串数字/布尔/对象/数组一律 —。绝不写 %、绝不写 ‰、绝不追加单位；权重就是原始整数。
- buildRatingMechanicsAi(weights: unknown): string 确定性模板逐字「权重 {present}/5 项齐备」，其中 present 是 RATING_KEYS 五键中 ratingValue 返回非 — 的键数；非法入参当作 0 项齐备；模板不得残留占位符、undefined、NaN。
- RATING_AI_BASIS 逐字：依据：公开 /api/v1/config 的 rating_weights，与服务端 RatingWeights / DefaultRatingWeights 同源；仅展示配置基值。
- RATING_AI_HONESTY 逐字：评分权重是配置基值，不是权威结算；实际收益与边界以游戏内为准；权威回放以 replay_hash 为准。
- RATING_AI_CTA 逐字：查看构筑评分 →。

## 3. UI 契约与选择器
- WikiView.vue:77 本地 ConfigResp.rating_weights 从 Record<string, number> 收紧为封闭五字段结构，与 types.gen.ts:566-572 的 RatingWeights 逐字一致：element_coverage / reaction_coverage / mastery_done / equipment_synergy / mechanic_depth，禁止任意字符串键、禁止 camelCase。
- 评分表用 .r40-rating-mechanics 包裹，表格外层 .r40-rating-table-scroll（overflow-x:auto；max-width:100%），只表内横向滚动，不制造页面级横向溢出。两列逐字：维度 / 权重；五行按 RATING_KEYS 固定顺序渲染，不得再用 Object.entries(cfg?.rating_weights ?? {}) 渲染裸英文 key。
- 维度列逐行调用 ratingLabel(key)，权重列逐行调用 ratingValue(cfg?.rating_weights?.[key])；未下发的键仍保留该行并显示 —（MECHANISM_EMPTY），不静默丢字段。
- .r40-ai-note 四要素同屏：AI 结论 buildRatingMechanicsAi(cfg?.rating_weights ?? {})、依据 RATING_AI_BASIS、红线 RATING_AI_HONESTY、CTA RATING_AI_CTA；三条逐字常量用 :title/:data-* 绑定，确保 DOM 可检查。
- CTA 必须 :href="urlFor('rating')" 且 @click.prevent="goTo({ tab: 'rating' })"；若实施阶段复用现有 basics tab，则必须使用 urlFor('basics') + goTo({ tab: 'basics' }) 且锚点定位到评分区块，禁止裸 href="#score" / href="#xxx"（R32 教训：createWebHashHistory catch-all 会重定向到 /admin/dashboard）。SPEC 定：以 urlFor('rating')+goTo({tab:'rating'}) 为准，SectionKey 增加 'rating' 与 SECTIONS 条目「构筑评分」；不新增裸 hash。
- 真实 import：WikiView.vue 必须从 @/ratingMechanicsText 引入 MECHANISM_EMPTY / RATING_KEYS / ratingLabel / ratingValue / buildRatingMechanicsAi / RATING_AI_BASIS / RATING_AI_HONESTY / RATING_AI_CTA，且对格式化/聚合函数有真实调用（仅 import 不算）。
- 同步修正测试夹具：laiyipao/admin/tests/WikiView.test.ts:69 的 rating_weights 必须改为五权威键（示例 12/15/10/12/8），:157-164 断言必须从 damage/30/tempo 改为中文标签（至少「元素覆盖」「机制深度」）与对应权重数字；这是实施阶段 t23 的写域，本规范只写明要求，本阶段不改该文件。

## 4. AI 四判据
- A1：评分权重区有可见汉字 AI 标记（AI 机制概览），不得只藏在注释或测试里。
- A2：AI 结论来自公开 cfg.rating_weights 的确定性聚合，不手填、不依赖焦点行；模板不得残留 {present} 占位符。
- A3：依据逐字 RATING_AI_BASIS；红线逐字 RATING_AI_HONESTY（含「配置基值，不是权威结算」与 replay_hash），与结论/依据/CTA 同屏。
- A4：CTA RATING_AI_CTA 用 urlFor+goTo hash 安全链接，可复制新标签打开；点击后 URL 保留 tab=rating（或复用的安全 basics tab），不跳看板。

## 5. 三视口 QA（实施后必须真浏览器实测）
- 390×844 H5 竖屏、768×1024 平板、1280×800 桌面打开 Wiki rating：五行中文标签与整数权重齐全，表内横向滚动可用，document.documentElement.scrollWidth <= innerWidth + 1，无 NaN/undefined/[object Object]/%/‰。
- 逐项核对：默认权重五键显示元素覆盖 12 / 反应链 15 / 专精投入 10 / 装备契合 12 / 机制深度 8；人为删除 mastery_done 时该行显示 —，AI 结论显示「权重 4/5 项齐备」；人为注入旧键 damage 时显示「（未知维度#damage）」，不吞字段。
- CTA 点击不跳看板；目录出现「构筑评分」且 ?tab=rating 深链可还原；WikiView.test.ts 夹具与断言改为权威键/中文标签后全绿；令牌三锚点不回退且无 --r40-；R31–R39 既有契约测试全绿。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiRatingMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, RATING_KEYS, ratingLabel, ratingValue, buildRatingMechanicsAi, RATING_AI_BASIS, RATING_AI_HONESTY, RATING_AI_CTA } from "../src/ratingMechanicsText"，禁止内嵌参考实现；模块缺失时整 suite RED。
- 测试覆盖：五权威 json tag 与默认权重、服务端 ComputeBuildRating、types.gen.ts 封闭五字段、miniapp 双端消费、本地类型收紧、五键映射/未知键兜底/脏值、整数含 0/非法边界、AI 确定性齐备数与脏值、逐字常量、UI 选择器与真实调用、AI 四要素与 hash 安全 CTA、WikiView.test.ts 夹具/断言修正要求、诚实红线与令牌锚点。
- RED 门禁命令：cd laiyipao/admin && pnpm exec vitest run tests/WikiRatingMechanics.test.ts --reporter=dot；必须 exit=1，逐字 Failed to resolve import "../src/ratingMechanicsText"，Test Files 1 failed / Tests no tests，且没有 PARSE_ERROR；不得内嵌参考实现伪造 GREEN。

## 7. 实施分工提示（t23）
- 创建 laiyipao/admin/src/ratingMechanicsText.ts 并按第 2 节导出；按第 3 节收紧 WikiView.vue 本地类型、替换 Object.entries 渲染、补 AI 四要素与 hash 安全 CTA；同步修正 WikiView.test.ts:69 与 :157-164；行内样式沿用现有 wiki 卡片/表格风格，不新增 --r40-* 令牌。
- 实施后跑本契约到 GREEN，再交真浏览器复查与终审；任何与公开 config 不一致的示例值以 config 为准。

## 8. 全站搜索门禁：评分五维度（t29 修复）
- `HitKind` 增加 `'rating'`，同步 `HIT_KINDS` 与 `KIND_LABEL.rating = '评分'`；`?focus=rating:<key>` 为合法深链。
- `searchIndex` 遍历 `RATING_KEYS` 逐键 push：`kind:'rating'`、`id:key`、`title: ratingLabel(key)`、`body` 同时含英文 key、中文标签与 `ratingValue` 权重整数、`section:'rating'`。
- 搜索示例必须真实命中并点击高亮：「反应链」→ reaction_coverage 行（权重 15）；`mechanic_depth` → 机制深度行（权重 8）；「元素覆盖」→ element_coverage 行（权重 12）。
- 评分表 `<el-table>` 绑定 `:row-class-name="rowFocusClass('rating')"`；命中行以 `row-focus` 高亮，哈希保持 `?tab=rating&focus=rating:<key>`，不跳看板。
- 诚实口径不变：搜索命中的权重仍是公开 config 的配置基值，不是权威结算；实际收益与边界以游戏内为准，权威回放以 replay_hash 为准。
