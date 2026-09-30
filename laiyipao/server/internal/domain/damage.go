package domain

// 伤害结算（I-1 的核心）。
//
// 全部使用定点整数（千分比），禁止浮点：既是为了与 TS 侧逐位一致，
// 也是因为 I-6 的回放哈希要求跨设备可复现。
//
// ── 三段式伤害模型（见 GAME_DESIGN I-1）────────────────────────────
//
//	单次命中伤害 = 直接伤害(0.40) + 元素层数伤害 + 反应伤害
//
//	直接伤害   = D × 0.40 × (1-护甲)                       吃养成
//	元素层数伤害 = E × 层数 × 元素系数 / 40                 只吃元素投入
//	反应伤害   = (E × 层数 × k × 阶 × 系数 + 攻击侧) × (1-抗性)
//
//	其中 D = 单发伤害 × (1 + 攻击力‰)，E = ElementPerStackBase（设计常量），
//	k = 反应系数，阶 = 技能阶数，系数 = 元素词条。
//
// ── 反通胀不变量（结构性保证，不是调参碰巧成立）────────────────────
//
//	reactAtk = min(D × k × 阶,  w × reactElem / (1-w))
//
//	由 reactAtk ≤ w·reactElem/(1-w) 可直接推出
//	    reactAtk / (reactElem + reactAtk) ≤ w
//	即"反应伤害中来自面板攻击力的部分恒 ≤ w ≤ 30%"，
//	对任意攻击力、任意层数、任意阶数都成立。测试直接断言该式。

const (
	// 千分比基数
	permille = 1000

	// 直接伤害权重（千分比）
	DirectWeightPermille = 400

	// ElementPerStackBase 单层元素的基础伤害基准（绝对值，不随攻击力缩放）。
	// 这是弱玩家的翻盘点：反应伤害的主体来自它，与养成完全无关。
	ElementPerStackBase int64 = 1200

	// 元素层数持续伤害的除数（该段不吃反应阶数与反应系数，避免与反应段重复计价）
	ElementTickDivisor int64 = 40

	// 抗性绝对值上限（千分比），对应 ±50%
	MaxResistPermille = 500

	// 减伤上限（千分比，75%），避免高护甲导致关卡无法推进
	MaxArmorPermille = 750

	// 反通胀红线：反应伤害中攻击力贡献占比上限（千分比）
	MaxReactionAttackWeightPermille = 300

	// MaxElementCap 元素层数上限。
	//
	// ⚠️ 此前是散落在 computeAttacker 里的字面量 `8`。
	// 宝石的 element_cap 是**后加的**（本轮接线），于是需要在两处
	// 都能读到同一个上界 —— 散落的字面量必然有一处被漏改。
	MaxElementCap = 8
)

// Attacker 是攻方结算所需的养成属性快照。
type Attacker struct {
	// Attack 面板攻击力，已含技能等级、装备、宝石加成
	Attack int64
	// CritPermille 暴击率（千分比，0..1000）
	CritPermille int64
	// CritMultiplierPermille 暴击倍率（千分比，1500 = 150%）
	CritMultiplierPermille int64
	// ReactionMultPermille 反应伤害倍率（千分比，来自专精树 + 装备契合度）
	ReactionMultPermille int64
	// ElementCap 单元素层数上限
	ElementCap int64
	// ReactionTier 反应等级（1..3，来自技能阶数）
	ReactionTier int64
	// ElementCoef 元素层数系数（千分比），吃元素词条
	ElementCoefPermille int64

	// ⚠️ 下面三个字段是本轮补上的**遗漏成长维度**。
	//
	// 它们此前**只被计算、从不被装配**：
	//   - MasteryEffect.HeatCapBonus 在 EvaluateMastery 里累加，
	//     但 computeAttacker 从不读它 → 专精树第 1/2 层的「热量上限」
	//     节点（8 系 × 2 层 × 1 槽 = 16 个节点）是纯装饰
	//   - MasteryEffect.ArmorBonus 同样只进 I-7 评分 → 第 3 层的
	//     「护甲」节点（8 个）是纯装饰
	//   - 专精第 3 层槽 1 的「机制改造」（mechanic，8 个节点）
	//     **在 EvaluateMastery 里连 case 都没有**，完全惰性
	//
	// 为什么不给「技能伤害加成」单开一个乘区，而是折进 Attack？
	// 因为 Attack 是 I-1 反通胀公式里**攻击力侧**的那一项：
	//
	//	反应伤害 = 攻击力侧 + 元素侧，且攻击力侧占比 ≤ 30‰ 红线
	//
	// 单开一个 `SkillDamagePermille` 乘区，就等于给养成开了一扇
	// 绕过这条红线的后门 —— 现有反通胀不变式将不再覆盖它。
	// 折进 Attack 则反通胀测试继续原样守着。
	//
	// HeatCapPermille 热量上限加成（专精 heat_cap + 宝石 gem_heat）。
	HeatCapPermille int64
	// ArmorPermille 玩家护甲（装备 BaseArmor + 专精 armor + 宝石 gem_armor），
	// 减免漏怪伤害。封顶 MaxArmorPermille —— 见 ApplyPlayerArmor。
	ArmorPermille int64
	// MechanicPermille 机制卡强度加成（专精 mechanic）。
	// 作用在**卡面数值**上：取牌时把 MechanicEffect.value
	// 乘以 (1000+本值)/1000。
	MechanicPermille int64
}

// DefaultAttacker 返回新号默认属性。零值不可直接使用，必须走构造函数。
func DefaultAttacker() Attacker {
	return Attacker{
		Attack:                 100,
		CritPermille:           50,
		CritMultiplierPermille: 1500,
		ReactionMultPermille:   1000,
		ElementCap:             3,
		ReactionTier:           1,
		ElementCoefPermille:    1000,
	}
}

// Defender 是守方结算所需的状态快照。
type Defender struct {
	HP             int64
	Shield         int64
	ArmorPermille  int64             // 护甲减伤（千分比）
	ResistPermille map[Element]int64 // 五维元素抗性，范围 -500..+500
	ElementStacks  map[Element]int64 // 身上已附着的元素层数
}

// NewDefender 构造守方状态。
func NewDefender(hp, shield, armorPermille int64) Defender {
	return Defender{
		HP:             hp,
		Shield:         shield,
		ArmorPermille:  armorPermille,
		ResistPermille: make(map[Element]int64, len(AllElements())),
		ElementStacks:  make(map[Element]int64, len(AllElements())),
	}
}

// Resist 返回目标对指定元素的抗性，未配置时视为 0（无抗性）。
func (d Defender) Resist(e Element) int64 {
	if v, ok := d.ResistPermille[e]; ok {
		return clampInt64(v, -MaxResistPermille, MaxResistPermille)
	}
	return 0
}

// Stacks 返回目标身上某元素的层数。
func (d Defender) Stacks(e Element) int64 {
	if v, ok := d.ElementStacks[e]; ok {
		return v
	}
	return 0
}

// 施加元素（受层数上限约束）。返回是否实际增加了层数。
func (d *Defender) ApplyElement(e Element, add, cap int64) bool {
	if e == "" || add <= 0 {
		return false
	}
	if d.ElementStacks == nil {
		d.ElementStacks = make(map[Element]int64, len(AllElements()))
	}
	cur := d.Stacks(e)
	next := cur + add
	if next > cap {
		next = cap
	}
	if next == cur {
		return false
	}
	d.ElementStacks[e] = next
	return true
}

// 清除元素层数（过热结束后调用，I-2 明确"过热期间层数不清除"）。
func (d *Defender) ClearElements() {
	d.ElementStacks = make(map[Element]int64, len(AllElements()))
}

// HitInput 描述一次命中。
type HitInput struct {
	// SkillDamage 单发基准伤害（技能表原始值）
	//
	// ⚠️ 这里**曾经注释成**「技能表原始值 × 等级系数」——
	// 而代码里从来没有等级系数：`user_skills.level` 只被读进 build 快照，
	// **从无写入**，且**没有任何技能升级端点**，所以等级恒为 1。
	//
	// 注释描述一个不存在的公式，比没有注释更糟：
	// 后来人会照着它去实现"等级加成"，或者反过来
	// 以为等级已经在生效、因而去查"为什么升级了没变强"。
	// 两者都是被这份注释误导的。
	//
	// 真要接等级成长，前置条件是**先有升级路径**（消耗资源 → 提升 level），
	// 再定缩放曲线，并保证两端一致（I-6）。已记入 README「未做」。
	SkillDamage int64
	// SkillElement 该技能携带的主元素
	SkillElement Element
	// ApplyStacks 本次施加的元素层数
	ApplyStacks int64
	// ForceReaction 显式指定反应（地形/工事触发时使用），为空则按元素表推导
	ForceReaction ReactionKey
	// ReactionElement 反应归属元素，用于查抗性；为空则取守方已附着元素
	ReactionElement Element
	// Roll 暴击判定用的确定性随机数。
	//
	// ⚠️ 量纲是**千分比**（0..999），与 CritPermille 同量纲。
	// 曾经是万分比（0..9999）而判定阈值 `roll >= 1000 - critPermille`
	// 是千分比，两个量纲混用导致 critPermille=50（标称 5%）实际
	// 产生 90.5% 暴击率，10 倍偏差，且把所有取值压进 90%+ 的区间
	// 使该成长维度的边际收益消失。
	// 客户端 BattleRng.roll() 已同步为 intn(1000)；
	// 由 TestCritRateDistribution（Go 与 TS 各一份）守住。
	Roll int64
}

// HitResult 是一次命中的完整结算结果。
type HitResult struct {
	DirectDamage   int64
	ElementDamage  int64
	ReactionDamage int64
	// ReactionElemPortion 反应伤害中来自元素投入的部分（与养成无关）
	ReactionElemPortion int64
	// ReactionAttackPortion 反应伤害中来自攻击力的部分，用于反通胀不变量断言
	ReactionAttackPortion int64
	TotalDamage           int64
	// Reaction 实际触发的反应，空表示未触发
	Reaction ReactionKey
	// Resisted 抗性修正后的最终系数（千分比），供诊断使用
	ResistAppliedPermille int64
	// Crit 是否暴击
	Crit bool
	// ShieldBroken 护盾是否被打碎
	ShieldBroken bool
	// Killed 目标是否被击杀
	Killed bool
	// DispelShield 是否驱散了护盾
	DispelShield bool
	// StatusDurationMs 附加状态时长
	StatusDurationMs int
}

// ResolveReaction 决定本次命中触发哪条反应链。
// 优先使用显式指定；否则按"守方已附着元素 + 技能主元素"查表。
func ResolveReaction(def *Defender, in HitInput) (ReactionKey, bool) {
	if in.ForceReaction != "" {
		return in.ForceReaction, true
	}
	// 以守方已有层数最多的元素作为"已附着元素"
	var best Element
	var bestStacks int64
	for _, e := range AllElements() {
		if s := def.Stacks(e); s > bestStacks {
			best, bestStacks = e, s
		}
	}
	if bestStacks == 0 {
		return "", false
	}
	return LookupReaction(best, in.SkillElement)
}

// ResolveHit 执行一次完整命中结算。这是全项目唯一伤害入口，
// 客户端引擎与服务端校验都必须走它（或其 TS 等价实现）。
//
// 计算顺序（顺序本身影响结果，必须两端一致）：
//  1. 基准伤害 D = SkillDamage × (1 + Attack/1000 归一后的攻击力倍率)
//     实际实现用 Attack 直接作为乘区，避免除法：D = SkillDamage × (1000 + Attack) / 1000
//  2. 直接伤害 = D × 400/1000 × (1 - 护甲)
//  3. 元素层数伤害 = D × 层数/8 × 200/1000
//  4. 反应伤害 = 攻击力侧 + 元素侧（见下）
//  5. 抗性统一作用于第 4 段（反应伤害）
//  6. 暴击作用于第 1、2 段之和
func ResolveHit(att Attacker, def *Defender, in HitInput) HitResult {
	res := HitResult{ResistAppliedPermille: permille}

	// 1) 基准伤害。Attack 以千分比叠加，避免除法。
	d := mulDiv(in.SkillDamage, permille+att.Attack, permille)
	if d < 0 {
		d = 0
	}

	// 2) 直接伤害：唯一的养成主来源
	direct := mulDiv(d, DirectWeightPermille, permille)
	direct = applyArmor(direct, def.ArmorPermille)

	// 3) 元素层数持续伤害：只吃层数与元素系数，不吃反应阶数/系数。
	//    同样受技能元素的抗性约束 —— 否则元素体系会变成绕过抗性矩阵的后门。
	stacks := totalElementStacks(def)
	var elementDmg int64
	if stacks > 0 && in.SkillElement != "" {
		elementDmg = ElementPerStackBase * stacks
		elementDmg = mulDiv(elementDmg, att.ElementCoefPermille, permille)
		elementDmg = elementDmg / ElementTickDivisor
		elementDmg = mulDiv(elementDmg, permille-def.Resist(in.SkillElement), permille)
		if elementDmg < 0 {
			elementDmg = 0
		}
	}

	// 4) 反应伤害
	reactionKey, hasReaction := ResolveReaction(def, in)
	var reactionDmg, reactionAttackPortion, reactionElemPortion int64
	if hasReaction {
		spec := reactionKey.Spec()
		k := int64(spec.BaseCoef)
		w := int64(spec.AttackWeightPct)
		t := att.ReactionTier

		// 元素侧：绝对伤害基准 × 层数 × 反应系数 × 阶 × 元素系数，与养成完全无关
		reactionElemPortion = ElementPerStackBase * stacks
		reactionElemPortion = mulDiv(reactionElemPortion, k, permille)
		reactionElemPortion = mulDiv(reactionElemPortion, t, 1)
		reactionElemPortion = mulDiv(reactionElemPortion, att.ElementCoefPermille, permille)

		// 攻击力侧：吃养成，但有结构性上限
		//
		// ⚠️ 上限要**除以 ReactionMultPermille**，且乘完倍率之后仍要满足
		// I-1 的不变式「反应伤害中攻击力占比 ≤ w」：
		//
		//   A·M/(E + A·M) ≤ w  ⟺  A·M ≤ wE/(1-w)  ⟺  A ≤ wE/((1-w)·M)
		//
		// 详见 miniapp/src/game/damage.ts 的 reactionAttackCap 注释
		// （两端必须逐位一致，否则 I-6 回放哈希必然失配）。
		//
		// ReactionMultPermille = 1000（无加成）时上限与旧式 wE/(1-w)
		// 逐位相同，所以既有契约向量不受影响。
		attackRaw := mulDiv(d, k, permille)
		attackRaw = mulDiv(attackRaw, t, 1)
		cap := reactionAttackCap(w, reactionElemPortion, att.ReactionMultPermille)
		if attackRaw > cap {
			attackRaw = cap
		}
		reactionAttackPortion = mulDiv(attackRaw, att.ReactionMultPermille, permille)
	}

	// 5) 抗性只作用于反应伤害（I-1 的设计意图：抗性决定"搭配是否正确"，
	//    而不是简单地把所有伤害一起砍掉）
	if hasReaction {
		resElement := in.ReactionElement
		if resElement == "" {
			resElement = dominantElement(def)
		}
		resist := def.Resist(resElement)
		res.Reaction = reactionKey
		res.ResistAppliedPermille = permille - resist
		spec := reactionKey.Spec()
		res.DispelShield = spec.DispelShield
		res.StatusDurationMs = spec.StatusDurationMs
		total := reactionElemPortion + reactionAttackPortion
		reactionDmg = mulDiv(total, permille-resist, permille)
		reactionAttackPortion = mulDiv(reactionAttackPortion, permille-resist, permille)
	}

	// 6) 暴击作用于直接伤害 + 元素层数伤害（不作用于反应伤害，
	//    否则抗性会被暴击绕开，抗性矩阵就失去意义）
	crit := false
	critPart := direct + elementDmg
	if att.CritPermille > 0 && in.Roll >= permille-att.CritPermille {
		crit = true
		critPart = mulDiv(critPart, att.CritMultiplierPermille, permille)
	}
	direct = critPart

	total := direct + reactionDmg

	// 7) 落伤害到守方状态：先扣护盾（可被反应驱散），再扣血
	if total > 0 {
		if res.DispelShield {
			def.Shield = 0
		}
		remaining := total
		if def.Shield > 0 {
			absorbed := minInt64(def.Shield, remaining)
			def.Shield -= absorbed
			remaining -= absorbed
			if def.Shield == 0 {
				res.ShieldBroken = true
			}
		}
		if remaining > 0 {
			def.HP -= remaining
			if def.HP <= 0 {
				def.HP = 0
				res.Killed = true
			}
		}
	}

	res.DirectDamage = direct
	res.ElementDamage = elementDmg
	res.ReactionDamage = reactionDmg
	res.ReactionElemPortion = reactionElemPortion
	res.ReactionAttackPortion = reactionAttackPortion
	res.TotalDamage = total
	res.Crit = crit
	return res
}

// reactionAttackCap 返回"攻击力侧"的结构性上限。
//
// 不变式（I-1 的核心）：反应伤害中攻击力贡献的占比 ≤ w。
//
//	A·M/(E + A·M) ≤ w  ⟺  A·M ≤ wE/(1-w)  ⟺  A ≤ wE/((1-w)·M)
//
// 三个参数：
//
//	w            反应的攻击力权重（千分比，≤ MaxReactionAttackWeightPermille=300）
//	elemPortion  元素侧的绝对伤害（与养成完全无关）
//	multPermille 反应倍率（千分比，1000 = 无加成）
//
// ⚠️ multPermille 必须参与上限计算。少了它，倍率就成了绕过反通胀保证的后门：
// 专精树的 reaction_mult 节点、装备契合度、防线装置「特斯拉栅格」全部只写不读，
// 正是因为读不到地方 —— 而一旦读进来又不管上限，玩家堆一个节点就能让
// 反应伤害里的攻击力占比突破 30%，整套设计的根基被破坏。
//
// multPermille = 1000 时本函数与旧式 wE/(1-w) 逐位相同，
// 所以既有契约向量与平衡数据都不受影响。
//
// 分母 (permille-w) 在 w ≤ 300 时恒为正，不会除零。
func reactionAttackCap(w, elemPortion, multPermille int64) int64 {
	if w >= permille {
		// 防御性分支：设计上不会出现，这里让上限无穷大而不是除零 panic
		return elemPortion * permille
	}
	if multPermille <= 0 {
		// 倍率为 0 等价于"攻击侧完全无效"
		return 0
	}
	base := mulDiv(w, elemPortion, permille-w)
	return mulDiv(base, permille, multPermille)
}

// ApplyElementStacks 在命中结算后施加元素层数。
// 注意顺序：必须先结算伤害/反应，再施加新层数，
// 否则"本次施加的元素"会立刻成为反应链的"已附着元素"，导致自我触发。
func ApplyElementStacks(att Attacker, def *Defender, in HitInput) {
	if in.SkillElement == "" || in.ApplyStacks <= 0 {
		return
	}
	def.ApplyElement(in.SkillElement, in.ApplyStacks, att.ElementCap)
}

// ReactionAttackRatio 返回反应伤害中攻击力贡献的占比（千分比）。
// GAME_DESIGN I-1 红线断言：此值必须 ≤ MaxReactionAttackWeightPermille。
func ReactionAttackRatio(r HitResult) int64 {
	if r.ReactionDamage <= 0 {
		return 0
	}
	return mulDiv(r.ReactionAttackPortion, permille, r.ReactionDamage)
}

// --- 内部工具 ---

// applyArmor 按千分比护甲削减伤害。
//
// ⚠️ 不能写成 mulDiv(dmg, permille-armorPermille, permille)：
// 那是 dmg * kept，dmg 超过 MaxInt64/kept 时 int64 静默回绕成**负数**。
// 实测 applyArmor(math.MaxInt64/4, 250) 返回 -4611686018427387904 ——
// 护甲越高伤害越负，比没有护甲更糟。
//
// 当前内容表的伤害量级（敌人血量上限 4 万）远达不到溢出，
// 但 damage.go 里的值来自 int64 报文的乘积，且客户端的 resolveHit
// 走 bigint（无溢出）—— 两端在同一输入上会给出不同结果。
//
// 下面的除法分解与 dmg*kept/permille **精确等价**（都是向零截断的整数除法），
// 但中间值不溢出：q 一定 <= MaxInt64/permille，所以 q*kept <= MaxInt64。
func applyArmor(dmg, armorPermille int64) int64 {
	if dmg <= 0 {
		// 负伤害原样返回：负数在结算里表示"反伤/异常"，不该被护甲改写。
		return dmg
	}
	if armorPermille <= 0 {
		return dmg
	}
	if armorPermille > MaxArmorPermille {
		armorPermille = MaxArmorPermille
	}
	kept := permille - armorPermille
	// kept = 1000 - armorPermille，而 armorPermille 已被夹进 (0, 750]，
	// 所以 kept ∈ [250, 999] 恒正 —— 这里不需要（也曾经多写过）
	// 一层 `if kept <= 0` 兜底，那是封顶引入之前的旧写法残留。
	// dmg = q*permille + r  =>  dmg*kept/permille = q*kept + r*kept/permille
	q := dmg / permille
	r := dmg % permille
	return q*kept + r*kept/permille
}

// totalElementStacks 统计守方身上的元素层数总和。
func totalElementStacks(def *Defender) int64 {
	if def == nil || def.ElementStacks == nil {
		return 0
	}
	var total int64
	for _, e := range AllElements() {
		total += def.Stacks(e)
	}
	return total
}

// dominantElement 返回层数最多的元素，无层数时返回空。
func dominantElement(def *Defender) Element {
	var best Element
	var bestStacks int64
	for _, e := range AllElements() {
		if s := def.Stacks(e); s > bestStacks {
			best, bestStacks = e, s
		}
	}
	return best
}

// mulDiv 是 (a * b) / c 的整数版本，内部用 int64 防溢出。
// 全部公式都经过它，保证两端除法语义一致（都是向零截断）。
func mulDiv(a, b, c int64) int64 {
	if c == 0 {
		return 0
	}
	return (a * b) / c
}

func minInt64(a, b int64) int64 {
	if a < b {
		return a
	}
	return b
}

func clampInt64(v, lo, hi int64) int64 {
	if v < lo {
		return lo
	}
	if v > hi {
		return hi
	}
	return v
}
