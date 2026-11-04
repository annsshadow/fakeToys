// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R96 P3 应对中的 P2：管理端重算档位端点（此前 0% 覆盖）。
//
// 这两个 handler 是**破坏性操作**的入口 —— 把 enforce 改成 off 等于关掉
// 反作弊校验。它们此前一次都没被走过，意味着「确认口令有没有生效」
// 「非法档位会不会静默降级」「切档有没有写审计」都没有回归保护。
//
// 断言按后果写：档位真的变了、非法档位被拒且**没变**、少 confirm 被拒、
// 审计里确有其事。

import (
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminRecomputeModeGetAndSet(t *testing.T) {
	e := newE2E(t)
	// 切档是写操作：requireWritable 只放行 admin/ops（superadmin 是只读角色）。
	adminID, tok := e.newAdminRole(t, "r96mode", "admin")

	status, body := e.get(t, "/api/v1/admin/recomputes/mode", tok)
	if status != fiber.StatusOK {
		t.Fatalf("GET /admin/recomputes/mode = %d %v", status, body)
	}
	original, _ := body["mode"].(string)
	if original == "" {
		t.Fatalf("应返回当前档位，实际 %v", body)
	}
	if configured, _ := body["configured_mode"].(string); configured != original {
		t.Errorf("启动时未熔断，mode 与 configured_mode 应一致：%q vs %q", original, configured)
	}
	if engine, _ := body["engine_mode"].(string); engine != original {
		t.Errorf("启动时未熔断，engine_mode 应与 mode 一致：%q vs %q", original, engine)
	}
	if skills, _ := body["skills_mode"].(string); skills != original {
		t.Errorf("S 段永不降级，skills_mode 应与 mode 一致：%q vs %q", original, skills)
	}

	// 正常切档：带确认口令 → 生效，并写审计。
	status, body = e.put(t, "/api/v1/admin/recomputes/mode", tok, map[string]any{
		"mode": "enforce", "confirm": "confirm",
	})
	if status != fiber.StatusOK {
		t.Fatalf("PUT mode=enforce = %d %v", status, body)
	}
	if got, _ := body["mode"].(string); got != "enforce" {
		t.Fatalf("切档后 mode 应为 enforce，实际 %v", body)
	}
	if got := e.scalarInt(t, `
		SELECT COUNT(*) FROM admin_audit_logs
		 WHERE admin_id = $1 AND action = 'set_recompute_mode' AND target = 'enforce'`,
		adminID); got != 1 {
		t.Errorf("切档必须写 1 条审计，实际 %d 条", got)
	}

	// 少 confirm → 拒绝，档位不动（防手滑）。
	if status, body := e.put(t, "/api/v1/admin/recomputes/mode", tok, map[string]any{
		"mode": "off",
	}); status != fiber.StatusBadRequest {
		t.Fatalf("缺 confirm 应 400，实际 %d %v", status, body)
	}
	_, body = e.get(t, "/api/v1/admin/recomputes/mode", tok)
	if got, _ := body["mode"].(string); got != "enforce" {
		t.Fatalf("被拒的切档不许改档位：期望 enforce，实际 %q", got)
	}

	// 非法档位 → 拒绝且不静默降级。
	if status, body := e.put(t, "/api/v1/admin/recomputes/mode", tok, map[string]any{
		"mode": "bogus", "confirm": "confirm",
	}); status != fiber.StatusBadRequest {
		t.Fatalf("非法档位应 400，实际 %d %v", status, body)
	}
	_, body = e.get(t, "/api/v1/admin/recomputes/mode", tok)
	if got, _ := body["mode"].(string); got != "enforce" {
		t.Fatalf("非法档位不许静默降级：期望仍为 enforce，实际 %q", got)
	}

	// 无管理员身份 → 401（防止普通玩家把手滑变成事故）。
	_, userTok := e.newPlayer(t, "r96modeuser")
	for _, path := range []string{"/api/v1/admin/recomputes/mode"} {
		if status, _ := e.get(t, path, userTok); status != fiber.StatusUnauthorized {
			t.Errorf("无管理员身份 GET %s 应 401，实际 %d", path, status)
		}
		if status, _ := e.put(t, path, userTok, map[string]any{"mode": "off", "confirm": "confirm"}); status != fiber.StatusUnauthorized {
			t.Errorf("无管理员身份 PUT %s 应 401，实际 %d", path, status)
		}
	}

	// 还原成初始档位，避免本用例对同进程其他测试留下副作用。
	if status, body := e.put(t, "/api/v1/admin/recomputes/mode", tok, map[string]any{
		"mode": original, "confirm": "confirm",
	}); status != fiber.StatusOK {
		t.Fatalf("还原档位 %q 失败：%d %v", original, status, body)
	}
}
