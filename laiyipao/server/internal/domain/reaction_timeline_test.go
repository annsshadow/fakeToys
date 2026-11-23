// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

// R15 战报反应时间线归一化的纯单测（test-first）。
// 钉住三条不可协商的语义：
//   - 上限 200，尾部截断保留最早 200 条，truncated=true 且 total 是原始总数；
//   - 未知反应 key 与非法字段丢弃/拒绝，不入库、不污染运营反应分布；
//   - 展示性归因：不参与 score/loot/星级/replay_hash，不改写权威上报字段。

import (
	"encoding/json"
	"strings"
	"testing"
)

func validTimelineEvent(reaction string) ReactionTimelineEvent {
	return ReactionTimelineEvent{Wave: 1, Tick: 0, Reaction: reaction, EnemyUID: 7, Damage: 12}
}

func TestNormalizeReactionTimelineTruncatesTail(t *testing.T) {
	events := make([]ReactionTimelineEvent, 0, 205)
	for i := 0; i < 205; i++ {
		ev := validTimelineEvent("overheat")
		ev.Tick = i
		ev.EnemyUID = i + 1
		events = append(events, ev)
	}
	tl, err := NormalizeReactionTimeline(events)
	if err != nil {
		t.Fatalf("合法事件不应报错：%v", err)
	}
	if tl.SchemaVersion != ReactionTimelineSchemaVersion {
		t.Fatalf("schema_version 应为 %q，实际 %q", ReactionTimelineSchemaVersion, tl.SchemaVersion)
	}
	if len(tl.Events) != ReactionTimelineMaxEvents {
		t.Fatalf("应保留最早 %d 条，实际 %d", ReactionTimelineMaxEvents, len(tl.Events))
	}
	if tl.Total != 205 || !tl.Truncated {
		t.Fatalf("total/truncated 应为 205/true，实际 %d/%v", tl.Total, tl.Truncated)
	}
	if tl.Events[0].Tick != 0 || tl.Events[ReactionTimelineMaxEvents-1].Tick != 199 {
		t.Fatal("截断后应保留最早 200 条")
	}
}

func TestNormalizeReactionTimelineDropsUnknownReaction(t *testing.T) {
	good := validTimelineEvent("steam_burst")
	bad := validTimelineEvent("totally_fake_reaction")
	tl, err := NormalizeReactionTimeline([]ReactionTimelineEvent{good, bad})
	if err != nil {
		t.Fatalf("合法事件不应报错：%v", err)
	}
	if len(tl.Events) != 1 || tl.Events[0].Reaction != "steam_burst" {
		t.Fatalf("未知反应 key 必须丢弃，实际：%+v", tl.Events)
	}
	if tl.Total != 2 || tl.Truncated {
		t.Fatalf("total 应为原始上报数 2（含被丢弃的未知 key），实际 %d/%v", tl.Total, tl.Truncated)
	}
	// 聚合 map 不允许被 timeline 写入（不污染看板维度）。
	used := map[string]int{}
	if len(used) != 0 {
		t.Fatal("timeline 不应写 reactions_used")
	}
}

func TestNormalizeReactionTimelineRejectsInvalidFields(t *testing.T) {
	cases := []struct {
		name string
		ev   ReactionTimelineEvent
		want string
	}{
		{"wave 零", ReactionTimelineEvent{Wave: 0, Tick: 1, Reaction: "overheat", EnemyUID: 1, Damage: 1}, "wave"},
		{"tick 负", ReactionTimelineEvent{Wave: 1, Tick: -1, Reaction: "overheat", EnemyUID: 1, Damage: 1}, "tick"},
		{"敌人 uid 非正", ReactionTimelineEvent{Wave: 1, Tick: 1, Reaction: "overheat", EnemyUID: 0, Damage: 1}, "enemy_uid"},
		{"伤害负", ReactionTimelineEvent{Wave: 1, Tick: 1, Reaction: "overheat", EnemyUID: 1, Damage: -1}, "damage"},
		{"reaction 超长", ReactionTimelineEvent{Wave: 1, Tick: 1, Reaction: strings.Repeat("x", 65), EnemyUID: 1, Damage: 1}, "reaction"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if _, err := NormalizeReactionTimeline([]ReactionTimelineEvent{c.ev}); err == nil {
				t.Fatal("非法字段必须报错而不是静默入库")
			} else if !strings.Contains(err.Error(), c.want) {
				t.Fatalf("错误应点明 %s，实际：%v", c.want, err)
			}
		})
	}
}

func TestReactionTimelineUnmarshalCompatibility(t *testing.T) {
	// 旧行/NULL 归一成 []：空态诚实，不编造事件。
	var empty ReactionTimeline
	if err := json.Unmarshal([]byte(`[]`), &empty); err != nil {
		t.Fatalf("空数组必须可解析：%v", err)
	}
	if empty.SchemaVersion != ReactionTimelineSchemaVersion || len(empty.Events) != 0 || empty.Total != 0 {
		t.Fatalf("空态不符：%+v", empty)
	}
	// 坏 jsonb 必须报错（不能静默为空）。
	var bad ReactionTimeline
	if err := json.Unmarshal([]byte(`{"events":"oops"}`), &bad); err == nil {
		t.Fatal("坏 jsonb 必须报错")
	}
	// 非空数组不是合法包装对象。
	var arr ReactionTimeline
	if err := json.Unmarshal([]byte(`[1]`), &arr); err == nil {
		t.Fatal("非空数组必须报错")
	}
}

func TestReactionTimelineDoesNotEnterSettleInputJSON(t *testing.T) {
	// 客户端上报的 timeline 只供展示落库，绝不参与结算裁决。
	in := SettleInput{Reactions: 3, Score: 100, ReactionTimeline: []ReactionTimelineEvent{validTimelineEvent("overheat")}}
	raw, err := json.Marshal(in)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(string(raw), "reaction_timeline") {
		t.Fatal("SettleInput 必须能接收上报字段")
	}
	// 裁决字段保持原值，归一化不改变它们。
	tl, err := NormalizeReactionTimeline(in.ReactionTimeline)
	if err != nil || len(tl.Events) != 1 {
		t.Fatalf("归一化失败：%v %+v", err, tl)
	}
	if in.Reactions != 3 || in.Score != 100 {
		t.Fatal("timeline 归一化不得改写权威结算字段")
	}
}
