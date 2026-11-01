package service

// 第 10 轮：未跨级的结算跳过两条解锁语句。
//
// 判据用「改名 skill_recipes」做双向判别器 —— 结算路径里只有
// applyProgress 的解锁 INSERT 会碰这张表：
//   - 未跨级：语句被跳过 → 表已改名仍成功；
//   - 跨级：语句必跑 → 表已改名必报错（unlock skills by level）。
//
// 两条合起来证明守卫「该跳的跳、该跑的跑」，而不是单边自证。

import (
	"context"
	"testing"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

func TestApplyProgressSkipsUnlockWhenLevelUnchanged(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 等级垫到 L5（exp = 5×D）：胜局 +50 仍在等级 5，不跨线。
	d := int64(domain.PlayerLevelDivisor)
	scratch.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, 5*d)

	scratch.renameTable(t, "skill_recipes", "skill_recipes_gone_guard")
	err := scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
		_, err := scratch.applyProgress(ctx, tx, uid, 1, true, 0)
		return err
	})
	if err != nil {
		t.Fatalf("未跨级结算不应触碰解锁语句（skill_recipes 已改名仍须成功）：%v", err)
	}
}

func TestApplyProgressRunsUnlockWhenLevelCrossed(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 等级垫到 6 级差 1 点（exp = 6×D − 1）：胜局 +50 跨入等级 6。
	d := int64(domain.PlayerLevelDivisor)
	scratch.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, 6*d-1)

	scratch.renameTable(t, "skill_recipes", "skill_recipes_gone_cross")
	err := scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
		_, err := scratch.applyProgress(ctx, tx, uid, 1, true, 0)
		return err
	})
	mustErr(t, err, "unlock skills by level")
}
