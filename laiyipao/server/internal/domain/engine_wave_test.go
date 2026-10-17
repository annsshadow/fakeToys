package domain

// Block B（波次与刷怪）的纯逻辑守卫。
//
// 这些不是跨端对齐，而是**跨端对齐抓不到**的那部分：
// 真实关卡数据里出现不到、或只在构造形状下才成立的路径。
//
// ⚠️ 教训（Block B 实测）：
//
//	把 `updateSpawns` 的倒序遍历改成正序，五个真实场景**全部照样绿**。
//	真实关卡把 `atTick` 拉开了，同 tick 到期的通常只有一只，
//	而「删元素后跳过紧邻项」只在「一次删多项」时发生。
//
//	所以：跨端对齐证明的是「在真实数据上两端一致」，
//	**不覆盖「代码想处理的形状」**。后者要靠构造用例。

import "testing"

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
// ⚠️⚠️ 所有状态计时都是**先递减再判定**，所以 `X = TickMs` 意味着
//
//	「本 tick 正好耗尽」→ 判定为不生效。
//
//	我第一版用 `SlowedMs = TickMs` 期望拿到减速位移 25，实际拿到全速 50
//	—— **是我的期望错了，实现与 TS 一致**。
//	要用到减速必须给 `2*TickMs`，让它熬过本 tick 的递减。
//	这条边界单独列在下一条测试里。
func TestStepEnemyMotionSpeedMath(t *testing.T) {
	e := newProbeEngine()

	// 全速
	en := newMotionEnemy()
	en.Speed = 1000
	x0 := en.X
	e.StepEnemyMotion(en, false)
	if got, want := x0-en.X, int64(50); got != want {
		t.Errorf("全速位移 = %d，期望 %d（1000*50/1000）", got, want)
	}

	// 减速 50%
	en2 := newMotionEnemy()
	en2.Speed = 1000
	en2.SlowedMs = 2 * TickMs // 熬过本 tick 的递减后仍 > 0
	x2 := en2.X
	e.StepEnemyMotion(en2, false)
	if got, want := x2-en2.X, int64(25); got != want {
		t.Errorf("减速位移 = %d，期望 %d（1000*500/1000*50/1000=25）", got, want)
	}
}

// TestStepEnemyMotionStatusExpiresOnItsOwnTick 钉住「先递减后判定」这条语义。
//
// ⚠️ 这是最容易在移植时改错一处：把判定移到递减之前，
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
	//	`slowed` 不阻断，只是把速度减半（两种情况下都在动），
	//	所以它区分不出「本 tick 生效还是耗尽」。
	//	它的边界由 `TestStepEnemyMotionSpeedMath` 的**距离**断言覆盖 ——
	//	把一个只比较「动不动」的用例塞进来，只会得到一个恒真的断言。
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

// TestStepEnemyMotionKnockbackOverridesSpeed 守住击退优先。
func TestStepEnemyMotionKnockbackOverridesSpeed(t *testing.T) {
	e := newProbeEngine()
	en := newMotionEnemy()
	en.Knockback = -30
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

// TestScaledDelayTicks 守住「毫秒→tick 的折算」（第 158/159 轮的 tuning 版）。
//
// ⚠️ 两条不变量：
//
//  1. **向上取整**：保证「间隔 N 毫秒」不会因为截断变成「间隔 0 毫秒」
//     而让同组怪同时出现。反过来（向下取整）会让同组怪**同 tick 全部刷出**，
//     而那种画面差异极难从数据里看出来。
//  2. **下限 1 tick**：TS 是 `Math.max(1, Math.ceil(...))` ——
//     `ms = 0` 或倍率压到 0 tick 时仍然是 1，而不是 0。
//     旧实现（`ceilDivMsToTicks`）对 0 返回 0，与 TS 不一致。
//
// 判据覆盖边界两侧、不能整除、倍率缩放与下限四种情形。
func TestScaledDelayTicks(t *testing.T) {
	cases := []struct {
		name     string
		ms       int64
		permille int64
		want     int64
	}{
		{"零毫秒仍占 1 tick（TS 的 max(1,…)）", 0, 1000, 1},
		{"负毫秒也占 1 tick", -1, 1000, 1},
		{"1ms 不足一 tick 向上取整", 1, 1000, 1},
		{"50ms 恰好一 tick", 50, 1000, 1},
		{"51ms 跨到两 tick", 51, 1000, 2},
		{"1000ms = 20 tick", 1000, 1000, 20},
		{"不改倍率时与旧口径一致（1012ms）", 1012, 1000, 21},
		{"800‰ 缩放第 1 关 delay：1012×0.8=809.6ms → 17", 1012, 800, 17},
		{"850‰ 缩放 interval：600×0.85=510ms → 11", 600, 850, 11},
		{"倍率压到极小仍保 1 tick", 10, 1, 1},
	}
	for _, c := range cases {
		if got := scaledDelayTicks(c.ms, c.permille); got != c.want {
			t.Errorf("scaledDelayTicks(%d, %d) = %d，期望 %d（%s）",
				c.ms, c.permille, got, c.want, c.name)
		}
	}
}

// TestUpdateSpawnsHandlesMultipleDueInOneTick 守住「倒序遍历」。
//
// # 为什么跨端对齐**抓不到**这个 bug
//
// 实测：把 `updateSpawns` 的倒序改成正序，五个真实场景全部照样绿。
// 原因是真实关卡的刷怪组把 `atTick` 拉开了，同 tick 到期的通常只有一只。
//
// 构造依据合法：这是「同组怪同时出现」的极限形态 ——
// 而 `scaledDelayTicks` 的下限 1 tick 设计意图恰恰是**避免**它。
// 所以这是「配置的边界」，不是「配置的正常用法」。
//（调用方若用 BeginWave 造队列，间隔 0ms 也会被抬到 1 tick；本用例直接
// 构造 spawnQueue，绕过折算，专测倒序遍历本身的正确性。）
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
	//	因为遍历是 i 从尾到头，push 到 enemies 的顺序就是遍历顺序。
	//	TS 用的是同一个 `for (let i = len-1; i >= 0; i--)`，所以两端一致。
	//	我第一版期望正序，错了 —— 而这恰恰是「倒序遍历」最容易写坏的地方：
	//	改成正序后次序会变，但它仍然是「刷出 3 只」，只查数量抓不到。
	for i, want := range []int64{3, 2, 1} {
		if e.Enemies[i].UID != want {
			t.Errorf("场上第 %d 只的 uid = %d，期望 %d（倒序遍历 => 倒序出现，与 TS 同）",
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
// 五个真实场景都抓不到它：它们的 waves 都非空。
func TestEmptyLevelIsNotAutoWin(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Level = GeneratedLevel{ID: 999, Waves: nil}
	e.TotalEnemies = 0
	e.Score = 4242

	e.Start() // -> BeginWave(0) -> waves[0] 不存在 -> 应判负

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

// TestBeginWaveAtLaterIndex 守住「第 2 波及以后的 tick 偏移」。
//
// # 为什么需要它
//
// 跨端对齐的场景**全部停在第 1 波** —— 进第 2 波要先解掉卡牌选择，
// 那要过 `rollWaveCards`（消耗 RNG）与 `skipCards`，都属于 F 块。
//
// 而 `BeginWave(index)` 对 `index >= 1` 与 `index == 0` 唯一的差别是
//
//	atTick = 当前 tick + 折算后的 delay
//
// 第 1 波时当前 tick 恒为 0，于是 `+ 当前 tick` 这一项**永远是加 0**，
// 把 `e.Tick` 写成常量 0 也能全绿。这个 bug 只会在第 2 波暴露成
// 「整波怪提前刷出」。
func TestBeginWaveAtLaterIndex(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Level = GeneratedLevel{
		ID: 1,
		Waves: []Wave{
			{Index: 0, Spawns: []Spawn{{EnemyID: 1, Count: 1}}},
			{Index: 1, Spawns: []Spawn{
				{EnemyID: 1, Count: 1, Delay: 1012},
				{EnemyID: 2, Count: 2, Delay: 500, Interval: 700},
			}},
		},
	}
	e.Cfg.Enemies = map[int]SeedEnemy{
		1: {ID: 1, HP: 100},
		2: {ID: 2, HP: 100},
	}
	e.Tick = 4321 // 任意非零：第 1 波时它恒为 0，所以要显式构造

	e.BeginWave(1)

	if len(e.spawnQueue) != 3 {
		t.Fatalf("第 2 波应有 3 项刷怪，实际 %d 项", len(e.spawnQueue))
	}
	// 比对**集合**而不是队列顺序：BeginWave 末尾有洗牌，
	// 顺序由 RNG 决定（跨端对齐已经逐项比过顺序）。
	// 这里只关心「atTick 算对了没」——
	// 把两件事搅在一起断言，失败时说不清是哪一处坏了。
	//
	// 期望值（第 158/159 轮起要过 tuning 倍率，DEFAULT: delay 800‰ / interval 850‰）：
	//
	//	1012ms × 800‰ = 809.6ms → ceil(809.6/50) = 17 → 4321+17          = 4338
	//	 500ms × 800‰ = 400ms   → ceil(400/50)   =  8 → 4321+8           = 4329
	//	 700ms × 850‰ = 595ms   → ceil(595/50)   = 12 → 4329+12          = 4341
	wantTicks := map[int64]bool{4338: true, 4329: true, 4341: true}
	for i, q := range e.spawnQueue {
		if !wantTicks[q.AtTick] {
			t.Errorf("刷怪队列[%d].AtTick = %d，不在期望集合 {4329, 4338, 4341} 内 —— "+
				"它必须含 e.Tick 偏移 = 4321（写死 0 的话第 1 波全绿、第 2 波整波提前刷）",
				i, q.AtTick)
		}
		delete(wantTicks, q.AtTick)
	}
	if len(wantTicks) != 0 {
		t.Errorf("期望集合里还剩 %d 个没被刷出来：%v", len(wantTicks), wantTicks)
	}
	if e.WaveIndex != 1 || e.Phase != PhaseWave {
		t.Errorf("BeginWave(1) 后 waveIndex=%d phase=%q，期望 1/wave", e.WaveIndex, e.Phase)
	}
}
