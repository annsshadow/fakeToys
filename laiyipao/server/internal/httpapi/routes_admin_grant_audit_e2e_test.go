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

// R37：后台发币/回收与审计必须同一事务。
//
// 余额变动而审计缺失，等于一笔没有来源的资产转移：事后既查不到
// 是谁发的、也分不清「发放」还是「回收」。修前 handler 在钱包事务
// 提交之后才补审计，失败只打日志 —— 钱照发。
func TestE2EAdminGrantAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, _ := e.newPlayer(t, "grant_audit")
	adminID, tok := e.newAdmin(t, "grant_audit_admin")

	coin := func() int {
		return e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid)
	}
	countAudits := func(action string) int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = $2`,
			adminID, action)
	}
	grantURL := fmt.Sprintf("/api/v1/admin/users/%d/grant", uid)
	body := map[string]any{"currency": "coin", "amount": 100}

	before := coin()

	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	status, resp := e.post(t, grantURL, tok, body)
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时必须整体失败（500），实际 %d %v", status, resp)
	}
	if got := coin(); got != before {
		t.Errorf("审计失败后余额必须回滚：基线 %d，实际 %d", before, got)
	}
	if got := countAudits("grant_currency"); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	status, resp = e.post(t, grantURL, tok, body)
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后发币应成功，实际 %d %v", status, resp)
	}
	if got := coin(); got != before+100 {
		t.Errorf("发币成功后余额应 +100：基线 %d，实际 %d", before, got)
	}
	if got := countAudits("grant_currency"); got != 1 {
		t.Errorf("成功发币应恰有 1 行 grant_currency 审计，实际 %d", got)
	}
}
