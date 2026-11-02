// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"strconv"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// R40：兑换码启停/编辑与审计必须同生同死。
//
// 停用与改奖励都是会改变已发码价值的动作；审计写不进去时若业务照做，
// 「谁把一张泄露的码重新启用 / 谁把奖励改成 999」就没有任何记录。
// 失败注入沿用 R34/R36/R37/R39：把 admin_audit_logs 改名。
func TestE2ERedeemCodeEditsAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "redeem_audit")

	var codeID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ('R40-EDIT', '{"coin":10}'::jsonb, 5) RETURNING id`).
		Scan(&codeID); err != nil {
		t.Fatalf("预置兑换码失败：%v", err)
	}

	enabledOf := func() bool {
		var v bool
		if err := e.pool.QueryRow(context.Background(),
			`SELECT enabled FROM redeem_codes WHERE id = $1`, codeID).Scan(&v); err != nil {
			t.Fatalf("回读 enabled 失败：%v", err)
		}
		return v
	}
	coinOf := func() int {
		var v int
		if err := e.pool.QueryRow(context.Background(),
			`SELECT (reward->>'coin')::int FROM redeem_codes WHERE id = $1`, codeID).Scan(&v); err != nil {
			t.Fatalf("回读 reward 失败：%v", err)
		}
		return v
	}
	countAudits := func(action string) int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = $2`,
			adminID, action)
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	// --- 停用：审计失败则 enabled 不变 ---
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.do(t, fiber.MethodPut,
		"/api/v1/admin/redeem-codes/"+strconv.FormatInt(codeID, 10)+"/enabled", tok,
		map[string]any{"enabled": false})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时停用必须失败（500），实际 %d %v", status, body)
	}
	if !enabledOf() {
		t.Errorf("审计失败后 enabled 必须保持 true")
	}
	if got := countAudits("set_redeem_enabled"); got != 0 {
		t.Errorf("审计失败时不应有启停审计行，实际 %d", got)
	}

	status, body = e.do(t, fiber.MethodPut,
		"/api/v1/admin/redeem-codes/"+strconv.FormatInt(codeID, 10)+"/enabled", tok,
		map[string]any{"enabled": false})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后停用应成功，实际 %d %v", status, body)
	}
	if enabledOf() {
		t.Errorf("停用成功后 enabled 应为 false")
	}
	if got := countAudits("set_redeem_enabled"); got != 1 {
		t.Errorf("成功后应有 1 行 set_redeem_enabled 审计，实际 %d", got)
	}

	// --- 编辑奖励：审计失败则 reward 不变 ---
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body = e.do(t, fiber.MethodPut,
		"/api/v1/admin/redeem-codes/"+strconv.FormatInt(codeID, 10), tok,
		map[string]any{"reward": map[string]int{"coin": 999}, "max_uses": 5})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时编辑必须失败（500），实际 %d %v", status, body)
	}
	if got := coinOf(); got != 10 {
		t.Errorf("审计失败后 reward 必须保持 10，实际 %d", got)
	}
	if got := countAudits("update_redeem_code"); got != 0 {
		t.Errorf("审计失败时不应有编辑审计行，实际 %d", got)
	}

	status, body = e.do(t, fiber.MethodPut,
		"/api/v1/admin/redeem-codes/"+strconv.FormatInt(codeID, 10), tok,
		map[string]any{"reward": map[string]int{"coin": 999}, "max_uses": 5})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后编辑应成功，实际 %d %v", status, body)
	}
	if got := coinOf(); got != 999 {
		t.Errorf("编辑成功后 reward 应为 999，实际 %d", got)
	}
	if got := countAudits("update_redeem_code"); got != 1 {
		t.Errorf("成功后应有 1 行 update_redeem_code 审计，实际 %d", got)
	}
}
