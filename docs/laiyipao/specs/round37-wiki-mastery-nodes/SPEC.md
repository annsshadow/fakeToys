# R37 规范：Wiki 专精树节点字段与诚实效果口径

## 0. 缺口依据（亲读带行号）
- 权威节点结构：laiyipao/server/internal/domain/power.go:11-25 的 MasteryNode 含 ID/Family/Layer(1..3)/Slot(0..3)/Name/Kind/Value/PrereqFam(json:prereq_family)/PrereqLay(json:prereq_layer)；MasteryFamily 含 family/name/nodes。laiyipao/server/internal/domain/mastery_json_shape_test.go:67-71 逐字钉住 wire 小写键 id/layer/slot/kind/value/prereq_family/prereq_layer，且禁止大写键。
- 权威效果语义：laiyipao/server/internal/domain/power.go:175-206 的 EvaluateMastery 按 kind 分支：element_cap -> ElementCapBonus += v（层数），reaction_mult -> ReactionMultBonus += v（千分比），heat_cap -> HeatCapBonus += v*10（:181-195 明确注释必须换算成千分比），extra_slot -> ExtraSlots += 1（槽位），skill_damage/crit/armor/mechanic -> 对应 Bonus += v（千分比，见 :115-129）；:240-242 MasteryReactionMultPerNode=200。
- 下发路径：laiyipao/server/internal/service/service.go:1523 的 ConfigResp.Mastery []domain.MasteryFamily json:mastery_families，:1637 以 domain.AllMasteryFamilies() 下发；WikiView.vue:658 已有 api.get<ConfigResp>('/config', { auth: false })，即公开 /api/v1/config。laiyipao/admin/src/api/types.gen.ts:526-542 与 laiyipao/miniapp/src/game/types.ts:134-150 均为全字段 MasteryNode；miniapp/src/game/engine.ts:2549-2571 switch(e.kind) 真实消费。
- 当前漂移：laiyipao/admin/src/views/WikiView.vue:86 的 ConfigResp.mastery_families 只声明 family/name/nodes[{name,layer}]；模板 :1215-1234 仅按 layer filter 后输出 n.name，玩家在文档站看不到 id/kind/value/prereq_family/prereq_layer/slot，即看不到「节点给什么效果、数值口径、前置条件」。这与 R31-R36 同源漂移。

## 1. 范围与零改动面
- 本轮仅交付本规范与契约测试 laiyipao/admin/tests/WikiMasteryMechanics.test.ts，先制造真实 RED；不改 WikiView.vue，不创建 laiyipao/admin/src/masteryMechanicsText.ts，不改任何业务源码。
- 不改 engine/后端/replay/hash/score/settle/loot/tokens；不改种子数值、专精树结构、服务端字段；令牌三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 不动，禁止新增 --r37-* 令牌。
- 效果口径只需展示配置基值，禁止把配置值冒充实际战斗结算结果；实际效果以引擎结算为准，权威回放以 replay_hash 为准。

## 2. 显示格式契约（纯函数 ../src/masteryMechanicsText，真实 import，禁止内嵌参考实现）
- MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用，同值为破折号（—）；禁止在本模块重新声明或写第二份字面量。所有新 helper 入参一律 unknown，非法/0/空值统一 —，禁止输出 NaN/undefined/[object Object]。
- kindText(kind: unknown): string：element_cap->元素层数上限、reaction_mult->反应倍率、heat_cap->热量上限、extra_slot->额外插槽、skill_damage->技能伤害、crit->暴击、armor->护甲、mechanic->机制改造；未知但合法非空字符串原样返回（不伪造中文名）；非字符串/空字符串 -> —。
- valueText(kind: unknown, value: unknown): string：value 必须是有限整数，且 heat_cap 应接受 power.go 中 v*10 后的权威配置值 10/20/30；element_cap/extra_slot 为绝对值语义，其余为千分比语义，逐字规则：heat_cap -> +N‰（N 为 value 原值，不再次乘 10，因为服务端已存 v*10），extra_slot -> +1 槽（value 配置基值不是槽数，须独立显示 +1 槽），element_cap -> +N 层，reaction_mult/skill_damage/crit/armor/mechanic -> +N‰，未知合法 kind 只显示原值并追加说明「单位以游戏内结算为准」；0/负数/小数/NaN/±Infinity/非整数/非法 kind -> —。禁止把千分比冒充百分比，禁止对 heat_cap 再乘 10 或写百分比。
- prereqText(prereq_family: unknown, prereq_layer: unknown): string：空族（非字符串或空串）且 prereq_layer 为 0 -> 无前置；否则必须族为非空字符串且层为有限正整数，返回「前置：{族} 第{N}层」；族合法但层非法、族非法层0以外、其它脏组合 -> —。
- buildMasteryMechanicsAi(families: readonly ...[]): string：入参为只读数组，逐字模板「共 {families} 系：{nodes} 个节点，含前置 {gated} 个，额外插槽 {slots} 个」；families 统计合法 family 对象（含非空字符串 family）数量，nodes 统计其合法节点（含非空字符串 name）数量，gated 统计 prereqText 不以「无前置」开头的合法前置节点数，slots 统计 kind === 'extra_slot' 的合法节点数（每个 +1 槽，不读 value）。两次同输入结果严格相等，空输入返回「共 0 系：0 个节点，含前置 0 个，额外插槽 0 个」。
- MASTERY_AI_BASIS 逐字：依据：公开 /api/v1/config 的 mastery_families（与服务端 MasteryFamily / MasteryNode 同源）；效果按 power.go 口径展示，实际以引擎结算为准。
- MASTERY_AI_HONESTY 逐字：辅助展示配置基值，非权威战斗数据；实际效果以引擎结算为准，权威回放以 replay_hash 为准。
- MASTERY_AI_CTA 逐字：查看专精树 →。

## 3. Wiki 专精树 UI 契约与选择器
- ConfigResp.mastery_families 本地类型必须补齐并与 Go json tag 逐字一致：family/name/nodes[{id,family,layer,slot,name,kind,value,prereq_family,prereq_layer}]，修复当前只有 name/layer 的漂移。
- activeTab === 'mastery' 的 section 整体包 .r37-mastery-mechanics，节点表外包 .r37-mastery-table-scroll（overflow-x: auto；max-width: 100%），只做表内横向滚动，不制造页面级横向溢出。
- 节点表七列逐字命名：系/层/槽/节点/效果/数值/前置；效果列 kindText(n.kind)、数值列 valueText(n.kind,n.value)、前置列 prereqText(n.prereq_family,n.prereq_layer)；保留现有 focus 高亮语义（mastery/n.name），不破坏其它 rowFocusClass。
- .r37-ai-note 四要素同屏：AI 结论（buildMasteryMechanicsAi(cfg?.mastery_families ?? [])）、依据 MASTERY_AI_BASIS、红线 MASTERY_AI_HONESTY、可执行 CTA MASTERY_AI_CTA。
- CTA 必须 :href="urlFor('mastery')" 且 @click.prevent="goTo({ tab: 'mastery' })"，禁止裸 href="#mastery"（R32 教训：createWebHashHistory catch-all 会把裸 hash 重定向到 /admin/dashboard）；链接可复制、可新标签打开。
- WikiView.vue 必须真实 import { MECHANISM_EMPTY, kindText, valueText, prereqText, buildMasteryMechanicsAi, MASTERY_AI_BASIS, MASTERY_AI_HONESTY, MASTERY_AI_CTA } from @/masteryMechanicsText，且对 kindText/valueText/prereqText/buildMasteryMechanicsAi 有真实调用（仅 import 不算）。

## 4. AI 四判据
- A1：专精段有可见汉字 AI 标记（AI 机制概览），不得只藏在注释或测试数据里。
- A2：AI 结论由公开 cfg.mastery_families 确定性聚合，不手填、不依赖焦点行；模板不得残留 {families}/{nodes}/{gated}/{slots} 占位符。
- A3：依据行逐字 MASTERY_AI_BASIS；红线逐字 MASTERY_AI_HONESTY，与结论/依据/CTA 同屏。
- A4：CTA MASTERY_AI_CTA 走 urlFor+goTo，hash-router 安全、可复制新标签打开。
- 红线：禁止把 +N‰ 写成百分比、禁止对 heat_cap 二次乘 10、禁止把配置基值说成实际战斗结果或承诺。

## 5. 三视口 QA 门禁（实施后必须实测）
- 390×844 H5 竖屏、768×1024 平板、1280×800 桌面打开 Wiki 专精 tab：表格表内滚动可用，document.documentElement.scrollWidth <= innerWidth + 1；无 NaN/undefined/[object Object]。
- 逐项核对：heat_cap 配置 10/20/30 显示 +10‰/+20‰/+30‰；reaction_mult 节点（如配置 200）显示 +200‰；element_cap 配置 1/2/3 显示 +1层/+2层/+3层；extra_slot 显示 +1 槽；无前置显示「无前置」；有前置显示「前置：{族} 第{N}层」。
- AI 聚合与 CTA：点击 CTA 停留在 /wiki 专精 tab；令牌三锚点不回退且无 --r37-；R31–R36 既有契约测试全绿。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiMasteryMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, kindText, valueText, prereqText, buildMasteryMechanicsAi, MASTERY_AI_BASIS, MASTERY_AI_HONESTY, MASTERY_AI_CTA } from "../src/masteryMechanicsText"，禁止内嵌参考实现；模块不存在时整 suite RED。
- 逐字断言 §2 格式与 unknown 边界、§0 权威结构与 power.go 消费、§3 类型/列/真实调用、§4 四判据与 hash 安全 CTA、§1/§5 令牌锚点与禁用 --r37-。
- 先执行 cd laiyipao/admin && pnpm exec vitest run tests/WikiMasteryMechanics.test.ts --reporter=dot，保留模块缺失 RED（exit=1、逐字 Failed to resolve import "../src/masteryMechanicsText"、Tests no tests）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 分工
- t6（本轮）：本 SPEC + 契约测试 + RED 证据，不写实施、不改 WikiView.vue。
- 后续实施任务：落地 laiyipao/admin/src/masteryMechanicsText.ts（复用 @/enemyMechanicsText 的 MECHANISM_EMPTY）、改 Wiki 专精树表与本地类型、三视口实测；不得触碰 engine/后端/replay/hash/score/settle/loot/tokens。
