package httpapi

// 后台用户列表按注册日期范围过滤的端到端测试（2026-10-31 新一轮第 15 轮）。

import (
	"context"
	"net/url"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminUsersDateFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "udate")
	now := time.Now()

	// 两个用户：注册于 3 小时前、1 小时前
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO users (guest_token, nickname, is_guest, created_at, last_login_at)
		 VALUES ($1, 'udate_a', TRUE, $2, $2), ($3, 'udate_b', TRUE, $4, $4)`,
		"gt_udate_a", now.Add(-3*time.Hour), "gt_udate_b", now.Add(-1*time.Hour)); err != nil {
		t.Fatalf("插入用户失败：%v", err)
	}

	// 无日期过滤 → total 至少 2
	status, body := e.get(t, "/api/v1/admin/users?limit=200", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok || totalAll < 2 {
		t.Fatalf("无过滤应至少 2 名用户，实际 %v", body["total"])
	}

	// date_from = now-2h → 只包含 1 小时前注册的
	from := now.Add(-2 * time.Hour).Format(time.RFC3339)
	status, body = e.get(t, "/api/v1/admin/users?limit=200&date_from="+url.QueryEscape(from), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("date_from 过滤应 200，实际 %d %v", status, body)
	}
	totalFiltered, ok := body["total"].(float64)
	if !ok {
		t.Fatalf("响应缺少 total：%v", body)
	}
	if totalFiltered < 1 {
		t.Errorf("date_from=-2h 应至少 1 名用户（-1h 注册的），实际 %v", totalFiltered)
	}

	// 非法日期 → 400（fail-loud）
	status, body = e.get(t, "/api/v1/admin/users?date_from=not-a-date", adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("非法 date_from 应 400，实际 %d %v", status, body)
	}
}