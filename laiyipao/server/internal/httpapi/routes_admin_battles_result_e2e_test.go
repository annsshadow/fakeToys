package httpapi

// 后台战报列表按结果（胜/负）过滤的端到端测试（2026-11-01 新一轮第 44 轮）。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminBattlesResultFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "bresult")
	uid, _ := e.newPlayer(t, "bresultp")

	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash, created_at)
		 VALUES ($1, 1, 'win', 3, 'hw', NOW()), ($1, 2, 'lose', 0, 'hl', NOW())`,
		uid); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}

	// 无过滤 → total=2
	status, body := e.get(t, "/api/v1/admin/battles", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 2 {
		t.Fatalf("无过滤应 2 条，实际 %v", total)
	}

	// result=win → total=1 且都是 win
	status, body = e.get(t, "/api/v1/admin/battles?result=win", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("result=win 应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 1 {
		t.Errorf("result=win 应 1 条，实际 %v", total)
	}
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["result"] != "win" {
			t.Errorf("result=win 过滤下出现非胜利战报：%v", m["result"])
		}
	}

	// result=lose → total=1 且都是 lose
	status, body = e.get(t, "/api/v1/admin/battles?result=lose", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("result=lose 应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 1 {
		t.Errorf("result=lose 应 1 条，实际 %v", total)
	}
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["result"] != "lose" {
			t.Errorf("result=lose 过滤下出现非失败战报：%v", m["result"])
		}
	}

	// 非法 result 值（既不是 win 也不是 lose）→ 0 条但不报错（不是注入面）
	status, body = e.get(t, "/api/v1/admin/battles?result=draw", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("未知 result 应 200（空结果），实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 0 {
		t.Errorf("未知 result 应 0 条，实际 %v", total)
	}
}