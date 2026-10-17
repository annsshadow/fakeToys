package domain

// 地形（R1 第六步移植）的跨端契约守卫。
//
// # 为什么用「记录型 ctx」
//
// 地形与引擎的耦合面只有 6 个方法。契约把它们换成
// 「记录调用并返回预设敌人/弹丸」的假实现，于是断言的仍然是真实的东西：
//
//   - 地形自己的状态迁移（burning→idle、open↔closed、charge 阈值、角度）
//   - 地形向引擎**要什么**（半径 90？半径 260？elementCap 传了多少？）
//   - 地形对敌人**做什么**（扣多少血、叠几层、是否 onKill）
//
// 真实的引擎实体（Enemy / Projectile）**不在契约范围内** ——
// 那些要等引擎移植后单独验。现在假装能验，就是自己骗自己。
//
// 假实现必须**足够真**：敌人要有真实 hp 与 armorPermille，
// 否则 applyArmor 的效果测不出来（全 armor=0 时减伤恒为 1 倍）。

import (
	"fmt"
	"math"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"testing"
)

// --- 记录型假实现 ---

type recEnemy struct {
	id            string
	hp            int64
	armor         int64
	x, y          int
	dead          bool
	hitFlash      int64
	elemElement   string
	elemLayers    int64
	appliedLaters []int64 // 每次 applyElement 传入的 layers
	appliedCaps   []int64 // 每次 applyElement 传入的 cap
}

func (e *recEnemy) HP() int64            { return e.hp }
func (e *recEnemy) SetHP(v int64)        { e.hp = v }
func (e *recEnemy) ArmorPermille() int64 { return e.armor }
func (e *recEnemy) HitFlash(ms int64)    { e.hitFlash = ms }
func (e *recEnemy) Dead() bool           { return e.dead }
func (e *recEnemy) SetDead(v bool)       { e.dead = v }

// ApplyElement 复刻真实敌人的元素夹紧语义：
// 层数超过 cap 时按 cap 夹（types.ts 的 Enemy.applyElement 如此）。
func (e *recEnemy) ApplyElement(element string, layers, cap int64) {
	e.appliedLaters = append(e.appliedLaters, layers)
	e.appliedCaps = append(e.appliedCaps, cap)
	if e.elemElement != element {
		e.elemElement = element
		e.elemLayers = layers
	} else {
		e.elemLayers += layers
	}
	if e.elemLayers > cap {
		e.elemLayers = cap
	}
}

// snapshot 产出与向量里同形状的字符串。
func (e *recEnemy) snapshot() string {
	elems := ""
	if e.elemElement != "" {
		elems = fmt.Sprintf("%s:%d", e.elemElement, e.elemLayers)
	}
	return fmt.Sprintf("{%s hp=%d armor=%d dead=%t flash=%d elems=[%s]}",
		e.id, e.hp, e.armor, e.dead, e.hitFlash, elems)
}

type recProjectile struct {
	id    string
	x, y  int64
	vx    int64
	vy    int64
	added []string
}

func (p *recProjectile) Pos() (int64, int64) { return p.x, p.y }
func (p *recProjectile) AddVelocity(vx, vy int64) {
	p.vx += vx
	p.vy += vy
	p.added = append(p.added, fmt.Sprintf("%d,%d", vx, vy))
}

type recCtx struct {
	calls       []string
	enemies     []*recEnemy
	projectiles []*recProjectile
	terrainTick int64
	elementCap  int64
	// capCalls 记录 TerrainTickDamage 被取了几次。
	//
	// ⚠️ 它参与契约：油桶原本在 Go 第一版里**每帧只取一次** tick 伤害
	// 然后复用（我写岔了），而 TS 是**每个敌人取一次**。
	// 数字上碰巧一样（同一帧内值不变），但那是个不该依赖的巧合。
	capCalls int
}

func (c *recCtx) TerrainTickDamage() int64 { c.capCalls++; return c.terrainTick }
func (c *recCtx) ElementCap() int64        { return c.elementCap }
func (c *recCtx) Enemies() []TerrainEnemy {
	out := make([]TerrainEnemy, 0, len(c.enemies))
	for _, e := range c.enemies {
		out = append(out, e)
	}
	return out
}
func (c *recCtx) Projectiles() []TerrainProjectile {
	out := make([]TerrainProjectile, 0, len(c.projectiles))
	for _, p := range c.projectiles {
		out = append(out, p)
	}
	return out
}
func (c *recCtx) Within(tx, ty, r int, x, y int64) bool {
	dx := x - ToFixed(float64(tx))
	dy := y - ToFixed(float64(ty))
	rr := ToFixed(float64(r))
	return dx*dx+dy*dy <= rr*rr
}

// EnemiesInRadius 复刻 TS 的几何判定：逻辑坐标中心 + 逻辑半径，
// 敌人是逻辑坐标。**两边都乘 1000 变定点再比** ——
// 与 TS 的 `e.x*1000 - cx` 同口径。
func (c *recCtx) EnemiesInRadius(x, y, r int) []TerrainEnemy {
	c.calls = append(c.calls, fmt.Sprintf("enemiesInRadius(%d,%d,%d)", x, y, r))
	cx := ToFixed(float64(x))
	cy := ToFixed(float64(y))
	rr := ToFixed(float64(r))
	out := []TerrainEnemy{}
	for _, e := range c.enemies {
		dx := ToFixed(float64(e.x)) - cx
		dy := ToFixed(float64(e.y)) - cy
		if dx*dx+dy*dy <= rr*rr {
			out = append(out, e)
		}
	}
	return out
}

func (c *recCtx) OnKill(e TerrainEnemy) {
	c.calls = append(c.calls, "onKill("+e.(*recEnemy).id+")")
}

// --- 契约 ---

func TestTerrainConstantsMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	tc := v.Terrain.Consts
	pairs := []struct {
		name     string
		got, exp int
	}{
		{"FIELD_W", FieldW, tc.FieldW},
		{"FIELD_H", FieldH, tc.FieldH},
		{"BASE_X", BaseX, tc.BaseX},
		{"BASE_Y", BaseY, tc.BaseY},
		{"OIL_BURN_MS", OilBurnMs, tc.OilBurnMs},
		{"ROTOR_RADIUS", RotorRadius, tc.RotorRadius},
	}
	for _, p := range pairs {
		if p.got != p.exp {
			t.Errorf("%s = %d，TS 期望 %d", p.name, p.got, p.exp)
		}
	}
}

// TestRotorDirMatchesCrossEndSamples 断言方向表抽样两端一致。
func TestRotorDirMatchesCrossEndSamples(t *testing.T) {
	v := loadPureVectors(t)
	if len(v.Terrain.Consts.RotorDirSample) != len(v.Terrain.Consts.RotorDirSampleIdx) {
		t.Fatalf("抽样索引与取值数量不符：%d vs %d",
			len(v.Terrain.Consts.RotorDirSampleIdx), len(v.Terrain.Consts.RotorDirSample))
	}
	for i, idx := range v.Terrain.Consts.RotorDirSampleIdx {
		if idx < 0 || idx >= len(rotorDir1000) {
			t.Fatalf("抽样下标 %d 越界（表长 %d）", idx, len(rotorDir1000))
		}
		if got := rotorDir1000[idx]; got != v.Terrain.Consts.RotorDirSample[i] {
			t.Errorf("rotorDir1000[%d] = %d，TS 期望 %d", idx, got, v.Terrain.Consts.RotorDirSample[i])
		}
	}
}

// TestRotorDirIsUnitVector 守住方向表的**性质**。
//
// 抽样只能覆盖我挑的那几个下标 —— 720 个数里手抄错一个符号，
// 抽样完全可能碰不到。这条覆盖全部。
//
// 判据：每一对 (x,y) 的模长应约为 1000（允许 ±2 的四舍五入误差）。
func TestRotorDirIsUnitVector(t *testing.T) {
	v := loadPureVectors(t)
	if got := len(rotorDir1000); got != v.Terrain.Consts.RotorDirLen {
		t.Fatalf("方向表长度 = %d，TS 期望 %d", got, v.Terrain.Consts.RotorDirLen)
	}
	if len(rotorDir1000)%2 != 0 {
		t.Fatalf("方向表长度应为偶数（x/y 成对），实际 %d", len(rotorDir1000))
	}
	bad := 0
	for i := 0; i+1 < len(rotorDir1000); i += 2 {
		angle := i / 2
		x := float64(rotorDir1000[i])
		y := float64(rotorDir1000[i+1])
		mag := math.Sqrt(x*x + y*y)
		if math.Abs(mag-1000) > 2 {
			bad++
			if bad <= 5 {
				t.Errorf("角度 %d 的方向向量模长 = %.2f（x=%d y=%d），应约为 1000",
					angle, mag, rotorDir1000[i], rotorDir1000[i+1])
			}
		}
	}
	if bad > 5 {
		t.Errorf("共 %d 个角度模长异常", bad)
	}
	// 四个正交方向必须严格取整：0°→(1000,0)、90°→(0,1000)…
	//
	// 这四条是抽样之外的独立锚点：若表整体被写错（如少了一列），
	// 它们会最先崩。
	ortho := []struct{ angle, wantX, wantY int }{
		{0, 1000, 0}, {90, 0, 1000}, {180, -1000, 0}, {270, 0, -1000},
	}
	for _, o := range ortho {
		if gotX, gotY := rotorDir1000[o.angle*2], rotorDir1000[o.angle*2+1]; gotX != o.wantX || gotY != o.wantY {
			t.Errorf("角度 %d 应为 (%d,%d)，实际 (%d,%d)", o.angle, o.wantX, o.wantY, gotX, gotY)
		}
	}
}

// TestTerrainScriptsMatchCrossEnd 重放地形脚本并逐步比对。
func TestTerrainScriptsMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, script := range v.Terrain.Rows {
		t.Run(script.Label, func(t *testing.T) {
			terr := NewTerrain(TerrainPlacement{
				Kind:  script.Placement.Kind,
				X:     script.Placement.X,
				Y:     script.Placement.Y,
				Param: script.Placement.Param,
			})
			ctx := &recCtx{
				terrainTick: mustInt64(t, script.TerrainTick),
				elementCap:  mustInt64(t, script.ElementCap),
			}
			for _, e := range script.Enemies {
				ctx.enemies = append(ctx.enemies, &recEnemy{
					id: e.ID, hp: mustInt64(t, e.HP), armor: mustInt64(t, e.Armor),
					x: e.X, y: e.Y,
				})
			}
			for _, p := range script.Projectiles {
				ctx.projectiles = append(ctx.projectiles, &recProjectile{
					id: p.ID, x: ToFixed(float64(p.X)), y: ToFixed(float64(p.Y)),
					vx: mustInt64(t, p.VX), vy: mustInt64(t, p.VY),
				})
			}

			for i, step := range script.Trace {
				extra := map[string]any{}
				switch step.Op {
				case "init":
				case "update":
					eff := terr.Update(step.DtMs, ctx)
					extra["blocked"] = eff.Blocked
					if step.Blocked != nil && *step.Blocked != eff.Blocked {
						t.Fatalf("第 %d 步 blocked = %v，TS 期望 %v", i, eff.Blocked, *step.Blocked)
					}
				case "onHit":
					r := terr.OnHit(step.Element, mustInt64(t, step.Damage), step.HitX, step.HitY)
					if step.Result == nil || *step.Result != r {
						t.Fatalf("第 %d 步 onHit = %v，TS 期望 %s", i, r, showBool(step.Result))
					}
				case "onKillCharging":
					r := terr.OnKillCharging(step.Element, mustInt64(t, step.Cap), ctx)
					if step.Result == nil || *step.Result != r {
						t.Fatalf("第 %d 步 onKillCharging = %v，TS 期望 %s", i, r, showBool(step.Result))
					}
				default:
					t.Fatalf("向量里有本文件未实现的 op %q（第 %d 步）", step.Op, i)
				}

				if terr.State != step.State {
					t.Fatalf("第 %d 步（%s）后 state = %q，TS 期望 %q", i, step.Op, terr.State, step.State)
				}
				if terr.Timer != step.Timer {
					t.Fatalf("第 %d 步（%s）后 timer = %d，TS 期望 %d", i, step.Op, terr.Timer, step.Timer)
				}
				if terr.Charge != step.Charge {
					t.Fatalf("第 %d 步（%s）后 charge = %d，TS 期望 %d", i, step.Op, terr.Charge, step.Charge)
				}
				if terr.Angle != step.Angle {
					t.Fatalf("第 %d 步（%s）后 angle = %d，TS 期望 %d", i, step.Op, terr.Angle, step.Angle)
				}
				if terr.Triggered != step.Triggered {
					t.Fatalf("第 %d 步（%s）后 triggered = %v，TS 期望 %v",
						i, step.Op, terr.Triggered, step.Triggered)
				}
				if got, want := ctx.calls, step.Calls; len(got) != len(want) ||
					strings.Join(got, " ") != strings.Join(want, " ") {
					t.Fatalf("第 %d 步（%s）后 ctx 调用 = [%s]，TS 期望 [%s]",
						i, step.Op, strings.Join(got, " "), strings.Join(want, " "))
				}
				if got, want := enemySnapshots(ctx.enemies), renderWantEnemies(step.Enemies); got != want {
					t.Fatalf("第 %d 步（%s）后敌人状态 = [%s]，TS 期望 [%s]",
						i, step.Op, got, want)
				}
				if got, want := projSnapshots(ctx.projectiles), strings.Join(step.Projectiles, " "); got != want {
					t.Fatalf("第 %d 步（%s）后弹丸状态 = [%s]，TS 期望 [%s]",
						i, step.Op, got, want)
				}
			}
		})
	}
}

// renderWantEnemies 把向量里的敌人数组渲染成与 enemySnapshots 相同的形状。
//
// ⚠️ 必须逐字对齐格式（`{id hp=.. armor=.. dead=.. flash=.. elems=[..]}`）——
//
//	只比 hp 之类的单字段会漏掉「顺序变了」与「elems 拼接方式变了」。
func renderWantEnemies(rows []struct {
	ID            string `json:"id"`
	HP            string `json:"hp"`
	ArmorPermille string `json:"armor_permille"`
	Dead          bool   `json:"dead"`
	HitFlashMs    int64  `json:"hit_flash_ms"`
	Elements      string `json:"elements"`
}) string {
	parts := make([]string, 0, len(rows))
	for _, r := range rows {
		parts = append(parts, fmt.Sprintf("{%s hp=%s armor=%s dead=%t flash=%d elems=[%s]}",
			r.ID, r.HP, r.ArmorPermille, r.Dead, r.HitFlashMs, r.Elements))
	}
	return strings.Join(parts, " ")
}

func enemySnapshots(es []*recEnemy) string {
	parts := make([]string, 0, len(es))
	for _, e := range es {
		parts = append(parts, e.snapshot())
	}
	return strings.Join(parts, " ")
}

func projSnapshots(ps []*recProjectile) string {
	parts := make([]string, 0, len(ps))
	for _, p := range ps {
		parts = append(parts, fmt.Sprintf("%s:%d,%d", p.id, p.vx, p.vy))
	}
	return strings.Join(parts, " ")
}

// TestRotorDirMatchesTsSource 直接读 terrain.ts 并逐项比对 720 个数。
//
// # 为什么这条比「抽取时小心」可靠
//
// 720 个数字手抄，错一个符号不会有任何编译期提示，
// 而后果是那个角度的偏转方向反了 → 命中位置变 → 击杀 tick 变
// → replayHash 变 → I-6 把正常对局判成**伪造**。
//
// 写「抽取脚本要小心」是纪律；纪律挡不住手抄，守卫才挡得住。
//
// ⚠️ 它读的是 **TS 源码里的字面量**，不是跑 TS 算出来的值 ——
//
//	所以它守的是「抄对了」，与跨端向量守的「算法一致」是两件事。
func TestRotorDirMatchesTsSource(t *testing.T) {
	src := readFileFrom(t, "地形源码 miniapp/src/game/terrain.ts",
		filepath.Join("miniapp", "src", "game", "terrain.ts"))

	marker := "export const ROTOR_DIR_1000: ReadonlyArray<number> = ["
	i := strings.Index(src, marker)
	if i < 0 {
		t.Fatalf("terrain.ts 里找不到 %q —— 守卫的锚点失效了（写法变了？），"+
			"不是「两边一致」", marker)
	}
	rest := src[i+len(marker):]
	end := strings.Index(rest, "]")
	if end < 0 {
		t.Fatal("ROTOR_DIR_1000 的数组没有闭合")
	}
	re := regexp.MustCompile(`-?\d+`)
	want := make([]int, 0, 720)
	for _, m := range re.FindAllString(rest[:end], -1) {
		n, err := strconv.Atoi(m)
		if err != nil {
			t.Fatalf("terrain.ts 的方向表里有非整数 %q：%v", m, err)
		}
		want = append(want, n)
	}
	if len(want) != len(rotorDir1000) {
		t.Fatalf("terrain.ts 有 %d 个数，Go 表有 %d 个", len(want), len(rotorDir1000))
	}
	mismatch := 0
	for i := range want {
		if want[i] != rotorDir1000[i] {
			mismatch++
			if mismatch <= 5 {
				t.Errorf("下标 %d（角度 %d 的 %s 分量）：terrain.ts = %d，Go = %d",
					i, i/2, map[bool]string{true: "x", false: "y"}[i%2 == 0],
					want[i], rotorDir1000[i])
			}
		}
	}
	if mismatch > 5 {
		t.Errorf("共 %d 个分量与 terrain.ts 不一致", mismatch)
	}
}

// TestTerrainBlocksProjectileSemantics 独立钉住阻挡判定。
//
// ⚠️ 契约向量覆盖不到这条：脚本里没有 `blocksProjectile` 这个 API
// （TS 的 `blocks()` 是另一处）。而它是「掩体崩塌后弹道永久改变」
// 这个机制的**唯一对外开关** —— 写反了就是「掩体永远挡路」
// 或「掩体完全不挡路」，两个都是玩法级缺陷。
func TestTerrainBlocksProjectileSemantics(t *testing.T) {
	wall := NewTerrain(TerrainPlacement{Kind: TerrainCollapseWall, X: 400, Y: 600, Param: 3})
	if !wall.BlocksProjectile() {
		t.Error("未崩塌的掩体必须阻挡弹道")
	}
	wall.OnHit("kinetic", 900, ToFixed(400), ToFixed(600)) // 9*3=27 ≥ 3 → 崩塌
	if wall.State != TerrainCollapsed {
		t.Fatalf("应已崩塌，实际 state=%q", wall.State)
	}
	if wall.BlocksProjectile() {
		t.Error("崩塌后的掩体不得阻挡弹道")
	}
	// 非掩体地形一律不阻挡
	for _, kind := range []string{TerrainOilDrum, TerrainTidalGate, TerrainRotorVane, TerrainChargeTower} {
		tr := NewTerrain(TerrainPlacement{Kind: kind, X: 1, Y: 1, Param: 1})
		if tr.BlocksProjectile() {
			t.Errorf("%s 不应阻挡弹道", kind)
		}
	}
}

// TestToFixedRoundsHalfUp 守住逻辑单位 → 定点的取整方向。
//
// ⚠️ 为什么值得单列：直接整数截断（logical*1000）在 x.5 边界上
// 与 TS 的 `Math.round` 分道扬镳。而判定半径经常落在 x.5 附近
// （radius 90 → 90000，radius 120 → 120000），
// 差 1 个单位就会让「刚好擦边命中」变成「没命中」。
func TestToFixedRoundsHalfUp(t *testing.T) {
	// ⚠️ 判据必须是**带小数的输入**。
	//
	//   我第一版全用整数（0/1/500/1000），那时 `logical*1000` 恒为整数，
	//   `math.Round` 与截断给出同一个值 —— 变异验证把 Round 换成 Trunc
	//   后**测试没红**。守卫能通过，但证明不了任何东西。
	//
	//   整数输入下取整是恒等操作，写不写都一样；
	//   只有小数让乘积落在 .5 上时两种写法才分道扬镳。
	//
	//   要让 ×1000 之后以 .5 结尾，logical 必须以 .0005 结尾。
	cases := []struct {
		in   float64
		want int64
	}{
		{0, 0},
		{1, 1000},
		{500, 500000},
		{1000, 1000000},
		// .5 边界：JS 的 Math.round 是四舍五入，**不是**银行家舍入。
		// {0.0025, 3} 正是区分两者的那一条：银行家舍入会给 2。
		{0.0005, 1},
		{0.0015, 2},
		{0.0025, 3},
		{123.4565, 123457},
		{123.4564, 123456},
		// 负数：JS Math.round(-0.5) = -0（不是 -1），Go math.Round 同为 -0。
		{-0.0005, -1},
		{-0.0015, -2},
	}
	for _, c := range cases {
		if got := ToFixed(c.in); got != c.want {
			t.Errorf("ToFixed(%v) = %d，期望 %d —— 取整方式与 TS 的 Math.round 不一致",
				c.in, got, c.want)
		}
	}
}
