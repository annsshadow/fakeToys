package httpapi

// 第 155 轮：后台登出（admin_tokens 接线）的端到端守卫。
//
// 这一组用例钉的是**服务端真切断**，不是「前端清了 localStorage」：
// 登出后，那枚**签名仍然合法**的 access token 在下一个请求上就必须 401。
// 判据全部落在下一次真实请求上（能观测的那一层），不落在「表被 UPDATE 了」
// 这个实现形状上。

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/auth"
)

// sidOf 解出 access token 载荷里的会话 id（第 155 轮 AdminLogin 恒携带）。
func sidOf(t *testing.T, token string) string {
	t.Helper()
	parts := strings.SplitN(token, ".", 3)
	if len(parts) != 3 {
		t.Fatalf("令牌不是三段式 JWT：%q", token)
	}
	raw, err := base64.RawURLEncoding.DecodeString(parts[1])
	if err != nil {
		t.Fatalf("解 JWT 载荷失败：%v", err)
	}
	var claims struct {
		Sid string `json:"sid"`
	}
	if err := json.Unmarshal(raw, &claims); err != nil || claims.Sid == "" {
		t.Fatalf("登录令牌必须携带 sid（会话行关联键），实际载荷 %s / %v", raw, err)
	}
	return claims.Sid
}

func TestE2EAdminLogoutRevokesSessions(t *testing.T) {
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "logout")
	sid := sidOf(t, tok)

	// 无 token 打登出端点：requireAdmin 拦在 handler 前 → 401
	status, _ := e.post(t, "/api/v1/admin/logout", "", nil)
	if status != fiber.StatusUnauthorized {
		t.Errorf("无 token 登出应 401，实际 %d", status)
	}

	// 正常登出：吊销**该管理员**的全部会话
	status, body := e.post(t, "/api/v1/admin/logout", tok, nil)
	if status != 200 {
		t.Fatalf("登出应 200，实际 %d %v", status, body)
	}
	if n := num(body, "revoked_sessions"); n < 1 {
		t.Errorf("登出应至少吊销 1 条会话（本测试登录过一次），实际 %d %v", n, body)
	}

	// 服务端真切断：同一枚签名仍合法的 token 立刻 401
	status, _ = e.get(t, "/api/v1/admin/me", tok)
	if status != fiber.StatusUnauthorized {
		t.Errorf("登出后旧令牌访问 /admin/me 应 401，实际 %d", status)
	}

	// 会话行确实被吊销（判据停在「登出后旧令牌 401」已足够，
	// 这里再补一条库层断言，让「401 来自哪一层」对排错的人可读）
	if got := e.scalarInt(t,
		`SELECT COUNT(*) FROM admin_tokens WHERE token_hash = $1 AND revoked_at IS NULL`,
		auth.HashRefreshToken(sid)); got != 0 {
		t.Errorf("登出后该 sid 的有效会话行应为 0，实际 %d", got)
	}

	// readonly 账号**可以**登出自己（会话生命周期不是数据写，
	// 这是豁免表里 /logout 的存在理由 —— 若被 requireWritable 403，
	// 误开的后台标签页里 readonly 的凭证将滞留 2h）
	_, roTok := e.newAdminRole(t, "logoutro", "readonly")
	status, body = e.post(t, "/api/v1/admin/logout", roTok, nil)
	if status != 200 {
		t.Errorf("readonly 登出自身应 200，实际 %d %v", status, body)
	}

	// 重新登录即恢复（新会话行）
	_, fresh := e.newAdminRole(t, "logoutfresh", "admin")
	status, _ = e.get(t, "/api/v1/admin/me", fresh)
	if status != 200 {
		t.Errorf("重新登录后的令牌应 200，实际 %d", status)
	}
	_ = adminID
	_ = sid
}

// TestE2EAdminRoleWhitelistIsFailClosed 第 155 轮：
// requireWritable 改成白名单（admin/ops）后的角色门禁全集。
func TestE2EAdminRoleWhitelistIsFailClosed(t *testing.T) {
	e := newE2E(t)
	uid, _ := e.newPlayer(t, "wrolevictim")
	writable := fmt.Sprintf("/api/v1/admin/users/%d/unban", uid)

	// readonly / ops / admin 三种**已知**角色
	cases := []struct {
		role string
		want int
	}{
		{"readonly", fiber.StatusForbidden},
		{"ops", 200},
		{"admin", 200},
	}
	for i, c := range cases {
		_, tok := e.newAdminRole(t, fmt.Sprintf("wrole%d", i), c.role)
		status, body := e.post(t, writable, tok, nil)
		if status != c.want {
			t.Errorf("角色 %q 的写操作应 %d，实际 %d %v", c.role, c.want, status, body)
		}
	}

	// 未知角色值（库里被改脏的 role）必须 **fail-closed**：
	// 第 154 轮前这里的语义是「只拒 readonly，其余放行」——
	// 意味着一个拼错/截断的 role 值会悄悄获得全部写权限。
	_, unknown := e.newAdminRole(t, "wrole-unknown", "superadmin")
	status, body := e.post(t, writable, unknown, nil)
	if status != fiber.StatusForbidden {
		t.Errorf("未知角色 %q 的写操作应 403（fail-closed），实际 %d %v", "superadmin", status, body)
	}

	// 读端点对未知角色**保持放行**（门禁只作用于写，别过度拒绝）
	status, body = e.get(t, "/api/v1/admin/dashboard", unknown)
	if status != 200 {
		t.Errorf("未知角色读端点应 200，实际 %d %v", status, body)
	}
}
