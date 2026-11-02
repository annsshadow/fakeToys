// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// exportTooLarge 必须可被 errors.Is 归类，且文案要给出「至少多少行 / 上限多少」
// —— 运营据此才知道该把筛选缩到多小。
func TestExportTooLargeIsClassifiableAndActionable(t *testing.T) {
	err := exportTooLarge(adminExportProbeRows)
	if !errors.Is(err, ErrExportTooLarge) {
		t.Fatalf("errors.Is(ErrExportTooLarge) = false；错误不可分类：%v", err)
	}
	msg := err.Error()
	if !strings.Contains(msg, "至少命中 100001 行") {
		t.Fatalf("文案缺少实际行数：%q", msg)
	}
	if !strings.Contains(msg, "100000") {
		t.Fatalf("文案缺少上限：%q", msg)
	}
}

// 边界必须是「恰好 10 万行仍成功，10 万零 1 行明确失败」。
//
// 这条用例直接打真库：如果只测 helper，SQL 里的 LIMIT 与常量漂移时
// 测试仍会绿，而线上又会回到静默截断。
func TestAdminExportAnnouncementsBoundary(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 清空后灌入恰好上限行数。
	ts.exec(t, `DELETE FROM announcements`)
	ts.exec(t, `INSERT INTO announcements (title, body)
	            SELECT 't' || g, 'b' FROM generate_series(1, $1) g`, adminExportMaxRows)

	data, err := ts.AdminExportAnnouncements(ctx)
	if err != nil {
		t.Fatalf("恰好 %d 行应当导出成功，实际报错：%v", adminExportMaxRows, err)
	}
	// CSV = 1 行表头 + N 行数据；body 不含换行，直接数 '\n'。
	if lines := strings.Count(string(data), "\n"); lines != adminExportMaxRows+1 {
		t.Fatalf("CSV 行数 = %d，期望 %d（表头 + %d 行数据）",
			lines, adminExportMaxRows+1, adminExportMaxRows)
	}

	// 多一行就超过上限：必须报 ErrExportTooLarge，而不是返回 10 万行的 CSV。
	ts.exec(t, `INSERT INTO announcements (title, body) VALUES ('overflow', 'b')`)
	if _, err := ts.AdminExportAnnouncements(ctx); !errors.Is(err, ErrExportTooLarge) {
		t.Fatalf("超出上限应返回 ErrExportTooLarge，实际：%v", err)
	}
}

// 带筛选的导出（单用户流水）同样要检测溢出：它的 LIMIT 与全量导出
// 是两条独立 SQL，只覆盖一支会漏。
func TestAdminExportUserFlowsOverLimit(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	var uid int64
	err := ts.pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname) VALUES ('bulk-export-probe', 'probe')
		 RETURNING id`).Scan(&uid)
	if err != nil {
		t.Fatalf("准备测试用户失败：%v", err)
	}
	ts.exec(t, `INSERT INTO wallet_flows (user_id, currency, delta, balance, reason)
	            SELECT $1, 'coin', 1, 1, 'bulk_export_probe'
	            FROM generate_series(1, $2)`, uid, adminExportProbeRows)

	if _, err := ts.AdminExportUserFlows(ctx, uid); !errors.Is(err, ErrExportTooLarge) {
		t.Fatalf("超出上限应返回 ErrExportTooLarge，实际：%v", err)
	}
}
