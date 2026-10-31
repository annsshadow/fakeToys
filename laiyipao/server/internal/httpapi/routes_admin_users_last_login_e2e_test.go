package httpapi

// 后台用户列表按最后登录时间范围过滤的端到端测试（2026-11-01 新一轮第 46 轮）。

import (
	"context"
	"net/url"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminUsersLastLoginFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ull")
	now := time.Now()

	// 两个用户：最后登录于 3 小时前、1 小时前
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO users (guest_token, nickname, is_guest, created_at, last_login_at)
		 VALUES ($1, 'ull_a', TRUE, $2, $3), ($4, 'ull_b', TRUE, $2, $5)`,
		"gt_ull_a", now.Add(-4*time.Hour), now.Add(-3*time.Hour),
		"gt_ull_b", now.Add(-1*time.Hour)); err != nil {
		t.Fatalf("插入用户失败：%v", err)
	}

	// last_login_from = now-2h → 只包含 1 小时前登录的
	from := now.Add(-2 * time.Hour).Format(time.RFC3339)
	status, body := e.get(t, "/api/v1/admin/users?limit=200&last_login_from="+url.QueryEscape(from), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("last_login_from 过滤应 200，实际 %d %v", status, body)
	}
	totalFiltered, ok := body["total"].(float64)
	if !ok {
		t.Fatalf("响应缺少 total：%v", body)
	}
	if totalFiltered < 1 {
		t.Errorf("last_login_from=-2h 应至少 1 名用户（-1h 登录的），实际 %v", totalFiltered)
	}
	// 列表里不应出现 -3h 登录的用户
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["nickname"] == "ull_a" {
			t.Error("last_login_from=-2h 过滤下不应出现 -3h 登录的用户")
		}
	}

	// 非法 last_login_from → 400（fail-loud）
	status, body = e.get(t, "/api/v1/admin/users?last_login_from=not-a-date", adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("非法 last_login_from 应 400，实际 %d %v", status, body)
	}
}