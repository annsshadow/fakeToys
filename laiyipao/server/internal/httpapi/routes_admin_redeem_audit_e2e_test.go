// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// R34：兑换码生成与「谁生成了它」的审计必须同生同死。
//
// 失败注入用一次性 scratch 库 + 表重命名（与 routes_failpath_e2e_test.go 同法）：
// 把 admin_audit_logs 改名后，业务 INSERT 仍能成功、审计 INSERT 必然报
// 「表不存在」。
//
// 修前的可观察行为是：接口 200、库里多出 N 个可兑换码、审计 0 行 ——
// 运营事后无法回答「这批码是谁在什么时候生成的」。
// 修后：接口 500、码数回到基线、审计 0；故障解除后同一请求成功且两样都有。
func TestE2EBatchRedeemCodesAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "redeem_audit")

	countCodes := func() int { return e.scalarInt(t, `SELECT COUNT(*) FROM redeem_codes`) }
	countAudits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'create_redeem_code'`,
			adminID)
	}

	before := countCodes()

	// 注入：审计表不存在。业务表（redeem_codes）不受影响。
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	status, body := e.post(t, "/api/v1/admin/redeem-codes/batch", tok, map[string]any{
		"count": 3, "reward": map[string]int{"coin": 10}, "max_uses": 1,
	})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时整批必须失败（500），实际 %d %v —— "+
			"改前这里会 200 并留下 %d 个无审计的码", status, body, countCodes()-before)
	}
	if got := countCodes(); got != before {
		t.Errorf("审计失败后兑换码必须整体回滚：基线 %d，实际 %d", before, got)
	}
	if got := countAudits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	// 故障解除后同一请求成功：码 +3、审计 +1。
	status, body = e.post(t, "/api/v1/admin/redeem-codes/batch", tok, map[string]any{
		"count": 3, "reward": map[string]int{"coin": 10}, "max_uses": 1,
	})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后批量生成应成功，实际 %d %v", status, body)
	}
	if got := countCodes(); got != before+3 {
		t.Errorf("成功批次应新增 3 个码，基线 %d，实际 %d", before, got)
	}
	if got := countAudits(); got != 1 {
		t.Errorf("成功批次应恰有 1 行审计，实际 %d", got)
	}
}
