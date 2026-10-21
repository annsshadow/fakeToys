package service

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// AbandonBattle 放弃一场未结算的战斗，并把**开战时扣掉的资源退回去**。
//
// # 为什么需要它（2026-10-21，P4-5）
//
// `StartBattle` 在**开战时**扣体力（`battle_start` 流水，ref_id = token id）。
// 而客户端此前**没有**任何放弃路径 —— `battle.vue` 的 `onUnload` 只停渲染器。
//
// 于是玩家中途退出（切后台被系统回收、误触返回、临时有事）：
//
//   - 体力**已经扣了**；
//   - 没有战报、没有奖励；
//   - 那枚开局凭证还在库里（`used_at IS NULL`），直到过期。
//
// 也就是「什么都没发生，但资源没了」。这与「打输了」完全不同 ——
// 打输了至少有战报、有星星、有掉落。
//
// # 退什么
//
//   - **体力**：全额退（`+gl.EnergyCost`）。
//   - **复活币**：如果这一局消耗了复活币，也退。
//     理由与体力一样：玩家没有获得任何收益（复活币的作用是
//     「本关多扛 N 次漏怪」，而这一局根本没打完）。
//
// # 不退什么
//
// 战报、星星、掉落 —— 因为本来就没有。
//
// # 幂等 / 竞争
//
// 开局凭证在放弃时被标记为已用（`used_at = now()`），
// 所以**同一枚凭证不能被放弃两次**，也不能「放弃后再结算」——
// 两者都在同一个事务里，靠 `used_at IS NULL` 的条件更新保证。
func (s *Service) AbandonBattle(ctx context.Context, userID int64, tokenID int64) (int64, error) {
	var refundedEnergy int64
	err := s.DB.Tx(ctx, func(tx txType) error {
		var levelID int
		var usedAt *time.Time
		var expiresAt time.Time
		err := tx.QueryRow(ctx,
			`SELECT level_id, used_at, expires_at FROM battle_tokens
			 WHERE id = $1 AND user_id = $2`, tokenID, userID).
			Scan(&levelID, &usedAt, &expiresAt)
		if err != nil {
			if errors.Is(err, pgx.ErrNoRows) {
				return fmt.Errorf("%w: 开局凭证 %d 不存在", ErrNotFound, tokenID)
			}
			return fmt.Errorf("read battle token: %w", err)
		}
		if usedAt != nil {
			// 已结算或已放弃 —— 不能再退一次。
			return fmt.Errorf("%w: 这局已经结束了", ErrBadInput)
		}
		if time.Now().After(expiresAt) {
			return fmt.Errorf("%w: 这局已经过期了", ErrBadInput)
		}
		gl := domain.GenerateLevel(levelID)

		// 1) 标记凭证已用（条件更新：只有仍未使用的那枚才会被改到）
		tag, err := tx.Exec(ctx,
			`UPDATE battle_tokens SET used_at = now()
			 WHERE id = $1 AND user_id = $2 AND used_at IS NULL`, tokenID, userID)
		if err != nil {
			return fmt.Errorf("mark token used: %w", err)
		}
		if tag.RowsAffected() == 0 {
			// 别人（另一个请求）已经结算/放弃了这一局。
			return fmt.Errorf("%w: 这局已经结束了", ErrBadInput)
		}

		// 2) 退体力
		if gl.EnergyCost > 0 {
			if err := s.grantWallet(ctx, tx, userID,
				map[string]int64{"energy": int64(gl.EnergyCost)}, "battle_abandon", tokenID); err != nil {
				return err
			}
			refundedEnergy = int64(gl.EnergyCost)
		}

		// 3) 退复活币（如果这一局真的消耗了）
		//
		// 判定依据是 wallet_flows 里那条 `revive_token / battle_start /
		// ref_id = tokenID` 的流水 —— 与 `StartBattle` 扣币时写的是同一条路径。
		var spent int64
		if err := tx.QueryRow(ctx,
			`SELECT COALESCE(SUM(-delta), 0) FROM wallet_flows
			 WHERE user_id = $1 AND currency = 'revive_token'
			   AND reason = 'battle_start' AND ref_id = $2`, userID, tokenID).Scan(&spent); err != nil {
			return fmt.Errorf("read revive spend: %w", err)
		}
		if spent > 0 {
			if err := s.grantToken(ctx, tx, userID, "revive_token", spent, "battle_abandon", tokenID); err != nil {
				return err
			}
		}
		return nil
	})
	if err != nil {
		return 0, err
	}
	return refundedEnergy, nil
}
