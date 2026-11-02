// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// R42：「重建全部 100 关」与审计必须同生同死。
//
// 这是全库最重的写操作：覆盖运营手工调过的关卡内容，且没有快照可回滚。
// 修前审计在 handler 提交后补写、失败只打日志 —— 生成器一次误调
// 就能把 100 关全部重置，而操作记录是 0 行。
//
// 失败注入沿用 R34/R36/R37/R39/R40/R41：把 admin_audit_logs 改名。
func TestE2ERegenerateLevelsAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "regen_audit")

	// 哨兵：把第 1 关的名字改成生成器绝不会产出的值。
	// 重建成功会覆盖它；注入失败则它必须原样留在库里。
	e.exec(t, `UPDATE levels SET name = 'R42 SENTINEL' WHERE id = 1`)
	levelName := func() string {
		var name string
		if err := e.pool.QueryRow(context.Background(),
			`SELECT name FROM levels WHERE id = 1`).Scan(&name); err != nil {
			t.Fatalf("回读关卡名失败：%v", err)
		}
		return name
	}
	if got := levelName(); got != "R42 SENTINEL" {
		t.Fatalf("哨兵写入失败：%q", got)
	}
	audits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'regenerate_levels'`,
			adminID)
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	// --- 审计失败：100 关必须一行不动 ---
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.post(t, "/api/v1/admin/levels/regenerate", tok,
		map[string]any{"confirm": "regenerate"})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时重建必须失败（500），实际 %d %v", status, body)
	}
	if got := levelName(); got != "R42 SENTINEL" {
		t.Errorf("审计失败后关卡必须整体回滚，第 1 关名为 %q（被覆盖）", got)
	}
	if got := audits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	// --- 故障解除：重建生效 + 审计 1 行 ---
	status, body = e.post(t, "/api/v1/admin/levels/regenerate", tok,
		map[string]any{"confirm": "regenerate"})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后重建应成功，实际 %d %v", status, body)
	}
	if got := levelName(); got == "R42 SENTINEL" {
		t.Error("重建成功后第 1 关应被生成器内容覆盖")
	}
	if got := body["generated"]; got != float64(100) {
		t.Errorf("重建应生成 100 关，实际 %v", got)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM levels`); got != 100 {
		t.Errorf("levels 表应保持 100 行，实际 %d", got)
	}
	if got := audits(); got != 1 {
		t.Errorf("成功后应有 1 行 regenerate_levels 审计，实际 %d", got)
	}
}
