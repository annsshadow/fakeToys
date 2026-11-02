// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"io"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"

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
