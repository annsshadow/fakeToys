// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// R46：商城配置修改与审计必须同生同死。
func TestE2EAdminUpdateShopItemAndAuditAreAtomic(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "shop_audit")

	const sentinel = "R46 SENTINEL"
	e.exec(t, `UPDATE shop_items SET name = $1 WHERE id = 1`, sentinel)

	itemName := func() string {
		var name string
		if err := e.pool.QueryRow(context.Background(),
			`SELECT name FROM shop_items WHERE id = 1`).Scan(&name); err != nil {
			t.Fatalf("回读商品名失败：%v", err)
		}
		return name
	}
	audits := func() int {
		return e.scalarInt(t,
			`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'update_shop_item'`,
			adminID)
	}
	restore := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE admin_audit_logs_hidden RENAME TO admin_audit_logs`)
	}
	t.Cleanup(restore)

	e.exec(t, `ALTER TABLE admin_audit_logs RENAME TO admin_audit_logs_hidden`)
	status, body := e.do(t, fiber.MethodPut, "/api/v1/admin/shop/1", tok,
		map[string]any{"name": "R46 CHANGED"})
	restore()
	if status != fiber.StatusInternalServerError {
		t.Errorf("审计写不进去时改商品必须失败（500），实际 %d %v", status, body)
	}
	if got := itemName(); got != sentinel {
		t.Errorf("审计失败后商品名必须保持原值，实际 %q", got)
	}
	if got := audits(); got != 0 {
		t.Errorf("审计失败时不应有审计行，实际 %d", got)
	}

	status, body = e.do(t, fiber.MethodPut, "/api/v1/admin/shop/1", tok,
		map[string]any{"name": "R46 CHANGED"})
	if status != fiber.StatusOK {
		t.Fatalf("故障解除后改商品应成功，实际 %d %v", status, body)
	}
	if got := itemName(); got != "R46 CHANGED" {
		t.Errorf("修改成功后备查商品名应为 R46 CHANGED，实际 %q", got)
	}
	if got := audits(); got != 1 {
		t.Errorf("成功后应有 1 行 update_shop_item 审计，实际 %d", got)
	}
}
