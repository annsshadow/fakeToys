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

// TestCORSExposesDateForConfigClockCalibration 钉住 CORS 响应必须暴露 `Date` 头。
//
// 真实缺陷：miniapp H5 端（client.ts 的 fetchConfigIfVersion / headerTimeMs）
// 在 /config 返回 304（无响应体）时，只能靠 HTTP 标准 `Date` 响应头校准本地时钟
// （estimateServerNowMs），用于判断护盾 shielded_until 是否仍生效。
// 但 `Date` 不在 CORS safelisted response headers 之列，浏览器跨域 XHR 默认
// 读不到它；服务端 cors() 只设 Allow-Origin/Methods/Headers/Max-Age，
// 没有 Access-Control-Expose-Headers。于是 H5 跨域时 304 路径拿到的
// serverTimeMs 恒为 0 → store 回退本地时钟 → 时钟偏移随缓存存在时间漂移，
// 症状是「过期护盾显示为生效中」或反之，且只在长时间未更新配置的老玩家复现。
//
// H5 默认 BASE_URL=http://127.0.0.1:8080/api/v1，dev server 在别的 origin，
// 这是真实跨域场景（第 81 轮 Idempotency-Key 预检缺陷同型）。
//
// 判据必须是实测响应头：把 Expose-Headers 去掉时本用例 RED。
func TestCORSExposesDateForConfigClockCalibration(t *testing.T) {
	withDevMode(t, true)
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
	app.Use(cors([]string{"http://localhost:5173"}))
	app.Get("/api/v1/config", func(c *fiber.Ctx) error {
		return c.Status(fiber.StatusNotModified).Send(nil)
	})

	req := httptest.NewRequest(http.MethodGet, "/api/v1/config?if_version=abc", nil)
	req.Header.Set("Origin", "http://localhost:5173")
	resp, err := app.Test(req, 5000)
	if err != nil {
		t.Fatalf("config 请求失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusNotModified {
		t.Fatalf("版本号匹配应 304，实际 %d", resp.StatusCode)
	}
	got := resp.Header.Get("Access-Control-Expose-Headers")
	if !strings.Contains(strings.ToLower(got), "date") {
		t.Fatalf("CORS 响应未暴露 Date 头，H5 跨域下 /config 304 的时钟校准会失效；实际 Access-Control-Expose-Headers = %q", got)
	}
}
