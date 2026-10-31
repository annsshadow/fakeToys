package service

// 后台兑换码使用记录按码/用户过滤的测试（2026-10-31 新一轮第 23 轮）。

import (
	"context"
	"testing"
)

func TestAdminListRedeemUsagesFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uidA, _ := newGuestUser(t, ts, "ru_f_a")
	uidB, _ := newGuestUser(t, ts, "ru_f_b")
	// 三个码：A 用户用码1、码2；B 用户用码3
	codeA := seedCodeWithUsage(t, ts, 9101, "LYP-USAGE-A", uidA)
	seedCodeWithUsage(t, ts, 9102, "LYP-USAGE-B", uidA)
	seedCodeWithUsage(t, ts, 9103, "LYP-USAGE-C", uidB)
	_ = codeA

	// 按码搜索
	items, total, err := ts.AdminListRedeemUsages(ctx, 50, 0, "USAGE-B", 0)
	if err != nil {
		t.Fatalf("码搜索失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Code != "LYP-USAGE-B" {
		t.Errorf("搜 USAGE-B 应 1 条，实际 total=%d", total)
	}

	// 按用户过滤
	items, total, err = ts.AdminListRedeemUsages(ctx, 50, 0, "", uidB)
	if err != nil {
		t.Fatalf("用户过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].UserID != uidB {
		t.Errorf("uidB 应 1 条，实际 total=%d", total)
	}

	// 组合：码 + 用户
	items, total, err = ts.AdminListRedeemUsages(ctx, 50, 0, "USAGE", uidA)
	if err != nil {
		t.Fatalf("组合过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("USAGE+uidA 应 2 条，实际 %d", total)
	}

	// 无筛选
	items, total, err = ts.AdminListRedeemUsages(ctx, 50, 0, "", 0)
	if err != nil {
		t.Fatalf("无筛选失败：%v", err)
	}
	if total != 3 {
		t.Errorf("无筛选应 3 条，实际 %d", total)
	}
}

func seedCodeWithUsage(t *testing.T, ts *testService, id int, code string, uid int64) string {
	t.Helper()
	ctx := context.Background()
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_codes (id, code, reward, max_uses) VALUES ($1, $2, '{"coin":1}'::jsonb, 100)
		 ON CONFLICT (id) DO UPDATE SET code = $2`, id, code); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, id, uid); err != nil {
		t.Fatalf("插入使用记录失败：%v", err)
	}
	return code
}