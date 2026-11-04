// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 本文件补后台**读路径**的两类契约：
//
//  1. 分页参数夹取：负 offset 必须夹成 0。改前 `AdminListDefenses` 把 -1
//     原样拼进 SQL，PostgreSQL 报 `OFFSET must not be negative` → 整页 500
//     （R19 的注释记录了这个顺序错误）。这里把所有后台列表都过一遍，
//     免得同类缺陷只在「已经修过的那一个」上有守卫。
//  2. fail-loud：后台查询失败必须报错，不能返回「空列表 + total=0」。
//     运营看到的 0 条会被当成「没有数据」，而真相是表/列坏了 ——
//     这正是 `AdminDashboardSectionFailures` 同族的契约。

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// TestAdminListClampsNegativeOffset 负 offset 不得透传给 SQL。
func TestAdminListClampsNegativeOffset(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	cases := []struct {
		name string
		call func() error
	}{
		{"defenses", func() error {
			_, _, err := ts.AdminListDefenses(ctx, 0, -1, "", 0, nil)
			return err
		}},
		{"defense_challenges", func() error {
			_, _, err := ts.AdminListDefenseChallenges(ctx, 0, -1, 0, 0, nil, nil)
			return err
		}},
		{"orders", func() error {
			_, _, err := ts.AdminListOrders(ctx, 0, -1, "", nil, nil, "")
			return err
		}},
		{"redeem_usages", func() error {
			_, _, err := ts.AdminListRedeemUsages(ctx, 0, -1, "", 0)
			return err
		}},
		{"redeem_codes", func() error {
			_, _, err := ts.AdminListRedeemCodes(ctx, 0, -1, "", nil, nil)
			return err
		}},
		{"announcements", func() error {
			_, _, err := ts.AdminListAnnouncements(ctx, 0, -1, nil, nil, nil)
			return err
		}},
		{"audit_logs", func() error {
			_, _, err := ts.AdminListAuditLogs(ctx, 0, -1, "", 0, nil, nil, "")
			return err
		}},
		{"users", func() error {
			_, _, err := ts.AdminListUsers(ctx, "", 0, -1, nil, nil, nil, nil, nil, nil)
			return err
		}},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if err := c.call(); err != nil {
				t.Fatalf("offset=-1 必须夹取为 0，实际：%v", err)
			}
		})
	}
}

// TestAdminListRejectsOverlongKeyword 超长搜索词必须 400，而不是拿整段去 LIKE。
func TestAdminListRejectsOverlongKeyword(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	long := strings.Repeat("推", MaxAdminKeywordLen+1)
	cases := []struct {
		name string
		call func() error
	}{
		{"levels", func() error {
			_, _, err := ts.AdminListLevels(ctx, 0, long, 0, 0)
			return err
		}},
		{"users", func() error {
			_, _, err := ts.AdminListUsers(ctx, long, 0, 0, nil, nil, nil, nil, nil, nil)
			return err
		}},
		{"defenses", func() error {
			_, _, err := ts.AdminListDefenses(ctx, 0, 0, long, 0, nil)
			return err
		}},
		{"redeem_codes", func() error {
			_, _, err := ts.AdminListRedeemCodes(ctx, 0, 0, long, nil, nil)
			return err
		}},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			err := c.call()
			if !errors.Is(err, ErrBadInput) {
				t.Fatalf("%d 字搜索词应返回 ErrBadInput，实际：%v", MaxAdminKeywordLen+1, err)
			}
			if !strings.Contains(err.Error(), "keyword") {
				t.Fatalf("错误应点明 keyword，实际：%v", err)
			}
		})
	}
}

// TestAdminReadsFailLoudOnMissingTables 后台读路径的表/列坏掉时必须响。
//
// 每个子用例把目标表临时改名、调用、再改回；断言放在改回之后，
// 避免一条失败让后续子用例继承坏库状态。
func TestAdminReadsFailLoudOnMissingTables(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	cases := []struct {
		name, table, want string
		call              func() error
	}{
		{"levels list", "levels", "admin levels", func() error {
			_, _, err := ts.AdminListLevels(ctx, 0, "", 0, 0)
			return err
		}},
		{"level row", "levels", "level row", func() error {
			_, err := ts.AdminLevelRow(ctx, 1)
			return err
		}},
		{"level waves", "level_waves", "admin waves", func() error {
			_, err := ts.AdminLevelWaves(ctx, 1)
			return err
		}},
		{"skills", "skills", "fetch skills", func() error {
			_, err := ts.AdminSkills(ctx)
			return err
		}},
		{"equipment", "equipment", "admin equipment", func() error {
			_, err := ts.AdminEquipment(ctx)
			return err
		}},
		{"defenses", "defenses", "admin defenses", func() error {
			_, _, err := ts.AdminListDefenses(ctx, 0, 0, "", 0, nil)
			return err
		}},
		{"defense challenges", "defense_challenges", "admin defense challenges", func() error {
			_, _, err := ts.AdminListDefenseChallenges(ctx, 0, 0, 0, 0, nil, nil)
			return err
		}},
		{"orders", "orders", "admin orders", func() error {
			_, _, err := ts.AdminListOrders(ctx, 0, 0, "", nil, nil, "")
			return err
		}},
		{"redeem usages", "redeem_usages", "admin redeem usages", func() error {
			_, _, err := ts.AdminListRedeemUsages(ctx, 0, 0, "", 0)
			return err
		}},
		{"announcements", "announcements", "admin announcements", func() error {
			_, _, err := ts.AdminListAnnouncements(ctx, 0, 0, nil, nil, nil)
			return err
		}},
		{"redeem codes", "redeem_codes", "admin redeem_codes", func() error {
			_, _, err := ts.AdminListRedeemCodes(ctx, 0, 0, "", nil, nil)
			return err
		}},
		{"audit logs", "admin_audit_logs", "admin_audit_logs", func() error {
			_, _, err := ts.AdminListAuditLogs(ctx, 0, 0, "", 0, nil, nil, "")
			return err
		}},
		{"users list wallets", "user_wallets", "list users", func() error {
			_, _, err := ts.AdminListUsers(ctx, "", 0, 0, nil, nil, nil, nil, nil, nil)
			return err
		}},
		{"users export wallets", "user_wallets", "export users", func() error {
			_, err := ts.AdminExportUsers(ctx, "", nil, nil, nil, nil, nil, nil)
			return err
		}},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			backup := c.table + "_r96_gone"
			ts.renameTable(t, c.table, backup)
			err := c.call()
			ts.renameTable(t, backup, c.table)
			if err == nil {
				t.Fatalf("%s 不存在时后台读路径返回成功（0 条会被当成没有数据）", c.table)
			}
			if !strings.Contains(err.Error(), c.want) {
				t.Fatalf("错误应点明 %q，实际：%v", c.want, err)
			}
		})
	}
}

// TestAdminEconomyFailLoud 经济看板的两段查询与两个 jsonb 列同样不许静默。
func TestAdminEconomyFailLoud(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	t.Run("wallet_flows缺失", func(t *testing.T) {
		ts.renameTable(t, "wallet_flows", "wallet_flows_r96_gone")
		_, err := ts.adminEconomyUncached(ctx)
		ts.renameTable(t, "wallet_flows_r96_gone", "wallet_flows")
		if err == nil || !strings.Contains(err.Error(), "admin economy flows") {
			t.Fatalf("wallet_flows 缺失应报 admin economy flows，实际：%v", err)
		}
	})

	t.Run("shop_items缺失", func(t *testing.T) {
		ts.renameTable(t, "shop_items", "shop_items_r96_gone")
		_, err := ts.adminEconomyUncached(ctx)
		ts.renameTable(t, "shop_items_r96_gone", "shop_items")
		if err == nil || !strings.Contains(err.Error(), "admin shop") {
			t.Fatalf("shop_items 缺失应报 admin shop，实际：%v", err)
		}
	})

	// price / payload 被写坏时的报错与 LoadShop 同族：形状不对不能当空对象。
	for _, col := range []string{"price", "payload"} {
		t.Run(col+"坏jsonb", func(t *testing.T) {
			var raw []byte
			if err := ts.pool.QueryRow(ctx,
				`SELECT `+col+` FROM shop_items ORDER BY id LIMIT 1`).Scan(&raw); err != nil {
				t.Fatalf("读原始 %s 失败：%v", col, err)
			}
			ts.exec(t, `UPDATE shop_items SET `+col+` = '[]'::jsonb WHERE id = (SELECT MIN(id) FROM shop_items)`)
			_, err := ts.adminEconomyUncached(ctx)
			ts.exec(t, `UPDATE shop_items SET `+col+` = $1::jsonb WHERE id = (SELECT MIN(id) FROM shop_items)`, raw)
			if err == nil || !strings.Contains(err.Error(), "invalid jsonb") {
				t.Fatalf("%s 形状非法应报 invalid jsonb，实际：%v", col, err)
			}
			if !strings.Contains(err.Error(), "shop") {
				t.Fatalf("%s 的错误应点明 shop，实际：%v", col, err)
			}
		})
	}
}
