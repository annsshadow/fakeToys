package httpapi

// 后台订单列表按状态/日期筛选的端到端测试（2026-10-31 新一轮第 16 轮）。

import (
	"context"
	"net/url"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminOrdersFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ofilter")
	uid, _ := e.newPlayer(t, "ofilterp")
	now := time.Now()

	// 两条订单：3 小时前 pending、1 小时前 paid
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO orders (user_id, order_no, sku, amount, status, created_at)
		 VALUES ($1, 'ord_a', 'sku', 100, 'pending', $2), ($1, 'ord_b', 'sku', 100, 'paid', $3)`,
		uid, now.Add(-3*time.Hour), now.Add(-1*time.Hour)); err != nil {
		t.Fatalf("插入订单失败：%v", err)
	}

	// 无筛选 → total=2
	status, body := e.get(t, "/api/v1/admin/orders?limit=50", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("订单列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok || totalAll != 2 {
		t.Fatalf("无筛选应 2 条，实际 %v", body["total"])
	}

	// 状态筛选 pending → 1 条
	status, body = e.get(t, "/api/v1/admin/orders?limit=50&status=pending", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("状态筛选应 200，实际 %d %v", status, body)
	}
	totalPending, ok := body["total"].(float64)
	if !ok || totalPending != 1 {
		t.Errorf("pending 应 1 条，实际 %v", body["total"])
	}

	// 日期筛选 date_from=-2h → 1 条（-1h 的 paid）
	from := now.Add(-2 * time.Hour).Format(time.RFC3339)
	status, body = e.get(t, "/api/v1/admin/orders?limit=50&date_from="+url.QueryEscape(from), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("日期筛选应 200，实际 %d %v", status, body)
	}
	totalFiltered, ok := body["total"].(float64)
	if !ok || totalFiltered != 1 {
		t.Errorf("date_from=-2h 应 1 条，实际 %v", body["total"])
	}

	// 非法日期 → 400
	status, body = e.get(t, "/api/v1/admin/orders?date_from=not-a-date", adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("非法 date_from 应 400，实际 %d %v", status, body)
	}
}