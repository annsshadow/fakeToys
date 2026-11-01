package service

// 玩家购买记录的服务层测试（2026-10-30 实质优化第 20 轮）。
//
// 关键行为锚点：
//   - 返回按时间倒序的购买记录，含道具名/品类/价格快照/购买日期；
//   - 无记录返回空列表（不是 nil）；
//   - limit 有上界（>50 夹到 20）；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func seedShopItem(t *testing.T, ts *testService, id int, name, category string) {
	t.Helper()
	ctx := context.Background()
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO shop_items (id, code, name, category, price, payload)
		 VALUES ($1, $2, $3, $4, '{"coin":100}'::jsonb, '{"coin":100}'::jsonb)
		 ON CONFLICT (id) DO NOTHING`, id, fmt.Sprintf("code_%d", id), name, category); err != nil {
		t.Fatalf("插入商城道具失败：%v", err)
	}
}

func seedPurchase(t *testing.T, ts *testService, userID int64, itemID int, daysAgo int) {
	t.Helper()
	ctx := context.Background()
	purchaseDate := time.Now().AddDate(0, 0, -daysAgo).Format("2006-01-02")
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO user_purchases (user_id, item_id, purchase_date, price)
		 VALUES ($1, $2, $3::date, '{"coin":100}'::jsonb)`,
		userID, itemID, purchaseDate); err != nil {
		t.Fatalf("插入购买记录失败：%v", err)
	}
}

func TestMyPurchasesSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	seedShopItem(t, ts, 9001, "金币礼包", "currency")
	seedShopItem(t, ts, 9002, "钥匙包", "keys")
	id, _ := newGuestUser(t, ts, "purchases")
	seedPurchase(t, ts, id, 9001, 2)
	seedPurchase(t, ts, id, 9002, 1)

	items, err := ts.MyPurchases(ctx, id, 20)
	if err != nil {
		t.Fatalf("查询购买记录失败：%v", err)
	}
	if len(items) != 2 {
		t.Fatalf("应有 2 条记录，实际 %d", len(items))
	}
	// 倒序：最近的（钥匙包）在前
	if items[0].ItemName != "钥匙包" {
		t.Errorf("第一条应是最新购买（钥匙包），实际 %+v", items[0])
	}
	first := items[0]
	if first.Category != "keys" {
		t.Errorf("品类 = %q，期望 keys", first.Category)
	}
	if first.Price["coin"] != 100 {
		t.Errorf("价格快照 coin = %d，期望 100", first.Price["coin"])
	}
	if first.PurchaseDate == "" {
		t.Error("购买日期不应为空")
	}
	if items[1].ItemName != "金币礼包" {
		t.Errorf("第二条应是较早购买（金币礼包），实际 %+v", items[1])
	}
}

func TestMyPurchasesEmpty(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "purchases_empty")
	items, err := ts.MyPurchases(ctx, id, 20)
	if err != nil {
		t.Fatalf("查询购买记录失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("无记录应返回空列表（非 nil），实际 %v", items)
	}
}

func TestMyPurchasesLimitCap(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "purchases_cap")
	seedShopItem(t, ts, 9100, "道具", "boost")
	for i := 0; i < 25; i++ {
		seedPurchase(t, ts, id, 9100, i)
	}

	items, err := ts.MyPurchases(ctx, id, 1000)
	if err != nil {
		t.Fatalf("查询购买记录失败：%v", err)
	}
	if len(items) != 20 {
		t.Errorf("limit 超上限应夹到 20 条，实际 %d", len(items))
	}
}

func TestMyPurchasesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.MyPurchases(ctx, 1, 20)
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空列表")
	}
}
