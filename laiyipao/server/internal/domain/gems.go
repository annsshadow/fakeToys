package domain

// 宝石实例：词条池、品质权重与 roll。
//
// # 数值从哪来（第 152 轮标定）
//
// 初版（150 轮）的词条区间是「参照 defaultAttacker 反推」的占位值。
// 标定时先量了三件**可测**的事，全部来自生产代码与实测，不靠手感：
//
//	① 每条属性的**硬上限**与装备/专精已经占了多少
//	   （`service/loadout_attacker.go` 的 MaxLoadout* + equip_budget_test 实测）
//	② 装备**到底提供哪几条**属性
//	③ 元素堆叠上限是**天花板**还是倍率
//
// 结论表（`service/gem_balance_pins_test.go` 把每一条都钉成断言）：
//
//	属性          上限   装备+专精已占    宝石的位置
//	attack        1000   1080（已超 80）  **满装备后价值为 0**
//	armor          750    455（余 295）   填永久缺口，宝石有用
//	skill_damage  同 attack —— `loadout_attacker` 把它也加到 Attack
//	crit           500      0             **唯一来源**
//	element_coef  1000      0             **唯一来源**
//	reaction_mult  800      0             **唯一来源**
//	heat_cap       600      0             **唯一来源**
//	element_cap    base 3 / Max 8        **唯一来源，且是天花板不是倍率**
//
// 三条据此而来的设计决定：
//
//  1. **attack / skill_damage 的区间压到很小**（基准 5–20）。
//     满配后它们被 `clampPermille` 整个吃掉，玩家会以为词条没生效 ——
//     那比数值小更糟。留一点是因为 `user_equipment` 初期为空，
//     早期宝石仍有意义，但不该再把它当主属性宣传。
//
//  2. **装备不给的那四条（crit/element_coef/reaction_mult/heat_cap）给足**。
//     它们的上限 500/1000/800/600 全是空的，宝石是唯一来源；
//     三颗金宝石应能吃到各自上限的 20%–50%（`gem_balance_pins_test` 钉住）。
//
//  3. **element_cap 不随品质放大**（见 `scaleAffixesByQuality`）。
//     它是「堆叠到顶就停止累积」的天花板，×2 只会把总值推过 MaxElementCap=8
//     然后被下游 clamp 静默吃掉 —— 花掉的 roll 换不到任何伤害。
//
// 数值锚点：`DefaultAttacker()` 的 attack 100 / critPermille 50 /
// elementCoefPermille 1000 / elementCap 3。

// SeedGemAffix 是某种宝石的一条候选词条。
//
// 一颗宝石要 roll 出 `affix_count` 条（灰 1 条、金 3 条），
// 所以每种宝石的候选池必须 **≥ 3 条**，否则金宝石 roll 不出来。
type SeedGemAffix struct {
	GemID    int
	Affix    string
	MinValue int
	MaxValue int
}

// SeedGemAffixes 是全部宝石的候选词条池。
//
// 每种宝石 3 条：第 1 条是它自己的主属性，后 2 条是别的属性 ——
// 这样金宝石的 3 条词条不会全是同一种加成，构筑选择才有意义。
//
// 区间是**基准值**（灰宝石那一档），金宝石实际值 = 基准 × `gem_qualities.mult` / 1000。
//
// ⚠️ 每一行的 MaxValue 都是被 `service/gem_balance_pins_test.go` 反推出来的，
// 不是手感：判据是「三颗金宝石满配后占该属性上限的多少」或「占装备剩余预算的多少」。
// 改数值前先读那个文件 —— 它会告诉你哪个上限约束了这一行。
var SeedGemAffixes = []SeedGemAffix{
	// 攻击宝石。**刻意给得很小**：见文件头决定 ①。
	// 判据是「早期预算」—— `user_equipment` 为空时上限 1000 全在，
	// 满配三颗金 = 120（12%）；满配玩家身上则完全无效。
	{GemID: 1, Affix: "attack", MinValue: 5, MaxValue: 20},
	{GemID: 1, Affix: "skill_damage", MinValue: 5, MaxValue: 15},
	{GemID: 1, Affix: "crit", MinValue: 15, MaxValue: 40},

	// 元素宝石：宝石独占属性，可以给足（满配 3×160 = 480 / 上限 1000）
	{GemID: 2, Affix: "element_coef", MinValue: 30, MaxValue: 80},
	{GemID: 2, Affix: "reaction_mult", MinValue: 25, MaxValue: 65},
	{GemID: 2, Affix: "element_cap", MinValue: 1, MaxValue: 1},

	// 暴击宝石：cap 500、base 50。满配 3×80 = 240，恰好卡在上限的一半
	{GemID: 3, Affix: "crit", MinValue: 15, MaxValue: 40},
	{GemID: 3, Affix: "attack", MinValue: 5, MaxValue: 18},
	{GemID: 3, Affix: "element_coef", MinValue: 20, MaxValue: 50},

	// 反应宝石：cap 800，满配 3×130 = 390
	{GemID: 4, Affix: "reaction_mult", MinValue: 25, MaxValue: 65},
	{GemID: 4, Affix: "element_coef", MinValue: 25, MaxValue: 60},
	{GemID: 4, Affix: "element_cap", MinValue: 1, MaxValue: 1},

	// 上限宝石：宝石独占，且是天花板不是倍率（不随品质放大）
	{GemID: 5, Affix: "element_cap", MinValue: 1, MaxValue: 1},
	{GemID: 5, Affix: "element_coef", MinValue: 20, MaxValue: 50},
	{GemID: 5, Affix: "crit", MinValue: 15, MaxValue: 40},

	// 散热宝石：cap 600，满配 3×100 = 300（正好一半）
	{GemID: 6, Affix: "heat_cap", MinValue: 20, MaxValue: 50},
	{GemID: 6, Affix: "attack", MinValue: 5, MaxValue: 18},
	{GemID: 6, Affix: "crit", MinValue: 15, MaxValue: 40},

	// 护甲宝石：填那个 295 的**永久**缺口。满配 3×48 = 144，正好余量的一半
	{GemID: 7, Affix: "armor", MinValue: 8, MaxValue: 24},
	{GemID: 7, Affix: "element_cap", MinValue: 1, MaxValue: 1},
	{GemID: 7, Affix: "heat_cap", MinValue: 15, MaxValue: 40},

	// 技能宝石：skill_damage 走 Attack 那条路，同样刻意压小
	{GemID: 8, Affix: "skill_damage", MinValue: 5, MaxValue: 15},
	{GemID: 8, Affix: "attack", MinValue: 5, MaxValue: 18},
	// crit 取 15–40 而不是更高：守卫取的是**全表里这一条词条的最大值**，
	// 这里是全表唯一超过 40 的 crit，会把「三颗金宝石满配」顶到 300
	// （上限 500 的一半是 250）。
	{GemID: 8, Affix: "crit", MinValue: 15, MaxValue: 40},
}

// UnscaledGemAffixes 是**不随品质放大**的词条。
//
// 只有 `element_cap`：它是「堆叠到顶就停止累积」的天花板
// （见 `MaxElementCap` 的注释），不是倍率。base 3、上限 8，
// 乘 2 只会把总值推过 8 然后被下游 clamp 静默吃掉 ——
// 玩家看到「这颗宝石给了 +4 元素上限」，实际一点伤害都没多。
//
// 写成集合而不是在 scale 函数里 if：`gem_balance_pins_test` 要遍历它，
// 且以后若再加第二个不放大词条，改这一处即可。
var UnscaledGemAffixes = map[string]bool{
	"element_cap": true,
}

// GemQualityWeight 是抽到各品质的概率（万分比，合计 10000）。
//
// 取值理由：灰 40% 让人敢买（买十次至少有四次不亏），
// 金 3% 让「出金」是个记得住的时刻。再低会变成纯噪音，
// 再高会让金色词条失去稀缺感。
var GemQualityWeight = map[string]int{
	"gray":   4000,
	"blue":   3000,
	"purple": 1800,
	"red":    900,
	"gold":   300,
}

// GemShopPrice 是宝石实例的售价（钻石）。
//
// 参照同表：钥匙串 30 钻石、专精点重置 80 钻石、首充 300 钻石。
// 定 60 —— 在「日常补给」与「构筑向道具」之间。
//
// 量级对照（按金币产出 `CoinBase 200 + 每星 150 + 每击杀 20`，
// 三星十杀约 850/局；`coin_big` 是 50 钻石换 120,000 金币，即 1 局 ≈ 0.35 钻石）：
//
//	60 钻石 ≈ 170 局   ← 宝石本体，一颗能带三条词条
//	30,000 金币 ≈ 35 局 ← 洗练石（同表最便宜的续战令是 5,000 ≈ 6 局）
//
// 洗练比买便宜不是问题：宝石 60 钻石，洗一次要再赚 30 局，
// 重掷不起无限刷；而「买一颗灰宝石反复洗」的担心在本结构下不成立 ——
// 品质由买的那一次定死，洗练**不涨品质**。
const GemShopPrice = 60

// GemWashPrice 是洗练石的售价（金币）。
//
// # 这个数字改过一次（120,000 → 30,000），理由记在这里
//
// 初版定 120,000，是怕玩家「买一颗灰宝石反复洗」。
// 真机跑一遍才发现那个理由站不住：
//
//	120,000 金币 ≈ 140 局；而同表最便宜的消耗品「续战令」只要 5,000
//
// 两个消耗品差 24 倍，定的人自己都觉得不对劲。
// 现在的 30,000 ≈ 35 局，是续战令的 6 倍（重掷词条比重打一局贵，
// 但同一量级），也高过 20,000 的金币袋（不至于随手就洗）。
const GemWashPrice = 30000

// GemQualityMult 取品质系数（千分比），用于把词条基准区间放大。
//
// 放大而不是每个品质各写一套区间：只有一处标定点，
// 且「金 = 灰的 2 倍」这条关系由内容表自己保证（mult 2000/1000）。
func GemQualityMult(quality string) int64 {
	for _, q := range GemQualities {
		if q.Name == quality {
			return q.Mult
		}
	}
	return 1000
}

// GemQualityAffixCount 取该品质 roll 几条词条。
func GemQualityAffixCount(quality string) int {
	for _, q := range GemQualities {
		if q.Name == quality {
			return q.Count
		}
	}
	return 1
}
