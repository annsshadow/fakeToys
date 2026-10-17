package domain

import "math"

// GameTuning 是战斗节奏集中配置（第 158 轮「游戏节奏」整改的 Go 侧镜像）。
//
// # 为什么必须存在（第 159 轮事故的根因链之一）
//
// TS 侧第 158 轮引入 tuning.ts：所有影响手感的**时间与速度**参数集中
// 到 `cfg.tuning`（缺省 DEFAULT_TUNING）。但 Go 侧**从未同步** ——
// 服务端重算一直用「整改前」的编译期常量（波次延迟/间隔不缩放、
// 弹速 1.0、过热 2000ms…）。
//
// 后果：客户端（DEFAULT_TUNING）与重算（无 tuning）生成的时间线不同 →
// 即便双端选牌序列完全一致，`replayHash` 也必然不同 → **所有结算被拒**。
// 与「选牌时钟冻结」并列的 P0 根因，而且同样是**静默**的：
// 存量守卫用的 replay 向量是 158 轮前的旧产物（没有重生成），
// Go 与旧向量互相自洽，全体测试绿灯。
//
// # 逐字段与 tuning.ts 对齐
//
// ⚠️ 缺省值必须与 miniapp/src/game/tuning.ts 的 DEFAULT_TUNING 逐位一致，
// 并由跨端契约测试锁住（testdata/formula_vectors.json 的 `tuning` 段）。
// 两端漂移的后果与 card_picks 相同：静默、且现网表现为全体玩家被拒。
//
// 单位约定与 TS 相同：倍率一律**千分比**（1000 = 不变），时间用毫秒。
type GameTuning struct {
	// SpawnProgressStep 是敌人出场淡入的每 tick 进度增量（千分比）。
	SpawnProgressStep int64
	// EnemySpeedPermille 是敌人移动速度倍率。
	EnemySpeedPermille int64
	// ProjectileSpeedPermille 是弹丸飞行速度倍率（纯飞行表现，
	// 不影响任何伤害结算，只压缩「出膛到命中」的等待）。
	ProjectileSpeedPermille int64
	// SkillCooldownPermille 是技能冷却倍率。
	SkillCooldownPermille int64
	// WaveDelayPermille 是波内「首只出场延迟」倍率。
	WaveDelayPermille int64
	// WaveIntervalPermille 是波内「同组刷怪间隔」倍率。
	WaveIntervalPermille int64
	// EnemyAttackIntervalPermille 是远程敌人攻击间隔倍率。
	EnemyAttackIntervalPermille int64
	// OverheatDurationMs 是过热持续毫秒。
	OverheatDurationMs int64
	// HitFlashMs 是受击闪白时长（毫秒，纯表现）。
	HitFlashMs int64
	// FloatLifeMs 是伤害飘字寿命（毫秒，纯表现）。
	//
	// ⚠️ Go 引擎不生成飘字（`Floats` 只被测试直接构造），
	// 这个字段只为「结构完整 + 契约锁定」存在，没有任何运行时消费点。
	FloatLifeMs int64
}

// DefaultTuning 必须与 TS 的 DEFAULT_TUNING 逐位一致。
//
// 取值来源与论证见 miniapp/src/game/tuning.ts —— 那里记录了每一档是
// 怎么扫参定的、以及为什么 enemySpeed / skillCooldown 最终保持 1000。
// 本文件只镜像数值；要动手先读那边的注释（尤其是 spawnProgressStep 的
// 「不许改」铁律：FULL/STEP = 12.5 不能整除，改了会让「敌人何时可被
// 击中」平移一格，所有存量战报失配）。
var DefaultTuning = GameTuning{
	SpawnProgressStep:           SpawnProgressStep, // 80
	EnemySpeedPermille:          1000,
	ProjectileSpeedPermille:     1200,
	SkillCooldownPermille:       1000,
	WaveDelayPermille:           800,
	WaveIntervalPermille:        850,
	EnemyAttackIntervalPermille: 1000,
	OverheatDurationMs:          1600,
	HitFlashMs:                  110,
	FloatLifeMs:                 700,
}

// resolveTuning 解析配置里的 tuning（nil = DefaultTuning）。
//
// 与 TS `cfg.tuning ?? DEFAULT_TUNING` 同义。
func resolveTuning(t *GameTuning) GameTuning {
	if t == nil {
		return DefaultTuning
	}
	return *t
}

// scaledMs 复刻 TS 的 `Math.max(1, Math.round((ms * permille) / 1000))`。
//
// ⚠️ 语义必须逐位一致：先乘后除、浮点中间值、四舍五入取整、下限 1。
//
//	TS 的 Math.round 对正数是「.5 向上」；Go math.Round 是「.5 远离零」——
//	输入恒为正，两者一致。
func scaledMs(ms, permille int64) int64 {
	v := int64(math.Round(float64(ms) * float64(permille) / 1000))
	if v < 1 {
		v = 1
	}
	return v
}

// scaledDelayTicks 复刻 TS 的
// `Math.max(1, Math.ceil((ms * permille) / 1000 / TICK_MS))`。
//
// 用 float64 而不是整数运算，是为了与 TS 的 IEEE 754 双精度结果**逐位**
// 一致（含「真实值恰好整数但浮点表示略偏」的边界）—— 两端算出的 tick
// 只要差 1，整局时间线就整体错位。
func scaledDelayTicks(ms, permille int64) int64 {
	v := int64(math.Ceil(float64(ms) * float64(permille) / 1000 / float64(TickMs)))
	if v < 1 {
		v = 1
	}
	return v
}
