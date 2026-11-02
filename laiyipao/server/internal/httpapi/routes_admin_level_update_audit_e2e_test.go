// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// R43：关卡参数修改与审计必须同生同死。
//
// 改一关的血量/波次/星级门槛是运营日常动作，直接决定玩家遇到的难度。
// 修前审计在 handler 提交后补写、失败只打日志 —— 参数变了而「谁改的、
// 改成什么」没有任何记录，事后只能靠猜。
//
// 失败注入沿用 R34/R36/R37/R39/R40/R41/R42：把 admin_audit_logs 改名。
func TestE2EAdminUpdateLevelAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "level_audit")

	const sentinel = "R43 SENTINEL"
	e.exec(t, `UPDATE levels SET name = $1 WHERE id = 1`, sentinel)
	levelName := func() string {
		var name string
		if err := e.pool.QueryRow(context.Background(),
			`SELECT name FROM levels WHERE id = 1`).Scan(&name); err != nil {
			t.Fatalf("回读关卡名失败：%v", err)
		}
		return name
	}
	if got := levelName(); got != sentinel {
		t.Fatalf("哨兵写入失败：%q", got)
	}
	audits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'update_level'`,
			adminID)
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	// --- 审计失败：关卡参数必须一行不动 ---
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.do(t, fiber.MethodPut, "/api/v1/admin/levels/1", tok,
		map[string]any{"name": "R43 CHANGED"})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时改关卡必须失败（500），实际 %d %v", status, body)
	}
	if got := levelName(); got != sentinel {
		t.Errorf("审计失败后关卡名必须保持原值，实际 %q", got)
	}
	if got := audits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	// --- 故障解除：修改生效 + 审计 1 行 ---
	status, body = e.do(t, fiber.MethodPut, "/api/v1/admin/levels/1", tok,
		map[string]any{"name": "R43 CHANGED"})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后改关卡应成功，实际 %d %v", status, body)
	}
	if got := levelName(); got != "R43 CHANGED" {
		t.Errorf("修改成功后备查关卡名应为 R43 CHANGED，实际 %q", got)
	}
	if got := audits(); got != 1 {
		t.Errorf("成功后应有 1 行 update_level 审计，实际 %d", got)
	}
}
