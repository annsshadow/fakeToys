package domain

// 针对「技能卡是哑弹」这个 bug 的守卫。
//
// # 这个 bug 的形状
//
// `applyCard` 的分支判据是 `c.Kind == CardKindSkill && c.hasSkillID`
// （对应 TS 的 `card.skillId !== undefined`）。
// `hasSkillID` 是**非导出**字段，除了 JSON 反序列化，
// 只有 `rollSkillCards` 会置它。
//
// 我第一版忘了置，于是**每一张技能卡都是哑弹**：
//
//   - 卡被 `Deck.Take` 取走      ✅ 动作是真的
//   - `emit(card_taken)`         ✅ 事件是真的
//   - `record(card, ...)`        ✅ replay 记录是真的
//   - `applyCard` 进技能分支      ❌ **哑的**
//
// 症状之所以能一路绿灯穿过 B~F 两块的 25 个跨端场景，靠的是三个巧合：
//
//  1. `card` 的 replay 记录正常 → 事件流比对看不出问题
//  2. `kills` / `waves` / `terrain_used` 正常 → 只有伤害低一档
//  3. B~F 的 6 个卡牌场景用的全是 `-3` 编码 =「弃一张 + 取一张」，
//     取到的是**弃牌之后**的第 0 张（属性卡）——
//     **没有任何一个场景取过纯技能卡**
//
// 而我那批 `TestApplySkillCard*` 单测全都**手工构造**了
// `Card{..., hasSkillID: true}`，于是它们验的是
// 「`applySkillCard` 对不对」，而不是「deck 里的卡能不能走到那里」。
//
// # 本文件的三条守卫分别盯三个环节
//
//	卡牌产出  → TestRolledSkillCardsCarryTheSkillIDFlag
//	分发判据  → TestApplyCardDispatchesOnRolledCards
//	全路径    → TestTakingSkillCardFromDeckUpgradesSkill
//
// 第三条最关键：它从 `enterCardSelect` 出发，只用**玩家能做的动作**
// （`TakeCard`），中间不手工构造任何 `Card`。

import "testing"

// TestRolledSkillCardsCarryTheSkillIDFlag 钉住「抽出来的技能卡带着 skill_id」。
//
// ⚠️ 断言用 `HasSkillID()`（导出的读方法）而不是直接摸非导出字段：
//
//	测试与被测代码在同一个包里，直接摸字段会让
//	「字段被改名」变成编译错误而不是断言失败，
//	而读方法才是对外承诺。
func TestRolledSkillCardsCarryTheSkillIDFlag(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)

	var skillCard *Card
	for i := range e.Deck.Hand {
		if e.Deck.Hand[i].Kind == CardKindSkill {
			skillCard = &e.Deck.Hand[i]
		}
	}
	if skillCard == nil {
		t.Fatal("手牌里没有技能卡 —— rollWaveCards 的固定构成被改了")
	}
	if !skillCard.HasSkillID() {
		t.Error("抽出来的技能卡没有带 skill_id —— " +
			"它会被 applyCard 的 `hasSkillID` 判据挡掉，变成一张哑弹")
	}
}

// TestApplyCardDispatchesOnRolledCards 钉住「deck 里的三类卡都能被分发」。
//
// ⚠️ 这里**不用**手工构造的 `Card{..., hasSkillID: true}`，
//
//	而是把 `rollWaveCards` 的真实产出直接喂给 `applyCard`，
//	断言「每一类都真的产生了效果」。
//	只用「效果变了」当判据，是因为那才是玩家能观察到的东西。
func TestApplyCardDispatchesOnRolledCards(t *testing.T) {
	// 属性卡：抽到哪一张不确定（池子 6 张），所以断言「**任一**属性加成被加上」
	// 而不是「attack 一定被加」—— 写死具体那张卡等于把守卫绑在一次抽牌结果上
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	for _, c := range e.Deck.Hand {
		if c.Kind == CardKindAttribute {
			e.applyCard(c)
		}
	}
	if e.Buffs.AttackPermille+e.Buffs.ElementCoefPermille+e.Buffs.CritPermille+
		e.Buffs.ElementCapBonus+e.Buffs.ArmorPermille+e.Heat.CapBonus == 0 {
		t.Error("取属性卡没有加上任何加成 —— applyCard 的属性分支没走到")
	}

	// 机制卡：pierce_bonus（乘以专精系数后至少 +1）
	e2 := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e2)
	for _, c := range e2.Deck.Hand {
		if c.Kind == CardKindMechanic {
			e2.applyCard(c)
		}
	}
	total := e2.Buffs.PierceBonus + e2.Buffs.ChainBonus +
		e2.Buffs.AoeBonus + e2.Buffs.FreeDiscard + e2.Buffs.OverheatGuard
	if total == 0 {
		t.Error("取机制卡没有加上任何加成 —— applyCard 的机制分支没走到")
	}

	// 技能卡：已装备的技能必须升格
	e3 := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e3)
	before := e3.Skills[0]
	var applied int
	for _, c := range e3.Deck.Hand {
		if c.Kind == CardKindSkill {
			e3.applyCard(c)
			applied++
		}
	}
	if applied == 0 {
		t.Fatal("手牌里没有技能卡")
	}
	upgraded := false
	for _, s := range e3.Skills {
		if s.SkillID == before.SkillID &&
			(s.BaseDamage > before.BaseDamage || s.ApplyStacks > before.ApplyStacks) {
			upgraded = true
		}
	}
	if !upgraded {
		t.Errorf("取技能卡后技能没有升格（%v -> %v）—— "+
			"卡被取走、emit 了 card_taken、也记了 replay，"+
			"但效果那一步是哑的：玩家看到「获得技能卡」却什么都没发生",
			skillIDsOf(e3.Skills), skillIDsOf(e3.Skills))
	}
}

// TestTakingSkillCardFromDeckUpgradesSkill 钉住「玩家动作」这条全路径。
//
// ⚠️ 全程只用玩家能做的动作：`TakeCard`。
//
//	不手工构造 `Card`、不直接调 `applyCard`、不设任何字段。
//	断言只看**玩家能观察到的东西**：装备上的技能变强了。
//
//	这条是本 bug 的**主守卫** —— 前两条只要有一个跟着实现一起改就会失效，
//	而这条只依赖「发牌 → 取牌 → 效果」这条链本身。
func TestTakingSkillCardFromDeckUpgradesSkill(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	e.WaveIndex = 0
	enterCardSelect(t, e)

	var target string
	for _, c := range e.Deck.Hand {
		if c.Kind == CardKindSkill {
			target = c.ID
		}
	}
	if target == "" {
		t.Fatal("手牌里没有技能卡")
	}

	before := map[int]EquippedSkill{}
	for _, s := range e.Skills {
		before[s.SkillID] = s
	}
	// 清空手上的技能卡 id，让升级路径不依赖具体抽到哪个技能
	beforeIDs := skillIDsOf(e.Skills)

	if _, ok := e.TakeCard(target); !ok {
		t.Fatalf("取技能卡失败：%s", target)
	}

	changed := false
	for _, s := range e.Skills {
		b := before[s.SkillID]
		if s.BaseDamage > b.BaseDamage {
			changed = true
		}
	}
	if !changed {
		t.Errorf("从手牌取走技能卡后没有任何技能变强（%v）—— "+
			"`applyCard` 的技能分支没走到。\n"+
			"这是本 bug 的主守卫：它只依赖「发牌 → 取牌 → 效果」这条链，"+
			"不手工构造任何 Card，所以实现怎么改都不会绕过它。",
			beforeIDs)
	}
}

// TestTakeCardStillRecordsWhenEffectIsSilent 钉住「记录与效果是两件事」。
//
// ⚠️ 这条是**反向**守卫：它说明为什么这个 bug 能一路绿灯。
//
//	`TakeCard` 无论 `applyCard` 有没有产生效果，都会
//	`emit(card_taken)` + `record(card)` —— 所以 replay 事件流
//	**看不出**技能卡是不是哑弹。
//
//	这解释了「为什么事件流比对查不出这个 bug」，
//	也提醒下一个改这里的人：**不要因为「记录看起来对」就认为效果对**。
func TestTakeCardStillRecordsWhenEffectIsSilent(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	e.WaveIndex = 0
	enterCardSelect(t, e)
	e.DrainEvents()

	// 一张内容表里没有的技能卡：applyCard 会静默无操作
	ghost := Card{
		ID: "skill_999_0", Kind: CardKindSkill, SkillID: 999,
		Name: "ghost", hasSkillID: true,
	}
	e.Deck.SetHand([]Card{ghost})
	before := e.Skills

	if _, ok := e.TakeCard(ghost.ID); !ok {
		t.Fatal("取牌失败")
	}

	recorded := false
	for _, r := range e.replay {
		if r.Type == ReplayCard {
			recorded = true
		}
	}
	if !recorded {
		t.Error("效果是哑的，但 replay 里没有 card 记录 —— " +
			"那样就查不出「动作发生了而效果没发生」这类问题")
	}
	if len(e.Skills) != len(before) {
		t.Errorf("幽灵技能居然被装备上了（%d -> %d）", len(before), len(e.Skills))
	}
}
