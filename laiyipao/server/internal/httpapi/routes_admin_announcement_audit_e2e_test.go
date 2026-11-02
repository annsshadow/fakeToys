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

// R39：公告的发布/删除与「谁做的」审计必须同生同死。
//
// 失败注入沿用 R34/R36/R37：把 admin_audit_logs 改名后业务写入仍能成功、
// 审计写入必然失败。修前公告落库、接口 200、审计 0 行 ——
// 运营发了违规内容或撤回公告，事后无从追责。
func TestE2EAnnouncementWritesAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "ann_audit")

	countAnnouncements := func() int {
		return e.scalarInt(t, `SELECT COUNT(*) FROM announcements`)
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

	// --- 创建：审计失败则公告不落库 ---
	before := countAnnouncements()
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)

	status, body := e.post(t, "/api/v1/admin/announcements", tok, map[string]any{
		"title": "R39 公告", "body": "正文", "published": true,
	})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时创建必须失败（500），实际 %d %v", status, body)
	}
	if got := countAnnouncements(); got != before {
		t.Errorf("审计失败后公告必须整体回滚：基线 %d，实际 %d", before, got)
	}
	if got := countAudits("create_announcement"); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	// 故障解除后创建成功：公告 +1、审计 +1，并拿到 id 供删除用例使用。
	status, body = e.post(t, "/api/v1/admin/announcements", tok, map[string]any{
		"title": "R39 公告", "body": "正文", "published": true,
	})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后创建应成功，实际 %d %v", status, body)
	}
	item, _ := body["item"].(map[string]any)
	annID := int64(item["id"].(float64))
	if got := countAnnouncements(); got != before+1 {
		t.Errorf("成功后公告应 +1：期望 %d，实际 %d", before+1, got)
	}
	if got := countAudits("create_announcement"); got != 1 {
		t.Errorf("成功后应有 1 行 create_announcement 审计，实际 %d", got)
	}

	// --- 更新：审计失败则内容与发布状态都不变 ---
	titleOf := func() string {
		var title string
		if err := e.pool.QueryRow(context.Background(),
			`SELECT title FROM announcements WHERE id = $1`, annID).Scan(&title); err != nil {
			t.Fatalf("回读公告标题失败：%v", err)
		}
		return title
	}
	originalTitle := titleOf()
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body = e.do(t, fiber.MethodPut,
		"/api/v1/admin/announcements/"+strconv.FormatInt(annID, 10), tok,
		map[string]any{"title": "R39 改后", "body": "新正文", "published": false})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时更新必须失败（500），实际 %d %v", status, body)
	}
	if got := titleOf(); got != originalTitle {
		t.Errorf("审计失败后公告内容必须不变：期望 %q，实际 %q", originalTitle, got)
	}
	if got := countAudits("update_announcement"); got != 0 {
		t.Errorf("审计失败时不应有更新审计行，实际 %d", got)
	}

	status, body = e.do(t, fiber.MethodPut,
		"/api/v1/admin/announcements/"+strconv.FormatInt(annID, 10), tok,
		map[string]any{"title": "R39 改后", "body": "新正文", "published": false})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后更新应成功，实际 %d %v", status, body)
	}
	if got := titleOf(); got != "R39 改后" {
		t.Errorf("成功后公告标题应更新，实际 %q", got)
	}
	if got := countAudits("update_announcement"); got != 1 {
		t.Errorf("成功后应有 1 行 update_announcement 审计，实际 %d", got)
	}

	// --- 删除：审计失败则公告必须还在 ---
	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body = e.do(t, fiber.MethodDelete,
		"/api/v1/admin/announcements/"+strconv.FormatInt(annID, 10), tok, nil)
	restore()
	if status != fiber.StatusInternalServerError {
		t.Fatalf("审计写不进去时删除必须失败（500），实际 %d %v", status, body)
	}
	if got := countAnnouncements(); got != before+1 {
		t.Errorf("审计失败后公告必须仍在：期望 %d，实际 %d", before+1, got)
	}
	if got := countAudits("delete_announcement"); got != 0 {
		t.Errorf("审计失败时不应有删除审计行，实际 %d", got)
	}

	// 故障解除后删除成功：公告 -1、审计 +1。
	status, body = e.do(t, fiber.MethodDelete,
		"/api/v1/admin/announcements/"+strconv.FormatInt(annID, 10), tok, nil)
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后删除应成功，实际 %d %v", status, body)
	}
	if got := countAnnouncements(); got != before {
		t.Errorf("成功后公告应回到基线 %d，实际 %d", before, got)
	}
	if got := countAudits("delete_announcement"); got != 1 {
		t.Errorf("成功后应有 1 行 delete_announcement 审计，实际 %d", got)
	}
}
