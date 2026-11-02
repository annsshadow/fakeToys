// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
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

// R30：导出必须与列表同口径（published），否则筛完草稿/已发布导出拿到的是全部。
func TestE2EAdminExportAnnouncementsFilters(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "anndexpf")

	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO announcements (title, body, published) VALUES ('R30 已发布', '正文', TRUE)`); err != nil {
		t.Fatalf("插入已发布公告失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO announcements (title, body, published) VALUES ('R30 草稿', '正文', FALSE)`); err != nil {
		t.Fatalf("插入草稿公告失败：%v", err)
	}

	for _, q := range []string{"published=1", "published=0"} {
		listURL := "/api/v1/admin/announcements?limit=50&" + q
		status, body := e.get(t, listURL, adminTok)
		if status != 200 {
			t.Fatalf("%s 应 200，实际 %d", listURL, status)
		}
		total, _ := body["total"].(float64)
		status, raw := e.getRaw(t, "/api/v1/admin/announcements/export?"+q, adminTok)
		if status != 200 {
			t.Fatalf("导出 %s 应 200，实际 %d", q, status)
		}
		records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(raw, []byte("\xEF\xBB\xBF")))).ReadAll()
		if err != nil {
			t.Fatalf("解析 CSV 失败：%v", err)
		}
		if got := float64(len(records) - 1); got != total {
			t.Fatalf("%s 导出 %.0f 行，列表 total=%.0f（口径不一致）", q, got, total)
		}
	}
}
