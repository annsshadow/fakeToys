// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R17 验真链路 reaction_timeline 下发：backend test-first。
// 契约：GET /api/v1/battle/:id/replay 的 ReplayInfo 必须带 reaction_timeline；
// 旧行 / NULL / 结构异常一律 schema_version=reaction-timeline-v1 + events=[] 诚实空态，
// 不报错、不编造事件；权威 replay_hash 与验真 SQL 不受影响。

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

func TestParseReplayReactionTimelineHonestEmpty(t *testing.T) {
	cases := []struct {
		name string
		raw  []byte
	}{
		{"列缺失或 NULL", nil},
		{"空字节", []byte("")},
		{"JSON null", []byte("null")},
		{"旧行空数组", []byte(`[]`)},
		{"坏 JSON", []byte(`{oops`)},
		{"非空数组", []byte(`[{"wave":1}]`)},
		{"events 类型非法", []byte(`{"schema_version":"reaction-timeline-v1","events":"bad"}`)},
		{"schema 不符", []byte(`{"schema_version":"unknown","events":[{"wave":1,"tick":2,"reaction":"overheat","enemy_uid":7,"damage":12}]}`)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := parseReplayReactionTimeline(tc.raw)
			if got.SchemaVersion != domain.ReactionTimelineSchemaVersion {
				t.Fatalf("schema_version = %q，期望 %q", got.SchemaVersion, domain.ReactionTimelineSchemaVersion)
			}
			if got.Events == nil || len(got.Events) != 0 {
				t.Fatalf("events 必须是非 nil 空数组，实际 %#v", got.Events)
			}
			if got.Truncated || got.Total != 0 {
				t.Fatalf("空态 truncated/total 应为 false/0，实际 %v/%d", got.Truncated, got.Total)
			}
		})
	}
}

func TestParseReplayReactionTimelineKeepsValidData(t *testing.T) {
	raw := []byte(`{"schema_version":"reaction-timeline-v1","events":[{"wave":1,"tick":2,"reaction":"overheat","enemy_uid":7,"damage":12}],"truncated":true,"total":205}`)
	got := parseReplayReactionTimeline(raw)
	if got.SchemaVersion != domain.ReactionTimelineSchemaVersion {
		t.Fatalf("schema_version = %q", got.SchemaVersion)
	}
	if len(got.Events) != 1 {
		t.Fatalf("events 长度应为 1，实际 %d", len(got.Events))
	}
	ev := got.Events[0]
	if ev.Wave != 1 || ev.Tick != 2 || ev.Reaction != "overheat" || ev.EnemyUID != 7 || ev.Damage != 12 {
		t.Fatalf("事件五字段读回不一致：%+v", ev)
	}
	if !got.Truncated || got.Total != 205 {
		t.Fatalf("truncated/total 应为 true/205，实际 %v/%d", got.Truncated, got.Total)
	}
}

// 以下两条需要数据库；无库时 openTestService 自动 Skip（本机未装 PG）。
func TestGetReplayIncludesReactionTimeline(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)
	timeline := domain.ReactionTimeline{
		SchemaVersion: domain.ReactionTimelineSchemaVersion,
		Events:        []domain.ReactionTimelineEvent{{Wave: 1, Tick: 2, Reaction: "overheat", EnemyUID: 7, Damage: 12}},
		Total:         1,
	}
	raw, err := json.Marshal(timeline)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := ts.db.Pool.Exec(ctx, "UPDATE battle_records SET reaction_timeline = $1::jsonb WHERE id = $2", string(raw), battleID); err != nil {
		t.Fatalf("写入 timeline 失败：%v", err)
	}
	info, err := ts.GetReplay(ctx, owner, battleID)
	if err != nil {
		t.Fatalf("读 replay 失败：%v", err)
	}
	if info.ReactionTimeline.SchemaVersion != domain.ReactionTimelineSchemaVersion {
		t.Fatalf("schema_version 不符：%q", info.ReactionTimeline.SchemaVersion)
	}
	if len(info.ReactionTimeline.Events) != 1 || info.ReactionTimeline.Events[0].Reaction != "overheat" {
		t.Fatalf("replay 未带回落库 timeline：%+v", info.ReactionTimeline)
	}
	// GET /battle/:id/replay 的 handler 直接 c.JSON(info)，字段必须出现在响应 JSON 里。
	body, err := json.Marshal(info)
	if err != nil {
		t.Fatal(err)
	}
	var shape map[string]json.RawMessage
	if err := json.Unmarshal(body, &shape); err != nil {
		t.Fatal(err)
	}
	if _, ok := shape["reaction_timeline"]; !ok {
		t.Fatalf("replay 响应缺 reaction_timeline 字段：%s", body)
	}
}

func TestGetReplayReactionTimelineHonestEmpty(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)
	for _, raw := range []string{`[]`, `[{"wave":1}]`, `{"schema_version":"unknown","events":[]}`} {
		if _, err := ts.db.Pool.Exec(ctx, "UPDATE battle_records SET reaction_timeline = $1::jsonb WHERE id = $2", raw, battleID); err != nil {
			t.Fatalf("写入异常 timeline 失败：%v", err)
		}
		info, err := ts.GetReplay(ctx, owner, battleID)
		if err != nil {
			t.Fatalf("结构异常不应让 replay 报错（raw=%s）：%v", raw, err)
		}
		if info.ReactionTimeline.SchemaVersion != domain.ReactionTimelineSchemaVersion || info.ReactionTimeline.Events == nil || len(info.ReactionTimeline.Events) != 0 {
			t.Fatalf("结构异常应给诚实空态（raw=%s）：%+v", raw, info.ReactionTimeline)
		}
	}
}
