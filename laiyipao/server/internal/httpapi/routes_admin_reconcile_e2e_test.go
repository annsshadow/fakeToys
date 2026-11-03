// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R78：GET /admin/reconcile 的端到端：鉴权（路由挂载没挂错组）+ 报告形状。

import (
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminReconcile(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	// 玩家 token 不得访问（requireAdmin 在组上；穷举守卫另有覆盖，这里钉本路由）。
	_, playerTok := e.newPlayer(t, "r78recon")
	if st, _ := e.get(t, "/api/v1/admin/reconcile", playerTok); st != fiber.StatusForbidden && st != fiber.StatusUnauthorized {
		t.Errorf("玩家调对账应 401/403，实际 %d", st)
	}

	_, adminTok := e.newAdmin(t, "recon")
	st, body := e.get(t, "/api/v1/admin/reconcile", adminTok)
	if st != fiber.StatusOK {
		t.Fatalf("对账应 200，实际 %d %v", st, body)
	}
	if _, ok := body["ok"].(bool); !ok {
		t.Errorf("响应缺 ok 布尔字段：%v", body)
	}
	if num(body, "checks") != 9 {
		t.Errorf("checks=%v，期望 9（九项不变量）", body["checks"])
	}
	if _, ok := body["duration_ms"].(float64); !ok {
		t.Errorf("缺 duration_ms：%v", body)
	}
	if _, ok := body["notes"].([]any); !ok || len(body["notes"].([]any)) == 0 {
		t.Errorf("notes 必须随报告下发（合法漂移源说明）：%v", body["notes"])
	}
}
