# R39 规范：Wiki 皮肤图鉴 skins 与结算掉落 drop_rates 字段对齐权威 config

## 0. 缺口依据（亲读带行号）
- 权威下发端点：laiyipao/server/internal/service/service.go:1522 ConfigResp.Skins []domain.SeedSkin json:skins，:1531 DropRates domain.DropRates json:drop_rates（紧邻行含 :1532 起注释），:1636 Skins: domain.SeedSkins，:1643 DropRates: domain.DefaultDropRates()。WikiView.vue:657 已有 api.get<ConfigResp>('/config', { auth: false })，也就是公开 /api/v1/config；本轮不新增后端端点。
- 权威皮肤结构：laiyipao/server/internal/domain/content.go:103-111 SeedSkin 六字段 id/code/name/rarity/passive/descr，json tag 逐字 snake_case；:558-573 SeedSkins 共 6 款：1 skin_ranger/rare/kinetic_bonus 游猎者、2 skin_pyro/epic/fire_burn 炽炎使徒、3 skin_cryo/epic/ice_freeze 霜行者、4 skin_volt/epic/volt_chain 雷鸣者、5 skin_blight/legendary/blight_spread 腐沼行者、6 skin_chalk/legendary/boss_breaker 白垩女王。laiyipao/admin/src/api/types.gen.ts:772-779 同六字段。
- 权威掉落结构：laiyipao/server/internal/domain/battle.go:706-723 DropRates 六键 coin_base/coin_per_star/coin_per_kill/reaction_bonus/keys_on_three_star/energy_on_win，全部 int64；:725-731 DefaultDropRates 为 200/150/20/30/1/5。laiyipao/admin/src/api/types.gen.ts:478 GameConfig.drop_rates 同六键。这些值是配置基值，不是百分比、也不是千分比；CoinBase 源码注释里的「千分比口径」只说明结算基数口径，展示时绝不追加 % 或 ‰。
- 玩家端真实消费：laiyipao/miniapp/src/test/stage_drop_preview_feature.test.ts:10 / :27-35 钉死选关页用 store.config.drop_rates 汇总基础金币、三星金币、击杀预估、反应加成、三星钥匙、通关返还体力；laiyipao/miniapp/src/pages/stage/stage.vue:705 同据。文档站缺 skins 与 drop_rates 两块：WikiView.vue 本地 ConfigResp 到 :93 recipes 截止（无 skins / drop_rates），全文 grep 皮肤|skin|drop_rates|coin_base|reaction_bonus|energy_on_win 只命中 skill_rules.base_cost 的「金币」字样。
- 示例值非独立真源：本节列出的皮肤与掉落数字仅帮助实施核对，差异一律以公开 /api/v1/config 实际下发为准，禁止把示例数字硬编码进 Wiki。

## 1. 范围与零改动面
- 本轮只新增两个文件：本 SPEC 与 laiyipao/admin/tests/WikiSkinDropMechanics.test.ts；先真实 RED，不实现 laiyipao/admin/src/skinDropMechanicsText.ts（由 t18 实施），不改 WikiView.vue，不改任何业务源码。
- 禁改 engine/后端/replay/hash/score/settle/loot/tokens；禁改 SeedSkins/DefaultDropRates 数值；令牌三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 保留，禁止新增 --r39-* 令牌。
- 诚实硬门禁：皮肤被动是机制型词条 code，不自行翻译机制效果；掉落是配置基值，实际结算与获得以游戏内为准；权威回放以 replay_hash 为准；禁止展示任何不在公开 config 的数值真值（词条 MinValue/MaxValue 教训）。

## 2. 显示格式契约（../src/skinDropMechanicsText，真实 import，禁止内嵌参考实现）
- MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用同值破折号（—），禁止在本模块重新声明或写第二份字面量。所有格式化入参一律 unknown；脏值统一 —，禁止输出 NaN/undefined/Infinity/[object Object]。
- rarityText(v: unknown): string：rare→稀有、epic→史诗、legendary→传说（逐字映射，大小写敏感）；其它合法非空字符串（如运营新档位）原样展示；空字符串、非字符串、null/undefined/NaN/对象/数组一律 —。
- passiveText(v: unknown): string：非空字符串原样展示（机制型词条 code 如 kinetic_bonus/boss_breaker），保留原 code 不翻译、不扩展描述；空字符串（含全空白）、非字符串、null/undefined 一律 —。
- dropRatesText(v: unknown): string：只接受有限整数；0 也是合法配置基值，必须显示 String(v)（例如 0→"0"）；null/undefined/小数/负数/NaN/±Infinity/非数字一律 —。绝不写百分比、绝不冒充千分比、绝不加单位词；keys_on_three_star=1 显示「1 把」由 UI 层处理，格式化函数只给数字。逐项格式化六个键时统一调用本函数。
- SkinMechanicsRow 只读输入形状：{ id: number; code: string; name: string; rarity: string; passive: string; descr: string }（与 SeedSkin 六字段对齐，来源公开 config）。
- buildSkinDropMechanicsAi(skins: readonly SkinMechanicsRow[], dropRates: unknown): string：仅接受只读数组，非法入参当作空；确定性模板逐字「共 {total} 款皮肤：稀有 {rare}、史诗 {epic}、传说 {legendary}；掉落 6/6 项齐备」或「共 {total} 款皮肤：稀有 {rare}、史诗 {epic}、传说 {legendary}；掉落 {present}/6 项齐备」。total 统计 id 为正整数的行；三档只统计 rarityText 命中 rar/epic/legendary 三映射的行（原样展示的未知档位不进三档计数，但仍计入 total）；掉落齐备数为 drop_rates 六键中 dropRatesText 返回非 — 的键数。同输入两次严格相等；空皮肤+空掉落返回「共 0 款皮肤：稀有 0、史诗 0、传说 0；掉落 0/6 项齐备」；模板不得残留占位符、undefined、NaN。
- SKIN_DROP_AI_BASIS 逐字：依据：公开 /api/v1/config 的 skins（与服务端 SeedSkin / SeedSkins 同源）与 drop_rates（与服务端 DropRates / DefaultDropRates 同源）；仅展示配置基值。
- SKIN_DROP_AI_HONESTY 逐字：皮肤被动与掉落均为配置基值，实际结算与获得以游戏内为准；权威回放以 replay_hash 为准。
- SKIN_DROP_AI_CTA 逐字：查看皮肤图鉴 →。
- 硬红线：皮肤区不得展示伪数值收益；掉落区不得把 coin_base/reaction_bonus 写成百分比或千分比；不得用生成文案替代机制型 passive code。

## 3. UI 契约与选择器
- SectionKey 增加 'skins'；SECTIONS 目录新增 { key: 'skins', label: '皮肤图鉴' }，位置在 equipment 之后、rules 之前；模板新增 <section v-show="activeTab === 'skins'">，与目录顺序一致，深链 ?tab=skins 可打开。
- WikiView.vue 本地 ConfigResp 必须补 skins: Array<{ id: number; code: string; name: string; rarity: string; passive: string; descr: string }> 与 drop_rates: { coin_base: number; coin_per_star: number; coin_per_kill: number; reaction_bonus: number; keys_on_three_star: number; energy_on_win: number }，字段名逐字保持 snake_case，禁止 camelCase 漂移。
- 皮肤表用 .r39-skin-mechanics 包裹，表格外层 .r39-skin-table-scroll（overflow-x:auto；max-width:100%），只表内横向滚动，不制造页面级横向溢出。五列逐字：ID / 名称 / 稀有度 / 被动 / 说明；ID=row.id，名称=row.name，稀有度=rarityText(row.rarity)，被动=passiveText(row.passive)，说明=row.descr（code 已在被动列，不再并入说明）；空数据用 el-empty(emptyText)。
- drop_rates 小表放在 rules tab 的 #score 区块之后，类名 .r39-drop-table，两列逐字：规则 / 数值；六行键中文名逐字：基础金币 / 每星金币 / 每次击杀金币 / 反应加成 / 三星钥匙 / 通关返还体力，对应键顺序 coin_base/coin_per_star/coin_per_kill/reaction_bonus/keys_on_three_star/energy_on_win；数值一律 dropRatesText(cfg?.drop_rates?.coin_base) 等逐项调用，如 UI 要给钥匙追加「把」，追加只能在数值文本之后，不得篡改格式化函数输出。
- .r39-ai-note 四要素同屏：AI 结论 buildSkinDropMechanicsAi(cfg?.skins ?? [], cfg?.drop_rates ?? {})、依据 SKIN_DROP_AI_BASIS、红线 SKIN_DROP_AI_HONESTY、CTA SKIN_DROP_AI_CTA；三条逐字常量用 :title/:data-* 绑定，确保 DOM 可检查。
- CTA 必须 :href="urlFor('skins')" 且 @click.prevent="goTo({ tab: 'skins' })"，禁止裸 href="#xxx"（R32 教训：createWebHashHistory catch-all 会重定向到 /admin/dashboard）；真链接可复制、可新标签打开，点击后停留在 /wiki 的 skins。
- 真实 import：WikiView.vue 必须从 @/skinDropMechanicsText 引入 MECHANISM_EMPTY / rarityText / passiveText / dropRatesText / buildSkinDropMechanicsAi / SKIN_DROP_AI_BASIS / SKIN_DROP_AI_HONESTY / SKIN_DROP_AI_CTA，并对四个格式化/聚合函数有真实调用（仅 import 不算）。
- searchIndex 增加皮肤命中：遍历 cfg.skins，每条至少一条 SearchHit，kind='skin'（同步扩展 HitKind/HIT_KINDS/KIND_LABEL，中文标签逐字「皮肤」），id=String(s.id)，title=s.name，meta=皮肤稀有度（rarityText 结果），body 必须同时含 name 与 code，section='skins'；按名称或 code 均可搜到。掉落六键不进搜索索引（它是基础规则，不按名称检索）。

## 4. AI 四判据
- A1：皮肤图鉴区有可见汉字 AI 标记（AI 机制概览），不得只藏在注释或测试里。
- A2：AI 结论来自公开 cfg.skins / cfg.drop_rates 的确定性聚合，不手填、不依赖焦点行；模板不得残留 {total}/{rare}/{epic}/{legendary}/{present} 占位符。
- A3：依据逐字 SKIN_DROP_AI_BASIS；红线逐字 SKIN_DROP_AI_HONESTY（含「皮肤被动与掉落均为配置基值，实际结算与获得以游戏内为准」与 replay_hash），与结论/依据/CTA 同屏。
- A4：CTA SKIN_DROP_AI_CTA 用 urlFor+goTo hash 安全链接，可复制新标签打开；点击后 URL 保留 tab=skins。

## 5. 三视口 QA（实施后必须真浏览器实测）
- 390×844 H5 竖屏、768×1024 平板、1280×800 桌面打开 Wiki skins 与 basics#score：皮肤表与掉落小表都在，表内横向滚动可用，document.documentElement.scrollWidth <= innerWidth + 1，无 NaN/undefined/[object Object]/%/‰（掉落区）。
- 逐项核对：6 款皮肤稀有度分布=稀有 1/史诗 3/传说 2；白垩女王被动显示 boss_breaker 原 code；掉落六键显示 200/150/20/30/1/5；人为把 coin_base 设为 0 时显示 0，设为 null/小数/NaN 时显示 —。
- 搜索「白垩」「skin_chalk」能命中皮肤并跳到 skins；CTA 点击不跳看板；目录出现「皮肤图鉴」且 ?tab=skins 深链可还原；令牌三锚点不回退且无 --r39-；R31–R38 既有契约测试全绿。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiSkinDropMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, rarityText, passiveText, dropRatesText, buildSkinDropMechanicsAi, SKIN_DROP_AI_BASIS, SKIN_DROP_AI_HONESTY, SKIN_DROP_AI_CTA } from "../src/skinDropMechanicsText"，禁止内嵌参考实现；模块缺失时整 suite RED。
- 测试覆盖：六字段权威证据与公开 config 端点、皮肤六款与掉落六键真实值、rarityText 三档映射/原样/非法边界、passiveText 非空字符串与非法边界、dropRatesText 0/整数/null/undefined/小数/负数/NaN/±Infinity/非数边界、AI 确定性聚合与脏值、逐字常量、UI 类型/五列/四选择器/真实调用/AI 四要素/hash 安全 CTA/searchIndex 皮肤命中、诚实红线与令牌三锚点。
- RED 门禁命令：cd laiyipao/admin && pnpm exec vitest run tests/WikiSkinDropMechanics.test.ts --reporter=dot；必须 exit=1，逐字 Failed to resolve import "../src/skinDropMechanicsText"，Test Files 1 failed / Tests no tests，且没有 PARSE_ERROR；不得内嵌参考实现伪造 GREEN。

## 7. 实施分工提示（t18）
- 只创建 laiyipao/admin/src/skinDropMechanicsText.ts 并按第 2 节导出；随后按第 3 节改 WikiView.vue；行内样式沿用现有 wiki 卡片/表格风格，不新增 --r39-* 令牌。
- 实施后跑本契约到 GREEN，再交真浏览器复查与终审；任何与公开 config 不一致的示例值以 config 为准。
