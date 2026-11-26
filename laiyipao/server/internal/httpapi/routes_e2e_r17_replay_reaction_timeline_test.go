// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R17 后端 E2E：GET /api/v1/battle/:id/replay 必须下发 reaction_timeline。
// 权威 replay_hash 不变，新增字段只是辅助对照线索；
// 旧行 / 结构异常走 schema_version=reaction-timeline-v1 + events=[] 诚实空态。

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/testutil/scratchdb"
)

// skipIfNoAdminDB 沿用 scratchdb 的环境约定：管理库不可达时跳过 DB 用例，
// 而不是把「本机没装 PG」报成测试失败；有库时绝不放过真失败。
func skipIfNoAdminDB(t *testing.T) {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()
	p, err := pgxpool.New(ctx, scratchdb.AdminDSN())
	if err != nil {
		t.Skipf("无可用管理库，跳过 DB 用例：%v", err)
	}
	defer p.Close()
	if err := p.Ping(ctx); err != nil {
		t.Skipf("管理库不可达，跳过 DB 用例：%v", err)
	}
}

func TestE2ER17ReplayIncludesReactionTimeline(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "r17replay")
	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusOK {
		t.Fatalf("开局应成功，实际 %d %v", status, body)
	}
	tokenID := num(body, "token_id")
	events := []map[string]any{{"wave": 1, "tick": 3, "reaction": "overheat", "enemy_uid": 9, "damage": 15}}
	battleID := r15SettleWithTimeline(t, e, tok, tokenID, events, "e2e_r17replay")

	status, replay := e.get(t, "/api/v1/battle/"+fmt.Sprint(battleID)+"/replay", tok)
	if status != fiber.StatusOK {
		t.Fatalf("replay 应 200，实际 %d %v", status, replay)
	}
	tl, ok := replay["reaction_timeline"].(map[string]any)
	if !ok {
		t.Fatalf("replay 响应缺 reaction_timeline 字段：%v", replay)
	}
	if tl["schema_version"] != "reaction-timeline-v1" {
		t.Fatalf("schema_version 不符：%v", tl)
	}
	arr, ok := tl["events"].([]any)
	if !ok || len(arr) != 1 {
		t.Fatalf("replay 应回带 1 条落库事件：%v", tl)
	}
	ev, _ := arr[0].(map[string]any)
	if ev["reaction"] != "overheat" || ev["enemy_uid"] != float64(9) || ev["damage"] != float64(15) {
		t.Fatalf("事件字段回带不一致：%v", ev)
	}
	// 权威 hash 字段仍在，不能被新字段挤掉。
	if _, ok := replay["expected_hash"].(string); !ok {
		t.Fatalf("replay 缺 expected_hash：%v", replay)
	}
}

func TestE2ER17ReplayReactionTimelineHonestEmpty(t *testing.T) {
	skipIfNoAdminDB(t)
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "r17empty")
	var battleID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, replay_hash, reaction_timeline)`+
			` VALUES ($1, 1, 'win', 'h', '[{"wave":1}]'::jsonb) RETURNING id`, uid).Scan(&battleID); err != nil {
		t.Fatalf("建旧战报失败：%v", err)
	}
	// 结构异常的 timeline 必须走诚实空态，不能让 replay 端点报错。
	status, replay := e.get(t, "/api/v1/battle/"+fmt.Sprint(battleID)+"/replay", tok)
	if status != fiber.StatusOK {
		t.Fatalf("replay 应 200，实际 %d %v", status, replay)
	}
	tl, ok := replay["reaction_timeline"].(map[string]any)
	if !ok {
		t.Fatalf("replay 缺 reaction_timeline 空态：%v", replay)
	}
	if tl["schema_version"] != "reaction-timeline-v1" {
		t.Fatalf("空态 schema_version 不符：%v", tl)
	}
	if arr, _ := tl["events"].([]any); len(arr) != 0 {
		t.Fatalf("结构异常应诚实空数组：%v", tl["events"])
	}
}
