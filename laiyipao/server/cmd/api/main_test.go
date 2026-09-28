package main

// api 入口的可测部分：buildApp 组装的路由与统一错误处理。
// run() 本体（监听端口、信号处理、os.Exit）是进程级副作用，按豁免处理。

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/service"
	"github.com/laiyipao/server/internal/store"
)

// testService 连真实库构造 Service（buildApp 本身不碰库，
// 但路由依赖 Service 的方法集完整存在）。
func testService(t *testing.T) *service.Service {
	t.Helper()
	dsn := os.Getenv("TEST_DATABASE_URL")
	if dsn == "" {
		dsn = "postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
	}
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("无可用测试数据库，跳过：%v", err)
	}
	t.Cleanup(db.Close)
	return service.New(db, cfg)
}

// TestBuildAppRoutesAreUp 完整路由必须挂上（健康检查可达、
// 鉴权端点对无凭证请求返回 401 而不是 404 —— 404 说明路由没注册）。
func TestBuildAppRoutesAreUp(t *testing.T) {
	app := buildApp(testService(t))

	resp, err := app.Test(httptest.NewRequest(http.MethodGet, "/healthz", nil), 5000)
	if err != nil {
		t.Fatalf("healthz 请求失败：%v", err)
	}
	resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("healthz 应 200，实际 %d", resp.StatusCode)
	}

	// 无凭证访问受保护端点 → 401（路由存在 + 中间件生效）
	resp2, err := app.Test(httptest.NewRequest(http.MethodGet, "/api/v1/me/", nil), 5000)
	if err != nil {
		t.Fatalf("me 请求失败：%v", err)
	}
	resp2.Body.Close()
	if resp2.StatusCode != fiber.StatusUnauthorized {
		t.Errorf("无凭证 /me 应 401，实际 %d", resp2.StatusCode)
	}
}

// TestBuildAppErrorHandler 统一错误处理：fiber.Error 转结构化 JSON，
// 其它错误原样上抛。后台前端依赖 {"error":{...}} 形状解析 404。
func TestBuildAppErrorHandler(t *testing.T) {
	app := buildApp(testService(t))

	resp, err := app.Test(httptest.NewRequest(http.MethodGet, "/api/v1/definitely-missing", nil), 5000)
	if err != nil {
		t.Fatalf("404 请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Fatalf("未知路由应 404，实际 %d", resp.StatusCode)
	}
	var body struct {
		Error struct {
			Code    string `json:"code"`
			Message string `json:"message"`
		} `json:"error"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("404 响应不是结构化 JSON（Error Handler 未生效）：%v", err)
	}
	if body.Error.Code != "http_error" {
		t.Errorf("错误码 = %q，期望 http_error", body.Error.Code)
	}
}

// TestBuildAppErrorHandlerPassthrough 非 *fiber.Error 的错误必须**原样上抛**
// （不被包装成 http_error）。构造：在 buildApp 产出的 app 上临时挂一个返回
// 普通 error 的路由，命中它 → ErrorHandler 走 `return err` 分支。
// 意义：只有 fiber.Error 才应被翻成结构化 body；其它错误保持默认 500，
// 若把它们也当 http_error 会掩盖真正的内部故障类型。
func TestBuildAppErrorHandlerPassthrough(t *testing.T) {
	app := buildApp(testService(t))
	app.Get("/_test/plain-error", func(_ *fiber.Ctx) error {
		return errors.New("这是一个非 fiber.Error 的普通错误")
	})

	resp, err := app.Test(httptest.NewRequest(http.MethodGet, "/_test/plain-error", nil), 5000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	// 默认 fiber 处理普通错误 → 500
	if resp.StatusCode != fiber.StatusInternalServerError {
		t.Errorf("普通错误应走默认 500，实际 %d", resp.StatusCode)
	}
}
