# R34 规范：Wiki 关卡机制字段与地形首领 AI 概览

## 0. 缺口依据（亲读带行号）
- laiyipao/admin/src/views/WikiView.vue:1165-1192 关卡 section（v-show activeTab === 'levels'）现有 9 列（关/名称/难度/防线血量/波次/体力/层数上限/反应阶/星级门槛），没有首领、地形、护甲千分比、满分四列；:33 本地 ConfigResp.levels 类型缺 armor_permille/max_score，terrain 只声明为 unknown[]，玩家在文档站看不到这些已下发字段。
- 权威结构 laiyipao/admin/src/api/types.gen.ts:508-524 LevelSummary：seed/base_hp/wave_count/difficulty/energy_cost/star_targets/terrain: TerrainPlacement[]/is_boss/element_cap/armor_permille/max_reaction_tier/max_score，字段 snake_case，与公开 /api/v1/config 逐字一致。
- 地形 laiyipao/miniapp/src/game/types.ts:73-85 TerrainKind 五类 oil_drum/tidal_gate/rotor_vane/collapse_wall/charge_tower，TerrainPlacement{kind,x,y,param}；laiyipao/miniapp/src/game/terrain.ts:1-11 注释逻辑坐标 0..1000，约 41% 关卡使用地形。
- 结算侧 terrain_used 是权威记录（本局实际触发的去重地形集合，参与 replay_hash 校验）；配置里的 terrain 只是“该关配置了哪些地形”，两者不可混同，展示时必须说清。
- 权威配置端点沿用 R33 裁决：WikiView.vue:621-623 api.get<ConfigResp>('/config', { auth: false }) 读公开 /api/v1/config，不以本地硬编码为基准。

## 1. 范围与零改动面
- 仅改 admin Wiki 关卡表展示层：WikiView.vue 本地 ConfigResp.levels 类型补 armor_permille/max_score（terrain 保持 unknown[] 即可，不臆造字段）；关卡表新增首领/地形/护甲/满分四列；表格包一层 .r34-level-table-scroll；新增 .r34-ai-note 说明行。
- 不改 engine.ts / 后端 / replay / hash / score / settle / loot / tokens；不改敌人表、技能表、其它 Wiki 章节；不动令牌三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6，禁止新增 --r34-*。
- 不新增裸 hash 路由：CTA 必须走 urlFor('levels') + goTo({ tab: 'levels' })，禁止 href="#levels"（createWebHashHistory 下会被 catch-all 重定向到数据看板）。
- 本规范只写契约与测试，不创建业务实现；实施模块 laiyipao/admin/src/levelMechanicsText.ts 由后续任务落地。

## 2. 显示格式契约（纯函数 ../src/levelMechanicsText，真实 import，禁止内嵌参考实现）
- 统一空值常量 MECHANISM_EMPTY = '—'。所有函数入参一律 unknown（真实覆盖非数字），纯显示、无副作用、不改入参。
- bossText(v: unknown): string：严格 v === true 返回 '是'；false/0/1/'true'/null/undefined/对象一律 '—'（无首领标记不显示「否」，避免被误读为有此机制）。
- terrainText(terrain: unknown): string：入参必须是数组，逐项取 item.kind（字符串且非空才有效）；按 kind 去重计数，中文映射仅 oil_drum→'油桶'、collapse_wall→'塌墙'、charge_tower→'蓄能塔'；其余合法 kind（如 tidal_gate、rotor_vane）原样显示 kind 字符串，禁止编造中文名；每项格式为 名称×N；多项按首次出现顺序用 '、' 连接，如 '油桶×2、蓄能塔×1'；空数组/非数组/无任何合法 kind → '—'。非对象项或 kind 非字符串项跳过（不编造），但其余合法项照常显示。
- armorPermilleText(v: unknown): string：v 为 number 且 Number.isInteger(v) 且 v > 0 → 原值 + '‰'（千分号，250 → '250‰'）；必须显示千分号 ‰，绝不按百分比写成 25%（与 R31 amplifyPctText、R33 armorText 同一坑）；0/负数/小数/NaN/±Infinity/字符串/null/undefined/对象 → '—'。
- maxScoreText(v: unknown): string：v 为 number 且 Number.isInteger(v) 且 v > 0 → 原值字符串（1200 → '1200'）；0/负数/小数/NaN/±Infinity/非数字 → '—'；不做千分位、不追加「分」以外的推断词。
- 零值规则总纲：0 是真实配置值，语义为「无此机制/无此数值」，统一显示 '—'；非法值同样 '—'。禁止输出 undefined/NaN/[object Object]/Infinity 到表格。
- 诚实口径：全部格式化仅展示配置基值，不参与任何战斗计算；terrain 是配置地形，实际触发以结算 terrain_used 为准。

## 3. Wiki 关卡表 UI 契约与选择器
- 关卡 section（WikiView.vue:1166，v-show activeTab === 'levels'）整体包一层 .r34-level-mechanics；el-table 外包 .r34-level-table-scroll（overflow-x: auto；max-width: 100%），表内横向滚动，不造成页面级横向溢出。
- 关卡表在现有 9 列之后追加四列：首领（模板调 bossText(row.is_boss)）、地形（模板调 terrainText(row.terrain)，表头含「配置基值」字样）、护甲（模板调 armorPermilleText(row.armor_permille)，列名「护甲‰」）、满分（模板调 maxScoreText(row.max_score)）。原 9 列、rowFocusClass('level')、el-empty 空态全部保留。
- 表头必须标明「配置基值」，避免把配置地形冒充实战触发；地形列不得展示 terrain_used 也不得把两者合并。
- 本地 ConfigResp.levels 类型必须含 is_boss、terrain、armor_permille、max_score 四字段，与 snake_case 逐字一致；不修改 types.gen.ts 权威类型。
- 单元测试守卫 WikiView.vue 必须真实 import { bossText, terrainText, armorPermilleText, maxScoreText, MECHANISM_EMPTY, buildLevelMechanicsAi, LEVEL_AI_CTA } from '@/levelMechanicsText'，且对 bossText/terrainText/armorPermilleText/maxScoreText/buildLevelMechanicsAi 逐个有调用（仅 import 不算）。

## 4. AI 四判据（人类可见 AI，结论/依据/CTA/红线同屏）
- A1：Wiki 关卡区必须存在可见汉字 AI 标记（新增 .r34-ai-note 说明行，含「AI 机制概览」字样）。
- A2：AI 结论由公开 cfg.levels 确定性聚合，导出 buildLevelMechanicsAi(levels: readonly LevelMechanicsRow[]): string，逐字模板：共 {total} 关：首领关 {boss} 个，含地形关 {terrain} 个，地形种类 {kinds} 种。total=levels.length；boss 严格 is_boss===true 计数；含地形关按 terrainText(row.terrain) 不为 '—' 计数；地形种类数按所有关卡 terrain 数组中去重后的合法 kind 计数（含未知 kind 原样，不漏计）。空数组返回 共 0 关：首领关 0 个，含地形关 0 个，地形种类 0 种（诚实空态，不编造）。模板绑定逐字 {{ buildLevelMechanicsAi(cfg?.levels ?? []) }}。
- A3：依据行逐字：依据：公开 /api/v1/config 的 levels（与 H5/Go 引擎同源 SeedLevel）；地形实际触发以结算 terrain_used 为准。
- A4：CTA 导出常量 LEVEL_AI_CTA = '查看关卡一览 →'，指向当前关卡 tab。必须 hash-router 安全：:href="urlFor('levels')" + @click.prevent="goTo({ tab: 'levels' })"，复用现有目录导航；禁止裸 href="#levels"。链接保留可复制/新标签打开能力。
- 诚实红线逐字：辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准。禁止把配置地形数量冒充「本局触发地形」，禁止把 max_score 冒充结算得分。
- 结论/依据/CTA/红线四要素同屏可见；{total}/{boss}/{terrain}/{kinds} 只是说明占位，禁止写进 UI（须用真实数值）。

## 5. 三视口 QA 门禁（实施后必须实测）
- 390×844 / 768×1024 / 1280×800 三视口打开 Wiki 关卡 tab：表内横向滚动可用、document.documentElement.scrollWidth <= innerWidth + 1；无 NaN/undefined/[object Object]。
- 地形列核对：配置含 oil_drum 两次与 charge_tower 一次的关卡显示 '油桶×2、蓄能塔×1'；含 tidal_gate 的关卡显示 'tidal_gate×1'（未知中文名不编造）；无地形的关卡显示 '—'。
- 首领列严格按 is_boss===true 显示 '是'；护甲列用千分号 ‰ 不写百分比；满分列为正整数原值。
- AI 聚合按 §4 由真实 cfg.levels 计算，禁止手填；CTA 点击后停留在 /wiki 关卡 tab。
- token 三锚点 v1/v3-r3/r6 保留；R33/R32 既有契约测试全绿；grep 确认无 --r34- 令牌。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiLevelMechanics.test.ts 必须真实 import { bossText, terrainText, armorPermilleText, maxScoreText, MECHANISM_EMPTY, buildLevelMechanicsAi, LEVEL_AI_CTA } from '../src/levelMechanicsText'，禁止内嵌参考实现；模块不存在时整 suite RED。
- 逐字断言 §2 全部格式与边界（严格 true、油桶/塌墙/蓄能塔映射、未知 kind 原样、'、' 连接与 ×N、千分号 ‰、0/非法 '—'、Number.isInteger/Number.isFinite 拒绝小数/NaN/Infinity/非数字）、§0 权威类型与地形种类、§3 选择器/列名/真实调用/类型字段、§4 AI 聚合/依据/CTA/红线/hash 安全链接、§1/§5 令牌锚点与禁用 --r34-。
- 先落本规范与测试并执行 cd laiyipao/admin && pnpm exec vitest run tests/WikiLevelMechanics.test.ts --reporter=verbose，保留模块缺失的 RED 输出（exit=1、Failed to resolve import ../src/levelMechanicsText）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 分工
- t265：本 SPEC + 契约测试 + RED 证据，不写实施。
- 后续实施任务：落地 laiyipao/admin/src/levelMechanicsText.ts 并改造 WikiView.vue 关卡区；完成三视口 QA。
