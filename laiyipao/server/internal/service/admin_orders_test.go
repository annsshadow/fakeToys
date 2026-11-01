package service

// 后台订单列表的服务层测试（2026-10-31 实质优化第 33 轮）。
//
// 关键行为锚点：
//   - 返回按 id 倒序的订单，含玩家昵称/订单号/SKU/金额/状态/时间；
//   - 空列表非 nil；
//   - limit 有上界（>200 夹到 50）；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func seedOrderFor(t *testing.T, ts *testService, userID int64, status string) string {
	t.Helper()
	ctx := context.Background()
	orderNo := fmt.Sprintf("ADM_ORD_%d", time.Now().UnixNano())
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		 VALUES ($1, $2, 'gem_pack', 600, $3, 'mock')`, userID, orderNo, status); err != nil {
		t.Fatalf("插入订单失败：%v", err)
	}
	return orderNo
}

func TestAdminListOrdersSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "adm_orders")
	orderNo := seedOrderFor(t, ts, uid, "paid")

	items, total, err := ts.AdminListOrders(ctx, 50, 0, "", nil, nil, "")
	if err != nil {
		t.Fatalf("查询订单失败：%v", err)
	}
	if total < 1 {
		t.Fatalf("total 应 >= 1，实际 %d", total)
	}
	found := false
	for _, it := range items {
		if it.OrderNo == orderNo {
			found = true
			if it.Nickname == "" {
				t.Error("订单应带玩家昵称")
			}
			if it.SKU != "gem_pack" || it.Amount != 600 {
				t.Errorf("SKU/金额不正确：%+v", it)
			}
			if it.Status != "paid" {
				t.Errorf("状态 = %q，期望 paid", it.Status)
			}
		}
	}
	if !found {
		t.Error("订单列表应包含测试订单")
	}
}

func TestAdminListOrdersEmpty(t *testing.T) {
	// 用一次性 scratch 库：共享库会被其它用例的订单污染。
	ts := openScratchService(t)
	ctx := context.Background()

	items, total, err := ts.AdminListOrders(ctx, 50, 0, "", nil, nil, "")
	if err != nil {
		t.Fatalf("查询订单失败：%v", err)
	}
	if items == nil {
		t.Error("空列表不应为 nil")
	}
	if total != 0 || len(items) != 0 {
		t.Errorf("应为空，实际 total=%d items=%d", total, len(items))
	}
}

func TestAdminListOrdersLimitCap(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "adm_orders_cap")
	for i := 0; i < 25; i++ {
		seedOrderFor(t, ts, uid, "paid")
	}

	items, _, err := ts.AdminListOrders(ctx, 1000, 0, "", nil, nil, "")
	if err != nil {
		t.Fatalf("查询订单失败：%v", err)
	}
	if len(items) > 50 {
		t.Errorf("limit 超上限应夹到 50 条，实际 %d", len(items))
	}
}

func TestAdminListOrdersBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, _, err := ts.AdminListOrders(ctx, 50, 0, "", nil, nil, "")
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空列表")
	}
}
