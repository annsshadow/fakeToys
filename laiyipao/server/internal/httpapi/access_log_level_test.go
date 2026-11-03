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

// TestEveryErrorCodeLogsAtContractLevel 是 R69 重构的**可观测性守卫**。
//
// 背景：R69 把 handler 改成「只 return err」，ErrorHandler 在中间件之后才写响应。
// 于是 requestLogger 在 `c.Next()` 返回时看到的 `StatusCode()` 恒为 200 ——
// 4xx 访问日志一度整体掉进 debug 档（`TestAccessLogLevels` 抓到了）。
//
// 当时的修复是在 requestLogger 里调用 `httperr.StatusOf(err)`。本条把
// 「每个对外错误码 → 正确的日志级别」写成**全表**断言，防止下一次：
//   - 新增错误码时忘了它会在中间件阶段出现；
//   - 有人在 requestLogger 里又把 `err` 分支改回「只打 debug」；
//   - 有人改了 Render 的判定顺序，让 StatusOf 与契约表分叉。
//
// 判据与用户要求一致：4xx 必须在 warn 档可见、error 档不可见；
// 5xx 必须在 error 档可见（且 Render 的原因行也在 error 档）。
func TestEveryErrorCodeLogsAtContractLevel(t *testing.T) {
	contracts := httperr.Contracts()
	// 集合规模下限：契约表被删空时本测试会静默通过 —— 那是它自己失效。
	if len(contracts) < 10 {
		t.Fatalf("契约表只有 %d 个码（下限 10）—— 全表断言失去意义", len(contracts))
	}
	for code, c := range contracts {
		code, c := code, c
		t.Run(code, func(t *testing.T) {
			err := httperr.New(c.Status, code, "x")
			// 先钉住 StatusOf 与契约表同源（中间件依赖的就是这个值）。
			if got := httperr.StatusOf(err); got != c.Status {
				t.Fatalf("StatusOf(%s) = %d，契约表是 %d —— 两处判定已分叉", code, got, c.Status)
			}

			app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
			app.Use(requestLogger())
			app.Get("/x", func(ctx *fiber.Ctx) error { return err })
			req, _ := http.NewRequest("GET", "/x", nil)

			SetLogLevel("error")
			readErr := captureAccessLog(t)
			if _, testErr := app.Test(req); testErr != nil {
				t.Fatalf("请求失败：%v", testErr)
			}
			visibleAtError := strings.Contains(readErr(), "/x")

			SetLogLevel("warn")
			readWarn := captureAccessLog(t)
			if _, testErr := app.Test(req); testErr != nil {
				t.Fatalf("请求失败：%v", testErr)
			}
			visibleAtWarn := strings.Contains(readWarn(), "/x")

			if !visibleAtWarn {
				t.Errorf("%s（HTTP %d）在 warn 档不可见 —— 4xx 访问日志会整体消失",
					code, c.Status)
			}
			wantErr := c.Status >= 500
			if visibleAtError != wantErr {
				t.Errorf("%s（HTTP %d）在 error 档可见=%v，期望 %v",
					code, c.Status, visibleAtError, wantErr)
			}
		})
	}

	// 框架错误（http_error）状态由 *fiber.Error 给出，也必须按同一规则分级。
	for _, fe := range []*fiber.Error{
		fiber.ErrNotFound,              // 404
		fiber.ErrMethodNotAllowed,      // 405
		fiber.ErrRequestEntityTooLarge, // 413
	} {
		fe := fe
		t.Run("fiber_"+fe.Message, func(t *testing.T) {
			app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
			app.Use(requestLogger())
			app.Get("/x", func(ctx *fiber.Ctx) error { return fe })
			req, _ := http.NewRequest("GET", "/x", nil)
			SetLogLevel("warn")
			read := captureAccessLog(t)
			if _, err := app.Test(req); err != nil {
				t.Fatalf("请求失败：%v", err)
			}
			if !strings.Contains(read(), "/x") {
				t.Errorf("框架错误 %d 在 warn 档不可见", fe.Code)
			}
		})
	}
	SetLogLevel("info")
}
