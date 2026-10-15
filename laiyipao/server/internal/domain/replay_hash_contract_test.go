package domain

// Block G 的跨端契约守卫：**整局重放**。
//
// # 这一块的分量与 B~F 都不同
//
// B~F 验的是「一局之内的中间状态」：逐步比对 tick / 敌人 / 弹丸 / 卡牌。
// G 验的是「一局跑完之后算出来的那 16 位十六进制」。
//
// 而 G 全绿意味着：
//
//	**服务端能拿存档里的 `build_snapshot` + `card_picks` + `seed`
//	自行重跑整局，并与存档里的 `replay_hash` 逐位比对。**
//
// 这正是 README 记的「A 段含局内 buff，服务端算不出」那条限制的解药：
// 局内 buff 由 `card_picks` 决定，`card_picks` 与 `build_snapshot`
// 都在 `battle_records` 同一行里 —— 于是重算的输入是完备的。
//
// # ⚠️ 当前口径：**1 条绿，4 条已知分歧**
//
// 见 `TestWholeBattleReplayKnownDivergences`。
// 诚实地说：G **尚未完成**，`SettleBattle` 不能接线。
//
// # 期望值全部来自 TS（玩家设备上运行的那份代码）
//
// 期望值**不由 Go 侧生成** —— 那是自证。
// 一条只断言「Go 算出来的 == Go 自己算出来的」的守卫永远绿，
// 且永远证明不了任何事。
//
// 生成器：`miniapp/scripts/gen-replay-vectors.ts`
//
// # 为什么只有 5 个关卡而不是全部 100 个
//
//   - 1 / 25 / 50 / 75 / 100 覆盖五段难度梯度
//   - 期间关卡含 BOSS 地形与 BOSS 分数段；早期关卡不含
//   - 每个关卡要生成完整**整局**，耗时与文件体积都线性增长
//
// 这 5 条不足以上「穷举」；它们是**采样**。

import (
	"strconv"
	"strings"
	"testing"
)

// toBuildSnapshot 把向量里的 build 转成生产的 BuildSnapshot。
//
// ⚠️ 必须**逐字段显式映射**，不能直接把 `replayBuild` 反序列化成 `BuildSnapshot`：
//
//	两者的 JSON tag 恰好对得上，直接反序列化**能成功** ——
//	而正因如此危险：一旦线上形状变了（比如 skills 的键从字符串改成数组），
//	这里会静默拿到空 map，而症状只是「哈希全错」。
//	显式写一遍之后，字段名对不上 / 键类型变了 / 字段漏了，
//	都会在编译期或断言处炸出来。
func toBuildSnapshot(t *testing.T, b replayBuild) BuildSnapshot {
	t.Helper()
	out := BuildSnapshot{
		Skills:      make(map[string]BuildSnapshotSkill, len(b.Skills)),
		ActiveSlots: b.ActiveSlots,
	}
	for k, s := range b.Skills {
		out.Skills[k] = BuildSnapshotSkill{
			ID:      s.ID,
			Element: Element(s.Element),
			Kind:    s.Kind,
			Level:   s.Level,
			Slot:    s.Slot,
		}
	}
	out.Attacker = &BuildSnapshotAttacker{
		Attack:                 int64(b.Attacker.Attack),
		CritPermille:           int64(b.Attacker.CritPermille),
		CritMultiplierPermille: int64(b.Attacker.CritMultPermille),
		ReactionMultPermille:   int64(b.Attacker.ReactionMultPermille),
		ElementCap:             int64(b.Attacker.ElementCap),
		ReactionTier:           int64(b.Attacker.ReactionTier),
		ElementCoefPermille:    int64(b.Attacker.ElementCoefPermille),
		HeatCapPermille:        int64(b.Attacker.HeatCapPermille),
		ArmorPermille:          int64(b.Attacker.ArmorPermille),
		MechanicPermille:       int64(b.Attacker.MechanicPermille),
	}
	return out
}

// buildReplayDeps 造重放所需的敌人/技能表与分数规则。
func buildReplayDeps(t *testing.T) (map[int]SeedEnemy, map[int]SeedSkill, ScoreRules) {
	t.Helper()
	fx := loadSmokeFixture(t)
	enemies := map[int]SeedEnemy{}
	for _, e := range fx.Enemies {
		enemies[e.ID] = e
	}
	skills := map[int]SeedSkill{}
	for _, s := range fx.Skills {
		skills[s.ID] = s
	}
	for _, s := range fx.AllSkills {
		skills[s.ID] = s
	}
	return enemies, skills, DefaultScoreRules()
}

// replayLevel 取关卡（按夹具，含 terrain / waves / element_cap 等）。
//
// ⚠️ 必须用**夹具里的关卡**而不是 `GenerateLevel(id)`：
//
//	向量的期望值来自 TS 侧读 `smoke_levels.json`，
//	而 `GenerateLevel` 是另一条生成路径。
//	两条路本该一致，但「本该一致」不是「同一份数据」——
//	混用时差异会以「刷怪数量不同」的形式出现，
//	而排查方向会指向错误的一边。
func replayLevel(t *testing.T, id int) GeneratedLevel {
	t.Helper()
	fx := loadSmokeFixture(t)
	for _, l := range fx.Levels {
		if l.ID != id {
			continue
		}
		waves := make([]Wave, 0, len(l.Waves))
		for _, w := range l.Waves {
			wv := Wave{Index: w.Index}
			for _, s := range w.Spawns {
				wv.Spawns = append(wv.Spawns, Spawn{
					EnemyID: s.EnemyID, Count: s.Count,
					Interval: int(s.Interval), Delay: int(s.Delay),
				})
			}
			waves = append(waves, wv)
		}
		return GeneratedLevel{
			ID: l.ID, BaseHP: l.BaseHP, ArmorPermille: l.ArmorPermille,
			ElementCap: l.ElementCap, MaxReactionTier: l.MaxReactionTier,
			Terrain: l.Terrain, Waves: waves,
		}
	}
	t.Fatalf("夹具里没有第 %d 关", id)
	return GeneratedLevel{}
}

// replayCaseResult 跑一个整局用例并回报结果。
func replayCaseResult(t *testing.T, c replayCase) ReplayOutcome {
	t.Helper()
	enemies, skills, rules := buildReplayDeps(t)
	return Replay(
		replayLevel(t, c.LevelID), toBuildSnapshot(t, c.Build),
		c.Seed, c.CardPicks, c.ExpectedHash, enemies, skills, rules,
	)
}

// assertCaseStats 比对整局统计。
//
// ⚠️ 先比统计、再比哈希：
//
//	统计先不符的话，哈希不符的原因就是「某一处数值错了」；
//	而只比哈希的话只能得到一个 16 位十六进制 —— 定位价值为零。
func assertCaseStats(t *testing.T, out ReplayOutcome, c replayCase) {
	t.Helper()
	for _, f := range []struct {
		name     string
		got, exp int64
	}{
		{"kills", out.Stats.Kills, int64(c.ExpectedStats.Kills)},
		{"leaked", out.Stats.Leaked, int64(c.ExpectedStats.Leaked)},
		{"waves", int64(out.Stats.Waves), int64(c.ExpectedStats.Waves)},
	} {
		if f.got != f.exp {
			t.Errorf("%s = %d，TS 期望 %d", f.name, f.got, f.exp)
		}
	}
}

// TestWholeBattleReplayMatchesCrossEnd 钉住**已收敛**的整局用例。
//
// 目前只有第 1 关：`card_picks` 为空 → `replay()` **不注入选牌脚本** →
// 引擎走交互模式，由 `RunToEnd` 兜底跳过整波。
// 也就是说这条只覆盖「无选牌脚本的整局」。
//
// 有脚本的 4 条在 `TestWholeBattleReplayKnownDivergences` 里。
func TestWholeBattleReplayMatchesCrossEnd(t *testing.T) {
	doc := loadReplayVectors(t)
	for _, c := range doc.Cases {
		if len(c.CardPicks) != 0 {
			continue // 有选牌脚本的用例在 KnownDivergences 里
		}
		c := c
		t.Run("lvl"+strconv.Itoa(c.LevelID), func(t *testing.T) {
			out := replayCaseResult(t, c)
			if out.Err != nil {
				t.Fatalf("重放没跑起来：%v", out.Err)
			}
			assertCaseStats(t, out, c)
			if out.Stats.Score != int64(c.ExpectedStats.Score) {
				t.Errorf("score = %d，TS 期望 %d", out.Stats.Score, c.ExpectedStats.Score)
			}
			if out.Stats.DurationMs != int64(c.ExpectedStats.DurationMs) {
				t.Errorf("durationMs = %d，TS 期望 %d", out.Stats.DurationMs, c.ExpectedStats.DurationMs)
			}
			if out.ComputedHash != c.ExpectedHash {
				t.Errorf("replayHash = %s，TS 期望 %s", out.ComputedHash, c.ExpectedHash)
			}
			if !out.Matched {
				t.Error("Matched = false（见上面的哈希不符）")
			}
			if out.Stats.Waves < 5 {
				t.Errorf("只跑到第 %d 波 —— 整局路径没被覆盖", out.Stats.Waves)
			}
		})
	}
}

// knownReplayDivergence 是一条**已知未收敛**的分歧的台账条目。
//
// ⚠️ 为什么写成台账而不是直接断言「哈希不等」：
//
//	「哈希不等」是**恒真**的断言 —— 它在问题修好的那一刻会**变红**，
//	而那一刻恰恰是好事。
//	写成台账则相反：修好之后它会**变红并提醒你删掉这条**，
//	删掉之后剩余的用例自动升格为「必须匹配」。
//	两种都红，但只有后者不会在人不想它红的时候红。
type knownReplayDivergence struct {
	LevelID int
	// Seed 原样记下来：换种子就换了另一场战斗，分歧点会变。
	Seed string
	// CardPicks 数一下：这个数说明「脚本有多长」，是分歧的形状。
	CardPicks int
	// FirstDiffTick 是两端 replay 事件流**第一次**不同步的 tick。
	// 实测值，定位靠它。
	FirstDiffTick int64
	// FirstDiffGo / FirstDiffTS 是那两条事件的原文。
	FirstDiffGo string
	FirstDiffTS string
	// Why 一句话写清「已排除什么」，省掉下一个人重走一遍。
	Why string
}

// knownReplayDivergences 是当前**已知**未收敛的分歧。
//
// # 已排除的东西（别重走）
//
//   - 构筑还原：`EquippedFromSnapshot` 产出的 S 段与 TS **逐字符相同**
//     （实测 `0:1:100:1:20,1:2:168:2:30,2:3:261:2:40,3:4:130:1:18`）
//   - 选牌脚本消费：`cardPicks` 与 TS 相同（`[0,0,-1,-1,-1]`），
//     波次边界的 **tick 也相同**（233 / 326）
//   - `ResolveHit` 的公式：Go 的实现与 TS 的 `resolveHit` 逐行等价
//     （`d` / `direct` / `elementDmg` / 反应两侧 / crit 区间都核对过）
//   - `currentAttacker()`：两端逐字段一致，**含** critPermille 的 `min(1000)` 夹取
//   - `mulDiv`：两端都是截断除法 `(a*b)/c`
//   - tick 序 / 事件流格式：分歧点之前的 replay 事件**逐条相同**
//
// # 现象
//
// 分歧只在**有选牌脚本**的用例上出现，且都在**第 1 波之后**——
// 也就是「局内 buff 生效之后」才出现。
// 第 1 波（buff 尚未生效）两端逐事件相同。
//
// 这把范围收得很窄：**嫌疑集中在「卡牌效果如何改变攻方系数」这条链上**，
// 而不是引擎主干。
var knownReplayDivergences = []knownReplayDivergence{
	{
		LevelID: 25, Seed: "-4611686018427387904", CardPicks: 6, FirstDiffTick: 444,
		FirstDiffTS: "444|hit|29|7|1",
		FirstDiffGo: "444|hit|29|6|1",
		Why: "reaction_mult/element_coef 都是 3500（远高于 B~F 场景的 ≤1100）；" +
			"同一发弹丸同一目标，直接伤害 TS 754 / Go 698、反应伤害 TS 339 / Go 325",
	},
	{
		LevelID: 50, Seed: "1234567890123456789", CardPicks: 8, FirstDiffTick: -1,
		Why: "同上，尚未逐条定位到第一处分歧",
	},
	{
		LevelID: 75, Seed: "999999999999999999", CardPicks: 10, FirstDiffTick: -1,
		Why: "同上；这关 max_reaction_tier=2，tier≠1 的反应路径也可能是嫌疑",
	},
	{
		LevelID: 100, Seed: "-1", CardPicks: 12, FirstDiffTick: -1,
		Why: "同上；这关 element_cap=8，是五关里层数上限最大的",
	},
}

// TestWholeBattleReplayKnownDivergences 钉住「已知分歧集合」本身。
//
// ⚠️ 这条测试的价值在于**集合的形状**，不在于每一条的细节：
//
//   - 新增分歧 → 这里红（说明还有别的没查清）
//   - 某条被修好 → 这里红并提示删掉它
//   - 消失一条却不更新 → 下一条 `MatchesCrossEnd` 会红
//
// 也就是说它把「我不知道还有没有别的分歧」这个不确定性，
// 变成一个每次跑都会回答的问题。
func TestWholeBattleReplayKnownDivergences(t *testing.T) {
	doc := loadReplayVectors(t)
	got := map[int]replayCase{}
	for _, c := range doc.Cases {
		got[c.LevelID] = c
	}
	want := map[int]bool{}
	for _, d := range knownReplayDivergences {
		want[d.LevelID] = true
	}

	for _, c := range doc.Cases {
		c := c
		out := replayCaseResult(t, c)
		if out.Err != nil {
			t.Fatalf("第 %d 关重放没跑起来：%v", c.LevelID, out.Err)
		}
		if !want[c.LevelID] {
			if !out.Matched {
				t.Errorf("第 %d 关不在已知分歧台账里却哈希不符（Go=%s TS=%s）—— "+
					"要么把它加进 knownReplayDivergences，要么这是一条新分歧",
					c.LevelID, out.ComputedHash, c.ExpectedHash)
			}
			continue
		}
		if out.Matched {
			t.Errorf("第 %d 关的已知分歧**已收敛**（哈希 %s == TS）—— "+
				"请把它从 knownReplayDivergences 里删掉，它会自动升格为必须匹配",
				c.LevelID, out.ComputedHash)
		}
		// 台账里记的种子/脚本长度必须与向量文件一致，
		// 否则「同一条分歧」的描述已经指向另一场战斗了。
		for _, d := range knownReplayDivergences {
			if d.LevelID != c.LevelID {
				continue
			}
			if d.Seed != c.Seed {
				t.Errorf("第 %d 关的台账种子 %s 与向量文件 %s 不一致 —— "+
					"分歧点会变，台账描述失效", d.LevelID, d.Seed, c.Seed)
			}
			if d.CardPicks != len(c.CardPicks) {
				t.Errorf("第 %d 关的台账记 card_picks %d 项，向量文件是 %d 项",
					d.LevelID, d.CardPicks, len(c.CardPicks))
			}
		}
		// 结构性统计必须仍然一致 —— 它们是「引擎主干没坏」的证据。
		assertCaseStats(t, out, c)
	}
}

// TestReplayContractHashNoteMatchesImplementation 钉住哈希公式的**形状**。
//
// ⚠️ 向量文件里的 `hash_note` 是人读的公式说明。
//
//	它与实现脱节时，症状是「向量全绿但没人知道在算什么」——
//	下一个人照着 `hash_note` 去改实现就会踩空。
func TestReplayContractHashNoteMatchesImplementation(t *testing.T) {
	doc := loadReplayVectors(t)
	for _, token := range []string{"L{id}", "A{7", "S{", "X{", ";", "{tick}|{类型}|{a}|{b}|{c}"} {
		if !strings.Contains(doc.HashNote, token) {
			t.Errorf("hash_note 里没有 %q —— 说明要么公式变了（要改注释），"+
				"要么本文件该更新了", token)
		}
	}
}

// TestReplaySkillsSegmentSortsLexicographically 钉住 S 段的排序是**字典序**。
//
// ⚠️ 这是 TS 的 `.map(...).sort().join(',')` 的语义：
//
//	JS 的 `Array.prototype.sort()` 默认按**字符串**比较，
//	所以 `slot=10` 排在 `slot=2` **前面**。
//	README 记的第 57 号 bug 就是这个：
//	曾经写成「按数值排序」以示「更正确」，结果 10 人打团全部失配。
//
//	Go 侧必须用 `sort.Strings` 而不是按 `Slot` 数值排序 ——
//	这是「照抄一个看起来是 bug 的行为」的正确场景：
//	它不是 bug，它是**跨端契约**。
func TestReplaySkillsSegmentSortsLexicographically(t *testing.T) {
	got := ReplaySkillsSegment([]ReplaySkillSlot{
		{Slot: 2, SkillID: 7, BaseDamage: 100, ApplyStacks: 1, HeatCost: 20},
		{Slot: 10, SkillID: 8, BaseDamage: 100, ApplyStacks: 1, HeatCost: 20},
	})
	want := "10:8:100:1:20,2:7:100:1:20"
	if got != want {
		t.Errorf("S 段 = %q，期望 %q —— 排序必须是**字典序**，"+
			"改成数值排序会让 10 人打团的战报全部失配（README 第 57 号 bug）", got, want)
	}
}

//	attack / crit / critMul / reactionMult / elementCoef  ← cfg.attacker
//	elementCap / reactionTier                            ← cfg.level
//
// ⚠️ 后两项**不读** `cfg.attacker`：`currentAttacker()` 里是
//
//	elementCap  = (lv.element_cap ? lv.element_cap : base.elementCap) + bonus
//	reactionTier = lv.max_reaction_tier || 1
//
// 也就是说关卡一旦给了值，攻方快照里的同名字段就是**死的**。
// 这条测试最初写成「改 `Cfg.Attacker.ElementCap` 应当改变哈希」，
// 于是红 —— 因为它压根没进锚点。改锚点要从**生效的那一侧**改。
//
//	多写一项进前缀不会让「校验更严」，只会让**所有历史战报**失配。
func TestReplayHashAnchorUsesOnlySevenAttackerFields(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Level.ID = 7
	e.Cfg.Level.ElementCap = 0 // 让 elementCap 落到攻方
	e.Cfg.Attacker = Attacker{
		Attack: 100, CritPermille: 50, CritMultiplierPermille: 1500,
		ReactionMultPermille: 1000, ElementCap: 3, ReactionTier: 1,
		ElementCoefPermille: 1000,
		// 这三项故意设成非 0：改它们**不该**影响哈希
		HeatCapPermille:  300,
		ArmorPermille:    750,
		MechanicPermille: 400,
	}
	before := e.ReplayHash()

	e.Cfg.Attacker.HeatCapPermille = 999
	e.Cfg.Attacker.ArmorPermille = 999
	e.Cfg.Attacker.MechanicPermille = 999
	if after := e.ReplayHash(); after != before {
		t.Errorf("改了 heatCap/armor/mechanic 三个系数后哈希变了（%s -> %s）—— "+
			"它们不在 A 段的 7 项里；多写一项会让**所有历史战报**失配",
			before, after)
	}

	// 而 7 项里的任意一项改动**必须**改变哈希
	for _, f := range []struct {
		name string
		mut  func(*BattleEngine)
	}{
		{"attack", func(b *BattleEngine) { b.Cfg.Attacker.Attack = 101 }},
		{"critPermille", func(b *BattleEngine) { b.Cfg.Attacker.CritPermille = 51 }},
		{"critMultiplierPermille", func(b *BattleEngine) { b.Cfg.Attacker.CritMultiplierPermille = 1501 }},
		{"reactionMultPermille", func(b *BattleEngine) { b.Cfg.Attacker.ReactionMultPermille = 1001 }},
		{"elementCap(攻方)", func(b *BattleEngine) { b.Cfg.Attacker.ElementCap = 4 }},
		{"reactionTier(关卡)", func(b *BattleEngine) { b.Cfg.Level.MaxReactionTier = 2 }},
		{"elementCoefPermille", func(b *BattleEngine) { b.Cfg.Attacker.ElementCoefPermille = 1001 }},
		{"elementCap(关卡)", func(b *BattleEngine) { b.Cfg.Level.ElementCap = 9 }},
	} {
		saveAtt, saveLv := e.Cfg.Attacker, e.Cfg.Level
		f.mut(e)
		if e.ReplayHash() == before {
			t.Errorf("改了 A 段里的 %s 哈希却没变 —— 该项不在锚点里，"+
				"意味着玩家可以白拿加成而校验照样通过", f.name)
		}
		e.Cfg.Attacker, e.Cfg.Level = saveAtt, saveLv
	}
}

// TestReplayHashIncludesExtraBaseHp 钉住 `|X{extraBaseHp}` 段存在。
func TestReplayHashIncludesExtraBaseHp(t *testing.T) {
	a := newProbeEngine()
	a.Cfg.Level.ID = 7
	b := newProbeEngine()
	b.Cfg.Level.ID = 7
	b.Cfg.ExtraBaseHp = 12345

	if a.ReplayHash() == b.ReplayHash() {
		t.Error("改了 extra_base_hp 后哈希没变 —— `|X` 段没起作用")
	}
}

// TestEquippedFromSnapshotSkipsUnslottedAndMissing 钉住快照还原的过滤规则。
func TestEquippedFromSnapshotSkipsUnslottedAndMissing(t *testing.T) {
	skills := map[int]SeedSkill{
		1: {ID: 1, Name: "a", Element: ElementFire, Kind: "active", BaseDamage: 100},
		2: {ID: 2, Name: "b", Element: ElementIce, Kind: "active", BaseDamage: 100},
	}
	build := BuildSnapshot{Skills: map[string]BuildSnapshotSkill{
		"1": {ID: 1, Slot: 0, Level: 1},
		"2": {ID: 2, Slot: -1, Level: 1}, // 未放入槽位
		"9": {ID: 9, Slot: 1, Level: 1},  // 内容表里没有
	}}
	got := EquippedFromSnapshot(build, skills)
	if len(got) != 1 {
		t.Fatalf("还原出 %d 个技能，期望 1（slot<0 跳过、下架技能跳过）", len(got))
	}
	if got[0].SkillID != 1 {
		t.Errorf("还原出的是技能 %d，期望 1", got[0].SkillID)
	}
}

// TestEquippedFromSnapshotSortsBySlotThenID 钉住排序。
//
// ⚠️ 顺序进 RNG（`rollSkillCards` 的 `rng.intn(actives)`），
// 不排序的话同一个种子会抽出不同的技能 ——
// 而症状是「偶发哈希失配」，极难定位。
func TestEquippedFromSnapshotSortsBySlotThenID(t *testing.T) {
	skills := map[int]SeedSkill{}
	for _, id := range []int{1, 2, 3, 4, 5} {
		skills[id] = SeedSkill{ID: id, Name: "s", Element: ElementFire,
			Kind: "active", BaseDamage: 100}
	}
	build := BuildSnapshot{Skills: map[string]BuildSnapshotSkill{
		"3": {ID: 3, Slot: 2, Level: 1},
		"1": {ID: 1, Slot: 0, Level: 1},
		"5": {ID: 5, Slot: 1, Level: 1},
		"2": {ID: 2, Slot: 0, Level: 1}, // 与 1 同槽 -> 按 id 兜底
	}}
	got := EquippedFromSnapshot(build, skills)
	want := []int{1, 2, 5, 3}
	for i := range want {
		if i >= len(got) || got[i].SkillID != want[i] {
			t.Fatalf("还原顺序 = %v，期望 %v", skillIDsOf(got), want)
		}
	}
}

// TestEquippedFromSnapshotBakesLevelIntoBaseDamage 钉住等级烘进 baseDamage。
//
// ⚠️ 等级若不烘进去，两个不同等级的构筑可能算出同一份哈希 ——
// **验真通过但伤害对不上**，哈希在说谎。
func TestEquippedFromSnapshotBakesLevelIntoBaseDamage(t *testing.T) {
	skills := map[int]SeedSkill{
		1: {ID: 1, Name: "a", Element: ElementFire, Kind: "active", BaseDamage: 100},
	}
	mk := func(level int) int64 {
		build := BuildSnapshot{Skills: map[string]BuildSnapshotSkill{
			"1": {ID: 1, Slot: 0, Level: level},
		}}
		return EquippedFromSnapshot(build, skills)[0].BaseDamage
	}
	if mk(1) != 100 {
		t.Errorf("level 1 应原样返回 baseDamage=100，实际 %d", mk(1))
	}
	if mk(10) <= mk(1) {
		t.Errorf("level 10 的伤害 %d 应大于 level 1 的 %d —— "+
			"等级没烘进 baseDamage 时，两个不同等级的构筑会算出同一份哈希", mk(10), mk(1))
	}
	// 等级 30 应被 clampLevel 压到 maxLevel=10
	if mk(30) != mk(10) {
		t.Errorf("level 30 应被 clampLevel 压到 10（coef 1450），实际 %d ≠ %d",
			mk(30), mk(10))
	}
}

// TestAttackerFromSnapshotDefaults 钉住攻方缺省值。
//
// ⚠️ 缺省值与 TS **逐字一致**（`?? 默认`），否则老战报的哈希会变。
func TestAttackerFromSnapshotDefaults(t *testing.T) {
	got, err := AttackerFromSnapshot(BuildSnapshot{
		Attacker: &BuildSnapshotAttacker{Attack: 777},
	})
	if err != nil {
		t.Fatal(err)
	}
	want := Attacker{
		Attack: 777, CritPermille: 50, CritMultiplierPermille: 1500,
		ReactionMultPermille: 1000, ElementCap: 3, ReactionTier: 1,
		ElementCoefPermille: 1000, HeatCapPermille: 0, ArmorPermille: 0,
		MechanicPermille: 0,
	}
	if got != want {
		t.Errorf("缺省攻方 = %+v，期望 %+v", got, want)
	}

	if _, err := AttackerFromSnapshot(BuildSnapshot{}); err == nil {
		t.Error("缺 attacker 字段时应当返回 error —— " +
			"返回一个必然不匹配的哈希并假装验过了，会让 I-6 变成制造假警报的噪音源")
	}
}

// TestReplayOutcomeDistinguishesCannotRunFromMismatch 钉住两种失败的区分。
//
// ⚠️ `Err != nil`（算不出来）与 `Matched == false`（算出来不符）必须分开：
// 把「数据缺失」报成「玩家作弊」会让真实 bug 被误判成攻击事件。
func TestReplayOutcomeDistinguishesCannotRunFromMismatch(t *testing.T) {
	enemies, skills, rules := buildReplayDeps(t)
	level := replayLevel(t, 1)

	out := Replay(level, BuildSnapshot{}, "1", nil, "x", enemies, skills, rules)
	if out.Err == nil {
		t.Error("缺构筑时应当有 Err")
	}
	if out.Matched {
		t.Error("缺构筑时 Matched 不该为 true")
	}

	build := BuildSnapshot{
		Skills:      map[string]BuildSnapshotSkill{"1": {ID: 1, Slot: 0, Level: 1}},
		Attacker:    &BuildSnapshotAttacker{Attack: 100},
		ActiveSlots: 4,
	}
	out = Replay(level, build, "not-a-number", nil, "x", enemies, skills, rules)
	if out.Err == nil {
		t.Error("种子非法时应当有 Err")
	}

	out = Replay(level, build, "1", nil, "deadbeefdeadbeef", enemies, skills, rules)
	if out.Err != nil {
		t.Fatalf("不该有 Err：%v", out.Err)
	}
	if out.Matched {
		t.Error("哈希不符时 Matched 必须为 false")
	}
}

// TestRunToEndSkipsOnlyWithoutScript 钉住「脚本模式下不外部跳牌」。
//
// ⚠️ 脚本模式下外部 `SkipCards` 会让脚本彻底失效：
//
//	引擎在 `Step()` 内部按脚本决策，外部抢先丢掉手牌会让脚本看不见手牌，
//	而症状是「hash 与原局不符」—— 完全看不出是这个函数。
//
// 判据用 `card_picks` 的**内容**而不是长度：
// `-1` 是「整波跳过」，`-3` 是「取第 0 张」。
// 两条路径都会产生**同样长度**的 card_picks，
// 只看长度的话「外部兜底跳牌」与「引擎按脚本跳牌」完全一样。
func TestRunToEndSkipsOnlyWithoutScript(t *testing.T) {
	enemies, skills, rules := buildReplayDeps(t)
	level := replayLevel(t, 1)
	build := BuildSnapshot{
		Skills: map[string]BuildSnapshotSkill{
			"1": {ID: 1, Slot: 0, Level: 1},
			"2": {ID: 2, Slot: 1, Level: 1},
			"3": {ID: 3, Slot: 2, Level: 1},
			"4": {ID: 4, Slot: 3, Level: 1},
		},
		Attacker:    &BuildSnapshotAttacker{Attack: 200, CritPermille: 50, ElementCap: 3},
		ActiveSlots: 4,
	}

	// 无脚本：RunToEnd 必须兜底跳牌，否则卡在 card_select 耗尽 tick 上限
	noScript := NewBattleEngine(BattleConfig{
		Level: level, Enemies: enemies, Skills: skills,
		Equipped: EquippedFromSnapshot(build, skills),
		Attacker: mustAttacker(t, build), Seed: 7046029255919282421,
		ActiveSlots: 4, ScoreRules: rules,
	})
	noScript.Start()
	RunToEnd(noScript, MaxTicksPerReplay)
	if noScript.Phase != PhaseWon && noScript.Phase != PhaseLost {
		t.Fatalf("无脚本的局跑完后 phase = %q，既没 won 也没 lost —— "+
			"RunToEnd 的兜底跳牌失效，引擎卡在 card_select 耗尽 tick 上限",
			noScript.Phase)
	}
	if len(noScript.CardPicks) != len(level.Waves)-1 {
		t.Errorf("card_picks = %v（%d 项），期望 %d 项（每波清空一次）",
			noScript.CardPicks, len(noScript.CardPicks), len(level.Waves)-1)
	}
	for i, p := range noScript.CardPicks {
		if p != -1 {
			t.Errorf("无脚本时 card_picks[%d] = %d，期望 -1（整波跳过）", i, p)
			break
		}
	}

	// 有脚本：必须由**引擎**按脚本决策。
	// 用 `-3`（取第 0 张）而不是 `-1`，这样「外部兜底」一眼就能看出来。
	withScript := NewBattleEngine(BattleConfig{
		Level: level, Enemies: enemies, Skills: skills,
		Equipped: EquippedFromSnapshot(build, skills),
		Attacker: mustAttacker(t, build), Seed: 7046029255919282421,
		ActiveSlots: 4, ScoreRules: rules,
	})
	withScript.SetReplayScript([]int{-3, -3, -3, -3, -3, -3})
	withScript.Start()
	RunToEnd(withScript, MaxTicksPerReplay)
	if withScript.Phase != PhaseWon && withScript.Phase != PhaseLost {
		t.Fatalf("有脚本的局跑完后 phase = %q —— 引擎卡住了", withScript.Phase)
	}
	if len(withScript.CardPicks) == 0 {
		t.Fatal("有脚本的局 card_picks 为空 —— 脚本根本没被执行")
	}
	for i, p := range withScript.CardPicks {
		if p == -1 {
			t.Errorf("card_picks[%d] = -1，但脚本全是 -3 —— "+
				"RunToEnd 在脚本模式下抢着调了 SkipCards，脚本被架空", i)
			break
		}
	}
}

func mustAttacker(t *testing.T, b BuildSnapshot) Attacker {
	t.Helper()
	a, err := AttackerFromSnapshot(b)
	if err != nil {
		t.Fatal(err)
	}
	return a
}

// TestMaxTicksPerReplayExceedsBattleCap 钉住两个上限的**顺序**。
//
// ⚠️ `MaxTicksPerReplay` 必须**大于** `MaxBattleTicks`：
// 否则 RunToEnd 的上限先到，会算出「没打完」的中间态哈希。
func TestMaxTicksPerReplayExceedsBattleCap(t *testing.T) {
	if MaxTicksPerReplay <= MaxBattleTicks {
		t.Errorf("MaxTicksPerReplay(%d) 必须大于 MaxBattleTicks(%d)",
			MaxTicksPerReplay, MaxBattleTicks)
	}
}
