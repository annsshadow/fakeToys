package httpapi

// 玩家支付订单历史端点的端到端测试（2026-10-31 实质优化第 27 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 有记录 → 200 + items 包含订单（订单号/状态/金额）；
//   - 无记录 → 200 + 空 items。

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMyOrdersRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/me/orders", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EMyOrdersSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "orders")
	orderNo := fmt.Sprintf("E2E_ORD_%d", time.Now().UnixNano())
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		 VALUES ($1, $2, 'gem_pack', 600, 'paid', 'mock')`, uid, orderNo); err != nil {
		t.Fatalf("插入订单失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/orders", nil)
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
		t.Fatalf("应有 1 条订单，实际 %d", len(body.Items))
	}
	item := body.Items[0]
	if item["order_no"] != orderNo {
		t.Errorf("order_no 应为 %s，实际 %v", orderNo, item["order_no"])
	}
	if item["status"] != "paid" {
		t.Errorf("status 应为 paid，实际 %v", item["status"])
	}
	if item["amount"] != float64(600) {
		t.Errorf("amount 应为 600，实际 %v", item["amount"])
	}
}

func TestE2EMyOrdersEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "orders_empty")
	req, _ := http.NewRequest("GET", "/api/v1/me/orders", nil)
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
