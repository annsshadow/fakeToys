package domain

// Block E（地形接线）的跨端对齐守卫。
//
// # 这一块要还的债
//
// D 块的五个场景**刻意剥离了 `terrain`** —— 那时 Go 侧还没接地形，
// 半接的后果是「地形记了触发事件却从不结算效果」，
// 症状落在数值上（分数、击杀、漏怪）而根因在接线，极难定位。
//
// 现在地形接上了，这里把债还掉：**五类地形各一个真实场景**。
//
// # 五类走完全不同的路径
//
//	`onHit`（油桶/掩体/风障）：弹丸或命中触发
//	`blocksProjectile`（掩体）：弹丸被吃掉
//	`blocked`（潮汐闸）：敌人被推回
//	`onKillCharging`（蓄能塔）：击杀驱动
//	`update`（全部）：每 tick 推进
//
// 只验其中一类，其余四类的接线错误全部观察不到 ——
// 而「接线」正是最容易漏的地方（少一个调用点，不报任何错）。

import (
	"strconv"
	"strings"
	"testing"
)

// terrainRow 是一行地形场景。
type terrainRow struct {
	Label       string
	LevelID     int
	Seed        string
	Ticks       int
	SampleEvery int
	Equipped    []int
	LevelWaves  []waveLevelDef
	// TerrainKinds 是**输入回显**：只声明关卡上有哪些地形，
	// 位置与参数由两端从同一份 `smoke_levels.json` 读。
	//
	// ⚠️ 不回显位置/参数是刻意的：
	//   回显了就变成「用 Go 的地形表生成期望值」，
	//   而期望值必须来自 TS 侧（玩家设备上运行的代码）。
	//   buildTerrainEngine 会重新读夹具并**逐项核对**这里的 kind 列表。
	TerrainKinds []string
	Attacker     Attacker
	Trace        []terrainStep
}

// terrainStep 是一步地形快照的期望值。
//
// ⚠️ 必须记地形的**全部状态**，不只是「触发没触发」：
//
//	油桶的火区计时、蓄能塔的充能、旋转风障的角度都跨 tick 演进 ——
//	只记布尔量的话「状态演进不一致」完全观察不到，
//	而它的症状是「火区早结束一帧」这类极细的偏差。
type terrainStep struct {
	Tick      int64  `json:"tick"`
	Phase     string `json:"phase"`
	ElapsedMs int64  `json:"elapsed_ms"`
	Terrains  []struct {
		Kind      string `json:"kind"`
		X         int    `json:"x"`
		Y         int    `json:"y"`
		State     string `json:"state"`
		Timer     string `json:"timer"`
		Charge    int    `json:"charge"`
		Angle     int    `json:"angle"`
		Triggered bool   `json:"triggered"`
	} `json:"terrains"`
	TerrainUsed string   `json:"terrain_used"`
	Events      []string `json:"events"`
	BaseHP      string   `json:"base_hp"`
	Score       string   `json:"score"`
	Shots       int64    `json:"shots"`
	Hits        int64    `json:"hits"`
	Kills       int64    `json:"kills"`
	Leaked      int64    `json:"leaked"`
	Replay      []string `json:"replay"`
	Enemies     []struct {
		UID int64  `json:"uid"`
		X   string `json:"x"`
		HP  string `json:"hp"`
	} `json:"enemies"`
}

// buildTerrainEngine 按场景重建引擎（**带地形**）。
//
// ⚠️ 地形位置/参数**必须**从关卡数据取，而不是从向量回显的 `terrain_kinds`
// 反推 —— 那样就变成「用 Go 的地形表生成期望值」，
// 而期望值必须来自 TS 侧（玩家设备上运行的代码）。
//
// 所以这里**重新读夹具**，取同一个关卡的 terrain 数组。
func buildTerrainEngine(t *testing.T, row terrainRow) *BattleEngine {
	t.Helper()
	fx := loadSmokeFixture(t)

	var level smokeLevel
	for i := range fx.Levels {
		if fx.Levels[i].ID == row.LevelID {
			level = fx.Levels[i]
			break
		}
	}
	if level.ID == 0 {
		t.Fatalf("夹具里没有第 %d 关", row.LevelID)
	}
	// ⚠️ 必须与向量声明的地形种类**逐项一致**：
	//   向量里只存了 kind（位置/参数由关卡数据决定，两端读同一份夹具），
	//   所以这里要确认「我读到的关卡确实带着这些地形」。
	if len(level.Terrain) != len(row.TerrainKinds) {
		t.Fatalf("关卡 %d 有 %d 个地形，向量声明了 %d 个 —— "+
			"两侧读的不是同一份关卡数据", row.LevelID, len(level.Terrain), len(row.TerrainKinds))
	}
	for i, k := range row.TerrainKinds {
		if level.Terrain[i].Kind != k {
			t.Fatalf("关卡 %d 地形[%d] = %q，向量声明 %q", row.LevelID, i, level.Terrain[i].Kind, k)
		}
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

	equipped := make([]EquippedSkill, 0, len(row.Equipped))
	for i, id := range row.Equipped {
		d, ok := skillDefs[id]
		if !ok {
			t.Fatalf("技能 id %d 不在夹具里", id)
		}
		equipped = append(equipped, EquippedSkill{
			SkillID: d.ID, Name: d.Name, Element: d.Element, Kind: d.Kind,
			HeatCost: d.HeatCost, CooldownMs: d.CooldownMs,
			Pierce: int(d.Pierce), AoeRadius: d.AoeRadius, BaseDamage: d.BaseDamage,
			ApplyElement: d.ApplyElement, ApplyStacks: d.ApplyStacks,
			ProjectileSpeed: d.ProjectileSpeed, Chain: int(d.Chain), Slot: i,
		})
	}

	seed, err := strconv.ParseInt(row.Seed, 10, 64)
	if err != nil {
		t.Fatalf("种子 %q 解析失败：%v", row.Seed, err)
	}

	return NewBattleEngine(BattleConfig{
		Level: GeneratedLevel{
			ID: row.LevelID, Waves: waves, BaseHP: level.BaseHP,
			ArmorPermille: level.ArmorPermille,
			ElementCap:    level.ElementCap, MaxReactionTier: level.MaxReactionTier,
			Terrain: level.Terrain,
		},
		Enemies: enemyDefs, Skills: skillDefs, Equipped: equipped,
		Attacker: row.Attacker, Seed: seed,
		ScoreRules: DefaultScoreRules(), ActiveSlots: ActiveSlots,
	})
}

func TestTerrainScenariosMatchCrossEnd(t *testing.T) {
	v := loadTerrainBattleVectors(t)
	if len(v.Rows) == 0 {
		t.Fatal("向量里没有地形场景 —— E 块的跨端对齐是空跑")
	}
	for _, r := range v.Rows {
		row := terrainRow{
			Label: r.Label, LevelID: r.LevelID, Seed: r.Seed, Ticks: r.Ticks,
			SampleEvery: r.SampleEvery, Equipped: r.Equipped,
			LevelWaves: r.LevelWaves, TerrainKinds: r.TerrainKinds, Trace: r.Trace,
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
		t.Run(row.Label, func(t *testing.T) { runTerrainScenario(t, row) })
	}
}

func runTerrainScenario(t *testing.T, row terrainRow) {
	t.Helper()
	e := buildTerrainEngine(t, row)
	e.Start()

	if len(e.Terrains) == 0 {
		t.Fatal("构造出的引擎没有地形 —— 这条场景在验空跑")
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
		checkTerrainStep(t, e, row.Trace[next], next, &replayCursor, pendingEvents)
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
		t.Fatalf("向量有 %d 步快照，Go 只跑了 %d 步", len(row.Trace), next)
	}
	if e.Shots == 0 {
		t.Fatal("整局开火 0 发 —— 场景实际上没验到任何东西")
	}
}

func checkTerrainStep(t *testing.T, e *BattleEngine, want terrainStep, step int,
	replayCursor *int, gotEvents []string) {
	t.Helper()

	if e.Tick != want.Tick {
		t.Fatalf("第 %d 步 tick = %d，TS 期望 %d", step, e.Tick, want.Tick)
	}
	if e.Phase != want.Phase {
		t.Fatalf("第 %d 步 phase = %q，TS 期望 %q", step, e.Phase, want.Phase)
	}
	for _, f := range []struct {
		name     string
		got, exp string
	}{
		{"base_hp", strconv.FormatInt(e.BaseHP, 10), want.BaseHP},
		{"score", strconv.FormatInt(e.Score, 10), want.Score},
		{"shots", strconv.FormatInt(e.Shots, 10), strconv.FormatInt(want.Shots, 10)},
		{"hits", strconv.FormatInt(e.Hits, 10), strconv.FormatInt(want.Hits, 10)},
		{"kills", strconv.FormatInt(e.Kills, 10), strconv.FormatInt(want.Kills, 10)},
		{"leaked", strconv.FormatInt(e.Leaked, 10), strconv.FormatInt(want.Leaked, 10)},
	} {
		if f.got != f.exp {
			t.Fatalf("第 %d 步 %s = %s，TS 期望 %s", step, f.name, f.got, f.exp)
		}
	}
	if got := strings.Join(e.TerrainUsed, ","); got != want.TerrainUsed {
		t.Fatalf("第 %d 步 terrainUsed = [%s]，TS 期望 [%s]", step, got, want.TerrainUsed)
	}
	if strings.Join(gotEvents, ",") != strings.Join(want.Events, ",") {
		t.Fatalf("第 %d 步渲染事件不一致（tick=%d）：\n  Go = [%s]\n  TS = [%s]",
			step, e.Tick, strings.Join(gotEvents, ","), strings.Join(want.Events, ","))
	}

	// 地形逐个逐字段
	if len(e.Terrains) != len(want.Terrains) {
		t.Fatalf("第 %d 步地形数 = %d，TS 期望 %d", step, len(e.Terrains), len(want.Terrains))
	}
	for i, tr := range e.Terrains {
		w := want.Terrains[i]
		if tr.Kind != w.Kind || tr.X != w.X || tr.Y != w.Y {
			t.Fatalf("第 %d 步地形[%d] = %s@(%d,%d)，TS 期望 %s@(%d,%d)",
				step, i, tr.Kind, tr.X, tr.Y, w.Kind, w.X, w.Y)
		}
		if tr.State != w.State {
			t.Fatalf("第 %d 步地形[%d](%s) state = %q，TS 期望 %q —— "+
				"地形状态机与 TS 不一致", step, i, tr.Kind, tr.State, w.State)
		}
		if got := strconv.FormatInt(tr.Timer, 10); got != w.Timer {
			t.Fatalf("第 %d 步地形[%d](%s) timer = %s，TS 期望 %s —— "+
				"跨 tick 演进的状态量必须逐 tick 比对，只比布尔量看不到这种偏差",
				step, i, tr.Kind, got, w.Timer)
		}
		if tr.Charge != w.Charge {
			t.Fatalf("第 %d 步地形[%d](%s) charge = %d，TS 期望 %d",
				step, i, tr.Kind, tr.Charge, w.Charge)
		}
		if tr.Angle != w.Angle {
			t.Fatalf("第 %d 步地形[%d](%s) angle = %d，TS 期望 %d",
				step, i, tr.Kind, tr.Angle, w.Angle)
		}
		if tr.Triggered != w.Triggered {
			t.Fatalf("第 %d 步地形[%d](%s) triggered = %v，TS 期望 %v",
				step, i, tr.Kind, tr.Triggered, w.Triggered)
		}
	}

	// 敌人位置与血量（潮汐闸的阻挡会改 x）
	if len(e.Enemies) != len(want.Enemies) {
		t.Fatalf("第 %d 步场上敌人数 = %d，TS 期望 %d", step, len(e.Enemies), len(want.Enemies))
	}
	for i, en := range e.Enemies {
		w := want.Enemies[i]
		if en.UID != w.UID {
			t.Fatalf("第 %d 步敌人[%d] uid = %d，TS 期望 %d", step, i, en.UID, w.UID)
		}
		if got := strconv.FormatInt(en.X, 10); got != w.X {
			t.Fatalf("第 %d 步敌人[%d] x = %s，TS 期望 %s —— "+
				"潮汐闸关闭时敌人会被推回闸门右侧，这个值是它的唯一证据",
				step, i, got, w.X)
		}
		if got := strconv.FormatInt(en.HP, 10); got != w.HP {
			t.Fatalf("第 %d 步敌人[%d] hp = %s，TS 期望 %s —— "+
				"油桶火区的每 tick 伤害从这里体现", step, i, got, w.HP)
		}
	}

	// 重放事件
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
