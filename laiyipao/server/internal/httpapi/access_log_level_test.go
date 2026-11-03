// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 访问日志分级的测试（第 4 轮）。
//
// 判据不是「打了日志没有」，而是**级别阶梯**：
//   - error 档只留 500 原因（线上最该看的）
//   - info 档加慢请求
//   - warn 档加 4xx
//   - debug 档全量
//
// LOG_LEVEL 此前是死配置（config 解析后全仓零消费），这里同时钉住
// 「接线真的生效」与「未知值回退 info 且出声」。

import (
	"bytes"
	"errors"
	"log"
	"net/http"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/httpapi/httperr"
)

func TestParseLogLevel(t *testing.T) {
	cases := []struct {
		in   string
		want logLevel
	}{
		{"debug", logDebug},
		{"DEBUG", logDebug},
		{"info", logInfo},
		{"", logInfo},
		{"warn", logWarn},
		{"warning", logWarn},
		{"error", logError},
	}
	for _, tc := range cases {
		if got := parseLogLevel(tc.in); got != tc.want {
			t.Errorf("parseLogLevel(%q) = %d, want %d", tc.in, got, tc.want)
		}
	}
	// 未知值必须回退 info 并打警告，不能静默 —— 配置写错会让运维
	// 以为日志在记，实际档位完全不同。
	var buf bytes.Buffer
	flags := log.Flags()
	out := log.Writer()
	log.SetOutput(&buf)
	log.SetFlags(0)
	t.Cleanup(func() { log.SetOutput(out); log.SetFlags(flags) })
	if got := parseLogLevel("verbose"); got != logInfo {
		t.Errorf("未知 LOG_LEVEL 应回退 info，实际 %d", got)
	}
	if !strings.Contains(buf.String(), "未知 LOG_LEVEL") {
		t.Errorf("未知 LOG_LEVEL 必须打警告，实际日志: %q", buf.String())
	}
}

// newAccessLogApp 构一个只挂 requestLogger 的最小 app（不走真库）。
func newAccessLogApp() *fiber.App {
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
	app.Use(requestLogger())
	app.Get("/ok", func(c *fiber.Ctx) error { return c.SendString("ok") })
	app.Get("/nf", func(c *fiber.Ctx) error {
		return httperr.New(fiber.StatusNotFound, httperr.CodeNotFound, "没有")
	})
	app.Get("/boom", func(c *fiber.Ctx) error { return errors.New("db down") })
	app.Get("/slow", func(c *fiber.Ctx) error {
		time.Sleep(5 * time.Millisecond)
		return c.SendString("slow")
	})
	return app
}

// captureAccessLog 把 stdlib log 重定向到缓冲区并返回读取函数。
func captureAccessLog(t *testing.T) func() string {
	t.Helper()
	var buf bytes.Buffer
	flags := log.Flags()
	out := log.Writer()
	log.SetOutput(&buf)
	log.SetFlags(0)
	t.Cleanup(func() { log.SetOutput(out); log.SetFlags(flags) })
	return buf.String
}

func TestAccessLogLevels(t *testing.T) {
	app := newAccessLogApp()

	t.Run("500原因在error档", func(t *testing.T) {
		SetLogLevel("error")
		read := captureAccessLog(t)
		req, _ := http.NewRequest("GET", "/boom", nil)
		app.Test(req)
		if !strings.Contains(read(), "db down") {
			t.Errorf("error 档必须记 500 真实原因，实际: %q", read())
		}
	})

	t.Run("4xx在warn档且error档不可见", func(t *testing.T) {
		SetLogLevel("warn")
		read := captureAccessLog(t)
		req, _ := http.NewRequest("GET", "/nf", nil)
		app.Test(req)
		if !strings.Contains(read(), "-> 404") {
			t.Errorf("warn 档必须记 4xx，实际: %q", read())
		}
		SetLogLevel("error")
		read = captureAccessLog(t)
		req, _ = http.NewRequest("GET", "/nf", nil)
		app.Test(req)
		if strings.Contains(read(), "/nf") {
			t.Errorf("error 档不应记 4xx 访问，实际: %q", read())
		}
	})

	t.Run("慢请求在info档", func(t *testing.T) {
		old := slowRequestThreshold
		slowRequestThreshold = time.Millisecond
		t.Cleanup(func() { slowRequestThreshold = old })
		SetLogLevel("info")
		read := captureAccessLog(t)
		req, _ := http.NewRequest("GET", "/slow", nil)
		app.Test(req)
		if !strings.Contains(read(), "慢请求") {
			t.Errorf("info 档必须记慢请求，实际: %q", read())
		}
	})

	t.Run("成功访问仅debug档", func(t *testing.T) {
		SetLogLevel("debug")
		read := captureAccessLog(t)
		req, _ := http.NewRequest("GET", "/ok", nil)
		app.Test(req)
		if !strings.Contains(read(), "-> 200") {
			t.Errorf("debug 档必须记成功访问，实际: %q", read())
		}
		SetLogLevel("info")
		read = captureAccessLog(t)
		req, _ = http.NewRequest("GET", "/ok", nil)
		app.Test(req)
		if strings.Contains(read(), "/ok") {
			t.Errorf("info 档不应记成功访问，实际: %q", read())
		}
	})
}

// TestRequestDurationHeaderStillSet：分级化不能丢掉原有的响应头职责。
func TestRequestDurationHeaderStillSet(t *testing.T) {
	SetLogLevel("error")
	read := captureAccessLog(t)
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
	app.Use(requestLogger())
	app.Get("/ok", func(c *fiber.Ctx) error { return c.SendString("ok") })
	req, _ := http.NewRequest("GET", "/ok", nil)
	resp, _ := app.Test(req)
	if resp.Header.Get("X-Request-Duration") == "" {
		t.Error("X-Request-Duration 头必须保留")
	}
	if strings.Contains(read(), "/ok") {
		t.Error("error 档不应记成功访问")
	}
}
