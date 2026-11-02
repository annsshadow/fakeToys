// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/service"
)

// 导出超限必须映射成 413 + 可分类的 code，而不是掉进 500。
// 掉 500 的后果：运营看到「服务内部错误」，会以为系统坏了并反复重试，
// 而真正要做的是缩小筛选范围。
func TestFailErrMapsExportTooLargeTo413(t *testing.T) {
	app := fiber.New()
	app.Get("/export", func(c *fiber.Ctx) error {
		return failErr(c, service.ErrExportTooLarge)
	})

	resp, err := app.Test(httptest.NewRequest("GET", "/export", nil))
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != fiber.StatusRequestEntityTooLarge {
		t.Fatalf("status = %d，期望 413", resp.StatusCode)
	}
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("读取响应体失败：%v", err)
	}
	if !strings.Contains(string(body), "export_too_large") {
		t.Fatalf("响应缺少 export_too_large 代码：%s", body)
	}
}

// 字节预算必须能沿「handler → service → failErr」一路走到 413。
//
// 单元用例证明了缓冲会拒写，这条证明**装配没有漏**：handler 用的是
// service 的导出方法，错误经 failErr 映射成 413；SQL 的 10 万行上限
// 完全没被触及（只有 1 行），所以超限只可能来自字节预算。
func TestE2EExportByteBudgetReturns413(t *testing.T) {
	e2eScratchDB(t)
	e := newE2EWith(t, func(c *config.Config) { c.ExportMaxBytes = 1 << 10 })

	_, adminTok := e.newAdmin(t, "exportbytes")
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO announcements (title, body) VALUES ('byte-budget', repeat('x', 4096))`); err != nil {
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
	if resp.StatusCode != fiber.StatusRequestEntityTooLarge {
		t.Fatalf("字节超预算应 413，实际 %d：%s", resp.StatusCode, raw)
	}
	if !strings.Contains(string(raw), "export_too_large") {
		t.Fatalf("响应缺少 export_too_large 代码：%s", raw)
	}
	// 文案要点明是字节预算，而不是让运营以为行数超了（两者该做的动作不同）。
	if !strings.Contains(string(raw), "字节") {
		t.Fatalf("响应应说明字节预算：%s", raw)
	}
}
