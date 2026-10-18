package domain

// 可交互地形（I-4）在服务端的镜像（R1 第六步移植）。
//
// # 移植前先确认了一件事：`onTerrainTrigger` 是**死接口**
//
// `terrain.ts` 的 `TerrainContext` 声明了 `onTerrainTrigger(kind, t)`，
// 但 `Terrain` 内部**从不调用**它 —— 全文件只有声明那一行（L70）。
// engine.ts 自己实现了这个回调并从别处调用，与 Terrain 无关。
//
// 所以本文件**不移植**它。移植一个从不执行的回调 =
// 造出一个「地形会在触发时通知引擎」的能力，而实际上没有 ——
// 那正是 README「已知边界」里「纯装饰」「只写不读」那两条的形状。
// 将来若引擎真的开始调它，再加不迟。

import (
	"fmt"
	"math"
)

// 坐标约定：地形配置用**逻辑单位**（0..1000），引擎内部实体坐标用
// **定点整数**（×1000）。两者混用会让判定范围差 1000 倍。
const (
	// FieldW / FieldH 是战场逻辑尺寸。
	FieldW = 1000
	FieldH = 1000
	// BaseX / BaseY 是防线所在位置（敌人从右向左推进）。
	BaseX = 60
	BaseY = 880
)

const (
	// OilBurnMs 是油桶火区持续毫秒。
	OilBurnMs = 8000
	// RotorRadius 是旋转风障作用半径（逻辑单位）。
	RotorRadius = 260
)

// 地形状态。
const (
	TerrainIdle      = "idle"
	TerrainBurning   = "burning"
	TerrainCollapsed = "collapsed"
	TerrainCharging  = "charging"
	TerrainOpen      = "open"
	TerrainClosed    = "closed"
)

// 地形类型。
const (
	TerrainOilDrum      = "oil_drum"
	TerrainTidalGate    = "tidal_gate"
	TerrainRotorVane    = "rotor_vane"
	TerrainCollapseWall = "collapse_wall"
	TerrainChargeTower  = "charge_tower"
)

// ToFixed 逻辑单位 → 定点整数（×1000）。
//
// ⚠️ 参数是 **float64**，与 TS 的 `toFixed(logical: number)` 同形。
//
// 这不是为了「看起来一样」，而是**必要**：TS 的关卡坐标来自 JSON，
// 理论上是浮点。若这里收 int，`logical*1000` 恒为整数，
// `math.Round` 与截断完全等价 —— 那道取整就成了装饰品，
// 而一旦线上出现带小数的坐标（比如运营在 DB 里写了 123.5），
// 两端就会往不同方向取整，判定半径差 1 个定点单位。
//
// 「擦边命中」恰好是最容易踩到边界的场景：命中点正好落在
// 「半径 × 1000」的整数上时，取整方式的差别立刻变成「命中/不命中」。
func ToFixed(logical float64) int64 {
	return int64(math.Round(logical * 1000))
}

// TerrainEnemy 是地形能操作的敌人**最小面**。
//
// 刻意只暴露地形真正写到的字段，而不是整个 Enemy：
// 依赖面越窄，将来引擎实体改字段时受影响的面越小。
type TerrainEnemy interface {
	HP() int64
	SetHP(int64)
	ArmorPermille() int64
	// ApplyElement 施加元素层数（由敌人自己按 cap 夹紧）。
	ApplyElement(element string, layers, cap int64)
	HitFlash(ms int64)
	Dead() bool
	SetDead(bool)
}

// TerrainProjectile 是地形能操作的弹丸**最小面**。
type TerrainProjectile interface {
	// Pos 返回定点坐标。
	Pos() (x, y int64)
	// AddVelocity 累加速度增量。
	AddVelocity(vx, vy int64)
}

// TerrainCtx 是地形能操作的引擎对象集合。
//
// 镜像 `terrain.ts` 的 `TerrainContext`，但用接口而不是结构体：
// Go 侧引擎实体还没移植，先把**依赖形状**定下来，
// 等引擎就位时只要实现这个接口即可，地形本身一行不用改。
type TerrainCtx interface {
	// TerrainTickDamage 是火区每 tick 伤害（**减伤前**）。
	TerrainTickDamage() int64
	// ElementCap 是当前元素层数上限。
	//
	// ⚠️ 必须来自 `currentAttacker().elementCap` 而不是常量 ——
	// 它由关卡的 `element_cap` 与局内 `elementCapBonus` 合成，
	// 玩家还能靠「元素容器」卡抬高。写死成 4n 会让该合成值完全无效。
	ElementCap() int64
	// EnemiesInRadius 返回逻辑坐标半径内的敌人。
	EnemiesInRadius(x, y, r int) []TerrainEnemy
	// Enemies 返回全部敌人（给全场元素用）。
	Enemies() []TerrainEnemy
	// Within 报告定点坐标 (x,y) 是否落在逻辑坐标 (tx,ty) 的 r 半径内。
	Within(tx, ty, r int, x, y int64) bool
	// Projectiles 返回当前所有弹丸。
	Projectiles() []TerrainProjectile
	// OnKill 通知引擎某敌人死亡。
	OnKill(TerrainEnemy)
}

// TerrainEffect 是地形对当前帧的影响。
type TerrainEffect struct {
	// Blocked 报告是否阻挡该区域（潮汐闸关闭 / 掩体未崩塌）。
	Blocked bool
}

// Terrain 是一个地形实例（镜像 `terrain.ts` 的 `Terrain` class）。
type Terrain struct {
	Kind  string
	X     int
	Y     int
	Param int

	State  string
	Timer  int64
	Charge int
	// Angle 是旋转风障当前角度（度，0..359）。
	Angle int
	// Triggered 报告本局是否被触发过（计入 terrain_used）。
	Triggered bool
}

// NewTerrain 从关卡配置造一个地形。
func NewTerrain(p TerrainPlacement) *Terrain {
	return &Terrain{
		Kind:   p.Kind,
		X:      p.X,
		Y:      p.Y,
		Param:  p.Param,
		State:  TerrainIdle,
		Timer:  0,
		Charge: 0,
		Angle:  0,
	}
}

// unknownKindWarned 记住已告警过的未知地形类型。
//
// 与 TS 的 `warnedUnknownKind` 同构：一个坏数据在 12000 tick 里
// 打 12000 条日志，会把真正的错误淹掉。
var unknownKindWarned = map[string]bool{}

// warnUnknownKind 打印一次未知地形告警。
func warnUnknownKind(kind string) {
	if unknownKindWarned[kind] {
		return
	}
	unknownKindWarned[kind] = true
	fmt.Printf("[terrain] 未知地形类型 %q，已按无效果处理。客户端与服务端的地形表可能不同步。\n", kind)
}

// Update 推进一帧。
//
// ⚠️ `default` 分支不是「防御性编程」，是**必需**的：
// `kind` 来自**网络数据**（DB 里的关卡行被手改、灰度中的新地形
// 下发到旧客户端、JSON 缺字段），任何一种都会让匹配落空。
// 未知地形应当退化成「什么都不做」——这也是渲染层 canvas.ts 的既有行为。
func (t *Terrain) Update(dtMs int64, ctx TerrainCtx) TerrainEffect {
	switch t.Kind {
	case TerrainOilDrum:
		return t.updateOilDrum(dtMs, ctx)
	case TerrainTidalGate:
		return t.updateTidalGate(dtMs)
	case TerrainRotorVane:
		return t.updateRotorVane(dtMs, ctx)
	case TerrainCollapseWall:
		return t.updateCollapseWall()
	case TerrainChargeTower:
		return t.updateChargeTower(dtMs)
	default:
		warnUnknownKind(t.Kind)
		return TerrainEffect{Blocked: false}
	}
}

// updateOilDrum：受焰元素命中达阈值即引燃 → 爆炸 + 生成 8s 火区。
func (t *Terrain) updateOilDrum(dtMs int64, ctx TerrainCtx) TerrainEffect {
	if t.State != TerrainBurning {
		return TerrainEffect{Blocked: false}
	}
	t.Timer -= dtMs
	if t.Timer <= 0 {
		t.State = TerrainIdle
		t.Timer = 0
		return TerrainEffect{Blocked: false}
	}
	// 火区持续伤害。
	//
	// ⚠️ 护甲必须走标准管线：引擎里敌人是按 `Defender(hp, shield, armor)`
	// 处理的，每一发普通命中都过护甲。火区若直接 `hp -= tick`，
	// 等于给敌人开了一个「无视护甲」通道 ——
	// 高护甲敌人在火区里承受的**相对**伤害比在别处高一倍。
	//
	// ⚠️ 元素层数上限取 ctx.ElementCap()，由敌人自己夹紧。
	// 写死成 4n 会让「上限 > 4 的构筑拿不到收益、上限 < 4 的构筑白拿 1 层」。
	cap := ctx.ElementCap()
	for _, e := range ctx.EnemiesInRadius(t.X, t.Y, 90) {
		e.SetHP(e.HP() - applyArmor(ctx.TerrainTickDamage(), e.ArmorPermille()))
		e.HitFlash(120)
		e.ApplyElement("fire", 1, cap)
		if e.HP() <= 0 {
			e.SetHP(0)
			e.SetDead(true)
			ctx.OnKill(e)
		}
	}
	return TerrainEffect{Blocked: false}
}

// updateTidalGate：周期开合，开时低层敌人可通行。
func (t *Terrain) updateTidalGate(dtMs int64) TerrainEffect {
	period := int64(t.Param)
	if period < 1 {
		period = 1
	}
	t.Timer += dtMs
	for t.Timer >= period {
		t.Timer -= period
		if t.State == TerrainOpen {
			t.State = TerrainClosed
		} else {
			t.State = TerrainOpen
		}
	}
	return TerrainEffect{Blocked: t.State == TerrainClosed}
}

// updateRotorVane：持续改变区域内弹丸飞行方向。
//
// ⚠️ 这里**不用** `math.Cos` / `math.Sin`。
//
// 超越函数的精度是**实现定义**的：不同语言版本、不同 CPU 架构
// 可能给出相差 1 ulp 的结果，而 `math.Round` 会把这个 1 ulp
// 放大成**整整 1 个定点单位**。
//
// 后果不是「风障偏了一点点」：弹丸速度变了 → 命中位置变了 →
// 击杀的 tick 变了 → replayHash 变了 → I-6 把正常对局判成伪造。
//
// 解法：方向表在**构建期**用浮点算好后烘焙成源码里的字面量，
// 运行时只做整数查表。表是数据，两端读同一份字面量，逐位一致。
func (t *Terrain) updateRotorVane(dtMs int64, ctx TerrainCtx) TerrainEffect {
	// 角度增量 = floor(param × dtMs / 1000)。
	//
	// 用整数截断而非 math.Floor：param 与 dtMs 均为非负，
	// 两种写法同值；写成 floor 反而多一个浮点往返。
	t.Angle = (t.Angle + int(int64(t.Param)*dtMs/1000)) % 360
	ux := int64(rotorDir1000[t.Angle*2])
	uy := int64(rotorDir1000[t.Angle*2+1])
	// 幅度 = param × 0.4。用整数乘除，避免 Math.round：
	// round 在 x.5 边界上的行为依赖浮点误差，而这里已经是精确的千分比。
	mag := int64(t.Param) * 4 / 10
	deflectX := (ux * mag * 2) / 1000
	deflectY := (uy * mag * 2) / 1000
	for _, p := range ctx.Projectiles() {
		px, py := p.Pos()
		if ctx.Within(t.X, t.Y, RotorRadius, px, py) {
			p.AddVelocity(deflectX, deflectY)
		}
	}
	return TerrainEffect{Blocked: false}
}

// updateCollapseWall：未崩塌时阻挡弹道。
//
// ⚠️ 这里**不做任何充能**。曾经这里每 tick 扫一遍半径 110 内的动能弹丸
// 累加 charge，但那条路径在几何上**永远走不到**：
// engine 的弹丸阻挡判定半径是 100，弹丸进入 110 的瞬间就已被标记 dead
// 并从 projectiles 里滤掉；且 step() 里 updateProjectiles 先于 updateTerrain。
// 所以掩体的 charge 恒为 0，param 调到多大都没用。
//
// 唯一的充能来源是 OnHit（按命中伤害累计）。
// 留一个走不到的分支比没有分支更糟：它让人以为机制在工作。
func (t *Terrain) updateCollapseWall() TerrainEffect {
	if t.State == TerrainCollapsed {
		return TerrainEffect{Blocked: false}
	}
	return TerrainEffect{Blocked: true}
}

// updateChargeTower：充能由击杀驱动（见 OnKillCharging），这里只做计时展示。
func (t *Terrain) updateChargeTower(dtMs int64) TerrainEffect {
	t.Timer = dtMs
	return TerrainEffect{Blocked: false}
}

// OnHit 由命中结算调用：把伤害与元素喂给地形。
//
// ⚠️ 必须带位置判定。engine 的调用点是
// `for (const t of this.terrains) if (t.onHit(p.element, res.totalDamage))`
// —— **对所有地形都调一遍，不看弹丸与地形是否挨着**。
// 于是「站在 x=900 的油桶」会被「打到 x=200 的敌人」的火焰点燃，
// 机制语义整个掉：玩家无法通过站位影响地形，只能靠无脑堆同元素。
func (t *Terrain) OnHit(element string, damage int64, hitX, hitY int64) bool {
	if t.State == TerrainCollapsed {
		return false
	}
	tx := ToFixed(float64(t.X))
	ty := ToFixed(float64(t.Y))
	near := func(r float64) bool {
		rr := ToFixed(r)
		dx := hitX - tx
		dy := hitY - ty
		return dx*dx+dy*dy <= rr*rr
	}
	switch t.Kind {
	case TerrainOilDrum:
		// 判定半径比阻挡半径（100）略大，保证「擦边命中」也算。
		if element == "fire" && t.State == TerrainIdle && near(120) {
			t.Charge += int(damage / 100)
			if t.Charge >= t.Param {
				t.State = TerrainBurning
				t.Timer = OilBurnMs
				t.Charge = t.Param
				t.Triggered = true
				return true
			}
		}
		return false
	case TerrainCollapseWall:
		// 动能打进掩体。半径 100 与阻挡判定对齐 —— 弹丸要真的「打到墙上」才算。
		//
		// ⚠️ 任何元素都能充能，动能 ×3。曾经是**只有动能能充能**：
		// 「硬性克制」在这里等于「硬性死锁」—— 掩体挡掉弹丸，
		// 而只能被动能打破的话，不带动能的构筑**永远打不开弹道**。
		// 默认构筑恰好是 fire/fire/fire/ice，一个动能都没有，
		// 于是第 22/33/34 关对默认构筑不可通关。
		//
		// 改成「动能快 3 倍、其余也能砸」后：带动能仍明显更优，
		// 不带动能能砸开只是慢，关卡不会变成死局。
		// 一般原则：地形机制可以有**偏好**，不能有**唯一解**。
		if near(100) {
			mul := 1
			if element == "kinetic" {
				mul = 3
			}
			t.Charge += int(damage/100) * mul
			if t.Charge >= t.Param {
				t.State = TerrainCollapsed
				t.Triggered = true
				return true
			}
		}
		return false
	default:
		return false
	}
}

// OnKillCharging 击杀时充能。蓄能塔满时给全场敌人上同种元素。
func (t *Terrain) OnKillCharging(element string, cap int64, ctx TerrainCtx) bool {
	if t.Kind != TerrainChargeTower {
		return false
	}
	t.Charge++
	if t.Charge < t.Param {
		return false
	}
	t.Charge = 0
	t.Triggered = true
	t.applyChargeToAll(element, ctx, cap)
	return true
}

// applyChargeToAll 给全场敌人上同种元素（翻盘机制）。
func (t *Terrain) applyChargeToAll(element string, ctx TerrainCtx, cap int64) {
	for _, e := range ctx.Enemies() {
		e.ApplyElement(element, 1, cap)
	}
}

// BlocksProjectile 报告是否阻挡弹道（崩塌后的掩体不再阻挡）。
func (t *Terrain) BlocksProjectile() bool {
	return t.Kind == TerrainCollapseWall && t.State != TerrainCollapsed
}

// TerrainSnapshot 是地形的对外表示（镜像 TS 的 `serialize()`）。
type TerrainSnapshot struct {
	Kind      string `json:"kind"`
	X         int    `json:"x"`
	Y         int    `json:"y"`
	State     string `json:"state"`
	Triggered bool   `json:"triggered"`
}

// Serialize 返回地形快照。
func (t *Terrain) Serialize() TerrainSnapshot {
	return TerrainSnapshot{
		Kind:      t.Kind,
		X:         t.X,
		Y:         t.Y,
		State:     t.State,
		Triggered: t.Triggered,
	}
}

// rotorDir1000 是旋转风障的单位方向表：**整数千分比**的方向向量。
//
// 扁平存放，下标 `角度*2` 是 x（cos）、`角度*2+1` 是 y（sin），
// 角度范围 0..359 度，共 720 个数。
//
// ⚠️ 必须是**从 terrain.ts 原样抽取**的，不允许手抄 ——
//
//	手抄 720 个数字，错一个符号不会有任何编译期提示，
//	而后果是某个角度的弹丸偏转方向反了 → 命中位置变 → 击杀 tick 变
//	→ replayHash 变 → I-6 把正常对局判成伪造。
//
// 校验：每项的模长应为 1000（允许 ±1 的四舍五入误差），
// 由 TestRotorDirIsUnitVector 守住；与 TS 的一致性由跨端向量守住。
//
// 抽取来源由 `TestRotorDirMatchesTsSource` **强制**：那条守卫直接读
// `miniapp/src/game/terrain.ts` 并逐项比对 720 个数，
// 所以「抄错了」不可能合入。文档化的纪律挡不住手抄，能挡住的是守卫。
var rotorDir1000 = [...]int{
	1000, 0, 1000, 17, 999, 35, 999, 52, 998, 70, 996, 87, 995, 105, 993, 122, 990, 139, 988, 156,
	985, 174, 982, 191, 978, 208, 974, 225, 970, 242, 966, 259, 961, 276, 956, 292, 951, 309, 946, 326,
	940, 342, 934, 358, 927, 375, 921, 391, 914, 407, 906, 423, 899, 438, 891, 454, 883, 469, 875, 485,
	866, 500, 857, 515, 848, 530, 839, 545, 829, 559, 819, 574, 809, 588, 799, 602, 788, 616, 777, 629,
	766, 643, 755, 656, 743, 669, 731, 682, 719, 695, 707, 707, 695, 719, 682, 731, 669, 743, 656, 755,
	643, 766, 629, 777, 616, 788, 602, 799, 588, 809, 574, 819, 559, 829, 545, 839, 530, 848, 515, 857,
	500, 866, 485, 875, 469, 883, 454, 891, 438, 899, 423, 906, 407, 914, 391, 921, 375, 927, 358, 934,
	342, 940, 326, 946, 309, 951, 292, 956, 276, 961, 259, 966, 242, 970, 225, 974, 208, 978, 191, 982,
	174, 985, 156, 988, 139, 990, 122, 993, 105, 995, 87, 996, 70, 998, 52, 999, 35, 999, 17, 1000,
	0, 1000, -17, 1000, -35, 999, -52, 999, -70, 998, -87, 996, -105, 995, -122, 993, -139, 990, -156, 988,
	-174, 985, -191, 982, -208, 978, -225, 974, -242, 970, -259, 966, -276, 961, -292, 956, -309, 951, -326, 946,
	-342, 940, -358, 934, -375, 927, -391, 921, -407, 914, -423, 906, -438, 899, -454, 891, -469, 883, -485, 875,
	-500, 866, -515, 857, -530, 848, -545, 839, -559, 829, -574, 819, -588, 809, -602, 799, -616, 788, -629, 777,
	-643, 766, -656, 755, -669, 743, -682, 731, -695, 719, -707, 707, -719, 695, -731, 682, -743, 669, -755, 656,
	-766, 643, -777, 629, -788, 616, -799, 602, -809, 588, -819, 574, -829, 559, -839, 545, -848, 530, -857, 515,
	-866, 500, -875, 485, -883, 469, -891, 454, -899, 438, -906, 423, -914, 407, -921, 391, -927, 375, -934, 358,
	-940, 342, -946, 326, -951, 309, -956, 292, -961, 276, -966, 259, -970, 242, -974, 225, -978, 208, -982, 191,
	-985, 174, -988, 156, -990, 139, -993, 122, -995, 105, -996, 87, -998, 70, -999, 52, -999, 35, -1000, 17,
	-1000, 0, -1000, -17, -999, -35, -999, -52, -998, -70, -996, -87, -995, -105, -993, -122, -990, -139, -988, -156,
	-985, -174, -982, -191, -978, -208, -974, -225, -970, -242, -966, -259, -961, -276, -956, -292, -951, -309, -946, -326,
	-940, -342, -934, -358, -927, -375, -921, -391, -914, -407, -906, -423, -899, -438, -891, -454, -883, -469, -875, -485,
	-866, -500, -857, -515, -848, -530, -839, -545, -829, -559, -819, -574, -809, -588, -799, -602, -788, -616, -777, -629,
	-766, -643, -755, -656, -743, -669, -731, -682, -719, -695, -707, -707, -695, -719, -682, -731, -669, -743, -656, -755,
	-643, -766, -629, -777, -616, -788, -602, -799, -588, -809, -574, -819, -559, -829, -545, -839, -530, -848, -515, -857,
	-500, -866, -485, -875, -469, -883, -454, -891, -438, -899, -423, -906, -407, -914, -391, -921, -375, -927, -358, -934,
	-342, -940, -326, -946, -309, -951, -292, -956, -276, -961, -259, -966, -242, -970, -225, -974, -208, -978, -191, -982,
	-174, -985, -156, -988, -139, -990, -122, -993, -105, -995, -87, -996, -70, -998, -52, -999, -35, -999, -17, -1000,
	0, -1000, 17, -1000, 35, -999, 52, -999, 70, -998, 87, -996, 105, -995, 122, -993, 139, -990, 156, -988,
	174, -985, 191, -982, 208, -978, 225, -974, 242, -970, 259, -966, 276, -961, 292, -956, 309, -951, 326, -946,
	342, -940, 358, -934, 375, -927, 391, -921, 407, -914, 423, -906, 438, -899, 454, -891, 469, -883, 485, -875,
	500, -866, 515, -857, 530, -848, 545, -839, 559, -829, 574, -819, 588, -809, 602, -799, 616, -788, 629, -777,
	643, -766, 656, -755, 669, -743, 682, -731, 695, -719, 707, -707, 719, -695, 731, -682, 743, -669, 755, -656,
	766, -643, 777, -629, 788, -616, 799, -602, 809, -588, 819, -574, 829, -559, 839, -545, 848, -530, 857, -515,
	866, -500, 875, -485, 883, -469, 891, -454, 899, -438, 906, -423, 914, -407, 921, -391, 927, -375, 934, -358,
	940, -342, 946, -326, 951, -309, 956, -292, 961, -276, 966, -259, 970, -242, 974, -225, 978, -208, 982, -191,
	985, -174, 988, -156, 990, -139, 993, -122, 995, -105, 996, -87, 998, -70, 999, -52, 999, -35, 1000, -17,
}
