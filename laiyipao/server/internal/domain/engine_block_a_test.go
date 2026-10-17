package domain

// 引擎骨架（R1 Block A）的守卫。
//
// # 为什么 A 阶段**不**比对 TS 事件流
//
// `Start()` 会调 `BeginWave(0)`，而 `BeginWave` 属于 B 块、A 阶段是空实现。
// 所以 A 阶段的事件流**必然**与 TS 不同 —— 那是「尚未搬完」的如实反映，
// 拿它去比对只会得到一个恒红的守卫，而恒红守卫的唯一效果是被忽略。
//
// A 阶段能验的是**排序不变量**：它们不依赖 B~F，且错了会立刻出事。
// 第一次真正的跨端逐事件对齐在 B。
//
// 详见 `docs/audits/r1-engine-port-plan.md`。

// # 排序约束 (1) 与 (2) 为什么**不在本文件**验
//
// 它们分别是「`cardSelectDone` 收尾在 `applyReplayDecision` 之前」
// 与「`applyReplayDecision` 在 `tick++` 之前」，而 `applyReplayDecision`
// 是 **F 块**（卡牌）的方法，A 阶段是空实现。
//
// 空实现下这两种顺序**行为完全相同**，所以此刻没有任何可观测的差异，
// 写不出能失败的守卫 —— 硬写一条只能靠代码结构断言，那是装饰品。
//
// 它们的验证属于 **F 块**：那时 `applyReplayDecision` 有真实行为，
// 顺序错了会表现为「选牌索引序列错乱」（TS 注释记的实测是
// `picks` 记成 `[-3,-3,-1,-1]`），那是可观测、可断言的。
//
// 本文件只验约束 (3)（停滞位置），因为它现在就有行为。
//
// 详见 `docs/audits/r1-engine-port-plan.md`。

import (
	"path/filepath"
	"testing"
)

// newProbeEngine 造一个**停留在 wave 阶段**的引擎。
//
// # 为什么必须让 `spawnQueue` 非空
//
// `Step` 末尾会调 `checkWaveEnd`，而它的第一条判定是
// 「刷怪队列为空 → 本波结束」。空队列 + 空场上 ⇒ **第一个 `Step`
// 就把战斗判结束**，于是「tick 怎么长」这类断言在第 1 步就失去意义
// —— 我第一次跑就撞上了这个：
//
//	TestStepAdvancesTickAndElapsed: 第 1 次 Step 后 tick = 1
//
// 原因不是断言写错了，而是 B 块落地后 `checkWaveEnd` 成了真实实现，
// 而夹具仍按「A 阶段它什么也不做」的假设搭的。
//
// 所以这里放一个**很久之后**才到期的刷怪项：
// 它在 2^40 tick 之前都不会兑现，因此对被测的时间线无影响，
// 却足以让引擎停在 wave 阶段。
//
// # 为什么不调 Start()
//
// Start 会走 `BeginWave(0)`，那会把刷怪队列**换成关卡真实的数据**，
// 于是这个夹具不再是「受控的时间线」，而是被关卡内容决定。
// 本文件验的是 tick 时序，不该被关卡内容影响。
func newProbeEngine() *BattleEngine {
	e := NewBattleEngine(BattleConfig{
		Level:       GenerateLevel(1),
		ActiveSlots: 4,
		ScoreRules:  DefaultScoreRules(),
	})
	e.spawnQueue = []SpawnItem{{EnemyID: -1, AtTick: 1 << 40, UID: 1}}
	return e
}

// TestStepAdvancesTickAndElapsed 守住 tick 节奏。
//
// 判据用 `elapsed == tick*TickMs` 这个**关系**而不是逐个数值：
// 前者说明两者的换算关系永远成立，后者只说明第 3 步碰巧对了。
func TestStepAdvancesTickAndElapsed(t *testing.T) {
	e := newProbeEngine()
	e.Phase = PhaseWave
	for i := int64(1); i <= 50; i++ {
		e.Step()
		if e.Tick != i {
			t.Fatalf("第 %d 次 Step 后 tick = %d", i, e.Tick)
		}
		if want := i * TickMs; e.ElapsedMs != want {
			t.Fatalf("第 %d 次 Step 后 elapsedMs = %d，期望 tick×%d = %d",
				i, e.ElapsedMs, TickMs, want)
		}
	}
}

// TestTickConstantsMatchClient 守住 tick 相关常量与 miniapp 一致。
//
// # 背景：battle.go 的注释声称 `TestTickMsMatchesClient` 锁住了 TickMs，
//
// 而**那条测试并不存在** —— 我是照着注释去找才发现的。
// 注释里的假声明比没有注释更糟：它让人以为已经有人管过了。
// 所以本条把它补上。
//
// ⚠️ 判据必须读 **TS 源码的字面量**，不能拿 Go 的常量互相比，
//
// 也不能让「起点」与「判据」同源。
//
// 我第一版写的是「以 `MaxBattleTicks - 1` 为起点，跑一步看是否停滞」——
// 变异把 30000 改成 30001 后**测试没红**，因为起点跟着常量一起挪了。
// 起点与判据同源时，测试只能证明「两端自洽」，证明不了「与 TS 一致」。
func TestTickConstantsMatchClient(t *testing.T) {
	src := readFileFrom(t, "引擎源码 miniapp/src/game/engine.ts",
		filepath.Join("miniapp", "src", "game", "engine.ts"))

	if TickHz != tsConst(t, src, "TICK_HZ") {
		t.Errorf("TickHz = %d，engine.ts 的 TICK_HZ = %d", TickHz, tsConst(t, src, "TICK_HZ"))
	}
	if TickMs != tsConst(t, src, "TICK_MS") {
		t.Errorf("TickMs = %d，engine.ts 的 TICK_MS = %d", TickMs, tsConst(t, src, "TICK_MS"))
	}
	if MaxBattleTicks != tsConst(t, src, "MAX_BATTLE_TICKS") {
		t.Errorf("MaxBattleTicks = %d，engine.ts 的 MAX_BATTLE_TICKS = %d",
			MaxBattleTicks, tsConst(t, src, "MAX_BATTLE_TICKS"))
	}

	// 跨端**关系**不变量。三者都要满足，否则其中一句说明在撒谎：
	//   「每秒 20 tick」与「每 tick 50ms」必须自洽；
	//   停滞上限换算成秒之后不该超过 duration_ms 的 1800s 上限；
	//   相对实测最长战斗要有 2 倍以上余量。
	if 1000/TickHz != TickMs {
		t.Errorf("TickHz=%d 与 TickMs=%d 不自洽（1000/TickHz=%d）", TickHz, TickMs, 1000/TickHz)
	}
	if secs := MaxBattleTicks * TickMs / 1000; secs > 1800 {
		t.Errorf("停滞上限 %d tick = %ds，超过 duration_ms 的 1800s 上限 —— "+
			"一个合法战斗不该被停滞兜底截断", MaxBattleTicks, secs)
	}
	if MaxBattleTicks < 2*11880 {
		t.Errorf("停滞上限 %d 相对实测最长战斗（11880 tick）余量不足 2 倍", MaxBattleTicks)
	}
}

// TestStalemateFiresExactlyAtMaxTicks 守住停滞触发的**精确时刻**。
//
// ⚠️ 为什么必须是「第 30000 tick 之前不触发，第 30000 tick 触发」：
//
//	差一个 tick 就是差 50ms，而停滞事件本身进 replay 哈希。
//	「差 50ms」在日志里看不出来，只表现为「这一局的哈希对不上」。
//
// 判据用边界两侧（29999 / 30000），不是「跑 30000 次看看结果」。
func TestStalemateFiresExactlyAtMaxTicks(t *testing.T) {
	e := newProbeEngine()
	e.Phase = PhaseWave
	e.Tick = MaxBattleTicks - 1
	e.ElapsedMs = e.Tick * TickMs

	evs := e.Step() // 这一步把 tick 推到 MaxBattleTicks
	if e.Phase != PhaseLost {
		t.Fatalf("tick 达到上限时 phase 应为 lost，实际 %q", e.Phase)
	}
	var sawStalemate bool
	for _, ev := range evs {
		if ev.Type == EventStalemate {
			sawStalemate = true
		}
	}
	if !sawStalemate {
		t.Error("tick 达到上限时必须发出 stalemate 渲染事件")
	}
	if len(e.replay) != 1 || e.replay[0].Type != ReplayStalemate {
		t.Errorf("停滞必须记一条重放事件，实际 %+v", e.replay)
	} else if e.replay[0].T != MaxBattleTicks || e.replay[0].A != MaxBattleTicks {
		t.Errorf("停滞事件的 tick/a 应都是 %d，实际 t=%d a=%d",
			MaxBattleTicks, e.replay[0].T, e.replay[0].A)
	}

	// 差一个 tick 不该触发 —— 用另一个引擎验边界内侧
	e2 := newProbeEngine()
	e2.Phase = PhaseWave
	e2.Tick = MaxBattleTicks - 2
	e2.ElapsedMs = e2.Tick * TickMs
	e2.Step()
	if e2.Phase == PhaseLost {
		t.Errorf("tick = %d 时不应停滞（边界应在 %d）", MaxBattleTicks-1, MaxBattleTicks)
	}
}

// TestStalemateRunsBeforeCombatLogic 守住排序约束 (3)。
//
// 「到上限」这件事必须是这次 tick 的**第一个事实** ——
// 若战斗逻辑先跑，停滞会在敌人又动了一格之后才触发，
// 而那一格已经进了事件流，偏移量取决于战斗逻辑的长度，
// 也就是「每关不一样」。那种偏差几乎不可能被定位。
func TestStalemateRunsBeforeCombatLogic(t *testing.T) {
	e := newProbeEngine()
	e.Phase = PhaseWave
	e.Tick = MaxBattleTicks - 1
	e.ElapsedMs = e.Tick * TickMs
	e.Step()

	if e.Phase != PhaseLost {
		t.Fatalf("应已终局，实际 %q", e.Phase)
	}
	// 停滞之后战斗逻辑不应再产生任何重放事件。
	// 当前只有停滞一条；B~F 落地后这里会变成「事件数仍只有停滞那条」。
	if len(e.replay) != 1 {
		t.Errorf("停滞后不应再有重放事件，实际 %+v", e.replay)
	}
}

// TestStepAfterTerminalDoesNotAdvance 守住「终局后 Step 不推进」。
//
// ⚠️ 判据包含 `Tick` 不变，不只是「不再有新事件」。
//
//	只查事件的话，一个「把后续 tick 全跑完但不记录」的实��也能通过 ——
//	而它会让 elapsedMs 多涨 50ms，进而让 duration_ms 偏大，
//	症状是「偶尔有战斗的时长比记录多一点点」，极难查。
func TestStepAfterTerminalDoesNotAdvance(t *testing.T) {
	for _, phase := range []string{PhaseWon, PhaseLost} {
		t.Run(phase, func(t *testing.T) {
			e := newProbeEngine()
			e.Phase = phase
			e.Tick = 100
			e.ElapsedMs = 100 * TickMs
			e.Heat.Heat = 42

			for i := 0; i < 10; i++ {
				e.Step()
			}
			if e.Tick != 100 {
				t.Errorf("终局后 tick 仍在推进：%d", e.Tick)
			}
			if e.ElapsedMs != 100*TickMs {
				t.Errorf("终局后 elapsedMs 仍在推进：%d", e.ElapsedMs)
			}
			if e.Heat.Heat != 42 {
				t.Errorf("终局后热量仍在变化：%d", e.Heat.Heat)
			}
		})
	}
}

// TestCheckStalemateIsFalseAfterTerminal 守住双重判定。
//
// TS 的注释：这里返回 true 只支撑**兜底**，
// 而 `Step` 开头的终局判定已经处理了常规终局。
// 单靠 phase 判定不到「tick 耗尽但逻辑认为还没结束」的状态 ——
// 双重判定是必需的，少一个就漏一类。
func TestCheckStalemateIsFalseAfterTerminal(t *testing.T) {
	for _, phase := range []string{PhaseWon, PhaseLost} {
		e := newProbeEngine()
		e.Phase = phase
		e.Tick = MaxBattleTicks + 100 // 远超上限
		if e.CheckStalemate() {
			t.Errorf("phase=%s 时 CheckStalemate 应为 false —— "+
				"终局后再触发停滞会追加第二条终局事件", phase)
		}
	}
}

// TestDrainEventsClearsQueue 守住「取走即清空」。
//
// ⚠️ 不清空的后果是**每个后续 tick 都重发上一批事件** ——
//
//	渲染层会看到事件重复，而事件流进 replay，重复会改变哈希。
func TestDrainEventsClearsQueue(t *testing.T) {
	e := newProbeEngine()
	e.Phase = PhaseWave
	e.Tick = MaxBattleTicks - 1
	e.ElapsedMs = e.Tick * TickMs

	first := e.Step()
	if len(first) == 0 {
		t.Fatal("停滞那一步应发出事件")
	}
	if second := e.DrainEvents(); len(second) != 0 {
		t.Errorf("DrainEvents 之后队列应为空，实际 %d 条", len(second))
	}
}

// TestFinishEmitsTerminalEventOnce 守住「终局只发一次」。
func TestFinishEmitsTerminalEventOnce(t *testing.T) {
	e := newProbeEngine()
	e.Score = 12345
	e.Cfg.Level = GenerateLevel(1)

	e.Finish(true, false)
	evs := e.DrainEvents()
	if len(evs) != 1 || evs[0].Type != EventWon {
		t.Fatalf("胜利应恰好发一条 won 事件，实际 %+v", evs)
	}
	if len(e.replay) != 0 {
		t.Errorf("普通终局不应记重放事件，实际 %+v", e.replay)
	}

	// 停滞终局：多一条 stalemate 重放事件
	e2 := newProbeEngine()
	e2.Finish(false, true)
	evs2 := e2.DrainEvents()
	if len(evs2) != 1 || evs2[0].Type != EventLost {
		t.Fatalf("停滞战败应恰好发一条 lost 事件，实际 %+v", evs2)
	}
	if len(e2.replay) != 1 || e2.replay[0].Type != ReplayStalemate {
		t.Errorf("停滞必须记一条 stalemate 重放事件，实际 %+v", e2.replay)
	}
}

// TestEngineIsPureWrtConstruction 守住「构造不偷跑 RNG」。
//
// ⚠️ 这是重放可复现的前提：引擎必须是纯函数，
//
//	NewBattleEngine(cfg) + 同样的 Step 序列 ⇒ 同样的结果，
//
// 与调用时刻、调用次数、宿主语言都无关。
// 任何「构造时偷跑一次 RNG」都会让重放不可复现 ——
// 而症状是「同一个种子重放两次得到不同哈希」。
func TestEngineIsPureWrtConstruction(t *testing.T) {
	build := func() *BattleEngine {
		e := newProbeEngine()
		e.Phase = PhaseWave
		for i := 0; i < 100; i++ {
			e.Step()
		}
		return e
	}
	a, b := build(), build()
	if a.Tick != b.Tick || a.ElapsedMs != b.ElapsedMs {
		t.Errorf("同样的输入两次运行得到不同 tick 时序：(%d,%d) vs (%d,%d)",
			a.Tick, a.ElapsedMs, b.Tick, b.ElapsedMs)
	}
	if len(a.replay) != len(b.replay) {
		t.Errorf("重放事件数不同：%d vs %d", len(a.replay), len(b.replay))
	}
	// UID 序列也必须从同一个起点开始 —— 它是事件流的一部分。
	if a.uidSeq != b.uidSeq {
		t.Errorf("uidSeq 起点不同：%d vs %d", a.uidSeq, b.uidSeq)
	}
}

// TestNextUIDIsMonotonic 守住 UID 生成。
//
// UID 进 replay 事件的 `a` 位，所以它的分配顺序必须确定性。
func TestNextUIDIsMonotonic(t *testing.T) {
	e := newProbeEngine()
	if got := e.nextUID(); got != 1 {
		t.Errorf("首个 uid 应为 1（TS 的 uidSeq 初值），实际 %d", got)
	}
	for i := int64(2); i <= 10; i++ {
		if got := e.nextUID(); got != i {
			t.Fatalf("第 %d 个 uid 应为 %d，实际 %d", i, i, got)
		}
	}
}

// TestStarsForScore 守住星级阈值。
func TestStarsForScore(t *testing.T) {
	targets := []int64{1000, 3000, 5000}
	cases := []struct {
		score int64
		want  int
	}{
		{0, 0}, {999, 0}, {1000, 1}, {2999, 1},
		{3000, 2}, {4999, 2}, {5000, 3}, {99999, 3},
	}
	for _, c := range cases {
		if got := starsForScore(c.score, targets); got != c.want {
			t.Errorf("starsForScore(%d) = %d，期望 %d", c.score, got, c.want)
		}
	}
	// 空阈值 → 0 星（不是 3 星）
	if got := starsForScore(99999, nil); got != 0 {
		t.Errorf("无阈值时应为 0 星，实际 %d", got)
	}
}

// TestUpdateFloatsOnlyRemovesExpired 守住「浮动文字不进 replay」。
//
// ⚠️ 判据是「不产生任何重放事件」，不只是「表现正确」。
//
//	浮动文字是纯表现层；一旦它开始进 replay，
//	渲染层的任何改动都会改变哈希。
func TestUpdateFloatsOnlyRemovesExpired(t *testing.T) {
	e := newProbeEngine()
	e.Floats = []FloatText{
		{X: 1, Y: 2, Text: "a", LifeMs: 25, MaxLifeMs: 50}, // 这一 tick 后过期
		{X: 3, Y: 4, Text: "b", LifeMs: 200, MaxLifeMs: 200},
	}
	e.updateFloats()
	if len(e.Floats) != 1 || e.Floats[0].Text != "b" {
		t.Errorf("应只剩一条未过期的浮动文字，实际 %+v", e.Floats)
	}
	if len(e.replay) != 0 {
		t.Errorf("浮动文字不得进重放流，实际 %+v", e.replay)
	}
	// 再跑一 tick：LifeMs 恰好归零的也应消失
	e.Floats[0].LifeMs = TickMs
	e.updateFloats()
	if len(e.Floats) != 0 {
		t.Errorf("LifeMs 归零的应被移除，实际 %+v", e.Floats)
	}
}
