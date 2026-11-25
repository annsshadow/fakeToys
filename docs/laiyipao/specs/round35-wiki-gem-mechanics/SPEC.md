# R35 规范：Wiki 宝石图鉴与公开 config 权威对齐

## 0. 缺口依据（亲读带行号）
- 权威定义 laiyipao/server/internal/domain/content.go:82-88 SeedGem{ID,Code,Name,Attr,Descr}；:547-556 SeedGems 8 种（attack/element_coef/crit/reaction_mult/element_cap/heat_cap/armor/skill_damage）；:91-101 GemQualities 5 档（gray/1/1000、blue/1/1200、purple/2/1450、red/2/1700、gold/3/2000，name/affix_count/mult）。
- 下发路径 laiyipao/server/internal/service/service.go:1520-1521 ConfigResp.Gems []domain.SeedGem + Qualities []QualityInfo；:1635 Gems: domain.SeedGems；:1645-1647 逐档 append QualityInfo{q.Name,q.Count,q.Mult}；QualityInfo 定义 :1548（Name string / Count int / Mult int64）。公开 /api/v1/config 快照含 gems(8) 与 gem_qualities(5)，字段 snake_case。
- 双端真实消费：laiyipao/server/internal/service/gems.go roll 词条（:154 GemQualityMult、:161 词条值=value*mult/1000、:186 从 SeedGems 随机选），miniapp 背包镶嵌读 gem 实例；词条与品质进 attacker 派生属性并进 replayHash prefix（gems.go:16），故 Wiki 只做展示，不改变 roll/结算。
- 缺口：laiyipao/admin/src/views/WikiView.vue ConfigResp(:60-81) 无 gems/gem_qualities；SectionKey(:85-94) 无 gems；SECTIONS(:114-144) 无宝石目录；HitKind/HIT_KINDS/KIND_LABEL(:155-167) 无 gem；searchIndex(:414-) 无宝石命中；模板关卡 section(:1177-1215) 之后直接实战统计(:1217)，无宝石 section。玩家文档站对「宝石有哪些属性、品质分几档」无答案。

## 1. 范围与零改动面
- 仅改 admin 文档站展示层：WikiView.vue 的 ConfigResp 补 gems/gem_qualities 类型、SectionKey/SECTIONS/HIT_KINDS/KIND_LABEL/searchIndex、模板宝石 section、.r35-* CSS；新增 laiyipao/admin/src/gemMechanicsText.ts 纯展示 helper（实施任务落地）。
- 不改 engine / 后端 / replay_hash / score / settle / loot / tokens；不改 SeedGemAffixes 数值与 roll 逻辑；不动令牌三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6，禁止新增 --r35-*。
- CTA 必须走 urlFor + goTo（hash-router 安全），禁止裸 href="#xxx"（R32 教训：createWebHashHistory catch-all 会把裸 hash 重定向到数据看板）。
- 本规范只写契约与测试，不创建业务实施模块、不改 WikiView；实施任务据此落地后复跑应全绿。

## 2. 显示格式契约（纯函数 ../src/gemMechanicsText，真实 import，禁止内嵌参考实现）
- 统一空值常量 MECHANISM_EMPTY = 破折号（—）。所有函数入参一律 unknown，纯展示、无副作用、不改入参。0/null/undefined/小数/NaN/±Infinity/字符串/对象等非合法值统一 —，禁止输出 undefined/NaN/[object Object]。
- attrText(v: unknown): string：严格按权威 attr 映射 attack→攻击、element_coef→元素系数、crit→暴击率、reaction_mult→反应倍率、element_cap→元素层数上限、heat_cap→热量上限、armor→护甲、skill_damage→技能伤害；未知但非空字符串原样返回（不伪造中文名）；空字符串/非字符串/null/undefined → —。
- qualityText(v: unknown): string：入参为合法对象且含 name(非空字符串)/affix_count(正整数)/mult(正整数)；返回 name + 间隔符 + affix_count 条 + 间隔符 + mult 千分号。逐字段：name 非空字符串原样（未知档位原样，不伪造）；affix_count 为 Number.isInteger 且 >0 显示 N 条，否则 —；mult 为 Number.isInteger 且 >0 显示 N‰（千分比系数，绝不换算百分比，如 1000 显示 1000‰ 而非 100%）。格式逐字示例：gray · 1 条 · 1000‰。非对象/null/undefined/数组 → —。
- GEM_AFFIX_NOTE = 词条随机，具体数值以游戏内为准（逐字，诚实边界硬门禁）。
- GEM_AI_BASIS = 依据：公开 /api/v1/config 的 gems 与 gem_qualities（与服务端 SeedGem / GemQualities 同源）；词条随机，具体数值以游戏内为准（逐字）。
- GEM_AI_HONESTY = 辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准（逐字）。
- GEM_AI_CTA = 查看宝石图鉴 →（逐字）。
- buildGemMechanicsAi(gems: readonly unknown[], qualities: readonly unknown[]): string：由配置确定性聚合；宝石种类数=gems 数组长度；品质档数=qualities 数组长度；词条条数范围取 qualities 中合法正整数 affix_count 的最小值与最大值（1..3 显示 1–3 条，单值显示 N 条，全非法显示 —）；逐字模板：共 G 种宝石，Q 档品质，词条 MIN–MAX 条。空输入返回 共 0 种宝石，0 档品质，词条 — 条（诚实空态，不编造）。两次调用同输入结果严格相等。
- 诚实边界：SeedGemAffixes 的 MinValue/MaxValue 词条数值范围不在公开 /config，禁止在 Wiki 任何位置展示具体数值范围；文案必须如实写 GEM_AFFIX_NOTE；attr/quality 未知项原样展示不伪造；mult 是千分比系数不臆造百分比。

## 3. Wiki 宝石图鉴 UI 契约与选择器
- SectionKey 新增 gems；SECTIONS 新增目录项 label 逐字「宝石图鉴」；HitKind 新增 gem 与 quality 并加入 HIT_KINDS，KIND_LABEL.gem = 宝石、KIND_LABEL.quality = 宝石品质。
- ConfigResp 新增 gems: Array<{id:number;code:string;name:string;attr:string;descr:string}> 与 gem_qualities: Array<{name:string;affix_count:number;mult:number}>，字段名与 Go json tag 逐字一致（snake_case）。
- 模板新增 section v-show activeTab === gems，整体包 .r35-gem-mechanics；el-table 外包 .r35-gem-table-scroll（overflow-x:auto；max-width:100%），表内横向滚动不造成页面级溢出。
- 宝石表四列：ID / 名称 / 属性（attrText(row.attr)）/ 说明（row.descr）；行 :row-class-name="rowFocusClass('gem')"；空态 el-empty 沿用 emptyText。品质表三列：品质 / 词条数 / 系数（qualityText 或逐字段函数展示 mult 千分号，禁百分比）；品质表绑定 :row-class-name="rowFocusClass('quality')"，rowFocusClass 仅对 kind='quality' 用 scope.row.name 与焦点 id 比较，其余表仍用 id??key 语义不变。
- searchIndex 宝石命中：遍历 c.gems 生成 kind:gem 的 SearchHit，id=String(id)，title=name，meta 含 attrText 结果，body=descr，section=gems；另遍历 c.gem_qualities 生成 kind:quality 的 SearchHit，id=name、title=name、meta 含 qualityText 结果、body 含词条数/倍率文本、section=gems；点击经 goTo 跳宝石 tab 并高亮对应行（宝石 rowFocusClass gem、品质 rowFocusClass quality）。搜索门禁：搜索 "gold" 必须命中品质条目并产生 focus=quality:gold。
- .r35-ai-note 四要素：AI 结论（buildGemMechanicsAi(cfg?.gems ?? [], cfg?.gem_qualities ?? [])）、依据 GEM_AI_BASIS、CTA GEM_AI_CTA（a :href="urlFor('gems')" @click.prevent="goTo({ tab: 'gems' })"）、红线 GEM_AI_HONESTY；并展示 GEM_AFFIX_NOTE 词条随机说明。
- 测试守卫 WikiView.vue 必须真实从 @/gemMechanicsText import 上述具名导出，且对 attrText/qualityText/buildGemMechanicsAi 有真实调用（仅 import 不算）。

## 4. AI 四判据
- A1 人类可见 AI：宝石 section 含可见汉字 AI 标记（AI 机制概览）。
- A2 真实数据结论：由公开 cfg 确定性聚合（种类数/档数/词条范围），不手填、不编造。
- A3 依据行逐字 GEM_AI_BASIS，含词条随机说明。
- A4 可执行 CTA GEM_AI_CTA 走 urlFor+goTo，hash-router 安全、可复制新标签打开。
- 红线逐字 GEM_AI_HONESTY，与依据/CTA/结论同屏可见；禁止把词条数值范围、百分比系数、配置值冒充实战效果。

## 5. 三视口 QA 门禁（实施后实测）
- 390×844 / 768×1024 / 1280×800 打开 Wiki 宝石图鉴 tab：两表横向滚动可用、document.scrollWidth <= innerWidth + 1；无 NaN/undefined/[object Object]。
- 8 种宝石属性中文映射逐条核对；未知 attr 原样；quality 5 档 mult 显示千分号（1000‰/1200‰/1450‰/1700‰/2000‰）不写百分比。
- 搜索「锋锐石」「gold」命中并跳宝石 tab 高亮；CTA 停留在 /wiki 宝石 tab；目录项与 section 一一对应。
- 全文无 MinValue/MaxValue 数值范围展示；令牌三锚点保留、无 --r35-；R31-R34 既有测试全绿。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiGemMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, attrText, qualityText, buildGemMechanicsAi, GEM_AFFIX_NOTE, GEM_AI_BASIS, GEM_AI_HONESTY, GEM_AI_CTA } from ../src/gemMechanicsText，禁止内嵌参考实现；模块不存在时整 suite RED。
- 逐字断言 §2 全部格式与 unknown 边界、§0 权威字段与种子条目、§3 选择器/列/真实调用/类型字段、§4 四判据与 hash 安全 CTA、§1/§5 令牌锚点与禁用 --r35-、诚实边界禁数值范围。
- 先执行 cd laiyipao/admin && pnpm exec vitest run tests/WikiGemMechanics.test.ts --reporter=verbose，保留模块缺失 RED（exit=1、Failed to resolve import ../src/gemMechanicsText）作为 test-first 证据；实施落地后同命令全绿。

## 7. 分工
- t270：本 SPEC + 契约测试 + RED 证据，不写实施。
- 后续实施任务：落地 laiyipao/admin/src/gemMechanicsText.ts 并改造 WikiView.vue 宝石图鉴；完成三视口 QA。