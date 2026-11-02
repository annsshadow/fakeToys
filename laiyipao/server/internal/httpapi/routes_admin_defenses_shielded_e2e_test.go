// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 后台防线列表按护盾状态筛选的端到端测试（2026-11-01 新一轮第 100 轮）。

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminDefensesShieldedFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "defshield")
	uidA, _ := e.newPlayer(t, "defshielda")
	uidB, _ := e.newPlayer(t, "defshieldb")
	now := time.Now()

	// 两条防线（每人一条）：护盾有效 / 无护盾
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '护盾防线', 8000, '{}'::jsonb, '{}'), ($2, '无护盾防线', 7000, '{}'::jsonb, '{}')`,
		uidA, uidB); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}
	var activeID int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT id FROM defenses WHERE name='护盾防线'`).Scan(&activeID); err != nil {
		t.Fatalf("回读防线失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`UPDATE defenses SET shielded_until = $2 WHERE id = $1`,
		activeID, now.Add(24*time.Hour)); err != nil {
		t.Fatalf("设置护盾失败：%v", err)
	}

	// 无筛选 → 2 条
	status, body := e.get(t, "/api/v1/admin/defenses?limit=200", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 2 {
		t.Fatalf("无筛选应 2 条，实际 %v", total)
	}

	// shielded=1 → 只含护盾防线
	status, body = e.get(t, "/api/v1/admin/defenses?limit=200&shielded=1", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("shielded=1 应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 1 {
		t.Errorf("shielded=1 应 1 条，实际 %v", total)
	}
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["name"] == "无护盾防线" {
			t.Error("shielded=1 不应包含无护盾防线")
		}
	}

	// shielded=0 → 只含无护盾防线
	status, body = e.get(t, "/api/v1/admin/defenses?limit=200&shielded=0", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("shielded=0 应 200，实际 %d %v", status, body)
	}
	if total := body["total"].(float64); total != 1 {
		t.Errorf("shielded=0 应 1 条，实际 %v", total)
	}
	for _, it := range body["items"].([]any) {
		m := it.(map[string]any)
		if m["name"] == "护盾防线" {
			t.Error("shielded=0 不应包含护盾防线")
		}
	}
}
