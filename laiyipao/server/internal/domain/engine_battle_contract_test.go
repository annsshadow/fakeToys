package domain

// Block D（开火与弹道）的**跨端逐事件对齐**守卫。
//
// # 为什么这一块是里程碑
//
// B/C 的场景是**空构筑** —— 敌人只会被「抵达防线」处理掉。
// D 第一次让引擎真的开火、命中、结算伤害、触发反应、击杀，
// 于是**整局 `replay_hash`** 第一次可以被服务端自行计算。
//
// 那是 R1 从「记录不符」变成「拒发不符」的技术前提。

import (
	"encoding/json"
	"strconv"
	"strings"
	"testing"
)

// battleStep 是一步战斗快照的期望值。
type battleStep struct {
	Tick        int64  `json:"tick"`
	Phase       string `json:"phase"`
	ElapsedMs   int64  `json:"elapsed_ms"`
	Projectiles []struct {
		UID        int64  `json:"uid"`
		SkillID    int    `json:"skill_id"`
		X          string `json:"x"`
		Y          string `json:"y"`
		VX         string `json:"vx"`
		VY         string `json:"vy"`
		PierceLeft int    `json:"pierce_left"`
		ChainLeft  int    `json:"chain_left"`
		HitSet     string `json:"hit_set"`
		AoeRadius  int64  `json:"aoe_radius"`
		Dead       bool   `json:"dead"`
	} `json:"projectiles"`
	Enemies []struct {
		UID                int64  `json:"uid"`
		DefID              int    `json:"def_id"`
		X                  string `json:"x"`
		Y                  string `json:"y"`
		HP                 string `json:"hp"`
		Shield             string `json:"shield"`
		SpawnProgress      int    `json:"spawn_progress"`
		Dead               bool   `json:"dead"`
		AttackCooldown     string `json:"attack_cooldown"`
		Stacks             string `json:"stacks"`
		FrozenMs           int64  `json:"frozen_ms"`
		StunnedMs          int64  `json:"stunned_ms"`
		SlowedMs           int64  `json:"slowed_ms"`
		ArmorShredMs       int64  `json:"armor_shred_ms"`
		ArmorShredPermille string `json:"armor_shred_permille"`
		Knockback          string `json:"knockback"`
		AmplifyPermille    string `json:"amplify_permille"`
		HitFlashMs         int64  `json:"hit_flash_ms"`
	} `json:"enemies"`
	Replay []string `json:"replay"`
	Events []string `json:"events"`

	BaseHP        string   `json:"base_hp"`
	Leaked        int64    `json:"leaked"`
	Kills         int64    `json:"kills"`
	Shots         int64    `json:"shots"`
	Hits          int64    `json:"hits"`
	Reactions     int64    `json:"reactions"`
	Score         string   `json:"score"`
	Heat          string   `json:"heat"`
	HeatCap       string   `json:"heat_cap"`
	Overheated    bool     `json:"overheated"`
	Cooldowns     []string `json:"cooldowns"`
	ElementsUsed  string   `json:"elements_used"`
	ReactionsUsed string   `json:"reactions_used"`
}

// battleRow 是一行战斗场景。
type battleRow struct {
	Label        string
	LevelID      int
	Seed         string
	Ticks        int
	SampleEvery  int
	Equipped     []int
	LevelWaves   []waveLevelDef
	BaseHP       int64
	ArmorMill    int64
	ElementCap   int64
	ReactionTier int64
	Attacker     Attacker
	Trace        []battleStep
}

// buildBattleEngine 按场景重建引擎。
//
// ⚠️ 技能**必须**从服务端内容表取，且槽位 = 列表下标。
//
//	内容表 id 是全局递增的（1..42），直接拿 id 当 slot 的话
//	全部技能都 >= activeSlots = 4，一发都放不出来。
func buildBattleEngine(t *testing.T, row battleRow) *BattleEngine {
	t.Helper()
	fx := loadSmokeFixture(t)

	waves := make([]Wave, 0, len(row.LevelWaves))
	for _, w := range row.LevelWaves {
		wv := Wave{Index: w.Index}
		for _, s := range w.Spawns {
			wv.Spawns = append(wv.Spawns, Spawn{
				EnemyID:  s.EnemyID,
				Count:    s.Count,
				Interval: int(s.Interval),
				Delay:    int(s.Delay),
			})
		}
		waves = append(waves, wv)
	}

	enemyDefs := map[int]SeedEnemy{}
	for _, en := range fx.Enemies {
		enemyDefs[en.ID] = en
	}
	skillDefs := map[int]SeedSkill{}
	for _, s := range fx.Skills {
		skillDefs[s.ID] = s
	}

	equipped := make([]EquippedSkill, 0, len(row.Equipped))
	for i, id := range row.Equipped {
		d, ok := skillDefs[id]
		if !ok {
			t.Fatalf("技能 id %d 不在夹具里", id)
		}
		equipped = append(equipped, EquippedSkill{
			SkillID:         d.ID,
			Name:            d.Name,
			Element:         d.Element,
			Kind:            d.Kind,
			HeatCost:        d.HeatCost,
			CooldownMs:      d.CooldownMs,
			Pierce:          int(d.Pierce),
			AoeRadius:       d.AoeRadius,
			BaseDamage:      d.BaseDamage,
			ApplyElement:    d.ApplyElement,
			ApplyStacks:     d.ApplyStacks,
			ProjectileSpeed: d.ProjectileSpeed,
			Chain:           int(d.Chain),
			Slot:            i,
		})
	}

	seed, err := strconv.ParseInt(row.Seed, 10, 64)
	if err != nil {
		t.Fatalf("种子 %q 解析失败：%v", row.Seed, err)
	}

	level := GeneratedLevel{
		ID:              row.LevelID,
		Waves:           waves,
		BaseHP:          row.BaseHP,
		ArmorPermille:   row.ArmorMill,
		ElementCap:      row.ElementCap,
		MaxReactionTier: row.ReactionTier,
		// ⚠️ 地形留空：E 块的范围。D 阶段 Go 侧不做掩体判定，
		//   而 TS 会做 —— 带上地形会让对齐在「弹丸被油桶吃掉」处分叉。
		Terrain: nil,
	}
	return NewBattleEngine(BattleConfig{
		Level:       level,
		Enemies:     enemyDefs,
		Skills:      skillDefs,
		Equipped:    equipped,
		Attacker:    row.Attacker,
		Seed:        seed,
		ScoreRules:  DefaultScoreRules(),
		ActiveSlots: ActiveSlots,
	})
}

func TestBattleScenariosMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	if len(v.Battle.Rows) == 0 {
		t.Fatal("向量里没有战斗场景 —— D 块的跨端对齐是空跑")
	}
	for _, r := range v.Battle.Rows {
		row := battleRow{
			Label: r.Label, LevelID: r.LevelID, Seed: r.Seed, Ticks: r.Ticks,
			SampleEvery: r.SampleEvery, Equipped: r.Equipped,
			LevelWaves: r.LevelWaves, BaseHP: r.LevelBaseHP,
			ArmorMill: r.LevelArmorPermille, ElementCap: r.LevelElementCap,
			ReactionTier: r.LevelMaxReactionTier, Trace: r.Trace,
			Attacker: Attacker{
				Attack:                 mustInt64(t, r.Attacker.Attack),
				CritPermille:           mustInt64(t, r.Attacker.CritPermille),
				CritMultiplierPermille: mustInt64(t, r.Attacker.CritMultiplierPermille),
				ReactionMultPermille:   mustInt64(t, r.Attacker.ReactionMultPermille),
				ElementCap:             mustInt64(t, r.Attacker.ElementCap),
				ReactionTier:           mustInt64(t, r.Attacker.ReactionTier),
				ElementCoefPermille:    mustInt64(t, r.Attacker.ElementCoefPermille),
				HeatCapPermille:        mustInt64(t, r.Attacker.HeatCapPermille),
				ArmorPermille:          mustInt64(t, r.Attacker.ArmorPermille),
				MechanicPermille:       mustInt64(t, r.Attacker.MechanicPermille),
			},
		}
		t.Run(row.Label, func(t *testing.T) { runBattleScenario(t, row) })
	}
}

func runBattleScenario(t *testing.T, row battleRow) {
	t.Helper()
	e := buildBattleEngine(t, row)
	e.Start()

	// 断言这个构筑**真的开了火**。
	//
	// ⚠️ 「跑完不报错」不等于「验到了东西」。
	// 一份空构筑（甚至一份槽位全错的构筑）同样会跑完，
	// 且逐条比对全部通过 —— 因为两端都发 0 发。
	//
	// 所以在开跑前就确认构筑是有效的：
	// 4 个技能、槽位 0..3、热量成本非零。
	if len(row.Equipped) == 0 {
		t.Fatal("构筑为空 —— 这条守卫会变成空跑")
	}
	for i, id := range row.Equipped {
		if i >= ActiveSlots {
			t.Fatalf("构筑有 %d 个技能，超过 activeSlots=%d —— "+
				"多出来的槽位永远是「被动」，等于没验", len(row.Equipped), ActiveSlots)
		}
		if id <= 0 {
			t.Fatalf("构筑[%d] 的技能 id = %d，非正", i, id)
		}
	}

	var pendingEvents []string
	for _, ev := range e.DrainEvents() {
		pendingEvents = append(pendingEvents, formatEvent(ev))
	}

	next := 0
	replayCursor := 0
	mustMatch := func() {
		if next >= len(row.Trace) {
			t.Fatalf("第 %d 步已跑完，但 Go 侧还有快照要产生。tick=%d phase=%q",
				next, e.Tick, e.Phase)
		}
		checkBattleStep(t, e, row.Trace[next], next, &replayCursor, pendingEvents)
		pendingEvents = nil
		next++
	}
	mustMatch()

	for i := 0; i < row.Ticks; i++ {
		if e.Phase != PhaseWave && e.Phase != PhasePrepare {
			break
		}
		before := len(e.replay)
		e.Step()
		for _, ev := range e.DrainEvents() {
			pendingEvents = append(pendingEvents, formatEvent(ev))
		}
		if e.Tick%int64(row.SampleEvery) == 0 || len(e.replay) > before {
			mustMatch()
		}
	}
	if next < len(row.Trace) || len(pendingEvents) > 0 {
		mustMatch()
	}
	if next != len(row.Trace) {
		t.Fatalf("向量有 %d 步快照，Go 只跑了 %d 步 —— 抽样判据两端不一致",
			len(row.Trace), next)
	}

	// ⚠️ 收尾断言：这条场景**必须真的打过架**。
	//
	// 若 shots == 0，最可能的原因不是「引擎坏了」，而是
	// 「构筑没被识别成主动槽」或「热量一 tick 就满」。
	// 两种情况下逐条比对都会通过 —— 因为两端都是 0 发。
	if e.Shots == 0 {
		t.Fatalf("整局开火 0 发 —— 这条场景实际上没验到任何东西。" +
			"检查槽位映射（slot 必须 = 列表下标，不是技能 id）与热量上限")
	}
	if e.Hits > e.Shots {
		t.Fatalf("hits(%d) > shots(%d) —— 服务端 ValidateSettle 会直接拒（ErrInvalidHitRate）。"+
			"溅射/弹射内部不得给 hits 自增", e.Hits, e.Shots)
	}
}

func checkBattleStep(t *testing.T, e *BattleEngine, want battleStep, step int,
	replayCursor *int, gotEvents []string) {
	t.Helper()

	if e.Tick != want.Tick {
		t.Fatalf("第 %d 步 tick = %d，TS 期望 %d", step, e.Tick, want.Tick)
	}
	if e.Phase != want.Phase {
		t.Fatalf("第 %d 步 phase = %q，TS 期望 %q", step, e.Phase, want.Phase)
	}
	if e.ElapsedMs != want.ElapsedMs {
		t.Fatalf("第 %d 步 elapsedMs = %d，TS 期望 %d", step, e.ElapsedMs, want.ElapsedMs)
	}

	// 标量状态
	for _, f := range []struct {
		name     string
		got, exp string
	}{
		{"base_hp", strconv.FormatInt(e.BaseHP, 10), want.BaseHP},
		{"score", strconv.FormatInt(e.Score, 10), want.Score},
		{"heat", strconv.FormatInt(e.Heat.Heat, 10), want.Heat},
		{"heat_cap", strconv.FormatInt(e.Heat.Cap(), 10), want.HeatCap},
		{"shots", strconv.FormatInt(e.Shots, 10), strconv.FormatInt(want.Shots, 10)},
		{"hits", strconv.FormatInt(e.Hits, 10), strconv.FormatInt(want.Hits, 10)},
		{"kills", strconv.FormatInt(e.Kills, 10), strconv.FormatInt(want.Kills, 10)},
		{"leaked", strconv.FormatInt(e.Leaked, 10), strconv.FormatInt(want.Leaked, 10)},
		{"reactions", strconv.FormatInt(e.ReactionsCount, 10), strconv.FormatInt(want.Reactions, 10)},
	} {
		if f.got != f.exp {
			t.Fatalf("第 %d 步 %s = %s，TS 期望 %s", step, f.name, f.got, f.exp)
		}
	}
	if e.Heat.Overheated != want.Overheated {
		t.Fatalf("第 %d 步 overheated = %v，TS 期望 %v", step, e.Heat.Overheated, want.Overheated)
	}
	if got := mapJSON(e.ElementsUsed); got != want.ElementsUsed {
		t.Fatalf("第 %d 步 elementsUsed = %s，TS 期望 %s", step, got, want.ElementsUsed)
	}
	if got := mapJSON(e.ReactionsUsed); got != want.ReactionsUsed {
		t.Fatalf("第 %d 步 reactionsUsed = %s，TS 期望 %s", step, got, want.ReactionsUsed)
	}

	// 渲染事件
	if strings.Join(gotEvents, ",") != strings.Join(want.Events, ",") {
		t.Fatalf("第 %d 步渲染事件不一致（tick=%d）：\n  Go = [%s]\n  TS = [%s]",
			step, e.Tick, strings.Join(gotEvents, ","), strings.Join(want.Events, ","))
	}

	// 弹丸
	if len(e.Projectiles) != len(want.Projectiles) {
		t.Fatalf("第 %d 步飞行中弹丸数 = %d，TS 期望 %d（tick=%d）",
			step, len(e.Projectiles), len(want.Projectiles), e.Tick)
	}
	for i, p := range e.Projectiles {
		w := want.Projectiles[i]
		if p.UID != w.UID || p.SkillID != w.SkillID {
			t.Fatalf("第 %d 步弹丸[%d] uid=%d skill=%d，TS 期望 uid=%d skill=%d",
				step, i, p.UID, p.SkillID, w.UID, w.SkillID)
		}
		for _, f := range []struct {
			name     string
			got, exp string
		}{
			{"x", strconv.FormatInt(p.X, 10), w.X},
			{"y", strconv.FormatInt(p.Y, 10), w.Y},
			{"vx", strconv.FormatInt(p.VX, 10), w.VX},
			{"vy", strconv.FormatInt(p.VY, 10), w.VY},
		} {
			if f.got != f.exp {
				t.Fatalf("第 %d 步弹丸[%d] %s = %s，TS 期望 %s", step, i, f.name, f.got, f.exp)
			}
		}
		if p.PierceLeft != w.PierceLeft {
			t.Fatalf("第 %d 步弹丸[%d] pierceLeft = %d，TS 期望 %d", step, i, p.PierceLeft, w.PierceLeft)
		}
		if p.ChainLeft != w.ChainLeft {
			t.Fatalf("第 %d 步弹丸[%d] chainLeft = %d，TS 期望 %d", step, i, p.ChainLeft, w.ChainLeft)
		}
		if p.AoeRadius != w.AoeRadius {
			t.Fatalf("第 %d 步弹丸[%d] aoeRadius = %d，TS 期望 %d", step, i, p.AoeRadius, w.AoeRadius)
		}
		if p.Dead != w.Dead {
			t.Fatalf("第 %d 步弹丸[%d] dead = %v，TS 期望 %v", step, i, p.Dead, w.Dead)
		}
		if got := hitSetString(p.HitSet); got != w.HitSet {
			t.Fatalf("第 %d 步弹丸[%d] hitSet = %q，TS 期望 %q", step, i, got, w.HitSet)
		}
	}

	// 敌人
	if len(e.Enemies) != len(want.Enemies) {
		t.Fatalf("第 %d 步场上敌人数 = %d，TS 期望 %d（tick=%d）",
			step, len(e.Enemies), len(want.Enemies), e.Tick)
	}
	for i, en := range e.Enemies {
		w := want.Enemies[i]
		if en.UID != w.UID || en.DefID != w.DefID {
			t.Fatalf("第 %d 步敌人[%d] uid=%d def=%d，TS 期望 uid=%d def=%d",
				step, i, en.UID, en.DefID, w.UID, w.DefID)
		}
		for _, f := range []struct {
			name     string
			got, exp string
		}{
			{"x", strconv.FormatInt(en.X, 10), w.X},
			{"y", strconv.FormatInt(en.Y, 10), w.Y},
			{"hp", strconv.FormatInt(en.HP, 10), w.HP},
			{"shield", strconv.FormatInt(en.Shield, 10), w.Shield},
			{"attack_cooldown", strconv.FormatInt(en.AttackCooldown, 10), w.AttackCooldown},
			{"armor_shred_permille", strconv.FormatInt(en.ArmorShredPermille, 10), w.ArmorShredPermille},
			{"knockback", strconv.FormatInt(en.Knockback, 10), w.Knockback},
			{"amplify_permille", strconv.FormatInt(en.AmplifyPermille, 10), w.AmplifyPermille},
		} {
			if f.got != f.exp {
				t.Fatalf("第 %d 步敌人[%d] %s = %s，TS 期望 %s", step, i, f.name, f.got, f.exp)
			}
		}
		if got := stacksString(en.Stacks); got != w.Stacks {
			t.Fatalf("第 %d 步敌人[%d] stacks = %s，TS 期望 %s", step, i, got, w.Stacks)
		}
		for _, f := range []struct {
			name     string
			got, exp int64
		}{
			{"spawn_progress", en.SpawnProgress, int64(w.SpawnProgress)},
			{"frozen_ms", en.FrozenMs, w.FrozenMs},
			{"stunned_ms", en.StunnedMs, w.StunnedMs},
			{"slowed_ms", en.SlowedMs, w.SlowedMs},
			{"armor_shred_ms", en.ArmorShredMs, w.ArmorShredMs},
			{"hit_flash_ms", en.HitFlashMs, w.HitFlashMs},
		} {
			if f.got != f.exp {
				t.Fatalf("第 %d 步敌人[%d] %s = %d，TS 期望 %d", step, i, f.name, f.got, f.exp)
			}
		}
		if en.Dead != w.Dead {
			t.Fatalf("第 %d 步敌人[%d] dead = %v，TS 期望 %v", step, i, en.Dead, w.Dead)
		}
	}

	// 重放事件（本步新增）
	fresh := make([]string, 0, len(e.replay)-*replayCursor)
	for _, r := range e.replay[*replayCursor:] {
		fresh = append(fresh, fmtReplay(r))
	}
	*replayCursor = len(e.replay)
	if strings.Join(fresh, ";") != strings.Join(want.Replay, ";") {
		t.Fatalf("第 %d 步重放事件不一致（tick=%d）：\n  Go = [%s]\n  TS = [%s]",
			step, e.Tick, strings.Join(fresh, ";"), strings.Join(want.Replay, ";"))
	}
}

// mapJSON 把 map[string]int64 序列化成与 TS `JSON.stringify` 相同的字符串。
//
// ⚠️ 必须**排序后**输出：
//
//	TS 的 `JSON.stringify` 按插入序输出整数键（整数键会升序），
//	而 Go 的 `encoding/json` 对 map 也会排序 —— 但只保证一致，
//	不保证与「首次插入序」一致。
//	而 TS 的 elementsUsed 是按**首次使用顺序**插入的，
//	与升序可能不同。
//
//	所以两边都必须按**键排序**再序列化，
//	否则「元素使用统计的差异」会以看似无意义的顺序差出现。
func mapJSON(m map[string]int64) string {
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	// 简单插入排序：键数量极小（<= 5 个元素 + <= 7 个反应）
	for i := 1; i < len(keys); i++ {
		for j := i; j > 0 && keys[j] < keys[j-1]; j-- {
			keys[j], keys[j-1] = keys[j-1], keys[j]
		}
	}
	parts := make([]string, 0, len(keys))
	for _, k := range keys {
		parts = append(parts, `"`+k+`":`+strconv.FormatInt(m[k], 10))
	}
	return "{" + strings.Join(parts, ",") + "}"
}

// hitSetString 把命中集合序列化成逗号分隔的升序 uid 列表。
func hitSetString(s map[int64]bool) string {
	uids := make([]int64, 0, len(s))
	for uid, hit := range s {
		if hit {
			uids = append(uids, uid)
		}
	}
	for i := 1; i < len(uids); i++ {
		for j := i; j > 0 && uids[j] < uids[j-1]; j-- {
			uids[j], uids[j-1] = uids[j-1], uids[j]
		}
	}
	parts := make([]string, 0, len(uids))
	for _, uid := range uids {
		parts = append(parts, strconv.FormatInt(uid, 10))
	}
	return strings.Join(parts, ",")
}

// stacksString 把元素层数序列化成 `fire:0|ice:1|...` 形式。
func stacksString(m map[Element]int64) string {
	parts := make([]string, 0, 5)
	for _, el := range AllElements() {
		parts = append(parts, string(el)+":"+strconv.FormatInt(m[el], 10))
	}
	return strings.Join(parts, "|")
}

var _ = json.Marshal

// mustInt64 解析向量里的定点数字符串。
//
// ⚠️ 向量里所有定点量都用**字符串**而不是 JSON number：
//
//	JS 的 JSON 没有 int64，而定点坐标量级到 10^9 以上。
//	用 number 存再解析，跨过 2^53 之后两端会给出不同的值 ——
//	症状是「坐标在几十万的位置上开始差 1」，而根因在序列化层。
