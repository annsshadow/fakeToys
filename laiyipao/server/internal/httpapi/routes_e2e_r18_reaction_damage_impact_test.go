// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R18 后端聚合端点 E2E：真实 GET /admin/reaction-damage-impact。
// 断言五字段、窗口标注、旧战报排除数，以及 count 来自物化视图而非 events。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2ER18ReactionDamageImpactShape(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newAdmin(t, "r18shape")
	status, body := e.get(t, "/api/v1/admin/reaction-damage-impact", tok)
	if status != fiber.StatusOK {
		t.Fatalf("聚合端点应 200，实际 %d：%v", status, body)
	}
	if body["window_days"] != float64(30) {
		t.Fatalf("window_days 应为 30，实际 %v", body["window_days"])
	}
	items, ok := body["items"].([]any)
	if !ok {
		t.Fatalf("items 必须是数组（空也非 null）：%v", body["items"])
	}
	if len(items) != 0 {
		t.Fatalf("空库 items 应为空数组，实际 %d 条", len(items))
	}
	if _, ok := body["excluded_old_battles"]; !ok {
		t.Fatalf("缺 excluded_old_battles：%v", body)
	}
	if _, ok := body["window_from"]; !ok {
		t.Fatalf("缺 window_from：%v", body)
	}
}

func TestE2ER18ReactionDamageImpactAggregates(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	uid, _ := e.newPlayer(t, "r18agg")
	_, adminTok := e.newAdmin(t, "r18aggadmin")
	ctx := context.Background()
	// 一条带时间线的战报：overheat 两段伤害 30+20；reactions_used 供触发器写权威次数 5。
	var battleID int64
	if err := e.pool.QueryRow(ctx, `
		INSERT INTO battle_records (user_id, level_id, result, reactions_used, reaction_timeline, replay_hash)
		 VALUES ($1, 1, 'win', '{"overheat":5}'::jsonb,
		          '{"schema_version":"reaction-timeline-v1","events":[{"wave":1,"tick":1,"reaction":"overheat","enemy_uid":1,"damage":30},{"wave":1,"tick":2,"reaction":"overheat","enemy_uid":2,"damage":20}],"truncated":false,"total":2}'::jsonb,
		          'r18hash') RETURNING id`, uid).Scan(&battleID); err != nil {
		t.Fatalf("建时间线战报失败：%v", err)
	}
	_ = battleID
	// 一条旧战报（无时间线，走默认值 []），验证 excluded_old_battles 如实计数。
	if _, err := e.pool.Exec(ctx, `
		INSERT INTO battle_records (user_id, level_id, result, replay_hash)
		 VALUES ($1, 1, 'win', 'r18old'))`, uid); err != nil {
		t.Fatalf("建旧战报失败：%v", err)
	}
	status, body := e.get(t, "/api/v1/admin/reaction-damage-impact", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("聚合端点应 200，实际 %d：%v", status, body)
	}
	if body["excluded_old_battles"] != float64(1) {
		t.Fatalf("应排除 1 条无时间线旧战报，实际 %v", body["excluded_old_battles"])
	}
	items, _ := body["items"].([]any)
	if len(items) != 1 {
		t.Fatalf("应有 1 行聚合结果，实际 %d：%v", len(items), body)
	}
	row, _ := items[0].(map[string]any)
	if row["reaction"] != "overheat" || row["total_damage"] != float64(50) || row["events"] != float64(2) || row["count"] != float64(5) {
		t.Fatalf("聚合/权威次数不符（count 不得用 events 冒充）：%v", row)
	}
	if _, ok := row["damage_share_pct"]; !ok {
		t.Fatalf("缺 damage_share_pct：%v", row)
	}
	_ = battleID
}
