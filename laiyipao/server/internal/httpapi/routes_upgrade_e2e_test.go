package httpapi

// 游客账号转正端点的端到端测试。
//
// 覆盖：
//   - 未认证 → 401；
//   - 游客成功转正 → 200、is_guest 变 false、新用户名密码可登录回同一账号；
//   - 用户名已被占用 → 400 bad_input；
//   - 输入校验（空用户名 / 密码太短）→ 400 bad_input。

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EUpgradeGuestAccountRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	status, body := e.post(t, "/api/v1/me/upgrade", "", map[string]any{
		"username": "valid_name",
		"password": "new-password-12345",
	})
	if status != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d body=%v", status, body)
	}
}

func TestE2EUpgradeGuestAccountSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, token := e.newPlayer(t, "upgrade")
	username := fmt.Sprintf("upgraded_e2e_%d", time.Now().UnixNano())
	const password = "new-password-12345"

	status, body := e.post(t, "/api/v1/me/upgrade", token, map[string]any{
		"username": username,
		"password": password,
	})
	if status != fiber.StatusOK {
		t.Fatalf("转正应 200，实际 %d body=%v", status, body)
	}

	// 库中 is_guest 必须已翻转
	var isGuest bool
	if err := e.pool.QueryRow(context.Background(),
		`SELECT is_guest FROM users WHERE id = $1`, uid).Scan(&isGuest); err != nil {
		t.Fatalf("查询 is_guest 失败：%v", err)
	}
	if isGuest {
		t.Error("升级后 is_guest 应为 false")
	}

	// 新用户名密码必须能登录回同一账号
	tp, err := e.svc.Login(context.Background(), username, password)
	if err != nil {
		t.Fatalf("升级后新密码登录失败：%v", err)
	}
	if tp.User.ID != uid {
		t.Errorf("登录身份应为 %d，实际 %d", uid, tp.User.ID)
	}
}

func TestE2EUpgradeGuestAccountUsernameTaken(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	takenUsername := fmt.Sprintf("taken_e2e_%d", time.Now().UnixNano())
	if _, err := e.svc.EnsureBootstrapPlayer(context.Background(), takenUsername, "test-password-12345"); err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}

	_, token := e.newPlayer(t, "upgradetaken")
	status, body := e.post(t, "/api/v1/me/upgrade", token, map[string]any{
		"username": takenUsername,
		"password": "new-password-12345",
	})
	if status != fiber.StatusBadRequest {
		t.Fatalf("用户名被占用应 400，实际 %d body=%v", status, body)
	}
	if code, _ := body["error"].(map[string]any)["code"].(string); code != "bad_input" {
		t.Errorf("错误码应为 bad_input，实际 %v", body)
	}
}

func TestE2EUpgradeGuestAccountValidation(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, token := e.newPlayer(t, "upgradebad")
	cases := []struct {
		name               string
		username, password string
	}{
		{"空用户名", "", "new-password-12345"},
		{"密码太短", "valid_name", "short"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			status, body := e.post(t, "/api/v1/me/upgrade", token, map[string]any{
				"username": tc.username,
				"password": tc.password,
			})
			if status != fiber.StatusBadRequest {
				t.Errorf("应 400，实际 %d body=%v", status, body)
			}
			if code, _ := body["error"].(map[string]any)["code"].(string); code != "bad_input" {
				t.Errorf("错误码应为 bad_input，实际 %v", body)
			}
		})
	}
}
