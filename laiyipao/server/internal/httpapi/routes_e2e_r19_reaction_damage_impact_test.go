// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R19 后端 E2E：GET /api/v1/me/reaction-damage-impact
// 严格按 user_id 隔离；count 取 reactions_used；旧战报次数仍入 count。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func r19InsertBattle(t *testing.T, e *e2e, uid int64, timeline, used string) int64 {
	t.Helper()
	var id int64
	err := e.pool.QueryRow(context.Background(), `
		INSERT INTO battle_records (user_id, level_id, result, replay_hash, reactions_used, reaction_timeline)
		VALUES ($1, 1, 'win', 'h', $2::jsonb, $3::jsonb) RETURNING id`, uid, used, timeline).Scan(&id)
	if err != nil {
		t.Fatalf("建 R19 战报失败：%v", err)
	}
	return id
}

func TestE2ER19MeReactionDamageImpactIsolation(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	uidA, tokA := e.newPlayer(t, "r19a")
	uidB, tokB := e.newPlayer(t, "r19b")
	timelineA := `{"schema_version":"reaction-timeline-v1","events":[{"wave":1,"tick":1,"reaction":"overheat","enemy_uid":1,"damage":30}],"truncated":false,"total":1}`
	timelineB := `{"schema_version":"reaction-timeline-v1","events":[{"wave":1,"tick":1,"reaction":"steam_burst","enemy_uid":2,"damage":70}],"truncated":false,"total":1}`
	r19InsertBattle(t, e, uidA, timelineA, `{"overheat":5}`)
	r19InsertBattle(t, e, uidB, timelineB, `{"steam_burst":2}`)

	status, bodyA := e.get(t, "/api/v1/me/reaction-damage-impact", tokA)
	if status != fiber.StatusOK {
		t.Fatalf("A 应 200，实际 %d %v", status, bodyA)
	}
	status, bodyB := e.get(t, "/api/v1/me/reaction-damage-impact", tokB)
	if status != fiber.StatusOK {
		t.Fatalf("B 应 200，实际 %d %v", status, bodyB)
	}
	if num(bodyA, "window_days") != 30 {
		t.Fatalf("window_days 应为 30：%v", bodyA)
	}
	itemsA, ok := bodyA["items"].([]any)
	if !ok || len(itemsA) != 1 {
		t.Fatalf("A 应只有 1 行：%v", bodyA)
	}
	itA, _ := itemsA[0].(map[string]any)
	if itA["reaction"] != "overheat" || itA["count"] != float64(5) || itA["total_damage"] != float64(30) {
		t.Fatalf("A 数据不符（应只有自己的 overheat）：%v", itA)
	}
	itemsB, ok := bodyB["items"].([]any)
	if !ok || len(itemsB) != 1 {
		t.Fatalf("B 应只有 1 行：%v", bodyB)
	}
	itB, _ := itemsB[0].(map[string]any)
	if itB["reaction"] != "steam_burst" || itB["count"] != float64(2) || itB["total_damage"] != float64(70) {
		t.Fatalf("B 数据不符（应只有自己的 steam_burst）：%v", itB)
	}
}

func TestE2ER19MeReactionDamageImpactOldBattles(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "r19old")
	// 旧战报：有 reactions_used 权威次数，但 timeline 为空数组。
	r19InsertBattle(t, e, uid, `[]`, `{"overheat":4}`)
	status, body := e.get(t, "/api/v1/me/reaction-damage-impact", tok)
	if status != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d %v", status, body)
	}
	if num(body, "excluded_old_battles") < 1 {
		t.Fatalf("旧战报应计入 excluded：%v", body)
	}
	items, _ := body["items"].([]any)
	var found bool
	for _, raw := range items {
		it, _ := raw.(map[string]any)
		if it["reaction"] == "overheat" && it["count"] == float64(4) {
			found = true
		}
	}
	if !found {
		t.Fatalf("旧战报次数仍应入 count：%v", items)
	}
}

func TestE2ER19MeReactionDamageImpactEmpty(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "r19empty")
	status, body := e.get(t, "/api/v1/me/reaction-damage-impact", tok)
	if status != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d %v", status, body)
	}
	items, ok := body["items"].([]any)
	if !ok || len(items) != 0 {
		t.Fatalf("新玩家 items 应为 []：%v", body)
	}
}
