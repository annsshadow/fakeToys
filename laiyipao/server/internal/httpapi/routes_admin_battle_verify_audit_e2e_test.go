// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"fmt"
	"testing"
	"time"
)

// R48：运营验真的记录与审计必须同生同死。
//
// `replay_verifications` 里的一行可能是「这局对不上」的指控证据；修前
// 审计由 handler 提交后补写、失败只打日志，于是可以出现「结论已入库、
// 但没人知道是谁在何时判的」。
func TestE2EAdminBattleVerifyAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, _, ids := playerWithBattles(t, e, "r48_verify", 1)
	battleID := ids[0]
	adminID, tok := e.newAdmin(t, "r48_verify")

	url := fmt.Sprintf("/api/v1/admin/battles/%d/verify", battleID)
	verifications := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID)
	}
	audits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'verify_replay'`,
			adminID)
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	baseline := verifications()
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.post(t, url, tok, map[string]any{
		"replay_hash": "0000000000000000",
	})
	restore()
	if status != 200 && status != 500 {
		t.Fatalf("验真返回了意外状态 %d：%v", status, body)
	}
	if status != 500 {
		t.Errorf("审计写不进去时验真必须失败（500），实际 %d %v", status, body)
	}
	if got := verifications(); got != baseline {
		t.Errorf("审计失败后验真记录不得新增：基线 %d 现在 %d", baseline, got)
	}
	if got := audits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	baseline = verifications()
	status, body = e.post(t, url, tok, map[string]any{
		"replay_hash": "0000000000000000",
	})
	if status != 200 {
		t.Fatalf("故障解除后验真应成功，实际 %d %v", status, body)
	}
	if got := verifications(); got != baseline+1 {
		t.Errorf("验真成功应新增 1 行记录：基线 %d 现在 %d", baseline, got)
	}
	if got := audits(); got != 1 {
		t.Errorf("成功后应有 1 行 verify_replay 审计，实际 %d", got)
	}
}
