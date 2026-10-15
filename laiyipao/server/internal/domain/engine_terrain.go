package domain

// 地形接线（R1 Block E）。
//
// 本块把 `updateTerrain` 从空实现换成真实实现，
// 并把地形接进 D 块刻意留空的三处：
// 弹丸被掩体阻挡、`hitEnemy` 的地形联动、击杀时的蓄能塔充能。
//
// # 为什么 D 阶段刻意不接
//
// TS 的 `updateProjectiles` 里有「弹丸被掩体挡住」的判定。
// Go 侧 D 阶段做了一半的话，后果是「地形记了触发事件却从不结算效果」——
// 表现为「玩家看到油桶被点燃但没有任何作用」。
// 不做则是「地形完全不参与」，症状明确、可定位。
//
// 半接比不接更糟，因为它的症状落在**数值**上（分数、击杀、漏怪），
// 而根因在**接线**上。

// EventTerrain 是地形触发的渲染事件类型。
const EventTerrain = "terrain"

// terrainOrder 是地形类型在重放事件里的编号顺序。
//
// ⚠️ 顺序**必须**与 TS 的 `terrainIndex` 逐项一致。
// 改成 map 遍历会让编号随机 —— 而编号进 `replay_hash`。
var terrainOrder = [...]string{
	TerrainOilDrum,
	TerrainTidalGate,
	TerrainRotorVane,
	TerrainCollapseWall,
	TerrainChargeTower,
}

// TerrainIndex 地形类型的重放编号（从 1 起）。
//
// 未在表内的类型返回 0 —— 与 `ReactionIndex` 同约定：
// 0 表示「没有这条记录」，可与「编号就是 0」区分开。
func TerrainIndex(kind string) int64 {
	for i, k := range terrainOrder {
		if k == kind {
			return int64(i + 1)
		}
	}
	return 0
}

// updateTerrain 推进全部地形。
func (e *BattleEngine) updateTerrain() {
	if len(e.Terrains) == 0 {
		return
	}
	ctx := terrainCtx{e: e}
	for _, t := range e.Terrains {
		eff := t.Update(TickMs, ctx)
		if !eff.Blocked {
			continue
		}
		// 潮汐闸关闭：低层敌人无法通过，被挡在闸门右侧。
		//
		// ⚠️ `!e.burrow` 是必需的：钻地敌人能从关闭的闸门下面过去，
		//   这是「钻地」这个机制存在的全部理由。
		//
		// ⚠️ 高度判定用 `|y - gate.y| < 60`（逻辑单位）：
		//   闸门只挡住**贴着它那一行**的敌人。
		//   写成 `<=` 会多挡一行；写成 60 以外的值会让闸门的实际
		//   作用高度与内容表描述不符。
		gateY := ToFixed(float64(t.Y))
		gateX := ToFixed(float64(t.X))
		for _, en := range e.Enemies {
			if en.Burrow {
				continue
			}
			d := en.Y - gateY
			if d < 0 {
				d = -d
			}
			// ⚠️ 阈值是 `60 * 1000`：TS 那边是
			//   `Math.abs(Number(e.y - toFixed(t.y)) / 1000) < 60`，
			//   也就是「先减、再除以 1000 还原成逻辑单位、比较 60」。
			//   这里改成「比较定点 60000」是**同一件事**，但避免了
			//   除法带来的截断边界歧义（`|d|/1000 < 60` 与
			//   `|d| < 60000` 在整数上完全等价）。
			if d >= 60*1000 {
				continue
			}
			if en.X < gateX {
				en.X = gateX
			}
		}
	}
}

// terrainOnHit 通知全部地形「这一发打在 (x,y) 上」。
//
// 返回是否至少有一个地形被触发。
//
// ⚠️ 必须传**命中位置**：地形要判断「这一发是否真的打在地形上」，
//
//	否则站在远处的油桶会被任意位置的火焰点燃
//	（见 `Terrain.OnHit` 注释）。
func (e *BattleEngine) terrainOnHit(element Element, damage, hitX, hitY int64) bool {
	any := false
	for _, t := range e.Terrains {
		if !t.OnHit(string(element), damage, hitX, hitY) {
			continue
		}
		e.recordTerrainTrigger(t)
		any = true
	}
	return any
}

// terrainOnKill 通知全部地形「这只敌人死了」。
func (e *BattleEngine) terrainOnKill(en *Enemy, cap int64) {
	dom := dominantElementOfEnemy(en)
	if dom == "" {
		dom = ElementFire
	}
	for _, t := range e.Terrains {
		if !t.OnKillCharging(string(dom), cap, terrainCtx{e: e}) {
			continue
		}
		// ⚠️ 这里**不**加 `if !includes` 守卫：
		//
		//	`terrainUsed` 是「本局用到过哪些地形」的**去重列表**，
		//	重复项会让服务端 `ValidateSettle` 的
		//	`len(terrain_used) <= len(distinct)` 判定失败 ——
		//	症状是「玩家打了 100 次油桶，结算直接 422」。
		//
		//	对比：`onTerrainTrigger`（弹丸/命中触发那条路径）允许重复，
		//		因为 TS 那边也是 push 不去重。
		//		两条路径的语义不同，不能互相照抄。
		e.markTerrainUsed(t)
	}
}

// recordTerrainTrigger 记一次地形触发（**不去重**）。
func (e *BattleEngine) recordTerrainTrigger(t *Terrain) {
	e.TerrainUsed = append(e.TerrainUsed, t.Kind)
	e.emit(BattleEvent{Type: EventTerrain, X: int64(t.X), Y: int64(t.Y), Text: t.Kind})
	e.record(e.Tick, ReplayTerrain, TerrainIndex(t.Kind), int64(t.X), int64(t.Y))
}

// markTerrainUsed 记一次地形「被用到」（**去重**）。
//
// ⚠️ 与 `recordTerrainTrigger` 的区别就是去重。
// 两条路径的调用点不同：
//   - trigger 路径（弹丸挡住 / 命中点燃）：每次触发都记重放事件，
//     `terrainUsed` 也每次追加（与 TS 同）
//   - used 路径（击杀充能）：只在首次记入
func (e *BattleEngine) markTerrainUsed(t *Terrain) {
	for _, k := range e.TerrainUsed {
		if k == t.Kind {
			return
		}
	}
	e.TerrainUsed = append(e.TerrainUsed, t.Kind)
	e.emit(BattleEvent{Type: EventTerrain, X: int64(t.X), Y: int64(t.Y), Text: t.Kind})
	e.record(e.Tick, ReplayTerrain, TerrainIndex(t.Kind), int64(t.X), int64(t.Y))
}

// withinTerrain 报告定点坐标 (x,y) 是否落在逻辑坐标 (tx,ty) 的 r 半径内。
func (e *BattleEngine) withinTerrain(tx, ty, r int, x, y int64) bool {
	return dist2(ToFixed(float64(tx)), ToFixed(float64(ty)), x, y) <=
		ToFixed(float64(r))*ToFixed(float64(r))
}

// --- TerrainCtx 的引擎侧实现 ---

// terrainCtx 让 `*BattleEngine` 满足 `TerrainCtx`。
//
// ⚠️ 与 `terrainEnemy` 同理：`BattleEngine` 已有同名的 `Enemies“n//   与 `Projectiles` **字段**，Go 不允许同名字段与方法共存。
//
//	包一层而不是改字段名：字段名与 TS 的 `BattleEngine` 一一对应，
//	而跨端对齐逐字段比对依赖这些名字保持可读。
type terrainCtx struct {
	e *BattleEngine
}

// TerrainTickDamage 是火区每 tick 伤害（减伤前）。
//
// ⚠️ 值 = `ElementPerStackBase / 8`，与 TS 的
//
//	`terrainTick: ELEMENT_PER_STACK_BASE / 8n`
//
// 逐字对应。写成常量会与 `ElementPerStackBase` 的调参脱钩 ——
// 那个常量是设计常量，跨端由 `consistency_test.go` 锁定。
func (c terrainCtx) TerrainTickDamage() int64 { return ElementPerStackBase / 8 }

// ElementCap 是当前元素层数上限。
//
// ⚠️ 必须来自 `currentAttacker().elementCap` 而不是常量：
//
//	它是关卡 `element_cap` + 局内 `elementCapBonus` 的合成值，
//	玩家还能靠「元素容器」卡抬高。
//	用 `currentAttacker()` 而不是各处重算：那是唯一一处已经把这个
//	合成值算对的地方，重���就是**第二份实现**。
func (c terrainCtx) ElementCap() int64 { return c.e.currentAttacker().ElementCap }

// Enemies 返回全部敌人（给全场元素用）。
func (c terrainCtx) Enemies() []TerrainEnemy {
	e := c.e
	out := make([]TerrainEnemy, 0, len(e.Enemies))
	for _, en := range e.Enemies {
		out = append(out, terrainEnemy{en})
	}
	return out
}

// EnemiesInRadius 返回逻辑坐标半径内的敌人。
func (c terrainCtx) EnemiesInRadius(x, y, r int) []TerrainEnemy {
	e := c.e
	rr := ToFixed(float64(r))
	cx, cy := ToFixed(float64(x)), ToFixed(float64(y))
	out := make([]TerrainEnemy, 0, len(e.Enemies))
	for _, en := range e.Enemies {
		if en.Dead {
			continue
		}
		if dist2(cx, cy, en.X, en.Y) <= rr*rr {
			out = append(out, terrainEnemy{en})
		}
	}
	return out
}

// Within 报告定点坐标 (x,y) 是否落在逻辑坐标 (tx,ty) 的 r 半径内。
func (c terrainCtx) Within(tx, ty, r int, x, y int64) bool {
	rr := ToFixed(float64(r))
	return dist2(ToFixed(float64(tx)), ToFixed(float64(ty)), x, y) <= rr*rr
}

// Projectiles 返回当前所有弹丸。
func (c terrainCtx) Projectiles() []TerrainProjectile {
	e := c.e
	out := make([]TerrainProjectile, 0, len(e.Projectiles))
	for _, p := range e.Projectiles {
		out = append(out, p)
	}
	return out
}

// OnKill 通知引擎某敌人死亡（地形导致的，如油桶火区烧死）。
//
// ⚠️ 击杀分必须在这里记，与 `hitEnemy` 里的那条口径一致。
//
//	原先只 kills++ 不加分，于是「用油桶火区烧死」与「用弹丸打死」
//	同样一只怪差 500 分（BOSS 差 5000）——
//	分数不再只取决于战果，还取决于敌人怎么死，
//	直接影响上报的 score 与星级判定。
func (c terrainCtx) OnKill(t TerrainEnemy) {
	e := c.e
	en, ok := t.(terrainEnemy)
	if !ok {
		return
	}
	e.killEnemy(en.Enemy)
}

// dominantElementOfEnemy 返回敌人身上层数最多的元素。
func dominantElementOfEnemy(en *Enemy) Element {
	best := Element("")
	bestStacks := int64(0)
	for _, el := range AllElements() {
		if en.Stacks[el] > bestStacks {
			best, bestStacks = el, en.Stacks[el]
		}
	}
	return best
}

// terrainEnemy 让 `*Enemy` 满足 `TerrainEnemy`。
//
// # ⚠️ 为什么需要适配器而不是直接给 `*Enemy` 加方法
//
// `TerrainEnemy` 要求方法 `HP()` / `ArmorPermille()` / `Dead()`，
// 而 `Enemy` 已经有**同名的字段** `HP` / `ArmorPermille` / `Dead`。
// Go 不允许同一个类型上字段与方法同名 ——
// 所以只能包一层。
//
// 包一层而不是改 `Enemy` 的字段名：字段名与 TS 的 `Enemy` 一一对应，
// 改它会让所有读 `en.HP` 的地方都要动，
// 而「跨端对齐逐字段比对」依赖这些名字保持可读。
type terrainEnemy struct {
	*Enemy
}

func (a terrainEnemy) HP() int64            { return a.Enemy.HP }
func (a terrainEnemy) SetHP(v int64)        { a.Enemy.HP = v }
func (a terrainEnemy) ArmorPermille() int64 { return a.Enemy.ArmorPermille }
func (a terrainEnemy) ApplyElement(el string, layers, cap int64) {
	a.Enemy.Stacks[Element(el)] += layers
	if cap > 0 && a.Enemy.Stacks[Element(el)] > cap {
		a.Enemy.Stacks[Element(el)] = cap
	}
}
func (a terrainEnemy) HitFlash(ms int64)   { a.Enemy.HitFlashMs = ms }
func (a terrainEnemy) Dead() bool          { return a.Enemy.Dead }
func (a terrainEnemy) SetDead(v bool)      { a.Enemy.Dead = v }
func (a terrainEnemy) Pos() (int64, int64) { return a.Enemy.X, a.Enemy.Y }
func (a terrainEnemy) AddVelocity(vx, vy int64) {
	a.Enemy.X += vx
	a.Enemy.Y += vy
}

// Pos 返回弹丸的定点坐标。
func (p *Projectile) Pos() (x, y int64) { return p.X, p.Y }

// AddVelocity 累加速度增量。
func (p *Projectile) AddVelocity(vx, vy int64) {
	p.VX += vx
	p.VY += vy
}
