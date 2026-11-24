// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R21 后端 E2E：GET /api/v1/me/level-coach
// 严格 user_id 隔离；window_days 只接受 30；连续失败阈值与空态诚实。

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func r21InsertBattle(t *testing.T, e *e2e, uid int64, level int, result string, wave int, createdAt time.Time) {
	t.Helper()
	_, err := e.pool.Exec(context.Background(), `
		INSERT INTO battle_records (user_id, level_id, result, wave_reached, created_at)
		VALUES ($1, $2, $3, $4, $5)`, uid, level, result, wave, createdAt)
	if err != nil {
		t.Fatalf("建 R21 战报失败：%v", err)
	}
}

func TestE2ER21MeLevelCoachIsolation(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	uidA, tokA := e.newPlayer(t, "r21a")
	uidB, tokB := e.newPlayer(t, "r21b")
	now := time.Now().UTC()
	r21InsertBattle(t, e, uidA, 1, "lose", 3, now.Add(-3*time.Hour))
	r21InsertBattle(t, e, uidA, 1, "lose", 4, now.Add(-2*time.Hour))
	r21InsertBattle(t, e, uidB, 9, "win", 8, now.Add(-time.Hour))

	status, bodyA := e.get(t, "/api/v1/me/level-coach", tokA)
	if status != fiber.StatusOK {
		t.Fatalf("A 应 200，实际 %d %v", status, bodyA)
	}
	status, bodyB := e.get(t, "/api/v1/me/level-coach", tokB)
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
	if itA["level_id"] != float64(1) || num(map[string]any{"v": itA["attempts"]}, "v") != 2 || itA["stuck"] != true {
		t.Fatalf("A 数据不符（应只有自己的连续失败关卡1）：%v", itA)
	}
	itemsB, ok := bodyB["items"].([]any)
	if !ok || len(itemsB) != 1 {
		t.Fatalf("B 应只有 1 行：%v", bodyB)
	}
	itB, _ := itemsB[0].(map[string]any)
	if itB["level_id"] != float64(9) || itB["stuck"] != false {
		t.Fatalf("B 数据不符（应只有自己的关卡9）：%v", itB)
	}
}

func TestE2ER21MeLevelCoachWindowParam(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "r21param")
	for _, url := range []string{
		"/api/v1/me/level-coach?window_days=7",
		"/api/v1/me/level-coach?window_days=30&window_days=30",
	} {
		status, body := e.get(t, url, tok)
		if status != fiber.StatusBadRequest {
			t.Fatalf("%s 应 400，实际 %d %v", url, status, body)
		}
	}
	status, body := e.get(t, "/api/v1/me/level-coach?window_days=30", tok)
	if status != fiber.StatusOK {
		t.Fatalf("window_days=30 应 200，实际 %d %v", status, body)
	}
}

func TestE2ER21MeLevelCoachEmpty(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "r21empty")
	status, body := e.get(t, "/api/v1/me/level-coach", tok)
	if status != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d %v", status, body)
	}
	items, ok := body["items"].([]any)
	if !ok || len(items) != 0 {
		t.Fatalf("新玩家 items 应为 []：%v", body)
	}
	if fmt.Sprint(body["window_from"]) == "" || fmt.Sprint(body["window_to"]) == "" {
		t.Fatalf("空态也应有窗口标注：%v", body)
	}
}
