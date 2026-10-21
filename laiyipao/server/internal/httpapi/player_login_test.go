package httpapi

// `POST /api/v1/auth/login`（玩家账号登录）与
// `PLAYER_LOGIN_REQUIRED` 开关的端到端测试。
//
// # 为什么这两件事要放一起测
//
// 开关打开后，**游客建号必须被彻底关掉** ——
// 只在客户端挡（弹登录页）是不够的：浏览器里缓存着旧 bundle 的
// 客户端仍会调 /auth/guest，于是「每个浏览器一个号、测试数据散在各处」
// 会在**看起来已经修好**之后继续存在。
//
// 所以服务端这一道是权威判据，客户端那一层只是体验。

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
)

// newAccountedPlayer 建一个带用户名口令的玩家账号，返回用户名与口令。
func newAccountedPlayer(t *testing.T, e *e2e, tag string) (string, string) {
	t.Helper()
	username := fmt.Sprintf("acct_%s_%d", tag, time.Now().UnixNano())
	const password = "test-password-12345"
	if _, err := e.svc.EnsureBootstrapPlayer(context.Background(), username, password); err != nil {
		t.Fatalf("引导玩家账号失败：%v", err)
	}
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM users WHERE username = $1`, username)
	})
	return username, password
}

// TestPlayerLoginEndpoint 账号登录端点：口令正确给令牌，错误给 401。
func TestPlayerLoginEndpoint(t *testing.T) {
	e := newE2E(t)
	username, password := newAccountedPlayer(t, e, "login")

	status, body := e.post(t, "/api/v1/auth/login", "", map[string]any{
		"username": username,
		"password": password,
	})
	if status != fiber.StatusOK {
		t.Fatalf("口令正确应 200，实际 %d：%v", status, body)
	}
	tok, _ := body["access_token"].(string)
	if tok == "" {
		t.Fatalf("响应里没有 access_token：%v", body)
	}
	user, _ := body["user"].(map[string]any)
	if user == nil || user["id"] == nil {
		t.Fatalf("响应里没有 user：%v", body)
	}

	// 令牌必须真的能用：拿它读一次 /me。
	st, me := e.get(t, "/api/v1/me", tok)
	if st != fiber.StatusOK {
		t.Fatalf("用新令牌访问 /me 应 200，实际 %d：%v", st, me)
	}

	// 口令错 → 401，且**不**泄露用户是否存在。
	st2, body2 := e.post(t, "/api/v1/auth/login", "", map[string]any{
		"username": username,
		"password": "wrong-password-0000",
	})
	if st2 != fiber.StatusUnauthorized {
		t.Fatalf("口令错应 401，实际 %d：%v", st2, body2)
	}
	errObj, _ := body2["error"].(map[string]any)
	if errObj == nil || errObj["code"] == nil {
		t.Fatalf("错误响应形状不对：%v", body2)
	}
}

// TestGuestLoginRejectedWhenLoginRequired 开关打开时游客建号被彻底关掉。
func TestGuestLoginRejectedWhenLoginRequired(t *testing.T) {
	e := newE2EWith(t, func(c *config.Config) { c.PlayerLoginRequired = true })

	status, body := e.post(t, "/api/v1/auth/guest", "", map[string]any{
		"guest_token": fmt.Sprintf("should_not_exist_%d", time.Now().UnixNano()),
		"nickname":    "游客",
	})
	if status != fiber.StatusForbidden {
		t.Fatalf("要求账号登录时 /auth/guest 应 403，实际 %d：%v", status, body)
	}
	errObj, _ := body["error"].(map[string]any)
	if errObj == nil {
		t.Fatalf("错误响应形状不对：%v", body)
	}
	if code, _ := errObj["code"].(string); code != "login_required" {
		t.Errorf("错误码应是 login_required（客户端据此跳登录页），实际 %q", code)
	}

	// ⚠️ 还要断言「**没有真的建号**」——
	// 只回 403 但把账号建出来，等于「看起来禁用了、实际还在造号」。
	var count int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT count(*) FROM users WHERE nickname = $1`, "游客").Scan(&count); err != nil {
		t.Fatalf("查库失败：%v", err)
	}
	if count > 0 {
		t.Errorf("被拒的游客建号仍然落了库（%d 行）—— 开关没有真正生效", count)
	}
}

// TestGuestLoginAllowedByDefault 开关缺省关闭时，游客通道**不受影响**。
//
// 这条是回归守卫：新加的开关不能顺手把默认行为改了
// （默认关闭 = 现有部署照旧工作）。
func TestGuestLoginAllowedByDefault(t *testing.T) {
	e := newE2E(t)

	status, body := e.post(t, "/api/v1/auth/guest", "", map[string]any{
		"guest_token": fmt.Sprintf("guest_ok_%d", time.Now().UnixNano()),
		"nickname":    "游客",
	})
	if status != fiber.StatusOK {
		t.Fatalf("默认应仍能游客建号，实际 %d：%v", status, body)
	}
	id, _ := body["user"].(map[string]any)["id"].(float64)
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM users WHERE id = $1`, int64(id))
	})
}

// TestConfigAdvertisesRequireLogin /config 必须告诉客户端「本环境要账号登录」。
//
// 客户端据此**先**显示登录页，而不是先撞一次 403 再跳转。
// 但它只是提示：权威判据仍是服务端那道 403（配置副本可能过期）。
func TestConfigAdvertisesRequireLogin(t *testing.T) {
	e := newE2EWith(t, func(c *config.Config) { c.PlayerLoginRequired = true })
	status, body := e.get(t, "/api/v1/config", "")
	if status != fiber.StatusOK {
		t.Fatalf("/config 应 200，实际 %d", status)
	}
	if v, ok := body["require_login"].(bool); !ok || !v {
		t.Errorf("/config 应下发 require_login=true，实际 %v", body["require_login"])
	}
}
