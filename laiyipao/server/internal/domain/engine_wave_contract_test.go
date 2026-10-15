package domain

// Block B / C（波次刷怪 + 敌人推进与漏怪）的**跨端逐事件对齐**守卫。
//
// # 这是 R1 移植过程中第一次真正的跨端对齐
//
// A 块只能验排序不变量（`BeginWave` 还是空实现，事件流必然不一致）。
// B/C 把它换成真实实现之后，「同一关 + 同一种子 + 同一空构筑」下，
// 两端的 tick / phase / elapsed_ms / 刷怪队列 / 每个敌人的全部字段 /
// 防线血量 / 全部重放事件 / 全部渲染事件 都必须逐条相同。
//
// # ⚠️ 为什么逐步比对而不是只比最终状态
//
// 只比最终值的话，中间若干 tick 错了又绕回来的实现能全绿。
// 而刷怪队列的长度是「非增」的，中间的错误很容易被后续追平。

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
)

// smokeLevel 是夹具里一个关卡的形状。
//
// ⚠️ 具名类型而不是匿名结构体：
//
//	`buildWaveEngine` 与 `buildTerrainEngine` 都要用同一个形状，
//	匿名写法必须逐字复制 tag —— 复制中改错一个就是「静默得到零值」。
type smokeLevel struct {
	ID              int                `json:"id"`
	BaseHP          int64              `json:"base_hp"`
	ArmorPermille   int64              `json:"armor_permille"`
	ElementCap      int64              `json:"element_cap"`
	MaxReactionTier int64              `json:"max_reaction_tier"`
	Waves           []waveLevelDef     `json:"waves"`
	Terrain         []TerrainPlacement `json:"terrain"`
}

// loadSmokeFixture 读服务端关卡夹具（关卡 + 敌人表 + 技能表）。
//
// ⚠️ 必须用**同一份**夹具：向量里的 `level_waves` 是从这里取的，
//
//	地形场景的 `terrain_kinds` 也是从这里核对的。
//	Go 侧若改用 `GenerateLevel` 重新生成，刷怪组可能不同 ——
//	而 levelgen 是确定性的，两条路本该一致，但那只是「巧合」不是「同一份数据」。
//	混用两个来源时差异会以「刷怪数量不同」的形式出现，
//	而排查方向会指向错误的一边。
func loadSmokeFixture(t *testing.T) struct {
	Levels    []smokeLevel `json:"levels"`
	Enemies   []SeedEnemy  `json:"enemies"`
	Skills    []SeedSkill  `json:"skills"`
	AllSkills []SeedSkill  `json:"composite_skills"`
} {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "..", "testdata", "smoke_levels.json"))
	if err != nil {
		t.Fatalf("读夹具失败：%v", err)
	}
	var f struct {
		Levels    []smokeLevel `json:"levels"`
		Enemies   []SeedEnemy  `json:"enemies"`
		Skills    []SeedSkill  `json:"skills"`
		AllSkills []SeedSkill  `json:"composite_skills"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		t.Fatalf("夹具解析失败：%v", err)
	}
	return f
}

// waveLevelDef 是关卡的一波（输入回显）。
type waveLevelDef struct {
	Index  int `json:"wave_index"`
	Spawns []struct {
		EnemyID  int   `json:"enemy_id"`
		Count    int   `json:"count"`
		Interval int64 `json:"interval"`
		Delay    int64 `json:"delay"`
	} `json:"spawns"`
}

// waveTraceStep 是一步的期望快照。
//
// ⚠️ 用**具名类型**而不是匿名结构体：
//
//	这个形状在 `engine_pure_contract_test.go` 的反序列化结构里也要用一次，
//	匿名写法必须逐字复制 tag —— 复制时改错一个就是「静默得到零值」。
type waveTraceStep struct {
	Tick       int64  `json:"tick"`
	Phase      string `json:"phase"`
	ElapsedMs  int64  `json:"elapsed_ms"`
	SpawnQueue []struct {
		EnemyID int   `json:"enemy_id"`
		AtTick  int64 `json:"at_tick"`
		UID     int64 `json:"uid"`
	} `json:"spawn_queue"`
	Enemies []struct {
		UID            int64  `json:"uid"`
		DefID          int    `json:"def_id"`
		X              string `json:"x"`
		Y              string `json:"y"`
		HP             string `json:"hp"`
		Shield         string `json:"shield"`
		ArmorPermille  string `json:"armor_permille"`
		SpawnProgress  int    `json:"spawn_progress"`
		Dead           bool   `json:"dead"`
		Resist         string `json:"resist"`
		AttackCooldown string `json:"attack_cooldown"`
		AttackRange    string `json:"attack_range"`
	} `json:"enemies"`
	// Replay 是**本步新增**的重放事件（不是全量）。
	//
	// ⚠️ 增量是必需的：全量存 22000 步会让向量文件变成 42 MB，
	//   而没人会 review 42 MB —— 没人看的契约等于没有契约。
	Replay []string `json:"replay"`
	// Events 是本步 drain 出的渲染事件，形如 `type:damage`。
	Events []string `json:"events"`
	// BaseHP / Leaked 是防线状态。
	//
	// ⚠️ BaseHP 必须逐步比对：只看终值的话，
	// 「同一 tick 漏了两只」与「两个 tick 各漏一只」终值相同，
	// 而那是两种完全不同的战斗。
	BaseHP string `json:"base_hp"`
	Leaked int64  `json:"leaked"`
}

// waveRow 是从向量里取出的一行场景。
type waveRow struct {
	Label                 string
	LevelID               int
	Seed                  string
	Ticks                 int
	SampleEvery           int
	AttackerArmorPermille int64
	LevelBaseHP           int64
	LevelArmorPermille    int64
	LevelWaves            []waveLevelDef
	Trace                 []waveTraceStep
}

// buildWaveEngine 按向量里的场景重建引擎。
//
// ⚠️ 必须用**向量回显的那份关卡数据**（`LevelWaves` / `LevelBaseHP` /
// `LevelArmorPermille`），而不是 `GenerateLevel` 重新生成。
//
// levelgen 是确定性的，两条路本该一致 —— 但那是「巧合」不是「同一份数据」。
// 混用两个来源时差异会以「刷怪数量不同」或「漏怪伤害不同」的形式出现，
// 而排查方向会指向错误的一边。
func buildWaveEngine(t *testing.T, row waveRow) *BattleEngine {
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

	seed, err := strconv.ParseInt(row.Seed, 10, 64)
	if err != nil {
		t.Fatalf("种子 %q 解析失败：%v", row.Seed, err)
	}

	level := GeneratedLevel{
		ID:            row.LevelID,
		Waves:         waves,
		BaseHP:        row.LevelBaseHP,
		ArmorPermille: row.LevelArmorPermille,
	}
	return NewBattleEngine(BattleConfig{
		Level: level,
		// ⚠️ 敌人表用**服务端内容表**，关卡用**向量回显**：
		//   敌人属性（attack / attackRange / attackInterval / armor）
		//   是漏怪伤害的直接输入，必须与线上跑的是同一份。
		Enemies:     enemyDefs,
		Skills:      skillDefs,
		Equipped:    nil,
		Attacker:    Attacker{ArmorPermille: row.AttackerArmorPermille},
		Seed:        seed,
		ScoreRules:  DefaultScoreRules(),
		ActiveSlots: ActiveSlots,
	})
}

func TestWaveScenariosMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)

	// 常量也要对齐
	if v.Wave.Consts.TickMs != TickMs {
		t.Errorf("TICK_MS = %d，TS 期望 %d", TickMs, v.Wave.Consts.TickMs)
	}
	if v.Wave.Consts.SpawnProgressFull != SpawnProgressFull {
		t.Errorf("SPAWN_PROGRESS_FULL = %d，TS 期望 %d",
			SpawnProgressFull, v.Wave.Consts.SpawnProgressFull)
	}
	if v.Wave.Consts.SpawnProgressStep != SpawnProgressStep {
		t.Errorf("SPAWN_PROGRESS_STEP = %d，TS 期望 %d",
			SpawnProgressStep, v.Wave.Consts.SpawnProgressStep)
	}

	for _, r := range v.Wave.Rows {
		row := waveRow{
			Label: r.Label, LevelID: r.LevelID, Seed: r.Seed, Ticks: r.Ticks,
			SampleEvery:           r.SampleEvery,
			AttackerArmorPermille: r.AttackerArmorPermille,
			LevelBaseHP:           r.LevelBaseHP,
			LevelArmorPermille:    r.LevelArmorPermille,
			LevelWaves:            r.LevelWaves,
			Trace:                 r.Trace,
		}
		t.Run(row.Label, func(t *testing.T) { runWaveScenario(t, row) })
	}
}

// runWaveScenario 按抽样协议重放一个场景。
//
// # ⚠️ 抽样判据必须**由 Go 自己算**，不能照抄 trace 的下标
//
// 照抄下标等于「Go 跑到第 k 个采样点就断言第 k 个期望值」，
// 两端之间没有任何时序约束 —— 那会退化成逐点查表，
// 「Go 少跑了两步」这种错误就永远抓不到。
//
// 这里用与生成器**逐字相同**的判据：
//
//	tick % sampleEvery == 0  ||  本步新增了重放事件
//
// 两端各自算得出「这一步要不要快照」，所以期望值与实际推进严格绑定。
func runWaveScenario(t *testing.T, row waveRow) {
	t.Helper()
	if row.SampleEvery <= 0 {
		t.Fatalf("sample_every = %d，必须 >= 1", row.SampleEvery)
	}
	e := buildWaveEngine(t, row)
	e.Start()

	// ⚠️ `Start()` 自己也会发事件（`wave_start`），必须在这里排空。
	//
	//   生成器的 `snap()` 内部就调了 `drainEvents()`，
	//   所以向量的第 0 步**已经包含**了 start 的事件。
	//   Go 侧若不排空，第 0 步会比成「Go 空 vs TS 有 wave_start」。
	var pendingEvents []string
	for _, ev := range e.DrainEvents() {
		pendingEvents = append(pendingEvents, formatEvent(ev))
	}

	// 期望值游标：trace[0] 对应 start() 之后的那一步。
	next := 0
	// 重放事件游标：向量里存的是本步新增，Go 侧要数着数。
	replayCursor := 0

	mustMatch := func() {
		if next >= len(row.Trace) {
			t.Fatalf("第 %d 步已跑完，但 Go 侧还有快照要产生 —— "+
				"向量比 Go 的轨迹短。tick=%d phase=%q", next, e.Tick, e.Phase)
		}
		checkWaveTick(t, e, row.Trace[next], next, &replayCursor, pendingEvents)
		pendingEvents = nil
		next++
	}
	mustMatch()

	for i := 0; i < row.Ticks; i++ {
		// ⚠️ 必须与生成器**逐字相同**的终止条件。
		//
		//	写成「won 或 lost」的话，`card_select` 会被漏掉 ——
		//	Go 会在一个不再推进的状态上继续 step 到 tick 上限，
		//	而 `checkWaveEnd` 每 tick 都会再发一次 wave_clear。
		//	症状是「后半段全是重复事件」，看起来像对不上，
		//	实际是终止条件写窄了。
		//
		//	写成「既不是 wave 也不是 prepare」还能自动涵盖将来新增的终局状态。
		if e.Phase != PhaseWave && e.Phase != PhasePrepare {
			break
		}
		before := len(e.replay)
		e.Step()
		for _, ev := range e.DrainEvents() {
			pendingEvents = append(pendingEvents, formatEvent(ev))
		}
		if e.Tick%int64(row.SampleEvery) == 0 || len(e.replay) > before || len(pendingEvents) > 0 {
			mustMatch()
		}
	}
	// ⚠️ 收尾那一步：生成器在「最后一步既不是抽样点、也没产生 replay 事件」
	//   时会补一次快照（否则 Go 侧靠 phase 判定终局后不再 step，那步就没有期望值）。
	if next < len(row.Trace) || len(pendingEvents) > 0 {
		mustMatch()
	}
	if next != len(row.Trace) {
		t.Fatalf("向量有 %d 步快照，Go 只跑了 %d 步 —— "+
			"场景定义与 trace 长度不一致（多半是抽样判据两端不一致）",
			len(row.Trace), next)
	}
}

// formatEvent 把渲染事件格式化成向量的表示法 `type:damage`。
//
// ⚠️ 非漏怪事件的 damage 是空串 —— 与生成器里的 `ev.damage ?? ”` 一致。
//
//	写成 `0` 会让「没这个字段」与「伤害是 0」变得不可区分。
func formatEvent(ev BattleEvent) string {
	d := ""
	if ev.Damage != 0 {
		d = strconv.FormatInt(ev.Damage, 10)
	}
	return ev.Type + ":" + d
}

func checkWaveTick(t *testing.T, e *BattleEngine, want waveTraceStep, step int,
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
	if got := strconv.FormatInt(e.BaseHP, 10); got != want.BaseHP {
		t.Fatalf("第 %d 步 baseHp = %s，TS 期望 %s", step, got, want.BaseHP)
	}
	if e.Leaked != want.Leaked {
		t.Fatalf("第 %d 步 leaked = %d，TS 期望 %d", step, e.Leaked, want.Leaked)
	}

	// 渲染事件：逐条（含上一个快照点之后那些未被记录的 tick）。
	if strings.Join(gotEvents, ",") != strings.Join(want.Events, ",") {
		t.Fatalf("第 %d 步渲染事件不一致（tick=%d）：\n  Go = [%s]\n  TS = [%s]",
			step, e.Tick, strings.Join(gotEvents, ","), strings.Join(want.Events, ","))
	}

	// 刷怪队列：长度、顺序、每一项的三个字段都要对上。
	// ⚠️ 顺序比内容更要紧 —— 洗牌错了会同时改顺序与 uid 分配，
	//   而 uid 决定事件流里的 `a` 位。
	if len(e.spawnQueue) != len(want.SpawnQueue) {
		t.Fatalf("第 %d 步刷怪队列长度 = %d，TS 期望 %d",
			step, len(e.spawnQueue), len(want.SpawnQueue))
	}
	for i := range e.spawnQueue {
		g, w := e.spawnQueue[i], want.SpawnQueue[i]
		if g.EnemyID != w.EnemyID || g.AtTick != w.AtTick || g.UID != w.UID {
			t.Fatalf("第 %d 步刷怪队列[%d] = {enemy=%d at=%d uid=%d}，TS 期望 {enemy=%d at=%d uid=%d}",
				step, i, g.EnemyID, g.AtTick, g.UID, w.EnemyID, w.AtTick, w.UID)
		}
	}

	// 场上敌人：逐个逐字段。
	if len(e.Enemies) != len(want.Enemies) {
		t.Fatalf("第 %d 步场上敌人数 = %d，TS 期望 %d（tick=%d）",
			step, len(e.Enemies), len(want.Enemies), e.Tick)
	}
	for i, en := range e.Enemies {
		w := want.Enemies[i]
		if en.UID != w.UID {
			t.Fatalf("第 %d 步敌人[%d] uid = %d，TS 期望 %d", step, i, en.UID, w.UID)
		}
		if en.DefID != w.DefID {
			t.Fatalf("第 %d 步敌人[%d] defId = %d，TS 期望 %d", step, i, en.DefID, w.DefID)
		}
		// 坐标与血量是定点整数；比对**字符串**以免 JSON number 在 2^53 以上失精。
		for _, f := range []struct {
			name     string
			got, exp string
		}{
			{"x", strconv.FormatInt(en.X, 10), w.X},
			{"y", strconv.FormatInt(en.Y, 10), w.Y},
			{"hp", strconv.FormatInt(en.HP, 10), w.HP},
			{"shield", strconv.FormatInt(en.Shield, 10), w.Shield},
			{"armor", strconv.FormatInt(en.ArmorPermille, 10), w.ArmorPermille},
			{"attack_cooldown", strconv.FormatInt(en.AttackCooldown, 10), w.AttackCooldown},
			{"attack_range", strconv.FormatInt(en.AttackRange, 10), w.AttackRange},
		} {
			if f.got != f.exp {
				t.Fatalf("第 %d 步敌人[%d] %s = %s，TS 期望 %s",
					step, i, f.name, f.got, f.exp)
			}
		}
		if en.SpawnProgress != int64(w.SpawnProgress) {
			t.Fatalf("第 %d 步敌人[%d] spawnProgress = %d，TS 期望 %d",
				step, i, en.SpawnProgress, w.SpawnProgress)
		}
		if en.Dead != w.Dead {
			t.Fatalf("第 %d 步敌人[%d] dead = %v，TS 期望 %v", step, i, en.Dead, w.Dead)
		}
		if got := resistString(en); got != w.Resist {
			t.Fatalf("第 %d 步敌人[%d] resist = %s，TS 期望 %s", step, i, got, w.Resist)
		}
	}

	// 重放事件：逐条比对**本步新增**的那些（向量里也是增量）。
	//
	// ⚠️ 用游标而不是全量：全量比对要 O(n²) 的格式化开销，
	//   而长场景有 22000 步 × 数百条事件。
	//   游标顺带把「漏掉了某条事件」变成可见的：
	//   若 Go 少记一条，游标就会落在错的位置，后面每一步都红。
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

// fmtReplay 把一条重放事件格式化成向量的表示法。
func fmtReplay(r ReplayEvent) string {
	return strconv.FormatInt(r.T, 10) + "|" + r.Type + "|" +
		strconv.FormatInt(r.A, 10) + "|" + strconv.FormatInt(r.B, 10) + "|" +
		strconv.FormatInt(r.C, 10)
}

func resistString(en *Enemy) string {
	parts := make([]string, 0, 5)
	for _, el := range AllElements() {
		parts = append(parts, string(el)+":"+strconv.FormatInt(en.Resist[el], 10))
	}
	return strings.Join(parts, "|")
}

func eventTypes(evs []BattleEvent) []string {
	out := make([]string, 0, len(evs))
	for _, ev := range evs {
		out = append(out, ev.Type)
	}
	return out
}
