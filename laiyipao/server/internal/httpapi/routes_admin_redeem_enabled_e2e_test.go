package httpapi

// 后台兑换码启用/禁用端点的端到端测试（2026-10-31 实质优化第 24 轮）。
//
// 覆盖：
//   - 未认证 → 401；readonly → 403；
//   - 禁用成功 → 200 + 库中 enabled=false；
//   - 不存在的兑换码 → 404。

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"strconv"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminSetRedeemEnabledRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/1/enabled", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminSetRedeemEnabledReadonlyForbidden(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "redeemro", "readonly")
	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/1/enabled", nil)
	req.Header.Set("Authorization", "Bearer "+roTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusForbidden {
		t.Fatalf("readonly 应 403，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminSetRedeemEnabledSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "redeemenable")
	var id int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ('E2E_DISABLE', '{"coin":100}'::jsonb, 10)
		 RETURNING id`).Scan(&id); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}

	raw, _ := json.Marshal(map[string]any{"enabled": false})
	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/"+strconv.FormatInt(id, 10)+"/enabled", bytes.NewReader(raw))
	req.Header.Set("Authorization", "Bearer "+adminTok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("禁用应 200，实际 %d", resp.StatusCode)
	}

	var enabled bool
	if err := e.pool.QueryRow(context.Background(),
		`SELECT enabled FROM redeem_codes WHERE id = $1`, id).Scan(&enabled); err != nil {
		t.Fatalf("查询兑换码失败：%v", err)
	}
	if enabled {
		t.Error("禁用后 enabled 应为 false")
	}
}

func TestE2EAdminSetRedeemEnabledNotFound(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "redeemnf")
	raw, _ := json.Marshal(map[string]any{"enabled": false})
	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/999999/enabled", bytes.NewReader(raw))
	req.Header.Set("Authorization", "Bearer "+adminTok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Errorf("禁用不存在的兑换码应 404，实际 %d", resp.StatusCode)
	}
}
