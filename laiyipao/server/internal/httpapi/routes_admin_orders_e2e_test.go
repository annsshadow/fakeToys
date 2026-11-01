package httpapi

// 后台订单列表的端到端测试（2026-10-31 实质优化第 33 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 有订单 → 200 + items 包含订单（订单号/玩家/状态/金额）。

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminOrdersRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/orders", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminOrdersSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "adm_orders")
	_, adminTok := e.newAdmin(t, "adm_orders")
	orderNo := fmt.Sprintf("E2E_ADM_ORD_%d", time.Now().UnixNano())
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		 VALUES ($1, $2, 'gem_pack', 600, 'paid', 'mock')`, uid, orderNo); err != nil {
		t.Fatalf("插入订单失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/orders", nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
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
	found := false
	for _, it := range body.Items {
		if it["order_no"] == orderNo {
			found = true
			if it["status"] != "paid" {
				t.Errorf("状态应为 paid，实际 %v", it["status"])
			}
			if it["amount"] != float64(600) {
				t.Errorf("金额应为 600，实际 %v", it["amount"])
			}
		}
	}
	if !found {
		t.Error("订单列表应包含测试订单")
	}
}
