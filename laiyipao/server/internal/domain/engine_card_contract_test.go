package domain

// Block F（卡牌）的跨端对齐守卫。
//
// # 这是 R1 第一次能跑过多波次
//
// B~E 的所有对齐场景都停在第 1 波末尾的 `card_select` ——
// 硬跑满 tick 只会得到一片死空间。F 落地后场景能真正走完第 2、3、… 波，
// 而多波次是 `charge_tower`（需要 38+ 击杀）在 E 块里不可达的唯一原因。
//
// 现在的实测：`card_charge_tower` 场景跑出 `used=[charge_tower]` ——
// 蓄能塔真的被充能并触发了。

import (
	"encoding/json"
	"fmt"
	"strconv"
	"strings"
	"testing"
)

func buildCardEngine(t *testing.T, v *cardBattleVectors, i int) *BattleEngine {
	t.Helper()
	r := v.Rows[i]
	fx := loadSmokeFixture(t)

	var level smokeLevel
	for j := range fx.Levels {
		if fx.Levels[j].ID == r.LevelID {
			level = fx.Levels[j]
			break
		}
	}
	if level.ID == 0 {
		t.Fatalf("夹具里没有第 %d 关", r.LevelID)
	}
	if len(level.Terrain) != len(r.TerrainKinds) {
		t.Fatalf("关卡 %d 有 %d 个地形，向量声明 %d 个",
			r.LevelID, len(level.Terrain), len(r.TerrainKinds))
	}

	waves := make([]Wave, 0, len(level.Waves))
	for _, w := range level.Waves {
		wv := Wave{Index: w.Index}
		for _, s := range w.Spawns {
			wv.Spawns = append(wv.Spawns, Spawn{
				EnemyID: s.EnemyID, Count: s.Count,
				Interval: int(s.Interval), Delay: int(s.Delay),
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
	// ⚠️ 内容表必须是**全量**（含 composite），不能只放已装备的那几个：
	//   `rollWaveCards` 的抽牌池来自 `cfg.skills` 的全部已解锁技能，
	//   池子小了就抽不到真实玩家会抽到的牌。
	//
	// ⚠️⚠️ 用 `sk.ID` 而不是 `range` 的下标做键：
	//   `range` 切片给出的是**下标**，用它做 map 键会让
	//   `skillDefs[1]` 变成「下标为 1 的那条」——
	//   实测症状是「Go 侧装的是 composite 技能而 TS 装的是 base 技能」，
	//   两者 id 完全不同（1 vs 32），一眼就能看出但很难联想到 range 语义。
	for _, sk := range fx.AllSkills {
		skillDefs[sk.ID] = sk
	}

	equipped := make([]EquippedSkill, 0, len(r.Equipped))
	for slot, id := range r.Equipped {
		d, ok := skillDefs[id]
		if !ok {
			t.Fatalf("技能 id %d 不在夹具里", id)
		}
		equipped = append(equipped, EquippedSkill{
			SkillID: d.ID, Name: d.Name, Element: d.Element, Kind: d.Kind,
			HeatCost: d.HeatCost, CooldownMs: d.CooldownMs,
			Pierce: int(d.Pierce), AoeRadius: d.AoeRadius, BaseDamage: d.BaseDamage,
			ApplyElement: d.ApplyElement, ApplyStacks: d.ApplyStacks,
			ProjectileSpeed: d.ProjectileSpeed, Chain: int(d.Chain), Slot: slot,
		})
	}

	seed, err := strconv.ParseInt(r.Seed, 10, 64)
	if err != nil {
		t.Fatalf("种子 %q 解析失败：%v", r.Seed, err)
	}

	e := NewBattleEngine(BattleConfig{
		Level: GeneratedLevel{
			ID: r.LevelID, Waves: waves, BaseHP: level.BaseHP,
			ArmorPermille: level.ArmorPermille,
			ElementCap:    level.ElementCap, MaxReactionTier: level.MaxReactionTier,
			Terrain: level.Terrain,
		},
		Enemies: enemyDefs, Skills: skillDefs, Equipped: equipped,
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
		Seed: seed, ScoreRules: DefaultScoreRules(), ActiveSlots: ActiveSlots,
	})
	e.SetReplayScript(r.Picks)
	return e
}

func TestCardScenariosMatchCrossEnd(t *testing.T) {
	v := loadCardBattleVectors(t)
	if len(v.Rows) == 0 {
		t.Fatal("向量里没有卡牌场景 —— F 块的跨端对齐是空跑")
	}
	for i := range v.Rows {
		row := v.Rows[i]
		t.Run(row.Label, func(t *testing.T) {
			e := buildCardEngine(t, &v, i)
			e.Start()

			next := 0
			replayCursor := 0
			var pendingEvents []string
			for _, ev := range e.DrainEvents() {
				pendingEvents = append(pendingEvents, formatEvent(ev))
			}
			mustMatch := func() {
				if next >= len(row.Trace) {
					t.Fatalf("第 %d 步已跑完，但 Go 侧还有快照要产生。tick=%d phase=%q",
						next, e.Tick, e.Phase)
				}
				checkCardStep(t, e, row.Trace[next], next, &replayCursor, pendingEvents)
				pendingEvents = nil
				next++
			}
			mustMatch()

			for k := 0; k < row.Ticks; k++ {
				// ⚠️ **不能**在 card_select 停下：
				//   解卡牌是 step() 内部做的（applyReplayDecision），
				//   break 会正好跳过它 —— 于是 card_picks 永远为空，
				//   整条守卫变成「两端都发 0 次牌」的同义反复。
				if e.Phase != PhaseWave && e.Phase != PhasePrepare && e.Phase != PhaseCardSelect {
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
				t.Fatalf("向量有 %d 步快照，Go 只跑了 %d 步", len(row.Trace), next)
			}
			// ⚠️ 收尾断言：必须真的**选过牌**且**跑过多波**。
			//   「跑完不报错」不等于「验到了东西」。
			if len(e.CardPicks) == 0 {
				t.Fatal("card_picks 为空 —— 选牌脚本根本没被执行")
			}
			if e.WaveIndex < 1 {
				t.Fatalf("只跑到第 %d 波 —— 多波次路径没被覆盖", e.WaveIndex)
			}
		})
	}
}

func checkCardStep(t *testing.T, e *BattleEngine, want cardStep, step int,
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
	for _, f := range []struct {
		name     string
		got, exp string
	}{
		{"base_hp", strconv.FormatInt(e.BaseHP, 10), want.BaseHP},
		{"score", strconv.FormatInt(e.Score, 10), want.Score},
		{"heat", strconv.FormatInt(e.Heat.Heat, 10), want.Heat},
		{"heat_cap", strconv.FormatInt(e.Heat.Cap(), 10), want.HeatCap},
		{"card_picks", joinInts(e.CardPicks), want.CardPicks},
		{"terrain_used", strings.Join(e.TerrainUsed, ","), want.TerrainUsed},
	} {
		if f.got != f.exp {
			t.Fatalf("第 %d 步 %s = [%s]，TS 期望 [%s]", step, f.name, f.got, f.exp)
		}
	}
	for _, f := range []struct {
		name     string
		got, exp int64
	}{
		{"shots", e.Shots, want.Shots},
		{"hits", e.Hits, want.Hits},
		{"kills", e.Kills, want.Kills},
		{"leaked", e.Leaked, want.Leaked},
	} {
		if f.got != f.exp {
			t.Fatalf("第 %d 步 %s = %d，TS 期望 %d", step, f.name, f.got, f.exp)
		}
	}
	if e.Heat.Overheated != want.Overheated {
		t.Fatalf("第 %d 步 overheated = %v，TS 期望 %v", step, e.Heat.Overheated, want.Overheated)
	}
	if e.Deck.DiscardsLeft != want.DiscardsLeft {
		t.Fatalf("第 %d 步 discardsLeft = %d，TS 期望 %d —— "+
			"「快速装填」卡要同时加到 Buffs 与当前剩余次数上",
			step, e.Deck.DiscardsLeft, want.DiscardsLeft)
	}

	// 手牌
	gotHand := make([]string, 0, len(e.Deck.Hand))
	for _, c := range e.Deck.Hand {
		gotHand = append(gotHand, c.Kind+":"+c.ID)
	}
	if strings.Join(gotHand, ",") != strings.Join(want.Hand, ",") {
		t.Fatalf("第 %d 步手牌 = [%s]，TS 期望 [%s] —— "+
			"rollWaveCards 的抽取顺序与 id 生成都进 RNG，错了后续全部偏移",
			step, strings.Join(gotHand, ","), strings.Join(want.Hand, ","))
	}

	// 技能状态（升格会改 baseDamage / applyStacks / pierce）
	gotSkills := make([]string, 0, len(e.Skills))
	for _, s := range e.Skills {
		gotSkills = append(gotSkills,
			strconv.Itoa(s.SkillID)+"@"+strconv.Itoa(s.Slot)+":"+
				strconv.FormatInt(s.BaseDamage, 10)+":"+
				strconv.FormatInt(s.ApplyStacks, 10)+":"+
				strconv.Itoa(s.Pierce)+":"+
				strconv.FormatInt(s.CooldownRemaining, 10))
	}
	if strings.Join(gotSkills, ",") != strings.Join(want.Skills, ",") {
		t.Fatalf("第 %d 步技能 = [%s]，TS 期望 [%s]", step,
			strings.Join(gotSkills, ","), strings.Join(want.Skills, ","))
	}

	// 局内加成
	if msg := compareBuffs(e, want.Buffs); msg != "" {
		t.Fatalf("第 %d 步局内加成：%s", step, msg)
	}

	// 地形
	if len(e.Terrains) != len(want.Terrains) {
		t.Fatalf("第 %d 步地形数 = %d，TS 期望 %d", step, len(e.Terrains), len(want.Terrains))
	}
	for i, tr := range e.Terrains {
		w := want.Terrains[i]
		if tr.Kind != w.Kind || tr.State != w.State ||
			strconv.FormatInt(tr.Timer, 10) != w.Timer ||
			tr.Charge != w.Charge || tr.Angle != w.Angle || tr.Triggered != w.Triggered {
			t.Fatalf("第 %d 步地形[%d] = %s/%s/t=%s/c=%d/a=%d/trig=%v，TS 期望 %s/%s/t=%s/c=%d/a=%d/trig=%v",
				step, i, tr.Kind, tr.State, strconv.FormatInt(tr.Timer, 10), tr.Charge, tr.Angle, tr.Triggered,
				w.Kind, w.State, w.Timer, w.Charge, w.Angle, w.Triggered)
		}
	}

	// 敌人
	if len(e.Enemies) != len(want.Enemies) {
		t.Fatalf("第 %d 步场上敌人数 = %d，TS 期望 %d", step, len(e.Enemies), len(want.Enemies))
	}
	for i, en := range e.Enemies {
		w := want.Enemies[i]
		if got := strconv.FormatInt(en.X, 10); got != w.X {
			t.Fatalf("第 %d 步敌人[%d] x = %s，TS 期望 %s", step, i, got, w.X)
		}
		if got := strconv.FormatInt(en.HP, 10); got != w.HP {
			t.Fatalf("第 %d 步敌人[%d] hp = %s，TS 期望 %s", step, i, got, w.HP)
		}
	}

	// 事件与重放
	if strings.Join(gotEvents, ",") != strings.Join(want.Events, ",") {
		t.Fatalf("第 %d 步渲染事件不一致（tick=%d）：\n  Go = [%s]\n  TS = [%s]",
			step, e.Tick, strings.Join(gotEvents, ","), strings.Join(want.Events, ","))
	}
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

// joinInts 把选牌序列格式化成逗号分隔。
func joinInts(v []int) string {
	parts := make([]string, 0, len(v))
	for _, x := range v {
		parts = append(parts, strconv.Itoa(x))
	}
	return strings.Join(parts, ",")
}

// buffsJSON 把局内加成编码成 JSON。
//
// ⚠️ 值一律用**字符串**编码：与向量生成器一致（BigInt -> "123"），
// 且避免 JS number 在 2^53 以上的精度问题。
func buffsJSON(e *BattleEngine) string {
	type kv struct{ k, v string }
	fields := []kv{
		{"attack", strconv.FormatInt(e.Buffs.AttackPermille, 10)},
		{"element_coef", strconv.FormatInt(e.Buffs.ElementCoefPermille, 10)},
		{"crit", strconv.FormatInt(e.Buffs.CritPermille, 10)},
		{"element_cap", strconv.FormatInt(e.Buffs.ElementCapBonus, 10)},
		{"armor", strconv.FormatInt(e.Buffs.ArmorPermille, 10)},
		{"pierce", strconv.Itoa(e.Buffs.PierceBonus)},
		{"chain", strconv.Itoa(e.Buffs.ChainBonus)},
		{"aoe", strconv.Itoa(e.Buffs.AoeBonus)},
		{"free_discard", strconv.Itoa(e.Buffs.FreeDiscard)},
		{"overheat_guard", strconv.Itoa(e.Buffs.OverheatGuard)},
		// heat_cap_bonus 单列：它不是 Buffs 的一部分，
		// 被 Heat.Cap() 单独读走 —— 混进 Buffs 就看不出
		// 「散热涂层」卡到底有没有生效。
		{"heat_cap_bonus", strconv.FormatInt(e.Heat.CapBonus, 10)},
	}
	parts := make([]string, 0, len(fields))
	for _, f := range fields {
		parts = append(parts, strconv.Quote(f.k)+":"+strconv.Quote(f.v))
	}
	return "{" + strings.Join(parts, ",") + "}"
}

// compareBuffs 逐字段比较局内加成。
//
// # 为什么**解码 JSON** 再比，而不是逐字符比对两个字符串
//
// 两端各写一遍格式化逻辑时，格式差异（定点量加不加引号、空格、键序）
// 会淹没真正的值差异。我第一版就是逐字符比 `buffsJSON(e)` 与向量里的
// 字符串，结果花了好几轮才定位到「Go 侧的反引号原始字符串字面量里
// 写了 `\"`，而原始字面量不转义，反斜杠被原样写进结果」——
// 症状是「Go 比 TS 长 1 个字符」，而两边日志肉眼看起来完全一样。
//
// 现在两端都产出 JSON，Go 侧解析回 `map[string]interface{}` 再逐字段比：
// 格式问题在编码/解码两侧各自解决一次，比对环节只关心「值」。
func compareBuffs(e *BattleEngine, want string) string {
	var m map[string]interface{}
	if err := json.Unmarshal([]byte(want), &m); err != nil {
		return "向量里的 buffs 不是合法 JSON：" + err.Error()
	}
	var got map[string]interface{}
	if err := json.Unmarshal([]byte(buffsJSON(e)), &got); err != nil {
		return "Go 侧的 buffs 不是合法 JSON：" + err.Error()
	}
	for k, wv := range m {
		gv, ok := got[k]
		if !ok {
			return "Go 侧缺少字段 " + k
		}
		if fmt.Sprint(gv) != fmt.Sprint(wv) {
			return fmt.Sprintf("字段 %s = %v，TS 期望 %v", k, gv, wv)
		}
	}
	for k := range got {
		if _, ok := m[k]; !ok {
			return "Go 侧多出字段 " + k
		}
	}
	return ""
}
