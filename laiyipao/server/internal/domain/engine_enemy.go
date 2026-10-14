package domain

// 敌人推进与漏怪结算（R1 Block C）。
//
// 本块把 `engine.go` 里的 `updateEnemies` 从空实现换成真实实现，
// 并补上它的三个协作者：`leakDamage` / `leakDamageFor` /
// `defenseArmorPermille`，外加 D 块要用的 `withinEnemy`。
//
// # C 块消除了 B 块的两条已知差异
//
// B 块的 `blockBDivergences` 里记着 `x` 与 `spawn_progress` 两处
// 「TS 会推进、Go 侧静止」的差异 —— 因为 `StepEnemyMotion` 唯一的
// 调用点是 `updateEnemies`，而那时它是空实现。
// 本块接上之后，那两条差异被消除，`blockBDivergences` 必须清空。

// LeakAttackerDivisor 是「漏一只**有攻击力**的怪」扣多少血的分母。
//
// ⚠️ 必须与 `miniapp/src/game/engine.ts` 的 `LeakAttackerDivisor` 完全一致。
//
// 与 `LeakDamageDivisor`（无攻击力杂兵那档）并存的原因：
// 两种怪不能同一个价，否则要么远程怪白送（漏了几乎不掉血），
// 要么杂兵劝退（漏一只就劝退）。
const LeakAttackerDivisor = 3600

// BreachDamageFallback 是 `baseHpMax <= 0` 时漏怪伤害的兜底值。
//
// ⚠️ 100 而不是 0：零伤害意味着「漏怪不扣血」，
// 于是 `baseHp` 永远不归零、`leaked` 无限增长，
// 一关就能白刷无限个 leaked。零值在这里是**最坏**的兜底，不是最安全的。
const BreachDamageFallback = 100

// leak 渲染事件类型。
const EventLeak = "leak"

// updateEnemies 推进敌人。
//
// # ⚠️ 远程攻击的 `x > BaseX` 条件是**必需**的，不是优化
//
// 下面的「抵达防线」分支已经在处理 `x <= BaseX` 的敌人。
// 曾经两个 if 顺序执行、没有互斥：敌人越过防线后
// `dist = x - BaseX` 变成负数，`<= attackRange` 必然成立，
// 于是同一个 tick 里既远程打一次、又按抵达扣一次 ——
// **双倍伤害、leaked 计 2、发出两条 leak 事件**。
//
// 内容表里远程敌人（attack_range 150~600）大量存在，
// 每个抵达时都触发一次，防线血量被额外扣掉接近一倍。
//
// 这个 bug 的隐蔽之处：它**只影响远程怪**，近战怪完全正常，
// 而「近战怪的防线数值看起来对」是最容易让人停下来的地方。
func (e *BattleEngine) updateEnemies() {
	frozenAll := e.Heat.Overheated
	for _, en := range e.Enemies {
		if en.Dead {
			continue
		}
		e.StepEnemyMotion(en, frozenAll)

		// 远程敌人攻击
		if !en.Burrow && en.Attack > 0 && en.AttackRange > 0 && en.X > ToFixed(float64(BaseX)) {
			en.AttackCooldown -= TickMs
			if en.AttackCooldown <= 0 {
				en.AttackCooldown = en.AttackInterval
				dist := en.X - ToFixed(float64(BaseX))
				if dist <= ToFixed(float64(en.AttackRange)) {
					dmg := e.leakDamageFor(en)
					e.BaseHP -= dmg
					e.Leaked++
					e.emit(BattleEvent{Type: EventLeak, Damage: dmg})
					// ⚠️ `a` = 敌人 uid，`b` = 伤害值。
					//
					//	这里曾经传的是 `uidOf(dmg)` —— 而 `uidOf(v) = Number(v % 100000)`，
					//	实参是**伤害值**，于是 `a` 变成了「漏怪伤害 mod 100000」，
					//	与任何 uid 无关。`uidOf` 全仓只有一个调用点，
					//	函数名说「取 uid」而实参是伤害 —— 名字与实参对不上，
					//	说明写的时候想的是别的东西。
					//
					//	后果：事件不记录是哪只怪漏的，事后无法定位责任目标；
					//	伤害相差 100000 的两次漏怪在哈希里不可区分。
					e.record(e.Tick, ReplayLeak, en.UID, dmg, 0)
					if e.BaseHP <= 0 {
						e.BaseHP = 0
						e.Finish(false, false)
						return
					}
				}
			}
		}

		// 抵达防线
		if en.X <= ToFixed(float64(BaseX)) {
			en.Dead = true
			dmg := e.leakDamage(en)
			e.BaseHP -= dmg
			e.Leaked++
			e.emit(BattleEvent{Type: EventLeak, Damage: dmg})
			e.record(e.Tick, ReplayLeak, en.UID, dmg, 0)
			if e.BaseHP <= 0 {
				e.BaseHP = 0
				e.Finish(false, false)
				return
			}
		}
	}
	e.Enemies = filterAliveEnemies(e.Enemies)
}

// filterAliveEnemies 剔除已死敌人。
//
// ⚠️ 必须是**新切片**而不是原地压缩：
//
//	`e.Enemies` 的指针被 `leakDamageFor` 之类的地方持有不是问题，
//	但 `StepEnemyMotion` 的入参是 `*Enemy` ——
//	若原地 `append` 压缩会把存活元素前移，遍历中的指针仍然有效，
//	看似安全；然而下一次 `updateEnemies` 拿到的顺序就变了，
//	而顺序会影响「同一 tick 内谁先抵达、谁先触发 finish」。
//	新切片是「旧顺序保留、只做过滤」，这正是 TS `filter` 的语义。
func filterAliveEnemies(in []*Enemy) []*Enemy {
	out := in[:0]
	for _, en := range in {
		if !en.Dead {
			out = append(out, en)
		}
	}
	// ⚠️ 必须把尾部清零：`in[:0]` 与 `in` 共享底层数组，
	//   不清零会让被剔除的 `*Enemy` 指针仍可从切片底层数组读到。
	//   那不是内存安全问题（GC 仍然持有这些指针），
	//   但它意味着「从底层数组读出已死敌人」是可能的 ——
	//   而那正是「死了却还在挨打」这类 bug 的入口。
	for i := len(out); i < len(in); i++ {
		in[i] = nil
	}
	return out
}

// leakDamage 是 `leakDamageFor` 的转发。
//
// TS 里这两个是同一个函数（`leakDamage` 直接 return `leakDamageFor`），
// 保留转发是为了让「两条漏怪路径读起来一样」这件事在代码上可见 ——
// 它们将来若要分叉（比如抵达扣更多血），分叉点就在这里。
func (e *BattleEngine) leakDamage(en *Enemy) int64 {
	return e.leakDamageFor(en)
}

// leakDamageFor 算一只敌人漏进防线时的伤害。
//
// ⚠️ 两条路径**共用同一个数量级**，否则玩家每 50 标志承受的
//
//		「远程射程内扣血」与「抵达防线扣血」会差出一个数量级，
//		而两者之间没有足够的提示让玩家理解差异 ——
//		第 19 关远程怪直达防线，近战怪 50 标志左右也到防线，
//		玩家会觉得「同一关难度忽高忽低」。
//
//		具体：
//	  - 有攻击力（远程）：底血 × attack / LeakAttackerDivisor
//	  - 无攻击力（杂兵）：过 breachDamage，代价恒为底血的 1/LeakDamageDivisor
//
// ⚠️ 顺序固定为「先按攻击力缩放，再过护甲」。
//
//	交换顺序会让「无攻击力」分支也吃 `applyArmor`，
//	于是玩家装护甲后杂兵的漏怪伤害反而变成 0（乘 0 再乘护甲还是 0），
//	而远程怪的伤害会被护甲减两次 —— 两个错叠在一起。
func (e *BattleEngine) leakDamageFor(en *Enemy) int64 {
	armor := e.defenseArmorPermille()
	if en.Attack <= 0 {
		return applyArmor(e.breachDamage(), armor)
	}
	// 先乘后除，与 `mulDiv` 的语义一致。
	// 交换顺序会在 attack 很大时溢出 —— int64 乘 int64 上界远小于这里。
	scaled := e.BaseHPMax * en.Attack / LeakAttackerDivisor
	return applyArmor(scaled, armor)
}

// breachDamage 是「无攻击力杂兵」漏进防线的固定代价。
//
// ⚠️ 必须**只与关卡血量挂钩、与敌人血量无关**，
//
//	否则调敌人血量会连带把生存难度也改了，
//	两个变量纠缠在一起就没法单独调平衡。
//
// ⚠️ 除数越大越宽容 —— baseHp=1000、Divisor=10 时漏 10 只才归零，
//
//	给玩家「失误几次还能救」的空间，而不是「漏一只就结束」。
func (e *BattleEngine) breachDamage() int64 {
	if e.BaseHPMax > 0 {
		return e.BaseHPMax / LeakDamageDivisor
	}
	return BreachDamageFallback
}

// defenseArmorPermille 是「到达防线的有效护甲」。
//
// ⚠️ **不含** `Buffs.ArmorPermille` 的敌人侧加成 ——
//
//	卡牌「加固工事：防线护甲 +10%」的效果**完全生效**，
//	因为它的消费点就是这里的 `+ e.Buffs.ArmorPermille`。
//	（对照：`spawnEnemy` 里敌人自身的护甲**不加** `Buffs.ArmorPermille`，
//	那张卡曾经被加在敌人护甲上，效果完全反向 ——
//	玩家所有伤害打敌人时先被减 10%，而漏怪伤害一分不减免。）
//
// 三项相加后再封顶 `MaxArmorPermille`：
//
//	最坏：防线 250‰ + 砖 600‰ + 装备 250‰ = 1100‰。
//	`applyArmor` 会夹到 750‰，所以这是有意设计而不是 bug ——
//	护甲封顶是防线上不能被完全免疫。
func (e *BattleEngine) defenseArmorPermille() int64 {
	return e.BaseArmorPermille + e.Buffs.ArmorPermille + e.Cfg.Attacker.ArmorPermille
}

// withinEnemy 报告命中点 (x, y) 是否落在敌人身上。
//
// 飞行与钻地：只有溅射/穿透能命中 —— 所以半径按 `flyHeight` 放大。
//
// ⚠️ 用**平方距离**比较而不是开方：
//
//	`sqrt` 在不同语言/平台上对同一个输入可能给出不同的最后一位，
//	而边界判定 `dist <= r` 恰恰会在「正好等于 r」时把它放大成
//	「命中」与「不命中」的差别。平方比较消除了这个不确定性来源。
//
// ⚠️ 半径是 `toFixed(28 + flyHeight*0.1)` —— 浮点只在这一处出现，
//
//	取整后即转定点，之后全程整数运算。
func (e *BattleEngine) withinEnemy(en *Enemy, x, y int64) bool {
	r := 28 + float64(en.FlyHeight)*0.1
	rr := ToFixed(r)
	return dist2(en.X, en.Y, x, y) <= rr*rr
}

// dist2 是平方距离。
func dist2(x1, y1, x2, y2 int64) int64 {
	dx := x1 - x2
	dy := y1 - y2
	return dx*dx + dy*dy
}
