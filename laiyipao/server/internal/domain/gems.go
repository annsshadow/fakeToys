package domain

// 宝石实例：词条池、品质权重与 roll。
//
// # 为什么这些数值集中在这里
//
// 它们是**尚未标定**的平衡数值，且散落在三处就会漂移
// （词条范围在这里、品质权重在这里、商城定价在 seeder 里）。
// 所以：本文件只放「数值 + 为什么这么取」，
// 定价单独放在 seeder 并注明它引用的是本文件的量级。
//
// # 取值锚点（domain.DefaultAttacker，与 miniapp 的 defaultAttacker 同源）
//
//	attack 100 · critPermille 50 · reactionMultPermille 1000
//	elementCoefPermille 1000 · elementCap 3 · heatCap 0 · armor 0
//
// ��「一颗金宝石 ≈ 三阶装备的加成量级」反推：
// equipment 三阶 base_bonus_pct 是 250‰，对应到上面这些绝对值量级，
// 单词条落在 base 的 5%–12% 比较合理 —— 少到不能替代装备，多到值得刷。
//
// ⚠️ **这些数需要按真实关卡分布标定。** 改它们只需动本文件，
// `TestGemAffixPoolIsSane` 会守住形状不变量（见该测试）。

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
// 每种宝石 3 条：第 1 条是它自己的主属性（范围最大），
// 后 2 条是别的属性（范围较小）—— 这样金宝石的 3 条词条不会全是同一种加��，
// 构筑选择才有意义。
var SeedGemAffixes = []SeedGemAffix{
	// 攻击宝石：主攻 attack
	{GemID: 1, Affix: "attack", MinValue: 20, MaxValue: 60},
	{GemID: 1, Affix: "skill_damage", MinValue: 10, MaxValue: 30},
	{GemID: 1, Affix: "crit", MinValue: 10, MaxValue: 25},

	// 元素宝石：主攻 element_coef
	{GemID: 2, Affix: "element_coef", MinValue: 40, MaxValue: 120},
	{GemID: 2, Affix: "reaction_mult", MinValue: 20, MaxValue: 50},
	{GemID: 2, Affix: "element_cap", MinValue: 1, MaxValue: 1},

	// 暴击宝石：主攻 crit
	{GemID: 3, Affix: "crit", MinValue: 30, MaxValue: 80},
	{GemID: 3, Affix: "attack", MinValue: 10, MaxValue: 30},
	{GemID: 3, Affix: "element_coef", MinValue: 20, MaxValue: 50},

	// 反应宝石：主攻 reaction_mult
	{GemID: 4, Affix: "reaction_mult", MinValue: 30, MaxValue: 90},
	{GemID: 4, Affix: "element_coef", MinValue: 30, MaxValue: 80},
	{GemID: 4, Affix: "element_cap", MinValue: 1, MaxValue: 1},

	// 上限宝石：主攻 element_cap（范围刻意压窄 —— 它是「档位数」，
	// 一次给太多会直接顶穿反应权重上限）
	{GemID: 5, Affix: "element_cap", MinValue: 1, MaxValue: 2},
	{GemID: 5, Affix: "element_coef", MinValue: 20, MaxValue: 60},
	{GemID: 5, Affix: "crit", MinValue: 10, MaxValue: 30},

	// 散热宝石：主攻 heat_cap
	{GemID: 6, Affix: "heat_cap", MinValue: 100, MaxValue: 300},
	{GemID: 6, Affix: "attack", MinValue: 10, MaxValue: 25},
	{GemID: 6, Affix: "crit", MinValue: 10, MaxValue: 25},

	// 护甲宝石：主攻 armor（默认 0，是护甲的主要来源）
	{GemID: 7, Affix: "armor", MinValue: 20, MaxValue: 60},
	{GemID: 7, Affix: "element_cap", MinValue: 1, MaxValue: 1},
	{GemID: 7, Affix: "heat_cap", MinValue: 50, MaxValue: 150},

	// 技能宝石：主攻 skill_damage
	{GemID: 8, Affix: "skill_damage", MinValue: 10, MaxValue: 30},
	{GemID: 8, Affix: "attack", MinValue: 10, MaxValue: 25},
	{GemID: 8, Affix: "crit", MinValue: 15, MaxValue: 40},
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
const GemShopPrice = 60

// GemWashPrice 是洗练石的售价（金币）。
//
// # 这个数字改过一次（120,000 → 30,000），理由记在这里
//
// 初版定 120,000，是怕玩家「买一颗灰宝石反复洗」。
// 真机跑一遍才发现那个理由站不住：
//
//	战斗金币产出（domain/battle.go）CoinBase 200 + 每星 150 + 每击杀 20
//	  → 三星十杀约 850/局
//	120,000 金币 ≈ 140 局；而同表最便宜的消耗品「续战令」只要 5,000 金币
//
// 两个消耗品差 24 倍，定的人自己都觉得不对劲。
// 「反复洗」也不是真风险：宝石 60 钻石 ≈ 一个金币箱的量，
// 洗一次就要再赚一个金币箱，刷不出金币色。
//
// 现在的 30,000：约 35 局，是续战令的 6 倍（重掷词条比重打一局贵，
// 但同一量级），也高过 20,000 的金币袋（不至于随手就洗）。
//
// ⚠️ 与 GemShopPrice 一样，**未经按真实关卡分布标定**。
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
