package httpapi

// 后台审计日志按操作类型/日期筛选的端到端测试（2026-10-31 新一轮第 18 轮）。

import (
	"context"
	"net/url"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminAuditLogsFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "audit_f")
	now := time.Now()

	// 三条审计：不同 action / 时间
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO admin_audit_logs (admin_id, username, action, target, detail, created_at)
		 VALUES (1, 'admin', 'update_redeem', 'a', '{}'::jsonb, $1),
		        (1, 'admin', 'ban_user', 'b', '{}'::jsonb, $2),
		        (1, 'admin', 'update_redeem', 'c', '{}'::jsonb, $3)`,
		now.Add(-3*time.Hour), now.Add(-2*time.Hour), now.Add(-1*time.Hour)); err != nil {
		t.Fatalf("插入审计日志失败：%v", err)
	}

	// 无筛选 → total=3
	status, body := e.get(t, "/api/v1/admin/audit-logs?limit=100", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok || totalAll != 3 {
		t.Fatalf("无筛选应 3 条，实际 %v", body["total"])
	}

	// action 筛选 update_redeem → 2 条
	status, body = e.get(t, "/api/v1/admin/audit-logs?limit=100&action=update_redeem", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("action 筛选应 200，实际 %d %v", status, body)
	}
	totalAction, ok := body["total"].(float64)
	if !ok || totalAction != 2 {
		t.Errorf("update_redeem 应 2 条，实际 %v", body["total"])
	}

	// 日期筛选 date_from=-2.5h → 2 条
	from := now.Add(-150 * time.Minute).Format(time.RFC3339)
	status, body = e.get(t, "/api/v1/admin/audit-logs?limit=100&date_from="+url.QueryEscape(from), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("日期筛选应 200，实际 %d %v", status, body)
	}
	totalDate, ok := body["total"].(float64)
	if !ok || totalDate != 2 {
		t.Errorf("date_from=-2.5h 应 2 条，实际 %v", body["total"])
	}

	// 非法日期 → 400
	status, body = e.get(t, "/api/v1/admin/audit-logs?date_from=not-a-date", adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("非法 date_from 应 400，实际 %d %v", status, body)
	}
}