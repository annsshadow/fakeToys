package service

// 整局引擎重算的结算侧接线（R1，第 87 轮）。
//
// # 这条路径解决什么
//
// `replay_skills`（S 段）只能证明「技能槽配置没被改」，证明不了战斗过程。
// 引擎移植完成后（`domain/replay.go`），服务端可以拿存档里的
// `build_snapshot` + `card_picks` + `seed` **把整局重跑一遍**，
// 与上报的 `replay_hash` 逐位比对 —— 这才把「可刷分」收敛成「不符即拒发」。
//
// # 为什么单独一个文件而不是塞进 game.go
//
// `SettleBattle` 已经 200+ 行，里面那段 S 段校验就占了 45 行。
// 再塞 60 行进去会让「结算的主流程」完全读不下去。
// 抽成方法还有个实际好处：它可以被**单独**测 —— 第二、三组用例
// （重复结算、快照不可用）直接打这个方法，不必拉起整条结算链。
//
// # ⚠️ 三种结果必须严格区分
//
// `domain.Replay` 的 `Err` 与 `Matched=false` 绝不能混为一谈，
// 混了就会把「我算不出来」报成「玩家作弊」——
// 而那会让一个真实 bug 被误判成攻击事件，进而触发熔断、批量误伤诚实玩家。
//
//	Err != nil   算不出来（缺关卡/缺构筑/种子非法）  -> **放行**，计 cannotRun
//	Matched      算出来了且一致                        -> 放行，计 verified
//	!Matched     算出来了但不一致                      -> 台账 + 闸门，可能拒发

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"sync/atomic"

	"github.com/laiyipao/server/internal/domain"
)

// RecomputeEngineVersionWholeBattle 是整局引擎重算的版本标识。
//
// 台账里必须能区分「哪个引擎判的不符」。skills-only 与 whole-battle
// 的语义完全不同：前者是「上报值与同一份 DB 数据不符」，
// 后者是「我方引擎与客户端引擎对同一局给出了不同结果」——
// 后者可能是引擎漂移，所以它有熔断而前者没有（见 RecomputeScope 的注释）。
const RecomputeEngineVersionWholeBattle = "whole-battle/v1"

// ⚠️ 这里**不**新声明一个 `ErrReplayHashMismatch` 哨兵。
//
// 整局重算的不符与 S 段的不符在 HTTP 层是**同一类拒绝**（都是「上报不可信」
// 家族，都该回 422），所以 `errors.Is(err, ErrReplaySkillsMismatch)` 已经
// 能正确分类。
//
// 新增一个几乎同义的哨兵会踩到 sentinel_classification_test.go 那道守卫
// （`sentinelNames` 登记表与源码声明数必须相等），而它的注释明写着
// 「不要往清单里加临时项，否则这道守卫会失效」。
//
// 两者真正的区别——「哪个引擎判的」——已经由台账的 `mismatched_fields`
// 与 `engine_version` 两列承载，那才是它该在的地方。

// recomputeEngine 用存档里的输入重跑整局，比对 replay_hash。
//
// 返回非 nil 表示**拒绝发奖**。其余所有情况都放行 ——
// 包括「算不出来」，因为那是我们自己的问题而不是玩家的。
//
// buildRaw 传的是写库用的那份 JSON（`marshalJSON(buildSnapshot)` 的结果），
// 而不是 map 本身：那条路径保证了两端的字段名、类型、缺省值完全一致，
// 而 map → struct 的手工转换会在每个字段上多一个出错点。
func (s *Service) recomputeEngine(
	ctx context.Context,
	userID int64,
	tok domain.BattleToken,
	gl domain.GeneratedLevel,
	buildRaw []byte,
	in domain.SettleInput,
) error {
	// ── 老客户端不上报 replay_hash：放行 ──
	//
	// 与 S 段的「空串放行」同一条规则，但要分开计数：
	// 如果把「没上报」也算成 verified，那么把整段校验写死成永远返回
	// true 的代码，也能让这个计数器一路上涨。
	if in.ReplayHash == "" {
		atomic.AddInt64(&s.recomputeSkipped, 1)
		return nil
	}

	// 快照解不出来 → 放行。它是「我方数据不完整」，不是「上报不可信」。
	var build domain.BuildSnapshot
	if err := json.Unmarshal(buildRaw, &build); err != nil {
		atomic.AddInt64(&s.recomputeCannotRun, 1)
		fmt.Fprintf(os.Stderr,
			"[recompute] build_snapshot 解码失败，放行本局（mode 不适用，user=%d, level=%d, err=%v）\n",
			userID, tok.LevelID, err)
		return nil
	}

	outcome := domain.Replay(
		gl, build, fmt.Sprintf("%d", tok.Seed), in.CardPicks, in.ReplayHash,
		domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules(),
	)

	// ⚠️ 这三行顺序不能换：Err 必须先判。
	//   `Matched == false` 在 Err != nil 时同样成立，先判 Matched
	//   会把「算不出来」当成「不一致」—— 正是本文件开头警告的那件事。
	if outcome.Err != nil {
		atomic.AddInt64(&s.recomputeCannotRun, 1)
		fmt.Fprintf(os.Stderr,
			"[recompute] 整局重算跑不起来，放行本局（user=%d, level=%d, err=%v）\n",
			userID, tok.LevelID, outcome.Err)
		return nil
	}

	if outcome.Matched {
		atomic.AddInt64(&s.recomputeVerified, 1)
		return nil
	}

	// ── 到这里才是「算出来了但不一致」──
	atomic.AddInt64(&s.recomputeMismatch, 1)

	mismatch := fmt.Errorf(
		"%w：重放哈希与重算不符\n上报 %q\n重算 %q",
		ErrReplaySkillsMismatch, in.ReplayHash, outcome.ComputedHash)

	// 台账先行 —— 与 S 段同一条路径、同一张表、同一套熔断。
	//
	// ⚠️ battle_id 记 0：拒绝发生在写战报**之前**，此时还没有战报 id。
	mode, enforce := s.recomputeVerdict(ctx, ScopeEngine)
	s.RecordRecompute(ctx, RecomputeRecord{
		BattleID:             0,
		UserID:               userID,
		LevelID:              tok.LevelID,
		ReportedScore:        in.Score,
		ReportedKills:        in.Kills,
		ReportedDurationMs:   in.DurationMs,
		ReportedHash:         in.ReplayHash,
		RecomputedHash:       outcome.ComputedHash,
		RecomputedScore:      outcome.Stats.Score,
		RecomputedKills:      int(outcome.Stats.Kills),
		RecomputedDurationMs: int(outcome.Stats.DurationMs),
		HasRecomputed:        true,
		MismatchedFields:     []string{"replay_hash"},
		Mode:                 mode,
		Rejected:             enforce,
		EngineVersion:        RecomputeEngineVersionWholeBattle,
	})

	if enforce {
		return mismatch
	}
	// 非 enforce 档（off / shadow / 熔断后）：只记录，继续正常发奖。
	fmt.Fprintf(os.Stderr,
		"[recompute] 整局重算不符但未拒发（mode=%s, user=%d, level=%d, reported=%s, recomputed=%s）\n",
		mode, userID, tok.LevelID, in.ReplayHash, outcome.ComputedHash)
	return nil
}

// RecomputeEngineCounters 是重算各分支的累计计数。
//
// # 为什么不直接查台账
//
// 台账 `settle_recomputes` **只记不符**（部分索引
// `WHERE mismatched_fields <> '{}'` 就是明证）。于是：
//
//	「校验正常工作，100% 通过」-> 台账 0 行
//	「校验根本没跑」          -> 台账 0 行
//
// **两者在运营眼里完全一样**，而后者是灾难性的静默失败。
// 本仓已经吃过一次这样的亏（CI 里 vue-tsc 报 "Search string not found"
// 然后退出 0，类型检查一直是假绿）。
//
// 所以必须有「跑到了多少局」这个独立读数：
//
//	Verified  > 0        校验确实在跑
//	Verified == 0 且有结算  校验是死的 —— 这是必须告警的状态
type RecomputeEngineCounters struct {
	// Verified 是「重算跑完且一致」的局数。
	Verified int64 `json:"verified"`
	// Mismatch 是「重算跑完但不一致」的局数（不论是否拒发）。
	Mismatch int64 `json:"mismatch"`
	// CannotRun 是「重算没跑起来」的局数（缺输入 / 数据不完整）。
	// ⚠️ 它高不是好事也不是坏事，但**必须可见** ——
	//   它持续上涨意味着重算长期失效，而台账上看不出来。
	CannotRun int64 `json:"cannot_run"`
	// Skipped 是「客户端没上报 replay_hash」的局数（老客户端）。
	Skipped int64 `json:"skipped"`
}

// EngineCounters 读当前的重算计数快照。
func (s *Service) EngineCounters() RecomputeEngineCounters {
	return RecomputeEngineCounters{
		Verified:  atomic.LoadInt64(&s.recomputeVerified),
		Mismatch:  atomic.LoadInt64(&s.recomputeMismatch),
		CannotRun: atomic.LoadInt64(&s.recomputeCannotRun),
		Skipped:   atomic.LoadInt64(&s.recomputeSkipped),
	}
}

// resetEngineCounters 归零。**只给测试用。**
func (s *Service) resetEngineCounters() {
	atomic.StoreInt64(&s.recomputeVerified, 0)
	atomic.StoreInt64(&s.recomputeMismatch, 0)
	atomic.StoreInt64(&s.recomputeCannotRun, 0)
	atomic.StoreInt64(&s.recomputeSkipped, 0)
}
