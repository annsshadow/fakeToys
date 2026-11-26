# R42 规范：Wiki 关卡表补所属章节列与章节×关卡交叉引用

## 0. 缺口依据（亲读带行号）
- 权威字段：laiyipao/admin/src/api/types.gen.ts:510 的 LevelSummary.chapter:number（同文件 :194 另一结构也含 chapter:number）；公开 /api/v1/config 每关都下发 chapter（1–6 分布）。
- 服务端真实消费：laiyipao/server/internal/domain/levelgen.go:84-85 的注释 'ChapterOf 返回关卡所属章节。' 与 func ChapterOf(levelID int) Chapter，逐关按 StartLevel/EndLevel 判定归属，证明 chapter 是配置归属而非展示噪声。
- 本地类型已就绪：laiyipao/admin/src/views/WikiView.vue:55 的 ConfigResp.levels 已含 chapter:number；:750-752 chapterLevels 已按 activeChapter 过滤当前章（约 20 关）；:478 搜索 level hit 的 meta 已用章节号。唯独 :1533-1560 关卡表 13 列（关/名称/难度/防线血量/波次/体力/层数上限/反应阶/星级门槛/首领/地形（配置基值）/护甲‰/满分）没有 chapter 列。
- 交叉引用基础已存在：R41 在 laiyipao/admin/src/views/WikiView.vue:1494 落地 .r41-chapter-mechanics，锚点 id=chapter-mechanics 已在，chapters 的 id/name 六字段已展示，可据此把 level.chapter 映射成章节名并回跳。
- 红线概念：chapter 是配置归属，与结算 terrain_used（参与 replay_hash）不同，不得混同权威战斗数据。
- 令牌锚点：laiyipao/admin/src/styles/tokens.css:12-14 三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 现状保留。

## 1. 范围与零改动面
- 仅改 admin Wiki 展示层：新增纯函数 laiyipao/admin/src/levelChapterText.ts（后续实施任务落地）；WikiView.vue 关卡表新增一列所属章节（章节名可点击回跳章节表锚点），并在关卡表下新增 .r42-level-chapter-note 的 AI 四要素。不新增搜索 kind。
- 禁改 engine/后端/replay/hash/score/settle/loot/tokens；不改 types.gen.ts、不改 chapterMechanicsText.ts、不改其它 Wiki 章节；不动令牌三锚点，禁止新增 --r42-* 令牌。
- 不新增裸 hash 路由：列内链接与 AI CTA 都必须用 urlFor + @click.prevent=goTo，禁止 href="#levels"（createWebHashHistory 的 catch-all 会重定向到数据看板）。
- 不硬编码示例数值：章节数、关卡数由公开 /api/v1/config 实际数据聚合，配置外的数值范围一律不进 Wiki；差异以公开 /api/v1/config 为准。
- 本规范只写契约与测试，不创建业务实现；WikiLevelChapterMechanics.test.ts 对缺失模块整 suite RED，禁止内嵌参考实现。

## 2. 显示格式契约（纯函数 ../src/levelChapterText，真实 import，禁止内嵌参考实现）
- MECHANISM_EMPTY 从 @/levelMechanicsText 同源复用（同值 '—'），禁止在本模块重新声明；测试同时从两个模块导入并断言严格相等。
- 所有函数入参 unknown（真实覆盖脏值），纯显示、无副作用、不改入参；复用与 levelMechanicsText.positiveInt 相同的判定：仅 typeof v === 'number' 且 Number.isInteger(v) 且 v > 0 为合法正整数。
- levelChapterText(chapter: unknown, chapters: unknown): string：chapter 非合法正整数 → MECHANISM_EMPTY。chapter 合法时，在 chapters（仅当 Array.isArray）中找 id 严格等于 chapter 且 id 合法正整数、name 为非空字符串的项，返回该项 name；找不到或 name 非法 → '未知章节#N'（N 为原 chapter 数字）；chapters 非数组/空/元素脏值一律安全回退，不抛异常。
- buildLevelChapterAi(levels: unknown, chapters: unknown): string：逐字模板 共N关，覆盖M章（N、M 为阿拉伯数字，无空格，逗号为中文逗号）。N = Array.isArray(levels) ? levels.length : 0（每个关卡条目计一关，含脏条目，诚实反映数据条数）。M = levels 中合法正整数 chapter 去重后的数量（与 chapters 参数无关；chapters 仅为 API 对称保留，全 unknown 安全）。空/非数组 → '共0关，覆盖0章'。禁止 undefined / NaN / [object Object] 残留。
- 三常量逐字：LEVEL_CHAPTER_AI_BASIS = '依据：公开 /api/v1/config 的 levels.chapter 与 chapters（与服务端 LevelSummary.chapter / domain.ChapterOf 同源）'；LEVEL_CHAPTER_AI_HONESTY = '章节是配置归属，不是地形使用 terrain_used，也不参与 replay_hash；实际战斗数据以结算为准，差异以公开 /api/v1/config 为准'；LEVEL_CHAPTER_AI_CTA = '查看章节一览 →'。
- 不改 types.gen.ts，不改既有 levelMechanicsText / chapterMechanicsText。

## 3. Wiki 关卡表 UI 契约
- 关卡 section（class 为 wiki-section r34-level-mechanics，v-show activeTab === levels）的关卡表（WikiView.vue:1533，:data=chapterLevels）在既有 13 列之后新增第 14 列：表头 label=所属章节，模板调 levelChapterText(row.chapter, cfg?.chapters ?? []) 显示章节名。
- 所属章节列内用可点击链接包住章节名：:href=urlFor('levels', 'chapter-mechanics') + @click.prevent=goTo({ tab: 'levels', anchor: 'chapter-mechanics' })，点击后切到关卡 tab 并滚动到章节表锚点 #chapter-mechanics（R41 已存在），实现章节×关卡交叉引用；不新增裸 hash。
- 原 13 列、rowFocusClass('level')、el-empty 空态、r34-level-table-scroll 横滚全部保留；新增列不破坏表格。
- 关卡表下新增 .r42-level-chapter-note 的 AI 四要素区（结论/依据/红线/CTA 同屏）；结论绑定 {{ buildLevelChapterAi(cfg?.levels ?? [], cfg?.chapters ?? []) }}，禁止手填数值。
- WikiView.vue 必须真实 import { MECHANISM_EMPTY, levelChapterText, buildLevelChapterAi, LEVEL_CHAPTER_AI_BASIS, LEVEL_CHAPTER_AI_HONESTY, LEVEL_CHAPTER_AI_CTA } from '@/levelChapterText'，且 levelChapterText / buildLevelChapterAi 逐个有调用（仅 import 不算）。

## 4. AI 四判据（结论/依据/红线/CTA 同屏）
- A1：.r42-level-chapter-note 内出现可见汉字 'AI 章节交叉概览'。
- A2：结论由公开 cfg.levels 确定性聚合，模板绑定逐字 {{ buildLevelChapterAi(cfg?.levels ?? [], cfg?.chapters ?? []) }}。
- A3：依据常量逐字 LEVEL_CHAPTER_AI_BASIS（见 §2），展示为可见文本且 :data-ai-basis 同值。
- A4：CTA 常量逐字 LEVEL_CHAPTER_AI_CTA = '查看章节一览 →'，必须 hash-router 安全：:href=urlFor('levels') + @click.prevent=goTo({ tab: 'levels', anchor: 'chapter-mechanics' })，禁止裸 href。
- 诚实红线常量逐字 LEVEL_CHAPTER_AI_HONESTY（见 §2）：章节是配置归属非 terrain_used、不参与 replay_hash、以结算为准。四要素同屏，不得把配置归属冒充实战数据。

## 5. 三视口 QA 门禁（实施后必须真浏览器实测）
- 390×844 / 768×1024 / 1280×800 三视口打开 Wiki 关卡 tab：关卡表 14 列内横滚可用、document.documentElement.scrollWidth <= innerWidth + 1；无 NaN / undefined / [object Object]。
- 所属章节列显示章节名（如第 1 关 → 边境哨站），未知章显示 '未知章节#N'，chapter 非法显示 '—'；点击章节名滚到 #chapter-mechanics 且 URL 为 ?tab=levels&anchor=chapter-mechanics。
- AI 聚合按 §2 由真实 cfg.levels 计算，禁止手填；CTA 点击后停留在 /wiki 关卡 tab 的章节表。
- 令牌三锚点 v1 / v3-r3 / r6 保留；R34/R41 既有契约测试全绿；grep 确认无 --r42- 令牌。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiLevelChapterMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, levelChapterText, buildLevelChapterAi, LEVEL_CHAPTER_AI_BASIS, LEVEL_CHAPTER_AI_HONESTY, LEVEL_CHAPTER_AI_CTA } from '../src/levelChapterText'，禁止内嵌参考实现；模块不存在时整 suite RED。
- 逐字断言 §2 全部格式与边界（同源空值、章节名映射、未知章、非法空态、AI 模板与诚实空态、全 unknown 安全）、§0 权威类型与 ChapterOf、§3 列/锚点/真实调用、§4 常量/CTA hash 安全、§1/§5 令牌锚点与禁用 --r42-。
- 先落本规范与测试并执行 cd laiyipao/admin && pnpm exec vitest run tests/WikiLevelChapterMechanics.test.ts --reporter=dot，保留模块缺失的 RED 输出（exit=1、Failed to resolve import '../src/levelChapterText'、无 PARSE_ERROR）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 实施分工提示（后续任务，不在本轮）
- 本轮 t38 只交 SPEC + 契约测试 + RED 证据，不写 levelChapterText.ts、不改 WikiView.vue。
- 后续实施任务：落地 laiyipao/admin/src/levelChapterText.ts（MECHANISM_EMPTY 从 @/levelMechanicsText re-export）、改 WikiView.vue 关卡表新增所属章节列与交叉引用链接、补 .r42-level-chapter-note AI 四要素、完成三视口 QA。

## 8. 全站搜索门禁：复用 chapter 命中，既有类型不回退
- R41 已在 HitKind / HIT_KINDS / KIND_LABEL 加 chapter，并在 searchIndex 增 for (const ch of c.chapters ?? []) 命中；R42 不新增 kind，不重复索引。
- 关卡 level hit（WikiView.vue:475-483）meta 已含章节号与关卡 id，可在 body 追加章节名以提升 chapter 名搜索命中，但不得回退原 meta/body 结构。
- 既有 kind: 'level' / 'chapter' / 'skin' / 'rating' 及对应 rowFocusClass 一律不回退；KIND_LABEL 保留 chapter: 章节、skin: 皮肤、rating: 评分。
