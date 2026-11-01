package service

// 第 12 轮：AdminListUsers 验真计数改为两阶段（先页内用户，再按页内 id 聚合）。
//
// 原实现每次翻页都对 replay_verifications⋈battle_records 做**全表** GROUP BY，
// 而消费计数的只有本页 ≤50 个用户。本测试钉住回填的正确性：
// 匹配/不匹配两次验真 → 列表里该用户的 checked/mismatched 必须精确。

import (
	"context"
	"testing"
)

func TestAdminListUsersVerifyStatsFilled(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	in := loseSettleInput()
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	var battleID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).Scan(&battleID); err != nil {
		t.Fatalf("读战报失败：%v", err)
	}

	// 同一局验真两次：一次匹配、一次不匹配（重验允许，正是「行数放大」的来源）。
	if _, err := ts.VerifyReplay(ctx, uid, battleID, in.ReplayHash); err != nil {
		t.Fatalf("匹配验真失败：%v", err)
	}
	if _, err := ts.VerifyReplay(ctx, uid, battleID, "ffffffffffffffff"); err != nil {
		t.Fatalf("不匹配验真失败：%v", err)
	}

	users, _, err := ts.AdminListUsers(ctx, "", 50, 0, nil, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("列表失败：%v", err)
	}
	for _, u := range users {
		if u.ID != uid {
			continue
		}
		if u.VerifyChecked != 2 {
			t.Errorf("验真次数应为 2，实际 %d", u.VerifyChecked)
		}
		if u.VerifyMismatched != 1 {
			t.Errorf("不匹配次数应为 1，实际 %d", u.VerifyMismatched)
		}
		return
	}
	t.Fatalf("列表结果里没有刚结算的用户 %d（共 %d 行）", uid, len(users))
}
