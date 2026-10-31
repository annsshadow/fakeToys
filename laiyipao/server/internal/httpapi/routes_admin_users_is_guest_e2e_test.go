package httpapi

// 后台用户列表按游客/账号筛选的端到端测试（2026-11-01 新一轮第 37 轮）。

import (
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminUsersIsGuestFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "guest_f")
	// newPlayer 创建的是游客（is_guest=true）
	guestID, _ := e.newPlayer(t, "guest_f_p")

	// 只看游客 → 应包含该游客
	status, body := e.get(t, "/api/v1/admin/users?limit=200&is_guest=1", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("游客筛选应 200，实际 %d %v", status, body)
	}
	items, _ := body["items"].([]any)
	found := false
	for _, it := range items {
		m := it.(map[string]any)
		if m["id"] == float64(guestID) {
			found = true
		}
		if m["is_guest"] != true {
			t.Errorf("游客筛选返回了非游客：%v", m)
		}
	}
	if !found {
		t.Errorf("游客筛选应包含游客 %d，实际 %v", guestID, body["items"])
	}

	// 只看账号 → 不应包含该游客
	status, body = e.get(t, "/api/v1/admin/users?limit=200&is_guest=0", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("账号筛选应 200，实际 %d %v", status, body)
	}
	items, _ = body["items"].([]any)
	for _, it := range items {
		m := it.(map[string]any)
		if m["id"] == float64(guestID) {
			t.Errorf("账号筛选不应包含游客 %d", guestID)
		}
	}
}