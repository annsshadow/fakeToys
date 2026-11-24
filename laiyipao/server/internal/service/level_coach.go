// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R21 玩家卡点教练：从当前用户近 30 天 battle_records 跨局聚合。
// 辅助诊断，不改结算/score/星级/loot/replay_hash；权威验真仍以 replay_hash 为准。
// 严格 WHERE user_id；连续失败沿用 domain.StuckThreshold=2（遇 win 即停）。

import (
	"context"
	"fmt"
	"math"
	"sort"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

const levelCoachWindowDays = 30

// LevelCoachWindowDaysValue 供 httpapi 层复用窗口参数校验，避免两处硬编码漂移。
const LevelCoachWindowDaysValue = levelCoachWindowDays

// levelCoachRow 是一条 SQL 聚合行（按 level_id 分组）。
type levelCoachRow struct {
	LevelID       int
	Attempts      int64
	Wins          int64
	BestWave      int
	WaveSum       int64
	LastResult    string
	RecentResults []string // 该关最近若干局，newest-first；够判定连续失败即可
}

// LevelCoachAdvice 是逐行确定性教练建议（四字段，SPEC §3 钉死）。
type LevelCoachAdvice struct {
	Title       string   `json:"title"`
	Detail      string   `json:"detail"`
	Suggestions []string `json:"suggestions"`
	Confidence  float64  `json:"confidence"`
}

// LevelCoachItem 是响应行（八字段，SPEC §1 钉死）。
type LevelCoachItem struct {
	LevelID    int              `json:"level_id"`
	Attempts   int64            `json:"attempts"`
	Wins       int64            `json:"wins"`
	BestWave   int              `json:"best_wave"`
	AvgWave    float64          `json:"avg_wave"`
	LastResult string           `json:"last_result"`
	Stuck      bool             `json:"stuck"`
	Coach      LevelCoachAdvice `json:"coach"`
}

// LevelCoachResp 是 GET /me/level-coach 响应。
type LevelCoachResp struct {
	WindowDays int              `json:"window_days"`
	WindowFrom string           `json:"window_from"`
	WindowTo   string           `json:"window_to"`
	Items      []LevelCoachItem `json:"items"`
}

func levelCoachAverage(sum, count int64) float64 {
	if count <= 0 {
		return 0
	}
	return math.Round(float64(sum)/float64(count)*10) / 10
}

// levelCoachStuck 从 newest-first 的结果序列头数连续失败，遇 win 即停。
func levelCoachStuck(results []string) bool {
	n := 0
	for _, r := range results {
		if r != "lose" {
			break
		}
		n++
		if n >= domain.StuckThreshold {
			return true
		}
	}
	return false
}

// deriveLevelCoachAdvice 由本行真实数据确定性推导，不硬编码反应名/关卡名结论。
func deriveLevelCoachAdvice(it LevelCoachItem) LevelCoachAdvice {
	if it.Attempts <= 0 {
		return LevelCoachAdvice{}
	}
	advice := LevelCoachAdvice{
		Title: "AI · 卡点教练",
		Detail: fmt.Sprintf("近 30 天该关尝试 %d 次、胜 %d 次、最好到第 %d 波，平均到第 %.1f 波",
			it.Attempts, it.Wins, it.BestWave, it.AvgWave),
		// 置信度只表达样本量（真实 attempts 单调递增、封顶 100），不宣称模型/权威结论。
		Confidence: math.Min(100, float64(it.Attempts)*10),
	}
	suggestion := "保持当前构筑，继续挑战更高波次"
	if it.Stuck {
		suggestion = "最近连续失利，先调整构筑或技能组合再挑战"
	} else if it.Wins == 0 {
		suggestion = "先提升输出与生存，再重试本关"
	} else if it.Wins*2 < it.Attempts {
		suggestion = "复盘失败波次，针对性补强短板"
	}
	advice.Suggestions = []string{suggestion}
	return advice
}

// AggregateLevelCoach 把 SQL 结构化行转为响应（纯逻辑，单测覆盖生产路径）。
func AggregateLevelCoach(rows []levelCoachRow, now time.Time) LevelCoachResp {
	items := make([]LevelCoachItem, 0, len(rows))
	for _, row := range rows {
		it := LevelCoachItem{
			LevelID:    row.LevelID,
			Attempts:   row.Attempts,
			Wins:       row.Wins,
			BestWave:   row.BestWave,
			AvgWave:    levelCoachAverage(row.WaveSum, row.Attempts),
			LastResult: row.LastResult,
			Stuck:      levelCoachStuck(row.RecentResults),
		}
		it.Coach = deriveLevelCoachAdvice(it)
		items = append(items, it)
	}
	sort.Slice(items, func(i, j int) bool {
		if items[i].Attempts != items[j].Attempts {
			return items[i].Attempts > items[j].Attempts
		}
		return items[i].LevelID < items[j].LevelID
	})
	u := now.UTC()
	midnight := time.Date(u.Year(), u.Month(), u.Day(), 0, 0, 0, 0, time.UTC)
	return LevelCoachResp{
		WindowDays: levelCoachWindowDays,
		WindowFrom: midnight.AddDate(0, 0, -levelCoachWindowDays).Format("2006-01-02"),
		WindowTo:   midnight.Format("2006-01-02"),
		Items:      items,
	}
}

func levelCoachWindow(now time.Time) (time.Time, time.Time) {
	u := now.UTC()
	midnight := time.Date(u.Year(), u.Month(), u.Day(), 0, 0, 0, 0, time.UTC)
	return midnight.AddDate(0, 0, -levelCoachWindowDays), midnight.AddDate(0, 0, 1)
}

// MeLevelCoach 严格按当前 user_id + 30 天 UTC 窗口聚合，不接受前端传 user_id。
func (s *Service) MeLevelCoach(ctx context.Context, userID int64) (LevelCoachResp, error) {
	now := time.Now()
	from, to := levelCoachWindow(now)
	rows, err := s.pool.Query(ctx, `
		SELECT level_id,
		       COUNT(*)::bigint AS attempts,
		       COALESCE(SUM(CASE WHEN result = 'win' THEN 1 ELSE 0 END), 0)::bigint AS wins,
		       COALESCE(MAX(wave_reached), 0) AS best_wave,
		       COALESCE(SUM(wave_reached), 0)::bigint AS wave_sum
		  FROM battle_records
		 WHERE user_id = $1 AND created_at >= $2 AND created_at < $3
		 GROUP BY level_id`, userID, from, to)
	if err != nil {
		return LevelCoachResp{}, fmt.Errorf("level coach aggregate: %w", err)
	}
	agg := make(map[int]*levelCoachRow)
	for rows.Next() {
		var r levelCoachRow
		if err := rows.Scan(&r.LevelID, &r.Attempts, &r.Wins, &r.BestWave, &r.WaveSum); err != nil {
			rows.Close()
			return LevelCoachResp{}, err
		}
		agg[r.LevelID] = &r
	}
	if err := rows.Err(); err != nil {
		rows.Close()
		return LevelCoachResp{}, err
	}
	rows.Close()
	// 连续失败判定：只取每关最近 StuckThreshold 局（newest-first），遇 win 即停。
	recent, err := s.pool.Query(ctx, `
		SELECT level_id, result
		  FROM (SELECT level_id, result,
		               ROW_NUMBER() OVER (PARTITION BY level_id ORDER BY created_at DESC, id DESC) AS rn
		          FROM battle_records
		         WHERE user_id = $1 AND created_at >= $2 AND created_at < $3) t
		 WHERE t.rn <= $4
		 ORDER BY t.level_id, t.rn`, userID, from, to, domain.StuckThreshold)
	if err != nil {
		return LevelCoachResp{}, fmt.Errorf("level coach recent results: %w", err)
	}
	for recent.Next() {
		var levelID int
		var result string
		if err := recent.Scan(&levelID, &result); err != nil {
			recent.Close()
			return LevelCoachResp{}, err
		}
		if r := agg[levelID]; r != nil {
			if len(r.RecentResults) == 0 {
				r.LastResult = result
			}
			r.RecentResults = append(r.RecentResults, result)
		}
	}
	if err := recent.Err(); err != nil {
		recent.Close()
		return LevelCoachResp{}, err
	}
	recent.Close()
	rowsOut := make([]levelCoachRow, 0, len(agg))
	for _, r := range agg {
		rowsOut = append(rowsOut, *r)
	}
	return AggregateLevelCoach(rowsOut, now), nil
}
