// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

// R15 逐次反应时间线：仅用于战报回顾展示，服务端归一化后落库。
// 红线：不参与结算裁决、score/loot/星级、replay_hash；不该被当作权威结算输入。
// 依据 SPEC round15-reaction-timeline：上限 200 条尾部截断（保留最早 200），
// 未知反应 key 丢弃；非法字段拒绝并 fail loud，不静默写库。

import (
	"encoding/json"
	"fmt"
)

const (
	// ReactionTimelineSchemaVersion 当前唯一支持的展示结构版本。
	ReactionTimelineSchemaVersion = "reaction-timeline-v1"
	// ReactionTimelineMaxEvents 单条战报保留的事件上限（10MB 硬红线下的展示预算）。
	ReactionTimelineMaxEvents = 200
	// reactionTimelineMaxReactionLen 单个 reaction key 的长度上限，防滥用。
	reactionTimelineMaxReactionLen = 64
)

// ReactionTimelineEvent 单条逐次反应归因（展示性数据，非权威结算字段）。
type ReactionTimelineEvent struct {
	Wave     int    `json:"wave"`
	Tick     int    `json:"tick"`
	Reaction string `json:"reaction"`
	EnemyUID int    `json:"enemy_uid"`
	Damage   int    `json:"damage"`
}

// ReactionTimeline 外层包装：total 是归一化前的原始事件总数，
// truncated=true 表示原始数超过 Retention 上限而被尾部截断。
type ReactionTimeline struct {
	SchemaVersion string                  `json:"schema_version"`
	Events        []ReactionTimelineEvent `json:"events"`
	Truncated     bool                    `json:"truncated"`
	Total         int                     `json:"total"`
}

// UnmarshalJSON 兼容两种合法存储形状：
//   - {"schema_version":...,"events":[...],...}：R15 新结算写入的包装对象；
//   - []：迁移前旧行 / NULL 归一化后的诚实空态（不编造事件）。
//
// 其他形状（坏 jsonb）一律报错，由调用方 fail loud。
func (t *ReactionTimeline) UnmarshalJSON(data []byte) error {
	var empty []json.RawMessage
	if err := json.Unmarshal(data, &empty); err == nil {
		if len(empty) != 0 {
			return fmt.Errorf("reaction_timeline 不能是非空数组")
		}
		t.SchemaVersion = ReactionTimelineSchemaVersion
		t.Events = []ReactionTimelineEvent{}
		t.Truncated = false
		t.Total = 0
		return nil
	}
	type plain ReactionTimeline
	var v plain
	if err := json.Unmarshal(data, &v); err != nil {
		return err
	}
	*t = ReactionTimeline(v)
	if t.Events == nil {
		t.Events = []ReactionTimelineEvent{}
	}
	return nil
}

// NormalizeReactionTimeline 对上报的时间线做展示层归一化：
//   - 严格校验每个事件的字段范围与 reaction key 合法性，非法即返回错误（fail loud）；
//   - 未知 reaction key 直接丢弃（不写库、不污染 reactions_used 运营维度）；
//   - 超过 ReactionTimelineMaxEvents 时保留最早 200 条，并标记 truncated/total。
//
// 返回的 ReactionTimeline 永远非 nil，events 永远是非 nil 空数组（旧行/空态诚实为空）。
func NormalizeReactionTimeline(in []ReactionTimelineEvent) (ReactionTimeline, error) {
	out := ReactionTimeline{
		SchemaVersion: ReactionTimelineSchemaVersion,
		Events:        make([]ReactionTimelineEvent, 0, len(in)),
		Total:         len(in),
		Truncated:     len(in) > ReactionTimelineMaxEvents,
	}
	for i, ev := range in {
		if err := validateReactionTimelineEvent(ev); err != nil {
			return ReactionTimeline{}, fmt.Errorf("reaction_timeline[%d]: %w", i, err)
		}
		if !knownReactions[ev.Reaction] {
			// 未知 key 是历史客户端/恶意上报，丢弃但不影响其余合法事件。
			continue
		}
		if len(out.Events) < ReactionTimelineMaxEvents {
			out.Events = append(out.Events, ev)
		}
	}
	return out, nil
}

func validateReactionTimelineEvent(ev ReactionTimelineEvent) error {
	switch {
	case ev.Wave < 1:
		return fmt.Errorf("wave 必须 >=1，实际 %d", ev.Wave)
	case ev.Tick < 0:
		return fmt.Errorf("tick 必须 >=0，实际 %d", ev.Tick)
	case ev.Reaction == "":
		return fmt.Errorf("reaction 不能为空")
	case len(ev.Reaction) > reactionTimelineMaxReactionLen:
		return fmt.Errorf("reaction 长度不能超过 %d", reactionTimelineMaxReactionLen)
	case ev.EnemyUID < 1:
		return fmt.Errorf("enemy_uid 必须 >=1，实际 %d", ev.EnemyUID)
	case ev.Damage < 0:
		return fmt.Errorf("damage 必须 >=0，实际 %d", ev.Damage)
	}
	return nil
}
