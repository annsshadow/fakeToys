package domain

import (
	"errors"
	"fmt"
	"time"
)

// 战斗结算校验（防作弊）。
//
// 我们刻意**不在服务端跑战斗引擎** —— 那需要 Go/TS 双引擎逐位对齐，
// 成本与漂移风险都不可接受。改用行业标准的"上限校验"：
//
//  1. battle_token：开局下发一次性凭证，结算必须匹配未使用未过期的 token，
//     从根上杜绝重放同一份战报反复领奖。
//  2. 交叉约束：击杀数 ≤ 关卡总怪数、命中率 ≤ 100%、最短时长、分数速率上限。
//  3. 掉落封顶：按关卡理论上限裁剪。
//
// 而"分数正确性"由 I-6 的回放哈希保证 —— 任何人可用 battle_id 本地重放证伪。
// 两者是互补的：上限校验挡住极端构造，哈希提供可验证性。

var (
	// ErrTokenNotFound 表示 token 不存在。
	ErrTokenNotFound = errors.New("战斗凭证不存在")
	// ErrTokenUsed 表示 token 已被使用（重放）。
	ErrTokenUsed = errors.New("战斗凭证已被使用")
	// ErrTokenExpired 表示 token 已过期。
	ErrTokenExpired = errors.New("战斗凭证已过期")
	// ErrLevelMismatch 表示结算的关卡与凭证不符。
	ErrLevelMismatch = errors.New("结算关卡与战斗凭证不符")
	// ErrTooManyKills 表示击杀数超过关卡怪物总数。
	ErrTooManyKills = errors.New("击杀数超过该关卡敌人总数")
	// ErrTooShort 表示战斗时长不合理（秒通）。
	ErrTooShort = errors.New("战斗时长过短，数据不可信")
	// ErrScoreRateExceeded 表示得分速率超过上限。
	ErrScoreRateExceeded = errors.New("得分速率超过上限")
	// ErrInvalidHitRate 表示命中率越界。
	ErrInvalidHitRate = errors.New("命中率越界")
	// ErrWaveExceeded 表示波次超过关卡总波次。
	ErrWaveExceeded = errors.New("波次超过该关卡总波数")
	// ErrTooManyReactions / ErrTooManyLeaked / ErrInvalidField 用于上报字段的上界校验。
	// 缺了它们，客户端可以把任意大的值塞进任务进度与成就进度。
	ErrTooManyReactions = errors.New("反应次数超过理论上限")
	ErrTooManyShots     = errors.New("发射数超过该时长的物理上限")
	ErrTooManyLeaked    = errors.New("漏怪数超过该关总怪数")
	ErrInvalidField     = errors.New("上报字段非法")
)

// MaxReactionsPerHit 是一次开火最多能触发的反应次数：
// 主目标 1 次 + 溅射范围内至多 N-1 个额外目标。
// 取 8 覆盖当前最宽的溅射覆盖，留一点余量。
const MaxReactionsPerHit = 8

// MaxReactionsPerEnemy 是一整局里单只敌人最多能参与的反应次数上界。
// 一只敌人身上同时挂着 5 种元素、每种至多 element_cap 层；
// 对同一种元素再次施加同种元素不触发反应（LookupReaction 排除同元素），
// 所以每种元素最多贡献 element_cap 次反应机会。
// 取 5 元素 × 4 层 = 20 作为宽松上界。
const MaxReactionsPerEnemy = 20

// MaxShotsPerTickSlack 是每 tick 发射数上界相对「技能数」的余量倍数。
// 引擎的 updateAutoFire 对每个已装备技能每 tick 试射一次（主动槽 4 个），
// 留 8 倍余量给手动射击、弹道飞行中补射与未来改动。
const MaxShotsPerTickSlack = 8

// TickMs 是战斗逻辑步长（毫秒）。
//
// ⚠️ 必须与 miniapp/src/game/engine.ts 的 `export const TICK_MS = 50` 一致。
// 它决定两件事：回放重放的 tick 推进节奏（I-6 的哈希依赖它），
// 以及 maxShotsFor 推导出的发射数上界。
// 只改一边不改另一边，要么让所有已存战报的哈希对不上，
// 要么让上界放得比预想的松。TestTickMsMatchesClient 锁住这一点。
const TickMs = 50

// MaxReportedHeat 是允许上报的峰值热量上限。
// 真实值不会超过 HEAT_MAX(100) + 卡牌加成；留 10 倍余量后仍是硬上界。
const MaxReportedHeat = 1000

// fullStarTarget 返回最高一档星级门槛，缺档时返回 0。
//
// ⚠️ 取代原来的 `gl.StarTargets[len(gl.StarTargets)-1]`。
// 那个裸索引在空切片上会 panic（index out of range），而 recoverPanic
// 会把整场请求转成 500「服务内部错误」—— 攻击者用一个畸形关卡就能
// 稳定触发，且日志里只有一句"panic"。
//
// 当前不可达：StartBattle/SettleBattle 都走 domain.GenerateLevel() 现算，
// 生成器必定填 3 档。但关卡是网络数据、且 stats.go 提供了后台改关卡的入口，
// 所以这里必须有保护。
func fullStarTarget(gl GeneratedLevel) int64 {
	if len(gl.StarTargets) == 0 {
		return 0
	}
	return gl.StarTargets[len(gl.StarTargets)-1]
}

// maxShotsFor 返回给定时长下的物理发射数上界。
//
// ⚠️ 必须对 int64 溢出保持 fail-closed。
// 时长来自客户端上报，DurationMs 已经过 `> 0` 校验但没有上界，
// durationMs = 2^62 时除法结果仍很大，再乘 8 会溢出成负数 ——
// 负数比较会让所有上报都通过，那比没有校验更糟。
// 这里先夹到 24 小时（远超任何真实对局），保证后续运算不溢出。
func maxShotsFor(durationMs int) int64 {
	const maxPlausibleMs = 24 * 60 * 60 * 1000
	if durationMs > maxPlausibleMs {
		durationMs = maxPlausibleMs
	}
	if durationMs <= 0 {
		return 0
	}
	return (int64(durationMs)/TickMs+1)*MaxShotsPerTickSlack + MaxShotsPerTickSlack
}

// BattleToken 是一次战斗的凭证。
type BattleToken struct {
	ID        int64
	UserID    int64
	LevelID   int
	Seed      int64
	IssuedAt  time.Time
	ExpiresAt time.Time
	UsedAt    *time.Time
}

// IsUsable 判断 token 当前是否可用。
func (t BattleToken) IsUsable(now time.Time) error {
	if t.UsedAt != nil {
		return ErrTokenUsed
	}
	if now.After(t.ExpiresAt) {
		return ErrTokenExpired
	}
	return nil
}

// SettleInput 是客户端上报的结算数据。
type SettleInput struct {
	Result        string         `json:"result"` // win / lose
	Stars         int            `json:"stars"`
	Score         int64          `json:"score"`
	Kills         int            `json:"kills"`
	Leaked        int            `json:"leaked"`
	HPLeft        int            `json:"hp_left"`
	WaveReached   int            `json:"wave_reached"`
	DurationMs    int            `json:"duration_ms"`
	Shots         int            `json:"shots"`
	Hits          int            `json:"hits"`
	Reactions     int            `json:"reactions"`
	HeatMax       int            `json:"heat_max"`
	ElementsUsed  map[string]int `json:"elements_used"`
	ReactionsUsed map[string]int `json:"reactions_used"`
	TerrainUsed   []string       `json:"terrain_used"`
	ReplayHash    string         `json:"replay_hash"`
	// CardPicks 每波选中的手牌索引（-1 表示整波跳过）。
	//
	// ⚠️ 这是 I-6 重放闭环的最后一环：选牌会改变后续战斗，
	// 缺了它第三方无法复现原局，验真会把正常对局判成伪造。
	CardPicks []int `json:"card_picks"`
}

// SettleLimits 是校验阈值。
type SettleLimits struct {
	// MinDurationMs 是「清完全部敌人」的最短合理时长。
	MinDurationMs int64
	// MaxScorePerSec 已废弃，不参与判定。
	//
	// 它曾直接计算时长额度（sec * MaxScorePerSec），但值是硬编码的 50000，
	// 而第 1 关满分只有 19539 —— min() 永远取满分那一侧，
	// 速率限制实际失效：上报 duration_ms=1000 就能合法拿满分、3 星。
	// 这是星级门槛修复的连带回归（门槛从 78000 降到 14656 之后，
	// 50000/秒 再也碰不到了）。
	//
	// 现在额度由 scoreCapFor 按「满分需 ScoreFullAtSec 秒」动态计算，
	// 与关卡自动匹配，不需要为 100 关逐个标定速率。
	// 字段保留只为不破坏 service 层的构造调用点。
	MaxScorePerSec int64
}

// ScoreFullAtSec 是「理论上最快多少秒能拿到满分」。
//
// 取 60 秒的依据：第 1 关实测 6806 tick（340 秒）拿 16048 分，
// 速率约 47 分/秒、满分需约 415 秒。60 秒作为「绝对最快」下界，
// 既拦住「1 秒速通拿满分」，又不会误伤正常对局 ——
// 玩家再怎么快，实测也是 5~6 分钟。
//
// ⚠️ 若将来把关卡调到 60 秒以内可通关，这个值要跟着调；
// TestScoreCapNeverAllowsInstantFull 会在调整前提醒。
const ScoreFullAtSec = 60

// scoreCapFor 返回给定秒数下的得分上界。
//
// 规则：线性插值到理论满分，满分需 ScoreFullAtSec 秒。
// full <= 0（关卡缺 star_targets）时返回 0 —— 宁可不给分，
// 也不能因为数据缺失而放开。
func scoreCapFor(gl GeneratedLevel, sec int64) int64 {
	full := fullStarTarget(gl)
	if full <= 0 || sec <= 0 {
		return 0
	}
	if sec >= ScoreFullAtSec {
		return full
	}
	// 先乘后除不会溢出（full 与 sec 都是小整数），
	// 且 sec < ScoreFullAtSec 保证结果 < full。
	return full * sec / ScoreFullAtSec
}

// SettleResult 是服务端裁剪后的最终结果。
type SettleResult struct {
	// 修正后的分数（可能被裁剪）
	Score int64
	Stars int
	// 各字段是否被裁剪，供后台风控与诊断
	Clamped     bool
	ClampReason string
	// 胜利判定由服务端根据是否漏怪完毕决定
	Win bool
	// 星级由服务端按关卡门槛重算，忽略客户端上报
	StarsRecomputed int
	// 掉落（按理论上限封顶）
	Loot map[string]int64
}

// MaxKillsFor 返回关卡允许的最大击杀数。
func MaxKillsFor(gl GeneratedLevel) int { return gl.TotalEnemies() }

// ValidateSettle 执行全部上限校验并返回裁剪后的结算结果。
//
// 注意：任何"数据不可信"的情况都返回 error 而非静默修正 ——
// 静默修正会让刷子以为自己在正常游戏，掩盖问题。只有"可安全裁剪"的
// 超限（例如掉落、星级）才做裁剪，并记录 Clamped 供后台观察。
func ValidateSettle(
	tok BattleToken,
	gl GeneratedLevel,
	in SettleInput,
	limits SettleLimits,
	now time.Time,
) (SettleResult, error) {
	// 1) token 校验
	if err := tok.IsUsable(now); err != nil {
		return SettleResult{}, err
	}
	if tok.LevelID != gl.ID {
		return SettleResult{}, ErrLevelMismatch
	}

	// 2) 击杀数不得超过关卡总怪数
	maxKills := MaxKillsFor(gl)
	if in.Kills < 0 || in.Kills > maxKills {
		return SettleResult{}, fmt.Errorf("%w：%d > %d", ErrTooManyKills, in.Kills, maxKills)
	}

	// 3) 波次不得超过总波次
	if in.WaveReached < 0 || in.WaveReached > gl.WaveCount {
		return SettleResult{}, fmt.Errorf("%w：%d > %d", ErrWaveExceeded, in.WaveReached, gl.WaveCount)
	}

	// 4) 时长必须为正
	//
	// ⚠️ 绝不能允许 0：DurationMs 是得分速率裁剪的分母，
	// 0 会让整段裁剪逻辑被跳过，任意大的分数都能通过。
	if in.DurationMs <= 0 {
		return SettleResult{}, fmt.Errorf("%w：%dms", ErrTooShort, in.DurationMs)
	}

	// 5) 秒通关检查。
	//
	// ⚠️ 位置有意义：它必须在下面「发射数物理上界」**之前**。
	// 一个 10ms 的结算里报 400 发，两个检查都会拒 ——
	// 但 ErrTooShort 说的是「这局时长不可信」，而 ErrTooManyShots
	// 说的是「你的发射数超了 10ms 能有的量」。对作弊者来说后者更精确，
	// 对排查的人来说前者才是根因（时长造假会连带把发射数一起造大）。
	// 两个检查都保留，调整顺序只影响报哪一个错，不影响放行判定。
	//
	// 失败局不查最短时长：秒退是合法行为。
	if in.Result == "win" && in.Kills >= MaxKillsFor(gl) && int64(in.DurationMs) < limits.MinDurationMs {
		return SettleResult{}, fmt.Errorf("%w：%dms < %dms", ErrTooShort, in.DurationMs, limits.MinDurationMs)
	}

	// 6) 发射数必须有**物理上界**，不只是非负。
	//
	// ⚠️ 这是上一轮修复留下的洞，必须补。
	// 反应次数的上界原本写成 `in.Reactions <= in.Shots * MaxReactionsPerHit`，
	// 而 Shots 当时只校验了「非负」和「hits <= shots」。
	// 于是上报 {"shots":125000000,"hits":125000000,"reactions":1000000000}
	// 能让上界正好变成 1e9 —— 而 1e9 正是那条校验要挡的值。
	// 约束了被保护量（reactions）却没约束控制量（shots），等于没约束。
	//
	// 上界来自引擎的物理节奏：逻辑步长 TickMs=50ms，
	// updateAutoFire 对每个已装备技能每 tick 最多试射一次，
	// 主动槽上限 4 个。留 MaxShotsPerTickSlack 倍余量
	// 容纳手动射击与未来改动，但仍比"无界"紧了几个数量级：
	// 30 秒的一局上界 4808 发，而攻击者原本能报 1.25 亿。
	if in.Shots < 0 || in.Hits < 0 {
		return SettleResult{}, ErrInvalidHitRate
	}
	maxShots := maxShotsFor(in.DurationMs)
	if int64(in.Shots) > maxShots {
		return SettleResult{}, fmt.Errorf("%w：发射 %d > %d（%dms 内物理上限）",
			ErrTooManyShots, in.Shots, maxShots, in.DurationMs)
	}
	if in.Hits > in.Shots {
		return SettleResult{}, fmt.Errorf("%w：命中 %d > 发射 %d", ErrInvalidHitRate, in.Hits, in.Shots)
	}
	if in.Reactions < 0 {
		return SettleResult{}, errors.New("反应次数不能为负")
	}
	// 反应次数上界。取两个上界的较小者：
	// ① Shots * MaxReactionsPerHit —— 一次开火最多触发 N 次反应
	//    （主目标 1 次 + 溅射范围内至多 N-1 个额外目标）。
	//    shots=0 时 reactions 必须为 0：没开火不可能有反应。
	// ② maxKills * MaxReactionsPerEnemy —— 每只敌人身上最多
	//    ELEMENT_ORDER.length 种元素各 element_cap 层，
	//    单种元素的反应每被施加一层最多触发 1 次。
	// 第二个上界不依赖 shots，因此在 shots 上界被绕过时仍然有效。
	//
	// ⚠️ 两个乘法都要防溢出：shots 已是 int，*8 在 32 位平台会溢出，
	// maxKills*20 同理。Go 的 int 在 64 位平台是 64 位，
	// 但攻击者可以把 Shots 设到接近 maxShotsFor 的上限再乘 8 ——
	// 上面的 maxShotsFor 已把时长夹到 24 小时，192 万 × 8 远在 int64 内，
	// 这里是防御性的一层，保证上界永远单调不减。
	maxReactions := int64(in.Shots) * MaxReactionsPerHit
	if absMax := int64(maxKills) * MaxReactionsPerEnemy; absMax < maxReactions {
		maxReactions = absMax
	}
	if int64(in.Reactions) > maxReactions {
		return SettleResult{}, fmt.Errorf("%w：%d > %d", ErrTooManyReactions, in.Reactions, maxReactions)
	}
	// 其余上报字段的上界（同样是任务进度的输入，不能放任）
	if in.HeatMax < 0 || in.HeatMax > MaxReportedHeat {
		return SettleResult{}, fmt.Errorf("%w：峰值热量 %d", ErrInvalidField, in.HeatMax)
	}
	if in.Leaked < 0 || in.Leaked > maxKills {
		return SettleResult{}, fmt.Errorf("%w：漏怪 %d > %d", ErrTooManyLeaked, in.Leaked, maxKills)
	}
	if in.HPLeft < 0 {
		return SettleResult{}, fmt.Errorf("%w：剩余血量 %d", ErrInvalidField, in.HPLeft)
	}

	// 时长过短检查已上移到第 5 步（在发射数上界之前，理由见那里的注释）。
	// 这里只保留胜负判定所需的 win 标记。
	win := in.Result == "win"

	// 7) 得分上限：时长额度与理论满分取小
	res := SettleResult{}
	if in.Score < 0 {
		in.Score = 0
	}
	sec := int64(in.DurationMs) / 1000
	if sec < 1 {
		sec = 1
	}
	// ⚠️ 额度必须锚定「理论满分」，不能是硬编码的每秒分值。
	// MaxScorePerSec 曾是 50000，而第 1 关满分只有 19539 ——
	// 于是 min() 永远取满分那一侧，**速率裁剪彻底失效**：
	// 上报 duration_ms=1000 就能合法拿到满分、3 星。
	// 这是星级门槛修复的连带回归：门槛从 78000 降到 14656 之后，
	// 50000/秒 这个数再也碰不到了。
	//
	// 现在的规则是「至少要花 ScoreFullAtSec 秒才可能拿到满分」。
	// 裁剪与关卡自动匹配，不需要为 100 关逐个标定速率。
	maxScore := scoreCapFor(gl, sec)
	if in.Score > maxScore {
		res.Score = maxScore
		res.Clamped = true
		res.ClampReason = fmt.Sprintf("得分超时长额度：%d → %d（%ds 内上限，满分需 %ds）",
			in.Score, maxScore, sec, ScoreFullAtSec)
	} else {
		res.Score = in.Score
	}
	// 曾经这里有一段「res.Score == 0 时回落到 in.Score」的兜底，
	// 与上方「if in.DurationMs > 0 才裁剪」组合成完整绕过：
	// 上报 DurationMs=0 → 裁剪整段跳过 → 兜底把未裁剪的原始分数写回
	// → 分数任意大 → 星级恒 3 → 单局掉落触顶。
	//
	// 现在裁剪是无条件执行的、且 res.Score 走 if/else 显式赋值，
	// 这段兜底已成为死代码。更重要的是它**有可观测性危害**：
	// 「Score == 0 时回落」让"裁剪没执行"和"客户端确实报 0 分"无法区分，
	// 正是那种静默失效得以长期存在的形状。所以删掉，不保留。

	// 8) 胜利判定
	//
	// 语义是「清空全部波次且防线未破」，即每一只怪都已被解决 ——
	// 被击杀（kills）或漏进防线（leaked）都算解决。
	//
	// ⚠️ 曾经是 `in.Kills >= maxKills`，那个口径是错的：
	// 漏怪已经扣了 base_hp（那是"漏怪容忍度"的设计载体），
	// 血还够却因为漏过而直接判负，等于同一件事惩罚两次且第二次更严。
	// 实测第 1 关默认构筑 kills=32 / leaked=7，于是永远无法通关。
	//
	// 引擎侧口径同步为「波次队列空 + 场上无敌人 + 防线未破」，
	// 两端必须一致，否则客户端播通关动画而服务端回失败。
	//
	// 之所以仍要服务端裁决而不是纯信客户端：kills + leaked 的守恒
	// 由上面的 upper bound 校验保证（两者各自 <= maxKills 且都非负），
	// 上报不实的部分会在第 2/7 步被拒。
	res.Win = win && int64(in.Kills)+int64(in.Leaked) >= int64(maxKills)

	// 9) 星级由服务端按门槛重算，不信任客户端上报
	res.StarsRecomputed = computeStars(res.Score, gl.StarTargets)
	if res.StarsRecomputed != in.Stars {
		res.Stars = res.StarsRecomputed
		if !res.Clamped {
			res.Clamped = true
			res.ClampReason = fmt.Sprintf("星级由客户端重算：%d → %d", in.Stars, res.StarsRecomputed)
		}
	} else {
		res.Stars = in.Stars
	}

	// 10) 掉落按关卡理论上限封顶
	res.Loot = computeLoot(gl, int64(res.Stars), int64(in.Kills), int64(maxKills), int64(in.Reactions))

	return res, nil
}

// computeStars 按三档门槛计算星级。
func computeStars(score int64, targets []int64) int {
	stars := 0
	for _, t := range targets {
		if score >= t {
			stars++
		}
	}
	return stars
}

// DropRates 是掉落表（千分比，作用于满额掉落基数）。
// 刻意让"反应次数"影响掉落：鼓励玩家用搭配而非只堆伤害。
type DropRates struct {
	CoinBase        int64
	CoinPerStar     int64
	CoinPerKill     int64
	ReactionBonus   int64 // 每 10 次反应额外金币
	KeysOnThreeStar int64
	EnergyOnWin     int64
}

// DefaultDropRates 返回首版掉落表。
func DefaultDropRates() DropRates {
	return DropRates{
		CoinBase: 200, CoinPerStar: 150, CoinPerKill: 20,
		ReactionBonus: 30, KeysOnThreeStar: 1, EnergyOnWin: 5,
	}
}

// LootCaps 是掉落封顶（防刷子的硬上限，与关卡无关的全局天花板）。
type LootCaps struct {
	MaxCoinPerBattle   int64
	MaxKeysPerBattle   int64
	MaxEnergyPerBattle int64
}

// DefaultLootCaps 返回首版封顶。
func DefaultLootCaps() LootCaps {
	return LootCaps{MaxCoinPerBattle: 60000, MaxKeysPerBattle: 3, MaxEnergyPerBattle: 15}
}

// computeLoot 计算单局掉落并按封顶裁剪。
//
// ⚠️ 所有输入先夹到非负，且**封顶在加法之后、下溢检查之前**统一做。
// 曾经的写法是 `coin += (reactions/10)*ReactionBonus` 后再 `if coin > MaxCoin`：
// reactions 传 2^62 时乘法溢出成 -4611686018427387916，
// 于是 `coin > 60000` 为假、**裁剪被跳过**、负数金币直接进 grantWallet。
// 上游 ValidateSettle 现在会给 reactions 加上界，但 computeLoot 是导出 API
// （ComputeLootExported），不能把安全性寄托在"调用方都走校验"上。
func computeLoot(gl GeneratedLevel, stars, kills, maxKills int64, reactions int64) map[string]int64 {
	r := DefaultDropRates()
	caps := DefaultLootCaps()

	// 饱和加法：任一输入异常大时结果封顶在 MaxCoinPerBattle，
	// 不依赖中间值不溢出。
	coin := satAdd(caps.MaxCoinPerBattle,
		r.CoinBase,
		satMul(int64(stars), r.CoinPerStar, caps.MaxCoinPerBattle),
		satMul(kills, r.CoinPerKill, caps.MaxCoinPerBattle),
		satMul(divNonNeg(reactions, 10), r.ReactionBonus, caps.MaxCoinPerBattle),
	)
	if coin < 0 {
		coin = 0
	}

	loot := map[string]int64{"coin": coin}
	if stars >= 3 {
		loot["keys"] = r.KeysOnThreeStar
	}
	return loot
}

// --- 饱和算术 ---
//
// 掉落计算的所有输入都来自客户端上报的结算报文。int64 乘法在极端输入下
// 会静默回绕成负数，而负数能骗过一切 `if x > cap` 形式的裁剪。
// 这三个辅助函数让"异常大的上报"变成"封顶"而不是"负数"。

func nonNeg(v int64) int64 {
	if v < 0 {
		return 0
	}
	return v
}

func divNonNeg(a, b int64) int64 {
	if b <= 0 {
		return 0
	}
	return nonNeg(a) / b
}

// satMul 返回 a*b，超过 cap 时封顶为 cap。不依赖中间值不溢出。
//
// ⚠️ cap 是**第三个**参数。把 cap 放中间是很容易犯的错
// （曾经就写成 satMul(cap, a, b) 然后传错，于是 cap 变成了乘数之一，
// 封顶值退化成 20 —— 而"永远不越界"的测试照样绿，
// 只有"极端输入应恰好顶到封顶"那条能抓住）。
func satMul(a, b, cap int64) int64 {
	a, b, cap = nonNeg(a), nonNeg(b), nonNeg(cap)
	if a == 0 || b == 0 || cap == 0 {
		return 0
	}
	// 先除后乘：a > cap/b 时直接返回 cap，避免 a*b 真的溢出。
	if a > cap/b {
		return cap
	}
	return a * b
}

// satAdd 把各项相加，任一项使总和超过 cap 时返回 cap。
func satAdd(cap int64, terms ...int64) int64 {
	cap = nonNeg(cap)
	sum := int64(0)
	for _, t := range terms {
		t = nonNeg(t)
		if t > cap-sum {
			return cap
		}
		sum += t
	}
	return sum
}

// ComputeLootExported 是 computeLoot 的导出包装，供 service 层使用。
func ComputeLoot(gl GeneratedLevel, stars, kills, maxKills, reactions int64) map[string]int64 {
	return computeLoot(gl, stars, kills, maxKills, reactions)
}

// --- 诊断（L-1） ---

// DiagnoseInput 是失败诊断的输入。
type DiagnoseInput struct {
	LevelID      int
	FailedTimes  int
	WaveReached  int
	Kills        int
	TotalEnemies int
	ElementsUsed map[string]int
	Loadout      []Element
}

// Diagnose 是失败诊断结论。
type Diagnose struct {
	Stage       string   // 诊断分类
	Title       string   // 一句话结论
	Detail      string   // 详细数据
	Suggestions []string // 可执行建议
	Confidence  int      // 置信度 0..100
}

// DiagnoseFailure 分析玩家为何卡关，给出针对性建议。
//
// 设计意图（L-1）：原作玩家卡关时只能自己猜；我们直接把"被什么卡住"
// 量化出来。这里刻意区分"打得不够多"与"搭配不对"—— 前者建议升级，
// 后者建议换元素。如果两者混为一谈，诊断就退化成"升级更快一点"，
// 那还不如不说。
func DiagnoseFailure(in DiagnoseInput, gl GeneratedLevel) Diagnose {
	d := Diagnose{Confidence: 60}

	progressPct := 0
	if gl.TotalEnemies() > 0 && in.Kills > 0 {
		progressPct = int(in.Kills) * 100 / gl.TotalEnemies()
	}

	// 统计已装载元素
	have := map[Element]bool{}
	for _, e := range in.Loadout {
		have[e] = true
	}

	// 1) 反应触发过少 → 搭配问题
	totalElements := 0
	for _, v := range in.ElementsUsed {
		totalElements += v
	}
	reactionsLow := in.FailedTimes >= 2 && totalElements < in.Kills

	switch {
	case totalElements == 0:
		d.Stage = "no_element"
		d.Title = "你的技能几乎没有施加任何元素"
		d.Detail = fmt.Sprintf("本局共 %d 次击杀，但元素施加次数为 0 —— 元素反应系统完全没被用上。", in.Kills)
		d.Suggestions = append(d.Suggestions,
			"检查技能是否带元素：带元素的技能才会与敌人身上的元素触发反应",
			"开局先用单一元素铺层数，再用第二元素打出反应")
		d.Confidence = 90

	case reactionsLow:
		d.Stage = "low_reaction"
		d.Title = "元素反应触发过少，输出主要靠直接伤害"
		d.Detail = fmt.Sprintf("本局击杀 %d 个敌人，元素施加仅 %d 次。"+
			"反应伤害与面板攻击力无关，是低养成玩家翻盘的主要来源。", in.Kills, totalElements)
		d.Suggestions = append(d.Suggestions,
			"技能搭配至少覆盖 2 种元素，才能打出 7 条反应链中的多数",
			"优先选带「叠加层数」效果的技能，而不是只加大伤害的技能")
		d.Confidence = 80

	case len(have) < 3:
		d.Stage = "narrow_element"
		d.Title = "元素覆盖过窄，缺少可用的反应组合"
		d.Detail = fmt.Sprintf("当前只装载了 %d 种元素，敌人只要对这些元素高抗，你就没有任何反应可用。", len(have))
		d.Suggestions = append(d.Suggestions,
			"补齐到 3 种以上元素，用专精点压制点数换取反应链覆盖",
			"把与技能同系的装备穿上 —— 同系装备给反应伤害的加成是异系的 3 倍")
		d.Confidence = 75

	case progressPct < 30 && in.FailedTimes >= 2:
		d.Stage = "low_power"
		d.Title = "不是搭配问题，是硬实力不足"
		d.Detail = fmt.Sprintf("本局仅推进 %d%% 的敌人，且元素使用正常。"+
			"说明你的面板攻击力与反应倍率都偏低。", progressPct)
		d.Suggestions = append(d.Suggestions,
			"用金币升级已解锁技能等级（最便宜的成长）",
			"把钻石优先投在洗练宝石上，而不是抽更多装备")
		d.Confidence = 70

	default:
		d.Stage = "unclear"
		d.Title = "卡点不明显，需要更多数据"
		d.Detail = fmt.Sprintf("推进 %d%%，元素使用 %d 次。样本不足以归因。", progressPct, totalElements)
		d.Suggestions = append(d.Suggestions, "再失败一次即可获得更明确的诊断")
		d.Confidence = 40
	}
	return d
}
