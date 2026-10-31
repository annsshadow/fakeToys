package httpapi

// 后台用户列表按「需核查」筛选的端到端测试（2026-11-01 新一轮第 60 轮）。

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminUsersSuspiciousFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ususp")
	now := time.Now()

	// 用户 A：战力 6000 最高关 1（需核查）；用户 B：正常
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO users (guest_token, nickname, is_guest, created_at, last_login_at)
		 VALUES ('gt_ususp_a', 'susp_a', TRUE, $1, $1), ('gt_ususp_b', 'norm_b', TRUE, $1, $1)`,
		now); err != nil {
		t.Fatalf("插入用户失败：%v", err)
	}
	var idA, idB int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT id FROM users WHERE guest_token='gt_ususp_a'`).Scan(&idA); err != nil {
		t.Fatalf("回读用户 A 失败：%v", err)
	}
	if err := e.pool.QueryRow(context.Background(),
		`SELECT id FROM users WHERE guest_token='gt_ususp_b'`).Scan(&idB); err != nil {
		t.Fatalf("回读用户 B 失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO user_progress (user_id, max_stage, level_exp) VALUES ($1, 1, 6000), ($2, 10, 100)`,
		idA, idB); err != nil {
		t.Fatalf("写入 user_progress 失败：%v", err)
	}

	// suspicious=true → 只含 A
	status, body := e.get(t, "/api/v1/admin/users?limit=200&suspicious=1", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("suspicious=true 应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 1 {
		t.Errorf("suspicious=true 应 1 名用户，实际 %v", total)
	}
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["nickname"] == "norm_b" {
			t.Error("suspicious=true 不应包含正常用户")
		}
	}

	// suspicious=0 → 排除 A（至少 1 名正常用户）
	status, body = e.get(t, "/api/v1/admin/users?limit=200&suspicious=0", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("suspicious=0 应 200，实际 %d %v", status, body)
	}
	totalNorm, ok := body["total"].(float64)
	if !ok || totalNorm < 1 {
		t.Errorf("suspicious=0 应至少 1 名正常用户，实际 %v", body["total"])
	}
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["nickname"] == "susp_a" {
			t.Error("suspicious=0 不应包含需核查用户")
		}
	}
}