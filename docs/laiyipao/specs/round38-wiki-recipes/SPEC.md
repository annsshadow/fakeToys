# R38 规范：Wiki 合成配方（recipes）字段与诚实口径

## 0. 缺口依据（亲读带行号）
- 权威下发端点：laiyipao/server/internal/service/service.go:1518 的 ConfigResp.Recipes []domain.SeedRecipe json:recipes，:1633 Recipes: domain.SeedRecipes。WikiView.vue:661 已有 api.get<ConfigResp>('/config', { auth: false })，即公开 /api/v1/config，无需新增后端端点。
- 权威配方结构：laiyipao/server/internal/domain/content.go:59-64 SeedRecipe 仅四字段 output/a/b/out_tier，json tag 逐字为 output/a/b/out_tier；:474-495 SeedRecipes 共 18 条，例：31=1+2 二阶灼热弹幕、33=3+11 三阶焦土重锤、48=12+15 三阶光棱攻城。laiyipao/admin/src/api/types.gen.ts:743-748 同四字段。
- 玩家端真实消费：laiyipao/miniapp/src/pages/bag/bag.vue:101-145 熔炼区展示「材料A + 材料B → 产物 N 阶」；:732-734 `const recipes = computed(() => store.config?.recipes ?? [])`；:765-767 skillNameOf 用技能 id 查名，未知 id 只回退 #id，不编造。当前 WikiView.vue 本地 ConfigResp 无 recipes（见 :75-88 附近），searchIndex 也无配方命中（:428 起只索引 skill/composite 等），玩家文档站无法回答「合成技能由哪两个技能熔炼而成」。
- 与 R36 互补：R36 合成技能表只展示属性；R38 展示同 section 的获取方式（配方），不重复属性列，不覆盖 R36 选择器 r36-composite-mechanics / r36-composite-table-scroll。

## 1. 范围与零改动面
- 本轮只新增本 SPEC 与契约测试 laiyipao/admin/tests/WikiRecipeMechanics.test.ts；先真实 RED，不实现 laiyipao/admin/src/recipeMechanicsText.ts，不改 WikiView.vue，不改任何业务源码。
- 禁改 engine/后端/replay/hash/score/settle/loot/tokens；禁改种子配方数值与结构；令牌三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 保留，禁止新增 --r38- 令牌。
- 诚实硬门禁：配方只说明获取方式，能否熔炼以游戏内材料拥有状态为准；配置基值非权威战斗数据，实际效果以引擎结算为准，权威回放以 replay_hash 为准；禁止展示任何不在公开 config 的数值真值（词条 MinValue/MaxValue 教训）。

## 2. 显示格式契约（../src/recipeMechanicsText，真实 import，禁止内嵌参考实现）
- MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用同值破折号（—），禁止在本模块重新声明或写第二份字面量。所有格式化入参一律 unknown；0/负数/小数/NaN/±Infinity/null/undefined/非数字/空对象一律 —，禁止输出 NaN/undefined/[object Object]。
- recipeMaterialsText(r: unknown, skillNameById: unknown): string：r 必须是含 output/a/b 的对象；a、b 必须为正整数；从 skillNameById（Map/Record/函数均可）按 id 映射为技能名称；映射得到非空字符串则原样显示，格式为「{材料A} + {材料B}」；未知 id 必须逐字标注为「（未知技能#N）」（例如 id=999 输出「（未知技能#999）」），绝不编造或猜测技能名。a/b 非法时统一 —。映射函数抛错也回退为未知标注，不得把异常泄漏给玩家。
- tierText(out_tier: unknown): string：仅有限正整数 -> 「N 阶」（如 2 -> 2 阶、3 -> 3 阶）；0/负数/小数/NaN/Infinity/null/undefined/非数字一律 —。
- buildRecipeMechanicsAi(recipes: unknown, skills: unknown, compositeSkills: unknown): string：三个入参必须是只读数组；逐字模板「共 {n} 条配方：二阶 {t2} 条、三阶 {t3} 条」。n 只统计 output/a/b 均为合法正整数的配方；t2/t3 分别统计其中 out_tier 严格等于 2 或 3 的配方。skills/compositeSkills 仅用于名称映射的候选集合，不参与计数；它们含脏数据时不得改变聚合结果。两次同输入严格相等；空数组返回「共 0 条配方：二阶 0 条、三阶 0 条」。
- RECIPE_AI_BASIS 逐字：依据：公开 /api/v1/config 的 recipes（与服务端 SeedRecipe / SeedRecipes 同源）；仅展示配方，不改熔炼结算。
- RECIPE_AI_HONESTY 逐字：能否熔炼以游戏内材料拥有状态为准；配置基值非权威战斗数据，实际效果以引擎结算为准，权威回放以 replay_hash 为准。
- RECIPE_AI_CTA 逐字：查看合成配方 →。
- 硬红线：不得在配方区显示 base_damage / heat_cost / cooldown_ms 等战斗数值，也不得显示 MinValue/MaxValue 词条范围；配方区只回答「由哪两个技能熔炼、产物几阶」。

## 3. UI 契约与选择器
- ConfigResp 本地类型必须补 `recipes: Array<{ output: number; a: number; b: number; out_tier: number }>`，与 Go json tag 逐字一致；字段名禁止 camelCase（保持 snake_case）。
- 配方表放在 skills tab 的 #composite 锚点区内、R36 合成技能表之后，使用 .r38-recipe-mechanics 包裹，表格外层 .r38-recipe-table-scroll（overflow-x:auto；max-width:100%），仅表内横向滚动，不制造页面级横向溢出，也不改 r36-* 选择器。
- 配方表四列逐字命名：产物 / 材料A / 材料B / 阶数；产物=recipeMaterialsText 之外的 output 对应技能名（映射不到显示「（未知技能#N）」），材料A=recipeMaterialsText 的 a 部分，材料B=recipeMaterialsText 的 b 部分，阶数=tierText(row.out_tier)；空数据用 el-empty(emptyText)。
- .r38-ai-note 四要素同屏：AI 结论 buildRecipeMechanicsAi(cfg?.recipes ?? [], cfg?.skills ?? [], cfg?.composite_skills ?? [])、依据 RECIPE_AI_BASIS、红线 RECIPE_AI_HONESTY、CTA RECIPE_AI_CTA；三条逐字常量用 :title/:data-* 绑定，确保 DOM 可检查。
- CTA 必须 :href="urlFor('skills', 'composite')" 且 @click.prevent="goTo({ tab: 'skills', anchor: 'composite' })"，禁止裸 href="#xxx"（R32 教训：createWebHashHistory catch-all 会重定向到 /admin/dashboard）；真链接保证可复制、可新标签打开，点击后停留在 /wiki 的 skills/composite。
- 目录/深链：配方表沿用 skills 目录项与 #composite 锚点，不新增 SECTION key；URL `?tab=skills&anchor=composite` 打开即定位到含 R36 表 + R38 配方表的合成区。
- 真实 import：WikiView.vue 必须从 @/recipeMechanicsText 引入 MECHANISM_EMPTY / recipeMaterialsText / tierText / buildRecipeMechanicsAi / RECIPE_AI_BASIS / RECIPE_AI_HONESTY / RECIPE_AI_CTA，且对 recipeMaterialsText/tierText/buildRecipeMechanicsAi 有真实调用（仅 import 不算）。
- searchIndex 必须新增配方命中：遍历 cfg.recipes，每条至少一条 SearchHit，kind 可为 'recipe'（需同步扩展 HitKind/KIND_LABEL，中文标签逐字「配方」），id=String(r.output)，title=产物技能名（映射不到用「（未知技能#N）」），meta='合成配方'，body 必须同时包含材料A/材料B名称与阶数文本，使按产物名或材料名都能搜到；section='skills'，anchor='composite'。不得用裸 '#id' 当唯一标题。

## 4. AI 四判据
- A1：合成技能区有可见汉字 AI 标记（AI 机制概览），不得只藏在注释或测试里。
- A2：AI 结论来自公开 cfg.recipes/cfg.skills/cfg.composite_skills 的确定性聚合，不手填、不依赖焦点行；模板不得残留 {n}/{t2}/{t3} 占位符。
- A3：依据逐字 RECIPE_AI_BASIS；红线逐字 RECIPE_AI_HONESTY（含材料拥有状态与 replay_hash），与结论/依据/CTA 同屏。
- A4：CTA RECIPE_AI_CTA 用 urlFor+goTo hash 安全链接，可复制新标签打开；点击后 URL 保留 tab=skills&anchor=composite。

## 5. 三视口 QA（实施后必须真浏览器实测）
- 390×844 H5 竖屏、768×1024 平板、1280×800 桌面打开 Wiki skills/composite：R36 属性表与 R38 配方表都在，表内横向滚动可用，document.documentElement.scrollWidth <= innerWidth + 1，无 NaN/undefined/[object Object]。
- 逐项核对：31=制式炮管+燃油弹→二阶；33=焦土重锤为三阶；48=光棱攻城为三阶；未知 id 显示「（未知技能#N）」；非法 out_tier 显示 —。
- 搜索「焦土」「光棱」等产物/材料名能命中配方并跳到 skills/composite；CTA 点击不跳看板；令牌三锚点不回退且无 --r38-；R31–R37 既有契约测试全绿。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiRecipeMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, recipeMaterialsText, tierText, buildRecipeMechanicsAi, RECIPE_AI_BASIS, RECIPE_AI_HONESTY, RECIPE_AI_CTA } from "../src/recipeMechanicsText"，禁止内嵌参考实现；模块缺失时整 suite RED。
- 测试覆盖：四字段权威证据、id→技能名映射与未知 id 诚实标注、tierText 正整数/非法边界、AI 确定性聚合与非法入参、unknown 全边界（0/负数/小数/NaN/Infinity/null/undefined/非数字）、UI 类型/四列/选择器/真实调用/AI 四要素/hash 安全 CTA/searchIndex 配方命中、诚实红线与令牌三锚点。
- 先执行 cd laiyipao/admin && pnpm exec vitest run tests/WikiRecipeMechanics.test.ts --reporter=dot，保留模块缺失 RED（exit=1、逐字 Failed to resolve import "../src/recipeMechanicsText"、Tests no tests）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 分工
- t12（本轮）：本 SPEC + 契约测试 + RED 证据，不写实施、不改 WikiView.vue。
- 后续实施：落地 laiyipao/admin/src/recipeMechanicsText.ts（复用 @/enemyMechanicsText 的 MECHANISM_EMPTY）、补 ConfigResp.recipes 与配方表/searchIndex、真浏览器三视口验证；不得触碰 engine/后端/replay/hash/score/settle/loot/tokens。
