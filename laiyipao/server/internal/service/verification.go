package service

// 验真的**唯一**实现 —— 玩家侧与运营侧共用。
//
// ## 为什么抽出来
//
// 验真此前只有玩家侧一条路径（`POST /api/v1/battle/verify`）。
// 要让运营也能发起（`POST /admin/battles/:id/verify`）时，如果各写一份，
// 就会出现两份「比对 + 落库」逻辑 —— 而它们必须在**每一个细节上**一致
// （读哪些列、怎么比、往哪张表写、`matched` 怎么算）。
// 两份实现只要有一处漂移，就是「玩家验真和运营验真给出不同结论」，
// 而这种 bug 极难发现。
//
// 所以这里抽一个共用实现，差异只有「验真人是谁」。
//
// ## 这个机制到底证明了什么（诚实标注）
//
// 比对的是 `replay_verifications` 记下的 `expected_hash`
// 与提交方给的 `actual_hash` 是否相同。
//
// `expected_hash` 是**结算时客户端自己报上来的**。所以：
//
//   - 如果提交方是**诚实的重算者**（小程序验证页会用
//     `replay as runReplay` 重跑引擎），不匹配就确实证明了分数被篡改。
//   - 如果提交方是**原始作弊客户端**，它可以随便报一个相同的 hash 让结果「一致」。
//
// 也就是说：**这个机制防的是「改数据不改凭证」，不是「从一开始就伪造」**。
// 后者要靠别的手段（服务端重算）—— 那需要把 TS 引擎移植到 Go，
// 属于另一个量级的工程。
//
// 之前 `service.go` 里把这件事记成了「验真一致率」，读起来像是后者；
// 实际是前者。注释留在原地，别再被误读。

import (
	"context"
	"errors"
	"fmt"
	"strconv"
	"time"

	"github.com/jackc/pgx/v5"
)

// verifier 标识「这次验真是谁做的」。
//
// 玩家与运营在**两张不同的表**里（users / admin_users），
// 所以要分开两个字段；migration 00011 加了 CHECK 保证恰好一个非空。
type verifier struct {
	// userID 非 0 时写 verifier_id，否则写 admin_verifier_id
	userID  int64
	adminID int64
	isAdmin bool
}

func userVerifier(id int64) verifier  { return verifier{userID: id} }
func adminVerifier(id int64) verifier { return verifier{adminID: id, isAdmin: true} }

// recordVerification 把一条验真结果写进 replay_verifications。
func (s *Service) recordVerification(ctx context.Context, v verifier, battleID int64, expected, actual string, matched bool) error {
	var userArg, adminArg any
	if v.isAdmin {
		adminArg = v.adminID
	} else {
		userArg = v.userID
	}
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO replay_verifications
		   (battle_id, verifier_id, admin_verifier_id, expected_hash, actual_hash, matched)
		 VALUES ($1,$2,$3,$4,$5,$6)`,
		battleID, userArg, adminArg, expected, actual, matched); err != nil {
		return fmt.Errorf("record verification: %w", err)
	}
	return nil
}

// compareAndRecord 是验真的共用实现。
//
// 流程刻意与旧实现逐字一致（读同一批列、同一个比较、同一张表），
// 这样玩家侧的行为**不会因为这次抽取而发生任何变化**。
func (s *Service) compareAndRecord(ctx context.Context, v verifier, battleID int64, actualHash string) (VerifyResult, error) {
	var r VerifyResult
	var createdAt time.Time
	var seed int64
	// created_at 是 timestamptz 而非 text —— 扫 string 失败会得到
	// 「扫到零值」而不是「扫不到战报」，两者必须区分。
	err := s.pool.QueryRow(ctx,
		`SELECT br.level_id, COALESCE(bt.seed, 0), br.replay_hash, br.created_at
		 FROM battle_records br
		 LEFT JOIN battle_tokens bt ON bt.id = br.battle_token_id
		 WHERE br.id = $1`, battleID).
		Scan(&r.LevelID, &seed, &r.Expected, &createdAt)
	if errors.Is(err, pgx.ErrNoRows) {
		return VerifyResult{}, fmt.Errorf("%w: battle %d", ErrNotFound, battleID)
	}
	if err != nil {
		return VerifyResult{}, fmt.Errorf("load battle %d for verify: %w", battleID, err)
	}
	r.RecordedAt = createdAt.UTC().Format(timeFormat)
	r.BattleID = battleID
	// Seed 用字符串下发：int64 可能超过 2^53，作为 JSON number 会被 JS 解析成错的
	// 值。客户端与运营拿到的必须是同一个 seed，否则重算出来的哈希必然不同 ——
	// 那会被记成「不匹配」，而责任其实在序列化上。
	r.Seed = strconv.FormatInt(seed, 10)
	r.Actual = actualHash
	r.Matched = r.Expected == actualHash

	if err := s.recordVerification(ctx, v, battleID, r.Expected, r.Actual, r.Matched); err != nil {
		return VerifyResult{}, err
	}
	return r, nil
}

// VerifyReplay 玩家侧验真。
//
// ⚠️ 本函数**不重算**哈希 —— 比对的是「提交方报的」与「结算时记的」。
// 详见本文件头「这个机制到底证明了什么」。
func (s *Service) VerifyReplay(ctx context.Context, userID, battleID int64, actualHash string) (VerifyResult, error) {
	return s.compareAndRecord(ctx, userVerifier(userID), battleID, actualHash)
}

// AdminVerifyReplay 运营侧验真。
//
// 补上的是「能采取行动的人，唯一能主动发起验真」这一段。
// 参数与返回值与玩家侧**完全一致**，因为它们做的是同一件事。
func (s *Service) AdminVerifyReplay(ctx context.Context, adminID, battleID int64, actualHash string) (VerifyResult, error) {
	return s.compareAndRecord(ctx, adminVerifier(adminID), battleID, actualHash)
}

// auditVerificationDetail 供审计日志用的明细。
//
// 运营发起的验真会同时写 `admin_audit_logs`（谁、何时、对哪场、结论），
// 因为这是一次**管理动作**，审计轨迹要独立于统计表存在。
func AuditVerificationDetail(r VerifyResult) map[string]any {
	return map[string]any{
		"battle_id":  r.BattleID,
		"level_id":   r.LevelID,
		"matched":    r.Matched,
		"expected":   r.Expected,
		"actual":     r.Actual,
		"seed":       r.Seed,
		"recordedAt": r.RecordedAt,
	}
}
