package httpapi

// 玩家购买记录端点的端到端测试（2026-10-30 实质优化第 20 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 有记录 → 200 + items 包含购买记录（道具名/品类/价格快照）；
//   - 无记录 → 200 + 空 items。

import (
	"context"
	"encoding/json"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMyPurchasesRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/me/purchases", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EMyPurchasesSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "purchases")
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO shop_items (id, code, name, category, price, payload)
		 VALUES (9201, 'e2e_item', '测试道具', 'boost', '{"gem":50}'::jsonb, '{"gem":50}'::jsonb)
		 ON CONFLICT (id) DO NOTHING`); err != nil {
		t.Fatalf("插入商城道具失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO user_purchases (user_id, item_id, purchase_date, price)
		 VALUES ($1, 9201, CURRENT_DATE, '{"gem":50}'::jsonb)`, uid); err != nil {
		t.Fatalf("插入购买记录失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/purchases", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	var body struct {
		Items []map[string]any `json:"items"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d", resp.StatusCode)
	}
	if len(body.Items) != 1 {
		t.Fatalf("应有 1 条记录，实际 %d", len(body.Items))
	}
	item := body.Items[0]
	if item["item_name"] != "测试道具" {
		t.Errorf("item_name 应为 测试道具，实际 %v", item["item_name"])
	}
	if item["category"] != "boost" {
		t.Errorf("category 应为 boost，实际 %v", item["category"])
	}
	price, ok := item["price"].(map[string]any)
	if !ok || price["gem"] != float64(50) {
		t.Errorf("price 应为 {gem:50}，实际 %v", item["price"])
	}
}

func TestE2EMyPurchasesEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "purchases_empty")
	req, _ := http.NewRequest("GET", "/api/v1/me/purchases", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	var body struct {
		Items []any `json:"items"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if body.Items == nil || len(body.Items) != 0 {
		t.Errorf("无记录应返回空 items，实际 %v", body.Items)
	}
}
