package domain

// 波次与刷怪（R1 Block B）。
//
// 本块把 `engine.go` 里的 `BeginWave` / `spawnEnemy` / `updateSpawns` /
// `stepEnemyMotion` / `checkWaveEnd` 从空实现换成真实实现，
// 并第一次拿到**跨端逐事件对齐**的验证。

// 刷怪进度常量。
//
// ⚠️ `spawnProgress` 是 0..1000 的**整数**千分比，不是 0..1 的浮点。
//
//	写成浮点会在「加 0.08」这一步产生二进制表示误差，
//	而 `Math.min(1000, p + 0.08)` 的边界恰好决定「这只怪何时开始移动」——
//	差一个 tick 就整局错位。
const (
	// SpawnProgressFull 是刷怪进度满值。
	SpawnProgressFull = 1000
	// SpawnProgressStep 是每 tick 的刷怪进度增量。
	//
	// 1000 / 80 ≈ 12.5 tick 的出场动画。
	SpawnProgressStep = 80
	// TickBig 是 TickMs 的定点形式（`mulDiv` 那条链上要用 BigInt）。
	TickBig int64 = TickMs
)

// 波次与刷怪的渲染事件类型。
const (
	EventWaveStart = "wave_start"
	EventWaveClear = "wave_clear"
	EventCardOffer = "card_offer"
)

// BeginWave 开第 index 波。
//
// # ⚠️ delay/interval 的单位是**毫秒**，而 atTick 是 **tick 数**
//
// 直接相加等于把整关节奏放慢 50 倍：实测第 1 关第一波 delay=1012ms
// 被当成 1012 tick = 50.6 秒才出第一只怪，全关 39 只怪要刷 18.7 分钟，
// 期间玩家在空场干等。
//
// 这里统一折算成 tick，并**向上取整** —— 向上取整保证「间隔 N 毫秒」
// 不会因为截断变成「间隔 0 毫秒」而让同组怪同时出现。
func (e *BattleEngine) BeginWave(index int) {
	e.WaveIndex = index
	e.Phase = PhaseWave
	e.Deck.NewWave()
	e.Deck.DiscardsLeft += e.Buffs.FreeDiscard
	// 每波重置「本波弃过牌」标记 ——
	// 编码只对**当前波**的 card_picks 项有意义。
	e.discardedThisWave = false

	var wave Wave
	found := false
	if index >= 0 && index < len(e.Cfg.Level.Waves) {
		wave = e.Cfg.Level.Waves[index]
		found = true
	}
	if !found {
		// ⚠️ 空关卡**不是胜利**。
		//
		// 原先这里是 finish(true)：waves=[] 的关卡 start() 立刻判胜，
		// 而服务端 MaxKillsFor(gl)=0 让 `kills(0) >= 0` 成立，照样发掉落。
		// 畸形关卡因此变成「秒胜 + 发钱」。
		//
		// 当前 levelgen 必填 >=5 波，所以不可达；但 level 是网络数据，
		// 缺失字段时这个分支正是入口。
		e.Finish(false, false)
		return
	}

	e.spawnQueue = nil
	for _, group := range wave.Spawns {
		delayTicks := ceilDivMsToTicks(int64(group.Delay))
		intervalTicks := ceilDivMsToTicks(int64(group.Interval))
		for i := 0; i < group.Count; i++ {
			e.spawnQueue = append(e.spawnQueue, SpawnItem{
				EnemyID: group.EnemyID,
				AtTick:  e.Tick + delayTicks + int64(i)*intervalTicks,
				UID:     e.nextUID(),
			})
		}
	}

	// 确定性洗牌：同一波内的出场顺序由 PRNG 决定。
	//
	// ⚠️ 必须是 Fisher-Yates 而不是 sort.Slice：洗牌的**具体交换序列**
	// 决定 RNG 被消耗的次数与顺序，而 RNG 状态又决定后续所有随机量。
	// 换一个算法等价但消耗不同的实现，刷怪顺序就会变。
	for i := len(e.spawnQueue) - 1; i > 0; i-- {
		j := e.Rng.Intn(i + 1)
		e.spawnQueue[i], e.spawnQueue[j] = e.spawnQueue[j], e.spawnQueue[i]
	}

	e.emit(BattleEvent{Type: EventWaveStart, X: int64(index)})
	e.record(e.Tick, ReplayWave, int64(index), 0, 0)
}

// ceilDivMsToTicks 把毫秒折算成 tick 数，**向上取整**。
func ceilDivMsToTicks(ms int64) int64 {
	if ms <= 0 {
		return 0
	}
	return (ms + TickMs - 1) / TickMs
}

// spawnEnemy 造一个敌人实例。
//
// ⚠️ 护甲这里**只能**是敌人自身的，不能加 `Buffs.ArmorPermille`。
//
// `Buffs.ArmorPermille` 来自卡牌「加固工事：防线护甲 +10%」，
// 玩家预期是**减少漏怪时的伤害**。但它曾被加到敌人的护甲上，
// 于是效果完全反向：一张写着「防线护甲」的卡让玩家变弱 ——
// 玩家所有伤害打敌人时先被减 10%，而漏怪伤害一分不减免。
//
// 正确的消费点是漏怪结算的两处 `applyArmor`（C 块），
// 那里用的才是 `BaseArmorPermille`。
func (e *BattleEngine) spawnEnemy(enemyID int, uid int64) {
	def, ok := e.Cfg.Enemies[enemyID]
	if !ok {
		return
	}
	resist := make(map[Element]int64, len(AllElements()))
	for _, el := range AllElements() {
		v := def.Resist[el]
		resist[el] = clampInt64(v, -MaxResistPermille, MaxResistPermille)
	}
	en := &Enemy{
		UID:             uid,
		DefID:           enemyID,
		Name:            def.Name,
		Cat:             def.Category,
		X:               ToFixed(float64(FieldW - 40)),
		Y:               ToFixed(float64(600 + e.Rng.Intn(340))),
		HP:              def.HP,
		MaxHP:           def.HP,
		Shield:          def.ShieldHP,
		ArmorPermille:   def.Armor,
		FlyHeight:       def.FlyHeight,
		Burrow:          def.Burrow,
		IsBoss:          def.IsBoss,
		Resist:          resist,
		Stacks:          map[Element]int64{},
		Speed:           def.Speed,
		Attack:          def.Attack,
		AttackRange:     def.AttackRange,
		AttackInterval:  def.AttackEvery,
		AttackCooldown:  def.AttackEvery,
		AmplifyPermille: 0,
		SpawnProgress:   0,
	}
	e.Enemies = append(e.Enemies, en)
}

// updateSpawns 处理刷怪队列。
//
// ⚠️ **倒序**遍历 + 命中即删。
//
//	正序遍历会在遍历过程中删元素，导致跳过紧邻的项 ——
//	症状是「同组怪有时少刷一只」，且只在刷怪密集的波次出现。
func (e *BattleEngine) updateSpawns() {
	for i := len(e.spawnQueue) - 1; i >= 0; i-- {
		if e.spawnQueue[i].AtTick <= e.Tick {
			e.spawnEnemy(e.spawnQueue[i].EnemyID, e.spawnQueue[i].UID)
			e.spawnQueue = append(e.spawnQueue[:i], e.spawnQueue[i+1:]...)
		}
	}
}

// StepEnemyMotion 推进敌人位置。
//
// 抽成方法是为了让引擎内部调用与测试调用走同一份逻辑 ——
// 之前位移写在 `updateEnemies` 开头，导致测试无法单独验证。
//
// # 本方法属于 B 块，但它的**引擎级验收在 C 块**
//
// 它由 `updateEnemies` 逐敌调用，而 `updateEnemies` 是 C 块的。
// 所以 B 的引擎级对齐里 `spawnProgress` 全程为 0（B 不移动敌人）——
// 那是 B 的如实边界，不是缺陷。位移本身的验收在 C。
//
// 本方法的纯逻辑（出场进度、状态衰减、冻结、击退、减速）由
// `TestStepEnemyMotion*` 直接覆盖，不依赖引擎。
//
// ⚠️ `e.Dead` 守卫的 true 分支目前**不可达**（唯一调用点 `updateEnemies`
// 在循环里已先跳过 dead）。保留是防御未来新增调用点。
func (e *BattleEngine) StepEnemyMotion(en *Enemy, frozenAll bool) {
	if en.Dead {
		return
	}
	// ⚠️ 出场动画期间**直接返回**：既不衰减状态也不移动。
	//   漏掉这条 return 会让「正在出场」的怪也参与冻结判定，
	//   而出场进度会随 tick 变化 → 每只怪的冻结起始时刻都不同。
	if en.SpawnProgress < SpawnProgressFull {
		en.SpawnProgress += SpawnProgressStep
		if en.SpawnProgress > SpawnProgressFull {
			en.SpawnProgress = SpawnProgressFull
		}
		return
	}
	if en.HitFlashMs > 0 {
		en.HitFlashMs -= TickMs
	}
	if en.FrozenMs > 0 {
		en.FrozenMs -= TickMs
	}
	if en.StunnedMs > 0 {
		en.StunnedMs -= TickMs
	}
	if en.SlowedMs > 0 {
		en.SlowedMs -= TickMs
	}
	if en.ArmorShredMs > 0 {
		en.ArmorShredMs -= TickMs
	}

	if en.FrozenMs > 0 || en.StunnedMs > 0 || frozenAll {
		return
	}

	if en.Knockback != 0 {
		en.X += en.Knockback
		en.Knockback = 0
		return
	}
	slowFactor := int64(1000)
	if en.SlowedMs > 0 {
		slowFactor = 500
	}
	// ⚠️ 先乘后除，与 `mulDiv` 的语义一致。
	//   交换顺序会在 slowFactor<1000 时向下取整到 0 —— 减速反而让怪不动。
	step := mulDiv(en.Speed, slowFactor, 1000) * TickBig / 1000
	en.X -= step
}

// checkWaveEnd 判定波次结束。
//
// 通关语义：**清空全部波次且防线未破**。
//
// ⚠️ 这里**不能**改成 `kills >= totalEnemies`（曾一度对齐过服务端，那个口径是错的）：
// 漏怪的敌人已经扣了 base_hp（那就是「漏怪容忍度」的设计载体），
// 若血还够却因为「漏过」而直接判负，等于同一件事惩罚两次且第二次更严。
// 实测第 1 关默认构筑 kills=32 / leaked=7，于是永远无法通关。
//
// 现在两端统一为「清空即胜」：
//
//	引擎：波次队列空 + 场上无敌人 + 防线未破
//	服务端：kills + leaked >= 总怪数（两者都表示「这只怪已不再构成威胁」）
func (e *BattleEngine) checkWaveEnd() {
	if len(e.spawnQueue) > 0 {
		return
	}
	if len(e.Enemies) > 0 {
		return
	}
	e.emit(BattleEvent{Type: EventWaveClear, X: int64(e.WaveIndex)})
	e.record(e.Tick, ReplayWave, int64(e.WaveIndex), 1, 0)

	next := e.WaveIndex + 1
	// TotalEnemies > 0 是为了排除空关卡秒胜（见 BeginWave 里的说明）。
	if next >= len(e.Cfg.Level.Waves) || e.TotalEnemies <= 0 {
		e.Finish(e.TotalEnemies > 0, false)
		return
	}
	e.Phase = PhaseCardSelect
	e.beginCardSelect() // F 块实现：抽牌与 card_offer
}

// beginCardSelect 抽本波手牌（F 块实现）。
//
// B 阶段它刻意是空实现：`checkWaveEnd` 已经把 `Phase` 置为 `card_select`，
// 而 B 的验收场景是「跑到本波清空」—— 玩家是否真的拿到牌属于 F。
func (e *BattleEngine) beginCardSelect() {}
