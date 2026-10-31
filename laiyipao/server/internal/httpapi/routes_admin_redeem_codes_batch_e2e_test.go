package httpapi

// 后台兑换码批量生成的端到端测试（2026-11-01 新一轮第 57 轮）。
//
// 覆盖：
//   - 未认证 → 401；readonly → 403；
//   - 批量生成成功 → 200 + 返回 count 个不重复码；
//   - count 越界 → 400。

import (
	"bytes"
	"encoding/json"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminCreateRedeemCodesBatchRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("POST", "/api/v1/admin/redeem-codes/batch", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminCreateRedeemCodesBatchReadonlyForbidden(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "redeembatchro", "readonly")
	body := bytes.NewBufferString(`{"count":3,"reward":{"coin":100},"max_uses":1}`)
	req, _ := http.NewRequest("POST", "/api/v1/admin/redeem-codes/batch", body)
	req.Header.Set("Authorization", "Bearer "+roTok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusForbidden {
		t.Fatalf("readonly 应 403，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminCreateRedeemCodesBatchSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "redeembatch")
	body := bytes.NewBufferString(`{"count":5,"reward":{"coin":100,"gem":5},"max_uses":3}`)
	req, _ := http.NewRequest("POST", "/api/v1/admin/redeem-codes/batch", body)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()

	var out struct {
		Items []map[string]any `json:"items"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&out); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d", resp.StatusCode)
	}
	if len(out.Items) != 5 {
		t.Fatalf("应生成 5 个码，实际 %d", len(out.Items))
	}
	seen := map[string]bool{}
	for _, it := range out.Items {
		code := it["code"].(string)
		if seen[code] {
			t.Errorf("出现重复码：%s", code)
		}
		seen[code] = true
		if len(code) != 12 {
			t.Errorf("码长度应为 12，实际 %q", code)
		}
		if it["max_uses"] != float64(3) {
			t.Errorf("max_uses 应为 3，实际 %v", it["max_uses"])
		}
		reward := it["reward"].(map[string]any)
		if reward["coin"] != float64(100) || reward["gem"] != float64(5) {
			t.Errorf("奖励不正确：%v", reward)
		}
	}
}

func TestE2EAdminCreateRedeemCodesBatchInvalidCount(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "redeembatchbad")
	for _, payload := range []string{
		`{"count":0,"reward":{"coin":1},"max_uses":1}`,
		`{"count":101,"reward":{"coin":1},"max_uses":1}`,
	} {
		req, _ := http.NewRequest("POST", "/api/v1/admin/redeem-codes/batch", bytes.NewBufferString(payload))
		req.Header.Set("Authorization", "Bearer "+adminTok)
		req.Header.Set("Content-Type", "application/json")
		resp, err := e.app.Test(req, 10000)
		if err != nil {
			t.Fatalf("请求失败：%v", err)
		}
		resp.Body.Close()
		if resp.StatusCode != fiber.StatusBadRequest {
			t.Errorf("count 越界应 400，实际 %d", resp.StatusCode)
		}
	}
}