// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R76-④：R72 全局限流器的额外开销基准。
//
// 基线路径（/config、/leaderboard…）没有挂限流器，不受 R72 影响；
// 受影响的是挂了限流的五条写路由（多一层进程内计数检查）。
// 这里量「多挂一层 globalLimiter」本身的成本：同一 dummy 路由，
// 无限流 / 单层窄层 / 窄层+全局层 三组对照。

import (
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/httpapi/httperr"
)

func benchChain(b *testing.B, middlewares ...fiber.Handler) {
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render, DisableStartupMessage: true})
	handlers := append(middlewares, func(c *fiber.Ctx) error { return c.SendString("ok") })
	app.Get("/x", handlers...)
	req := httptest.NewRequest("GET", "/x", nil)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		resp, err := app.Test(req, -1)
		if err != nil {
			b.Fatalf("请求失败：%v", err)
		}
		resp.Body.Close()
		if resp.StatusCode != 200 {
			b.Fatalf("状态 %d", resp.StatusCode)
		}
	}
}

func BenchmarkLimiterNone(b *testing.B)       { benchChain(b) }
func BenchmarkLimiterNarrowOnly(b *testing.B) { benchChain(b, perIPLimiter("bench", 1_000_000)) }
func BenchmarkLimiterNarrowPlusGlobal(b *testing.B) {
	benchChain(b, perIPLimiter("bench", 1_000_000), globalLimiter("bench", 1_000_000))
}
