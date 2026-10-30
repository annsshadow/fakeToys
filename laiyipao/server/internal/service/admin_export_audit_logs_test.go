package service

// 后台审计日志 CSV 导出的服务层测试（2026-10-30 实质优化第 15 轮）。
//
// 关键行为锚点：
//   - 输出是 UTF-8 BOM + CSV（Excel 中文兼容）；
//   - 表头 7 列齐全（id/admin_id/username/action/target/detail/created_at）；
//   - detail 以原始 JSON 文本导出（归档不解析，坏数据不中断）；
//   - NULL 的 admin_id 导出为空串；
//   - 库故障必须报错而不是静默返回空 CSV。

import (
	"bytes"
	"context"
	"encoding/csv"
	"testing"
)

func TestAdminExportAuditLogsCSV(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 直接插入两条审计（admin_id 为 NULL，username 手写）
	for _, a := range []struct {
		username string
		action   string
		target   string
		detail   string
	}{
		{"ops_alice", "ban_user", "42", `{"reason":"测试"}`},
		{"ops_bob", "grant_currency", "7", `{"currency":"coin","amount":100}`},
	} {
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO admin_audit_logs (username, action, target, detail)
			 VALUES ($1, $2, $3, $4::jsonb)`, a.username, a.action, a.target, a.detail); err != nil {
			t.Fatalf("插入审计日志失败：%v", err)
		}
	}

	data, err := ts.AdminExportAuditLogs(ctx)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	if !bytes.HasPrefix(data, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}

	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 3 {
		t.Fatalf("应有表头 + 至少 2 行数据，实际 %d 行", len(records))
	}

	wantHeader := []string{"id", "admin_id", "username", "action", "target", "detail", "created_at"}
	if len(records[0]) != len(wantHeader) {
		t.Fatalf("表头列数 = %d，期望 %d：%v", len(records[0]), len(wantHeader), records[0])
	}
	for i, h := range wantHeader {
		if records[0][i] != h {
			t.Errorf("表头第 %d 列 = %q，期望 %q", i, records[0][i], h)
		}
	}

	// 找到 ban_user 行：admin_id 为空串、detail 是原始 JSON 文本
	// （jsonb 会规范化键值空格为 `{"reason": "测试"}`，这是数据库行为）。
	var found bool
	for _, rec := range records[1:] {
		if rec[3] == "ban_user" {
			found = true
			if rec[1] != "" {
				t.Errorf("NULL admin_id 应导出为空串，实际 %q", rec[1])
			}
			if rec[5] != `{"reason": "测试"}` {
				t.Errorf("detail 应以原始 JSON 文本导出，实际 %q", rec[5])
			}
		}
	}
	if !found {
		t.Error("导出应包含测试审计行")
	}
}

func TestAdminExportAuditLogsBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportAuditLogs(ctx); err == nil {
		t.Fatal("连接池已关闭时导出必须报错，不能静默返回空 CSV")
	}
}
