// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R67：管理端鉴权穷举探针。
//
// 不做抽查：从 Fiber 的路由表里枚举**所有** `/api/v1/admin*` 路由，
// 每条分别用「无 token」和「玩家 token」各打一次，断言只可能是
// 401/403。漏挂 `requireAdmin` 的路由会落到 handler，返回 200/400/404
// 等其它状态，立即被这个用例抓出（不再依赖人肉检查路由分组）。
//
// 登录路由 `/api/v1/admin/login` 是公开的，显式排除。

import (
	"net/http/httptest"
	"regexp"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

var routeParamRe = regexp.MustCompile(`:[A-Za-z_][A-Za-z0-9_]*`)

func TestEveryAdminRouteRequiresAdmin(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	_, playerTok := e.newPlayer(t, "admin_guard")

	checked := 0
	for _, r := range e.app.GetRoutes() {
		if !strings.HasPrefix(r.Path, "/api/v1/admin") || r.Path == "/api/v1/admin/login" {
			continue
		}
		// OPTIONS 是 CORS 预检，由全局 CORS 中间件在鉴权前直接回 204，
		// 不会进入 handler，也不可能读到管理端数据。把它算成「漏鉴权」是误报。
		if r.Method == fiber.MethodOptions {
			continue
		}
		// 参数替换成 1 只是为了能匹配到路由；鉴权在 handler 之前，
		// 所以参数是否存在、是否合法都不影响本用例的结论。
		path := routeParamRe.ReplaceAllString(r.Path, "1")
		checked++

		for _, tc := range []struct{ name, token string }{
			{"无 token", ""},
			{"玩家 token", playerTok},
		} {
			req := httptest.NewRequest(r.Method, path, nil)
			if tc.token != "" {
				req.Header.Set("Authorization", "Bearer "+tc.token)
			}
			resp, err := e.app.Test(req, 10000)
			if err != nil {
				t.Fatalf("%s %s（%s）请求失败：%v", r.Method, path, tc.name, err)
			}
			resp.Body.Close()
			if resp.StatusCode != fiber.StatusUnauthorized && resp.StatusCode != fiber.StatusForbidden {
				t.Errorf("%s %s（%s）返回 %d，期望 401/403 —— 该路由可能漏挂 requireAdmin",
					r.Method, path, tc.name, resp.StatusCode)
			}
		}
	}
	// 空转保护：路由表若因注册方式变化而枚举不到，必须报错而不是静默通过。
	if checked < 20 {
		t.Fatalf("只枚举到 %d 条管理端路由，明显不对（路由表/前缀变了？）", checked)
	}
	t.Logf("穷举覆盖 %d 条管理端路由 × 2 种身份 = %d 次鉴权断言", checked, checked*2)
}
