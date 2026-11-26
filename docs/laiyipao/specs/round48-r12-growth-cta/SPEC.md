# R48 规范：R12 成长优先级 CTA 自导航死循环修复

> 一句话：R12 跨系统优先级卡的 CTA 在 bag／equipment 页指向自身路由，点击后导航回本页形成死循环；本轮只在消费层识别自指并改为页内可执行动作与页内指引文案，不改短板算法与非自指跨页导航。

## §0 缺口依据（源码行号，规范轮已独立核实）
- laiyipao/miniapp/src/pages/_shared/r12GrowthPriority.ts:40-46 DIM_HINT 五维去向：元素覆盖:/pages/bag/bag（:41）、机制深度:/pages/bag/bag（:45）、装备契合:/pages/equipment/equipment（:44）、反应链:/pages/reactions/reactions（:42）、专精投入:/pages/mastery/mastery（:43）。
- :53 computeGrowthPriority 为纯函数；:71-72 的 hint 与 hintLabel 直接取 DIM_HINT，:74-81 只做排序与本分，不判断当前页。
- laiyipao/miniapp/src/pages/bag/bag.vue:12 CTA 为 r12-priority-cta 绑定 r12Go()；:513 r12Priority 计算；:514-517 r12Go 无条件 uni.navigateTo（url: p.hint）。bag 自身路由即 /pages/bag/bag（pages.json:31），故元素覆盖／机制深度为短板时点 CTA 跳回本页。
- laiyipao/miniapp/src/pages/equipment/equipment.vue:17 CTA 同构；:265 计算；:266-269 r12Go 同样无条件 navigateTo；装备契合为短板时跳回 /pages/equipment/equipment。
- laiyipao/miniapp/src/pages/gems/gems.vue:18/:175 与 skills/skills.vue:17/:300 同构，但五维 hint 从不指向 gems／skills，故当前无自指缺陷（仍纳入防御性防护）。
- laiyipao/miniapp/src/pages/index/index.vue:668-674 DIM_HINT 同源副本、:676-678 go()；首页路由为 index，从不自指，本轮不动首页副本。
- 既有契约 laiyipao/miniapp/src/styles/r12GrowthPriority.contract.test.ts:55-68 只覆盖一个跨页场景（装备契合→equipment）与空态，无自指守卫、无页内动作断言。

## §1 范围、写域与禁改
- 规范轮交付：本 SPEC + 在 r12GrowthPriority.contract.test.ts 追加 R48 源码守卫与真实导入行为测试（只写规范与契约，不写实现）。
- 实施轮写域：pages/_shared/r12GrowthPriority.ts（仅新增消费层纯函数，不改 computeGrowthPriority）、pages/bag/bag.vue、pages/equipment/equipment.vue、pages/gems/gems.vue、pages/skills/skills.vue。
- 硬禁改：engine／replay／replay_hash／score／settle／loot／tokens／后端；pages/index/index.vue 首页副本；DIM_HINT 五条路由值；computeGrowthPriority 的维度、权重、短板选择结果。
- 令牌三锚点原样保留：--lyp-token-version: lyp-tokens-v1、--lyp-r3-version: lyp-tokens-v3-r3、--lyp-r6-version: lyp-tokens-r6（tokens.css:12-14）。禁新增 --r47-*／--r48-*。
- 禁写真值、本机绝对路径；禁把个人环境细节写进任何交付物。

## §2 队长裁定与根因
- 裁定：自指时做页内可执行动作、不 navigateTo 自身；非自指的跨页导航保持原 hintLabel 与 navigateTo 不变。
- 自指集合（以当前路由 R 与短板 hint 严格相等判定）：bag 页的元素覆盖、机制深度；equipment 页的装备契合。gems／skills 当前不自指，但四页均加同形防护以防未来 hint 变更。
- 根因：hint 是「该维去哪补」的跨页导航数据，被四个页面各自无条件消费；消费层缺少「目标就是本页」的识别，导致导航自环、CTA 文案暗示跳转却原地不动。
- 修复只在消费层：新增纯函数按当前路由解析 self 与 CTA 文案；页面 r12Go 在 self 时执行页内动作并提前返回，否则原样 navigateTo。

## §3 消费层纯函数契约（实施轮必须实现）
在 pages/_shared/r12GrowthPriority.ts 新增并导出：
  export interface R12CtaView { self: boolean; label: string }
  export function resolveR12Cta(priority: GrowthPriority | null, selfRoute: string): R12CtaView
行为（确定性、纯函数、无副作用、不 import store/game/api）：
- self 判定唯一口径：priority 为 null 时 self=false；否则 self = priority.hint === selfRoute（严格相等，与维度无关）。
- label：priority 为 null 时为空字符串；self=true 时为「在本页强化」+ priority.label（例：在本页强化元素覆盖）；self=false 时逐字返回 priority.hintLabel（例：看反应搭配、去配装备、去点亮专精）。
- 不改 hint／hintLabel／weightPct／gain／basis 等 GrowthPriority 任何字段；不写配置、不触碰数值算法。
判定表（契约按此取值）：
  元素覆盖短板 hint=/pages/bag/bag，selfRoute=/pages/bag/bag → self=true，label=在本页强化元素覆盖
  机制深度短板 hint=/pages/bag/bag，selfRoute=/pages/bag/bag → self=true，label=在本页强化机制深度
  装备契合短板 hint=/pages/equipment/equipment，selfRoute=/pages/equipment/equipment → self=true，label=在本页强化装备契合
  反应链短板 hint=/pages/reactions/reactions，selfRoute=/pages/bag/bag → self=false，label=看反应搭配（原 hintLabel 逐字）
  反应链短板 hint=/pages/reactions/reactions，selfRoute=/pages/reactions/reactions → self=true，label=在本页强化反应链（防御性一致）
  null，selfRoute 任意 → self=false，label=空字符串

## §4 四页接入契约（实施轮必须实现）
统一模式（四页一致，便于契约守卫与未来维护）：
- 从 @/pages/_shared/r12GrowthPriority 同时 import computeGrowthPriority 与 resolveR12Cta。
- 声明本页自身路由常量（如 selfRoute = /pages/bag/bag，逐字取 pages.json 的 path）。
- 新增 r12Cta = computed(() => resolveR12Cta(r12Priority.value, selfRoute))。
- CTA 模板由 「r12Priority.hintLabel →」 改为 「r12Cta.label →」，使自指时显示页内指引口径、非自指时仍显示原 hintLabel 逐字。
- r12Go 先取 p = r12Priority.value，空则 return；若 r12Cta.value.self 则执行本页页内动作并 return；否则原样 uni.navigateTo（url: p.hint）。navigateTo 调用必须保留在非自指路径。
各页页内动作（self 时执行，不做任何跳转）：
- bag：按短板切到对应页签并定位内容——机制深度 → bagTab 切 more，元素覆盖 → bagTab 切 skills；r12Go 内出现 bagTab.value 赋值，可滚动/高亮对应内容。
- equipment：定位装备列表——清空部位筛选 r3Slot.value 并滚动到列表，或调用页内 r12FocusInPage；r12Go 内出现可识别的页内定位动作，不含 navigateTo 自身。
- gems／skills：当前不自指，仍加自身路由常量与 self 分支；self 时做页内定位（滚动/高亮对应卡）并 return，不 navigateTo。

## §5 红线与不回退
- computeGrowthPriority 仍返回同样短板、hint、hintLabel、weightPct、gain、basis；DIM_HINT 五条路由逐字不变（尤其元素覆盖与机制深度仍指向 bag、装备契合仍指向 equipment）。
- 非自指场景：CTA 文案仍为原 hintLabel 逐字，r12Go 仍 uni.navigateTo（url: p.hint），不得改成页内动作。
- 首页 index.vue 副本不动：不引入 resolveR12Cta，DIM_HINT 与 go() 原样。
- 不回退 R12 原四页 AI 卡四判据（badge/text/basis/cta）与 R6/R7/R10 标记、幂等加载；令牌三锚点原样。

## §6 契约守卫清单与 RED 门禁
在 r12GrowthPriority.contract.test.ts 的 R12 describe 之后追加 R48 describe，全部为真实导入／真实读源码，无内嵌参考实现：
- 行为测试：import 真实 helper；用等权五维 rating 构造元素覆盖、机制深度、装备契合、反应链四种短板；断言 computeGrowthPriority 的 hint 仍是自指路由（记录缺口来源），断言 resolveR12Cta 的 self／label 符合 §3 判定表，断言非自指 label 等于原 hintLabel 逐字，断言 null 空态安全。
- 四页源码守卫：逐页读 .vue 源码，断言 import resolveR12Cta、声明自身路由常量、r12Cta = computed、CTA 用 r12Cta.label 且不再绑定 r12Priority.hintLabel、r12Go 内引用 r12Cta.value.self 且 self 分支提前 return、非自指路径仍保留 uni.navigateTo。
- 页内动作守卫：bag 的 r12Go 出现 bagTab.value 赋值；equipment 的 r12Go 出现页内定位动作（r3Slot.value／r12FocusInPage／scrollIntoView 之一）。
- 不回退守卫：helper 仍含 DIM_HINT 三条自指路由与 computeGrowthPriority 导出；tokens.css 无 --r47-*／--r48-*；首页 index.vue 不含 resolveR12Cta。
RED 门禁（规范轮必须实测并记录逐字失败行）：
  pnpm exec vitest run src/styles/r12GrowthPriority.contract.test.ts --config vitest.config.ts --reporter=dot
  期望 exit=1，失败来自 R48 新增断言（resolveR12Cta 缺失／页内动作缺失／CTA 仍绑 hintLabel），不得出现 PARSE_ERROR 或配置错误导致的假 RED。

## §7 分工与诚实边界
- 规范轮（本轮）：只交付本 SPEC 与契约守卫，跑真实 RED 并回报逐字失败行。
- 实施轮：按 §3／§4 改 helper 与四页，使 R48 契约 GREEN，同时保持 §5 不回退；前端 type-check 与 build:h5 必须通过。
- 真浏览器复查轮：三视口实测 bag 元素覆盖／机制深度、equipment 装备契合点 CTA 停留本页且页内动作生效，非自指场景仍正常跨页导航。
- 终审轮：独立复跑契约／全量／type-check／build、核对截图与非自指文案、核对该死循环三处均闭环。
- 诚实边界：gems／skills 当前不自指，其防护属防御性一致而非已复现缺陷；首页副本未改属裁定而非遗漏；本轮不触碰任何结算／存档／令牌真值。