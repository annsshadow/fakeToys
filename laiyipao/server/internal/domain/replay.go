package domain

// 整局重放与结算重算（R1 Block G）。
//
// # 这一块要回答的问题
//
// README 记的最大限制是「A 段（攻方系数锚点）含局内 buff，服务端算不出」。
// 本块证明：**`card_picks`（选牌脚本）确定 ⇒ 局内 buff 确定 ⇒ A 段可算**。
//
// 前提是 `BuildSnapshot`（构筑快照）在**开局时**就存进了 `battle_records`，
// 它含攻方的 10 项系数；而选牌脚本也在同一行里（`card_picks`）。
// 两者合起来，服务端就掌握了重算所需的全部输入。
//
// # ⚠️ 「重放」与「结算重算」是同一条路径
//
// 不是「先重放再校验」的两步，而是**同一个函数**：
// 服务端拿存档里的 `build_snapshot` + `card_picks` + `seed` 重跑一遍，
// 拿 `replay_hash` 与存档里记录的 `replay_hash` 比。
// 不符即拒发奖励。
//
// 这也是为什么本文件的每个字段都标了「两端必须逐字一致」——
// 差一位就是「合法战报被判伪造」，或者更糟：
// 「伪造战报被判合法」。

import (
	"errors"
	"sort"
	"strconv"
	"strings"
)

// MaxTicksPerReplay 单局推进上限，防止异常关卡把进程卡死。
//
// ⚠️ 必须**大于** `MaxBattleTicks`：
//
//	引擎的停滞兜底在 tick 到 `MaxBattleTicks` 时强制 finish(lost)，
//	所以循环必从内部 won/lost 提前 return ——
//	`36000 > 30000` 的余量就是安全阀本身。
//	若两者调换，循环的 false 退出分支变成**可达**，
//	而那条分支上引擎还没终局，哈希会算出一个「没打完」的中间态。
const MaxTicksPerReplay = 60 * 60 * 10

// BuildSnapshotSkill 是构筑快照里的一个技能槽。
//
// ⚠️ `Level` 是**必填**的，且它会被烘进 `BaseDamage`。
//
//	`replayHash` 的 S 段锚点是 `slot:skillId:baseDamage:applyStacks:heatCost`。
//	等级若不烘进去，两个不同等级的构筑可能算出同一份哈希 ——
//	**验真通过但伤害对不上**，哈希在说谎。
//	与 `active_slots` 缺省必须是 4 同一个道理：
//	凡是能改变战斗结果的输入，都必须落在哈希锚点里。
type BuildSnapshotSkill struct {
	ID      int     `json:"id"`
	Name    string  `json:"name"`
	Family  string  `json:"family"`
	Element Element `json:"element"`
	Kind    string  `json:"kind"`
	Level   int     `json:"level"`
	Slot    int     `json:"slot"`
}

// BuildSnapshotAttacker 是服务端**权威下发**的攻方属性。
//
// ⚠️ 绝不能本地推算。重放哈希由「关卡 + 种子 + 攻方属性 + 技能」共同决定，
// 两端各算一套的话，哈希永远对不上。
type BuildSnapshotAttacker struct {
	Attack                 int64 `json:"attack"`
	CritPermille           int64 `json:"crit_permille"`
	CritMultiplierPermille int64 `json:"crit_multiplier_permille"`
	ReactionMultPermille   int64 `json:"reaction_mult_permille"`
	ElementCap             int64 `json:"element_cap"`
	ReactionTier           int64 `json:"reaction_tier"`
	ElementCoefPermille    int64 `json:"element_coef_permille"`
	// ⚠️ 这三项缺省 0，是**有意的**：老战报的 build_snapshot 里没有它们
	// （那时它们根本不存在），而「缺失 = 0」正好等价于「没有装备/宝石/专精」。
	// 填任何非 0 的缺省都会让老战报重放出的哈希与当年记录的不符。
	HeatCapPermille int64 `json:"heat_cap_permille"`
	ArmorPermille   int64 `json:"armor_permille"`
	// ⚠️ 缺省**不是** 1000，而是 **0**，与前两项一致。
	//
	// 我第一版在这里写了「缺省 1000（= 100% 卡面）」，理由是
	// 「缺 0 会让『机制改造』专精树整层失效」—— 那个推理是错的：
	// 缺 0 只意味着「没有专精加成」，而**没有加成的玩家本来就不该有加成**；
	// 填 1000 反而会让所有**老战报**（`mechanic_permille` 字段还不存在时记录的）
	// 重放出「卡面被放大 2 倍」的机制卡，与当年记录不符。
	//
	// TS 那边是 `BigInt(a.mechanic_permille ?? 0)`，逐字对应。
	MechanicPermille int64 `json:"mechanic_permille"`
}

// BuildSnapshot 是开局时刻冻结的构筑输入。
//
// ⚠️ 它必须是**开局时**的快照，不是结算时的当前值：
//
//	`applyCard` 会替换 `this.skills`（`map` 出一个新数组），
//	所以「结算时读当前技能」拿到的是 `0:1:120:2:20`，
//	而存档里记的是 `0:1:100:1:20` ——
//	症状是**所有老战报都无法重放**，报错却是 `ErrReplaySkillsMismatch`（422）。
type BuildSnapshot struct {
	// Skills 是**按 id 字符串**索引的 map，顺序不定。
	//
	// ⚠️ 所以 `EquippedFromSnapshot` 必须显式排序 ——
	// `sort.Strings` 对数字字符串会给出字典序（"10" < "2"），
	// 而槽位语义上是数值序。TS 那边也用字典序（见下），
	// 所以**必须照抄 TS 的排序规则**，不能"修正"它。
	Skills      map[string]BuildSnapshotSkill `json:"skills"`
	Attacker    *BuildSnapshotAttacker        `json:"attacker"`
	ActiveSlots int                           `json:"active_slots"`
}

// EquippedFromSnapshot 由构筑快照还原装备的技能列表。
//
// # 这是 `EquippedSkill` 的**唯一**构造点
//
// 此前 `pages/battle/battle.vue` 与 `replay.ts` 各有一份**逐行复制**的本函数
// —— 实战开局自己拼一遍、重放再拼一遍。
//
// 那是本项目「战斗层与重放层各读各的」的典型形状：
// 两份都自洽，改一处忘另一处，
// 症状是「实战哈希与重放哈希永远对不上」，
// 而排查方向会自然滑向「I-6 坏了」而不是「构造逻辑有两份」。
//
// # ⚠️ 三条不可省的规则
//
//  1. 按 `slot` 再按 `id` 升序（`sort(a.slot - b.slot || a.id - b.id)`）。
//     顺序进 RNG（`rollSkillCards` 的 `rng.intn(actives)`），
//     不排序的话同一个种子会抽出不同的技能。
//  2. `slot < 0` 跳过（未放入槽位）。
//  3. 内容表里查不到的技能**跳过而不是报错** ——
//     技能下架后老战报仍要能重放，否则历史战报集体失效。
func EquippedFromSnapshot(build BuildSnapshot, skills map[int]SeedSkill) []EquippedSkill {
	list := make([]BuildSnapshotSkill, 0, len(build.Skills))
	for _, s := range build.Skills {
		list = append(list, s)
	}
	// ⚠️ 排序必须是**稳定**的：同 slot 同 id 时不能随机换序。
	sort.SliceStable(list, func(i, j int) bool {
		if list[i].Slot != list[j].Slot {
			return list[i].Slot < list[j].Slot
		}
		return list[i].ID < list[j].ID
	})

	rules := DefaultSkillRules()
	out := make([]EquippedSkill, 0, len(list))
	for _, s := range list {
		if s.Slot < 0 {
			continue
		}
		def, ok := skills[s.ID]
		if !ok {
			continue // 技能已下架，跳过而不是报错
		}
		applyElement := def.ApplyElement
		if applyElement == "" {
			applyElement = def.Element
		}
		out = append(out, EquippedSkill{
			SkillID:    def.ID,
			Name:       def.Name,
			Element:    def.Element,
			Kind:       def.Kind,
			HeatCost:   def.HeatCost,
			CooldownMs: def.CooldownMs,
			Pierce:     int(def.Pierce),
			AoeRadius:  def.AoeRadius,
			// ⚠️ 等级在这里烘进 baseDamage，见 BuildSnapshotSkill.Level 的说明。
			BaseDamage:      SkillBaseDamageAtLevel(rules, def.BaseDamage, s.Level),
			ApplyElement:    applyElement,
			ApplyStacks:     def.ApplyStacks,
			ProjectileSpeed: def.ProjectileSpeed,
			Chain:           int(def.Chain),
			Slot:            s.Slot,
		})
	}
	return out
}

// AttackerFromSnapshot 由构筑快照还原攻方属性。
//
// ⚠️ 缺省值必须与 TS **逐字一致**，否则老战报的哈希会变。
//
//	前 7 项的缺省来自 TS 的 `?? 默认`；
//	最后 3 项见 BuildSnapshotAttacker 的字段注释。
func AttackerFromSnapshot(build BuildSnapshot) (Attacker, error) {
	a := build.Attacker
	if a == nil {
		return Attacker{}, errors.New(
			"构筑快照不含 attacker 字段，无法还原攻方属性。" +
				"（服务端需在 build_snapshot 里下发攻击力/元素系数等系数）")
	}
	return Attacker{
		Attack:                 a.Attack,
		CritPermille:           orDefault(a.CritPermille, 50),
		CritMultiplierPermille: orDefault(a.CritMultiplierPermille, 1500),
		ReactionMultPermille:   orDefault(a.ReactionMultPermille, 1000),
		ElementCap:             orDefault(a.ElementCap, 3),
		ReactionTier:           orDefault(a.ReactionTier, 1),
		ElementCoefPermille:    orDefault(a.ElementCoefPermille, 1000),
		HeatCapPermille:        a.HeatCapPermille,
		ArmorPermille:          a.ArmorPermille,
		MechanicPermille:       a.MechanicPermille,
	}, nil
}

func orDefault(v, def int64) int64 {
	if v == 0 {
		return def
	}
	return v
}

// ReplayHash 算本局的结算重放哈希。
//
//	hex16(fnv1a64("L{id}|A{7 项攻方系数}|S{槽位技能段}|X{额外血量};事件流"))
//
// ⚠️ A 段只有 **7 项**（attack / crit / critMul / reactionMult /
// elementCap / reactionTier / elementCoef），
// `heatCapPermille` / `armorPermille` / `mechanicPermille` **不在里面**。
//
//	前两者不进前缀是因为它们的消费者不在伤害公式里
//	（`heat.capBonus` 被 `Heat.Cap()` 单独读、`armorPermille` 走
//	`defenseArmorPermille()`），所以两者本局内恒定，
//	而**本局内变化的部分由事件流覆盖**。
//
// ⚠️ 事件流逐条为 `"{tick}|{类型}|{a}|{b}|{c}"`，以 `;` 连接。
// 分隔符用 `;` 而不是换行：事件里不会出现换行，
// 而 `;` 在事件字段取值集合里也不可能出现。
func (e *BattleEngine) ReplayHash() string {
	a := e.currentAttacker()
	prefix := "L" + strconv.Itoa(e.Cfg.Level.ID) +
		"|A" + strconv.FormatInt(a.Attack, 10) + "." +
		strconv.FormatInt(a.CritPermille, 10) + "." +
		strconv.FormatInt(a.CritMultiplierPermille, 10) + "." +
		strconv.FormatInt(a.ReactionMultPermille, 10) + "." +
		strconv.FormatInt(a.ElementCap, 10) + "." +
		strconv.FormatInt(a.ReactionTier, 10) + "." +
		strconv.FormatInt(a.ElementCoefPermille, 10) +
		"|S" + e.ReplaySkillsSegment() +
		"|X" + strconv.FormatInt(e.Cfg.ExtraBaseHp, 10)

	// ⚠️ 未定义时写 `|X0` 而不是省略整段：
	//
	//	省略会与「前缀里忘了写这段」不可区分，
	//	而漏掉整段只会让 hash 不同，不会产生「看起来对但结果错」的情况。
	//	前缀里多出锚点分隔符的鲁棒性 > 前缀更短。
	parts := make([]string, 0, len(e.replay)+1)
	parts = append(parts, prefix)
	for _, r := range e.replay {
		parts = append(parts, r.String())
	}
	return hex16(fnv1a64(strings.Join(parts, ";")))
}

// ReplaySkillsSegment 拼当前槽位技能段（S 段）。
//
// ⚠️ 读的是 `this.skills`（**会被 applyCard 替换**），
// 与「结算时读存档快照」不同。
//
// 之前这里被 `applyCard` 改过，存档记录的是 `0:1:120:2:20`，
// 而这里算的是 `0:1:100:1:20` —— 服务端比对时直接 422。
// 所以**锚点与记录必须是同一时刻的同一份数据**。
func (e *BattleEngine) ReplaySkillsSegment() string {
	items := make([]ReplaySkillSlot, 0, len(e.Skills))
	for _, s := range e.Skills {
		items = append(items, ReplaySkillSlot{
			Slot:        s.Slot,
			SkillID:     s.SkillID,
			BaseDamage:  s.BaseDamage,
			ApplyStacks: s.ApplyStacks,
			HeatCost:    s.HeatCost,
		})
	}
	return ReplaySkillsSegment(items)
}

// ReplayEventCount 是本局已记录的事件条数。
func (e *BattleEngine) ReplayEventCount() int { return len(e.replay) }

// RunToEnd 把引擎推到 won/lost 或 tick 上限。
//
// ⚠️ 脚本模式下**不能**外部 SkipCards ——
//
//	引擎会在 `Step()` 内部按脚本自动决策。
//	外部抢先丢掉手牌会让脚本彻底失效，
//	而症状是「hash 与原局不符」—— 完全看不出是这里。
//	只有无脚本（旧记录缺 `card_picks`）时才兜底跳过整波：
//	那对应旧记录（`card_picks` 为空），此时无法复现原局，
//	只能跑完看哈希是否巧合相符。
func RunToEnd(e *BattleEngine, maxTicks int) int {
	ticks := 0
	for ticks < maxTicks {
		switch e.Phase {
		case PhaseWon, PhaseLost:
			return ticks
		case PhaseCardSelect:
			if !e.HasReplayScript() {
				e.SkipCards()
			}
		}
		e.Step()
		ticks++
	}
	return ticks
}

// ReplayStats 是一局重放后对外汇报的统计。
type ReplayStats struct {
	Kills      int64
	Leaked     int64
	Score      int64
	Waves      int
	DurationMs int64
}

// ReplayOutcome 是一次重放的结果。
type ReplayOutcome struct {
	ComputedHash string
	Matched      bool
	Stats        ReplayStats
	// Err 非空表示**根本没跑起来**（缺关卡 / 缺构筑 / 种子非法）。
	//
	// ⚠️ 与 `Matched=false` 严格区分：
	//
	//	前者是「我算不出来」，后者是「我算出来了，和存档不符」。
	//	把两者混为一谈会让上层把「数据缺失」报成「玩家作弊」，
	//	而那会让真实 bug 被误判成攻击事件。
	Err error
}

// Replay 重放一局并比对哈希。
//
// 全程同步、无渲染、无定时器 —— 纯状态机推演。
// 10 分钟的战斗约 12000 tick，毫秒级完成，因此可以同步跑。
func Replay(level GeneratedLevel, build BuildSnapshot, seed string,
	cardPicks []int, expectedHash string,
	enemies map[int]SeedEnemy, skills map[int]SeedSkill,
	scoreRules ScoreRules) ReplayOutcome {
	empty := func(err error) ReplayOutcome {
		return ReplayOutcome{Err: err}
	}

	if len(build.Skills) == 0 {
		return empty(errors.New("缺少构筑快照（build_snapshot）。重放哈希依赖构筑，缺它算出的哈希必然不符。"))
	}
	equipped := EquippedFromSnapshot(build, skills)
	if len(equipped) == 0 {
		return empty(errors.New("构筑快照里没有任何已装备技能（全部 slot < 0），无法重放"))
	}
	att, err := AttackerFromSnapshot(build)
	if err != nil {
		return empty(err)
	}
	seedVal, err := strconv.ParseInt(seed, 10, 64)
	if err != nil {
		return empty(errors.New("种子格式非法：" + seed))
	}

	e := NewBattleEngine(BattleConfig{
		Level: level, Enemies: enemies, Skills: skills,
		Equipped: equipped, Attacker: att, Seed: seedVal,
		// 槽位数同样必须与原局同口径 ——
		// 玩家点了「额外插槽」的局槽位不是 4，重放算出的战斗会完全不同。
		// 老战报没有这个字段，零值落到引擎缺省 `ActiveSlots = 4`。
		ActiveSlots: build.ActiveSlots,
		// 分数规则与原局同口径（调用方从 /config 的 score_rules 转来）。
		ScoreRules: scoreRules,
	})
	// 注入选牌脚本：原局玩家点了哪张，重放就点哪张。
	// 缺脚本（空）时不注入，引擎走交互模式，由 `RunToEnd` 兜底跳过整波 ——
	// 对应旧记录无法复现选牌的情形。
	if len(cardPicks) > 0 {
		e.SetReplayScript(cardPicks)
	}
	e.Start()
	RunToEnd(e, MaxTicksPerReplay)

	computed := e.ReplayHash()
	return ReplayOutcome{
		ComputedHash: computed,
		Matched:      computed == expectedHash,
		Stats: ReplayStats{
			Kills:      e.Kills,
			Leaked:     e.Leaked,
			Score:      e.Score,
			Waves:      e.WaveIndex + 1,
			DurationMs: e.ElapsedMs,
		},
	}
}
