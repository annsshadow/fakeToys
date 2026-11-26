// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R19 玩家反应伤害贡献聚合（辅助分析维度）。
// 权威触发次数 count 取 battle_records.reactions_used（结算写入，与看板同源），
// 不得用时间线 events 条数冒充；伤害 only 来自 R15 reaction_timeline.events.damage。
// 旧战报无 timeline 但 reactions_used 有次数仍计入 count、伤害不计、excluded 自增。
// 严格按当前 user_id + 30天 UTC 窗口过滤，禁止前端传 user_id。
// 不改结算、不改 score/loot/星级、不参与 replay_hash（权威仍是 replay_hash）。

import (
	"context"
	"math"
	"sort"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

type meDamageRow struct {
	Reaction    string
	TotalDamage int64
	Events      int64
}

type MeReactionDamageImpactItem struct {
	Reaction       string  `json:"reaction"`
	TotalDamage    int64   `json:"total_damage"`
	DamageSharePct float64 `json:"damage_share_pct"`
	Events         int64   `json:"events"`
	Count          int64   `json:"count"`
}

type MeReactionDamageImpactResp struct {
	WindowDays         int                          `json:"window_days"`
	WindowFrom         string                       `json:"window_from"`
	WindowTo           string                       `json:"window_to"`
	ExcludedOldBattles int                          `json:"excluded_old_battles"`
	Items              []MeReactionDamageImpactItem `json:"items"`
}

// AggregateMeReactionDamageImpact 把已按 user 聚合的伤害行与权威次数合并。
// damages 来自 timeline.events（按 reaction 分组的 SUM/COUNT），authCounts 来自 reactions_used。
func AggregateMeReactionDamageImpact(damages []meDamageRow, authCounts map[string]int64, excluded int, now time.Time) MeReactionDamageImpactResp {
	valid := validReactionKeys()
	// 先以权威次数建行：旧战报无 timeline 但次数仍展示。
	type acc struct{ damage, events int64 }
	agg := map[string]*acc{}
	ensure := func(k string) *acc {
		if a := agg[k]; a != nil {
			return a
		}
		a := &acc{}
		agg[k] = a
		return a
	}
	for k, c := range authCounts {
		if !valid[k] || c <= 0 {
			continue
		}
		ensure(k) // count 单独存，稍后赋回
	}
	for _, row := range damages {
		if !valid[row.Reaction] {
			continue
		}
		a := ensure(row.Reaction)
		a.damage += row.TotalDamage
		a.events += row.Events
	}
	items := make([]MeReactionDamageImpactItem, 0, len(agg))
	for key, a := range agg {
		items = append(items, MeReactionDamageImpactItem{
			Reaction: key, TotalDamage: a.damage, Events: a.events, Count: authCounts[key],
		})
	}
	return finalizeMeReactionDamageImpact(items, excluded, now)
}

// finalizeMeReactionDamageImpact 统一占比、排序、窗口与空态。
func finalizeMeReactionDamageImpact(items []MeReactionDamageImpactItem, excluded int, now time.Time) MeReactionDamageImpactResp {
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
		if items[i].TotalDamage != items[j].TotalDamage {
			return items[i].TotalDamage > items[j].TotalDamage
		}
		return items[i].Reaction < items[j].Reaction
	})
	if items == nil {
		items = []MeReactionDamageImpactItem{}
	}
	u := now.UTC()
	return MeReactionDamageImpactResp{
		WindowDays:         reactionDamageWindowDays,
		WindowFrom:         u.AddDate(0, 0, -reactionDamageWindowDays).Format("2006-01-02"),
		WindowTo:           u.Format("2006-01-02"),
		ExcludedOldBattles: excluded,
		Items:              items,
	}
}

// MeReactionDamageImpact 从当前用户的 battle_records 聚合近 30 天真实伤害与权威次数。
// SQL 口径：先锁 user_id + created_at 窗口（索引），再展开 reactions_used 取权威次数、
// 展开 reaction_timeline.events 取伤害；无有效 timeline 的窗口内战报计入 excluded。
func (s *Service) MeReactionDamageImpact(ctx context.Context, userID int64) (MeReactionDamageImpactResp, error) {
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
	// 伤害与事件数：必须限定 schema_version + 合法 key + user_id + 时间窗。
	damageRows, err := s.pool.Query(ctx, `
		SELECT e->>'reaction' AS reaction,
		       SUM((e->>'damage')::bigint) AS total_damage,
		       COUNT(*) AS events
		  FROM battle_records br,
		       jsonb_array_elements(br.reaction_timeline->'events') AS e
		 WHERE br.user_id = $1
		   AND br.created_at >= $2 AND br.created_at < $3
		   AND br.reaction_timeline->>'schema_version' = $5
		   AND e->>'reaction' = ANY($4)
		 GROUP BY e->>'reaction'`, userID, from, to, keys, domain.ReactionTimelineSchemaVersion)
	if err != nil {
		return MeReactionDamageImpactResp{}, err
	}
	var damages []meDamageRow
	for damageRows.Next() {
		var row meDamageRow
		if err := damageRows.Scan(&row.Reaction, &row.TotalDamage, &row.Events); err != nil {
			damageRows.Close()
			return MeReactionDamageImpactResp{}, err
		}
		damages = append(damages, row)
	}
	if err := damageRows.Err(); err != nil {
		damageRows.Close()
		return MeReactionDamageImpactResp{}, err
	}
	damageRows.Close()
	// 权威次数：reactions_used（结算写入），按 user + 窗口聚合，不能用 events 冒充。
	countRows, err := s.pool.Query(ctx, `
		SELECT k.key, SUM((k.value::text)::bigint)
		  FROM battle_records br,
		       jsonb_each(br.reactions_used) AS k(key, value)
		 WHERE br.user_id = $1
		   AND br.created_at >= $2 AND br.created_at < $3
		   AND k.key = ANY($4)
		 GROUP BY k.key`, userID, from, to, keys)
	if err != nil {
		return MeReactionDamageImpactResp{}, err
	}
	authCounts := map[string]int64{}
	for countRows.Next() {
		var k string
		var c int64
		if err := countRows.Scan(&k, &c); err != nil {
			countRows.Close()
			return MeReactionDamageImpactResp{}, err
		}
		authCounts[k] = c
	}
	if err := countRows.Err(); err != nil {
		countRows.Close()
		return MeReactionDamageImpactResp{}, err
	}
	countRows.Close()
	// 诚实空态统计：窗口内无有效 timeline 的战报数。
	var excluded int
	if err := s.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM battle_records
		 WHERE user_id = $1
		   AND created_at >= $2 AND created_at < $3
		   AND (reaction_timeline IS NULL
		        OR reaction_timeline->>'schema_version' IS DISTINCT FROM $4
		        OR reaction_timeline->'events' IS NULL
		        OR jsonb_array_length(reaction_timeline->'events') = 0)`, userID, from, to, domain.ReactionTimelineSchemaVersion).Scan(&excluded); err != nil {
		return MeReactionDamageImpactResp{}, err
	}
	// 复用同一聚合函数（纯逻辑测试覆盖的正是这条生产路径）。
	return AggregateMeReactionDamageImpact(damages, authCounts, excluded, now), nil
}
