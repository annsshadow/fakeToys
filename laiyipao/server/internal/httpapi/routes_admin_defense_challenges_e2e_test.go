package httpapi

// 后台防线挑战记录列表的端到端测试（2026-10-31 新一轮第 30 轮）。

import (
	"context"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminDefenseChallenges(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "dc_f")
	ownerID, _ := e.newPlayer(t, "dc_f_owner")
	challengerID, _ := e.newPlayer(t, "dc_f_challenger")
	now := time.Now()

	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '铁壁', 100, '{}'::jsonb, 's1')`, ownerID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms, hp_left_pct, loot, settled, created_at)
		 VALUES ((SELECT id FROM defenses WHERE owner_id = $1), $2, 1, TRUE, 1500, 90, '{"coin":100}'::jsonb, TRUE, $3)`,
		ownerID, challengerID, now.Add(-time.Hour)); err != nil {
		t.Fatalf("插入挑战记录失败：%v", err)
	}

	// 无筛选 → total=1
	status, body := e.get(t, "/api/v1/admin/defense-challenges?limit=50", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok || totalAll != 1 {
		t.Fatalf("无筛选应 1 条，实际 %v", body["total"])
	}
	items, _ := body["items"].([]any)
	first := items[0].(map[string]any)
	if first["defense_name"] != "铁壁" || first["challenger_nickname"] == "" || first["challenger_nickname"] == nil {
		t.Errorf("记录字段不完整：%v", first)
	}
	if first["won"] != true {
		t.Errorf("won 应为 true，实际 %v", first["won"])
	}
	loot, ok := first["loot"].(map[string]any)
	if !ok || loot["coin"] != float64(100) {
		t.Errorf("loot 应含 coin:100，实际 %v", first["loot"])
	}

	// 按挑战者过滤 → 1 条
	status, body = e.get(t, "/api/v1/admin/defense-challenges?limit=50&challenger_id="+fmtInt(challengerID), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("挑战者过滤应 200，实际 %d %v", status, body)
	}
	totalCh, ok := body["total"].(float64)
	if !ok || totalCh != 1 {
		t.Errorf("challenger 过滤应 1 条，实际 %v", body["total"])
	}

	// 非法日期 → 400
	status, body = e.get(t, "/api/v1/admin/defense-challenges?date_from=not-a-date", adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("非法 date_from 应 400，实际 %d %v", status, body)
	}

	// 未认证 → 401
	status, _ = e.get(t, "/api/v1/admin/defense-challenges", "")
	if status != fiber.StatusUnauthorized {
		t.Errorf("未认证应 401，实际 %d", status)
	}
}