// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"encoding/csv"
	"errors"
	"strings"
	"testing"
)

// exportBuffer 必须在写入**前**拒绝超预算的块：事后检查等于内存已经分配。
//
// csv.Writer 内部是 bufio.Writer（4 KiB），所以小记录会先攒着 —— 错误在
// Flush 时暴露，这也是各导出末尾必须 `Flush()` + `Error()` 的原因。
func TestExportBufferRejectsOverBudget(t *testing.T) {
	buf := newExportBuffer(12)
	buf.WriteString("\xEF\xBB\xBF") // BOM 占 3 字节，必须计入预算
	w := csv.NewWriter(buf)

	if err := writeCSVRow(w, []string{"ok"}); err != nil {
		t.Fatalf("预算内的行不应失败：%v", err)
	}
	w.Flush()
	if err := w.Error(); err != nil {
		t.Fatalf("预算内的行不应失败：%v", err)
	}
	before := buf.Len()

	if err := writeCSVRow(w, []string{"0123456789abcdef"}); err != nil {
		t.Fatalf("bufio 尚未刷新，写入阶段不该报错：%v", err)
	}
	w.Flush()
	err := w.Error()
	if !errors.Is(err, ErrExportTooLarge) {
		t.Fatalf("超预算应返回 ErrExportTooLarge，实际：%v", err)
	}
	if !strings.Contains(err.Error(), "字节") {
		t.Fatalf("文案应说明是字节预算：%v", err)
	}
	// 被拒的块不得写进缓冲 —— 这是「写入前拦」与「事后检查」的区别。
	if buf.Len() != before {
		t.Fatalf("超预算的记录写入了缓冲：%d → %d 字节", before, buf.Len())
	}
	// 一旦超预算，后续写入继续失败，不会悄悄恢复增长。
	if err := writeCSVRow(w, []string{"x"}); !errors.Is(err, ErrExportTooLarge) {
		t.Fatalf("超预算后写入仍应失败，实际：%v", err)
	}
}

// 0 = 不限（与其它配置的 0 语义一致）：排查时可关掉预算。
func TestExportBufferZeroLimitIsUnlimited(t *testing.T) {
	buf := newExportBuffer(0)
	w := csv.NewWriter(buf)
	for i := 0; i < 100; i++ {
		if err := writeCSVRow(w, []string{strings.Repeat("y", 100)}); err != nil {
			t.Fatalf("limit=0 不应报错：%v", err)
		}
	}
	w.Flush()
	if err := w.Error(); err != nil {
		t.Fatalf("limit=0 不应报错：%v", err)
	}
	if buf.Len() < 10*1000 {
		t.Fatalf("缓冲长度异常：%d", buf.Len())
	}
}

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

	data, err := ts.AdminExportAnnouncements(ctx, nil, nil, nil)
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
	if _, err := ts.AdminExportAnnouncements(ctx, nil, nil, nil); !errors.Is(err, ErrExportTooLarge) {
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

// 字节预算必须**独立于行数上限**生效：20 行远没到 10 万行，但行宽大，
// 总量超过 4 KiB 就必须 413，而不是把 8 KB 物化进内存后照样返回。
//
// 这条用例的意义是：只测 helper 时，「导出函数用了 budget buffer」
// 这件事没有被验证 —— 漏接一个导出仍是 OOM 面。
func TestAdminExportAnnouncementsByteBudget(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	cfg := *ts.Cfg()
	cfg.ExportMaxBytes = 4 << 10
	ts.ReloadConfig(cfg)

	ts.exec(t, `DELETE FROM announcements`)
	ts.exec(t, `INSERT INTO announcements (title, body)
	            SELECT 'wide-' || g, repeat('x', 400) FROM generate_series(1, 20) g`)

	_, err := ts.AdminExportAnnouncements(ctx, nil, nil, nil)
	if !errors.Is(err, ErrExportTooLarge) {
		t.Fatalf("字节超预算应返回 ErrExportTooLarge，实际：%v", err)
	}
	if !strings.Contains(err.Error(), "字节") {
		t.Fatalf("文案应指出字节预算而不是行数：%v", err)
	}

	// 放开预算后同一批数据必须成功 —— 证明拦截来自预算，而不是数据本身有问题。
	cfg.ExportMaxBytes = 64 << 20
	ts.ReloadConfig(cfg)
	if _, err := ts.AdminExportAnnouncements(ctx, nil, nil, nil); err != nil {
		t.Fatalf("预算放开后应导出成功，实际：%v", err)
	}
}
