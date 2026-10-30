package service

// 玩家支付订单历史的服务层测试（2026-10-31 实质优化第 27 轮）。
//
// 关键行为锚点：
//   - 返回按时间倒序的订单，字段完整（订单号/SKU/金额/状态/时间）；
//   - 无记录返回空列表（不是 nil）；
//   - limit 有上界（>50 夹到 20）；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func seedOrder(t *testing.T, ts *testService, userID int64, status string, createdAt time.Time) string {
	t.Helper()
	ctx := context.Background()
	orderNo := fmt.Sprintf("ORD_%d", time.Now().UnixNano())
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider, created_at)
		 VALUES ($1, $2, 'gem_pack', 600, $3, 'mock', $4)`,
		userID, orderNo, status, createdAt); err != nil {
		t.Fatalf("插入订单失败：%v", err)
	}
	return orderNo
}

func TestMyOrdersSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "orders")
	oldNo := seedOrder(t, ts, id, "paid", time.Now().Add(-2*time.Hour))
	newNo := seedOrder(t, ts, id, "delivered", time.Now().Add(-1*time.Hour))

	items, err := ts.MyOrders(ctx, id, 20)
	if err != nil {
		t.Fatalf("查询订单失败：%v", err)
	}
	if len(items) != 2 {
		t.Fatalf("应有 2 条订单，实际 %d", len(items))
	}
	// 倒序：新的在前
	if items[0].OrderNo != newNo {
		t.Errorf("第一条应是最新订单，实际 %+v", items[0])
	}
	if items[1].OrderNo != oldNo {
		t.Errorf("第二条应是较早订单，实际 %+v", items[1])
	}
	first := items[0]
	if first.SKU != "gem_pack" || first.Amount != 600 {
		t.Errorf("SKU/金额不正确：%+v", first)
	}
	if first.Status != "delivered" {
		t.Errorf("状态 = %q，期望 delivered", first.Status)
	}
	if first.Provider != "mock" {
		t.Errorf("provider = %q，期望 mock", first.Provider)
	}
}

func TestMyOrdersEmpty(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "orders_empty")
	items, err := ts.MyOrders(ctx, id, 20)
	if err != nil {
		t.Fatalf("查询订单失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("无记录应返回空列表（非 nil），实际 %v", items)
	}
}

func TestMyOrdersLimitCap(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "orders_cap")
	for i := 0; i < 25; i++ {
		seedOrder(t, ts, id, "paid", time.Now().Add(-time.Duration(i)*time.Minute))
	}

	items, err := ts.MyOrders(ctx, id, 1000)
	if err != nil {
		t.Fatalf("查询订单失败：%v", err)
	}
	if len(items) != 20 {
		t.Errorf("limit 超上限应夹到 20 条，实际 %d", len(items))
	}
}

func TestMyOrdersBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.MyOrders(ctx, 1, 20)
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空列表")
	}
}
