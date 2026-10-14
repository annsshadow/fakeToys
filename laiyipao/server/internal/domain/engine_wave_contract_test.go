package domain

// Block B（波次与刷怪）的**跨端逐事件对齐**守卫。
//
// # 这是 R1 移植过程中第一次真正的跨端对齐
//
// A 块只能验排序不变量（`BeginWave` 还是空实现，事件流必然不一致）。
// B 块把它换成真实实现之后，「同一关 + 同一种子 + 同一空构筑」下，
// 两端的**每一个 tick** 的 `tick` / `phase` / `刷怪队列` / `每个敌人的全部字段` /
// `全部重放事件` / `全部渲染事件` 都必须逐条相同。
//
// # 为什么逐步比对而不是只比最终状态
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

// loadSmokeFixture 读服务端关卡夹具。
//
// ⚠️ 必须用**同一份**夹具：向量里的 level_waves 是从这里取的，
//
//	Go 侧若用 `GenerateLevel` 重新生成，刷怪组可能不同 ——
//	而 levelgen 是确定性的，两条路本该一致，但那是「巧合」不是「同一份数据」。
//	混用两个来源时，差异会以「刷怪数量不同」的形式出现，
//	而排查方向会指向错误的一边。
func loadSmokeFixture(t *testing.T) struct {
	Levels []struct {
		ID    int `json:"id"`
		Waves []struct {
			Index  int `json:"wave_index"`
			Spawns []struct {
				EnemyID  int   `json:"enemy_id"`
				Count    int   `json:"count"`
				Interval int64 `json:"interval"`
				Delay    int64 `json:"delay"`
			} `json:"spawns"`
		} `json:"waves"`
	} `json:"levels"`
	Enemies []SeedEnemy `json:"enemies"`
	Skills  []SeedSkill `json:"skills"`
} {
	t.Helper()
	path := filepath.Join("..", "..", "testdata", "smoke_levels.json")
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("读夹具失败：%v", err)
	}
	var f struct {
		Levels []struct {
			ID    int `json:"id"`
			Waves []struct {
				Index  int `json:"wave_index"`
				Spawns []struct {
					EnemyID  int   `json:"enemy_id"`
					Count    int   `json:"count"`
					Interval int64 `json:"interval"`
					Delay    int64 `json:"delay"`
				} `json:"spawns"`
			} `json:"waves"`
		} `json:"levels"`
		Enemies []SeedEnemy `json:"enemies"`
		Skills  []SeedSkill `json:"skills"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		t.Fatalf("夹具解析失败：%v", err)
	}
	return f
}

// blockBDivergences 列出 B 块与 TS 的**已知、已记录、未比对**的差异。
//
// # 为什么要有这张表，而不是「跳过就完事」
//
// 静默跳过的字段会变成**没人记得的债**：C 落地时若忘了接，
// 本文件会继续绿（因为它从来没比过那个值），而那正是 C 该抓的 bug。
//
// 把差异写下来并**指定消除方**，才有人会去消除它。
// `TestBlockBDivergencesAreStillReal` 反向盯着这张表：
// 一旦某条差异不再成立（比如 C 已经接上了），那条测试会红，
// 提醒「该把本文件改成真正比对了」。
var blockBDivergences = map[string]string{
	"spawn_progress": "C 块：updateEnemies 逐敌调用 StepEnemyMotion，" +
		"TS 会推进出场进度而 Go 侧此刻恒为 0",
	"x": "C 块：同上 —— 位移由 StepEnemyMotion 驱动，" +
		"TS 的 x 每 tick 递减而 Go 侧停在出生点 960000",
}

// isKnownBlockBDivergence 报告某字段是否在「已知差异」表里。
func isKnownBlockBDivergence(field string) bool {
	_, ok := blockBDivergences[field]
	return ok
}

// TestBlockBDivergencesAreStillReal 反向盯着上面那张表。
//
// 判据：每个「已知差异」都必须**仍然是差异**。
// 若某条不再成立（字段已经一致了），说明对应的块已落地而本文件没跟上 ——
// 那时这条会红，并指出该把比对改成真的。
func TestBlockBDivergencesAreStillReal(t *testing.T) {
	v := loadPureVectors(t)
	if len(blockBDivergences) == 0 {
		t.Fatal("blockBDivergences 为空 —— " +
			"C 块落地后请把本文件的 spawn_progress 改成真正比对，并清空这张表")
	}

	// spawn_progress 与 x：TS 应当仍在推进，而 Go 侧应当仍静止。
	tsMoves := false
	goMoves := false
	for _, row := range v.Wave.Rows {
		e := buildWaveEngine(t, struct {
			LevelID    int
			Seed       string
			LevelWaves []struct {
				Index  int `json:"wave_index"`
				Spawns []struct {
					EnemyID  int   `json:"enemy_id"`
					Count    int   `json:"count"`
					Interval int64 `json:"interval"`
					Delay    int64 `json:"delay"`
				} `json:"spawns"`
			}
		}{row.LevelID, row.Seed, row.LevelWaves})
		// 出生点 x：StepEnemyMotion 之前全体相同，用它判断「有没有动过」
		const birthX = "960000"
		e.Start()
		for i := 0; i < row.Ticks; i++ {
			if e.Phase == PhaseWon || e.Phase == PhaseLost {
				break
			}
			e.Step()
			if i >= len(row.Trace)-1 {
				continue
			}
			for j, en := range e.Enemies {
				if en.SpawnProgress != 0 || strconv.FormatInt(en.X, 10) != birthX {
					goMoves = true
				}
				if j < len(row.Trace[i+1].Enemies) {
					w := row.Trace[i+1].Enemies[j]
					if w.SpawnProgress != 0 || w.X != birthX {
						tsMoves = true
					}
				}
			}
		}
	}
	if !tsMoves {
		t.Error("TS 侧的 x / spawn_progress 全程未变 —— " +
			"若 TS 已不推进，说明本差异的前提变了，请重新评估这张表")
	}
	if goMoves {
		t.Fatal("Go 侧 x / spawn_progress 开始变化 —— " +
			"C 块已落地！请把 checkWaveTick 里的这两个字段改成与 TS 比对，" +
			"并从 blockBDivergences 里删掉这两条")
	}
}

func buildWaveEngine(t *testing.T, row struct {
	LevelID    int
	Seed       string
	LevelWaves []struct {
		Index  int `json:"wave_index"`
		Spawns []struct {
			EnemyID  int   `json:"enemy_id"`
			Count    int   `json:"count"`
			Interval int64 `json:"interval"`
			Delay    int64 `json:"delay"`
		} `json:"spawns"`
	}
}) *BattleEngine {
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
	for _, e := range fx.Enemies {
		enemyDefs[e.ID] = e
	}
	skillDefs := map[int]SeedSkill{}
	for _, s := range fx.Skills {
		skillDefs[s.ID] = s
	}

	seed, err := strconv.ParseInt(row.Seed, 10, 64)
	if err != nil {
		t.Fatalf("种子 %q 解析失败：%v", row.Seed, err)
	}

	// ⚠️ level 只带 Waves 与 ID：Block B 只碰波次与刷怪。
	//   刻意**不**用 GenerateLevel —— 那会把「关卡生成是否与夹具一致」
	//   混进本条守卫，而那是另一个问题（由 levelgen 自己的契约负责）。
	level := GeneratedLevel{ID: row.LevelID, Waves: waves}

	e := NewBattleEngine(BattleConfig{
		Level:       level,
		Enemies:     enemyDefs,
		Skills:      skillDefs,
		Equipped:    nil,
		Attacker:    Attacker{},
		Seed:        seed,
		ScoreRules:  DefaultScoreRules(),
		ActiveSlots: 4,
	})
	// TotalEnemies 是通关判定的分母，B 块要读它。
	total := 0
	for _, w := range waves {
		for _, s := range w.Spawns {
			total += s.Count
		}
	}
	e.TotalEnemies = total
	return e
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

	for _, row := range v.Wave.Rows {
		t.Run(row.Label, func(t *testing.T) {
			e := buildWaveEngine(t, struct {
				LevelID    int
				Seed       string
				LevelWaves []struct {
					Index  int `json:"wave_index"`
					Spawns []struct {
						EnemyID  int   `json:"enemy_id"`
						Count    int   `json:"count"`
						Interval int64 `json:"interval"`
						Delay    int64 `json:"delay"`
					} `json:"spawns"`
				}
			}{row.LevelID, row.Seed, row.LevelWaves})

			e.Start()
			checkWaveTick(t, e, row.Trace[0], 0)

			for i := 1; i < len(row.Trace) && i <= row.Ticks; i++ {
				if e.Phase == PhaseWon || e.Phase == PhaseLost {
					t.Fatalf("第 %d 步前已终局（phase=%q），但向量里还有 %d 步",
						i, e.Phase, len(row.Trace)-i)
				}
				e.Step()
				checkWaveTick(t, e, row.Trace[i], i)
			}
			if len(row.Trace) > row.Ticks+1 {
				t.Fatalf("向量有 %d 步，但只允许 %d 步 —— "+
					"场景定义与 trace 长度不一致", len(row.Trace), row.Ticks+1)
			}
		})
	}
}

func checkWaveTick(t *testing.T, e *BattleEngine, want struct {
	Tick       int64  `json:"tick"`
	Phase      string `json:"phase"`
	ElapsedMs  int64  `json:"elapsed_ms"`
	SpawnQueue []struct {
		EnemyID int   `json:"enemy_id"`
		AtTick  int64 `json:"at_tick"`
		UID     int64 `json:"uid"`
	} `json:"spawn_queue"`
	Enemies []struct {
		UID           int64  `json:"uid"`
		DefID         int    `json:"def_id"`
		X             string `json:"x"`
		Y             string `json:"y"`
		HP            string `json:"hp"`
		Shield        string `json:"shield"`
		ArmorPermille string `json:"armor_permille"`
		SpawnProgress int    `json:"spawn_progress"`
		Dead          bool   `json:"dead"`
		Resist        string `json:"resist"`
	} `json:"enemies"`
	Replay []string `json:"replay"`
	Events []string `json:"events"`
}, step int) {
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
		t.Fatalf("第 %d 步场上敌人数 = %d，TS 期望 %d", step, len(e.Enemies), len(want.Enemies))
	}
	for i, en := range e.Enemies {
		w := want.Enemies[i]
		if en.UID != w.UID {
			t.Fatalf("第 %d 步敌人[%d] uid = %d，TS 期望 %d", step, i, en.UID, w.UID)
		}
		if en.DefID != w.DefID {
			t.Fatalf("第 %d 步敌人[%d] defId = %d，TS 期望 %d", step, i, en.DefID, w.DefID)
		}
		// ⚠️ `x` 与 `spawn_progress` 是 B 块与 TS 的**已知差异**，不在本文件比对。
		//
		//   原因：向量由**完整**的 TS 引擎产出，而 TS 的 `updateEnemies`
		//   是真实实现 —— 它逐敌调用 `stepEnemyMotion`，于是坐标与出场进度
		//   都会推进。Go 侧 `updateEnemies` 是 C 块的空实现，两者都不动。
		//
		//   （我一度以为「TS 在这一段也不该动」，实测才发现 TS 会动 ——
		//    想当然地断言一个「两端应该一致」的字段是最坏的一类错误：
		//    它把「我还没做」写成了「规则如此」。)
		//
		//   本文件比对的正是 B 自己的产出：刷怪队列、uid 分配、出生坐标 y、
		//   事件流、phase。消除方见 blockBDivergences。
		if got := strconv.FormatInt(en.X, 10); got != w.X && !isKnownBlockBDivergence("x") {
			t.Fatalf("第 %d 步敌人[%d] x = %s，TS 期望 %s —— "+
				"该字段不在 blockBDivergences 里，说明它本该被比对却漏了",
				step, i, got, w.X)
		}
		if en.SpawnProgress != int64(w.SpawnProgress) && !isKnownBlockBDivergence("spawn_progress") {
			t.Fatalf("第 %d 步敌人[%d] spawn_progress = %d，TS 期望 %d —— "+
				"该字段不在 blockBDivergences 里，说明它本该被比对却漏了",
				step, i, en.SpawnProgress, w.SpawnProgress)
		}
		if got := strconv.FormatInt(en.Y, 10); got != w.Y {
			t.Fatalf("第 %d 步敌人[%d] y = %s，TS 期望 %s", step, i, got, w.Y)
		}
		if got := strconv.FormatInt(en.HP, 10); got != w.HP {
			t.Fatalf("第 %d 步敌人[%d] hp = %s，TS 期望 %s", step, i, got, w.HP)
		}
		if got := strconv.FormatInt(en.Shield, 10); got != w.Shield {
			t.Fatalf("第 %d 步敌人[%d] shield = %s，TS 期望 %s", step, i, got, w.Shield)
		}
		if got := strconv.FormatInt(en.ArmorPermille, 10); got != w.ArmorPermille {
			t.Fatalf("第 %d 步敌人[%d] armor = %s，TS 期望 %s", step, i, got, w.ArmorPermille)
		}

		if en.Dead != w.Dead {
			t.Fatalf("第 %d 步敌人[%d] dead = %v，TS 期望 %v", step, i, en.Dead, w.Dead)
		}
		if got := resistString(en); got != w.Resist {
			t.Fatalf("第 %d 步敌人[%d] resist = %s，TS 期望 %s", step, i, got, w.Resist)
		}
	}

	// 重放事件流：逐条。全量比对而不是只比条数 ——
	// 条数对但内容错的情形（类型写错、参数顺序颠倒）很常见。
	gotReplay := make([]string, 0, len(e.replay))
	for _, r := range e.replay {
		gotReplay = append(gotReplay, fmtReplay(r))
	}
	if strings.Join(gotReplay, ";") != strings.Join(want.Replay, ";") {
		t.Fatalf("第 %d 步重放事件流不一致：\n  Go = [%s]\n  TS = [%s]",
			step, strings.Join(gotReplay, ";"), strings.Join(want.Replay, ";"))
	}

	// 渲染事件：本步 drain 出来的类型序列。
	gotEvents := eventTypes(e.DrainEvents())
	if strings.Join(gotEvents, ",") != strings.Join(want.Events, ",") {
		t.Fatalf("第 %d 步渲染事件不一致：\n  Go = [%s]\n  TS = [%s]",
			step, strings.Join(gotEvents, ","), strings.Join(want.Events, ","))
	}
}

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

// --- stepEnemyMotion 的纯逻辑 ---
//
// ⚠️ 它的引擎级验收在 C 块（由 `updateEnemies` 驱动），
//   但**逻辑本身**是纯函数，现在就可以直接覆盖 ——
//   「位移写在哪」这个历史 bug 就是因为无法单独验证才发生的。

func newMotionEnemy() *Enemy {
	return &Enemy{
		X:             ToFixed(500),
		Y:             ToFixed(500),
		Speed:         1000,
		SpawnProgress: SpawnProgressFull,
	}
}

// TestStepEnemyMotionSpawnProgressGate 守住「出场动画期间不移动」。
//
// ⚠️ 这是最容易写漏的一处：漏掉 return 会让「正在出场」的怪
//
//	也参与冻结判定与位移，而出场进度随 tick 变化
//	→ 每只怪的冻结起始时刻都不同 → 整局错位。
//
// 判据用「进度未满 → 位置不变」这个**关系**，而不是逐个数值。
func TestStepEnemyMotionSpawnProgressGate(t *testing.T) {
	e := newProbeEngine()
	en := newMotionEnemy()
	en.SpawnProgress = 0
	x0 := en.X

	// 跑满 13 次：1000/80 = 12.5，第 13 次才到满
	for i := 0; i < 12; i++ {
		e.StepEnemyMotion(en, false)
		if en.X != x0 {
			t.Fatalf("第 %d 次：出场动画期间不应移动，x %d → %d", i+1, x0, en.X)
		}
	}
	if en.SpawnProgress != 12*SpawnProgressStep {
		t.Errorf("进度 = %d，期望 %d", en.SpawnProgress, 12*SpawnProgressStep)
	}
	// 第 13 次：进度到满，但仍**不**移动（return 发生在移动之前）
	e.StepEnemyMotion(en, false)
	if en.SpawnProgress != SpawnProgressFull {
		t.Errorf("第 13 次进度 = %d，期望满值 %d", en.SpawnProgress, SpawnProgressFull)
	}
	if en.X != x0 {
		t.Errorf("进度刚满的那一次仍不应移动，x %d → %d", x0, en.X)
	}
	// 第 14 次：开始移动
	e.StepEnemyMotion(en, false)
	if en.X == x0 {
		t.Error("进度满后应开始移动")
	}
}

// TestStepEnemyMotionSpeedMath 守住位移公式。
//
// 公式：`step = mulDiv(speed, slowFactor, 1000) * TICK_BIG / 1000`
//
// ⚠️ 判据给出**具体数值**而不只是「动了」：
//
//	speed=1000、slowFactor=1000、TICK_BIG=50 → step = 1000*50/1000 = 50。
//	「动了」会被「动了但动了错的量」蒙混过去。
//
// TestStepEnemyMotionSpeedMath 守住位移公式。
//
// 公式：`step = mulDiv(speed, slowFactor, 1000) * TICK_BIG / 1000`
//
// ⚠️ 判据给出**具体数值**而不只是「动了」：
//
//	speed=1000、slowFactor=1000、TICK_BIG=50 → step = 1000*50/1000 = 50。
//	「动了」会被「动了但动了错的量」蒙混过去。
//
// ⚠️⚠️ 所有状态计时都是**先递减再判定**，所以 `X = TickMs` 意味着
//
//	「本 tick 正好耗尽」→ 判定为不生效。
//
//	我第一版用 `SlowedMs = TickMs` 期望拿到减速位移 25，实际拿到全速 50
//	—— **是我的期望错了，实现与 TS 一致**。
//	要用到减速必须给 `2×TickMs`，让它熬过本 tick 的递减。
//	这条边界单独列在下一条测试里。
func TestStepEnemyMotionSpeedMath(t *testing.T) {
	e := newProbeEngine()

	// 全速
	en := newMotionEnemy()
	en.Speed = 1000
	x0 := en.X
	e.StepEnemyMotion(en, false)
	if got, want := x0-en.X, int64(50); got != want {
		t.Errorf("全速位移 = %d，期望 %d（1000×50/1000）", got, want)
	}

	// 减速 50%
	en2 := newMotionEnemy()
	en2.Speed = 1000
	en2.SlowedMs = 2 * TickMs // 熬过本 tick 的递减后仍 > 0
	x2 := en2.X
	e.StepEnemyMotion(en2, false)
	if got, want := x2-en2.X, int64(25); got != want {
		t.Errorf("减速位移 = %d，期望 %d（1000×500/1000×50/1000=25）", got, want)
	}
}

// TestStepEnemyMotionStatusExpiresOnItsOwnTick 钉住「先递减后判定」这条语义。
//
// ⚠️ 这是最容易在移植时改错的一处：把判定移到递减之前，
//
//	症状是「所有状态效果都晚一个 tick 结束」——
//	而「晚一个 tick」在整局里表现为敌人推进时机偏移，
//	最终变成哈希失配，且看不出是哪一步。
//
// 判据用「恰好等于 TickMs」与「TickMs+1」两侧。
func TestStepEnemyMotionStatusExpiresOnItsOwnTick(t *testing.T) {
	e := newProbeEngine()
	// ⚠️ 只列 frozen / stunned —— 它们**阻断**移动。
	//
	//   `slowed` 不阻断，只是把速度减半（两种情况下都在动），
	//   所以它区分不出「本 tick 生效还是耗尽」。
	//   它的边界由 `TestStepEnemyMotionSpeedMath` 的**距离**断言覆盖 ——
	//   把一个只比较「动不动」的用例塞进来，只会得到一个恒真的断言。
	cases := []struct {
		name     string
		apply    func(*Enemy, int64)
		wantMove bool
	}{
		{"frozen", func(en *Enemy, v int64) { en.FrozenMs = v }, true},
		{"stunned", func(en *Enemy, v int64) { en.StunnedMs = v }, true},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			// 恰好耗尽 → 递减到 0 → 判定为不生效 → 应移动
			en := newMotionEnemy()
			en.Speed = 1000
			c.apply(en, TickMs)
			x0 := en.X
			e.StepEnemyMotion(en, false)
			if (en.X != x0) != c.wantMove {
				t.Errorf("%s 恰好耗尽的 tick：应移动=%v，实际移动=%v",
					c.name, c.wantMove, en.X != x0)
			}

			// 熬过本 tick → 仍生效 → 不应移动
			en2 := newMotionEnemy()
			en2.Speed = 1000
			c.apply(en2, TickMs+1)
			x2 := en2.X
			e.StepEnemyMotion(en2, false)
			if en2.X != x2 {
				t.Errorf("%s 熬过本 tick 时不应移动，x %d → %d", c.name, x2, en2.X)
			}
		})
	}
}

// TestStepEnemyMotionFrozenBlocksMovement 守住冻结判定。
//
// ⚠️ 用 `2*TickMs` 而不是 `TickMs`：`TickMs` 恰好在本 tick 耗尽，
//
//	按「先递减后判定」的语义不生效 —— 那是上面那条测试的职责。
//	这里只守「仍生效时不移动」。
func TestStepEnemyMotionFrozenBlocksMovement(t *testing.T) {
	e := newProbeEngine()
	cases := []struct {
		name string
		mut  func(*Enemy)
	}{
		{"frozen", func(en *Enemy) { en.FrozenMs = 2 * TickMs }},
		{"stunned", func(en *Enemy) { en.StunnedMs = 2 * TickMs }},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			en := newMotionEnemy()
			c.mut(en)
			x0 := en.X
			e.StepEnemyMotion(en, false)
			if en.X != x0 {
				t.Errorf("%s 状态下不应移动，x %d → %d", c.name, x0, en.X)
			}
		})
	}
	// 全场冻结（过热）
	en := newMotionEnemy()
	x0 := en.X
	e.StepEnemyMotion(en, true)
	if en.X != x0 {
		t.Errorf("全场冻结时不应移动，x %d → %d", x0, en.X)
	}
}
func TestStepEnemyMotionKnockbackOverridesSpeed(t *testing.T) {
	e := newProbeEngine()
	en := newMotionEnemy()
	en.Knockback = -30 // 正值是往右（远离防线），这里用负值看加法
	x0 := en.X

	e.StepEnemyMotion(en, false)
	if got, want := en.X, x0-30; got != want {
		t.Errorf("击退位移 = %d，期望 %d", got, want)
	}
	if en.Knockback != 0 {
		t.Errorf("击退应被清零，实际 %d", en.Knockback)
	}
	// 下一 tick 回到按速度移动
	before := en.X
	e.StepEnemyMotion(en, false)
	if got, want := before-en.X, int64(50); got != want {
		t.Errorf("击退之后应回到速度位移，实际 %d，期望 %d", got, want)
	}
}

// TestStepEnemyMotionStatusTimersDecay 守住状态计时递减。
func TestStepEnemyMotionStatusTimersDecay(t *testing.T) {
	e := newProbeEngine()
	en := newMotionEnemy()
	en.HitFlashMs = TickMs
	en.FrozenMs = TickMs
	en.StunnedMs = TickMs
	en.SlowedMs = TickMs
	en.ArmorShredMs = TickMs

	e.StepEnemyMotion(en, false)
	for name, got := range map[string]int64{
		"hitFlash": en.HitFlashMs, "frozen": en.FrozenMs, "stunned": en.StunnedMs,
		"slowed": en.SlowedMs, "armorShred": en.ArmorShredMs,
	} {
		if got != 0 {
			t.Errorf("%s 应递减到 0，实际 %d", name, got)
		}
	}
	// 已为 0 的不应变负 —— 负值会让「>0」判定在下一 tick 行为不同
	en2 := newMotionEnemy()
	e.StepEnemyMotion(en2, false)
	if en2.HitFlashMs < 0 || en2.FrozenMs < 0 || en2.SlowedMs < 0 || en2.ArmorShredMs < 0 {
		t.Errorf("状态计时不应变负：%+v", en2)
	}
}

// TestCeilDivMsToTicks 守住「毫秒→tick 向上取整」。//
// ⚠️ 向上取整保证「间隔 N 毫秒」不会因为截断变成「间隔 0 毫秒」
//
//	而让同组怪同时出现。反过来（向下取整）会让同组怪**同 tick 全部刷出**，
//	而那种画面差异极难从数据里看出来。
//
// 判据覆盖边界两侧与不能整除的情形。
// TestUpdateSpawnsHandlesMultipleDueInOneTick 守住「倒序遍历」。
//
// # 为什么跨端对齐**抓不到**这个 bug
//
// 实测：把 `updateSpawns` 的倒序改成正序，五个真实场景全部照样绿。
//
// 原因是真实关卡的刷怪组把 `atTick` 拉开了，**同一 tick 到期的通常只有一只**，
// 而正序遍历的「删元素后跳过紧邻项」只在「一次要删多项」时才发生。
//
// 这正是 README 里「只写不读」那条的另一种形态：
// 跨端对齐证明了「在真实数据上等价」，却没覆盖「代码想处理的形状」。
// 所以这里用**构造出来的队列**直接打那个形状。
//
// 构造依据合法：`interval = 0` 时 `ceilDivMsToTicks(0) = 0`，
// 于是同组全部落在同一 tick —— 这正是「同组怪同时出现」的极限形态，
// 而向上取整的设计意图恰恰是**避免**它（见 ceilDivMsToTicks 的注释）。
// 所以这是「配置的边界」，不是「配置的正常用法」。
func TestUpdateSpawnsHandlesMultipleDueInOneTick(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Enemies = map[int]SeedEnemy{
		1: {ID: 1, Name: "a", HP: 100},
		2: {ID: 2, Name: "b", HP: 100},
		3: {ID: 3, Name: "c", HP: 100},
	}
	e.Phase = PhaseWave
	e.Tick = 10
	e.spawnQueue = []SpawnItem{
		{EnemyID: 1, AtTick: 10, UID: 1},
		{EnemyID: 2, AtTick: 10, UID: 2},
		{EnemyID: 3, AtTick: 10, UID: 3},
	}

	e.updateSpawns()

	if len(e.Enemies) != 3 {
		t.Fatalf("同 tick 到期的 3 只应全部刷出，实际 %d 只 —— "+
			"倒序遍历被改成正序会在删除元素后跳过紧邻项", len(e.Enemies))
	}
	if len(e.spawnQueue) != 0 {
		t.Errorf("刷完后队列应为空，实际剩 %d 项", len(e.spawnQueue))
	}
	// ⚠️ 出现次序是**倒序**（uid 3、2、1）。
	//
	//   因为遍历是 `i` 从尾到头，且 push 到 `enemies` 的顺序就是遍历顺序。
	//   TS 用的是同一个 `for (let i = len-1; i >= 0; i--)`，所以两端一致。
	//   我第一版期望正序，错了 —— 而这条恰恰是「倒序遍历」最容易写坏的地方：
	//   改成正序后次序会变，但它仍然是「刷出 3 只」，只查数量抓不到。
	for i, want := range []int64{3, 2, 1} {
		if e.Enemies[i].UID != want {
			t.Errorf("场上第 %d 只的 uid = %d，期望 %d（倒序遍历 ⇒ 倒序出现，与 TS 同）",
				i, e.Enemies[i].UID, want)
		}
	}
}

// TestUpdateSpawnsIgnoresFutureItems 守住「未到期的不刷」。
func TestUpdateSpawnsIgnoresFutureItems(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100}}
	e.Phase = PhaseWave
	e.Tick = 10
	e.spawnQueue = []SpawnItem{
		{EnemyID: 1, AtTick: 11, UID: 1},
		{EnemyID: 1, AtTick: 12, UID: 2},
	}
	e.updateSpawns()
	if len(e.Enemies) != 0 {
		t.Errorf("未到期的项不应刷出，实际 %d 只", len(e.Enemies))
	}
	if len(e.spawnQueue) != 2 {
		t.Errorf("未到期的项应留在队列，实际剩 %d 项", len(e.spawnQueue))
	}
}

// TestEmptyLevelIsNotAutoWin 守住「空关卡判负而不是判胜」。
//
// # 为什么这条重要
//
// 曾经的代码是 `finish(true)`：`waves=[]` 的关卡 `start()` 立刻判胜，
// 而服务端 `MaxKillsFor(gl)=0` 让 `kills(0) >= 0` 成立，**照样发掉落**。
// 畸形关卡因此变成「秒胜 + 发钱」。
//
// 当前 levelgen 必填 >=5 波，所以正常流程不可达；但 **level 是网络数据**，
// 缺失字段时这个分支正是入口 —— 所以必须守住。
//
// 五个真实场景都抓不到它：它们的 waves 都非空。
func TestEmptyLevelIsNotAutoWin(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Level = GeneratedLevel{ID: 999, Waves: nil}
	e.TotalEnemies = 0
	e.Score = 4242

	e.Start() // → BeginWave(0) → waves[0] 不存在 → 应判负

	if e.Phase != PhaseLost {
		t.Fatalf("空关卡应判负，实际 phase=%q —— "+
			"判胜会让服务端 MaxKillsFor(gl)=0 的 `kills(0) >= 0` 成立，"+
			"畸形关卡变成「秒胜 + 发钱」", e.Phase)
	}
	evs := e.DrainEvents()
	if len(evs) != 1 || evs[0].Type != EventLost {
		t.Fatalf("应恰好发一条 lost 事件，实际 %+v", evs)
	}
	// 且不得记 wave 重放事件（那是「开了一波」的事实）
	for _, r := range e.replay {
		if r.Type == ReplayWave {
			t.Errorf("空关卡不得记 wave 重放事件，实际 %+v", e.replay)
		}
	}
}

// TestUnknownEnemyIDIsSkipped 守住「刷怪表里没有的 id 直接跳过」。
//
// ⚠️ 不返回错误：敌人 id 来自关卡数据，而关卡是**网络数据**。
//
//	缺失一个 id 就让整场战斗崩掉，比少刷一只怪糟得多。
//	症状若是「崩」，会表现为「玩家在某关完全打不了」，且看不出是数据问题。
func TestUnknownEnemyIDIsSkipped(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100}}
	e.Phase = PhaseWave
	e.Tick = 1 // 必须让两项都到期，否则什么都测不到
	e.spawnQueue = []SpawnItem{
		{EnemyID: 999, AtTick: 1, UID: 1},
		{EnemyID: 1, AtTick: 1, UID: 2},
	}

	e.updateSpawns()
	// 缺失的那个被跳过，存在的那个刷出；队列照样被清空
	if len(e.Enemies) != 1 || e.Enemies[0].UID != 2 {
		t.Errorf("未知 id 应被跳过且不阻断其余刷怪，实际场上 %d 只", len(e.Enemies))
	}
	if len(e.spawnQueue) != 0 {
		t.Errorf("队列应被清空（含被跳过的未知 id），实际剩 %d 项", len(e.spawnQueue))
	}
}

func TestCeilDivMsToTicks(t *testing.T) {
	cases := []struct {
		ms   int64
		want int64
	}{
		{0, 0}, {-1, 0}, {1, 1}, {49, 1}, {50, 1}, {51, 2},
		{1000, 20}, {1012, 21}, // 第 1 关第一波的真实 delay
		{99, 2}, {100, 2},
	}
	for _, c := range cases {
		if got := ceilDivMsToTicks(c.ms); got != c.want {
			t.Errorf("ceilDivMsToTicks(%d) = %d，期望 %d", c.ms, got, c.want)
		}
	}
}
