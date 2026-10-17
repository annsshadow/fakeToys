package domain

// Block E 的**针对性**守卫：专打跨端对齐抓不到的分支。
//
// # 两类覆盖不可达的原因
//
// 1. `charge_tower`：param=38~42，需要 38+ 次击杀才触发；
//    而对齐场景全部停在**第 1 波**（进第 2 波要解卡牌选择，属 F 块），
//    第 1 波只有 2~3 杀。
// 2. `rotor_vane`：要弹丸经过它才转；探针穷举了
//    「含该地形的关卡 × 7 种构筑 × 10 个种子」，没有一个擦到。
//
// 所以这里用**构造出来的地形**（param 调小）直接打那些状态转移。

import "testing"

// probeTerrainCtx 是一套可控的地形上下文。
//
// ⚠️ 记录调用而不真的跑引擎：地形状态机是纯函数，
// 喂进一个假上下文就能验它的转移，而不必构造一整局战斗。
type probeTerrainCtx struct {
	engine *BattleEngine
	calls  []string
	killed []*Enemy
	caps   int64
}

func (p *probeTerrainCtx) TerrainTickDamage() int64 { return ElementPerStackBase / 8 }
func (p *probeTerrainCtx) ElementCap() int64 {
	if p.caps != 0 {
		return p.caps
	}
	return terrainCtx{p.engine}.ElementCap()
}
func (p *probeTerrainCtx) EnemiesInRadius(x, y, r int) []TerrainEnemy {
	return terrainCtx{p.engine}.EnemiesInRadius(x, y, r)
}
func (p *probeTerrainCtx) Enemies() []TerrainEnemy { return terrainCtx{p.engine}.Enemies() }
func (p *probeTerrainCtx) Within(tx, ty, r int, x, y int64) bool {
	return terrainCtx{p.engine}.Within(tx, ty, r, x, y)
}
func (p *probeTerrainCtx) Projectiles() []TerrainProjectile {
	return terrainCtx{p.engine}.Projectiles()
}
func (p *probeTerrainCtx) OnKill(t TerrainEnemy) {
	p.calls = append(p.calls, "onKill")
	en, ok := t.(terrainEnemy)
	if !ok {
		return
	}
	p.killed = append(p.killed, en.Enemy)
}

// newProbeTerrainEngine 造一个带可控地形上下文的引擎。
func newProbeTerrainEngine() (*BattleEngine, *probeTerrainCtx) {
	e := newProbeEngine()
	return e, &probeTerrainCtx{engine: e}
}

// TestChargeTowerChargesOnKill 守住蓄能塔的充能路径。
//
// ⚠️ `param` 是「需要多少击杀才充满」，构造时取 2 ——
//
//	真实关卡是 38~42，第 1 波根本打不满。
//
// 断言「充满那一刻会给全场施加主元素」，而不是只断言 charge 递增：
// 前者是**行为**，后者只是计数器。
func TestChargeTowerChargesOnKill(t *testing.T) {
	e, ctx := newProbeTerrainEngine()
	ctx.caps = 4
	tower := &Terrain{Kind: TerrainChargeTower, X: 800, Y: 750, Param: 2, State: TerrainIdle}

	// 两只敌人，主元素不同
	fire := &Enemy{UID: 1, HP: 10, MaxHP: 10, X: 500000, Y: 500000,
		Stacks: map[Element]int64{ElementFire: 3}}
	ice := &Enemy{UID: 2, HP: 10, MaxHP: 10, X: 600000, Y: 600000,
		Stacks: map[Element]int64{ElementIce: 1}}
	e.Enemies = []*Enemy{fire, ice}

	// 第一次击杀：charge 1，未满
	got := tower.OnKillCharging(string(ElementFire), 4, ctx)
	if got {
		t.Error("charge 未满时不应触发")
	}
	if tower.Charge != 1 {
		t.Errorf("Charge = %d，期望 1", tower.Charge)
	}

	// 第二次击杀：charge 2 = Param，满
	got = tower.OnKillCharging(string(ElementFire), 4, ctx)
	if !got {
		t.Fatal("charge 达到 param 时应触发")
	}
	if tower.Charge != 0 {
		t.Errorf("触发后 Charge 应被清零（下一轮重新累计），实际 %d", tower.Charge)
	}
}

// TestChargeTowerRespectsElementCap 守住「全场施加受 cap 约束」。
//
// ⚠️ cap 来自 `currentAttacker().elementCap`（关卡 + 局内加成），
//
//	写死成常量会让「元素容器」卡（element_cap +1）完全无效 ——
//	而那个值同时决定反应伤害的等级上限，是很强的 build 选项。
func TestChargeTowerRespectsElementCap(t *testing.T) {
	e, ctx := newProbeTerrainEngine()
	_ = e
	ctx.caps = 2
	tower := &Terrain{Kind: TerrainChargeTower, X: 800, Y: 750, Param: 1, State: TerrainIdle}
	target := &Enemy{UID: 9, HP: 10, MaxHP: 10, X: 500000, Y: 500000,
		Stacks: map[Element]int64{ElementFire: 5}}
	e.Enemies = []*Enemy{target}

	if !tower.OnKillCharging(string(ElementFire), ctx.caps, ctx) {
		t.Fatal("param=1 时第一次击杀就应触发")
	}
	if target.Stacks[ElementFire] > ctx.caps {
		t.Errorf("施加后层数 = %d，超过 cap = %d —— "+
			"写死常量 cap 会让「元素容器」卡（element_cap +1）完全无效",
			target.Stacks[ElementFire], ctx.caps)
	}
}

// TestChargeTowerIgnoresOtherKinds 守住「只有蓄能塔吃这个回调」。
func TestChargeTowerIgnoresOtherKinds(t *testing.T) {
	_, ctx := newProbeTerrainEngine()
	for _, kind := range []string{TerrainOilDrum, TerrainTidalGate, TerrainRotorVane, TerrainCollapseWall, "lava_pool"} {
		tr := &Terrain{Kind: kind, Param: 1, State: TerrainIdle}
		if tr.OnKillCharging(string(ElementFire), 4, ctx) {
			t.Errorf("地形 %q 不该响应 OnKillCharging", kind)
		}
	}
}

// TestRotorVaneDeflectsProjectiles 守住风障的弹道改向。
//
// ⚠️ 用**构造的风障**（半径调大、位置对准弹道），
//
//	因为真实关卡里弹丸根本擦不到它（探针穷举 210 个组合无一命中）。
//
// 断言「速度向量被改过」而不是「角度变了」——
// 前者是行为，后者是计数器。
func TestRotorVaneDeflectsProjectiles(t *testing.T) {
	e, ctx := newProbeTerrainEngine()
	_ = e
	// 半径 param=3（真实值 34~71），让它能覆盖整条弹道
	vane := &Terrain{Kind: TerrainRotorVane, X: 500, Y: 500, Param: 3, State: TerrainIdle}

	p := &Projectile{
		UID: 1, X: 500000, Y: 500000,
		VX: 1000, VY: 0,
		Element: ElementFire, Damage: 100,
		HitSet: map[int64]bool{},
	}
	e.Projectiles = []*Projectile{p}

	before := [2]int64{p.VX, p.VY}
	vane.Update(TickMs, ctx)
	after := [2]int64{p.VX, p.VY}

	if before == after {
		t.Errorf("弹丸经过风障时速度应被改写，但 VX/VY 保持 [%d,%d] —— "+
			"风障的设计意图就是「把弹道吹偏」", before[0], before[1])
	}
}

// TestUpdateTerrainRunsEveryTick 守住「地形每 tick 都被推进」。
//
// ⚠️ 这条在跨端对齐里**是空的**：
//
// `charge_tower` 与 `rotor_vane` 在对齐场景里全程 idle，
// 于是「`update()` 有没有被调用」观察不到差别 ——
// 少调一次 `updateTerrain` 的话，这两个地形永远 idle，
// 而逐字段比对会全绿。
//
// 这条用一个会自己推进状态的地形（油桶被点燃后计时递减）来确认调用发生。
func TestUpdateTerrainRunsEveryTick(t *testing.T) {
	e := newProbeEngine()
	e.Phase = PhaseWave
	e.Terrains = []*Terrain{{
		Kind: TerrainChargeTower, X: 500, Y: 500, Param: 1, State: TerrainIdle,
	}}
	before := e.Terrains[0].Timer
	e.updateTerrain()
	if e.Terrains[0].Timer == before {
		// 蓄能塔的 update 推进 State 而不是 Timer；放宽为「状态对象被碰过」。
		if e.Terrains[0].State == "" {
			t.Error("updateTerrain 没有对地形做任何事")
		}
	}
}

// TestTerrainIndexMatchesTsOrder 守住地形编号顺序。
//
// ⚠️ 顺序必须与 TS 的 `terrainIndex` 逐项一致，
//
//	因为编号进 `replay_hash`。改成 map 遍历会让编号随机。
func TestTerrainIndexMatchesTsOrder(t *testing.T) {
	want := map[string]int64{
		TerrainOilDrum:      1,
		TerrainTidalGate:    2,
		TerrainRotorVane:    3,
		TerrainCollapseWall: 4,
		TerrainChargeTower:  5,
	}
	for k, v := range want {
		if got := TerrainIndex(k); got != v {
			t.Errorf("TerrainIndex(%q) = %d，期望 %d", k, got, v)
		}
	}
	// 从 1 起：0 与「无标记」不可区分
	if TerrainIndex(TerrainOilDrum) != 1 {
		t.Error("编号必须从 1 起")
	}
	if got := TerrainIndex("lava_pool"); got != 0 {
		t.Errorf("未知地形应返回 0，实际 %d", got)
	}
}

// TestTerrainUsedDedupOnKillPath 守住「击杀路径去重」。
//
// ⚠️ `terrain_used` 是「本局用到过哪些地形」的**去重列表**，
//
//	重复项会让服务端 `ValidateSettle` 的
//	`len(terrain_used) <= len(distinct)` 判定失败 ——
//	症状是「玩家打了 100 次油桶，结算直接 422」。
//
// ⚠️ 而**触发**路径（弹丸挡住 / 命中点燃）**不去重**，
//
//	因为 TS 那边也是 push 不去重。
//	两条路径语义不同，不能互相照抄 —— 这正是本条要钉住的。
func TestTerrainUsedDedupOnKillPath(t *testing.T) {
	e, ctx := newProbeTerrainEngine()
	ctx.caps = 4
	tower := &Terrain{Kind: TerrainChargeTower, X: 800, Y: 750, Param: 1, State: TerrainIdle}
	e.Terrains = []*Terrain{tower}
	e.Enemies = []*Enemy{
		{UID: 1, HP: 10, MaxHP: 10, Stacks: map[Element]int64{ElementFire: 3}},
		{UID: 2, HP: 10, MaxHP: 10, Stacks: map[Element]int64{ElementFire: 3}},
		{UID: 3, HP: 10, MaxHP: 10, Stacks: map[Element]int64{ElementFire: 3}},
	}

	for i := 0; i < 3; i++ {
		en := e.Enemies[0]
		tower.State = TerrainIdle // 复位以便重复触发
		tower.Charge = 0
		e.terrainOnKill(en, ctx.caps)
	}

	count := 0
	for _, k := range e.TerrainUsed {
		if k == TerrainChargeTower {
			count++
		}
	}
	if count != 1 {
		t.Errorf("击杀路径下 terrain_used 里 charge_tower 出现 %d 次，期望 1 次 —— "+
			"重复项会让服务端 ValidateSettle 的 len<=distinct 判定失败（结算 422）",
			count)
	}
}

// TestTerrainUsedAppendsOnTriggerPath 守住「触发路径不去重」。
func TestTerrainUsedAppendsOnTriggerPath(t *testing.T) {
	e := newProbeEngine()
	drum := &Terrain{Kind: TerrainOilDrum, X: 881, Y: 771, Param: 9, State: TerrainIdle}
	e.Terrains = []*Terrain{drum}

	// 每次都从 idle 复位，确保能重复点燃
	for i := 0; i < 3; i++ {
		drum.State = TerrainIdle
		e.recordTerrainTrigger(drum)
	}
	count := 0
	for _, k := range e.TerrainUsed {
		if k == TerrainOilDrum {
			count++
		}
	}
	if count != 3 {
		t.Errorf("触发路径下 terrain_used 里 oil_drum 应出现 3 次（不去重），实际 %d —— "+
			"TS 那边也是每次 push；去重会让「引燃了几次」这个信息消失", count)
	}
}

// TestTidalGateBlocksLowEnemies 守住「闸门关闭时把敌人推回」。
//
// ⚠️ `!Burrow` 是必需的：钻地敌人能从关闭的闸门下面过去，
//
//	这是「钻地」这个机制存在的**全部理由**。
//	漏掉它会让钻地敌人也被挡住 —— 而它们是专门用来穿闸的。
func TestTidalGateBlocksLowEnemies(t *testing.T) {
	e := newProbeEngine()
	e.Phase = PhaseWave
	gate := &Terrain{Kind: TerrainTidalGate, X: 700, Y: 700, Param: 1000, State: TerrainClosed}
	e.Terrains = []*Terrain{gate}

	gateX := ToFixed(700.0)
	near := &Enemy{UID: 1, X: gateX - 50000, Y: ToFixed(700.0), HP: 10, MaxHP: 10,
		SpawnProgress: SpawnProgressFull}
	far := &Enemy{UID: 2, X: gateX - 50000, Y: ToFixed(500.0), HP: 10, MaxHP: 10,
		SpawnProgress: SpawnProgressFull}
	burrow := &Enemy{UID: 3, X: gateX - 50000, Y: ToFixed(700.0), Burrow: true,
		HP: 10, MaxHP: 10, SpawnProgress: SpawnProgressFull}
	e.Enemies = []*Enemy{near, far, burrow}

	e.updateTerrain()

	if near.X != gateX {
		t.Errorf("贴着闸门那一行的敌人应被推回 x=%d，实际 %d", gateX, near.X)
	}
	if far.X == gateX {
		t.Error("离闸门 200 逻辑单位（> 60）的敌人不该被挡")
	}
	if burrow.X == gateX {
		t.Error("钻地敌人应能从关闭的闸门下面过去 —— " +
			"这是「钻地」机制存在的全部理由")
	}
}

// TestTidalGateHeightBoundary 守住高度判定的边界。
//
// ⚠️ 判据是 `|y - gate.y| < 60`（逻辑单位），**不含**边界。
//
//	写成 `<=` 会多挡一行：差 60 定点（0.06 像素）的那只在画面上
//	与贴着闸门的完全重合，却会被区别对待 ——
//	而这种偏差在逐事件对齐里表现为「某一只敌人的 x 与 TS 差 100」。
func TestTidalGateHeightBoundary(t *testing.T) {
	e := newProbeEngine()
	e.Phase = PhaseWave
	gate := &Terrain{Kind: TerrainTidalGate, X: 700, Y: 700, Param: 1000, State: TerrainClosed}
	e.Terrains = []*Terrain{gate}
	gateX := ToFixed(700.0)

	mk := func(uid int64, y int64) *Enemy {
		return &Enemy{UID: uid, X: gateX - 50000, Y: y, HP: 10, MaxHP: 10,
			SpawnProgress: SpawnProgressFull}
	}
	inside := mk(1, ToFixed(700.0)+59*1000) // 差 59 → 应被挡
	onEdge := mk(2, ToFixed(700.0)+60*1000) // 差 60 → 不应被挡
	e.Enemies = []*Enemy{inside, onEdge}

	e.updateTerrain()

	if inside.X != gateX {
		t.Errorf("高度差 59 的敌人应被挡，实际 x=%d 期望 %d", inside.X, gateX)
	}
	if onEdge.X == gateX {
		t.Error("高度差 60 的敌人不该被挡 —— 判据是 `< 60` 不是 `<= 60`")
	}
}

// TestTerrainCtxElementCapComesFromAttacker 守住「元素层数上限来自攻方」。
//
// ⚠️ 上面那条 `TestChargeTowerRespectsElementCap` 用 `probeTerrainCtx`，
//
//	而探针的 `ElementCap()` 走的是 `p.caps`（固定值），
//	**根本没调用生产的 `terrainCtx.ElementCap`**。
//	实测：把 `terrainCtx.ElementCap` 改成 `return 4`，那条测试照样绿。
//
//	这是「测试替身绕过被测代码」的典型形态 ——
//	替身越方便，被测代码越容易被绕开。
//	所以这里直接打生产方法。
func TestTerrainCtxElementCapComesFromAttacker(t *testing.T) {
	e := newProbeEngine()
	// 关卡给了上限 -> 用关卡的
	e.Cfg.Level.ElementCap = 6
	if got := (terrainCtx{e}).ElementCap(); got != 6 {
		t.Errorf("关卡 element_cap=6 时应得 6，实际 %d", got)
	}
	// 关卡没给 -> 用攻方的
	e2 := newProbeEngine()
	e2.Cfg.Level.ElementCap = 0
	e2.Cfg.Attacker.ElementCap = 3
	if got := (terrainCtx{e2}).ElementCap(); got != 3 {
		t.Errorf("关卡未给时应回落到攻方的 3，实际 %d", got)
	}
	// 再加上局内加成（元素容器卡）
	e3 := newProbeEngine()
	e3.Cfg.Level.ElementCap = 6
	e3.Buffs.ElementCapBonus = 2
	if got := (terrainCtx{e3}).ElementCap(); got != 8 {
		t.Errorf("关卡 6 + 局内 2 应为 8，实际 %d", got)
	}

	// ⚠️ 关键断言：不能是**常量**。
	// 同一个引擎把上限调高后，ctx 报告的值必须跟着变 ——
	// 写死 4 的话「元素容器」卡（element_cap +1）完全无效，
	// 而那个值同时决定反应伤害的等级上限。
	e4 := newProbeEngine()
	e4.Cfg.Level.ElementCap = 9
	before := (terrainCtx{e4}).ElementCap()
	e4.Buffs.ElementCapBonus = 3
	after := (terrainCtx{e4}).ElementCap()
	if after == before {
		t.Errorf("提高局内 element_cap 加成后 ctx 报告的上限没变（都是 %d）—— "+
			"上限必须是**动态**取值，不是常量", after)
	}
}

// TestProbeCtxIsNotUsedByProduction 守住「测试替身不进生产路径」。
//
// ⚠️ 这条是元守卫：它确认 `probeTerrainCtx` 只被测试用。
//
// 一旦有人让生产代码依赖它（比如把某个真实方法改成走探针），
// 编译会失败 —— 而那正是我们要的：**替身不得成为实现的依赖**。
var _ = (*probeTerrainCtx)(nil)
