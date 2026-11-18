// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/httpapi/httperr"
)

// TestCORSPreflightAllowsIdempotencyKey 钉住 CORS 预检白名单必须放行
// `Idempotency-Key` 自定义请求头。
//
// 真实缺陷：H5 端（miniapp）buyItem() 给 POST /shop/:id/buy 带自定义头
// `Idempotency-Key`（client.ts 的 request/buyItem）。浏览器对带自定义头的
// 跨域请求必定先发 OPTIONS 预检，且要求预检响应 Access-Control-Allow-Headers
// 覆盖该头，否则拒绝发起真正 POST。修前 cors() 只放行
// `Content-Type,Authorization`，H5 跨域购买整批被拦在预检阶段 —— 正是
// routes.go 注释警告过的「忘改预检白名单症状是整批请求发不出去」。
// 既有 TestCORSHeadersAndPreflight 只验 Allow-Origin 与 204，漏掉头本身。
//
// 判据必须是实测预检响应头：业务代码回退（白名单去掉该头）时本用例 RED。
func TestCORSPreflightAllowsIdempotencyKey(t *testing.T) {
	withDevMode(t, true) // dev 才下发 Allow-Headers
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
	app.Use(cors(nil))
	app.Post("/api/v1/shop/1/buy", func(c *fiber.Ctx) error { return c.SendStatus(fiber.StatusOK) })

	pre := httptest.NewRequest(http.MethodOptions, "/api/v1/shop/1/buy", nil)
	pre.Header.Set("Origin", "http://localhost:5173")
	pre.Header.Set("Access-Control-Request-Method", http.MethodPost)
	pre.Header.Set("Access-Control-Request-Headers", "content-type,authorization,idempotency-key")

	resp, err := app.Test(pre, 5000)
	if err != nil {
		t.Fatalf("预检请求失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusNoContent {
		t.Fatalf("预检应 204，实际 %d", resp.StatusCode)
	}
	got := resp.Header.Get("Access-Control-Allow-Headers")
	if !strings.Contains(strings.ToLower(got), "idempotency-key") {
		t.Fatalf("CORS 预检 Allow-Headers 未放行 Idempotency-Key，H5 跨域购买会被浏览器拦截；实际 = %q", got)
	}
	low := strings.ToLower(got)
	for _, want := range []string{"content-type", "authorization"} {
		if !strings.Contains(low, want) {
			t.Errorf("CORS 预检 Allow-Headers 缺少原有头 %q；实际 = %q", want, got)
		}
	}
}
