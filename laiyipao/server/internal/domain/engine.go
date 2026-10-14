package domain

// 战斗引擎的**骨架**（R1 Block A）。
//
// # 这个文件只做框架，不做战斗
//
// 它包含：全部实体类型（字段与 TS 一一对应）、`BattleEngine` 的全部字段、
// `Step` 的**完整 tick 序**、`Start`、`Emit`/`Record`/`DrainEvents`、
// `CheckStalemate`、`Finish`、`CalcStars`。
//
// 它**不包含**：刷怪、敌人移动、开火、弹道、地形、卡牌 —— 那些是 B~F。
// 那些方法在这里是空实现，`Step` 按最终顺序调用它们。
//
// # 为什么可以这样做
//
// 因为 A 阶段的验收**不是**「事件流与 TS 一致」（那时必然不一致，
// 因为 `start()` 要调的 `beginWave` 还��空实现）。
// A 阶段验的是**排序不变量**：tick 怎么长、停滞在哪一刻触发、
// 终局后还推不推进 tick。这些不依赖 B~F，且错了会立刻出事。
//
// 第一次真正的跨端逐事件对齐在 B。详见 `docs/audits/r1-engine-port-plan.md`。

import "strconv"

// tick 节奏常量。
//
// ⚠️ **TickMs 不在这里定义** —— 它已在 `battle.go` 里（校验侧），
//
//	而 `TestTickMsMatchesClient` 锁的是「与 miniapp 的 TICK_MS 一致」。
//	再定义一份就是第二个真源：改了一处另一处不动，
//	于是「上限校验用的 tick 长度」与「引擎实际推进的 tick 长度」悄悄分叉。
//
//	这里只补 battle.go 没有的两个。
//
// ⚠️ TickMs 必须是**整数**。TS 的注释记着：引擎里到处是
// `BigInt(TICK_MS)`，若它是 1000/60 = 16.666 这种小数，
// `BigInt()` 会直接抛 RangeError。
const (
	// TickHz 是每秒 tick 数。
	TickHz = 20
	// MaxBattleTicks 是战斗终止的 tick 上限。
	//
	// 30000 tick = 1500s = 25 分钟，是**兜底**而不是预期时长。
	//
	// 预算口径：实测最难打通的关卡（第 100 关）约 594s = 11880 tick，
	// 30000 是它的 2.5 倍安全余量；触发即说明游戏逻辑出现了
	// 「双方都无法推进」的死循环。
	//
	// 同时用于防刷分：客户端上报的 duration_ms 不能超过 1800s，
	// 否则被服务端直接判非法。
	MaxBattleTicks = 30000
)

// 战斗阶段。
const (
	PhasePrepare    = "prepare"
	PhaseWave       = "wave"
	PhaseCardSelect = "card_select"
	PhaseWon        = "won"
	PhaseLost       = "lost"
)

// 重放事件类型。
//
// ⚠️ 这些字符串进 `replay_hash`，**改名即全量失配**。
//
// 而失配的表现是「验真失败」—— 看起来像作弊。
const (
	ReplayFire      = "fire"
	ReplayHit       = "hit"
	ReplayReaction  = "reaction"
	ReplayKill      = "kill"
	ReplayLeak      = "leak"
	ReplayWave      = "wave"
	ReplayOverheat  = "overheat"
	ReplayCard      = "card"
	ReplayTerrain   = "terrain"
	ReplayGuard     = "guard"
	ReplayStalemate = "stalemate"
)

// Buffs 是局内由属性卡/机制卡累积的加成。
type Buffs struct {
	AttackPermille      int64
	ElementCoefPermille int64
	CritPermille        int64
	ElementCapBonus     int64
	ArmorPermille       int64
	PierceBonus         int
	ChainBonus          int
	AoeBonus            int
	FreeDiscard         int
	OverheatGuard       int
}

// NewBuffs 返回全零加成。
func NewBuffs() Buffs { return Buffs{} }

// Enemy 是战斗中的敌人实例（字段与 TS `types.ts` 的 `Enemy` 一一对应）。
//
// ⚠️ 字段**齐但不实现行为**：`ApplyElement` 在 C 块实现（元素夹紧规则）。
//
//	这里只保留数据形状，让后续块不必再改结构体 ——
//	结构体一旦改形状，之前所有块的白就要重新跑一遍。
type Enemy struct {
	UID   int64
	DefID int
	Name  string
	Cat   string

	// X / Y 是定点坐标（×1000）。
	X, Y   int64
	HP     int64
	MaxHP  int64
	Shield int64
	// ArmorPermille 是护甲千分比。
	ArmorPermille int64
	FlyHeight     int64
	Burrow        bool
	IsBoss        bool
	// Resist / Stacks 是「元素 → 抗性/层数」。
	//
	// 用 map 而不是固定长度数组：元素集合是 5 个具名常量，
	// 但 `nil` 与「空 map」在「取不到就是 0」的语义下等价，
	// 固定数组反而要处理「越界索引」这个不该存在的问题。
	Resist map[Element]int64
	Stacks map[Element]int64

	// Speed 是移动速度（定点）。
	Speed          int64
	Attack         int64
	AttackRange    int64
	AttackInterval int64
	AttackCooldown int64

	// 状态效果剩余毫秒。
	FrozenMs  int64
	StunnedMs int64
	SlowedMs  int64

	// AmplifyPermille 是伤害放大千分比（雷姆/电桩等乘区）。
	AmplifyPermille int64
	// Knockback 是击退位移。
	Knockback int64
	// ArmorShredMs / ArmorShredPermille 是破甲效果；
	// ArmorShredMs>0 时 hitEnemy 构造 Defender 时还要叠加当前值。
	ArmorShredMs       int64
	ArmorShredPermille int64

	Dead bool
	// SpawnProgress 是生成进度 0..1（千分比）。
	SpawnProgress int64
	HitFlashMs    int64
}

// Projectile 是飞行中的弹丸。
type Projectile struct {
	UID         int64
	SkillID     int
	Element     Element
	X, Y        int64
	VX, VY      int64
	Damage      int64
	ApplyStacks int64
	PierceLeft  int
	ChainLeft   int
	// HitSet 是已命中的 uid。⚠️ **只增不减** —— 这是有意的：
	// 同一次穿透不得重复计入，所以「没有它」与「它被清空」行为不同。
	HitSet    map[int64]bool
	AoeRadius int64
	Dead      bool
	TrailLen  int
	Slot      int
}

// FloatText 是浮动文字（纯表现层，**不进 replay**）。
type FloatText struct {
	X, Y      int64
	Text      string
	Color     string
	LifeMs    int64
	MaxLifeMs int64
	Size      int
}

// ReplayEvent 是重放事件流的一条记录。
//
// ⚠️ 它是 `replay_hash` 的原料，所以**字段顺序与语义都不能改**。
type ReplayEvent struct {
	T    int64
	Type string
	A    int64
	B    int64
	C    int64
}

// BattleEvent 是给渲染层的事件。
//
// 与 ReplayEvent 刻意分开：渲染事件是**全量**的（含纯表现），
// 而重放事件**只记「影响战果的事实」**。
// 混在一起会让渲染层的任何改动都影响哈希。
type BattleEvent struct {
	Type string
	X    int64
	Y    int64
	// Damage 是漏怪事件的伤害值（C 块）。
	//
	// ⚠️ 它**只在 replay 里**，不在渲染事件里 ——
	//   漏怪的实际扣血已由 `BaseHP` 记录，渲染层再带一份就成了
	//   「同一个事实在两处各有一份」，而两处可能不同步。
	Damage int64
	Score  int64
	Stars  int
	Text   string
	Color  string
	Size   int
}

// 渲染/重放事件类型。
const (
	EventStalemate = "stalemate"
	EventWon       = "won"
	EventLost      = "lost"
)

// SpawnItem 是刷怪队列的一项。
type SpawnItem struct {
	EnemyID int
	AtTick  int64
	UID     int64
}

// EquippedSkill 是已装备的技能实例（局内可变副本）。
//
// ⚠️ 与 `SeedSkill`（内容表）是**不同**的东西：这里带运行时状态
// （冷却剩余）与选牌施加的升格。
type EquippedSkill struct {
	SkillID         int
	Name            string
	Element         Element
	Kind            string // active / passive
	HeatCost        int64
	CooldownMs      int64
	Pierce          int
	AoeRadius       int64
	BaseDamage      int64
	ApplyElement    Element
	ApplyStacks     int64
	ProjectileSpeed int64
	Chain           int
	Slot            int
	// CooldownRemaining 是冷却剩余毫秒。
	CooldownRemaining int64
}

// BattleConfig 是引擎的构造参数。
type BattleConfig struct {
	Level    GeneratedLevel
	Enemies  map[int]SeedEnemy
	Skills   map[int]SeedSkill
	Equipped []EquippedSkill
	Attacker Attacker
	// Seed 是战斗种子。
	Seed int64
	// ScoreRules 是分数规则。
	ScoreRules ScoreRules
	// ActiveSlots 是主动插槽数。
	ActiveSlots int
}

// BattleEngine 是战斗引擎。
//
// # 字段与 TS 的 BattleEngine 一一对应，**包括尚未使用的**
//
// 那些字段在 B~F 才被读写。保留它们是为了：后续块不需要改结构体形状，
// 而结构体一改形状，A 阶段验过的排序不变量就要重跑。
type BattleEngine struct {
	Cfg BattleConfig
	// Rng 是战斗 RNG。⚠️ **必须在构造时由 cfg.Seed 一次性定下** ——
	//
	//	它的状态决定刷怪洗牌、敌人 y 坐标、卡牌抽取……
	//	任何「用到时才构造」或「每次调用重建」的写法都会让重放不可复现，
	//	症状是「同一个种子重放两次得到不同哈希」。
	Rng   *LCG
	Heat  HeatSystem
	Deck  CardDeck
	Buffs Buffs

	// ActiveSlots 是主动插槽数（缺省 4，由 NewBattleEngine 定下）。
	//
	// ⚠️ 缺省必须是 4：改它会让所有 4 槽战报的重放哈希失配，
	//	而绝大多数战报都是 4 槽的。
	ActiveSlots int

	Phase     string
	Tick      int64
	ElapsedMs int64
	WaveIndex int

	BaseHP            int64
	BaseHPMax         int64
	BaseArmorPermille int64

	Enemies     []*Enemy
	Projectiles []*Projectile
	Floats      []FloatText
	Terrains    []*Terrain
	Skills      []EquippedSkill

	Shots          int64
	Hits           int64
	Kills          int64
	Leaked         int64
	ReactionsCount int64
	Score          int64
	HeatMax        int64

	ElementsUsed  map[string]int64
	ReactionsUsed map[string]int64
	TerrainUsed   []string

	// CardPicks 是每波选中的手牌索引（-1 = 整波跳过）。
	//
	// ⚠️ 这是 I-6 重放闭环的最后一环：选牌会改变后续战斗
	// （技能升格 / 属性加成 / 机制词条），第三方拿不到这个序列
	// 就复现不出原局，验真会把正常对局判成伪造。
	CardPicks []int

	events     []BattleEvent
	replay     []ReplayEvent
	uidSeq     int64
	spawnQueue []SpawnItem

	// ReplayScript 是外部注入的选牌序列；nil = 交互模式。
	ReplayScript    []int
	ReplayScriptPos int
	cardSelectDone  bool
	// discardedThisWave 标记「本波弃过牌」——
	// 编码只对**当前波**的 card_picks 项有意义。
	discardedThisWave bool

	// TotalEnemies 是本关总怪数（结算上限校验用）。
	TotalEnemies int
}

// NewBattleEngine 构造引擎。
//
// ⚠️ `Seed` 与 `elapsedMs` 等**不**在这里做任何随机化 ——
//
//	引擎必须是**纯函数**：`NewBattleEngine(cfg).Step()` 的结果
//	只由 cfg 决定，与调用时刻、调用次数、宿主语言都无关。
//	任何「构造时偷跑一次 RNG」都会让重放不可复现。
func NewBattleEngine(cfg BattleConfig) *BattleEngine {
	// ⚠️ `cfg.Equipped` 必须**复制**：
	//
	//	F 块的 `applyCard` 会给技能升格（改 BaseDamage），
	//	若共享同一批结构体，调用方持有的配置会被就地污染 ——
	//	于是「同一个配置跑第二局」的结果与第一局不同。
	//
	//	这类污染的表现极难查：第一次跑是对的，第二次开始错，
	//	而「我用的是同一个配置」恰好是所有人都会做的假设。
	skills := make([]EquippedSkill, len(cfg.Equipped))
	copy(skills, cfg.Equipped)
	for i := range skills {
		// 运行时副本的冷却必须归零：
		// 配置里的 cooldownRemaining 恒为 0，但复制过来后必须显式写 0 ——
		// 否则将来有人往配置里塞了个值，它就会静默生效。
		skills[i].CooldownRemaining = 0
	}
	// ⚠️ 复活币加在**当前**血量上，`BaseHPMax` 保持关卡原值。
	//
	//	反过来写（抬高 BaseHPMax）会把漏怪伤害一起抬高，等于什么都没给 ——
	//	漏怪代价是 `baseHpMax / LeakDamageDivisor`，抬高分母就是抬高代价。
	baseHPMax := cfg.Level.BaseHP

	// ⚠️ 总怪数必须**从关卡数出来**，不能由调用方传入。
	//
	//	它是三个判定共用的分母：`checkWaveEnd` 的「空关卡不秒胜」、
	//	结算的 `kills + leaked >= total`、以及上限校验。
	//	让调用方自己填的话，任何一处忘记填或填错都是「静默放宽」——
	//	比如 total=0 会让 `kills(0) >= 0` 成立，也就是发钱。
	total := 0
	for _, w := range cfg.Level.Waves {
		for _, sp := range w.Spawns {
			total += sp.Count
		}
	}

	// ⚠️ 缺省必须是 4（ActiveSlots）—— 改这个默认值会让所有 4 槽战报的
	// 重放哈希失配（绝大多数战报都是 4 槽的）。
	// 只有玩家在专精树点了「额外插槽」时才会 > 4，
	// 那个值由服务端 `/mastery` 的 `total_slots` 下发。
	activeSlots := cfg.ActiveSlots
	if activeSlots == 0 {
		activeSlots = ActiveSlots
	}

	heat := NewHeatSystem()
	// 热量上限加成（专精 heat_cap + 宝石 gem_heat）。
	//
	// ⚠️ 放在构造时注入一次而不是每次读 cap 时加：
	// heat_cap 是本局常量，而 Heat 的 capBonus 是可变状态。
	heat.CapBonus = cfg.Attacker.HeatCapPermille

	return &BattleEngine{
		Cfg:   cfg,
		Rng:   newLCG(cfg.Seed),
		Heat:  heat,
		Deck:  NewCardDeck(),
		Buffs: NewBuffs(),
		Phase: PhasePrepare,
		// ⚠️ `ElementsUsed` / `ReactionsUsed` 必须是**新 map**：
		// 直接复用 Cfg 里的（若将来加了）会导致两次运行共享统计。
		ElementsUsed:  map[string]int64{},
		ReactionsUsed: map[string]int64{},
		Skills:        skills,
		BaseHP:        baseHPMax,
		BaseHPMax:     baseHPMax,
		// ⚠️ `|| 0` 而不是直接取值：level 是**网络数据**，
		// 缺 armor_permille 时 TS 的 `|| 0` 给出 0，
		// 而 Go 的零值本来就是 0 —— 语义一致，但显式写出来是为了
		// 说明「这里故意不做校验，畸形数据不该崩掉整场战斗」。
		BaseArmorPermille: cfg.Level.ArmorPermille,
		TotalEnemies:      total,
		ActiveSlots:       activeSlots,
		uidSeq:            1,
	}
}

// SetReplayScript 注入重放脚本。传 nil 回到交互模式。
func (e *BattleEngine) SetReplayScript(picks []int) {
	e.ReplayScript = picks
	e.ReplayScriptPos = 0
}

// HasReplayScript 报告是否处于重放脚本模式。
//
// 外部驱动循环（如 `replay.go` 的 RunToEnd）必须先问这个：
// 脚本模式下**不能**自己调 SkipCards()，否则会抢在引擎的脚本决策之前
// 把手牌丢掉 —— 表现为「脚本看起来完全没生效」。
func (e *BattleEngine) HasReplayScript() bool { return e.ReplayScript != nil }

// Start 开战。
func (e *BattleEngine) Start() {
	e.Phase = PhaseWave
	e.BeginWave(0)
}

// DrainEvents 取走并清空渲染事件队列。
func (e *BattleEngine) DrainEvents() []BattleEvent {
	out := e.events
	e.events = nil
	return out
}

// ReplayEvents 返回重放事件流的副本。
func (e *BattleEngine) ReplayEvents() []ReplayEvent {
	out := make([]ReplayEvent, len(e.replay))
	copy(out, e.replay)
	return out
}

func (e *BattleEngine) emit(ev BattleEvent) { e.events = append(e.events, ev) }

// record 记一条重放事件。
//
// ⚠️ b/c 默认 0 —— TS 的默认参数。省略它们与显式传 0 结果相同，
// 但写出来是为了让「这条事件到底记了几个数」在代码上可见。
func (e *BattleEngine) record(t int64, typ string, a, b, c int64) {
	e.replay = append(e.replay, ReplayEvent{T: t, Type: typ, A: a, B: b, C: c})
}

// Step 推进一个 tick。
//
// # ⚠️ 这里的**位置**就是契约，顺序错了会静默失配
//
// 三条硬约束（TS 注释已点明，此处逐条对齐）：
//
//  1. `cardSelectDone` 收尾必须在 `applyReplayDecision` **之前**。
//     否则同一 tick 内 `applyReplayDecision` 会被再调一次，
//     消费 script 的下一项，把下一波的决策当成本波的第二张牌
//     （实测 picks 记成 [-3,-3,-1,-1]，事件流多一条原局没有的记录）。
//
//  2. `applyReplayDecision` 必须在 `tick++` **之前**。
//     tick 会写进每条 replay 事件，多一格会让后续所有时间戳整体偏移
//     → 哈希必然不同。原局里玩家点牌同样不推进 tick，两者必须严格一致。
//
//  3. `checkStalemate` 必须在 `tick++` **之后**、任何战斗逻辑**之前**。
//     这样「到上限」这件事本身是这次 tick 的第一个事实。
func (e *BattleEngine) Step() []BattleEvent {
	if e.Phase == PhaseWon || e.Phase == PhaseLost {
		return e.DrainEvents()
	}

	// (1) 待结束的本波在这里收尾。
	if e.cardSelectDone {
		e.cardSelectDone = false
		e.BeginWave(e.WaveIndex + 1)
	}

	// (2) 重放决策必须在 tick++ 之前。
	if e.Phase == PhaseCardSelect && e.ReplayScript != nil {
		e.applyReplayDecision()
		// 决策可能已推进到下一波（deck 空时 takeCard 会调 BeginWave），
		// 也可能仍是 card_select（脚本要求再选一张）。两种情况都继续本 tick。
	}

	e.Tick++
	e.ElapsedMs += TickMs

	// (3) 停滞兜底：tick++ 之后、任何战斗逻辑之前。
	if e.CheckStalemate() {
		return e.DrainEvents()
	}

	e.Heat.Update(TickMs)
	e.heatTickCooldown()

	if e.Phase == PhaseWave {
		e.updateSpawns()      // B
		e.updateEnemies()     // C
		e.updateAutoFire()    // D
		e.updateProjectiles() // D
		e.updateTerrain()     // E
		e.updateFloats()
		e.checkWaveEnd() // B / F
	}

	e.HeatMax = e.Heat.MaxHeatThisBattle

	return e.DrainEvents()
}

// heatTickCooldown 推进所有技能冷却。
func (e *BattleEngine) heatTickCooldown() {
	targets := make([]CooldownTarget, 0, len(e.Skills))
	for _, s := range e.Skills {
		targets = append(targets, CooldownTarget{
			CooldownMs:        s.CooldownMs,
			CooldownRemaining: s.CooldownRemaining,
		})
	}
	e.Heat.TickCooldown(TickMs, targets)
	// 写回：TickCooldown 收的是值切片，改动不会自动回落到技能上。
	for i := range e.Skills {
		e.Skills[i].CooldownRemaining = targets[i].CooldownRemaining
	}
}

// CheckStalemate 报告是否已达 tick 上限。
//
// 返回 true 只支撑**兜底**：`Step` 开头的 `if (won||lost) return` 已经挡住了
// 常规终局，而这里要在**那之后**触发。双重判定是必需的：
// 单靠 `phase` 判定不到「tick 耗尽但游戏逻辑认为还没结束」的状态。
func (e *BattleEngine) CheckStalemate() bool {
	if e.Phase == PhaseWon || e.Phase == PhaseLost {
		return false
	}
	if e.Tick < MaxBattleTicks {
		return false
	}
	e.emit(BattleEvent{Type: EventStalemate})
	e.Finish(false, true)
	return true
}

// Finish 结束战斗。
//
// ⚠️ 停滞与普通战败**必须走完全相同的终局路径**：
// 停滞同样要记一条**真实**的重放事件。若两边分叉，
// 从「看起来像普通战败」与「看起来像卡住不动」之间
// 挑一个描述「既不普通也不卡住」的状态，会让**无响应**看起来像普通战败，
// 而排查方向完全不同。
func (e *BattleEngine) Finish(win bool, stalemate bool) {
	if win {
		e.Phase = PhaseWon
	} else {
		e.Phase = PhaseLost
	}
	stars := e.CalcStars()
	if win {
		e.emit(BattleEvent{Type: EventWon, Score: e.Score, Stars: stars})
	} else {
		e.emit(BattleEvent{Type: EventLost, Score: e.Score})
	}
	if stalemate {
		e.record(e.Tick, ReplayStalemate, e.Tick, 0, 0)
	}
}

// CalcStars 计算星级。
func (e *BattleEngine) CalcStars() int {
	return starsForScore(e.Score, e.Cfg.Level.StarTargets)
}

func starsForScore(score int64, targets []int64) int {
	stars := 0
	for _, t := range targets {
		if score >= t {
			stars++
		}
	}
	return stars
}

// nextUID 分配下一个实体 UID。
//
// ⚠️ UID 进 replay 事件的 `a` 位，所以分配顺序必须完全确定 ——
//
//	任何「按需发号」「从 map 迭代顺序取号」的写法都会让哈希不可复现。
func (e *BattleEngine) nextUID() int64 {
	id := e.uidSeq
	e.uidSeq++
	return id
}

// --- 以下是 C~F 的空实现 ---
//
// ⚠️ 刻意集中在这里，并在每个上面写清「由哪一块替换」。
//   分散到各处会让「哪些还没搬」这件事看不出来 ——
//   而一个看不出完成度的移植，最容易在半成品状态下被人当成完成品。
//
// B 块（波次与刷怪）已实现在 `engine_wave.go`，故此处不再有空壳。
// C 块（敌人推进与漏怪）已实现在 `engine_enemy.go`，故此处不再有空壳。
// D 块（开火与弹道）已实现在 `engine_combat.go`，故此处不再有空壳。

// updateTerrain 推进地形（E 块实现）。
func (e *BattleEngine) updateTerrain() {}

// updateFloats 推进浮动文字。
//
// ⚠️ 这是纯表现层，**不进 replay**，所以它可以在 A 块就完整实现 ——
//
//	唯一要保证的是它不会碰任何战斗状态。
func (e *BattleEngine) updateFloats() {
	kept := e.Floats[:0]
	for _, f := range e.Floats {
		f.LifeMs -= TickMs
		if f.LifeMs > 0 {
			kept = append(kept, f)
		}
	}
	e.Floats = kept
}

// checkWaveEndUnused 是被 B 块取代的空壳，保留一个转发会让
//
//	「实现在哪个文件」这件事有两个答案。
func (e *BattleEngine) checkWaveEndUnused() {}

// applyReplayDecision 按注入的脚本决策取牌（F 块实现）。
func (e *BattleEngine) applyReplayDecision() {}

// ReplaySegmentA 是 A 段（攻方系数锚点）。
//
// ⚠️ 空实现 —— 完整实现在 G 块（D 完成后 currentAttacker 才存在）。
//
//	这里先给出函数骨架，让 `Step` 的调用点在 A 阶段就能编译通过。
func (e *BattleEngine) ReplaySegmentA() string { return "" }

// hexInt64 把整数格式化为十进制（重放分段用）。
func hexInt64(v int64) string { return strconv.FormatInt(v, 10) }
