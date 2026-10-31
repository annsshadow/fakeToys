package httpapi

// 后台兑换码使用记录列表的端到端测试（2026-10-31 新一轮第 17 轮）。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminRedeemUsagesRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	status, _ := e.get(t, "/api/v1/admin/redeem-usages", "")
	if status != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", status)
	}
}

func TestE2EAdminRedeemUsagesSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ru_e2e")
	uid, _ := e.newPlayer(t, "ru_e2e_p")
	var codeID int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT id FROM redeem_codes LIMIT 1`).Scan(&codeID); err != nil {
		t.Fatalf("取兑换码失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, codeID, uid); err != nil {
		t.Fatalf("插入使用记录失败：%v", err)
	}

	status, body := e.get(t, "/api/v1/admin/redeem-usages", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d %v", status, body)
	}
	items, ok := body["items"].([]any)
	if !ok || len(items) != 1 {
		t.Fatalf("应 1 条使用记录，实际 %v", body["items"])
	}
	first := items[0].(map[string]any)
	if first["code"] == "" || first["code"] == nil {
		t.Errorf("记录缺少 code：%v", first)
	}
	if first["user_id"] != float64(uid) {
		t.Errorf("user_id = %v，期望 %d", first["user_id"], uid)
	}
}

func TestE2EAdminRedeemUsagesEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ru_e2e_empty")
	status, body := e.get(t, "/api/v1/admin/redeem-usages", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d %v", status, body)
	}
	if items, ok := body["items"].([]any); !ok || len(items) != 0 {
		t.Errorf("空记录应返回空 items，实际 %v", body["items"])
	}
}