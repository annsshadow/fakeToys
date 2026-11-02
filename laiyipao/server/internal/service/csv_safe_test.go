// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"strings"
	"testing"
)

// 公式注入的中和规则：危险前导字符加 `'`，但**不能**把负数字符串变成文本，
// 否则充值/退款金额（-1.50）在表格里会退化成不可求和的文本。
func TestCsvSafeNeutralizesFormulasButKeepsNumbers(t *testing.T) {
	cases := []struct{ in, want string }{
		{"", ""},
		{"=HYPERLINK(\"http://evil\",\"click\")", "'=HYPERLINK(\"http://evil\",\"click\")"},
		{"+1+1", "'+1+1"},
		{"@SUM(A1)", "'@SUM(A1)"},
		{"\tleading-tab", "'\tleading-tab"},
		{"\rleading-cr", "'\rleading-cr"},
		// 负数保持数值语义
		{"-5", "-5"},
		{"-1.50", "-1.50"},
		// 不是合法数字的 `-` 开头仍要中和（Excel 会按表达式解析）
		{"-1+1", "'-1+1"},
		{"-HYPERLINK()", "'-HYPERLINK()"},
		// 普通文本不受影响
		{"正常昵称", "正常昵称"},
		{"nickname", "nickname"},
		{"2026-11-02T00:00:00Z", "2026-11-02T00:00:00Z"},
	}
	for _, c := range cases {
		if got := csvSafe(c.in); got != c.want {
			t.Errorf("csvSafe(%q) = %q，期望 %q", c.in, got, c.want)
		}
	}
}

// 端到端：审计日志的 target 是可控文本，导出后必须以 `'` 前缀落地，
// 否则运营在 Excel 里打开归档就会被公式执行。
func TestAdminExportAuditLogsNeutralizesFormulaCell(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	ts.exec(t, `DELETE FROM admin_audit_logs`)
	ts.exec(t, `INSERT INTO admin_audit_logs (username, action, target, detail)
	            VALUES ('op', 'probe_csv_injection', $1, '{}')`,
		`=HYPERLINK("http://example.invalid","click")`)

	data, err := ts.AdminExportAuditLogs(ctx, "", 0, nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	if !strings.Contains(string(data), `'=HYPERLINK(`) {
		t.Fatalf("危险单元格未被中和：\n%s", data)
	}
	if strings.Contains(string(data), "\n=HYPERLINK(") {
		t.Fatalf("仍存在可被求值的行首公式：\n%s", data)
	}
}

// 负数金额列不能被误伤：导出里必须仍是 `-12`，而不是 `'-12`。
func TestAdminExportKeepsNegativeNumericCells(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	var uid int64
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname) VALUES ('csv-neg-probe', 'neg')
		 RETURNING id`).Scan(&uid); err != nil {
		t.Fatalf("准备用户失败：%v", err)
	}
	ts.exec(t, `INSERT INTO wallet_flows (user_id, currency, delta, balance, reason)
	            VALUES ($1, 'coin', -12, 0, 'csv_neg_probe')`, uid)

	data, err := ts.AdminExportUserFlows(ctx, uid)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	if strings.Contains(string(data), "'-12") {
		t.Fatalf("负数金额被当成公式中和，数值语义被破坏：\n%s", data)
	}
	if !strings.Contains(string(data), "-12") {
		t.Fatalf("负数金额丢失：\n%s", data)
	}
}
