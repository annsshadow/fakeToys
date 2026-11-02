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

// R41：后台重置密码 = 改哈希 + 吊销全部 refresh token + 审计，
// 三步必须同生同死。
//
// 失败注入沿用 R34/R36/R37/R39/R40：把 admin_audit_logs 改名。
// 修前：哈希已换、所有设备已被踢下线，接口 200，审计 0 ——
// 「谁把所有人的会话切断了」没有任何记录，玩家也不知道密码被改过。
func TestE2EAdminResetPasswordAuditIsAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPasswordPlayer(t, "reset_audit")
	_, adminTok := e.newAdmin(t, "reset_audit")
	username := e.usernameOf(t, uid)

	const oldPassword = "test-password-12345"
	const newPassword = "admin-reset-password-99"
	tokenURL := "/api/v1/admin/users/" + strconv.FormatInt(uid, 10) + "/reset-password"

	// `newPasswordPlayer` 建号时已经登录过一次，这里直接用它留下的 token。
	// 只断言「≥1」：夹具内部实现变化（比如多签一张）不该让本用例变红。
	activeTokens := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL`, uid)
	}
	if got := activeTokens(); got < 1 {
		t.Fatalf("预置账号应至少有一张有效 token，实际 %d", got)
	}
	// 记住重置前已签发的最大 token id：后面用「这批 token 是否被吊销」
	// 断言吊销步骤，避免被「验密码时新登录产生的新 token」干扰。
	preResetMaxID := e.scalarInt(t,
		`SELECT COALESCE(MAX(id), 0) FROM refresh_tokens WHERE user_id = $1`, uid)
	preResetActive := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM refresh_tokens
			  WHERE user_id = $1 AND id <= $2 AND revoked_at IS NULL`, uid, preResetMaxID)
	}
	preResetActiveBase := preResetActive()
	audits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE action = 'reset_password' AND target = $1`,
			strconv.FormatInt(uid, 10))
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	// --- 审计失败：密码与令牌都不许动 ---
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.post(t, tokenURL, adminTok, map[string]any{"new_password": newPassword})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时重置必须失败（500），实际 %d %v", status, body)
	}
	if got := preResetActive(); got != preResetActiveBase {
		t.Errorf("审计失败后重置前的 token 应仍有效：基线 %d，实际 %d（会话被误杀）",
			preResetActiveBase, got)
	}
	// 旧密码必须仍能登录 —— 这是「玩家没有被锁在门外」的直接证据。
	if _, err := e.svc.Login(context.Background(), username, oldPassword); err != nil {
		t.Errorf("审计失败后旧密码必须仍能登录，实际：%v", err)
	}
	// 新密码绝不能提前生效（否则等于密码被静默改掉）。
	if _, err := e.svc.Login(context.Background(), username, newPassword); err == nil {
		t.Error("审计失败后新密码不应生效")
	}
	if got := audits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	// --- 故障解除：三步一起生效 ---
	status, body = e.post(t, tokenURL, adminTok, map[string]any{"new_password": newPassword})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后重置应成功，实际 %d %v", status, body)
	}
	if _, err := e.svc.Login(context.Background(), username, oldPassword); err == nil {
		t.Error("重置成功后旧密码不应再能登录")
	}
	if _, err := e.svc.Login(context.Background(), username, newPassword); err != nil {
		t.Fatalf("重置成功后新密码应能登录：%v", err)
	}
	// 注意：上面为了验密码又登录了两次，会产生新 token；
	// 断言「重置之前签发的那批 token 已全部被吊销」。
	if got := preResetActive(); got != 0 {
		t.Errorf("重置成功后应吊销此前的全部 token，实际仍有 %d 张", got)
	}
	if got := audits(); got != 1 {
		t.Errorf("成功后应有 1 行 reset_password 审计，实际 %d", got)
	}
}
