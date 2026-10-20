package domain

// Block F（卡牌）的**针对性**守卫：专打跨端对齐抓不到的分支。
//
// # 跨端对齐抓到了什么、没抓到什么
//
// 抓到了：`rollWaveCards` 的抽取顺序与 id 生成、`applyCard` 的三条分支、
// `card_picks` 编码、`replay` 事件流、局内加成、手牌、技能升格。
//
// 抓不到的（本文件负责）：
//
//  1. **重复触发**同类地形时，事件/replay 仍要发 ——
//     跨端那 6 个场景里只有第 42 关有两座蓄能塔，
//     而它们的触发点**恰好被采样到了**。换一个 param 就漏。
//  2. **「弃一张 + 取一张」的掩码编码** ——
//     跨端场景覆盖了它，但采样间隔 200/300 tick 会把中间那几 tick 抹平；
//     且第 159 轮起弃牌掩码（弃的是哪张、弃了几张）取代了旧的
//     「弃过与否」单 bit，这里逐条盯住编码值与重放复现。
//  3. `rollSkillCards` 的**重抽 guard** —— actives 只有 1 张而 count > 1 时
//     无 guard 会**永不退出**（引擎挂死）。跨端场景里池子总是 > 1 张。
//  4. 脚本**消费完**之后走 `skipCards` 而不是 panic / 空转。

import (
	"strconv"
	"strings"
	"testing"
	"time"
)

// newCardProbeEngine 造一个带可抽取技能池与真实手牌流程的引擎。
func newCardProbeEngine(t *testing.T, skillIDs ...int) *BattleEngine {
	t.Helper()
	e := NewBattleEngine(BattleConfig{
		Level:       GenerateLevel(1),
		ActiveSlots: 4,
		Skills: map[int]SeedSkill{
			1: {ID: 1, Name: "ember", Element: ElementFire, Kind: "active",
				HeatCost: 10, BaseDamage: 100, ProjectileSpeed: ToFixed(1000)},
			2: {ID: 2, Name: "napalm", Element: ElementFire, Kind: "active",
				HeatCost: 10, BaseDamage: 140, ProjectileSpeed: ToFixed(1000)},
			3: {ID: 3, Name: "frost", Element: ElementIce, Kind: "active",
				HeatCost: 10, BaseDamage: 90, ProjectileSpeed: ToFixed(1000)},
			9: {ID: 9, Name: "ward", Element: ElementFire, Kind: "passive", BaseDamage: 0},
			42: {ID: 42, Name: "ghost", Element: ElementFire, Kind: "active",
				HeatCost: 10, BaseDamage: 50, ProjectileSpeed: ToFixed(1000)},
		},
		Equipped:   mustEquip(t, skillIDs...),
		Attacker:   Attacker{Attack: 100, ElementCap: 3, MechanicPermille: permille},
		Seed:       12345,
		ScoreRules: DefaultScoreRules(),
	})
	return e
}

func mustEquip(t *testing.T, ids ...int) []EquippedSkill {
	t.Helper()
	kindOf := map[int]string{1: "active", 2: "active", 3: "active", 9: "passive", 42: "active"}
	out := make([]EquippedSkill, 0, len(ids))
	for i, id := range ids {
		out = append(out, EquippedSkill{
			SkillID: id, Name: "s" + strconv.Itoa(id), Element: ElementFire,
			Kind: kindOf[id], HeatCost: 10, BaseDamage: 100,
			ProjectileSpeed: ToFixed(1000), Slot: i,
		})
	}
	return out
}

// enterCardSelect 把引擎推到选牌阶段并发牌。
func enterCardSelect(t *testing.T, e *BattleEngine) {
	t.Helper()
	// eginCardSelect **不**改 phase —— 那是 checkWaveEnd 的职责。
	// 这里手动置成 card_select 再发牌，模拟 checkWaveEnd 的那一步。
	e.Phase = PhaseCardSelect
	e.beginCardSelect()
	if e.Phase != PhaseCardSelect {
		t.Fatalf("beginCardSelect 之后 phase = %q，期望 card_select", e.Phase)
	}
}

// TestRollWaveCardsShapeIsFixed 守住「每波 1 技能 + 1 属性 + 1 机制」。
//
// ⚠️ 固定构成**不是**「抽 3 张随机牌」：
//
//	顺序进 RNG，顺序改成「技能/属性/机制」之外的排列会让
//	后续所有波次的抽牌结果整体偏移 ——
//	症状是「某个关卡的某张卡不对」，定位时完全看不出是顺序问题。
func TestRollWaveCardsShapeIsFixed(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)

	if len(e.Deck.Hand) != 3 {
		t.Fatalf("手牌数 = %d，期望 3（1 技能 + 1 属性 + 1 机制）", len(e.Deck.Hand))
	}
	kinds := []string{e.Deck.Hand[0].Kind, e.Deck.Hand[1].Kind, e.Deck.Hand[2].Kind}
	if got := strings.Join(kinds, ","); got != "skill,attribute,mechanic" {
		t.Errorf("手牌顺序 = [%s]，期望 [skill,attribute,mechanic] —— "+
			"顺序进 RNG，换序会让所有波次的抽牌整体偏移", got)
	}
}

// TestRollSkillCardsSkipsPassives 守住「池子里只出主动技能」。
//
// ⚠️ 池子必须**包含**被动技能但只发主动的：
//
//	若 `unlockedSkillCards` 先把被动滤掉，
//	`rollSkillCards` 的 `actives.length === 0` 早退分支
//	就会让「玩家只装了被动」与「池子空了」变得不可区分。
func TestRollSkillCardsSkipsPassives(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3, 9)
	enterCardSelect(t, e)

	if got := e.Deck.Hand[0]; got.Kind != CardKindSkill {
		t.Fatalf("第 0 张应是技能卡，实际 %q", got.Kind)
	}
	// 只装被动时必须**不发技能卡**，但仍发属性/机制两张。
	onlyPassive := newCardProbeEngine(t, 9)
	enterCardSelect(t, onlyPassive)
	if len(onlyPassive.Deck.Hand) != 2 {
		t.Errorf("只装被动时手牌数 = %d，期望 2（属性 + 机制）", len(onlyPassive.Deck.Hand))
	}
	if onlyPassive.Deck.Hand[0].Kind == CardKindSkill {
		t.Error("只装被动时不该发出技能卡 —— 那是钻空子的入口")
	}
}

// TestRollSkillCardsSinglePoolTerminates 守住重抽 guard。
//
// ⚠️ actives 只有 1 张而 count = 2 时，
//
//	`for used[pick] { pick = rng.Intn(len) }` **永不退出** ——
//	引擎挂死，服务端线程卡住。这是本文件里唯一一条
//	「不测就线上事故」的守卫。
func TestRollSkillCardsSinglePoolTerminates(t *testing.T) {
	rng := newLCG(1)
	pool := []Card{{SkillID: 7, Name: "solo", Kind: "active"}}
	done := make(chan []Card, 1)
	go func() { done <- rollSkillCards(rng, pool, 3) }()
	select {
	case got := <-done:
		if len(got) != 3 {
			t.Errorf("池子只有 1 张时抽 3 张得到 %d 张", len(got))
		}
	case <-time.After(5 * time.Second):
		t.Fatal("rollSkillCards 在单元素池上死循环 —— 重抽缺 guard")
	}
}

// TestSkipCardsDropsWholeHand 守住「跳过本波要真的把手牌清空」。
//
// ⚠️ 这里踩过一个 Go 特有的坑：`Deck.Drop` 走 `splice`，
//
//	`for _, c := range e.Deck.Hand { e.Deck.Drop(c.ID) }`
//	在遍历中删除会**跳过元素**，3 张只删掉 2 张。
//	TS 那边写的是 `for (const c of [...this.deck.hand])` ——
//	那个展开运算符正为此存在，不是随手写的。
func TestSkipCardsDropsWholeHand(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	e.WaveIndex = 0

	e.SkipCards()

	if len(e.Deck.Hand) != 0 {
		t.Errorf("跳过后手牌还剩 %d 张：%v —— "+
			"遍历中 splice 会跳过元素，必须先复制再删", len(e.Deck.Hand), e.Deck.Hand)
	}
	if got := e.CardPicks; len(got) != 1 || got[0] != -1 {
		t.Errorf("card_picks = %v，期望 [-1]", got)
	}
}

// TestSkipCardsAfterDiscardEncodesMask 守住「弃过牌后跳过的编码携带弃牌掩码」。
//
// ⚠️ 第 159 轮重写（旧名 TestSkipCardsAfterDiscardUsesMinusSix）：
//
//	旧编码 -6 只说明「弃过一张」，说不出弃的是哪一张 ——
//	而重放侧固定弃 hand[0]，玩家弃第 2/3 张时必然失配。
//	新编码 -(3+code)：mask=0b1（弃原手牌第 0 张）+ 不取 → code=6 → -9。
func TestSkipCardsAfterDiscardEncodesMask(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	e.WaveIndex = 0
	e.Deck.DiscardsLeft = 1

	first := e.Deck.Hand[0].ID
	if _, ok := e.DiscardCard(first); !ok {
		t.Fatal("弃牌失败")
	}
	e.SkipCards()

	if len(e.Deck.Hand) != 0 {
		t.Errorf("跳过后手牌还剩 %d 张", len(e.Deck.Hand))
	}
	want := -(PickCodeBase + 6) // mask=1 的区间起点 4 + 「不取」= 6
	if got := e.CardPicks; len(got) != 1 || got[0] != want {
		t.Errorf("card_picks = %v，期望 [%d]（弃第 0 张后跳过的掩码编码）", got, want)
	}
}

// TestTakeCardEncodesHandIndexNotCardKind 守住 `card_picks` 记的是**下标**。
//
// ⚠️ `card_picks` 与 `record('card', ...)` 是**两套编码**，别搞混：
//
//	card_picks  → handIdx（玩家眼中的第几张，0/1/2）
//	record card → cardIndex（卡种编码：skillId*10 / 500+i / 900+i）
//
// 我第一版把 record 的 `cardIndex` 写进了 `recordPick` ——
// 症状是 `card_picks` 出现 `skill_2_0` 对应的 20 而不是 0。
func TestTakeCardEncodesHandIndexNotCardKind(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	e.WaveIndex = 0

	// 取第 2 张（机制卡），card_picks 应为 2 而非 900+1=901
	if _, ok := e.TakeCard(e.Deck.Hand[2].ID); !ok {
		t.Fatal("取牌失败")
	}
	if got := e.CardPicks; len(got) != 1 || got[0] != 2 {
		t.Errorf("card_picks = %v，期望 [2]（手牌下标，不是 cardIndex）", got)
	}
	// replay 里那条 `card` 记录必须是卡种编码
	found := false
	for _, r := range e.replay {
		if r.Type == ReplayCard {
			if r.A < 500 {
				t.Errorf("record('card') 的值 = %d，期望卡种编码（>=500）", r.A)
			}
			found = true
		}
	}
	if !found {
		t.Error("取牌没有写 record('card') 事件")
	}
}

// TestRecordPickOnlyOncePerWave 守住「每波只记一次 card_picks」。
//
// ⚠️ `recordPick` 的守卫是 `len(CardPicks) <= WaveIndex`，
//
//	它同时承担「每波一次」与「弃牌那一记不覆盖取牌那一记」两个职责。
//	守卫写错的话 `card_picks` 会比波数长，
//	而服务端 `validCardPick` 按 `len <= WaveCount` 校验 ——
//	症状是「玩家正常打完一局，结算被判非法」。
func TestRecordPickOnlyOncePerWave(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	e.WaveIndex = 0

	for i := 0; i < 3; i++ {
		if len(e.Deck.Hand) == 0 {
			break
		}
		e.recordPick(i)
	}
	if got := e.CardPicks; len(got) != 1 {
		t.Errorf("第 0 波连续记了 3 次，card_picks = %v，期望长度 1", got)
	}

	e.WaveIndex = 1
	e.recordPick(1)
	if got := e.CardPicks; len(got) != 2 {
		t.Errorf("第 1 波应追加第 2 项，card_picks = %v", got)
	}
}

// TestApplySkillCardUpgradesRatherThanDuplicates 守住「已装备则升格」。
//
// ⚠️ 升格公式 `baseDamage + baseDamage/5` 是**整数除法**，向零截断：
//
//	`baseDamage < 5` 时**一点伤害都不涨**。
//	这是 TS 的同款行为，不是在这里引入的，
//	但它意味着「升格低级技能不划算」在低等级时成立 ——
//	改公式前先确认 TS 那边是不是也这么写。
func TestApplySkillCardUpgradesRatherThanDuplicates(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2)
	before := e.Skills[0]
	card := Card{Kind: CardKindSkill, SkillID: before.SkillID, hasSkillID: true}
	e.applyCard(card)

	if len(e.Skills) != len(before2Skills(before)) {
		// 长度不应变化：已装备的技能是升格而不是新增
		if len(e.Skills) != 2 {
			t.Fatalf("升格后技能数 = %d，期望 2（不应新增也不应减少）", len(e.Skills))
		}
	}
	if e.Skills[0].ApplyStacks != before.ApplyStacks+1 {
		t.Errorf("applyStacks = %d，期望 %d", e.Skills[0].ApplyStacks, before.ApplyStacks+1)
	}
	if e.Skills[0].Pierce != before.Pierce+1 {
		t.Errorf("pierce = %d，期望 %d", e.Skills[0].Pierce, before.Pierce+1)
	}
	want := before.BaseDamage + before.BaseDamage/5
	if e.Skills[0].BaseDamage != want {
		t.Errorf("baseDamage = %d，期望 %d（+baseDamage/5，整数除法）",
			e.Skills[0].BaseDamage, want)
	}
}

// before2Skills 只为上面那条断言读一个 2 长度切片，恒真。
func before2Skills(before EquippedSkill) []EquippedSkill {
	_ = before
	return make([]EquippedSkill, 2)
}

// TestApplySkillCardFillsFreeSlot 守住「新技能装进**空槽**」。
//
// ⚠️ 这里踩过一个真实的坑：曾经写成
//
//	「找第一个 `s.slot < activeSlots && s.slot >= 0`」
//
//	而 skills 数组里每个元素就代表一个**已占用**的槽，
//	于是这个表达式命中的永远是第一个**已占用**的槽，随后直接把它替换掉。
//	后果：抽到任何**新**技能都会顶掉槽 0 的老技能，槽位数永远不增加。
//	实测已装 [1,2] 时抽到技能 3：before=[1,2] → after=[3,2]。
func TestApplySkillCardFillsFreeSlot(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2)
	if e.Skills[0].SkillID != 1 || e.Skills[1].SkillID != 2 {
		t.Fatalf("前置条件不成立：已装 %v", skillIDsOf(e.Skills))
	}

	e.applyCard(Card{Kind: CardKindSkill, SkillID: 3, hasSkillID: true})

	if len(e.Skills) != 3 {
		t.Fatalf("装入新技能后技能数 = %d，期望 3（槽 0/1 不能被顶掉）", len(e.Skills))
	}
	if got := skillIDsOf(e.Skills); got[0] != 1 || got[1] != 2 {
		t.Errorf("技能列表 = %v，期望前两个仍是 1,2 —— "+
			"新技能必须装进**空槽**，不能顶掉已占用的槽", got)
	}
	free := -1
	for _, s := range e.Skills {
		if s.SkillID == 3 {
			free = s.Slot
		}
	}
	if free != 2 {
		t.Errorf("新技能的 slot = %d，期望 2（第一个空槽）", free)
	}
}

// TestApplySkillCardFullSlotsUpgradesBest 守住「槽满时强化伤害最高的那个」。
//
// ⚠️ 槽满时最合理的处理不是「顶掉随机一个」，
//
//	而是明确地告诉玩家「没位置了」—— 所以这里做的是
//	「强化已有技能里伤害最高的那一个」，
//	至少不会让玩家的构筑凭空少一个技能。
func TestApplySkillCardFullSlotsUpgradesBest(t *testing.T) {
	// ⚠️ 必须装**满** `ActiveSlots`（4）个：
	//
	//	只装 3 个时槽 3 还空着，走的是「装入空槽」分支，
	//	根本到不了「槽满」的逻辑 —— 而那条分支本来就是对的，
	//	所以断言会「意外地通过」并给出错误的安心感。
	e := newCardProbeEngine(t, 1, 2, 3, 42)
	if len(e.Skills) != e.ActiveSlots {
		t.Fatalf("前置条件不成立：已装 %d 个，ActiveSlots = %d", len(e.Skills), e.ActiveSlots)
	}
	// 让 3 号技能的伤害最高，便于断言「被强化的是它」
	best := -1
	for i := range e.Skills {
		if e.Skills[i].SkillID == 3 {
			e.Skills[i].BaseDamage = 9999
			best = i
		}
	}
	beforeStacks := e.Skills[best].ApplyStacks

	// 9 号在内容表里存在（被动），抽到它却没有空槽可装。
	e.applyCard(Card{Kind: CardKindSkill, SkillID: 9, hasSkillID: true})

	if len(e.Skills) != e.ActiveSlots {
		t.Fatalf("槽满时技能数变成了 %d，期望仍为 %d", len(e.Skills), e.ActiveSlots)
	}
	if e.Skills[best].ApplyStacks != beforeStacks+1 {
		t.Errorf("槽满时没有强化伤害最高的技能（stacks %d -> %d）—— "+
			"槽满时最合理的处理不是「顶掉随机一个」，"+
			"而是明确告诉玩家「没位置了」并强化已有的最强项",
			beforeStacks, e.Skills[best].ApplyStacks)
	}
}

// TestApplySkillCardBranchesOnSkillIDPresence 守住「skill_id **出现过**就算技能卡」。
//
// # 为什么用「分支是否被选中」而不是「技能列表变没变」当判据
//
// `applySkillCard` 的末尾要查 `cfg.skills` 拿技能定义；
// 内容表当前 id 从 1 开始，所以 id=0 查不到 ——
// 于是「分支被选中」与「技能列表变化」在真实数据上**观察不到差别**。
//
// 但它们在**逻辑上**是两件事：
//
//	判据用 `SkillID != 0` → id=0 落进最后的隐式 else，
//	    卡被 consume 掉、emit 了 card_taken，但什么也没发生 ——
//	    玩家看到「获得技能卡」却没反应。
//	判据用 `hasSkillID`   → id=0 也走技能分支，
//	    「查不到定义」是内容表的问题，与判据无关。
//
// 所以这里往内容表里**注册一个 id=0 的技能**，
// 把两个分支的差别暴露出来。这不是为了造数据好看，
// 而是唯一能让这个判据的差异**可观测**的方式。
func TestApplySkillCardBranchesOnSkillIDPresence(t *testing.T) {
	newEngineWithZeroSkill := func() *BattleEngine {
		e := newCardProbeEngine(t, 1, 2)
		e.Cfg.Skills[0] = SeedSkill{
			ID: 0, Name: "zero", Element: ElementFire, Kind: "active",
			HeatCost: 10, BaseDamage: 10, ProjectileSpeed: ToFixed(1000),
		}
		return e
	}

	// 有 hasSkillID → 走技能分支 → 装入空槽
	withFlag := newEngineWithZeroSkill()
	withFlag.applyCard(Card{Kind: CardKindSkill, SkillID: 0, hasSkillID: true})
	if len(withFlag.Skills) != 3 {
		t.Errorf("skill_id=0 且字段出现过时技能数 = %d，期望 3 —— "+
			"判据退化成 `SkillID != 0` 会让这张卡被静默丢弃", len(withFlag.Skills))
	}

	// 没有 hasSkillID（来自缺字段的旧序列化）→ 落进隐式 else，什么也不做
	withoutFlag := newEngineWithZeroSkill()
	withoutFlag.applyCard(Card{Kind: CardKindSkill, SkillID: 0, hasSkillID: false})
	if len(withoutFlag.Skills) != 2 {
		t.Errorf("没有 skill_id 字段时技能数 = %d，期望 2（应落进隐式 else）",
			len(withoutFlag.Skills))
	}
}

// TestApplySkillCardUnknownSkillIsNoOp 守住「内容表里没有的技能卡不改变任何东西」。
//
// ⚠️ 这条本身是 TS 的同款行为（`cfg.skills.find(...)` 找不到就 return），
//
//	但它**必须**被钉住：一旦改成「查不到就用默认值造一个」，
//	玩家就能凭空装备任意技能，而那正是刷分的入口。
func TestApplySkillCardUnknownSkillIsNoOp(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2)
	before := len(e.Skills)
	beforeIDs := skillIDsOf(e.Skills)

	e.applyCard(Card{Kind: CardKindSkill, SkillID: 999, hasSkillID: true})

	if len(e.Skills) != before {
		t.Errorf("内容表里没有的技能卡却装备成功了（%v -> %v）—— "+
			"那是凭空获得技能的入口", beforeIDs, skillIDsOf(e.Skills))
	}
}

// TestApplyAttributeUnitsFollowKind 守住属性卡的单位约定。
//
// ⚠️ 同一个池子里混着**两种单位**，靠 kind 区分：
//
//	attack / element_coef / crit / heat_cap / armor → **千分比**
//	element_cap                                    → **绝对层数**
//
//	`heat_cap` 的 200 是第 72 轮从 `20` 修过来的 ——
//	`cap = HEAT_MAX * (1000 + capBonus) / 1000`，
//	所以 20 只把上限从 100 抬到 102，而 descr 承诺「+20」= 120。
//	玩家实际收益是标称的 1/10。
func TestApplyAttributeUnitsFollowKind(t *testing.T) {
	e := newCardProbeEngine(t, 1)

	e.applyAttribute(AttributeEffect{Kind: "attack", Value: 150})
	if e.Buffs.AttackPermille != 150 {
		t.Errorf("attack 应进千分比，实际 %d", e.Buffs.AttackPermille)
	}

	e.applyAttribute(AttributeEffect{Kind: "heat_cap", Value: 200})
	if e.Heat.CapBonus != 200 {
		t.Errorf("heat_cap 应进 Heat.CapBonus，实际 %d", e.Heat.CapBonus)
	}
	if e.Buffs.AttackPermille != 150 {
		t.Error("heat_cap 不该动 attackPermille —— 否则 Buffs 与 Heat 两处都有一份，" +
			"而真正被 Heat.Cap() 读走的是后者，前者成了只写不读的镜像字段")
	}

	e.applyAttribute(AttributeEffect{Kind: "element_cap", Value: 1})
	if e.Buffs.ElementCapBonus != 1 {
		t.Errorf("element_cap 应进 ElementCapBonus（绝对层数），实际 %d", e.Buffs.ElementCapBonus)
	}
}

// TestApplyMechanicScalesByTreeBonus 守住「机制卡面值的专精树放大」。
//
// ⚠️ 选「放大卡面数值」而不是「抽到好卡的概率更高」：
//
//	前者是确定性的（重放能逐位复现），
//	后者要改抽牌随机序列，而卡牌是客户端选的，
//	改序列会与 `card_picks` 闭环冲突。
func TestApplyMechanicScalesByTreeBonus(t *testing.T) {
	e := newCardProbeEngine(t, 1)
	e.Cfg.Attacker.MechanicPermille = 0 // 无树加成，面值原样生效

	e.applyMechanic(MechanicEffect{Kind: "pierce_bonus", Value: 1})
	if e.Buffs.PierceBonus != 1 {
		t.Errorf("PierceBonus = %d，期望 1", e.Buffs.PierceBonus)
	}

	e2 := newCardProbeEngine(t, 1)
	e2.Cfg.Attacker.MechanicPermille = permille / 2 // 150%
	e2.applyMechanic(MechanicEffect{Kind: "pierce_bonus", Value: 2})
	if e2.Buffs.PierceBonus != 3 {
		t.Errorf("150%% 放大下 PierceBonus = %d，期望 3（面值 2 乘 1.5）", e2.Buffs.PierceBonus)
	}
}

// TestApplyMechanicFreeDiscardAlsoTopsUpNow 守住「快速装填同时加剩余次数」。
//
// ⚠️ 只加 `Buffs.FreeDiscard` 的话，
//
//	本波的 `discardsLeft` 不会因为新卡而 +1 ——
//	玩家拿到「快速装填」却仍然只能用默认次数。
//	而 `discardsLeft` 进跨端比对，不一致时症状是「某一波的弃牌数不同」。
func TestApplyMechanicFreeDiscardAlsoTopsUpNow(t *testing.T) {
	e := newCardProbeEngine(t, 1)
	e.Cfg.Attacker.MechanicPermille = 0
	e.Deck.DiscardsLeft = 1

	e.applyMechanic(MechanicEffect{Kind: "free_discard", Value: 1})

	if e.Buffs.FreeDiscard != 1 {
		t.Errorf("FreeDiscard = %d，期望 1", e.Buffs.FreeDiscard)
	}
	if e.Deck.DiscardsLeft != 2 {
		t.Errorf("DiscardsLeft = %d，期望 2 —— "+
			"玩家拿到「快速装填」却仍然只能用默认次数的话，这张卡是死的", e.Deck.DiscardsLeft)
	}
}

// TestApplyReplayDecisionConsumesOnePerCall 守住「每个 card_select 恰好消费一项」。
//
// ⚠️ 第 159 轮重写：旧实现允许合成脚本在同一 tick 连点消费多项
// （`[-3,-3,-1,-1]` 这类错位就是这么来的），而线上真实玩家
// 做不到 50ms 内点三张卡 —— 那个语义从来就不存在。
func TestApplyReplayDecisionConsumesOnePerCall(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	e.WaveIndex = 0
	// -3 = mask=0（没弃牌）+ 取剩余第 0 张 → 等价普通取牌
	e.SetReplayScript([]int{-3, -3, -3, -3, -3, -3, -3, -3})

	e.applyReplayDecision()

	if e.ReplayScriptPos != 1 {
		t.Errorf("一次 applyReplayDecision 消费了 %d 项脚本，期望 1 —— "+
			"一整波的决策被塞进同一 tick，card_picks 会记成跨波的值", e.ReplayScriptPos)
	}
	if got := e.CardPicks; len(got) != 1 || got[0] != 0 {
		t.Errorf("card_picks = %v，期望 [0]（mask=0 的取牌回写成普通下标）", got)
	}
	if e.Phase != PhaseWave {
		t.Errorf("取牌后 phase = %q，期望 wave —— 取牌必须立即结束本波", e.Phase)
	}
}

// TestApplyReplayDecisionDiscardThenTakeFinishesImmediately 守住
// 「弃牌掩码可被重放复现 + 取牌立即结束本波」。
//
// ⚠️ 第 159 轮重写（旧名为 ...DefersOneTick）：
//
//	选牌时钟冻结之后「取牌」一律立即 finishCardSelect ——
//	`cardSelectDone` / `finishCardSelectNextTick` 已整体删除，
//	旧测试钉住的「延后一 tick 收尾」路径不存在了。
func TestApplyReplayDecisionDiscardThenTakeFinishesImmediately(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	e.WaveIndex = 0
	e.Deck.DiscardsLeft = 1
	// -15 = mask=0b100（弃原手牌第 2 张）+ 取剩余第 0 张
	e.SetReplayScript([]int{-15, -15, -15, -15})

	e.applyReplayDecision()

	if e.Phase != PhaseWave {
		t.Errorf("取牌后 phase = %q，期望 wave —— 取牌必须立即结束本波", e.Phase)
	}
	if len(e.Deck.Hand) != 0 {
		t.Errorf("剩余手牌应被静默丢弃，实际还剩 %d 张", len(e.Deck.Hand))
	}
	if got := e.CardPicks; len(got) != 1 || got[0] != -15 {
		t.Errorf("card_picks = %v，期望 [-15]（掩码编码必须与脚本逐位一致）", got)
	}
	if e.ReplayScriptPos != 1 {
		t.Errorf("ReplayScriptPos = %d，期望 1", e.ReplayScriptPos)
	}
}

// TestApplyReplayDecisionExhaustedScriptSkips 守住「脚本消费完走 SkipCards」。
//
// ⚠️ 走到这里必须是 `SkipCards` 而不是 panic / 空转：
//
//	空转会让 `Step` 在 card_select 里打转 ——
//	引擎既不报错也不推进，玩家看到界面卡死。
func TestApplyReplayDecisionExhaustedScriptSkips(t *testing.T) {
	e := newCardProbeEngine(t, 1, 2, 3)
	enterCardSelect(t, e)
	e.WaveIndex = 0
	e.SetReplayScript([]int{}) // 空脚本

	e.applyReplayDecision()

	if e.Phase != PhaseWave {
		t.Errorf("脚本耗尽后 phase = %q，期望 wave（回退为整波跳过）", e.Phase)
	}
	if got := e.CardPicks; len(got) != 1 || got[0] != -1 {
		t.Errorf("card_picks = %v，期望 [-1]", got)
	}
	if e.ReplayScriptPos != 0 {
		t.Errorf("ReplayScriptPos = %d，期望停在 0（不得越界）", e.ReplayScriptPos)
	}
}

// TestCardIndexEncoding 守住 record('card') 的卡种编码。
//
// ⚠️ 这套编码进 `replay_hash`，写错一位就是全量战报哈希失配。
func TestCardIndexEncoding(t *testing.T) {
	cases := []struct {
		card Card
		want int64
	}{
		{Card{Kind: CardKindSkill, SkillID: 2, hasSkillID: true}, 20},
		{Card{Kind: CardKindAttribute, Effect: AttributeEffect{Kind: "attack"}}, 501},
		{Card{Kind: CardKindAttribute, Effect: AttributeEffect{Kind: "armor"}}, 506},
		{Card{Kind: CardKindMechanic, Mechanic: MechanicEffect{Kind: "pierce_bonus"}}, 901},
		{Card{Kind: CardKindMechanic, Mechanic: MechanicEffect{Kind: "overheat_guard"}}, 905},
	}
	for _, c := range cases {
		if got := cardIndex(c.card); got != c.want {
			t.Errorf("cardIndex(%s/%s) = %d，期望 %d",
				c.card.Kind, c.card.Effect.Kind+string(rune(0)), got, c.want)
		}
	}
}

// TestAttributeAndMechanicIndexFollowPoolOrder 守住下标与卡池逐项对齐。
//
// ⚠️ 顺序错一位会让每张卡的 record 值都错，
//
//	而表现只是「replayHash 不对」—— 定位起来毫无线索。
func TestAttributeAndMechanicIndexFollowPoolOrder(t *testing.T) {
	wantAttr := []string{"attack", "element_coef", "crit", "heat_cap", "element_cap", "armor"}
	for i, k := range wantAttr {
		if attributePool[i].Effect.Kind != k {
			t.Errorf("attributePool[%d] = %q，期望 %q", i, attributePool[i].Effect.Kind, k)
		}
		if got := attributeIndex(k); got != int64(i+1) {
			t.Errorf("attributeIndex(%q) = %d，期望 %d", k, got, i+1)
		}
	}
	wantMech := []string{"pierce_bonus", "chain_bonus", "aoe_bonus", "free_discard", "overheat_guard"}
	for i, k := range wantMech {
		if mechanicPool[i].Mechanic.Kind != k {
			t.Errorf("mechanicPool[%d] = %q，期望 %q", i, mechanicPool[i].Mechanic.Kind, k)
		}
		if got := mechanicIndex(k); got != int64(i+1) {
			t.Errorf("mechanicIndex(%q) = %d，期望 %d", k, got, i+1)
		}
	}
}

// TestMarkTerrainUsedDeduplicatesListButAlwaysEmits 守住
// 「列表去重、事件每次都发」。
//
// ⚠️ 这是本轮真实踩到的 bug：第 42 关有两座蓄能塔（param 38 / 42），
//
//	第 38 杀触发第一座、第 42 杀触发第二座，两座 `kind` 都是 `charge_tower`。
//	我把去重守卫写成了「已记过就 return」，于是第二座的 terrain 事件被吃掉 ——
//	而 `kills` / `score` / 地形状态 / `terrain_used` **全都对得上**
//	（因为 `terrain_used` 本来就该去重），
//	唯一的差异是 replay 流少一条 `terrain|5|381|607`，
//	而那一条直接进 `replay_hash`。
//
// 断言分三件事：列表仍去重、事件发两次、replay 记两条。
func TestMarkTerrainUsedDeduplicatesListButAlwaysEmits(t *testing.T) {
	e := newCardProbeEngine(t, 1)
	tower := &Terrain{Kind: TerrainChargeTower, X: 800, Y: 750, State: TerrainIdle}
	other := &Terrain{Kind: TerrainChargeTower, X: 381, Y: 607, State: TerrainIdle}
	e.Terrains = []*Terrain{tower, other}
	e.DrainEvents()

	e.markTerrainUsed(tower)
	e.markTerrainUsed(other)

	if len(e.TerrainUsed) != 1 {
		t.Errorf("terrain_used = %v，期望只有 1 项（同类去重）", e.TerrainUsed)
	}
	if got := len(e.DrainEvents()); got != 2 {
		t.Errorf("terrain 渲染事件发了 %d 次，期望 2 次 —— "+
			"去重守卫若包住了 emit，第二座塔的触发就被静默吞掉", got)
	}
	rec := 0
	for _, r := range e.replay {
		if r.Type == ReplayTerrain {
			rec++
		}
	}
	if rec != 2 {
		t.Errorf("replay('terrain') 记了 %d 条，期望 2 条 —— "+
			"它进 replay_hash，少一条就全量战报失配", rec)
	}
}

func skillIDsOf(list []EquippedSkill) []int {
	out := make([]int, 0, len(list))
	for _, s := range list {
		out = append(out, s.SkillID)
	}
	return out
}
