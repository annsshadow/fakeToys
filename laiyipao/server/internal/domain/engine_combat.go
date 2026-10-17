package domain

// 开火与弹道（R1 Block D）。
//
// 本块把 `updateAutoFire` / `fire` / `updateProjectiles` /
// `checkProjectileHit` / `hitEnemy` / `currentAttacker` /
// `triggerSteamBurstAoe` / `applyAoe` 从空实现换成真实实现。
//
// # 为什么 D 是里程碑
//
// A~C 只能证明「引擎的骨架、波次、敌人推进与漏怪与 TS 一致」。
// D 接上之后，一次**有构筑的完整战斗**从头跑到尾，
// 于是整局 `replay_hash` 第一次可以被服务端自行计算 ——
// 那是 R1 从「记录不符」变成「拒发不符」的前提。

// 弹道相关的渲染/重放事件类型。
const (
	EventFire     = "fire"
	EventHit      = "hit"
	EventKill     = "kill"
	EventReaction = "reaction"
	EventOverheat = "overheat"
	EventCardHeat = "card_heated"
)

// ProjectileSubSteps 是每 tick 的子步数。
//
// ⚠️ **不能改成 1**。
//
//	高速弹丸一个 tick 能跨过整只敌人，于是「线段穿过敌人」被采成
//
// 「起点在左、终点在右」—— 两个采样点都不在敌人身上，于是判定不命中。
//
//	表现为「高弹速技能偶尔完全打不中」，且与弹速正相关，
//	排查时会指向「弹丸速度计算错了」，而真正的原因是采样太稀。
//
//	子步是**运动学**层面的：它把一条直线切成两段再各判一次，
//	不改任何速度或伤害公式。
const ProjectileSubSteps = 2

// UpdateAutoFire 自动索敌开火。
//
// 玩家只做微调，不做逐发操作。
//
// # ⚠️ 热量检查放在索敌**之后**
//
//	先确认有目标再扣热量，否则一波末尾敌人还没刷完就会白白积热并过热，
//	玩家会看到「我没出手却过热了」这种无法理解的死法。
func (e *BattleEngine) updateAutoFire() {
	if e.Heat.Overheated {
		return
	}

	// ⚠️ 死区兜底：必须在遍历技能**之前**做一次。
	//
	// 死区怎么形成的：TryCast 允许热量正好到 cap（`heat + cost > cap` 才拒），
	// 而 CheckOverheat 要求 heat >= cap。于是当
	//
	//	cap - 最便宜技能热量 < heat < cap
	//
	// 时，没有任何技能能放（都超 cap），可 CheckOverheat 又不触发（没到 cap）。
	// heat 停在这个区间里出不来，战斗永久停摆。
	//
	// 实测后果（第 1 关、4 个技能 20/18/22/20 热量、seed 12345）：
	// 385 秒只开出 5 发、0 杀 10 漏、0 星 —— 游戏完全不可玩。
	//
	// 修法：进自动循环前，若「任何技能都放不出」就强制走一次过热。
	// 语义上也说得通 —— 热量已经高到连最便宜的技能都用不起了，
	// 这在 I-2 的框架里就是「过热」，玩家的正确反应本就是停手。
	//
	// ⚠️ 必须在 `pickTarget` **之前**：
	//   无目标的空场里推进热量到死区同样需要能被过热解救，
	//   否则玩家在等刷怪时也会卡死。
	if e.heatStuck() {
		e.enterOverheat(-1)
		return
	}

	for i := range e.Skills {
		s := &e.Skills[i]
		if s.Slot >= e.ActiveSlots {
			continue // 被动槽不主动释放
		}
		if s.CooldownRemaining > 0 {
			continue
		}
		if e.Heat.Heat+s.HeatCost > e.Heat.Cap() {
			continue
		}

		target := e.pickTarget(s)
		if target == nil {
			continue
		}

		// ⚠️ `TryCast` 返回 false 的分支**不可达**：
		//
		//	上面已 `if heat+cost > cap continue` 预过滤，
		//	其判据与 TryCast 内部拒绝条件逐字相同，
		//	且两行之间 pickTarget 不改热量。
		//	故走到这里 TryCast 必然成功（并原子地累加热量）。
		//	保留检查是「预检 + 原子提交」的双保险，
		//	但当前 `continue` 无法触达。
		if !e.Heat.TryCast(s.HeatCost) {
			continue
		}
		s.CooldownRemaining = s.CooldownMs
		e.fire(s, target)

		// ⚠️ 必须在开火**后**检查过热 ——
		//   热量打满的那一刻正是「用尽最后一点资源」的时刻。
		//   若在开火前检查，玩家会看到「我还没出手就过热了」。
		//
		//   这个调用一旦漏掉，热量会永久卡在上限、再也放不出技能
		//   （衰减虽在，但不会触发过热状态），战斗直接停摆 ——
		//   属于**静默失效**：不报错、不崩溃，只是再也不动了。
		if e.Heat.CheckOverheat() {
			e.enterOverheat(i)
			break // 一次性触发，避免同帧多个技能重复发事件
		}
	}
}

// enterOverheat 进入过热（或被护盾扛过）。
//
// 这里是「隔热护罩」（overheat_guard 机制卡）的**唯一**消费点。
func (e *BattleEngine) enterOverheat(skillIndex int) {
	if e.Buffs.OverheatGuard > 0 {
		e.Buffs.OverheatGuard--
		// 护盾生效：清空热量当作「扛过去了」，但不进入过热状态。
		//
		// ⚠️ 清空是必须的 —— 否则热量仍停在死区里，
		//
		//	下一 tick 又会触发兜底，形成「每 tick 消耗一层护盾」的空转。
		e.Heat.AbsorbOverheat()
		e.emit(BattleEvent{Type: EventCardHeat})
		e.record(e.Tick, ReplayGuard, 0, 0, 0)
		return
	}
	e.Heat.ForceOverheat()
	e.emit(BattleEvent{Type: EventOverheat})
	e.record(e.Tick, ReplayOverheat, int64(skillIndex), 0, 0)
}

// heatStuck 报告是否卡在「任何技能都放不出」的死区。
func (e *BattleEngine) heatStuck() bool {
	costs := make([]int64, 0, len(e.Skills))
	for _, s := range e.Skills {
		if s.Slot >= e.ActiveSlots {
			continue // 被动槽不消耗热量，不参与判据
		}
		costs = append(costs, s.HeatCost)
	}
	return e.Heat.IsStuckFor(costs)
}

// skillIndexOf 在技能列表里找下标，找不到返回 -1（仅用于事件记录）。
func (e *BattleEngine) skillIndexOf(s *EquippedSkill) int {
	for i := range e.Skills {
		if e.Skills[i].SkillID == s.SkillID {
			return i
		}
	}
	return -1
}

// pickTarget 索敌：BOSS/精英优先，其次最靠近防线的。
//
// 飞行/钻地敌人只能被溅射或穿透命中，所以优先安排给对应技能。
//
// ⚠️ `spawnProgress >= SpawnProgressFull` 这个过滤是必需的：
//
//	正在出场的怪还不该被打中 —— 它在动画里，位置每 tick 都变。
//	少了这一条，弹丸会追着一个还在滑入的目标跑，
//	而「追不上」的判定是「每 tick 走一步」，于是第一发常常打空。
//
// 归约用 `a.x <= b.x ? a : b`：平手时取**先出现**的那个，
// 顺序来自刷怪洗牌，所以是确定的。
func (e *BattleEngine) pickTarget(s *EquippedSkill) *Enemy {
	var live []*Enemy
	for _, en := range e.Enemies {
		if !en.Dead && en.SpawnProgress >= SpawnProgressFull {
			live = append(live, en)
		}
	}
	if len(live) == 0 {
		return nil
	}
	// 飞行/钻地：只有溅射或穿透能命中
	hasMulti := s.AoeRadius > 0 || s.Pierce > 0
	for _, en := range live {
		if !hasMulti || (en.FlyHeight <= 0 && !en.Burrow) {
			continue
		}
		// 第一个「可被多目标技能命中」的敌人即候选；
		// 归约与下面同式。
		best := en
		for _, other := range live {
			if other.FlyHeight <= 0 && !other.Burrow {
				continue
			}
			if other.X < best.X {
				best = other
			}
		}
		return best
	}
	// 其余取最靠近防线的
	best := live[0]
	for _, en := range live[1:] {
		if en.X < best.X {
			best = en
		}
	}
	return best
}

// fire 开一枪。
func (e *BattleEngine) fire(s *EquippedSkill, target *Enemy) {
	e.Shots++
	speed := s.ProjectileSpeed
	if speed == 0 {
		speed = 60000
	}
	dx := target.X - ToFixed(float64(BaseX))
	dy := target.Y - ToFixed(float64(BaseY))
	length := sqrtApprox(dx*dx + dy*dy)
	if length < 1 {
		length = 1
	}
	// 归一化方向（定点，保持确定性）。
	//
	// ⚠️ 必须**先乘后除**且用定点整除：
	//   用浮点归一化会在不同平台上给出不同的最后一位，
	//   而弹丸轨迹是逐 tick 累加的 —— 一个微小的初值差异
	//   会让「是否命中」在某些布局上翻转。
	nx := dx * speed / length
	ny := dy * speed / length

	e.Projectiles = append(e.Projectiles, &Projectile{
		UID:         e.uidSeq,
		SkillID:     s.SkillID,
		Element:     s.Element,
		X:           ToFixed(float64(BaseX)),
		Y:           ToFixed(float64(BaseY)),
		VX:          nx,
		VY:          ny,
		Damage:      s.BaseDamage,
		ApplyStacks: s.ApplyStacks,
		PierceLeft:  s.Pierce + e.Buffs.PierceBonus,
		ChainLeft:   s.Chain + e.Buffs.ChainBonus,
		HitSet:      map[int64]bool{},
		AoeRadius:   s.AoeRadius + int64(e.Buffs.AoeBonus),
		Dead:        false,
		TrailLen:    0,
		Slot:        s.Slot,
	})
	e.uidSeq++

	e.emit(BattleEvent{Type: EventFire, X: int64(s.Slot), Score: int64(s.SkillID)})
	e.record(e.Tick, ReplayFire, int64(s.SkillID), int64(s.Slot), 0)
}

// updateProjectiles 推进弹丸。
//
// ⚠️ `p.dead` 守卫的 true 分支**不可达**：
//
//	函数末尾会 `filter(!p.dead)`，故下一 tick 进入循环时数组里不含 dead 弹丸；
//	本 tick 内每颗只遍历一次。保留是防御未来新增调用点。
func (e *BattleEngine) updateProjectiles() {
	tickBig := int64(TickMs)
	maxX := ToFixed(float64(FieldW))
	maxY := ToFixed(float64(FieldH))
	for _, p := range e.Projectiles {
		if p.Dead {
			continue
		}
		for s := 0; s < ProjectileSubSteps; s++ {
			p.X += p.VX / tickBig
			p.Y += p.VY / tickBig
			p.TrailLen++

			// 越界（用定点边界）
			if p.X < 0 || p.X > maxX || p.Y < 0 || p.Y > maxY {
				p.Dead = true
				break
			}

			// 被崩塌前的掩体阻挡。
			//
			// ⚠️ 这里**必须把弹丸的伤害喂给掩体**，否则掩体永远打不破。
			//
			//	曾经的写法只是 `p.dead = true; break`，于是：
			//	  掩体挡住弹丸 -> 弹丸销毁 -> 走不到 checkProjectileHit
			//	  -> 掩体唯一的充能源（onHit，只在弹丸命中**敌人**时调用）永不触发
			//	  -> 掩体永不崩塌 -> 弹丸继续被挡     ← 循环依赖
			//
			//	实测后果：第 22/33/34 关**永远打不完**。
			//	弹丸轨迹实测显示，从 (60,880) 射向 (884,638) 的弹丸在 (497,752) 消失，
			//	而那里距掩体 (514,654) 恰好 99.5 < 100 —— 被挡。
			//	命中率掉到 1%，12000 tick 只杀 3 只怪，引擎永久停在 wave 阶段。
			//
			// 现在弹丸打在掩体上等同于打在敌人身上（有伤害、有元素），
			// 于是「用动能砸开掩体、打开弹道」这个设计意图才真正成立。
			//
			// 被崩塌前的掩体阻挡。
			//
			// ⚠️ 这里**必须把弹丸的伤害喂给掩体**，否则掩体永远打不破。
			//
			//	曾经的写法只是 `p.dead = true; break`，于是：
			//	  掩体挡住弹丸 -> 弹丸销毁 -> 走不到 checkProjectileHit
			//	  -> 掩体唯一的充能源（onHit，只在弹丸命中**敌人**时调用）永不触发
			//	  -> 掩体永不崩塌 -> 弹丸继续被挡     <- 循环依赖
			//
			//	实测后果：第 22/33/34 关**永远打不完**。弹丸轨迹实测显示，
			//	从 (60,880) 射向 (884,638) 的弹丸在 (497,752) 消失，
			//	而那里距掩体 (514,654) 恰好 99.5 < 100 —— 被挡。
			//	命中率掉到 1%，12000 tick 只杀 3 只怪，引擎永久停在 wave 阶段。
			//
			// 现在弹丸打在掩体上等同于打在敌人身上（有伤害、有元素），
			// 于是「用动能砸开掩体、打开弹道」这个设计意图才真正成立。
			for _, t := range e.Terrains {
				if !t.BlocksProjectile() || !e.withinTerrain(t.X, t.Y, 100, p.X, p.Y) {
					continue
				}
				if t.OnHit(string(p.Element), p.Damage, p.X, p.Y) {
					e.recordTerrainTrigger(t)
				}
				p.Dead = true
				break
			}
			if p.Dead {
				break
			}
			e.checkProjectileHit(p)
			if p.Dead {
				break
			}
		}
	}
	e.Projectiles = filterAliveProjectiles(e.Projectiles)
}

// filterAliveProjectiles 剔除已死弹丸。
//
// ⚠️ 尾部必须置 nil：`in[:0]` 与 `in` 共享底层数组，
//
//	不清零会让已死弹丸仍能从底层数组读到 ——
//	而那正是「死弹丸还在命中」这类 bug 的入口。
func filterAliveProjectiles(in []*Projectile) []*Projectile {
	out := in[:0]
	for _, p := range in {
		if !p.Dead {
			out = append(out, p)
		}
	}
	for i := len(out); i < len(in); i++ {
		in[i] = nil
	}
	return out
}

// checkProjectileHit 判定弹丸命中。
func (e *BattleEngine) checkProjectileHit(p *Projectile) {
	for _, en := range e.Enemies {
		if en.Dead || en.SpawnProgress < SpawnProgressFull {
			continue
		}
		if p.HitSet[en.UID] {
			continue
		}
		if !e.hitEnemy(p, en, false) {
			continue
		}

		e.Hits++
		p.HitSet[en.UID] = true

		// 溅射
		//
		// ⚠️ `e.Hits` 只在**主目标**这里自增，applyAoe / 弹射内部**不**加。
		//
		//	hits 的语义是「有多少次开火命中了东西」，不是「总共造成了几次伤害结算」。
		//	理由是一条跨端强约束：服务端 ValidateSettle 拒 `in.Hits > in.Shots`
		//	（ErrInvalidHitRate），engine.test.ts 也有 hits <= shots。
		//	一发带 aoe/chain 的弹丸本就能命中多个敌人，
		//	每次结算都 hits++ 会让高 chain + 高 pierce 的构筑
		//	稳定产出 hits > shots —— 那是真的 422。
		if p.AoeRadius > 0 {
			e.applyAoe(p, en, p.AoeRadius)
		}

		// 弹射
		if p.ChainLeft > 0 {
			next := e.nearestUnhit(p)
			if next != nil {
				p.ChainLeft--
				p.X = next.X
				p.Y = next.Y
				// ⚠️ 弹射目标必须**当场结算伤害**。
				//
				//	原来只 `p.hitSet.add(next.uid); continue`：
				//	continue 只是数组游标前进，不会回头结算 next；
				//	而 next 已被写进 hitSet，下一轮开头的 hitSet.has 会跳过它。
				//	于是弹射只消耗 chainLeft、只位移弹丸，目标一点伤害都吃不到 ——
				//	连锁闪电 / 电弧弹这一整类多目标技能的定位完全失效。
				//
				//	结算顺序：先结算、后写 hitSet（反过来会重复结算）。
				if e.hitEnemy(p, next, true) {
					p.HitSet[next.UID] = true
				}
				continue
			}
		}

		// 穿透
		if p.PierceLeft > 0 {
			p.PierceLeft--
			continue
		}
		p.Dead = true
		return
	}
}

// nearestUnhit 找离弹丸最近的、尚未被这发弹丸命中的活着的敌人。
func (e *BattleEngine) nearestUnhit(p *Projectile) *Enemy {
	var best *Enemy
	var bestD int64
	for _, x := range e.Enemies {
		if x.Dead || p.HitSet[x.UID] {
			continue
		}
		d := dist2(p.X, p.Y, x.X, x.Y)
		if best == nil || d < bestD {
			best, bestD = x, d
		}
	}
	return best
}

// hitEnemy 对单个敌人结算一次命中。
//
// skipRadius 用于**已经确定要命中**的路径：溅射（applyAoe 已按 AoeRadius
// 选好目标）与弹射（弹丸直接跳到目标身上，距离恒为 0）。
//
// # ⚠️ 为什么必须显式跳过
//
// 原实现无条件执行 withinEnemy，而 withinEnemy 的半径是
// `toFixed(28 + flyHeight*0.1)` —— 那是**单体命中半径**，不是溅射半径。
// 于是 applyAoe 按 AoeRadius 正确选出的目标，紧接着被这 28 单位再裁一次：
// 声明的 aoe_radius 被当成 28，所有溅射技能实质失效
// （只有彼此贴在一起、距命中点不到 28 单位的敌人才会被顺带打到）。
func (e *BattleEngine) hitEnemy(p *Projectile, en *Enemy, skipRadius bool) bool {
	if !skipRadius && !e.withinEnemy(en, p.X, p.Y) {
		return false
	}

	// 把敌人的运行时状态包成 Defender，复用与 Go 完全一致的结算。
	// 削甲生效期（armor_break）内折进有效护甲：削到 0 为止，不为负。
	effArmor := en.ArmorPermille
	if en.ArmorShredMs > 0 && en.ArmorPermille > en.ArmorShredPermille {
		effArmor = en.ArmorPermille - en.ArmorShredPermille
	}
	def := NewDefender(en.HP, en.Shield, effArmor)
	for _, el := range AllElements() {
		def.ResistPermille[el] = en.Resist[el]
		def.ElementStacks[el] = en.Stacks[el]
	}
	// 受击伤害放大（超导/急速冻结）
	//
	// ⚠️ 用 `resist - amplify/10` 而不是「重新按放大比例算」——
	//   抗性是千分比，放大 10‰ 就是抗性减 100‰。
	//   写成「抗性 × (1 - amplify/1000)」会得到一个小数，
	//   而抗性在这个系统里是整数千分比。
	if en.AmplifyPermille > 0 {
		for _, el := range AllElements() {
			def.ResistPermille[el] -= en.AmplifyPermille / 10
		}
	}

	att := e.currentAttacker()
	in := HitInput{
		SkillDamage:  p.Damage,
		SkillElement: p.Element,
		ApplyStacks:  p.ApplyStacks,
		Roll:         e.Rng.Roll(),
	}
	res := ResolveHit(att, &def, in)
	// 施加元素必须在伤害结算之后，否则会自我触发反应
	ApplyElementStacks(att, &def, in)

	// 写回敌人状态
	en.HP = def.HP
	en.Shield = def.Shield
	en.Stacks = def.ElementStacks
	en.HitFlashMs = 140

	// ⚠️ `amplifyPct` 与 `statusDurationMs` 是**两件独立的事**，
	//
	//	原来被一个 `if res.statusDurationMs > 0` 捆在一起。
	//	而 `amplifyPct` 的生效被 `statusDurationMs > 0` 把门：
	//	新增一条 amplify_pct > 0 且 status_duration_ms = 0 的反应，
	//	受击增伤**永远不会生效**，而代码看起来完全正常。
	//
	// 为什么这不可能被测出来：当前 7 条反应里没有任何一条落在这个组合
	//（superconduct 600/4000、flash_freeze 300/2000，其余全 0）。
	//	所以判据「amplify 生效」在今天与「不生效」观察不到差别 ——
	//	它是一个**只有在改动之后才会显形**的洞。
	//
	// ⚠️ 查表本身**必须**留在 `res.Reaction` 非空的守卫里：
	//   无反应时 `res.Reaction` 是**空串**，`Spec()` 返回零值，
	//   直接读零值的字段不会崩（这与 TS 的 `undefined` 不同），
	//   但 `amplifyPct == 0` / `statusDurationMs == 0` 恰好让所有分支不进 ——
	//   也就是说**两边都靠零值短路**，但只有 TS 那边会崩。
	//   Go 侧必须显式判空，否则「无反应」与「反应字段全 0」变得不可区分。
	if res.Reaction != "" {
		applyReactionStatus(en, res)
	}
	dmgScore := DamageScore(e.Cfg.ScoreRules, res.TotalDamage)
	e.Score += dmgScore
	e.emit(BattleEvent{Type: EventHit, X: p.X, Y: p.Y,
		Damage: res.TotalDamage, Score: dmgScore})
	e.record(e.Tick, ReplayHit, en.UID, dmgScore, boolInt(res.Crit))

	// 元素使用统计
	if p.Element != "" {
		e.ElementsUsed[string(p.Element)]++
	}

	// 反应统计
	if res.Reaction != "" {
		e.ReactionsCount++
		e.ReactionsUsed[string(res.Reaction)]++
		resisted := res.ResistAppliedPermille < permille
		spec := res.Reaction.Spec()
		e.emit(BattleEvent{Type: EventReaction, X: en.X, Y: en.Y,
			Damage: res.ReactionDamage, Text: string(res.Reaction)})
		e.record(e.Tick, ReplayReaction, en.UID, ReactionIndex(res.Reaction), 0)
		_ = resisted
		_ = spec
	}

	// ⚠️ steam_burst 的溅射范围伤害。
	//
	// 口径（与 `reaction_specs.json` 服务端 canonical 逐行一致）：
	//   - 伤害 = reactionDamage >> 2（定点，25%）
	//   - 源体不���（自身已结算）
	//   - 半径 = 反应表 AoeRadius（像素，×1000 转定点）
	//   - 邻居伤害走 resolveHit 复用护甲/护盾/抗性，但**不再触发反应**
	//     —— 否则 steam_burst 会在溅射目标上**再次**爆开，形成指数级连锁。
	if res.Reaction == "steam_burst" && res.ReactionDamage > 0 {
		if spec, ok := LookupReactionSpec(res.Reaction); ok && spec.AoeRadius > 0 {
			e.triggerSteamBurstAoe(en, res.ReactionDamage, att)
		}
	}

	// 地形联动。地形自身负责判定弹丸是否经过它（updateTerrain 内部已做空间判定），
	// 因此这里只需传入元素与伤害。
	//
	// ⚠️ 必须传**命中位置**：地形要判断「这一发是否真的打在地形上」，
	// 否则站在远处的油桶会被任意位置的火焰点燃（见 Terrain.OnHit 注释）。
	e.terrainOnHit(p.Element, res.TotalDamage, p.X, p.Y)

	if res.Killed {
		e.killEnemy(en)
		// 蓄能塔充能：满则给全场敌人上该敌人身上的主元素。
		//
		// ⚠️ 必须**在** killEnemy 之后调：killEnemy 会把 en.Dead 置真，
		// 而 terrainOnKill 需要用它的元素层数算主导元素。
		// 顺序反了的话主元素会算成「零层时的默认值」。
		e.terrainOnKill(en, e.currentAttacker().ElementCap)
	}
	return true
}

// killEnemy 结算一次击杀。
func (e *BattleEngine) killEnemy(en *Enemy) {
	en.Dead = true
	e.Kills++
	e.Score += KillScore(e.Cfg.ScoreRules, en.IsBoss)
	e.emit(BattleEvent{Type: EventKill, X: en.X, Y: en.Y, Text: boolStr(en.IsBoss)})
	boss := int64(0)
	if en.IsBoss {
		boss = 1
	}
	e.record(e.Tick, ReplayKill, en.UID, boss, 0)
}

// applyAoe 溅射到半径内的其他敌人。
//
// ⚠️ 溅射目标**必须**记进 hitSet。原实现不记，于是「穿透 + 溅射」组合下
// 同一敌人会被反复结算：每穿透一个主目标就再跑一次 applyAoe。
// 内容表里「电磁栅栏」Pierce 12 / AoeRadius 50 就是这个组合。
func (e *BattleEngine) applyAoe(p *Projectile, origin *Enemy, radius int64) {
	rr := ToFixed(float64(radius))
	for _, en := range e.Enemies {
		if en.Dead || en.UID == origin.UID {
			continue
		}
		if p.HitSet[en.UID] {
			continue
		}
		if dist2(origin.X, origin.Y, en.X, en.Y) > rr*rr {
			continue
		}
		if e.hitEnemy(p, en, true) {
			p.HitSet[en.UID] = true
		}
	}
}

// triggerSteamBurstAoe steam_burst 的溅射范围伤害。
//
// 对「已命中触发 steam_burst 的敌人」周围 AoeRadius 内、存活的其他敌人，
// 造成 reactionDamage >> 2（25%）的二次伤害。
//
// # ⚠️ 二次伤害为何不用 `hitEnemy`
//
// `hitEnemy` 是投射体命中专用：它从弹丸里读伤害/元素/滚点，
// 并在结算后**施加元素层数**。溅射如果复用它会：
//   - 把技能元素再次施加到邻居身上（触发第二次 steam_burst / overheat）
//   - 消耗 `e.Rng.Roll()`（RNG 顺序偏移 -> 整局事件流改动 -> 哈希变化）
//
// 所以这里只复用 `resolveHit` 的**装配 + 结算**部分（护甲/护盾/抗性），
// 不施加元素、不掷骰、不触发反应 ——
// 二次伤害纯粹是「命中减去护甲/护盾」。
func (e *BattleEngine) triggerSteamBurstAoe(origin *Enemy, reactionDmg int64, att Attacker) {
	rr := ToFixed(float64(steamBurstAoeRadius()))
	splash := reactionDmg >> 2
	if splash <= 0 {
		return
	}
	for _, en := range e.Enemies {
		if en.Dead || en.UID == origin.UID {
			continue
		}
		if dist2(origin.X, origin.Y, en.X, en.Y) > rr*rr {
			continue
		}

		effArmor := en.ArmorPermille
		if en.ArmorShredMs > 0 && en.ArmorPermille > en.ArmorShredPermille {
			effArmor = en.ArmorPermille - en.ArmorShredPermille
		}
		def := NewDefender(en.HP, en.Shield, effArmor)
		for _, el := range AllElements() {
			def.ResistPermille[el] = en.Resist[el]
			def.ElementStacks[el] = en.Stacks[el]
		}

		// 二次伤害：复用 direct+damage 结算，不触发反应
		//（SkillElement 为空 → 不会配对出新反应）。
		//
		// ⚠️ Roll 固定 9999：暴击作用于 direct+element 两段，
		//   而溅射本身只有 direct 段，所以这个值不影响伤害；
		//   固定它是**为了不消耗 RNG** —— 若这里掷一次骰子，
		//   后续所有 `e.Rng.Roll()` 的取值都会偏移一格。
		res := ResolveHit(att, &def, HitInput{
			SkillDamage:  splash,
			SkillElement: "",
			Roll:         9999,
		})
		en.HP = def.HP
		en.Shield = def.Shield
		en.Stacks = def.ElementStacks

		if res.TotalDamage > 0 {
			dmgScore := DamageScore(e.Cfg.ScoreRules, res.TotalDamage)
			e.Score += dmgScore
			e.emit(BattleEvent{Type: EventHit, X: en.X, Y: en.Y, Damage: res.TotalDamage})
			e.record(e.Tick, ReplayHit, en.UID, dmgScore, 0)
		}
		if res.Killed {
			e.killEnemy(en)
			// 蓄能塔充能：满则给全场敌人上该敌人身上的主元素。
			//
			// ⚠️ 必须**在** killEnemy 之后调，且用 `att` 而不是
			//   `currentAttacker()`：killEnemy 会把 en.Dead 置真，
			//   而 terrainOnKill 需要用它的元素层数算主导元素；
			//   cap 则沿用触发本次溅射的那次攻击的攻方，
			//   与 TS `t.onKillCharging(dom, att.elementCap, ...)` 同源。
			//
			// ⚠️ 这条是 E 块**漏掉过一次**的接线点：
			//   最初只在 `hitEnemy` 里加了 terrainOnKill，
			//   而 steam_burst 的溅射击杀是**另一条**独立的击杀路径。
			//   症状极隐蔽：`kills` / `score` / 事件流全对，
			//   只有蓄能塔的 `charge` 少 1 —— 而 `charge` 进 `replay_hash`。
			e.terrainOnKill(en, att.ElementCap)
		}
	}
}

// steamBurstAoeRadius 读 steam_burst 的溅射半径。
//
// ⚠️ 单独抽出来是为了让「半径来自反应表」这件事在一处可见：
//
//	将来若改成按等级缩放，改这里就行。
func steamBurstAoeRadius() int64 {
	spec, ok := LookupReactionSpec("steam_burst")
	if !ok {
		return 0
	}
	return int64(spec.AoeRadius)
}

// currentAttacker 算本 tick 的攻方。
//
// # ⚠️ 关键：返回值参与重放哈希，**多加一次就哈希不符**
//
// 三项**原样透传**、不与局内 Buffs 相加，因为它们的消费者不在伤害公式里：
//
//	armorPermille    -> defenseArmorPermille()（防线护甲，不走 currentAttacker）
//	heatCapPermille  -> 构造器注入 heat.capBonus
//	mechanicPermille -> applyMechanic 放大卡面数值
//
// 在这里相加会让它们**既被计入攻方又被单独消费**一次。
func (e *BattleEngine) currentAttacker() Attacker {
	base := e.Cfg.Attacker
	lv := e.Cfg.Level

	// 元素上限：关卡给就用关卡的，否则用攻方的；再加上局内加成。
	elementCap := base.ElementCap
	if lv.ElementCap != 0 {
		elementCap = lv.ElementCap
	}

	reactionTier := int64(1)
	if lv.MaxReactionTier != 0 {
		reactionTier = lv.MaxReactionTier
	}

	return Attacker{
		Attack: base.Attack + e.Buffs.AttackPermille,
		// ⚠️ 这里必须夹一层 1000‰。
		//
		//	消费侧的判据是 `roll >= PERMILLE - critPermille`。
		//	当 critPermille > 1000 时右边变成**负数**，而 roll 是 0..999 ——
		//	于是判据恒真，**每一发都暴击**。
		//
		//	为什么能超过 1000：局内卡 applyAttribute 的 crit 卡无夹取，
		//	最难关卡 10 波、每波恰好 1 张属性卡，6 张 crit 卡就
		//	550 + 480 = 1030 > 1000。
		//	概率不高（每波 1/6，最多 10 波，P(>=6) 约 0.2%），
		//	但这是**确定性**的：种子给定后必然如此。
		//
		//	为什么是夹而不是「报错」：玩家无法选择抽到哪张卡，
		//	卡池由种子决定。夹取是唯一能把越界值变成合法值的地方。
		//
		//	为什么只夹 crit，不夹 attack / elementCoefPermille：
		//	那两者是**线性**消费，越界只是数值变大，没有行为**悬崖**。
		//	crit 是唯一有悬崖的：`permille - critPermille` 会变号。
		CritPermille:           minInt64(base.CritPermille+e.Buffs.CritPermille, permille),
		CritMultiplierPermille: base.CritMultiplierPermille,
		ReactionMultPermille:   base.ReactionMultPermille,
		ElementCap:             elementCap + e.Buffs.ElementCapBonus,
		ReactionTier:           reactionTier,
		ElementCoefPermille:    base.ElementCoefPermille + e.Buffs.ElementCoefPermille,
		// ↓ 这三项原样透传，见函数注释
		HeatCapPermille:  base.HeatCapPermille,
		ArmorPermille:    base.ArmorPermille,
		MechanicPermille: base.MechanicPermille,
	}
}

// sqrtApprox 整数平方根（牛顿法，避免浮点的不确定性）。
//
// ⚠️ 必须与 TS 的实现**逐步同构**：
//
//	这不是「数学上等价就行」的地方 ——
//	牛顿法的迭代次数取决于初值，而初值是 n 本身。
//	换成另一种开方算法（哪怕结果完全相同），
//	在 `len` 只差 1 的边界上就会给出不同的弹道方向。
func sqrtApprox(n int64) int64 {
	if n < 2 {
		return n
	}
	x := n
	y := (x + 1) / 2
	for y < x {
		x = y
		y = (x + n/x) / 2
	}
	return x
}

// DamageScore 伤害换算成分。
//
// ⚠️ `totalDamage <= 0` 时返回 0，而不是让 `damage / unit` 给出负分。
func DamageScore(rules ScoreRules, totalDamage int64) int64 {
	if totalDamage <= 0 {
		return 0
	}
	if rules.PerDamageUnit == 0 {
		return 0
	}
	return totalDamage / rules.PerDamageUnit
}

// KillScore 击杀分（BOSS 与普通怪不同）。
func KillScore(rules ScoreRules, isBoss bool) int64 {
	if isBoss {
		return rules.OnKillBoss
	}
	return rules.OnKillNormal
}

// ReactionIndex 反应在重放里的编号（从 1 起）。
//
// ⚠️ 从 1 起而不是 0：`0` 已经被 `hit` 事件的 crit 标志占用了，
//
//	两类事件共用 `a` 之外的位，而编号 0 与「无标记」不可区分。
func ReactionIndex(k ReactionKey) int64 {
	for i, r := range AllReactions() {
		if r == k {
			return int64(i + 1)
		}
	}
	return 0
}

func boolInt(b bool) int64 {
	if b {
		return 1
	}
	return 0
}

func boolStr(b bool) string {
	if b {
		return "boss"
	}
	return ""
}

// applyReactionStatus 把一次反应的效果写进敌人的运行时状态。
//
// # ⚠️⚠️ 为什么必须是**独立可测的函数**
//
// 这段逻辑的历史 bug 是：`amplify_pct`（受击伤害放大）的生效
// 被一个 `if statusDurationMs > 0` 把门 ——
// 于是新增一条「只有增伤、没有状态」的反应时，受击增伤**永远不会生效**，
// 而代码看起来完全正常。
//
// 为什么跨端对齐抓不到：**当前 7 条反应里没有任何一条落在这个组合**
//
//	amplify_pct > 0 的两条（superconduct 600、flash_freeze 300）
//	它们的 status_duration_ms 分别是 4000 / 2000（都 > 0）
//	status_duration_ms == 0 的其余五条 amplify_pct 全是 0
//
// 所以判据「amplify 生效」在今天与「不生效」观察不到差别。
// 实测把 `&& res.StatusDurationMs > 0` 加回去，五个跨端场景全绿。
//
// 这与第 80/85 轮记的同一个陷阱：
// **只要输入落不到分界线上，关于分界线的断言都是空的。**
//
// # 抽成函数就是为了打破这个循环
//
// 现在可以直接喂一个**构造出来的** `ReactionSpec{AmplifyPct: 300, StatusDurationMs: 0}`
// 进来，验「有增伤、无状态时增伤仍然生效」。
// 而不必等到「产品真的加了那条反应」那天才发现代码是错的。
//
// 代价是这个函数的参数是 `HitResult` 而不是从反应表里取 ——
// 那正是可测性的来源，也是它**不应该**藏进 `hitEnemy` 的理由。
func applyReactionStatus(en *Enemy, res HitResult) {
	if res.Reaction == "" {
		// 无反应：必须显式早退。
		//
		// ⚠️ TS 侧靠 `REACTIONS['']` 是 undefined 来挡住后续读字段；
		//   Go 侧 `Spec()` 返回**零值**，读它不会崩 ——
		//   于是「无反应」与「反应字段全 0」在 Go 里长得一样。
		//   这里显式判空，是为了保住那层「无反应就不查表」的语义。
		return
	}
	spec := res.Reaction.Spec()

	// ⚠️⚠️ 这一行**不能**加 `&& res.StatusDurationMs > 0`。
	//   增伤与状态时长是两件独立的事，各有自己的判据。
	if spec.AmplifyPct > 0 {
		en.AmplifyPermille = int64(spec.AmplifyPct)
	}

	if res.StatusDurationMs <= 0 {
		return
	}
	switch res.Reaction {
	case "flash_freeze", "superconduct":
		en.FrozenMs = int64(spec.StatusDurationMs)
	case "overheat":
		en.StunnedMs = int64(spec.StatusDurationMs)
	case "armor_break":
		if spec.ArmorShredPermille > 0 {
			// 削甲：StatusDurationMs 内目标护甲被削 spec.ArmorShredPermille 千分比
			//（hitEnemy 构建 Defender 时折进有效护甲）；
			// 击退：一次性右推（stepEnemyMotion 的 knockback 分支消费后清零）。
			en.ArmorShredMs = int64(spec.StatusDurationMs)
			en.ArmorShredPermille = int64(spec.ArmorShredPermille)
			en.Knockback += int64(spec.Knockback)
		}
	}
}
