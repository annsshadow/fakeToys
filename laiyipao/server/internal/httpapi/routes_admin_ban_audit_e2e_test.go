// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// R36：封禁与审计必须同一事务。
//
// 封禁是止损动作：如果 users.status 落库了、审计却没写进去，
// 事后无法回答「谁在什么时候因为什么封了这个人」，而这次止损在库里
// 看起来就像系统自动发生的。修前 handler 在业务提交后才补审计，
// 审计失败只打日志 —— 封禁照常生效。
func TestE2EBanUserAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, _ := e.newPlayer(t, "ban_audit")
	adminID, tok := e.newAdmin(t, "ban_audit_admin")

	userStatus := func() int {
		return e.scalarInt(t, `SELECT status FROM users WHERE id = $1`, uid)
	}
	countAudits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'ban_user'`,
			adminID)
	}
	banURL := fmt.Sprintf("/api/v1/admin/users/%d/ban", uid)

	before := userStatus()
	if before != 1 {
		t.Fatalf("封禁前 status 应为 1，实际 %d", before)
	}

	// 注入：审计表不存在，业务表正常。
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	status, body := e.post(t, banURL, tok, map[string]any{"reason": "注入测试"})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时必须整体失败（500），实际 %d %v", status, body)
	}
	if got := userStatus(); got != before {
		t.Errorf("审计失败后封禁必须回滚：基线 status=%d，实际 %d", before, got)
	}
	if got := countAudits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	// 故障解除后：封禁生效 + 一条审计。
	status, body = e.post(t, banURL, tok, map[string]any{"reason": "最终封禁"})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后封禁应成功，实际 %d %v", status, body)
	}
	if got := userStatus(); got != 2 {
		t.Errorf("封禁生效后 status 应为 2，实际 %d", got)
	}
	if got := countAudits(); got != 1 {
		t.Errorf("成功封禁应恰有 1 行审计，实际 %d", got)
	}
}
