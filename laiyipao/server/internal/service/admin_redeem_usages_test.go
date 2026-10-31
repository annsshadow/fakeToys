package service

// 后台兑换码使用记录列表的服务层测试（2026-10-31 新一轮第 17 轮）。

import (
	"context"
	"testing"
)

func TestAdminListRedeemUsages(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "ru_list")
	// seeder 已种 redeem_codes；取一个码并插入使用记录
	var codeID int
	var code string
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT id, code FROM redeem_codes LIMIT 1`).Scan(&codeID, &code); err != nil {
		t.Fatalf("取兑换码失败：%v", err)
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, codeID, uid); err != nil {
		t.Fatalf("插入使用记录失败：%v", err)
	}

	items, total, err := ts.AdminListRedeemUsages(ctx, 50, 0)
	if err != nil {
		t.Fatalf("查询使用记录失败：%v", err)
	}
	if total != 1 || len(items) != 1 {
		t.Fatalf("应有 1 条记录，实际 total=%d len=%d", total, len(items))
	}
	it := items[0]
	if it.Code != code {
		t.Errorf("code = %q，期望 %q", it.Code, code)
	}
	if it.UserID != uid {
		t.Errorf("user_id = %d，期望 %d", it.UserID, uid)
	}
	if it.Nickname == "" {
		t.Error("昵称不应为空")
	}
	if it.Reward == nil {
		t.Error("reward 不应为 nil")
	}
	if it.CreatedAt.IsZero() {
		t.Error("created_at 不应为零值")
	}
}

func TestAdminListRedeemUsagesEmpty(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	items, total, err := ts.AdminListRedeemUsages(ctx, 50, 0)
	if err != nil {
		t.Fatalf("查询使用记录失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("空列表应返回非 nil 空切片，实际 %v", items)
	}
	if total != 0 {
		t.Errorf("total 应为 0，实际 %d", total)
	}
}

func TestAdminListRedeemUsagesLimitCap(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "ru_cap")
	// 取码并插入 25 条使用记录（同一用户同一码会撞唯一约束，需不同码）
	var baseID int
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT COALESCE(MAX(id), 0) FROM redeem_codes`).Scan(&baseID); err != nil {
		t.Fatalf("取最大码 id 失败：%v", err)
	}
	for i := 0; i < 25; i++ {
		cid := baseID + 1 + i
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO redeem_codes (id, code, reward) VALUES ($1, $2, '{}')
			 ON CONFLICT (id) DO NOTHING`, cid, "CAP_"+string(rune('A'+i))); err != nil {
			t.Fatalf("插入兑换码失败：%v", err)
		}
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, cid, uid); err != nil {
			t.Fatalf("插入使用记录失败：%v", err)
		}
	}

	items, total, err := ts.AdminListRedeemUsages(ctx, 1000, 0)
	if err != nil {
		t.Fatalf("查询使用记录失败：%v", err)
	}
	if total != 25 {
		t.Errorf("total 应为 25，实际 %d", total)
	}
	if len(items) != 25 {
		t.Errorf("limit=1000 应夹到 50（数据 25 条全返回），实际 %d", len(items))
	}
}

func TestAdminListRedeemUsagesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, _, err := ts.AdminListRedeemUsages(ctx, 50, 0); err == nil {
		t.Fatal("连接池已关闭时必须报错，不能静默返回空列表")
	}
}