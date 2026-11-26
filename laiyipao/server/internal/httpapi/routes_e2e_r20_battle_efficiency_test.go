// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R20 后端 E2E：GET /api/v1/me/battle-efficiency 与 /api/v1/admin/battle-efficiency。
// 无 PG 时按仓库既有约定 Skip，不算失败。

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func r20InsertBattle(t *testing.T, e *e2e, uid int64, createdAt time.Time, shots, hits, heat, leaked, duration int) {
	t.Helper()
	_, err := e.pool.Exec(context.Background(), `
		INSERT INTO battle_records (user_id, level_id, result, created_at, shots, hits, heat_max, leaked, duration_ms, replay_hash)
		VALUES ($1, 1, 'win', $2, $3, $4, $5, $6, $7, 'r20hash')`, uid, createdAt, shots, hits, heat, leaked, duration)
	if err != nil {
		t.Fatalf("建 R20 战报失败：%v", err)
	}
}

func r20Buckets(t *testing.T, body map[string]any) []any {
	t.Helper()
	b, ok := body["buckets"].([]any)
	if !ok {
		t.Fatalf("buckets 必须是数组（空也非 null）：%v", body["buckets"])
	}
	return b
}

func TestE2ER20MeBattleEfficiencyIsolationAndFields(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	uidA, tokA := e.newPlayer(t, "r20a")
	uidB, tokB := e.newPlayer(t, "r20b")
	recent := time.Now().UTC().Add(-24 * time.Hour)
	r20InsertBattle(t, e, uidA, recent, 10, 7, 60, 2, 120000)
	r20InsertBattle(t, e, uidB, recent, 4, 1, 20, 5, 60000)

	status, bodyA := e.get(t, "/api/v1/me/battle-efficiency?window_days=30", tokA)
	if status != fiber.StatusOK {
		t.Fatalf("A 应 200，实际 %d %v", status, bodyA)
	}
	status, bodyB := e.get(t, "/api/v1/me/battle-efficiency?window_days=30", tokB)
	if status != fiber.StatusOK {
		t.Fatalf("B 应 200，实际 %d %v", status, bodyB)
	}
	for _, m := range []map[string]any{bodyA, bodyB} {
		for _, k := range []string{"window_days", "window_from", "window_to", "battles", "shots", "hits", "accuracy_pct", "avg_heat_max", "leaked_per_battle", "avg_duration_ms", "buckets", "ai_insight"} {
			if _, ok := m[k]; !ok {
				t.Fatalf("缺字段 %s：%v", k, m)
			}
		}
		if num(m, "window_days") != 30 {
			t.Fatalf("window_days 必须为 30：%v", m)
		}
	}
	if num(bodyA, "battles") != 1 || num(bodyA, "shots") != 10 || num(bodyA, "hits") != 7 {
		t.Fatalf("A 只能看到自己的一行：%v", bodyA)
	}
	if num(bodyB, "battles") != 1 || num(bodyB, "shots") != 4 || num(bodyB, "hits") != 1 {
		t.Fatalf("B 只能看到自己的一行：%v", bodyB)
	}
	if _, ok := bodyA["ai_insight"].(map[string]any); !ok {
		t.Fatalf("ai_insight 必须是对象：%v", bodyA["ai_insight"])
	}
}

func TestE2ER20EmptyAndWindowValidation(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "r20empty")
	status, body := e.get(t, "/api/v1/me/battle-efficiency", tok)
	if status != fiber.StatusOK {
		t.Fatalf("缺省 window_days 应视为 30 并 200，实际 %d %v", status, body)
	}
	if num(body, "battles") != 0 || len(r20Buckets(t, body)) != 0 {
		t.Fatalf("空玩家应 battles=0 buckets=[]：%v", body)
	}
	if v, ok := body["accuracy_pct"].(float64); !ok || v != 0 {
		t.Fatalf("空态 accuracy_pct 应归零：%v", body)
	}
	ai, ok := body["ai_insight"].(map[string]any)
	if !ok || len(ai) != 0 {
		t.Fatalf("空态 ai_insight 必须空对象：%v", body["ai_insight"])
	}
	status, _ = e.get(t, "/api/v1/me/battle-efficiency?window_days=7", tok)
	if status != fiber.StatusBadRequest {
		t.Fatalf("window_days=7 应 400，实际 %d", status)
	}
	status, _ = e.get(t, "/api/v1/me/battle-efficiency?window_days=30&window_days=30", tok)
	if status != fiber.StatusBadRequest {
		t.Fatalf("重复 window_days 应 400，实际 %d", status)
	}
}

func TestE2ER20DailyBucketsSortedAndAdminParity(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "r20parity")
	_, adminTok := e.newAdmin(t, "r20parityadmin")
	day1 := time.Now().UTC().AddDate(0, 0, -2)
	day2 := time.Now().UTC().AddDate(0, 0, -1)
	r20InsertBattle(t, e, uid, day2, 4, 1, 20, 1, 30000)
	r20InsertBattle(t, e, uid, day1, 10, 7, 60, 2, 120000)

	status, meBody := e.get(t, "/api/v1/me/battle-efficiency?window_days=30", tok)
	if status != fiber.StatusOK {
		t.Fatalf("me 应 200，实际 %d %v", status, meBody)
	}
	status, admBody := e.get(t, "/api/v1/admin/battle-efficiency", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("admin 应 200，实际 %d %v", status, admBody)
	}
	b := r20Buckets(t, meBody)
	if len(b) != 2 {
		t.Fatalf("应有 2 个日桶：%v", meBody["buckets"])
	}
	prev := ""
	for _, raw := range b {
		row, ok := raw.(map[string]any)
		if !ok {
			t.Fatalf("桶必须是对象：%v", raw)
		}
		bucket, _ := row["bucket"].(string)
		if bucket <= prev {
			t.Fatalf("桶必须按日升序：%q 之后 %q", prev, bucket)
		}
		prev = bucket
		for _, k := range []string{"bucket", "battles", "shots", "hits", "accuracy_pct"} {
			if _, ok := row[k]; !ok {
				t.Fatalf("桶缺字段 %s：%v", k, row)
			}
		}
	}
	for _, k := range []string{"battles", "shots", "hits", "accuracy_pct", "avg_heat_max", "leaked_per_battle", "avg_duration_ms"} {
		if meBody[k] != admBody[k] {
			t.Fatalf("admin 与 me 同口径字段 %s 不同：%v vs %v", k, meBody[k], admBody[k])
		}
	}
}
