package httpapi

// 玩家修改昵称端点的端到端测试。
//
// 覆盖：
//   - 未认证 → 401；
//   - 成功改昵称 → 200 + 库中更新；
//   - 空昵称 / 超长（>64 字）→ 400 bad_input。

import (
	"context"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EUpdateNicknameRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	status, body := e.put(t, "/api/v1/me/nickname", "", map[string]any{"nickname": "随便"})
	if status != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d body=%v", status, body)
	}
}

func TestE2EUpdateNicknameSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, token := e.newPlayer(t, "nickname")
	status, body := e.put(t, "/api/v1/me/nickname", token, map[string]any{"nickname": "新昵称"})
	if status != fiber.StatusOK {
		t.Fatalf("改昵称应 200，实际 %d body=%v", status, body)
	}

	var got string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT nickname FROM users WHERE id = $1`, uid).Scan(&got); err != nil {
		t.Fatalf("查询昵称失败：%v", err)
	}
	if got != "新昵称" {
		t.Errorf("库中昵称 = %q，期望 %q", got, "新昵称")
	}
}

func TestE2EUpdateNicknameValidation(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, token := e.newPlayer(t, "nicknameval")
	cases := []struct {
		name     string
		nickname string
	}{
		{"空昵称", "   "},
		{"超长", strings.Repeat("名", 65)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			status, body := e.put(t, "/api/v1/me/nickname", token, map[string]any{"nickname": tc.nickname})
			if status != fiber.StatusBadRequest {
				t.Errorf("应 400，实际 %d body=%v", status, body)
			}
			if code, _ := body["error"].(map[string]any)["code"].(string); code != "bad_input" {
				t.Errorf("错误码应为 bad_input，实际 %v", body)
			}
		})
	}
}
