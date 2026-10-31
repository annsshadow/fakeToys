package service

// 后台订单列表按状态 + 日期范围过滤的测试（2026-10-31 新一轮第 16 轮）。
//
// 运营查「今天待处理的订单」此前只能翻页。新增可选 status /
// date_from / date_to（`created_at < date_to` 开区间）。

import (
	"context"
	"testing"
	"time"
)

func TestAdminListOrdersFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "ofilter")
	now := time.Now()

	seedOrder := func(status string, at time.Time, tag string) {
		t.Helper()
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO orders (user_id, order_no, sku, amount, status, created_at)
			 VALUES ($1, $2, 'sku', 100, $3, $4)`,
			uid, "ord_"+tag, status, at); err != nil {
			t.Fatalf("插入订单失败：%v", err)
		}
	}
	seedOrder("pending", now.Add(-3*time.Hour), "a")
	seedOrder("paid", now.Add(-2*time.Hour), "b")
	seedOrder("pending", now.Add(-1*time.Hour), "c")

	// 只按状态 pending → 2 条（-3h 与 -1h）
	items, total, err := ts.AdminListOrders(ctx, 50, 0, "pending", nil, nil, "")
	if err != nil {
		t.Fatalf("状态过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("pending 应 2 条，实际 %d", total)
	}
	for _, o := range items {
		if o.Status != "pending" {
			t.Errorf("状态过滤返回了非 pending 订单：%+v", o)
		}
	}

	// 只按日期 date_from=-2.5h → 2 条（-2h paid / -1h pending）
	from := now.Add(-150 * time.Minute)
	items, total, err = ts.AdminListOrders(ctx, 50, 0, "", &from, nil, "")
	if err != nil {
		t.Fatalf("日期过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("date_from=-2.5h 应 2 条，实际 %d", total)
	}

	// 状态 + 日期组合：pending 且 date_from=-2.5h → 1 条（-1h）
	items, total, err = ts.AdminListOrders(ctx, 50, 0, "pending", &from, nil, "")
	if err != nil {
		t.Fatalf("组合过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 {
		t.Errorf("pending + date_from=-2.5h 应 1 条，实际 total=%d", total)
	}
}

func TestAdminListOrdersBrokenPoolFilter(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, _, err := ts.AdminListOrders(ctx, 50, 0, "pending", nil, nil, ""); err == nil {
		t.Fatal("连接池已关闭时必须报错，不能静默返回空列表")
	}
}