package httpapi

// 第 127 轮：readonly 角色门禁。
//
// admin_users.role 自 00007 迁移就标了 admin/ops/readonly 三个角色，
// 但历史上**没有任何端点消费它** —— readonly 账号可以和 admin 一样
// 封禁玩家、发币、改关卡/商城（「纸面角色」缺陷）。
//
// 修法：requireAdmin 把角色写进 Locals，写端点挂 requireWritable，
// readonly 一律 403；读端点保持放行。
// 本文件把「哪些端点挡、哪些放行、不挡错 ops」钉死。

import (
	"fmt"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EReadonlyGating(t *testing.T) {
	e := newE2E(t)
	_, roTok := e.newAdminRole(t, "ro127", "readonly")
	_, opsTok := e.newAdminRole(t, "ops127", "ops")

	// --- readonly：所有写端点必须 403（门禁在 handler 前，参数可非法） ---
	uid, _ := e.newPlayer(t, "rovictim")
	roWrites := []struct {
		method, url string
		body        any
	}{
		{"POST", fmt.Sprintf("/api/v1/admin/users/%d/ban", uid), map[string]any{}},
		{"POST", fmt.Sprintf("/api/v1/admin/users/%d/unban", uid), nil},
		{"POST", fmt.Sprintf("/api/v1/admin/users/%d/grant", uid), map[string]any{"currency": "coin", "amount": 1}},
		{"PUT", "/api/v1/admin/levels/1", map[string]any{"name": "x"}},
		{"POST", "/api/v1/admin/levels/regenerate", nil},
		{"PUT", "/api/v1/admin/skills/1", map[string]any{"name": "x"}},
		{"PUT", "/api/v1/admin/shop/1", map[string]any{"name": "x"}},
		{"POST", "/api/v1/admin/announcements", map[string]any{"title": "x", "body": "y"}},
		{"POST", "/api/v1/admin/redeem-codes", map[string]any{"code": "RO", "reward": map[string]int{"coin": 1}}},
		{"POST", "/api/v1/admin/battles/1/verify", map[string]any{"replay_hash": "0000000000000000"}},
	}
	for _, w := range roWrites {
		var status int
		var body map[string]any
		switch w.method {
		case "PUT":
			status, body = e.put(t, w.url, roTok, w.body)
		default:
			status, body = e.post(t, w.url, roTok, w.body)
		}
		if status != fiber.StatusForbidden {
			t.Errorf("readonly %s %s 应 403，实际 %d %v", w.method, w.url, status, body)
		}
	}

	// --- readonly：读端点保持放行（各 200） ---
	roReads := []string{
		"/api/v1/admin/dashboard",
		"/api/v1/admin/levels",
		"/api/v1/admin/levels/1/waves",
		"/api/v1/admin/skills",
		"/api/v1/admin/equipment",
		"/api/v1/admin/users",
		"/api/v1/admin/battles",
		"/api/v1/admin/battles/1",
		"/api/v1/admin/defenses",
		"/api/v1/admin/economy",
		"/api/v1/admin/announcements",
		"/api/v1/admin/redeem-codes",
		"/api/v1/admin/audit-logs",
	}
	for _, url := range roReads {
		status, body := e.get(t, url, roTok)
		if status != 200 {
			t.Errorf("readonly 读端点 %s 应 200，实际 %d %v", url, status, body)
		}
	}

	// --- ops：同一个写端点必须 200（证明没有过度拒绝，门禁只挡 readonly） ---
	status, body := e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/unban", uid), opsTok, nil)
	if status != 200 {
		t.Errorf("ops 账号解封应 200（不被 requireWritable 误伤），实际 %d %v", status, body)
	}
	status, body = e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/ban", uid), opsTok, map[string]any{})
	if status != 200 {
		t.Errorf("ops 账号封禁应 200，实际 %d %v", status, body)
	}
}
