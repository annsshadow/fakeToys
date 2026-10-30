package httpapi

// 玩家修改密码端点的端到端测试。
//
// 覆盖：
//   - 未认证 → 401；
//   - 游客号（无密码）→ 400 bad_input；
//   - 旧密码错误 → 400 bad_input；
//   - 正确改密 → 200 + 库中哈希更新 + refresh token 全部吊销 + 新密码可登录。

import (
	"context"
	"fmt"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func (e *e2e) newPasswordPlayer(t *testing.T, tag string) (int64, string) {
	t.Helper()
	ctx := context.Background()
	username := fmt.Sprintf("e2e_pw_%s_%d", tag, time.Now().UnixNano())
	const password = "test-password-12345"

	created, err := e.svc.EnsureBootstrapPlayer(ctx, username, password)
	if err != nil {
		t.Fatalf("引导玩家账号失败：%v", err)
	}
	if !created {
		t.Fatal("首次引导应报告「新建」")
	}
	tp, err := e.svc.Login(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	return tp.User.ID, tp.AccessToken
}

func TestE2EChangePasswordRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	status, body := e.post(t, "/api/v1/me/password", "", map[string]any{
		"old_password": "x",
		"new_password": "y",
	})
	if status != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d body=%v", status, body)
	}
}

func TestE2EChangePasswordSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, token := e.newPasswordPlayer(t, "ok")
	const newPassword = "brand-new-password-987"

	status, body := e.post(t, "/api/v1/me/password", token, map[string]any{
		"old_password": "test-password-12345",
		"new_password": newPassword,
	})
	if status != fiber.StatusOK {
		t.Fatalf("改密应 200，实际 %d body=%v", status, body)
	}
	revoked, ok := body["revoked"].(float64)
	if !ok || revoked < 1 {
		t.Errorf("响应应带 revoked>=1，实际 %v", body)
	}

	// 库中哈希必须已更新（旧密码登录失败）
	var storedUsername string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT username FROM users WHERE id = $1`, uid).Scan(&storedUsername); err != nil {
		t.Fatalf("反查用户名失败：%v", err)
	}
	if _, err := e.svc.Login(context.Background(), storedUsername, "test-password-12345"); err == nil {
		t.Error("旧密码不应再能登录")
	}
	tp, err := e.svc.Login(context.Background(), storedUsername, newPassword)
	if err != nil {
		t.Fatalf("新密码应能登录：%v", err)
	}
	if tp.User.ID != uid {
		t.Errorf("登录身份应为 %d，实际 %d", uid, tp.User.ID)
	}
}

func TestE2EChangePasswordWrongOld(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, token := e.newPasswordPlayer(t, "wrongold")
	status, body := e.post(t, "/api/v1/me/password", token, map[string]any{
		"old_password": "wrong-old-password",
		"new_password": "brand-new-password-987",
	})
	if status != fiber.StatusBadRequest {
		t.Fatalf("旧密码错误应 400，实际 %d body=%v", status, body)
	}
	if code, _ := body["error"].(map[string]any)["code"].(string); code != "bad_input" {
		t.Errorf("错误码应为 bad_input，实际 %v", body)
	}
}

func TestE2EChangePasswordGuestRejected(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, token := e.newPlayer(t, "guestpw")
	status, body := e.post(t, "/api/v1/me/password", token, map[string]any{
		"old_password": "whatever",
		"new_password": "brand-new-password-987",
	})
	if status != fiber.StatusBadRequest {
		t.Fatalf("游客号改密应 400，实际 %d body=%v", status, body)
	}
	if code, _ := body["error"].(map[string]any)["code"].(string); code != "bad_input" {
		t.Errorf("错误码应为 bad_input，实际 %v", body)
	}
}

func TestE2EChangePasswordMethodNotAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, token := e.newPasswordPlayer(t, "method")
	status, body := e.get(t, "/api/v1/me/password", token)
	if status != http.StatusMethodNotAllowed {
		t.Fatalf("GET 改密码应 405，实际 %d body=%v", status, body)
	}
}
