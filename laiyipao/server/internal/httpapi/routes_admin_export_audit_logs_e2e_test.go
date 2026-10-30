package httpapi

// 后台审计日志 CSV 导出的端到端测试（2026-10-30 实质优化第 15 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 角色**可**导出（读操作，不挂 requireWritable）；
//   - 响应是带 BOM 的 CSV 且包含审计数据。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"io"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportAuditLogsRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/audit-logs/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportAuditLogsReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "exportauditro", "readonly")

	req, _ := http.NewRequest("GET", "/api/v1/admin/audit-logs/export", nil)
	req.Header.Set("Authorization", "Bearer "+roTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("readonly 导出应 200，实际 %d", resp.StatusCode)
	}
	if ct := resp.Header.Get("Content-Type"); ct != "text/csv; charset=utf-8" {
		t.Errorf("Content-Type 应为 text/csv，实际 %q", ct)
	}
}

func TestE2EAdminExportAuditLogsContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "exportaudit")

	// 直接插入一条审计日志
	action := fmt.Sprintf("test_export_%d", time.Now().UnixNano())
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO admin_audit_logs (username, action, target, detail)
		 VALUES ('e2e_admin', $1, '123', '{"k":"v"}'::jsonb)`, action); err != nil {
		t.Fatalf("插入审计日志失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/audit-logs/export", nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("读响应失败：%v", err)
	}
	if !bytes.HasPrefix(raw, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}
	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(raw, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 2 {
		t.Fatalf("应有表头 + 数据，实际 %d 行", len(records))
	}
	found := false
	for _, rec := range records[1:] {
		if rec[3] == action {
			found = true
			// jsonb 会规范化键值空格为 {"k": "v"}
			if rec[5] != `{"k": "v"}` {
				t.Errorf("detail 应以原始 JSON 导出，实际 %q", rec[5])
			}
		}
	}
	if !found {
		t.Errorf("CSV 应包含测试审计行 %q，实际 %v", action, records)
	}
}
