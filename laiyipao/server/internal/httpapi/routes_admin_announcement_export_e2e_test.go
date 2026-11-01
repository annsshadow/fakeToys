package httpapi

// 后台公告 CSV 导出的端到端测试（2026-10-31 实质优化第 38 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 可导出（读操作）；
//   - 响应带 BOM 且包含公告数据。

import (
	"bytes"
	"context"
	"encoding/csv"
	"io"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportAnnouncementsRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/announcements/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportAnnouncementsReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "anndro", "readonly")
	req, _ := http.NewRequest("GET", "/api/v1/admin/announcements/export", nil)
	req.Header.Set("Authorization", "Bearer "+roTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("readonly 导出应 200，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportAnnouncementsContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "anndexp")
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO announcements (title, body, published)
		 VALUES ('导出标题', '导出正文', TRUE)`); err != nil {
		t.Fatalf("插入公告失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/announcements/export", nil)
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
	found := false
	for _, rec := range records[1:] {
		if rec[1] == "导出标题" {
			found = true
			if rec[2] != "导出正文" {
				t.Errorf("body 应为 导出正文，实际 %q", rec[2])
			}
			if rec[3] != "true" {
				t.Errorf("published 应为 true，实际 %q", rec[3])
			}
		}
	}
	if !found {
		t.Errorf("CSV 应包含测试公告")
	}
}
