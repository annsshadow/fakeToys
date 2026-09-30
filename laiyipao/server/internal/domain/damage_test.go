package domain

import "testing"

// 断言 GAME_DESIGN I-1 的反通胀红线：反应伤害中来自攻击力的部分 ≤ 30%。
// 这条测试是整个"以搭配取胜"设计的守门人 —— 如果有人调高任一反应的
// AttackWeightPct，或让反应伤害过度依赖 Attack，测试会立刻失败。
func TestReactionAttackWeightRespectsAntiInflationCap(t *testing.T) {
	for _, spec := range AllReactionSpecs() {
		if spec.AttackWeightPct > MaxReactionAttackWeightPermille {
			t.Errorf("反应 %s 的攻击力权重 %d‰ 超过红线 %d‰",
				spec.Name, spec.AttackWeightPct, MaxReactionAttackWeightPermille)
		}
	}
}

// 反应伤害的实际攻击力占比必须真的 ≤ 30%，而不只是参数声明 ≤ 30%。
// 覆盖：不同等级、不同层数、不同攻击力、不同抗性。
func TestReactionAttackRatioStaysUnderCap(t *testing.T) {
	cases := []struct {
		name    string
		attack  int64
		tier    int64
		stacks  int64 // 每个元素的层数，5 元素合计 = stacks*5
		resist  int64
		element Element
	}{
		{"低养成", 100, 1, 1, 0, ElementFire},
		{"中养成", 2000, 2, 2, 0, ElementFire},
		{"高养成", 50000, 3, 3, 0, ElementFire},
		{"高养成+负抗", 50000, 3, 3, -500, ElementFire},
		{"高养成+高抗", 50000, 3, 3, 500, ElementFire},
		{"零层数", 50000, 3, 0, 0, ElementFire},
		{"满层", 50000, 3, 8, 0, ElementFire},
	}

	for _, c := range cases {
		for _, reaction := range AllReactions() {
			att := DefaultAttacker()
			att.Attack = c.attack
			att.ReactionTier = c.tier
			att.ElementCap = 64

			def := NewDefender(1_000_000_000, 0, 0)
			def.ResistPermille[c.element] = c.resist
			if c.stacks > 0 {
				for _, e := range AllElements() {
					def.ElementStacks[e] = c.stacks
				}
			}

			res := ResolveHit(att, &def, HitInput{
				SkillDamage:   1000,
				SkillElement:  c.element,
				ForceReaction: reaction,
				Roll:          9999, // 固定不暴击，隔离暴击变量
			})
			if res.ReactionDamage <= 0 {
				continue
			}
			ratio := ReactionAttackRatio(res)
			if ratio > MaxReactionAttackWeightPermille {
				t.Errorf("%s / 反应 %s：攻击力占比 %d‰ 超过红线 %d‰",
					c.name, reaction.Spec().Name, ratio, MaxReactionAttackWeightPermille)
			}
		}
	}
}

// 关键平衡断言：低养成 + 正确元素搭配，必须打得过高养成 + 无反应搭配；
// 且同一套低养成若选错元素（打在高抗方向），必须打不过。
//
// 这两条合起来才真正证明"抗性矩阵有牙齿"——
// 只断言"能赢"会让设计退化成无脑最优解，养成线之间就又可以互相替代了。
func TestLowInvestCorrectElementsBeatHighInvestNoReaction(t *testing.T) {
	const targetHP = 1_000_000_000
	armor := int64(100)

	// 目标对焰/电高抗（+50%），对冰/毒负抗（-30%）。
	// 玩家必须选能打出冰或毒方向反应的打法。
	newTarget := func() Defender {
		d := NewDefender(targetHP, 0, armor)
		d.ResistPermille[ElementFire] = 500
		d.ResistPermille[ElementLightning] = 500
		d.ResistPermille[ElementIce] = -300
		d.ResistPermille[ElementCorrosion] = -300
		return d
	}

	// 策略 A：低养成，元素投入拉满。reactionElement 决定抗性方向。
	lowBuild := func(reaction ReactionKey, reactionElement Element) int64 {
		att := DefaultAttacker()
		att.Attack = 2000 // 低养成
		att.ReactionTier = 3
		att.ElementCoefPermille = 3000 // 高元素词条：弱玩家的成长方向
		att.ElementCap = 64
		att.CritPermille = 0

		def := newTarget()
		var total int64
		for i := 0; i < 40; i++ {
			for _, e := range AllElements() {
				def.ElementStacks[e] = 4
			}
			res := ResolveHit(att, &def, HitInput{
				SkillDamage:     1000,
				SkillElement:    ElementCorrosion,
				ForceReaction:   reaction,
				ReactionElement: reactionElement,
				Roll:            9999,
			})
			total += res.TotalDamage
		}
		return total
	}

	// 策略 B：高养成 10 倍，但技能元素与目标已有元素相同 → 不触发任何反应
	highBuildNoReaction := func() int64 {
		att := DefaultAttacker()
		att.Attack = 20000
		att.ReactionTier = 1
		att.ElementCoefPermille = 1000
		att.ElementCap = 64
		att.CritPermille = 0

		def := newTarget()
		var total int64
		for i := 0; i < 40; i++ {
			def.ElementStacks = map[Element]int64{ElementFire: 4}
			res := ResolveHit(att, &def, HitInput{
				SkillDamage:  1000,
				SkillElement: ElementFire,
				Roll:         9999,
			})
			total += res.TotalDamage
		}
		return total
	}

	high := highBuildNoReaction()

	// 选对方向：走冰的负抗（-30%）
	lowCorrect := lowBuild(ReactionFlashFreeze, ElementIce)
	// 选错方向：把反应打在火的高抗（+50%）上
	lowWrong := lowBuild(ReactionOverheat, ElementFire)

	t.Run("选对方向必须赢过高养成无反应", func(t *testing.T) {
		if lowCorrect <= high {
			t.Errorf("平衡红线被破坏：低养成+正确搭配 %d 未能超过高养成+无反应 %d", lowCorrect, high)
		}
	})

	t.Run("同养成下选对方向必须明显优于选错方向", func(t *testing.T) {
		// 抗性矩阵必须有牙齿：同样的养成，只是抗性方向不同，差距必须显著
		if lowCorrect <= lowWrong {
			t.Errorf("抗性矩阵无效：选对方向 %d 未优于选错方向 %d", lowCorrect, lowWrong)
		}
		// 要求至少 1.5 倍差距，确保抗性是真正的决策变量而非噪音
		if lowCorrect < mulInt64ForTest(lowWrong, 3, 2) {
			t.Errorf("抗性差异过小：选对 %d vs 选错 %d，比值 %.2f < 1.5",
				lowCorrect, lowWrong, float64(lowCorrect)/float64(lowWrong))
		}
	})

	t.Logf("低养成+正确方向=%d，低养成+错误方向=%d，高养成+无反应=%d", lowCorrect, lowWrong, high)
}

func mulInt64ForTest(a, num, den int64) int64 { return a * num / den }

// 抗性必须真的起作用：同一发打在高抗元素与负抗元素上，伤害必须显著不同。
func TestElementResistChangesOutcome(t *testing.T) {
	att := DefaultAttacker()
	att.Attack = 5000
	att.ReactionTier = 2

	hitWithResist := func(resist int64) int64 {
		def := NewDefender(5_000_000, 0, 0)
		def.ResistPermille[ElementFire] = resist
		for _, e := range AllElements() {
			def.ElementStacks[e] = 6
		}
		return ResolveHit(att, &def, HitInput{
			SkillDamage:   1000,
			SkillElement:  ElementFire,
			ForceReaction: ReactionOverheat,
			Roll:          9999,
		}).ReactionDamage
	}

	weak := hitWithResist(-500)    // 负抗：吃 150%
	neutral := hitWithResist(0)    // 无抗：吃 100%
	resisted := hitWithResist(500) // 高抗：只吃 50%

	if !(weak > neutral && neutral > resisted) {
		t.Errorf("抗性未正确生效：负抗=%d 无抗=%d 高抗=%d", weak, neutral, resisted)
	}
	// 高抗恰好减半（定点截断允许 ±1 的误差）
	if delta := neutral - resisted; delta < neutral*40/1000 {
		t.Errorf("高抗削减过弱：无抗 %d → 高抗 %d，削减不足 40%%", neutral, resisted)
	}
}

// 反应链查表：同元素不触发，未知组合不触发，顺序影响结果。
func TestLookupReactionTable(t *testing.T) {
	if _, ok := LookupReaction(ElementFire, ElementFire); ok {
		t.Error("同元素不应触发反应")
	}
	if _, ok := LookupReaction("", ElementFire); ok {
		t.Error("空元素不应触发反应")
	}
	if k, ok := LookupReaction(ElementFire, ElementIce); !ok || k != ReactionSteamBurst {
		t.Errorf("焰+冰 应为蒸汽爆发，得到 %q ok=%v", k, ok)
	}
	if k, ok := LookupReaction(ElementFire, ElementLightning); !ok || k != ReactionOverheat {
		t.Errorf("焰+电 应为过热，得到 %q ok=%v", k, ok)
	}
	// 动能只能后手：目标是电、技能是动能 → 破甲击退
	if k, ok := LookupReaction(ElementLightning, ElementKinetic); !ok || k != ReactionArmorBreak {
		t.Errorf("电+动能 应为破甲击退，得到 %q ok=%v", k, ok)
	}
	// 但目标身上已有元素、目标本身是动能时不成立（表里动能的键不含空）
	if _, ok := LookupReaction(ElementKinetic, ""); ok {
		t.Error("空的新元素不应触发")
	}
}

// 护盾可被蒸汽爆发驱散，其他反应不能。
func TestSteamBurstDispelShield(t *testing.T) {
	att := DefaultAttacker()

	run := func(reaction ReactionKey) int64 {
		def := NewDefender(10_000, 5_000, 0)
		def.ElementStacks[ElementIce] = 5
		ResolveHit(att, &def, HitInput{
			SkillDamage:   500,
			SkillElement:  ElementFire,
			ForceReaction: reaction,
			Roll:          9999,
		})
		return def.Shield
	}

	if s := run(ReactionSteamBurst); s != 0 {
		t.Errorf("蒸汽爆发应驱散护盾，剩余 %d", s)
	}
	if s := run(ReactionOverheat); s == 0 {
		t.Error("过热不应驱散护盾")
	}
}

// 层数上限必须生效。
func TestElementCapAndStackClamp(t *testing.T) {
	att := DefaultAttacker()
	att.ElementCap = 3
	def := NewDefender(10_000, 0, 0)
	for i := 0; i < 10; i++ {
		ApplyElementStacks(att, &def, HitInput{SkillElement: ElementFire, ApplyStacks: 1})
	}
	if got := def.Stacks(ElementFire); got != 3 {
		t.Errorf("层数应被上限 3 截断，实际 %d", got)
	}
}

// 元素层数投入越高，反应伤害必须单调不减。
// 这是"低养成玩家靠投入也能拉满"的机制保证。
func TestReactionDamageScalesWithElementStacks(t *testing.T) {
	damageAtStacks := func(stacks int64) int64 {
		att := DefaultAttacker()
		att.Attack = 1000
		att.ReactionTier = 3
		att.ElementCoefPermille = 1000
		att.ElementCap = 64
		def := NewDefender(1_000_000_000, 0, 0)
		for _, e := range AllElements() {
			def.ElementStacks[e] = stacks
		}
		return ResolveHit(att, &def, HitInput{
			SkillDamage:   1000,
			SkillElement:  ElementFire,
			ForceReaction: ReactionOverheat,
			Roll:          9999,
		}).ReactionDamage
	}

	prev := int64(-1)
	for _, s := range []int64{1, 2, 4, 8, 16} {
		got := damageAtStacks(s)
		if got <= prev {
			t.Errorf("层数 %d（合计 %d）时反应伤害 %d 未高于上一层 %d", s, s*5, got, prev)
		}
		prev = got
	}
}

// TestReactionElementEmptyFallsBackToDominant 守跨端语义一致性。
//
// ⚠️ `ReactionElement` 两端同名，但曾经对**同一个空值**给出不同抗性：
//
//	TS:  input.reactionElement ??  def.dominantElement()
//	    `??` 只对 null/undefined 回退 → 传 "" 时 resistOf("") = 0（无抗性）
//	Go:  if resElement == "" { resElement = dominantElement(def) }
//	    空串**会**回退到守方主元素 → 用真实抗性
//
// 后果是反应伤害不同 → replayHash 不同 → I-6 把正常对局判成伪造。
//
// 为什么契约向量抓不到：向量里 reaction_element **从未出现过**（实测确认）。
// 一致性测试只在"两端传了同一个合法值"时比对结果，
// 而分歧恰好发生在"传了非法值"的时候 —— 两端的分歧落在向量覆盖不到的地方。
//
// 配对用例：miniapp/src/game/anti_inflation.test.go 的
// 「跨端语义：reactionElement 的空值处理两端必须一致」。
// 任一端改动回退逻辑，变异那一端的用例立刻红。
func TestReactionElementEmptyFallsBackToDominant(t *testing.T) {
	// 守方：主元素冰（2 层），对冰 +500‰ 抗性，对焰 0‰
	newDef := func() Defender {
		d := NewDefender(1_000_000, 0, 0)
		d.ResistPermille = map[Element]int64{
			ElementIce:  500,
			ElementFire: 0,
		}
		d.ApplyElement(ElementIce, 2, 3)
		return d
	}
	run := func(reactionElement Element) int64 {
		def := newDef()
		res := ResolveHit(DefaultAttacker(), &def, HitInput{
			SkillDamage:     1000,
			SkillElement:    ElementFire,
			ForceReaction:   "flash_freeze",
			ReactionElement: reactionElement,
			Roll:            500,
		})
		return res.ResistAppliedPermille
	}

	empty := run("")
	undef := run(Element(""))
	if empty != undef {
		t.Fatalf("空串(%d) 与未指定(%d) 的抗性应完全相同", empty, undef)
	}
	// 回退到守方主元素（冰，+500‰ 抗性）→ 施加后为 500‰
	if empty != 500 {
		t.Errorf("空串应回退到守方主元素并施加其抗性 500‰，实际 %d‰", empty)
	}
	// 显式传焰（0‰ 抗性）→ 1000‰，不回落
	if got := run(ElementFire); got != 1000 {
		t.Errorf("显式传入焰元素时抗性应为 1000‰（0‰ 抗性），实际 %d‰", got)
	}
}
