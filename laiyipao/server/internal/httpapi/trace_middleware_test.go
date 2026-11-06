// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// trace_middleware_test.go — R84 请求级追踪标识的行为测试。
//
// 守的东西：
//  1. 生成：无头请求拿到 32 位 hex id，出现在响应头与访问日志；
//  2. 继承：合法 X-Request-Id 透传到响应头与日志；
//  3. 注入纪律：非法头**重新生成而不是原样回显**——
//     日志里绝不能出现客户端塞进头里的任意字符串；
//  4. 链路：同一个 id 同时出现在访问日志与 httperr 错误详情行
//    （否则「按 id 拼故事」在错误路径上断裂）。

import (
	"errors"
	"net/http"
	"regexp"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/laiyipao/server/internal/httpapi/httperr"
)

// newTraceTestApp 挂 trace + 访问日志的最小 app（含一条错误路由）。
func newTraceTestApp() *fiber.App {
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
	app.Use(traceMiddleware())
	app.Use(requestLogger())
	app.Get("/ok", func(c *fiber.Ctx) error { return c.SendString("ok") })
	app.Get("/nf", func(c *fiber.Ctx) error {
		return httperr.New(fiber.StatusNotFound, httperr.CodeNotFound, "没有")
	})
	app.Get("/boom", func(c *fiber.Ctx) error { return errors.New("db down") })
	return app
}

var hex32 = regexp.MustCompile(`^[0-9a-f]{32}$`)

// doReq 打一次请求并返回状态码与响应头里的 X-Request-Id。
func doReq(t *testing.T, app *fiber.App, path, reqID string) (int, string) {
	t.Helper()
	req, _ := testReq(http.MethodGet, path, nil)
	if reqID != "" {
		req.Header.Set("X-Request-Id", reqID)
	}
	resp, err := app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	return resp.StatusCode, resp.Header.Get("X-Request-Id")
}

func TestTraceIDGeneratedWhenAbsent(t *testing.T) {
	SetLogLevel("debug")
	app := newTraceTestApp()
	read := captureAccessLog(t)

	_, id := doReq(t, app, "/ok", "")
	if !hex32.MatchString(id) {
		t.Fatalf("无头时应生成 32 位 hex id，实际 %q", id)
	}
	if !strings.Contains(read(), "req="+id+" ") {
		t.Errorf("访问日志应含 req=%s，实际: %q", id, read())
	}
}

func TestTraceIDInheritedWhenValid(t *testing.T) {
	SetLogLevel("debug")
	app := newTraceTestApp()
	read := captureAccessLog(t)

	_, id := doReq(t, app, "/ok", "r84-inherit_TEST-01")
	if id != "r84-inherit_TEST-01" {
		t.Fatalf("合法头应继承，实际响应头 %q", id)
	}
	if !strings.Contains(read(), "req=r84-inherit_TEST-01 ") {
		t.Errorf("访问日志应含继承的 id，实际: %q", read())
	}
}

// TestTraceIDRejectsMalformed：非法头绝不能原样回显进日志/响应头。
// 这是日志注入面：`\r\n` 可伪造日志行、超长串爆磁盘、敏感词绕过禁词检查。
func TestTraceIDRejectsMalformed(t *testing.T) {
	SetLogLevel("debug")
	app := newTraceTestApp()
	read := captureAccessLog(t)

	malformed := []string{
		"short",                                  // 太短
		strings.Repeat("x", 65),                  // 太长
		"has space in id",                        // 空格
		"中文请求标识符",                                // 非 ASCII
		"ok\r\nX-Forged-Log-Line: fake",          // CRLF 注入
		"ok\n[http] req=admin GET /admin -> 200", // 伪造日志行
		"'; DROP TABLE users;--",                 // 任意 payload
	}
	for _, bad := range malformed {
		_, id := doReq(t, app, "/ok", bad)
		if id == bad {
			t.Errorf("非法头 %q 被原样回显 —— 日志注入面", bad)
		}
		if !hex32.MatchString(id) {
			t.Errorf("非法头 %q 的替代值应是 32 位 hex，实际 %q", bad, id)
		}
		if strings.Contains(read(), bad) {
			t.Errorf("日志里出现了客户端注入值 %q: %q", bad, read())
		}
	}
}

// TestTraceIDLinksAccessAndErrorLog：同一 id 必须同时出现在
// 访问日志与 httperr 错误详情行 —— 否则错误路径上拼不出完整故事。
//
// 用 500 而不是 4xx：4xx 按设计是 logNone（客户端错误不留详情，
// httperr.classify），真实故障（5xx/瞬时 503）才是需要拼故事的场景。
func TestTraceIDLinksAccessAndErrorLog(t *testing.T) {
	SetLogLevel("error")
	app := newTraceTestApp()
	read := captureAccessLog(t)

	_, id := doReq(t, app, "/boom", "")
	var accessLine, errorLine string
	for _, line := range strings.Split(read(), "\n") {
		if strings.Contains(line, "req="+id) && strings.Contains(line, "-> 500 (") {
			accessLine = line
		}
		if strings.Contains(line, "req="+id) && strings.Contains(line, "-> 500: ") {
			errorLine = line
		}
	}
	if accessLine == "" {
		t.Errorf("缺访问日志行（req=%s -> 500 (dur)）: %q", id, read())
	}
	if errorLine == "" {
		t.Errorf("缺错误详情行（req=%s -> 500: err）: %q", id, read())
	}
}

// TestTraceIDPatternCoversGenerated：生成器输出必须落在白名单内
// （否则自己产的 id 自己都不认，荒诞但要钉住）。
func TestTraceIDPatternCoversGenerated(t *testing.T) {
	for i := 0; i < 100; i++ {
		id := newTraceID()
		if !traceIDPattern.MatchString(id) {
			t.Fatalf("生成的 id %q 不在白名单内", id)
		}
	}
}
