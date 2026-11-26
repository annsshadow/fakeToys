// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R18 运营看板反应伤害贡献聚合（辅助分析维度）。
// 权威次数仍来自 dashboard_reaction_counts（stats.go 的 Dashboard.ReactionUsage），
// 本文件只用 R15 已落库的 reaction_timeline.events.damage 补充「打出多少伤害」；
// 不改结算、不改 score/loot/星级、不参与 replay_hash。

import (
	"context"
	"math"
	"sort"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

const reactionDamageWindowDays = 30

type ReactionDamageImpactItem struct {
	Reaction       string  `json:"reaction"`
	TotalDamage    int64   `json:"total_damage"`
	DamageSharePct float64 `json:"damage_share_pct"`
	Events         int64   `json:"events"`
	Count          int64   `json:"count"`
}

type ReactionDamageImpactResp struct {
	WindowDays         int                        `json:"window_days"`
	WindowFrom         string                     `json:"window_from"`
	WindowTo           string                     `json:"window_to"`
	ExcludedOldBattles int                        `json:"excluded_old_battles"`
	Items              []ReactionDamageImpactItem `json:"items"`
}

// AggregateReactionDamageImpact 把已展开的逐次事件按反应 key 聚合。
// authCounts 是 dashboard_reaction_counts 的权威次数（不能用 events 冒充）。
func AggregateReactionDamageImpact(events []domain.ReactionTimelineEvent, authCounts map[string]int64, excluded int, now time.Time) ReactionDamageImpactResp {
	valid := validReactionKeys()
	type acc struct{ damage, events int64 }
	agg := map[string]*acc{}
	for _, ev := range events {
		if !valid[ev.Reaction] {
			continue
		}
		a := agg[ev.Reaction]
		if a == nil {
			a = &acc{}
			agg[ev.Reaction] = a
		}
		a.damage += int64(ev.Damage)
		a.events++
	}
	items := make([]ReactionDamageImpactItem, 0, len(agg))
	for key, a := range agg {
		items = append(items, ReactionDamageImpactItem{Reaction: key, TotalDamage: a.damage, Events: a.events, Count: authCounts[key]})
	}
	return finalizeReactionDamageImpact(items, excluded, now)
}

func validReactionKeys() map[string]bool {
	m := make(map[string]bool)
	for _, spec := range domain.AllReactionSpecs() {
		m[string(spec.Key)] = true
	}
	return m
}

// finalizeReactionDamageImpact 统一算占比、排序、窗口与空态。
func finalizeReactionDamageImpact(items []ReactionDamageImpactItem, excluded int, now time.Time) ReactionDamageImpactResp {
	var total int64
	for _, it := range items {
		if it.TotalDamage > 0 {
			total += it.TotalDamage
		}
	}
	for i := range items {
		if total == 0 {
			items[i].DamageSharePct = 0
		} else {
			items[i].DamageSharePct = math.Round(float64(items[i].TotalDamage)/float64(total)*1000) / 10
		}
	}
	sort.Slice(items, func(i, j int) bool {
		if items[i].DamageSharePct != items[j].DamageSharePct {
			return items[i].DamageSharePct > items[j].DamageSharePct
		}
		return items[i].Reaction < items[j].Reaction
	})
	if items == nil {
		items = []ReactionDamageImpactItem{}
	}
	u := now.UTC()
	return ReactionDamageImpactResp{
		WindowDays:         reactionDamageWindowDays,
		WindowFrom:         u.AddDate(0, 0, -reactionDamageWindowDays).Format("2006-01-02"),
		WindowTo:           u.Format("2006-01-02"),
		ExcludedOldBattles: excluded,
		Items:              items,
	}
}

// AdminReactionDamageImpact 从 battle_records.reaction_timeline 展开事件，聚合近 30 天真实伤害。
// 口径：先按 created_at 窗口过滤（走 idx_battle_records_created 索引），再展开 JSONB；
// 无时间线/events 为空的窗口内旧战报计入 excluded_old_battles，如实标注不假装全量。
func (s *Service) AdminReactionDamageImpact(ctx context.Context) (ReactionDamageImpactResp, error) {
	now := time.Now()
	u := now.UTC()
	from := time.Date(u.Year(), u.Month(), u.Day(), 0, 0, 0, 0, time.UTC).AddDate(0, 0, -reactionDamageWindowDays)
	to := time.Date(u.Year(), u.Month(), u.Day(), 0, 0, 0, 0, time.UTC).AddDate(0, 0, 1)
	keys := make([]string, 0)
	valid := validReactionKeys()
	for _, spec := range domain.AllReactionSpecs() {
		if valid[string(spec.Key)] {
			keys = append(keys, string(spec.Key))
		}
	}
	rows, err := s.pool.Query(ctx, `
		SELECT e->>'reaction' AS reaction,
		       SUM((e->>'damage')::bigint) AS total_damage,
		       COUNT(*) AS events
		  FROM battle_records br,
		       jsonb_array_elements(br.reaction_timeline->'events') AS e
		 WHERE br.created_at >= $1 AND br.created_at < $2
		   AND br.reaction_timeline->>'schema_version' = $4
		   AND e->>'reaction' = ANY($3)
		 GROUP BY e->>'reaction'`, from, to, keys, domain.ReactionTimelineSchemaVersion)
	if err != nil {
		return ReactionDamageImpactResp{}, err
	}
	type aggRow struct {
		key            string
		damage, events int64
	}
	var aggs []aggRow
	for rows.Next() {
		var a aggRow
		if err := rows.Scan(&a.key, &a.damage, &a.events); err != nil {
			rows.Close()
			return ReactionDamageImpactResp{}, err
		}
		aggs = append(aggs, a)
	}
	if err := rows.Err(); err != nil {
		rows.Close()
		return ReactionDamageImpactResp{}, err
	}
	rows.Close()
	// 权威次数：与 Dashboard.ReactionUsage 同源（物化视图，非时间线事件数）。
	countRows, err := s.pool.Query(ctx, `SELECT reaction, SUM(count) FROM dashboard_reaction_counts GROUP BY reaction`)
	if err != nil {
		return ReactionDamageImpactResp{}, err
	}
	authCounts := map[string]int64{}
	for countRows.Next() {
		var k string
		var c int64
		if err := countRows.Scan(&k, &c); err != nil {
			countRows.Close()
			return ReactionDamageImpactResp{}, err
		}
		authCounts[k] = c
	}
	if err := countRows.Err(); err != nil {
		countRows.Close()
		return ReactionDamageImpactResp{}, err
	}
	countRows.Close()
	var excluded int
	if err := s.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM battle_records
		 WHERE created_at >= $1 AND created_at < $2
		   AND (reaction_timeline IS NULL
		        OR reaction_timeline->>'schema_version' IS DISTINCT FROM $3
		        OR reaction_timeline->'events' IS NULL
		        OR jsonb_array_length(reaction_timeline->'events') = 0)`, from, to, domain.ReactionTimelineSchemaVersion).Scan(&excluded); err != nil {
		return ReactionDamageImpactResp{}, err
	}
	items := make([]ReactionDamageImpactItem, 0, len(aggs))
	for _, a := range aggs {
		items = append(items, ReactionDamageImpactItem{Reaction: a.key, TotalDamage: a.damage, Events: a.events, Count: authCounts[a.key]})
	}
	return finalizeReactionDamageImpact(items, excluded, now), nil
}
