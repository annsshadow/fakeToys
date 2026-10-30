package httpapi

// 后台兑换码编辑端点的端到端测试（2026-10-31 实质优化第 25 轮）。
//
// 覆盖：
//   - 未认证 → 401；readonly → 403；
//   - 更新成功 → 200 + 库中奖励/次数/过期时间更新；
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

func TestE2EAdminUpdateRedeemCodeRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/1", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminUpdateRedeemCodeReadonlyForbidden(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "redeemro", "readonly")
	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/1", nil)
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

func TestE2EAdminUpdateRedeemCodeSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "redeemedit")
	var id int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ('E2E_EDIT', '{"coin":100}'::jsonb, 10)
		 RETURNING id`).Scan(&id); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}

	raw, _ := json.Marshal(map[string]any{
		"reward": map[string]int{"gem": 50}, "max_uses": 5,
	})
	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/"+strconv.FormatInt(id, 10), bytes.NewReader(raw))
	req.Header.Set("Authorization", "Bearer "+adminTok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("更新应 200，实际 %d", resp.StatusCode)
	}

	var reward []byte
	var maxUses int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT reward, max_uses FROM redeem_codes WHERE id = $1`, id).
		Scan(&reward, &maxUses); err != nil {
		t.Fatalf("查询兑换码失败：%v", err)
	}
	if maxUses != 5 {
		t.Errorf("次数上限 = %d，期望 5", maxUses)
	}
	var rewardMap map[string]int
	if err := json.Unmarshal(reward, &rewardMap); err != nil {
		t.Fatalf("解析奖励失败：%v", err)
	}
	if rewardMap["gem"] != 50 {
		t.Errorf("奖励未更新：%v", rewardMap)
	}
}

func TestE2EAdminUpdateRedeemCodeNotFound(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "redeemednf")
	raw, _ := json.Marshal(map[string]any{"reward": map[string]int{"coin": 1}, "max_uses": 1})
	req, _ := http.NewRequest("PUT", "/api/v1/admin/redeem-codes/999999", bytes.NewReader(raw))
	req.Header.Set("Authorization", "Bearer "+adminTok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Errorf("更新不存在的兑换码应 404，实际 %d", resp.StatusCode)
	}
}
