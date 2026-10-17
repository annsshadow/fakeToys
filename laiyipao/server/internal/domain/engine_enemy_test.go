package domain

// Block C（敌人推进与漏怪结算）的纯逻辑守卫。
//
// 与 B 同理：这些是**跨端对齐抓不到**的形状。
// 跨端场景跑的是「真实关卡 + 空构筑」，
// 于是「两条漏怪路径互斥」「护甲三项相加」「射程边界」这些形状
// 只能靠构造用例打。

import "testing"

// leakProbeEngine 造一个受控的漏怪夹具。
func leakProbeEngine(baseHP, baseArmor int64) *BattleEngine {
	return NewBattleEngine(BattleConfig{
		Level: GeneratedLevel{
			ID: 1, BaseHP: baseHP, ArmorPermille: baseArmor,
			Waves: []Wave{{Index: 0}},
		},
		ActiveSlots: ActiveSlots,
		ScoreRules:  DefaultScoreRules(),
	})
}

// TestLeakDamageForAttackerBranch 守住「有攻击力」那条路径。
//
// 公式：`底血 × attack / LeakAttackerDivisor` 再过护甲。
//
// ⚠️ 判据给**具体数值**并逐项写出来：
//
//	1000 × 3600 / 3600 = 1000，再过 applyArmor(0‰) = 1000。
//	用「attack/3600」这个约分看着对，但把除数写成 360 就会得到 10000 ——
//	那是个 10 倍的偏差，而「血掉得更快」在实战里表现为
//	「玩家莫名打不通第 1 关」，没人会怀疑是分母少了个 0。
func TestLeakDamageForAttackerBranch(t *testing.T) {
	e := leakProbeEngine(1000, 0)
	en := &Enemy{Attack: 3600}
	if got := e.leakDamageFor(en); got != 1000 {
		t.Errorf("1000 底血 × attack 3600 / %d = %d，期望 1000", LeakAttackerDivisor, got)
	}
	// attack 为 1 时应当**几乎不掉血**：
	// 1000*1/3600 = 0。这是真实内容表里最低攻击力的量级。
	en2 := &Enemy{Attack: 1}
	if got := e.leakDamageFor(en2); got != 0 {
		t.Errorf("attack=1 时期望 0（1000/3600 向零截断），实际 %d", got)
	}
}

// TestLeakDamageForBreachBranch 守住「无攻击力」那条路径。
//
// 公式：`底血 / LeakDamageDivisor`。
func TestLeakDamageForBreachBranch(t *testing.T) {
	e := leakProbeEngine(1000, 0)
	en := &Enemy{Attack: 0}
	if got := e.leakDamageFor(en); got != 100 {
		t.Errorf("零攻击力时期望 底血/%d = 100，实际 %d", LeakDamageDivisor, got)
	}
	// 负攻击力也走这档 —— `attack <= 0` 而不是 `=== 0`。
	// 写成 `=== 0` 的话负值会掉进「先乘后除」分支得到负伤害，
	// 而 applyArmor 对负数是**原样返回**（它把负伤害当反伤标记），
	// 于是 `baseHp -= 负数` = **加血**。关卡数据里出现负 attack 时，
	// 防线会越漏越厚。
	enNeg := &Enemy{Attack: -5}
	if got := e.leakDamageFor(enNeg); got != 100 {
		t.Errorf("负攻击力应与零同档（期望 100），实际 %d —— "+
			"落进乘除分支会得到负伤害，而 applyArmor 原样返回负数 = 加血", got)
	}
}

// TestBreachDamageFallbackIsNotZero 守住 `baseHpMax <= 0` 的兜底值。
//
// ⚠️ 兜底必须是 100 而不是 0。
//
//	零伤害意味着「漏怪不扣血」：baseHp 永远不归零、`leaked` 无限增长，
//	一关就能白刷无限个 leaked。而 `leaked` 是结算字段之一，
//	无限大只会被 `ErrTooManyLeaked` 挡住 —— 玩家看到的是
//	「结算失败」，不是「防线破了」。
func TestBreachDamageFallbackIsNotZero(t *testing.T) {
	e := leakProbeEngine(0, 0)
	if got := e.breachDamage(); got != BreachDamageFallback {
		t.Errorf("baseHpMax=0 时兜底 = %d，期望 %d", got, BreachDamageFallback)
	}
	if BreachDamageFallback <= 0 {
		t.Error("兜底值必须为正 —— 0 会让漏怪永远不扣血")
	}
}

// TestDefenseArmorPermilleSumsThreeTerms 守住「有效护甲 = 三项相加」。
//
// ⚠️ 这三项在真实场景里**永远只有一项非零**：
//
//	关卡 armor_permille（这批场景是 0）+ 卡牌 buffs（空构筑是 0）
//	+ 攻方 attacker（默认 0）。
//
//	所以不给构造值的话，把它写成减号也照样全绿。
func TestDefenseArmorPermilleSumsThreeTerms(t *testing.T) {
	e := leakProbeEngine(1000, 250)
	e.Buffs.ArmorPermille = 100
	e.Cfg.Attacker.ArmorPermille = 400
	if got := e.defenseArmorPermille(); got != 750 {
		t.Errorf("有效护甲 = %d，期望 250+100+400 = 750", got)
	}
	// ⚠️ 改其中一个符号会立刻改掉这个值 ——
	//   这是三条里唯一能让「写成减号」被抓住的构造。
	if got := e.defenseArmorPermille(); got != e.BaseArmorPermille+e.Buffs.ArmorPermille+e.Cfg.Attacker.ArmorPermille {
		t.Error("有效护甲必须是三项相加")
	}
}

// TestDefenseArmorPermilleIsClampedByApplyArmor 守住封顶行为。
//
// 合计 1100‰ 远超 750‰ 的封顶：护甲不能完全免疫漏怪。
// ⚠️ 封顶发生在 `applyArmor` 内部而不是这里，
//
//	所以 `defenseArmorPermille()` 本身会返回 1100 ——
//	断言「返回 750」会与实现不符，且会让「把封顶提前」这个
//	同样合法的改法变得不可见。
func TestDefenseArmorPermilleIsClampedByApplyArmor(t *testing.T) {
	e := leakProbeEngine(1000, 600)
	e.Buffs.ArmorPermille = 250
	e.Cfg.Attacker.ArmorPermille = 250
	if got := e.defenseArmorPermille(); got != 1100 {
		t.Fatalf("有效护甲（未封顶）= %d，期望 1100", got)
	}
	// 100 点伤害过 1100‰ 的等效护甲：封顶到 750‰ → 100 × 250/1000 = 25
	if got := e.leakDamageFor(&Enemy{Attack: 360}); got != 25 {
		t.Errorf("封顶后伤害 = %d，期望 25（100*250/1000）", got)
	}
	// 底线：任何护甲都不可能让漏怪伤害归零
	if got := e.leakDamageFor(&Enemy{Attack: 3600}); got <= 0 {
		t.Errorf("封顶后 1000 底血的漏怪伤害 = %d，必须为正 —— "+
			"护甲完全免疫会让防线不可破", got)
	}
}

// TestRangedAndArriveAreMutuallyExclusive 守住「远程攻击」与「抵达」互斥。
//
// ⚠️ 这是本块最贵的 bug：`x > BaseX` 那个条件是**必需**的，不是优化。
//
// 敌人越过防线后 `dist = x - BaseX` 变成负数，`<= attackRange` 必然成立，
// 于是同一个 tick 里既远程打一次、又按抵达扣一次 ——
// **双倍伤害、leaked 计 2、发出两条 leak 事件**。
//
// 症状极隐蔽：它只影响远程怪（近战怪 attack 条件不成立），
// 而「近战怪的防线数值看起来对」是最容易让人停下来的地方。
func TestRangedAndArriveAreMutuallyExclusive(t *testing.T) {
	// ⚠️⚠️ 血池必须扛得住**两次**漏怪，否则这条守卫是空的。
	//
	// 我第一版用 base_hp=100000、attack=3600 ——
	// 单次伤害 = 100000*3600/3600 = 100000，**一次就打空**，
	// 于是 `if baseHp <= 0 { finish(); return }` 立刻短路，
	// 第二个分支根本没机会执行。
	//
	// 实测：把 `x > BaseX` 那个守卫删掉，本测试**照样绿**。
	// 而那个守卫正是本函数要守的东西 ——
	// 删掉它的后果是「同一 tick 双倍伤害 + leaked 计 2 + 两条 leak 事件」。
	//
	// 教训：漏怪守卫的血池不能按「一次就死」来配，
	// 否则它在结构上就观测不到「第二次」。
	const (
		baseHP   = 100000
		attacker = 36 // 单次伤害 = 100000*36/3600 = 1000
		dmgOnce  = baseHP * attacker / LeakAttackerDivisor
	)

	e := leakProbeEngine(baseHP, 0)
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100, Attack: attacker}}
	e.Phase = PhaseWave
	e.BaseHP = baseHP
	e.Enemies = []*Enemy{{
		UID: 1, DefID: 1, X: ToFixed(float64(BaseX)), // 正好在防线上
		HP: 100, MaxHP: 100, SpawnProgress: SpawnProgressFull,
		Attack: attacker, AttackRange: 600, AttackInterval: 1000,
		AttackCooldown: 0,
	}}

	e.updateEnemies()

	// x == BaseX 时 `x > BaseX` 不成立 -> 只走抵达分支 -> 漏 1 只
	if e.Leaked != 1 {
		t.Errorf("正好抵达防线时应只漏 1 只，实际漏 %d 只 —— "+
			"远程分支与抵达分支同时生效就是双倍伤害", e.Leaked)
	}
	if e.BaseHP != baseHP-dmgOnce {
		t.Errorf("防线血 = %d，期望 %d（单次 %d）", e.BaseHP, baseHP-dmgOnce, dmgOnce)
	}
	var leaks int
	for _, ev := range e.DrainEvents() {
		if ev.Type == EventLeak {
			leaks++
		}
	}
	if leaks != 1 {
		t.Errorf("应恰好发 1 条 leak 事件，实际 %d 条", leaks)
	}
	// 重放事件也必须只有一条 —— 双倍伤害同时意味着回放里有两条 leak
	var replayLeaks int
	for _, r := range e.replay {
		if r.Type == ReplayLeak {
			replayLeaks++
		}
	}
	if replayLeaks != 1 {
		t.Errorf("应恰好记录 1 条 leak 重放事件，实际 %d 条", replayLeaks)
	}
}

// TestRangedFiresOnlyInsideRange 守住射程判定。
//
// ⚠️ 射程内扣血与抵达扣血**共用同一个数量级**（见 leakDamageFor 的注释），
//
//	所以射程内的敌人也走 `attack > 0` 那档，不是固定小数字。
//
// ⚠️⚠️ 这里的 `X` 必须是**定点**（×1000）。
//
//	我第一版传了逻辑值（661 而不是 661000），于是：
//	  dist = 661 - 60000 = -59339  -> `-59339 <= 600000` 成立 -> 远程开火
//	  `661 <= 60000` 也成立          -> 抵达扣血
//	两个分支**同时**触发，测试却是绿的 —— 因为我只断言了「漏了 1 只」，
//	而那次两条路径恰好给出同一个计数。
//
//	这恰好是 `TestRangedAndArriveAreMutuallyExclusive` 注释里说的那个 bug
//	的现场：一个「看起来对」的夹具，掩盖了双倍伤害。
func TestRangedFiresOnlyInsideRange(t *testing.T) {
	mk := func(xLogical int) *BattleEngine {
		e := leakProbeEngine(1000000, 0)
		e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100}}
		e.Phase = PhaseWave
		e.BaseHP = 1000000
		e.Enemies = []*Enemy{{
			UID: 1, DefID: 1,
			X:  ToFixed(float64(xLogical)), // ← 定点，不是逻辑值
			HP: 100, MaxHP: 100,
			SpawnProgress: SpawnProgressFull,
			Attack:        3600, AttackRange: 600, AttackInterval: 1000,
		}}
		e.Heat.Overheated = true // 冻结位移，让坐标可控
		return e
	}

	// 射程内：正好在边界（判据是 `<= attackRange`，边界算命中）
	inside := mk(BaseX + 600)
	inside.updateEnemies()
	if inside.Leaked != 1 {
		t.Errorf("正好在射程边界（x=%d）应扣血，实际漏 %d 只 —— "+
			"判据是 `<= attackRange`，边界必须算命中", BaseX+600, inside.Leaked)
	}
	if inside.Enemies[0].X != ToFixed(float64(BaseX+600)) {
		t.Errorf("射程内的敌人不该移动，x = %d，期望 %d",
			inside.Enemies[0].X, ToFixed(float64(BaseX+600)))
	}

	// 射程外一格
	outside := mk(BaseX + 601)
	outside.updateEnemies()
	if outside.Leaked != 0 {
		t.Errorf("射程外（x=%d）不应扣血，实际漏 %d 只", BaseX+601, outside.Leaked)
	}
	if len(outside.Enemies) != 1 {
		t.Errorf("射程外的敌人不该被移除，实际剩 %d 只", len(outside.Enemies))
	}
}

// TestBurrowAndZeroAttackDoNotRangedFire 守住远程分支的三个前置条件。
//
// ⚠️ 三个条件（`!burrow` / `attack > 0` / `attackRange > 0`）缺一不可，
//
//	去掉任何一个都会让「不该打的怪」每 tick 打一次。
func TestBurrowAndZeroAttackDoNotRangedFire(t *testing.T) {
	cases := []struct {
		name    string
		burrow  bool
		attack  int64
		atkRng  int64
		wantHit bool
	}{
		{"正常", false, 3600, 600, true},
		{"钻地", true, 3600, 600, false},
		{"零攻击", false, 0, 600, false},
		{"零射程", false, 3600, 0, false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			e := leakProbeEngine(10000, 0)
			e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100}}
			e.Phase = PhaseWave
			e.BaseHP = 10000
			e.Enemies = []*Enemy{{
				UID: 1, DefID: 1, X: ToFixed(float64(BaseX)) + 300, HP: 100, MaxHP: 100,
				SpawnProgress: SpawnProgressFull, Burrow: c.burrow,
				Attack: c.attack, AttackRange: c.atkRng, AttackInterval: 1000,
			}}
			e.updateEnemies()
			hit := e.Leaked > 0
			if hit != c.wantHit {
				t.Errorf("远程开火 = %v，期望 %v（burrow=%v attack=%d range=%d）",
					hit, c.wantHit, c.burrow, c.attack, c.atkRng)
			}
		})
	}
}

// TestAttackCooldownFiresOnInterval 守住攻击间隔。
//
// ⚠️ 必须测「冷却归零的那一 tick」与「归零后立刻重置为间隔」两件事：
//
//	漏怪伤害的**总量**完全取决于「一共开火了几次」，
//	而开火次数只由冷却决定 —— 冷却差一 tick，一整局的漏怪总量就变了。
func TestAttackCooldownFiresOnInterval(t *testing.T) {
	e := leakProbeEngine(100000, 0)
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100}}
	e.Phase = PhaseWave
	e.BaseHP = 100000
	e.Enemies = []*Enemy{{
		UID: 1, DefID: 1, X: ToFixed(float64(BaseX)) + 300, HP: 100, MaxHP: 100,
		SpawnProgress: SpawnProgressFull,
		Attack:        3600, AttackRange: 600, AttackInterval: 1000,
		AttackCooldown: 1000,
	}}
	e.Heat.Overheated = true // 冻结位移

	// 冷却 1000ms、每 tick 50ms → 第 20 次递减到 0 才开火
	for i := 0; i < 19; i++ {
		e.updateEnemies()
		if e.Leaked != 0 {
			t.Fatalf("第 %d 次就开火了，冷却 1000ms 应要 20 tick", i+1)
		}
	}
	e.updateEnemies()
	if e.Leaked != 1 {
		t.Fatalf("第 20 次应开火，实际漏 %d 只", e.Leaked)
	}
	if e.Enemies[0].AttackCooldown != 1000 {
		t.Errorf("开火后冷却应重置为间隔 1000，实际 %d", e.Enemies[0].AttackCooldown)
	}
	// 再 19 次不开火，第 20 次再开
	for i := 0; i < 19; i++ {
		e.updateEnemies()
	}
	if e.Leaked != 1 {
		t.Errorf("第二个周期内不该开火，实际漏 %d 只", e.Leaked)
	}
	e.updateEnemies()
	if e.Leaked != 2 {
		t.Errorf("第二个周期的第 20 次应开火，实际漏 %d 只", e.Leaked)
	}
}

// TestBaseHPClampsAtZeroAndFinishes 守住「防线破了判负」。
func TestBaseHPClampsAtZeroAndFinishes(t *testing.T) {
	e := leakProbeEngine(1000, 0)
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100}}
	e.Phase = PhaseWave
	e.BaseHP = 1000
	e.Enemies = []*Enemy{{
		UID: 1, DefID: 1, X: ToFixed(float64(BaseX)), HP: 100, MaxHP: 100,
		SpawnProgress: SpawnProgressFull, Attack: 3600, AttackRange: 0, AttackInterval: 0,
	}}

	e.updateEnemies()

	if e.Phase != PhaseLost {
		t.Errorf("血归零应判负，实际 phase=%q", e.Phase)
	}
	if e.BaseHP != 0 {
		t.Errorf("血应被夹到 0，实际 %d —— 负血会一路减下去", e.BaseHP)
	}
}

// TestDeadEnemiesAreFilteredOut 守住「本 tick 死的敌人被移除」。
//
// ⚠️ 移除**时机**是「整轮循环之后」而不是「死的那一刻」：
//
//	立刻移除会让遍历中的下标失效（Go 里是 panic / TS 里是跳项），
//	而 TS 的 `filter` 是循环后一次性替换。
func TestDeadEnemiesAreFilteredOut(t *testing.T) {
	e := leakProbeEngine(100000, 0)
	e.Cfg.Enemies = map[int]SeedEnemy{
		1: {ID: 1, HP: 100},
		2: {ID: 1, HP: 100},
	}
	e.Phase = PhaseWave
	e.BaseHP = 100000
	e.Enemies = []*Enemy{
		{UID: 1, DefID: 1, X: ToFixed(float64(BaseX)), HP: 100, MaxHP: 100,
			SpawnProgress: SpawnProgressFull},
		{UID: 2, DefID: 2, X: 500000, HP: 100, MaxHP: 100,
			SpawnProgress: SpawnProgressFull},
	}

	e.updateEnemies()

	if len(e.Enemies) != 1 || e.Enemies[0].UID != 2 {
		t.Errorf("应只剩 uid=2 那只，实际 %d 只：%v", len(e.Enemies), e.Enemies)
	}
}

// TestWithinEnemyUsesSquaredDistance 守住命中半径。
//
// ⚠️ 半径随 `flyHeight` 放大：`toFixed(28 + flyHeight*0.1)`。
//
//	飞行单位在 `r` 上比地面单位大 ——
//	而这条正是「溅射/穿透能打到飞行单位」的实现。
func TestWithinEnemyUsesSquaredDistance(t *testing.T) {
	e := newProbeEngine()
	ground := &Enemy{X: 500000, Y: 500000, FlyHeight: 0}
	fly := &Enemy{X: 500000, Y: 500000, FlyHeight: 100}

	// 地面：r = 28 → 28000 定点
	rGround := ToFixed(28)
	if !e.withinEnemy(ground, 500000+rGround, 500000) {
		t.Error("地面单位：正好在半径边界上应当命中")
	}
	if e.withinEnemy(ground, 500000+rGround+1, 500000) {
		t.Error("地面单位：超出半径 1 定点不应当命中")
	}
	// 飞行：r = 28 + 10 = 38 → 38000 定点，比地面远 10000
	rFly := ToFixed(38)
	if !e.withinEnemy(fly, 500000+rFly, 500000) {
		t.Error("飞行单位：半径应放大到 38")
	}
	if e.withinEnemy(ground, 500000+rFly, 500000) {
		t.Error("地面单位：不应被飞行单位的放大半径覆盖")
	}
	// 斜向：平方距离要同时算 dx 与 dy
	if e.withinEnemy(ground, 500000+ToFixed(20), 500000+ToFixed(20)) {
		t.Error("对角方向也要算进去：dx=20, dy=20 时距离 28.3 > 28，不应命中")
	}
}

// TestLeakReplayEventCarriesUIDAndDamage 守住重放事件的两个参数位。
//
// ⚠️ 曾经的 bug：两条漏怪路径一处传 uid 一处传伤害，
//
//	而射程内那条传的是 `uidOf(dmg)` ——
//	函数名说「取 uid」而实参是伤害值，说明写的时候想的是别的东西。
//	后果是「这条事件不记录是哪只怪漏的」，事后无法定位责任目标。
//
// 现在的约定：`a` = 敌人 uid，`b` = 伤害值。
func TestLeakReplayEventCarriesUIDAndDamage(t *testing.T) {
	e := leakProbeEngine(1000, 0)
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100}}
	e.Phase = PhaseWave
	e.BaseHP = 1000
	e.Enemies = []*Enemy{{
		UID: 4242, DefID: 1, X: ToFixed(float64(BaseX)), HP: 100, MaxHP: 100,
		SpawnProgress: SpawnProgressFull, Attack: 3600, AttackRange: 0,
		AttackInterval: 0,
	}}

	e.updateEnemies()

	var found *ReplayEvent
	for i := range e.replay {
		if e.replay[i].Type == ReplayLeak {
			found = &e.replay[i]
			break
		}
	}
	if found == nil {
		t.Fatal("应记录一条 leak 重放事件")
	}
	if found.A != 4242 {
		t.Errorf("leak 事件的 a（应为敌人 uid）= %d，期望 4242", found.A)
	}
	if found.B != 1000 {
		t.Errorf("leak 事件的 b（应为伤害值）= %d，期望 1000", found.B)
	}
	if found.T != e.Tick {
		t.Errorf("leak 事件的 tick = %d，期望 %d", found.T, e.Tick)
	}
}
