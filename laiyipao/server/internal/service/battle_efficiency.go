// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R20 战斗效率趋势聚合：权威 count 取 battle_records 行数（30 天 UTC 窗口），
// shots/hits/heat_max/leaked/duration_ms 取 SUM；按日分桶，命中率 hits/shots。
// 所有除法在分母为 0 时归零；不写 NaN/Infinity，不改结算/score/replay_hash。
// me 与 admin 复用同一聚合函数，me 额外 WHERE user_id = 当前登录用户。

import (
	"context"
	"fmt"
	"math"
	"sort"
	"time"
)

const battleEfficiencyWindowDays = 30

// BattleEfficiencyWindowDaysValue 供 httpapi 层校验 window_days 参数复用，避免两处硬编码漂移。
const BattleEfficiencyWindowDaysValue = 30

// battleEfficiencyRow 是 SQL 聚合返回的单行（me/admin 同口径，me 仅多 user_id 过滤）。
type battleEfficiencyRow struct {
	Bucket     string
	Battles    int64
	Shots      int64
	Hits       int64
	HeatMax    int64
	Leaked     int64
	DurationMs int64
}

// BattleEfficiencyBucket 是逐日效率桶，字段逐字钉死在 SPEC。
type BattleEfficiencyBucket struct {
	Bucket       string  `json:"bucket"`
	Battles      int64   `json:"battles"`
	Shots        int64   `json:"shots"`
	Hits         int64   `json:"hits"`
	AccuracyPct  float64 `json:"accuracy_pct"`
}

// BattleEfficiencyAIInsight 是辅助诊断（四判据：标题/结论/依据/CTA）。
type BattleEfficiencyAIInsight struct {
	Title      string `json:"title,omitempty"`
	Conclusion string `json:"conclusion,omitempty"`
	Evidence   string `json:"evidence,omitempty"`
	CTA        string `json:"cta,omitempty"`
}

// BattleEfficiencyResp 是 me/admin 两端共享的响应结构。
type BattleEfficiencyResp struct {
	WindowDays      int                       `json:"window_days"`
	WindowFrom      string                    `json:"window_from"`
	WindowTo        string                    `json:"window_to"`
	Battles         int64                     `json:"battles"`
	Shots           int64                     `json:"shots"`
	Hits            int64                     `json:"hits"`
	AccuracyPct     float64                   `json:"accuracy_pct"`
	AvgHeatMax      float64                   `json:"avg_heat_max"`
	LeakedPerBattle float64                   `json:"leaked_per_battle"`
	AvgDurationMs   float64                   `json:"avg_duration_ms"`
	Buckets         []BattleEfficiencyBucket  `json:"buckets"`
	AIInsight       BattleEfficiencyAIInsight `json:"ai_insight"`
}

func r20AccuracyPct(hits, shots int64) float64 {
	if shots == 0 {
		return 0
	}
	return math.Round(float64(hits)/float64(shots)*1000) / 10
}

func r20Average(sum, count int64) float64 {
	if count == 0 {
		return 0
	}
	return math.Round(float64(sum)/float64(count)*10) / 10
}

// AggregateBattleEfficiency 把 SQL 聚合行合并为响应（纯逻辑，单测覆盖生产路径）。
func AggregateBattleEfficiency(rows []battleEfficiencyRow, now time.Time) BattleEfficiencyResp {
	buckets := make([]BattleEfficiencyBucket, 0, len(rows))
	var battles, shots, hits, heat, leaked, duration int64
	for _, row := range rows {
		battles += row.Battles
		shots += row.Shots
		hits += row.Hits
		heat += row.HeatMax
		leaked += row.Leaked
		duration += row.DurationMs
		buckets = append(buckets, BattleEfficiencyBucket{
			Bucket: row.Bucket, Battles: row.Battles, Shots: row.Shots, Hits: row.Hits,
			AccuracyPct: r20AccuracyPct(row.Hits, row.Shots),
		})
	}
	sort.Slice(buckets, func(i, j int) bool { return buckets[i].Bucket < buckets[j].Bucket })
	u := now.UTC()
	resp := BattleEfficiencyResp{
		WindowDays: battleEfficiencyWindowDays,
		WindowFrom: u.AddDate(0, 0, -battleEfficiencyWindowDays).Format("2006-01-02"),
		WindowTo:   u.Format("2006-01-02"),
		Battles:    battles,
		Shots:      shots,
		Hits:       hits,
		AccuracyPct:     r20AccuracyPct(hits, shots),
		AvgHeatMax:      r20Average(heat, battles),
		LeakedPerBattle: r20Average(leaked, battles),
		AvgDurationMs:   r20Average(duration, battles),
		Buckets:         buckets,
		AIInsight:       BattleEfficiencyAIInsight{},
	}
	if battles > 0 {
		resp.AIInsight = deriveBattleEfficiencyInsight(resp)
	}
	return resp
}

// deriveBattleEfficiencyInsight 从聚合结果确定性推导 AI 四判据，不编造置信度、不宣称模型。
func deriveBattleEfficiencyInsight(resp BattleEfficiencyResp) BattleEfficiencyAIInsight {
	trend := "基本平稳"
	if len(resp.Buckets) >= 2 {
		first, last := resp.Buckets[0].AccuracyPct, resp.Buckets[len(resp.Buckets)-1].AccuracyPct
		if last > first+5 {
			trend = "呈上升趋势"
		} else if last < first-5 {
			trend = "呈下降趋势"
		}
	}
	heatTrend := ""
	if resp.AvgHeatMax >= 50 {
		heatTrend = "，峰值热量均值 " + fmt.Sprintf("%.1f", resp.AvgHeatMax) + "，建议先改善 heat 管理"
	}
	return BattleEfficiencyAIInsight{
		Title:      "AI · 战斗效率诊断",
		Conclusion: fmt.Sprintf("近 30 天命中率 %.1f%%，效率%s%s", resp.AccuracyPct, trend, heatTrend),
		Evidence:   "依据：近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准",
		CTA:        "去战斗页打一局，或到关卡页按短板调整构筑",
	}
}

// queryBattleEfficiency 是 me/admin 共用的唯一聚合 SQL 形状；userID<=0 表示 admin 全平台。
func (s *Service) queryBattleEfficiency(ctx context.Context, userID int64, from, to time.Time) ([]battleEfficiencyRow, error) {
	sql := `
		SELECT to_char(date_trunc('day', created_at AT TIME ZONE 'UTC'), 'YYYY-MM-DD') AS bucket,
		       COUNT(*)::bigint AS battles,
		       COALESCE(SUM(shots), 0)::bigint AS shots,
		       COALESCE(SUM(hits), 0)::bigint AS hits,
		       COALESCE(SUM(heat_max), 0)::bigint AS heat_max,
		       COALESCE(SUM(leaked), 0)::bigint AS leaked,
		       COALESCE(SUM(duration_ms), 0)::bigint AS duration_ms
		  FROM battle_records
		 WHERE created_at >= $1 AND created_at < $2`
	args := []any{from, to}
	if userID > 0 {
		sql += " AND user_id = $3"
		args = append(args, userID)
	}
	sql += " GROUP BY bucket ORDER BY bucket ASC"
	rows, err := s.pool.Query(ctx, sql, args...)
	if err != nil {
		return nil, fmt.Errorf("battle efficiency query: %w", err)
	}
	defer rows.Close()
	out := make([]battleEfficiencyRow, 0)
	for rows.Next() {
		var r battleEfficiencyRow
		if err := rows.Scan(&r.Bucket, &r.Battles, &r.Shots, &r.Hits, &r.HeatMax, &r.Leaked, &r.DurationMs); err != nil {
			return nil, err
		}
		out = append(out, r)
	}
	return out, rows.Err()
}

func battleEfficiencyWindow(now time.Time) (time.Time, time.Time) {
	u := now.UTC()
	midnight := time.Date(u.Year(), u.Month(), u.Day(), 0, 0, 0, 0, time.UTC)
	return midnight.AddDate(0, 0, -battleEfficiencyWindowDays), midnight.AddDate(0, 0, 1)
}

// MeBattleEfficiency 严格按当前 user_id + 30 天窗口聚合，不接受前端传 user_id。
func (s *Service) MeBattleEfficiency(ctx context.Context, userID int64) (BattleEfficiencyResp, error) {
	now := time.Now()
	from, to := battleEfficiencyWindow(now)
	rows, err := s.queryBattleEfficiency(ctx, userID, from, to)
	if err != nil {
		return BattleEfficiencyResp{}, err
	}
	return AggregateBattleEfficiency(rows, now), nil
}

// AdminBattleEfficiency 全平台同口径聚合（无 user_id 过滤）。
func (s *Service) AdminBattleEfficiency(ctx context.Context) (BattleEfficiencyResp, error) {
	now := time.Now()
	from, to := battleEfficiencyWindow(now)
	rows, err := s.queryBattleEfficiency(ctx, 0, from, to)
	if err != nil {
		return BattleEfficiencyResp{}, err
	}
	return AggregateBattleEfficiency(rows, now), nil
}
