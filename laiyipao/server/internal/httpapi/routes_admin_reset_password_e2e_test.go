package httpapi

// 后台重置玩家密码端点的端到端测试。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 角色 → 403；
//   - 成功重置 → 200、旧密码失效、新密码可登录回同一账号；
//   - 密码太短 / 游客号 → 400 bad_input。

import (
	"context"
	"strconv"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminResetPasswordRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	status, body := e.post(t, "/api/v1/admin/users/1/reset-password", "", map[string]any{
		"new_password": "admin-reset-password-99",
	})
	if status != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d body=%v", status, body)
	}
}

func TestE2EAdminResetPasswordReadonlyForbidden(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPasswordPlayer(t, "resetro")
	_, roTok := e.newAdminRole(t, "resetro", "readonly")
	status, body := e.post(t, "/api/v1/admin/users/"+strconv.FormatInt(uid, 10)+"/reset-password", roTok, map[string]any{
		"new_password": "admin-reset-password-99",
	})
	if status != fiber.StatusForbidden {
		t.Fatalf("readonly 应 403，实际 %d body=%v", status, body)
	}
}

func TestE2EAdminResetPasswordSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPasswordPlayer(t, "reset")
	_, adminTok := e.newAdmin(t, "reset")

	const newPassword = "admin-reset-password-99"
	status, body := e.post(t, "/api/v1/admin/users/"+strconv.FormatInt(uid, 10)+"/reset-password", adminTok, map[string]any{
		"new_password": newPassword,
	})
	if status != fiber.StatusOK {
		t.Fatalf("重置应 200，实际 %d body=%v", status, body)
	}

	// 旧密码失效（newPasswordPlayer 的初始密码是 test-password-12345）
	username := e.usernameOf(t, uid)
	if _, err := e.svc.Login(context.Background(), username, "test-password-12345"); err == nil {
		t.Error("旧密码不应再能登录")
	}
	tp, err := e.svc.Login(context.Background(), username, newPassword)
	if err != nil {
		t.Fatalf("新密码应能登录：%v", err)
	}
	if tp.User.ID != uid {
		t.Errorf("登录身份应为 %d，实际 %d", uid, tp.User.ID)
	}

	// 审计必须留下记录
	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM admin_audit_logs WHERE action = 'reset_password' AND target = $1`,
		strconv.FormatInt(uid, 10)).Scan(&n); err != nil {
		t.Fatalf("查询审计失败：%v", err)
	}
	if n < 1 {
		t.Error("重置密码必须写审计日志")
	}
}

func TestE2EAdminResetPasswordValidation(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPasswordPlayer(t, "resetbad")
	_, adminTok := e.newAdmin(t, "resetbad")

	status, body := e.post(t, "/api/v1/admin/users/"+strconv.FormatInt(uid, 10)+"/reset-password", adminTok, map[string]any{
		"new_password": "short",
	})
	if status != fiber.StatusBadRequest {
		t.Fatalf("密码太短应 400，实际 %d body=%v", status, body)
	}
}

func TestE2EAdminResetPasswordGuestRejected(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "resetguest")
	_, adminTok := e.newAdmin(t, "resetguest")

	status, body := e.post(t, "/api/v1/admin/users/"+strconv.FormatInt(uid, 10)+"/reset-password", adminTok, map[string]any{
		"new_password": "admin-reset-password-99",
	})
	if status != fiber.StatusBadRequest {
		t.Fatalf("游客号重置应 400，实际 %d body=%v", status, body)
	}
}

// usernameOf 反查账号用户名（httpapi e2e 辅助）。
func (e *e2e) usernameOf(t *testing.T, id int64) string {
	t.Helper()
	var username string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT username FROM users WHERE id = $1`, id).Scan(&username); err != nil {
		t.Fatalf("反查用户名失败：%v", err)
	}
	return username
}
