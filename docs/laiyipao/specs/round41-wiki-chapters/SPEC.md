# R41 规范：Wiki 章节元数据 chapters 六字段对齐权威配置

## 0. 缺口依据（亲读带行号）
- laiyipao/admin/src/api/types.gen.ts:435-441 的 ChapterInfo 已有权威六字段 id / name / start_level / end_level / terrain_kind / boss_enemy_id；laiyipao/server/internal/service/service.go:1553-1562 同名响应结构逐字对应，:1649-1652 由 domain.AllChapters() 组装下发。公开 /api/v1/config 已下发，但 Wiki 只在 laiyipao/admin/src/views/WikiView.vue:1474-1477 用章节 el-select 做关卡筛选，六字段从未成表展示。
- laiyipao/server/internal/domain/levelgen.go:42-73 权威六章：1 边境哨站 1–20 oil_drum；2 工业废墟 21–40 collapse_wall；3 冻原 41–60 charge_tower / BossEnemyID 16；4 感染矿井 61–80 tidal_gate；5 母巢外围 81–95 rotor_vane / BossEnemyID 19；6 母巢核心 96–100 charge_tower / BossEnemyID 22。:110 明确五种地形键 oil_drum / tidal_gate / rotor_vane / collapse_wall / charge_tower。
- laiyipao/admin/src/levelMechanicsText.ts 已导出 MECHANISM_EMPTY = '—' 且 TERRAIN_LABELS 只映射 oil_drum→油桶、collapse_wall→塌墙、charge_tower→蓄能塔；本章节的五种映射必须复用这三个同值中文名并补 tidal_gate→潮汐闸门、rotor_vane→旋翼风扇，不得另起译名。
- laiyipao/admin/src/views/WikiView.vue:198,200,202 的 HitKind / HIT_KINDS / KIND_LABEL 共 12 类，无 chapter；:464 的 searchIndex 遍历各配置生成命中，章节名 / id / terrain_kind 搜不到。
- 结算侧 terrain_used 是权威记录（laiyipao/server/internal/domain/battle_collections.go:238,251-275 校验去重且属于该关，参与 replay_hash）；章节 terrain_kind 只是配置基值，两者概念不同，展示必须写清。
- laiyipao/admin/src/styles/tokens.css:13-15 三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 现状保留。

## 1. 范围与零改动面
- 仅改 admin Wiki 展示层：新增纯函数 laiyipao/admin/src/chapterMechanicsText.ts（由后续实施任务落地）；WikiView.vue 关卡 tab 在章节下拉旁增章节六字段表与 AI 四要素；searchIndex 增 chapter 命中；HitKind / HIT_KINDS / KIND_LABEL 同步加 chapter，原有 rating / skin 等 12 类一律不回退。
- 禁改 engine/后端/replay/hash/score/settle/loot/tokens；不改敌人表、技能表、评分权重、其它 Wiki 章节；不动令牌三锚点，禁止新增 --r41-* 令牌。
- 不新增裸 hash 路由：CTA 必须 urlFor('levels') + goTo({ tab: 'levels' })，禁止 href="#levels"（createWebHashHistory 下会被 catch-all 重定向到数据看板）。
- 不硬编码示例数值：hp / armor_permille / max_element / 数值范围等 config 六字段之外的数字一律不进 Wiki；差异以公开 /api/v1/config 为准。
- 本规范只写契约与测试，不创建业务实现；WikiChapterMechanics.test.ts 对缺失模块整 suite RED，禁止内嵌参考实现。

## 2. 显示格式契约（纯函数 ../src/chapterMechanicsText，真实 import，禁止内嵌参考实现）
- MECHANISM_EMPTY 从 @/levelMechanicsText 同源复用（同值 '—'），禁止在本模块重新声明；测试同时从两个模块导入并断言严格相等。
- 所有函数入参 unknown（真实覆盖脏值），纯显示、无副作用、不改入参；复用与 levelMechanicsText.positiveInt 相同的判定：仅 typeof v === 'number' 且 Number.isInteger(v) 且 v > 0 为合法正整数。
- rangeText(start: unknown, end: unknown): string：两端都合法正整数 → 逐字 N–M关（连接号是 en-dash – U+2013，例如 41–60关）；否则 '—'。0 / 负数 / 小数 / NaN / ±Infinity / 字符串 / 布尔 / null / undefined / 对象 → '—'。
- terrainKindText(kind: unknown): string：仅非空字符串为合法 kind；oil_drum→油桶、collapse_wall→塌墙、charge_tower→蓄能塔（与 levelMechanicsText 同值）、tidal_gate→潮汐闸门、rotor_vane→旋翼风扇；其它合法 kind 原样返回 kind 字符串，不编造中文名；非字符串 / 空字符串 → '—'。
- bossText(bossEnemyId: unknown): string：合法正整数 → #N（如 16 → #16、19 → #19、22 → #22）；其余一律 '—'。首领敌人 id 是数字，不是布尔：true / false 必须 '—'（区别于关卡 is_boss 的「是/—」）。
- buildChapterMechanicsAi(chapters: readonly ChapterMechanicsRow[]): string：逐字模板 共 {total} 章：含首领 {boss} 章，地形 {kinds} 种。total = chapters.length；boss = boss_enemy_id 为正整数的章数；kinds = 全部章节中合法 terrain_kind（非空字符串，含未知原样 kind）去重数。空数组 → 共 0 章：含首领 0 章，地形 0 种（诚实空态）。禁止 undefined / NaN / [object Object] 残留。
- ChapterMechanicsRow 至少含 terrain_kind?: unknown / boss_enemy_id?: unknown（对应权威 snake_case 字段），不改 types.gen.ts。

## 3. Wiki 章节表 UI 契约与选择器
- 关卡 section（class 为 wiki-section r34-level-mechanics，v-show activeTab === levels，WikiView.vue:1474）在既有章节 el-select 之后新增 .r41-chapter-mechanics；其内 .r41-chapter-table-scroll 包 el-table，样式 overflow-x:auto；max-width:100%（表内横滚，不造成页面级溢出）。
- 章节表六列：ID（prop="id"）、名称（prop="name"）、关卡区间（模板调 rangeText(row.start_level, row.end_level)）、地形（配置基值）（模板调 terrainKindText(row.terrain_kind)）、首领敌人（模板调 bossText(row.boss_enemy_id)）、说明（由配置字段派生的简短说明，不引入 config 外数值）。:data="cfg?.chapters ?? []"、:row-class-name="rowFocusClass('chapter')"；原关卡表、rowFocusClass('level')、el-empty 空态全部保留。
- 表头「配置基值」必须出现；地形列不得展示 terrain_used，不得与实战触发合并。
- WikiView.vue 必须真实 import { MECHANISM_EMPTY, rangeText, terrainKindText, bossText, buildChapterMechanicsAi, CHAPTER_AI_BASIS, CHAPTER_AI_HONESTY, CHAPTER_AI_CTA } from '@/chapterMechanicsText'，且 rangeText / terrainKindText / bossText / buildChapterMechanicsAi 逐个有调用（仅 import 不算）。

## 4. AI 四判据（结论 / 依据 / 红线 / CTA 同屏）
- A1：.r41-ai-note 内出现可见汉字「AI 章节概览」。
- A2：结论由公开 cfg.chapters 确定性聚合，模板绑定逐字 {{ buildChapterMechanicsAi(cfg?.chapters ?? []) }}；{total}/{boss}/{kinds} 只是说明占位，禁止写进 UI（须用真实数值）。
- A3：依据常量逐字 CHAPTER_AI_BASIS = 依据：公开 /api/v1/config 的 chapters（与服务端 domain.AllChapters 同源）；章节 terrain_kind 为配置基值，实际地形触发以结算 terrain_used 为准。
- A4：CTA 常量逐字 CHAPTER_AI_CTA = 查看章节一览 →，必须 hash-router 安全：:href="urlFor('levels')" + @click.prevent="goTo({ tab: 'levels' })"，禁止裸 href="#levels"；链接保留可复制 / 新标签打开能力。
- 诚实红线常量逐字 CHAPTER_AI_HONESTY = 章节 terrain_kind 是配置基值，与关卡 terrain_used（参与 replay_hash）不同概念，不得混同权威战斗数据；示例值不硬编码，差异以公开 /api/v1/config 为准。
- 诚实口径：章节六字段均为配置基值，不是权威结算；权威回放以 replay_hash 为准。四要素同屏；不得把配置地形冒充本局触发，不得展示 config 外数值范围。

## 5. 三视口 QA 门禁（实施后必须真浏览器实测）
- 390×844 / 768×1024 / 1280×800 三视口打开 Wiki 关卡 tab：章节表内横滚可用、document.documentElement.scrollWidth <= innerWidth + 1；无 NaN / undefined / [object Object]。
- 章节区间用 en-dash（41–60关），首领列严格数字 id 显示 #16，0 / 空显示 '—'；五种地形显示中文，未知 kind 原样。
- AI 聚合按 §4 由真实 cfg.chapters 计算，禁止手填；CTA 点击后停留在 /wiki 关卡 tab。
- 令牌三锚点 v1 / v3-r3 / r6 保留；R34 / R40 既有契约测试全绿；grep 确认无 --r41- 令牌。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiChapterMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, rangeText, terrainKindText, bossText, buildChapterMechanicsAi, CHAPTER_AI_BASIS, CHAPTER_AI_HONESTY, CHAPTER_AI_CTA } from '../src/chapterMechanicsText'，禁止内嵌参考实现；模块不存在时整 suite RED。
- 逐字断言 §2 全部格式与边界（同源空值、五种地形、未知原样、en-dash 区间、#N 数字首领且布尔为 '—'、非法值 '—'、AI 模板与诚实空态）、§0 权威类型与六章五地形、§3 选择器 / 列名 / 真实调用、§4 常量 / CTA hash 安全、§1/§5 令牌锚点与禁用 --r41-。
- 先落本规范与测试并执行 cd laiyipao/admin && pnpm exec vitest run tests/WikiChapterMechanics.test.ts --reporter=dot，保留模块缺失的 RED 输出（exit=1、Failed to resolve import "../src/chapterMechanicsText"、无 PARSE_ERROR）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 实施分工提示（后续任务，不在本轮）
- 本轮 t33 只交 SPEC + 契约测试 + RED 证据，不写 chapterMechanicsText.ts、不改 WikiView.vue。
- 后续实施任务：落地 laiyipao/admin/src/chapterMechanicsText.ts（MECHANISM_EMPTY 从 @/levelMechanicsText re-export）、改造 WikiView.vue 关卡 tab、补 chapter 搜索类型与索引、完成三视口 QA。

## 8. 全站搜索门禁：章节 chapter（既有评分 / 皮肤不回退）
- HitKind / HIT_KINDS / KIND_LABEL 三处同步加 chapter，标签 章节；rating / skin 等原有 12 类保持原样。
- searchIndex 增 for (const ch of c.chapters ?? []) 循环，命中 kind: 'chapter'、id: String(ch.id)、title: ch.name、meta 含 rangeText(ch.start_level, ch.end_level)、body 含 ch.name / ch.id / terrainKindText(ch.terrain_kind) / ch.terrain_kind、section: levels、anchor: chapter-mechanics。
- .r41-chapter-mechanics 节点带 id="chapter-mechanics" 供 scrollToAnchor；pick() 通用写入 focus: { kind: hit.kind, id: hit.id }，章节表 rowFocusClass('chapter') 高亮（走通用 scope.row.id ?? scope.row.key，id 为数字自动 String 化）。
- 既有 kind: 'rating' / kind: 'skin' 命中与 rowFocusClass('rating') 等不得回退；KIND_LABEL 保留 rating: 评分 与 skin: 皮肤。
