package service

// 后台兑换码列表按码搜索 / 启用状态 / 过期筛选的测试
// （2026-10-31 新一轮第 20 轮）。

import (
	"context"
	"testing"
	"time"
)

func TestAdminListRedeemCodesFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 三个码：可用的 A、停用的 B、已过期的 C（enable 但过期）
	now := time.Now()
	seedAdminRedeemCode(t, ts, 9001, "LYP-ACTIVE-01", true, nil)
	seedAdminRedeemCode(t, ts, 9002, "LYP-DISABLED-02", false, nil)
	expiredAt := now.Add(-time.Hour)
	seedAdminRedeemCode(t, ts, 9003, "LYP-EXPIRED-03", true, &expiredAt)

	// 按码搜索（模糊包含）
	items, total, err := ts.AdminListRedeemCodes(ctx, 50, 0, "ACTIVE", nil, nil)
	if err != nil {
		t.Fatalf("码搜索失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Code != "LYP-ACTIVE-01" {
		t.Errorf("搜 ACTIVE 应 1 条，实际 total=%d", total)
	}

	// 按启用状态
	items, total, err = ts.AdminListRedeemCodes(ctx, 50, 0, "", boolPtr(false), nil)
	if err != nil {
		t.Fatalf("停用筛选失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Code != "LYP-DISABLED-02" {
		t.Errorf("停用应 1 条，实际 total=%d", total)
	}

	// 按过期
	items, total, err = ts.AdminListRedeemCodes(ctx, 50, 0, "", nil, boolPtr(true))
	if err != nil {
		t.Fatalf("过期筛选失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Code != "LYP-EXPIRED-03" {
		t.Errorf("过期应 1 条，实际 total=%d", total)
	}

	// 组合：启用 + 非过期 → 包含 9001，且不含停用的 9002 / 过期的 9003
	items, total, err = ts.AdminListRedeemCodes(ctx, 50, 0, "", boolPtr(true), boolPtr(false))
	if err != nil {
		t.Fatalf("组合筛选失败：%v", err)
	}
	found := map[string]bool{}
	for _, it := range items {
		found[it.Code] = true
	}
	if !found["LYP-ACTIVE-01"] {
		t.Errorf("启用+非过期应包含 LYP-ACTIVE-01，实际 %+v", items)
	}
	if found["LYP-DISABLED-02"] || found["LYP-EXPIRED-03"] {
		t.Errorf("启用+非过期不应包含停用/过期码，实际 %+v", items)
	}
	if total < 4 {
		t.Errorf("启用+非过期应至少 4 条（seeder 3 + ACTIVE），实际 %d", total)
	}
}

func seedAdminRedeemCode(t *testing.T, ts *testService, id int, code string, enabled bool, expires *time.Time) {
	t.Helper()
	ctx := context.Background()
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled, expires_at)
		 VALUES ($1, $2, '{"coin":100}'::jsonb, 1, 0, $3, $4)
		 ON CONFLICT (id) DO UPDATE SET code = $2, enabled = $3, expires_at = $4`,
		id, code, enabled, expires); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}
}

func boolPtr(v bool) *bool { return &v }