package httpapi

// 后台订单列表按渠道筛选的端到端测试（2026-11-01 新一轮第 67 轮）。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminOrdersProviderFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "oprov")
	uid, _ := e.newPlayer(t, "oprovp")

	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		 VALUES ($1, 'E2E_OP_MOCK', 'gem_pack', 600, 'paid', 'mock'),
		        ($1, 'E2E_OP_APPLE', 'gem_pack', 600, 'paid', 'apple')`,
		uid); err != nil {
		t.Fatalf("插入订单失败：%v", err)
	}

	// 无筛选 → 2 条
	status, body := e.get(t, "/api/v1/admin/orders?limit=200", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 2 {
		t.Fatalf("无筛选应 2 条，实际 %v", total)
	}

	// provider=apple → 1 条且都是 apple
	status, body = e.get(t, "/api/v1/admin/orders?limit=200&provider=apple", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("provider=apple 应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 1 {
		t.Errorf("provider=apple 应 1 条，实际 %v", total)
	}
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["provider"] != "apple" {
			t.Errorf("provider=apple 过滤下出现 %v 渠道订单", m["provider"])
		}
	}

	// 未知渠道 → 0 条（不报错，不是注入面）
	status, body = e.get(t, "/api/v1/admin/orders?limit=200&provider=unknown_chan", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("未知渠道应 200（空结果），实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 0 {
		t.Errorf("未知渠道应 0 条，实际 %v", total)
	}
}