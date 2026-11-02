// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// R45：技能文案修改与审计必须同生同死。
//
// 失败注入沿用既有手法：把 admin_audit_logs 改名。
func TestE2EAdminUpdateSkillAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "skill_audit")

	const sentinel = "R45 SENTINEL"
	e.exec(t, `UPDATE skills SET name = $1 WHERE id = 1`, sentinel)

	skillName := func() string {
		var name string
		if err := e.pool.QueryRow(context.Background(),
			`SELECT name FROM skills WHERE id = 1`).Scan(&name); err != nil {
			t.Fatalf("回读技能名失败：%v", err)
		}
		return name
	}
	audits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'update_skill'`,
			adminID)
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.do(t, fiber.MethodPut, "/api/v1/admin/skills/1", tok,
		map[string]any{"name": "R45 CHANGED"})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时改技能必须失败（500），实际 %d %v", status, body)
	}
	if got := skillName(); got != sentinel {
		t.Errorf("审计失败后技能名必须保持原值，实际 %q", got)
	}
	if got := audits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	status, body = e.do(t, fiber.MethodPut, "/api/v1/admin/skills/1", tok,
		map[string]any{"name": "R45 CHANGED"})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后改技能应成功，实际 %d %v", status, body)
	}
	if got := skillName(); got != "R45 CHANGED" {
		t.Errorf("修改成功后备查技能名应为 R45 CHANGED，实际 %q", got)
	}
	if got := audits(); got != 1 {
		t.Errorf("成功后应有 1 行 update_skill 审计，实际 %d", got)
	}
}
