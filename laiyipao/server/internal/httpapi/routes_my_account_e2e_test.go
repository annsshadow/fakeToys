package httpapi

// /me 账号信息端到端测试（2026-10-31 实质优化第 34 轮）。
//
// 覆盖：
//   - /me 响应包含 account.created_at 与 account.last_login_at。

import (
	"encoding/json"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMeReturnsAccountInfo(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "acctinfo")
	req, _ := http.NewRequest("GET", "/api/v1/me", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	var body map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d", resp.StatusCode)
	}
	account, ok := body["account"].(map[string]any)
	if !ok {
		t.Fatal("/me 响应应包含 account 对象")
	}
	if account["created_at"] == nil || account["created_at"] == "" {
		t.Error("account.created_at 不应为空")
	}
	if account["last_login_at"] == nil || account["last_login_at"] == "" {
		t.Error("account.last_login_at 不应为空")
	}
}
