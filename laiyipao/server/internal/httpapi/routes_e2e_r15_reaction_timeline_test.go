// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R15 战报反应时间线 E2E（test-first）：
//   - 真结算带 timeline 落库，并从 /me/battles/:id 读回一致；
//   - 超 200 条尾部截断，total=205/truncated=true、events=200；
//   - 旧战报（无该列/旧行）读回诚实空态 []，不编造事件。

import (
	"context"
	"encoding/json"
	"fmt"
	"testing"

	"github.com/gofiber/fiber/v2"
)

const r15TimelineJSON = `{"schema_version":"reaction-timeline-v1","events":[{"wave":1,"tick":2,"reaction":"overheat","enemy_uid":7,"damage":12}],"truncated":false,"total":1}`

func r15SettleWithTimeline(t *testing.T, e *e2e, tok string, tokenID int64, events []map[string]any, tag string) int64 {
	t.Helper()
	settle := e.settleBody(t, tokenID)
	// reactions_used 必须与 timeline 的合法事件同口径（不参与裁决，但保持既有集合校验通过）。
	settle["reactions"] = len(events)
	settle["reactions_used"] = map[string]int{"overheat": len(events)}
	settle["reaction_timeline"] = map[string]any{
		"schema_version": "reaction-timeline-v1",
		"events":         events,
		"truncated":      false,
		"total":          len(events),
	}
	status, resp := e.post(t, "/api/v1/battle/settle", tok, settle)
	if status != fiber.StatusOK {
		t.Fatalf("%s 结算应 200，实际 %d %v", tag, status, resp)
	}
	var battleID int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT id FROM battle_records WHERE user_id = (SELECT id FROM users WHERE guest_token LIKE $1 ORDER BY id DESC LIMIT 1) ORDER BY id DESC LIMIT 1`, tag+"%").
		Scan(&battleID); err != nil {
		t.Fatalf("%s 查落库战报失败：%v", tag, err)
	}
	return battleID
}

func TestE2ER15ReactionTimelinePersistsAndReadsBack(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "r15tl")
	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusOK {
		t.Fatalf("开局应成功，实际 %d %v", status, body)
	}
	tokenID := num(body, "token_id")
	events := []map[string]any{{"wave": 1, "tick": 2, "reaction": "overheat", "enemy_uid": 7, "damage": 12}}
	battleID := r15SettleWithTimeline(t, e, tok, tokenID, events, "e2e_r15tl")

	// 库里必须真的写进 JSONB，不只是响应里有。
	var raw []byte
	if err := e.pool.QueryRow(context.Background(),
		`SELECT reaction_timeline FROM battle_records WHERE id = $1`, battleID).Scan(&raw); err != nil {
		t.Fatalf("读 reaction_timeline 失败：%v", err)
	}
	var stored map[string]any
	if err := json.Unmarshal(raw, &stored); err != nil {
		t.Fatalf("解析落库 timeline 失败：%v", err)
	}
	if stored["schema_version"] != "reaction-timeline-v1" || stored["total"] != float64(1) {
		t.Fatalf("落库外层字段不符：%v", stored)
	}
	arr, _ := stored["events"].([]any)
	if len(arr) != 1 || arr[0].(map[string]any)["reaction"] != "overheat" {
		t.Fatalf("落库事件不符：%v", stored)
	}

	// 玩家详情读回一致。
	status, detail := e.get(t, "/api/v1/me/battles/"+fmt.Sprint(battleID), tok)
	if status != fiber.StatusOK {
		t.Fatalf("战报详情应 200，实际 %d %v", status, detail)
	}
	tl, ok := detail["reaction_timeline"].(map[string]any)
	if !ok {
		t.Fatalf("详情缺 reaction_timeline 字段：%v", detail)
	}
	if tl["schema_version"] != "reaction-timeline-v1" || tl["total"] != float64(1) {
		t.Fatalf("详情 timeline 不符：%v", tl)
	}
	// uid 用到了，避免编译器/静态检查误报未使用。
	_ = uid
}

func TestE2ER15ReactionTimelineTruncatesAt200(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "r15trunc")
	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusOK {
		t.Fatalf("开局应成功，实际 %d %v", status, body)
	}
	tokenID := num(body, "token_id")
	events := make([]map[string]any, 0, 205)
	for i := 0; i < 205; i++ {
		events = append(events, map[string]any{"wave": 1, "tick": i, "reaction": "overheat", "enemy_uid": i + 1, "damage": 1})
	}
	battleID := r15SettleWithTimeline(t, e, tok, tokenID, events, "e2e_r15trunc")
	var raw []byte
	if err := e.pool.QueryRow(context.Background(),
		`SELECT reaction_timeline FROM battle_records WHERE id = $1`, battleID).Scan(&raw); err != nil {
		t.Fatalf("读 reaction_timeline 失败：%v", err)
	}
	var stored struct {
		SchemaVersion string `json:"schema_version"`
		Events        []struct {
			Tick int `json:"tick"`
		} `json:"events"`
		Truncated bool `json:"truncated"`
		Total     int  `json:"total"`
	}
	if err := json.Unmarshal(raw, &stored); err != nil {
		t.Fatal(err)
	}
	if len(stored.Events) != 200 || !stored.Truncated || stored.Total != 205 {
		t.Fatalf("应为 200/truncated/total205，实际 %d/%v/%d", len(stored.Events), stored.Truncated, stored.Total)
	}
	if stored.Events[0].Tick != 0 || stored.Events[199].Tick != 199 {
		t.Fatal("应保留最早 200 条")
	}
}

func TestE2ER15OldBattleTimelineEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "r15old")
	// 模拟旧行：显式把列写回默认值（等价于迁移后存量行 / 迁移前无该字段的语义）。
	e.exec(t, `UPDATE users SET id = id WHERE id = $1`, uid)
	var battleID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, replay_hash)
		 VALUES ($1, 1, 'win', 'h') RETURNING id`, uid).Scan(&battleID); err != nil {
		t.Fatalf("建旧战报失败：%v", err)
	}
	status, detail := e.get(t, "/api/v1/me/battles/"+fmt.Sprint(battleID), tok)
	if status != fiber.StatusOK {
		t.Fatalf("详情应 200，实际 %d %v", status, detail)
	}
	tl, ok := detail["reaction_timeline"].(map[string]any)
	if !ok {
		t.Fatalf("详情必须带 reaction_timeline 空态，实际 %v", detail)
	}
	arr, ok := tl["events"].([]any)
	if !ok || len(arr) != 0 {
		t.Fatalf("旧战报必须诚实空数组，实际 %v", tl["events"])
	}
	if tl["total"] != float64(0) || tl["truncated"] != false {
		t.Fatalf("空态 total/truncated 应为 0/false，实际 %v", tl)
	}
}
