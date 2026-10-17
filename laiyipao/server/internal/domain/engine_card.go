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
			// ⚠️⚠️ **必须**在这里置位，否则技能卡永远落不进 `applyCard` 的技能分支。
			//
			// `applyCard` 的判据是 `c.Kind == CardKindSkill && c.hasSkillID`
			// （对应 TS 的 `card.skillId !== undefined`），而 `hasSkillID`
			// 是**非导出**字段：除了 JSON 反序列化，只有本函数会置它。
			//
			// 我第一版忘了置 —— 后果是**每一张技能卡都是哑弹**：
			// 卡被 `Deck.Take` 取走、`emit(card_taken)`、`record(card)`
			// 全都照做（所以 replay 事件流看起来完全正常），
			// 但 `applyCard` 一个分支都没进，技能一次都没升格。
			//
			// 症状之所以极难定位，是因为它同时踩中了三个「看起来无关」的巧合：
			//
			//  1. `card_picks` 与 `card` 的 replay 记录都正常（取牌动作是真的）
			//  2. `terrain_used` / `kills` / `waves` 都正常（只是伤害低了一档）
			//  3. B~F 的 6 个卡牌场景用的全是 **-3** 编码 =「弃一张 + 取一张」，
			//     取到的是**弃牌之后**的第 0 张（属性卡）——
			//     **从来没有一个场景取过纯技能卡**
			//
			// 而我那批 `TestApplySkillCard*` 单测全都**手工构造**了
			// `Card{..., hasSkillID: true}`，于是它们验的是
			// 「`applySkillCard` 对不对」，而不是「deck 里的卡能不能走到那里」。
			hasSkillID: true,
			Tier:       1,
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
	// ⚠️ 第 159 轮：每次发牌重置本波决策状态。
	//  - discardedMask：本波已弃牌的位掩码（原手牌下标），进 card_picks 编码
	//  - handSnapshot：原手牌 id 快照，把「弃了哪张」映射回原下标
	e.discardedMask = 0
	cards := rollWaveCards(e.Rng, e.unlockedSkillCards())
	snapshot := make([]string, len(cards))
	for i, c := range cards {
		snapshot[i] = c.ID
	}
	e.handSnapshot = snapshot
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

// card_picks 的「弃过牌」负编码基址（第 66 轮引入，第 159 轮重写）。
//
// 取值语义（与 TS `PICK_CODE_BASE` 逐位一致）：
//
//	0/1/2        普通取牌（无弃牌），下标是取牌当次的手牌下标
//	-1           整波跳过（无弃牌）
//	-(3+code)    本波弃过牌；code ∈ [0, PickCodeMax]，见 decodeDiscardCode
//
// # 为什么重写（第 159 轮）
//
// 旧编码（-3..-5 弃+取、-6..-8 弃+跳）表达不了两件**正常玩法**：
//
//  1. **弃的是哪一张** —— 重放侧固定弃 hand[0]，
//     而线上 UI 允许弃任意一张；弃了非首张的玩家必然失配。
//  2. **弃多于一张** ——「快速装填」（每波弃牌次数 +1）
//     让 discardsLeft > 1 成为正常玩法。
//
// 新编码把弃牌集合写成**手牌位掩码**：一个 code 同时携带
// 「弃了原手牌的哪几张」+「（不弃完时）取剩余第几张 / 跳过」。
//
// # 枚举（两端必须逐位一致，由 card_picks_contract 测试锁死）
//
// 按 mask 升序（bit i = 原手牌第 i 张被弃）：
//
//	mask       剩余    选择数     code
//	0 (不弃)   3 张    4          0..3
//	1 (弃0)    2 张    3          4..6
//	2 (弃1)    2 张    3          7..9
//	3 (弃0,1)  1 张    2          10..11
//	4 (弃2)    2 张    3          12..14
//	5 (弃0,2)  1 张    2          15..16
//	6 (弃1,2)  1 张    2          17..18
//	7 (弃全)   0 张    1          19
//
// 每个 mask 内部按 取剩余第 0 张、第 1 张、…、不取 排列。
// 下界 `PickMin = -(3+19) = -22`（服务端 `CardPickMin` 同值）。
const (
	PickCodeBase = 3
	PickCodeMax  = 19
)

// PickMin 是 card_picks 条目的合法取值下界（= battle_collections.go 的 CardPickMin）。
const PickMin = -(PickCodeBase + PickCodeMax) // = -22

// pickPopcount 是 3 位掩码的 popcount。
func pickPopcount(m int) int { return (m & 1) + ((m >> 1) & 1) + ((m >> 2) & 1) }

// pickChoiceCount 是 mask 下「选择」的个数：取剩余每张各一种 + 不取。
func pickChoiceCount(mask int) int { return 3 - pickPopcount(mask) + 1 }

// encodeDiscardCode 把（弃牌掩码, 选择）编码成 code。
//
// take ∈ [0, 剩余张数-1] 表示「取剩余的第 take 张」（剩余手牌保持原序）；
// take = -1 表示不取（跳过）。
func encodeDiscardCode(mask, take int) int {
	code := 0
	for m := 0; m < mask; m++ {
		code += pickChoiceCount(m)
	}
	if take < 0 {
		return code + pickChoiceCount(mask) - 1
	}
	return code + take
}

// decodeDiscardCode 解码 code，返回 (mask, take, ok)；越界返回 ok=false。
//
// take = -1 表示不取（跳过）。
func decodeDiscardCode(code int) (int, int, bool) {
	if code < 0 || code > PickCodeMax {
		return 0, 0, false
	}
	for mask := 0; mask < 8; mask++ {
		n := pickChoiceCount(mask)
		if code < n {
			take := code
			if take == n-1 {
				take = -1
			}
			return mask, take, true
		}
		code -= n
	}
	return 0, 0, false
}

// indexOfString 是 []string 的 indexOf（找不到返回 -1）。
//
// 用于 `handSnapshot` 查「这张牌在本波原手牌里的下标」。
func indexOfString(xs []string, s string) int {
	for i := range xs {
		if xs[i] == s {
			return i
		}
	}
	return -1
}

// SkipCards 丢光全部手牌，直接进下一波。
//
// -1 = 整波跳过（无弃牌）；弃过牌时记位掩码编码（第 159 轮）——
// 重放侧靠它复现那次/那几次 RefundHeat 与手牌变化。
func (e *BattleEngine) SkipCards() {
	if e.Phase != PhaseCardSelect {
		return
	}
	e.recordPick(e.encodeCurrentPick(-1))
	e.finishCardSelect()
}

// TakeCard 拿一张牌（取牌即结束本波选牌；第 159 轮）。
//
// ⚠️ 三处顺序不可调换：
//
//  1. `handIdx` 必须在 `take` **之前**取 —— 它是「玩家眼中的第几张」，
//     而 `card_picks` 记的是这个下标，不是卡种。
//  2. `applyCard` 在 `emit` **之前**：加成先落，再发事件，
//     这样事件驱动的 UI 读到的 `currentAttacker()` 已是加成后的值。
//  3. `record` 在 `recordPick` **之前**：
//     `record` 写的是 `cardIndex(card)`（卡种编码），
//     `recordPick` 写的是手牌下标编码。两者不是一个东西。
//
// ⚠️ 第 159 轮：取牌**总是**结束本波选牌。
//
//	改前只有取空手牌才结束，于是 UI 允许一波连点多张卡 ——
//	而 card_picks 每波只记一项，第二张起在重放里必然对不上
//	（线上 100% 结算被拒的根因之一）。设计本义（I-2）是「三选一」，
//	剩余手牌静默丢弃（与 `finishCardSelect` 同路径，不写事件）。
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
	e.recordPick(e.encodeCurrentPick(handIdx))
	e.finishCardSelect()
	return c, true
}

// encodeCurrentPick 把当前决策（discardedMask + 取牌下标/跳过）编码成 card_picks 项。
//
// `take = -1` 表示跳过。没弃过牌时保持旧值域（0/1/2 与 -1），
// 保证最常见打法的编码与他端历史行为逐位一致。
func (e *BattleEngine) encodeCurrentPick(take int) int {
	if e.discardedMask == 0 {
		return take
	}
	return -(PickCodeBase + encodeDiscardCode(e.discardedMask, take))
}

// DiscardCard 弃一张牌（第 159 轮重写记录语义）。
//
// ⚠️ 弃牌**不再写 record 事件**（旧实现写 flag=1 的 card 事件）。
//
//	那条事件的用途是让重放复现这次弃牌 —— 但「弃了哪张、弃了几张」
//	现在由 card_picks 的位掩码编码携带（见 `PickCodeBase`），
//	弃牌的全部副作用（RefundHeat / 次数 / 手牌变化）在重放侧由
//	解码路径复现。事件流里不再出现弃牌，于是**弃牌顺序也不进哈希**
//	（旧实现下「先弃 A 再弃 B」与「先弃 B 再弃 A」哈希不同，
//	而玩家点击顺序本来就不是语义）。
//
// ⚠️ 这里**不**写 card_picks：掩码与「取/跳」由后续的
// `TakeCard` / `SkipCards` 一并记（`encodeCurrentPick`）。
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
	// 记录到本波掩码：手牌在本次 offer 里的**原下标**（快照里查 id）
	if i := indexOfString(e.handSnapshot, captured.ID); i >= 0 {
		e.discardedMask |= 1 << i
	}
	if e.Deck.Size() == 0 {
		e.finishCardSelect()
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

// applyReplayDecision 消费选牌脚本（第 159 轮重写）。
//
// # 语义：**每个 card_select 恰好消费一项脚本**
//
// 线上玩家的决策被冻结在同一个 tick 上（见 `Step` 的时钟冻结说明），
// 重放同样在本 tick 内完成一次完整决策（弃牌掩码 + 取/跳）。
// 旧实现允许合成脚本在同一 tick 连点消费多项 —— 而线上真实玩家
// 做不到 50ms 内点三张卡，那个语义从来就不存在。
//
// 脚本耗尽或编码越界时回退为「跳过整波」，保证重放一定能跑到底
// （宁可哈希不符，也不要中途卡死让用户看到空白页）。
func (e *BattleEngine) applyReplayDecision() {
	if e.ReplayScript == nil {
		// 人工接管：`Step` 只在 `ReplayScript != nil` 时调本方法，
		// 这里保留是类型收窄（nil 排除）用。
		return
	}
	if e.ReplayScriptPos >= len(e.ReplayScript) {
		e.SkipCards()
		return
	}
	want := e.ReplayScript[e.ReplayScriptPos]
	e.ReplayScriptPos++

	// 负编码（第 159 轮）：-(3+code) = 弃牌掩码 + 取/跳
	if want <= -PickCodeBase {
		mask, take, ok := decodeDiscardCode(-(want + PickCodeBase))
		if !ok {
			e.SkipCards()
			return
		}
		// 复现弃牌：按**原手牌顺序**（顺序不进哈希 —— 弃牌不写事件，
		// 热量返还与手牌状态只与「弃哪几张」有关）。
		//
		// ⚠️ 与 TS 一致：额度不足时 `DiscardCard` 静默失败
		//（`Deck.Discard` 判 `DiscardsLeft`），后续取牌照常 ——
		// 两端必须产生同样的降级结果，否则哈希不同。
		for i := 0; i < 3; i++ {
			if mask&(1<<i) == 0 {
				continue
			}
			if i >= len(e.handSnapshot) {
				continue
			}
			id := e.handSnapshot[i]
			if cardIndexInHand(e.Deck.Hand, id) >= 0 {
				e.DiscardCard(id)
			}
		}
		if take < 0 {
			e.SkipCards()
			return
		}
		if take >= len(e.Deck.Hand) {
			e.SkipCards()
			return
		}
		e.TakeCard(e.Deck.Hand[take].ID)
		return
	}

	// 普通取牌：手牌顺序可能与原局不同（不同 seed 的洗牌结果），
	// 因此按**索引**取，而不是按卡牌 id —— 索引才是脚本记录的语义。
	if want >= 0 {
		if want >= len(e.Deck.Hand) {
			e.SkipCards()
			return
		}
		e.TakeCard(e.Deck.Hand[want].ID)
		return
	}

	// -1 与其余非法负值（如 -2）→ 整波跳过
	e.SkipCards()
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
