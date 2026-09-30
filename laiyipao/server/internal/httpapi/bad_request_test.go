package httpapi

// 各 handler 的「请求体/参数解析失败」分支补充覆盖。
//
// 这些分支在 happy-path E2E 里走不到（没人故意发坏 JSON），
// 但它们是用户可达的 —— 客户端版本错乱、恶意探测都会打到。
// 每条断言的契约统一：解析失败必须 400，
// 绝不能 500（把"客户端发错了"当成"服务坏了"会污染告警）。

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// rawJSONRequest 构造携带原始字节流的请求（发送非法 JSON 用）。
func rawJSONRequest(method, url, body string) *http.Request {
	req := httptest.NewRequest(method, url, strings.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	return req
}

func TestE2EBadJSONBodies(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newPlayer(t, "badjson")
	_, adminTok := e.newAdmin(t, "badjson")
	_, foeTok := e.newPlayer(t, "badfoe")

	cases := []struct {
		name   string
		method string
		url    string
		token  string
	}{
		{"wechat", "POST", "/api/v1/auth/wechat", ""},
		{"refresh", "POST", "/api/v1/auth/refresh", ""},
		{"saveLoadout", "PUT", "/api/v1/me/loadout", tok},
		{"startBattle", "POST", "/api/v1/battle/token", tok},
		{"settleBattle", "POST", "/api/v1/battle/settle", tok},
		{"verifyReplay", "POST", "/api/v1/battle/verify", tok},
		{"redeem", "POST", "/api/v1/redeem/", tok},
		{"saveDefense", "POST", "/api/v1/defenses/save", tok},
		{"challengeDefense", "POST", "/api/v1/defenses/1/challenge", foeTok},
		{"allocateMastery", "POST", "/api/v1/mastery/allocate", tok},
		{"adminLogin", "POST", "/api/v1/admin/login", ""},
		{"adminUpdateLevel", "PUT", "/api/v1/admin/levels/1", adminTok},
		{"adminUpdateSkill", "PUT", "/api/v1/admin/skills/1", adminTok},
		{"adminUpdateShop", "PUT", "/api/v1/admin/shop/1", adminTok},
		{"adminCreateAnnouncement", "POST", "/api/v1/admin/announcements", adminTok},
		{"adminCreateRedeemCode", "POST", "/api/v1/admin/redeem-codes", adminTok},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			req := rawJSONRequest(c.method, c.url, "{invalid-json")
			req.Header.Set("Authorization", "Bearer "+c.token)
			resp, err := e.app.Test(req, 5000)
			if err != nil {
				t.Fatalf("请求失败：%v", err)
			}
			defer resp.Body.Close()
			if resp.StatusCode != fiber.StatusBadRequest {
				t.Errorf("%s 坏 JSON 应 400，实际 %d", c.name, resp.StatusCode)
			}
		})
	}

	// 非法 :id 参数（ParseInt/Atoi 失败）—— 与坏 JSON 同类的用户可达分支
	status, _ := e.do(t, "POST", "/api/v1/admin/users/abc/unban", adminTok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("unban 非法 id 应 400，实际 %d", status)
	}
	status, _ = e.do(t, "POST", "/api/v1/tasks/abc/claim", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("claimTask 非法 id 应 400，实际 %d", status)
	}
}
