package httpapi

// 账号登录刷新 last_login_at 的端到端测试（2026-10-31 新一轮第 12 轮）。
//
// 游客/微信登录都会刷新 last_login_at，唯独账号密码登录此前只读不写，
// `/me` 的「最近登录」对账号玩家恒等于注册时间。修复后登录应刷新。

import (
	"context"
	"fmt"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2ELoginUpdatesLastLoginAt(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, token := e.newPlayer(t, "logintouch")
	username := fmt.Sprintf("lte_%d", time.Now().UnixNano())
	const password = "test-password-12345"

	// 升级为账号（写入 username/password_hash）
	status, body := e.post(t, "/api/v1/me/upgrade", token, map[string]any{
		"username": username,
		"password": password,
	})
	if status != fiber.StatusOK {
		t.Fatalf("升级账号失败：%d %v", status, body)
	}

	// 把 last_login_at 改成 1 小时前，模拟「上次登录是很久以前」
	if _, err := e.pool.Exec(context.Background(),
		`UPDATE users SET last_login_at = now() - interval '1 hour' WHERE id = $1`, uid); err != nil {
		t.Fatalf("改 last_login_at 失败：%v", err)
	}

	// 账号登录
	if _, err := e.svc.Login(context.Background(), username, password); err != nil {
		t.Fatalf("账号登录失败：%v", err)
	}

	// 登录后 last_login_at 应刷新为现在
	var lastLoginAt time.Time
	if err := e.pool.QueryRow(context.Background(),
		`SELECT last_login_at FROM users WHERE id = $1`, uid).Scan(&lastLoginAt); err != nil {
		t.Fatalf("读 last_login_at 失败：%v", err)
	}
	if time.Since(lastLoginAt) > time.Minute {
		t.Errorf("账号登录后 last_login_at 应刷新（原 1 小时前），实际 %v", lastLoginAt)
	}

	// 顺带确认 /me 端点正常（account.last_login_at 来自同一字段）
	req, _ := http.NewRequest("GET", "/api/v1/me", nil)
	req.Header.Set("Authorization", "Bearer "+token)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("GET /me 失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("GET /me 应 200，实际 %d", resp.StatusCode)
	}
}