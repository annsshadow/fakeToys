package domain

import "strconv"

// 卡牌（R1 Block F）。
//
// 本块让战斗**跨过第 1 波**：`checkWaveEnd` 在波次清空后进入 `card_select`，
// 而解掉它要经过 `rollWaveCards`（消耗 RNG，于是后续所有波次的刷怪洗牌
// 都会变）与 `applyCard` / `applyAttribute` / `applyMechanic`。
//
// # 为什么这是最后一块「解锁多波次」的部件
//
// B~E 的所有对齐场景都**停在第 1 波末尾的 `card_select`** ——
// 硬跑满 tick 只会得到一片 `phase: card_select` 的死空间。
// F 落地后，场景才能真正走完第 2、3、… 波，
// 而多波次是 `charge_tower`（需要 38+ 击杀）与
// `rotor_vane`（需要更长的弹道里程）这两类地形唯一的可达路径。
// 卡牌种类。
const (
	CardKindSkill     = "skill"
	CardKindAttribute = "attribute"
	CardKindMechanic  = "mechanic"
)

// AttributeEffect 是属性卡的数值效果。
//
// ⚠️ ⚠️ **同一个池子里混着两种单位**，靠 kind 区分：
//
//	attack / element_coef / crit / heat_cap / armor → **千分比**
//	element_cap                                    → **绝对层数**
//
// `heat_cap` 的千分比是第 72 轮从 `20n` 修成 `200n` 的 ——
// `cap = (HEAT_MAX * (1000 + capBonus)) / 1000`，所以 20 只把上限
// 从 100 抬到 102，而 descr 承诺「+20」= 120。玩家实际收益是标称的 1/10。
// 写成本文件里的字面量时不要再改回去。
type AttributeEffect struct {
	Kind  string `json:"kind"`
	Value int64  `json:"value"`
}

// MechanicEffect 是机制卡的数值效果。
//
// ⚠️ `Value` 是 **number**（小整数计数）而攻方是定点：
//
//	`applyMechanic` 要先取整再进定点域 ——
//	内容表的机制卡面值全是整数，取整不丢精度；
//	真出现小数时是内容表错了，静默四舍五入会让
//	「卡面显示 3、实际生效 2」这类问题更难查。
type MechanicEffect struct {
	Kind  string `json:"kind"`
	Value int    `json:"value"`
}

// attributePool 是属性卡池（每波抽 1 张）。
//
// ⚠️ **顺序进 RNG**：`rollAttributeCards` 用 `rng.intn(len)`，
//
//	所以这张表的**顺序**决定同种子下的抽卡结果，而抽卡结果
//	决定 `currentAttacker()` 的每一项，进而决定重放哈希。
//	插入/删除/换序都会改变所有战报的重放结果。
var attributePool = []Card{
	{Rarity: "common", Name: "锋锐训练", Descr: "面板攻击力 +15%",
		Effect: AttributeEffect{Kind: "attack", Value: 150}},
	{Rarity: "common", Name: "元素聚焦", Descr: "元素系数 +20%",
		Effect: AttributeEffect{Kind: "element_coef", Value: 200}},
	{Rarity: "rare", Name: "锐利视线", Descr: "暴击率 +8%",
		Effect: AttributeEffect{Kind: "crit", Value: 80}},
	{Rarity: "rare", Name: "散热涂层", Descr: "热量上限 +20%",
		Effect: AttributeEffect{Kind: "heat_cap", Value: 200}},
	{Rarity: "epic", Name: "元素容器", Descr: "单元素层数上限 +1",
		// ⚠️ 这里是**绝对值**而不是千分比：
		// 它加进 `attacker.elementCap`（层数），
		// 而 heat_cap 的 capBonus 是千分比。
		// 同一个池子里两种单位，靠 kind 区分 ——
		// 改这里之前先想清楚是哪一种。
		Effect: AttributeEffect{Kind: "element_cap", Value: 1}},
	{Rarity: "common", Name: "加固工事", Descr: "防线护甲 +10%",
		Effect: AttributeEffect{Kind: "armor", Value: 100}},
}

// mechanicPool 是机制卡池（每波抽 1 张）。
//
// ⚠️ 顺序同样进 RNG，见 `attributePool` 的说明。
var mechanicPool = []Card{
	{Rarity: "common", Name: "强化弹头", Descr: "所有技能穿透 +1",
		Mechanic: MechanicEffect{Kind: "pierce_bonus", Value: 1}},
	{Rarity: "rare", Name: "连锁导引", Descr: "所有技能弹射 +1",
		Mechanic: MechanicEffect{Kind: "chain_bonus", Value: 1}},
	{Rarity: "common", Name: "扩张装药", Descr: "所有技能溅射范围 +15%",
		Mechanic: MechanicEffect{Kind: "aoe_bonus", Value: 15}},
	{Rarity: "epic", Name: "快速装填", Descr: "每波弃牌次数 +1",
		Mechanic: MechanicEffect{Kind: "free_discard", Value: 1}},
	{Rarity: "epic", Name: "隔热护罩", Descr: "过热时免疫一次过热",
		Mechanic: MechanicEffect{Kind: "overheat_guard", Value: 1}},
}

// rollWaveCards 每波抽 3 张：1 技能 + 1 属性 + 1 机制。
//
// ⚠️ **固定构成**，不是「抽 3 张随机牌」：
//
//	顺序 1 技能 → 2 属性 → 3 机制。
//	顺序决定 RNG 消耗序列，因而决定后续所有随机量。
func rollWaveCards(rng *LCG, unlocked []Card) []Card {
	out := make([]Card, 0, 3)
	out = append(out, rollSkillCards(rng, unlocked, 1)...)
	out = append(out, rollAttributeCards(rng, 1)...)
	out = append(out, rollMechanicCards(rng, 1)...)
	return out
}

// rollSkillCards 从已解锁的主动技能里抽 count 张（不重复）。
func rollSkillCards(rng *LCG, unlocked []Card, count int) []Card {
	actives := make([]Card, 0, len(unlocked))
	for _, u := range unlocked {
		if u.Kind == "active" {
			actives = append(actives, u)
		}
	}
	if len(actives) == 0 {
		return nil
	}
	out := make([]Card, 0, count)
	used := make(map[int]bool, count)
	for i := 0; i < count; i++ {
		pick := actives[rng.Intn(len(actives))]
		// 重抽已用过的，最多重试 actives.length 次。
		//
		// ⚠️ 必须有 guard： actives 只有 1 个而 count > 1 时，
		// 无 guard 的循环**永不退出** —— 引擎挂死。
		guard := 0
		for used[pick.SkillID] && guard < len(actives) {
			pick = actives[rng.Intn(len(actives))]
			guard++
		}
		used[pick.SkillID] = true
		out = append(out, Card{
			ID:      "skill_" + strconv.Itoa(pick.SkillID) + "_" + strconv.Itoa(i),
			Kind:    CardKindSkill,
			Rarity:  "common",
			Name:    pick.Name,
			Descr:   "装备或升格 " + pick.Name,
			Element: pick.Element,
			SkillID: pick.SkillID,
			Tier:    1,
		})
	}
	return out
}

// rollAttributeCards 抽 count 张属性卡。
func rollAttributeCards(rng *LCG, count int) []Card {
	out := make([]Card, 0, count)
	for i := 0; i < count; i++ {
		tpl := attributePool[rng.Intn(len(attributePool))]
		c := tpl
		c.ID = "attr_" + strconv.Itoa(i) + "_" + strconv.Itoa(rng.Intn(1000))
		c.Kind = CardKindAttribute
		out = append(out, c)
	}
	return out
}

// rollMechanicCards 抽 count 张机制卡。
func rollMechanicCards(rng *LCG, count int) []Card {
	out := make([]Card, 0, count)
	for i := 0; i < count; i++ {
		tpl := mechanicPool[rng.Intn(len(mechanicPool))]
		c := tpl
		c.ID = "mech_" + strconv.Itoa(i) + "_" + strconv.Itoa(rng.Intn(1000))
		c.Kind = CardKindMechanic
		out = append(out, c)
	}
	return out
}

// beginCardSelect 抽本波手牌（F 块实现）。
//
// ⚠️ 必须**替换** A 块留的空实现 —— 那个空实现让 `checkWaveEnd`
// 进了 `card_select` 却没有发牌，于是场景永远停在第 1 波。
func (e *BattleEngine) beginCardSelect() {
	cards := rollWaveCards(e.Rng, e.unlockedSkillCards())
	e.Deck.SetHand(cards)
	e.emit(BattleEvent{Type: EventCardOffer, Score: int64(len(cards))})
}

// unlockedSkillCards 构造本波的技能抽牌池。
//
// # ⚠️ 池子来源是 **`cfg.equipped`（当前装备的技能）**，不是内容表
//
// 这看起来像「只能抽到自己已有的技能」的限制，但它是刻意的：
// 抽到一张**没装备**的技能卡会让 `applyCard` 去 `cfg.skills` 里查定义，
// 而 `cfg.skills` 是服务端内容表 —— 客户端的已解锁技能由
// 玩家进度决定，内容表里有不等于玩家有。
//
// ⚠️⚠️ 顺序必须是 `cfg.equipped` 的**数组顺序**，不能排序、不能用 map：
//
//	`rollSkillCards` 用 `rng.intn(len(actives))` 选牌，
//	而 `actives` 是池子**按顺序**过滤出来的 ——
//	顺序进 RNG，所以排序会改变抽取结果。
//
//	我第一版用 `cfg.Skills`（map）并**按 id 排序**，
//	结果抽出来的技能与 TS 完全对不上。
//
// ⚠️ 池子必须**包含被动技能**：`rollSkillCards` 自己过滤出主动的。
// 若这里先滤掉，被动技能就彻底不出现在池里，
// 而 `actives.length === 0` 早退分支会让
// 「玩家只装了被动」与「池子空了」变得不可区分。
func (e *BattleEngine) unlockedSkillCards() []Card {
	out := make([]Card, 0, len(e.Cfg.Equipped))
	for _, s := range e.Cfg.Equipped {
		out = append(out, Card{
			ID:      strconv.Itoa(s.SkillID),
			Name:    s.Name,
			Element: s.Element,
			Kind:    s.Kind, // active / passive
			SkillID: s.SkillID,
		})
	}
	return out
}

// recordPick 记一次选牌。
func (e *BattleEngine) recordPick(handIdx int) {
	if len(e.CardPicks) <= e.WaveIndex {
		e.CardPicks = append(e.CardPicks, handIdx)
	}
}

// 选牌编码基数（第 66 轮引入，进 `replay_hash`）。
//
// ⚠️ 编码约定两端必须逐字一致：
//
//	handIdx                      → 取第 handIdx 张（未弃过牌）
//	-1                           → 整波跳过（未弃过牌）
//	-(PickDiscardBase + handIdx) → 弃一张后取（-3..-5）
//	-PickDiscardSkipBase         → 弃一张后跳过（-6）
//
// 为什么要有「弃牌」这一档：卡牌由玩家在**运行时**选择，
// 重放方拿不到「玩家当时弃了几张」。
// 若只固定记 -1，判读侧就「既没看 discard 的 record 事件、
// 又没看 card_picks」，两种情况完全一样 ——
// 而 refundHeat(1) 与 record(...,1) 会让事件流不同、replayHash 不同。
const (
	PickDiscardBase     = 3
	PickDiscardSkipBase = 6
)

// SkipCards 丢光全部手牌，直接进下一波。
//
// ⚠️ 编码只记 `-PICK_DISCARD_SKIP_BASE`（不带 handIdx）：
//
//	弃牌的下标信息已经在 `DiscardCard` 的 record 事件里，
//	再编码一次就成了重复记账。
func (e *BattleEngine) SkipCards() {
	if e.Phase != PhaseCardSelect {
		return
	}
	encoded := -1
	if e.discardedThisWave {
		encoded = -PickDiscardSkipBase
	}
	e.recordPick(encoded)
	e.finishCardSelect()
}

// TakeCard 拿一张牌。
//
// ⚠️ 三处顺序不可调换（每一条都是第 66 轮踩出来的）：
//
//  1. `handIdx` 必须在 `take` **之前**取 —— 它是「玩家眼中的第几张」，
//     而 `card_picks` 记的是这个下标，不是卡种。
//  2. `applyCard` 在 `emit` **之前**：加成先落，再发事件，
//     这样事件驱动的 UI 读到的 `currentAttacker()` 已是加成后的值。
//  3. `record` 在 `recordPick` **之前**：
//     `record` 写的是 `cardIndex(card)`（卡种编码），
//     `recordPick` 写的是 `handIdx`（下标编码）。两者不是一个东西。
//
// ⚠️ `finishCardSelect` 而不是 `BeginWave`：
//
//	`finishCardSelect` 会先把剩余手牌 `drop` 掉再开波，
//	而开波时的 `401:wave` 事件必须落在**本 tick**。
//	直接 `beginWave` 的话，那条事件会晚一 tick（实测 DIFF@61 orig=402 repl=401）。
func (e *BattleEngine) TakeCard(id string) (Card, bool) {
	if e.Phase != PhaseCardSelect {
		return Card{}, false
	}
	handIdx := cardIndexInHand(e.Deck.Hand, id)
	if handIdx < 0 {
		return Card{}, false
	}
	c, ok := e.Deck.Take(id)
	if !ok {
		return Card{}, false
	}
	e.applyCard(c)
	e.emit(BattleEvent{Type: EventCardTaken, Text: c.Name})
	e.record(e.Tick, ReplayCard, cardIndex(c), 0, 0)
	if e.discardedThisWave {
		e.recordPick(-(PickDiscardBase + handIdx))
	} else {
		e.recordPick(handIdx)
	}
	if e.Deck.Size() == 0 {
		e.finishCardSelect()
	}
	return c, true
}

// DiscardCard 弃一张牌。
//
// ⚠️ 这里**不**写 card_picks：弃牌的下标由后续的
// `TakeCard`（`-(3+handIdx)`）或 `SkipCards`（`-6`）一并记。
// 弃牌自己只写一条 record 事件（flag=1），
// 那条事件负责「这一 tick 发生了什么」；
// `card_picks` 负责「这一波玩家做了什么决定」。两者职责不同。
func (e *BattleEngine) DiscardCard(id string) (Card, bool) {
	if e.Phase != PhaseCardSelect {
		return Card{}, false
	}
	idx := cardIndexInHand(e.Deck.Hand, id)
	if idx < 0 {
		return Card{}, false
	}
	captured := e.Deck.Hand[idx]
	r := e.Deck.Discard(id)
	if !r.OK {
		return Card{}, false
	}
	e.Heat.RefundHeat(r.Refund)
	e.emit(BattleEvent{Type: EventCardDiscarded, Text: captured.Name})
	e.record(e.Tick, ReplayCard, cardIndex(captured), 1, 0)
	e.discardedThisWave = true
	// ⚠️ 这里必须**直接** `beginWave`，不走 `finishCardSelect`：
	//   弃牌路径不写 card_picks，而 `finishCardSelect` 的语义是
	//   「丢弃剩余手牌并进下一波」—— 但弃完最后一张时手牌已经空了，
	//   两条路径的净效果一样，走 `beginWave` 更贴近 TS 且少一层。
	if e.Deck.Size() == 0 {
		e.BeginWave(e.WaveIndex + 1)
	}
	return captured, true
}

// finishCardSelect 丢掉剩余手牌并进入下一波。
//
// ⚠️ 必须**先复制再删**：
//
//	`Drop` 走的是 `take`，而 `take` 是 `splice` ——
//	在遍历中删除会让迭代器跳过元素，手牌**删不干净**。
//	症状是「跳过本波后，手牌里还剩 1 张」。
//	TS 那边的 `for (const c of [...this.deck.hand])` 里那个 `[...]`
//	正是同一个目的，不是随手写的。
//
// ⚠️ 这里**不能**为剩余手牌写 record 事件。
//
//	第一版写了，理由是「它们确实被丢弃了，应该记进事件流」。实测是**错的**：
//
//	  原局 ORIGCARD=401/30/1 401/506/0 **402/902/0** ...
//	  重放 REPLCARD=401/30/1 401/506/0 **401/902/1** ...
//	                                       ↑ tick 差 1 格、flag 差 1
//
//	根因：**原局根本没有「丢弃剩余手牌」这个动作**。
//	玩家取 1 张就离开选牌界面，剩下那 2 张一直躺在 `deck.hand` 里，
//	直到下一波的 `setHand` 直接覆盖 —— 全程没有任何事件。
func (e *BattleEngine) finishCardSelect() {
	e.dropAllHand()
	e.BeginWave(e.WaveIndex + 1)
}

// finishCardSelectNextTick 静默清空剩余手牌，**不**开下一波。
//
// 用于「弃一张 + 取一张 + 剩牌不动」的场景：
// `takeCard` 里 `deck.size !== 0` 所以不会开波，
// 剩下的牌要等**下一 tick** `step()` 开头再开波。
//
// ⚠️ 走这里而不是 `finishCardSelect`：
//
//	`finishCardSelect` 会**立刻** `beginWave`，
//	于是 `step()` 开头那一段（`if (this.cardSelectDone) {...}`）
//	还没跑到就被跳过了 ——
//	波形事件会落在**本 tick**，而原局落在下一 tick。
//	实测 `DIFF@61 orig=728 repl=727`。
func (e *BattleEngine) finishCardSelectNextTick() {
	e.dropAllHand()
	e.cardSelectDone = true
}

// dropAllHand 丢掉手牌里全部的牌。
//
// ⚠️ 必须**先复制再删**：`Deck.Drop` 走 `take`（= splice），
//
//	在遍历中删除会跳过元素，手牌删不干净 ——
//	症状是「跳过本波后手牌里还剩 1 张」，而下一波 `setHand` 直接覆盖它，
//	于是这个多出来的牌在跨端比对里表现为「某一刻 hand 比 TS 多一张」。
func (e *BattleEngine) dropAllHand() {
	snapshot := make([]Card, len(e.Deck.Hand))
	copy(snapshot, e.Deck.Hand)
	for _, c := range snapshot {
		e.Deck.Drop(c.ID)
	}
}

// applyReplayDecision 消费选牌脚本。
//
// # ⚠️ 必须**每个 tick 最多消费一项**
//
// 原实现把整个脚本在**同一 tick** 内消费完，
// 于是第 1 波的 picks 被记成 `[-3,-3,-1,-1]` ——
// 那是**两波**的决策，而事件流多出 `402/902/0`
// 这条原局没有的记录，`replayHash` 立刻不同。
//
// # ⚠️「弃一张 + 取一张」要 `finishCardSelectNextTick` + `return`
//
// 玩家「弃 1 张、拿 1 张、剩下 2 张不动」时 `deck.size !== 0`，
// 于是 `takeCard` 里的 `size == 0` 分支不成立，phase 停在 card_select；
// 剩下那 2 张要等**下一 tick** 玩家离开界面时才被丢弃。
//
// 直接 `continue` 会让 while 循环在同一 tick 内消费脚本下一项，
// 把**一整波**的决策塞进第二项，于是 picks 又被记成 `[-3,-3,-1,-1]`。
func (e *BattleEngine) applyReplayDecision() {
	// guard 防死循环：万一 `SkipCards` 没能离开 card_select。
	guard := 0
	for e.Phase == PhaseCardSelect && guard < 16 {
		guard++
		if e.ReplayScript == nil {
			// `script === null` 是 true 分支：**人工接管**。
			// `Step` 只在 `ReplayScript != nil` 时调本方法，
			// 而原局没有脚本时 `setReplayScript(null)` 一直为 null。
			return
		}
		if e.ReplayScriptPos >= len(e.ReplayScript) {
			e.SkipCards()
			continue
		}
		want := e.ReplayScript[e.ReplayScriptPos]
		e.ReplayScriptPos++

		switch {
		case want <= -PickDiscardSkipBase:
			// 「弃一张后跳过」
			if len(e.Deck.Hand) > 0 {
				e.DiscardCard(e.Deck.Hand[0].ID)
			}
			// recordPick 已在 discard 那一步的分支之外，
			// skipCards 那一记靠 `len <= WaveIndex` 只生效一次 —— 必须 return。
			e.SkipCards()
			return

		case want <= -PickDiscardBase:
			// 「弃一张 + 取一张」：
			// 弃牌与取牌**各自发自己的 record 事件**，与原局同源。
			if len(e.Deck.Hand) > 0 {
				e.DiscardCard(e.Deck.Hand[0].ID)
			}
			rest := -(want + PickDiscardBase)
			if rest < 0 || rest >= len(e.Deck.Hand) {
				e.SkipCards()
				continue
			}
			e.TakeCard(e.Deck.Hand[rest].ID)
			e.finishCardSelectNextTick()
			return

		case want < 0 || want >= len(e.Deck.Hand):
			// -1 或越界 → 整波跳过。
			//
			// ⚠️ 越界也走这里（而不是取最后一张）：
			//   取牌会发 record + refundHeat，
			//   与脚本的 `replayHash` 立刻不符（I-6 失败过）。
			e.SkipCards()
			continue
		}

		// ⚠️ 范围已在上面那个 case 里判过（want < 0 || want >= len(Hand)），
		// TakeCard 内部可能已开下一波；也可能没开（剩牌未清），
		// 那就交给 while 的下一轮 —— 但那时 phase 已不是 card_select，
		// 循环自然退出。
	}
}

// cardIndex 返回卡在 record 事件里的编号。
//
// ⚠️ **不是** `card_picks` 里的下标 —— 两套编码，别混：
//
//	skill      → skillId * 10
//	attribute  → 500 + attributeIndex(effect.kind)
//	mechanic   → 900 + mechanicIndex(mechanic.kind)
//
// 它只被 `record()` 用，而 record 的「值」进 `replay_hash`。
func cardIndex(c Card) int64 {
	switch c.Kind {
	case CardKindSkill:
		return int64(c.SkillID) * 10
	case CardKindAttribute:
		return 500 + attributeIndex(c.Effect.Kind)
	default:
		return 900 + mechanicIndex(c.Mechanic.Kind)
	}
}

// attributeIndex 是属性卡在其池子里的下标（1 起）。
//
// ⚠️ 顺序必须与 `attributePool` **逐项一致**：
// 顺序错一位会让每张属性卡的 record 值都错，
// 而表现是「replayHash 不对」—— 定位起来毫无线索。
func attributeIndex(kind string) int64 {
	for i, c := range attributePool {
		if c.Effect.Kind == kind {
			return int64(i + 1)
		}
	}
	return 0
}

// mechanicIndex 是机制卡在其池子里的下标（1 起）。
func mechanicIndex(kind string) int64 {
	for i, c := range mechanicPool {
		if c.Mechanic.Kind == kind {
			return int64(i + 1)
		}
	}
	return 0
}

// cardIndexInHand 找卡在手牌里的下标，找不到返回 -1。
//
// ⚠️ 调用点必须自己判 `idx < 0`：
//
//	让本函数返回 0 当「未找到」会让 `DiscardCard` 弃掉**第一张**而不是报错 ——
//	症状是「玩家点弃牌，弃掉的却是别的卡」。
func cardIndexInHand(hand []Card, id string) int {
	for i := range hand {
		if hand[i].ID == id {
			return i
		}
	}
	return -1
}

// applyCard 应用一张牌的效果。
func (e *BattleEngine) applyCard(c Card) {
	switch {
	case c.Kind == CardKindAttribute && c.Effect.Kind != "":
		e.applyAttribute(c.Effect)
	case c.Kind == CardKindMechanic && c.Mechanic.Kind != "":
		e.applyMechanic(c.Mechanic)
	case c.Kind == CardKindSkill && c.hasSkillID:
		e.applySkillCard(c)
	}
}

// applyAttribute 应用属性卡。
//
// ⚠️ `heat_cap` **只**写 `Heat.CapBonus`，不要在 Buffs 里再存一份：
//
//	被 `Heat.Cap()` 读走的是 CapBonus；Buffs 那份会成为**只写不读的镜像字段**。
//	危害不是「多占内存」，而是**陷阱**：实际没人维护的值，
//	而全绿的测试不会提示任何异常。
func (e *BattleEngine) applyAttribute(ef AttributeEffect) {
	switch ef.Kind {
	case "attack":
		e.Buffs.AttackPermille += ef.Value
	case "element_coef":
		e.Buffs.ElementCoefPermille += ef.Value
	case "crit":
		e.Buffs.CritPermille += ef.Value
	case "heat_cap":
		e.Heat.CapBonus += ef.Value
	case "element_cap":
		e.Buffs.ElementCapBonus += ef.Value
	case "armor":
		e.Buffs.ArmorPermille += ef.Value
	}
}

// applyMechanic 应用机制卡。
//
// 机制卡强度加成（专精树第 3 层「机制改造」）。
// 语义：把卡面数值整体放大 (1000 + mechanicPermille)/1000。
//
// ⚠️ 选「放大卡面数值」而不是「抽到好卡的概率更高」：
//
//	前者是确定性的（重放能逐位复现），
//	后者要改抽牌随机序列，而卡牌是客户端选的，
//	改序列会与 `card_picks` 闭环冲突。
func (e *BattleEngine) applyMechanic(m MechanicEffect) {
	// ⚠️ 先取整再进定点域：内容表面值全是整数，
	// 取整不丢精度；真出现小数时是内容表错了。
	v := mulDiv(int64(m.Value), permille+e.Cfg.Attacker.MechanicPermille, permille)
	switch m.Kind {
	case "pierce_bonus":
		e.Buffs.PierceBonus += int(v)
	case "chain_bonus":
		e.Buffs.ChainBonus += int(v)
	case "aoe_bonus":
		e.Buffs.AoeBonus += int(v)
	case "free_discard":
		e.Buffs.FreeDiscard += int(v)
		// ⚠️ **同时**加到当前剩余次数上：
		// 只加 Buffs 的话，本波的 `discardsLeft` 不会因为新卡而 +1，
		// 玩家拿到「快速装填」却仍然只能用默认次数。
		e.Deck.DiscardsLeft += int(v)
	case "overheat_guard":
		e.Buffs.OverheatGuard += int(v)
	}
}

// applySkillCard 应用技能卡：已装备则升格，否则装入空槽。
//
// ⚠️ 判据用 `HasSkillID`（skill_id **出现过**）而不是 `SkillID != 0`：
//
//	`skillId === 0` 曾落进最后的隐式 else ——
//	卡被 consume 掉、emit 了 card_taken，但什么也没发生 ——
//	玩家看到「获得技能卡」却没反应。
//	内容表当前 id 从 1 开始所以不触发，但 id 空间没有「必须非 0」的保证。
func (e *BattleEngine) applySkillCard(c Card) {
	existing := -1
	for i := range e.Skills {
		if e.Skills[i].SkillID == c.SkillID {
			existing = i
			break
		}
	}
	if existing >= 0 {
		e.upgradeSkill(e.Skills[existing].Slot)
		return
	}

	def, ok := e.Cfg.Skills[c.SkillID]
	if !ok {
		return
	}
	// 找**真正空着**的主动槽。
	//
	// ⚠️ 曾经写成「找第一个 `s.slot < activeSlots && s.slot >= 0`」，
	//
	//	而 skills 数组里每个元素就代表一个**已占用**的槽 ——
	//	于是这个表达式命中的永远是第一个**已占用**的槽，
	//	随后直接把它替换掉。
	//	后果：抽到任何**新**技能都会顶掉槽 0 的老技能，槽位数永远不增加。
	//	实测已装 [1,2] 时抽到技能 9：before=[1,2] -> after=[9,2]，
	//	技能 1 被销毁，而玩家的三选一白白消耗了一次机会。
	used := make(map[int]bool, len(e.Skills))
	for _, s := range e.Skills {
		if s.Slot < e.ActiveSlots {
			used[s.Slot] = true
		}
	}
	free := -1
	for i := 0; i < e.ActiveSlots; i++ {
		if !used[i] {
			free = i
			break
		}
	}
	if free < 0 {
		// 主动槽已满：升格一个伤害最高的技能（不消耗玩家的选牌机会）。
		//
		// 槽满时最合理的处理不是「顶掉随机一个」，
		// 而是明确地告诉玩家「没位置了」—— 所以这里做的是
		// 「强化已有技能里伤害最高的那一个」，
		// 至少不会让玩家的构筑凭空少一个技能。
		best := -1
		for i := range e.Skills {
			if e.Skills[i].Slot >= e.ActiveSlots {
				continue
			}
			if best < 0 || e.Skills[i].BaseDamage > e.Skills[best].BaseDamage {
				best = i
			}
		}
		if best >= 0 {
			e.upgradeSkill(e.Skills[best].Slot)
		}
		return
	}
	applyElement := def.ApplyElement
	if applyElement == "" {
		applyElement = def.Element
	}
	e.Skills = append(e.Skills, EquippedSkill{
		SkillID:           def.ID,
		Name:              def.Name,
		Element:           def.Element,
		Kind:              def.Kind,
		HeatCost:          def.HeatCost,
		CooldownMs:        def.CooldownMs,
		Pierce:            int(def.Pierce),
		AoeRadius:         def.AoeRadius,
		BaseDamage:        def.BaseDamage,
		ApplyElement:      applyElement,
		ApplyStacks:       def.ApplyStacks,
		ProjectileSpeed:   def.ProjectileSpeed,
		Chain:             int(def.Chain),
		Slot:              free,
		CooldownRemaining: 0,
	})
}

// upgradeSkill 升格槽上的技能：更高伤害 + 更高层数 + 更高穿透。
//
// ⚠️ `baseDamage / 5` 是**整数除法**，向零截断 ——
// 所以小数值技能（baseDamage < 5）会**一点伤害都不涨**。
// 这是 TS 的同款行为，不是在这里引入的；
// 但它意味着「升格低级技能不划算」在低等级时成立。
func (e *BattleEngine) upgradeSkill(slot int) {
	for i := range e.Skills {
		if e.Skills[i].Slot != slot {
			continue
		}
		s := &e.Skills[i]
		s.BaseDamage += s.BaseDamage / 5
		s.ApplyStacks++
		s.Pierce++
		return
	}
}
