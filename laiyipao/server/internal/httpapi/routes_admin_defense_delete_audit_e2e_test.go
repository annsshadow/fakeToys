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

// R47：删除防线与审计必须同生同死。
//
// 删除会连带 CASCADE 掉挑战历史，是不可逆操作；审计写不进去时
// 不能「删了但没人知道」。
func TestE2EAdminDeleteDefenseAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	ownerID, _ := e.newPlayer(t, "r47_owner")
	adminID, tok := e.newAdmin(t, "r47")

	var defID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, 'R47 违规防线', 100, '{}'::jsonb, 'snap')
		 RETURNING id`, ownerID).Scan(&defID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}

	url := fmt.Sprintf("/api/v1/admin/defenses/%d", defID)
	defenseExists := func() bool {
		var n int
		if err := e.pool.QueryRow(context.Background(),
			`SELECT COUNT(*) FROM defenses WHERE id = $1`, defID).Scan(&n); err != nil {
			t.Fatalf("统计防线失败：%v", err)
		}
		return n > 0
	}
	audits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'delete_defense'`,
			adminID)
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.do(t, fiber.MethodDelete, url, tok, nil)
	restore()
	if status != fiber.StatusInternalServerError {
		t.Errorf("审计写不进去时删防线必须失败（500），实际 %d %v", status, body)
	}
	if !defenseExists() {
		t.Error("审计失败后防线必须仍然存在")
	}
	if got := audits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	status, body = e.do(t, fiber.MethodDelete, url, tok, nil)
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后删防线应成功，实际 %d %v", status, body)
	}
	if defenseExists() {
		t.Error("删除成功后防线应不存在")
	}
	if got := audits(); got != 1 {
		t.Errorf("成功后应有 1 行 delete_defense 审计，实际 %d", got)
	}
}
