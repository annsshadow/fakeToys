// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R20 后端纯逻辑契约：battle_records 权威聚合、按日分桶、归零与 AI 四判据。
// test-first：先写测试锁定口径，再写实现。

import (
	"encoding/json"
	"math"
	"testing"
	"time"
)

func TestAggregateBattleEfficiencyBucketsAndZeroDivision(t *testing.T) {
	now := time.Date(2026, 11, 24, 12, 0, 0, 0, time.UTC)
	rows := []battleEfficiencyRow{
		{Bucket: "2026-11-23", Battles: 2, Shots: 10, Hits: 7, HeatMax: 60, Leaked: 4, DurationMs: 120000},
		{Bucket: "2026-11-24", Battles: 1, Shots: 0, Hits: 0, HeatMax: 15, Leaked: 0, DurationMs: 30000},
	}
	got := AggregateBattleEfficiency(rows, now)
	if got.WindowDays != 30 || got.WindowFrom != "2026-10-25" || got.WindowTo != "2026-11-24" {
		t.Fatalf("窗口不符：%+v", got)
	}
	if got.Battles != 3 || got.Shots != 10 || got.Hits != 7 {
		t.Fatalf("汇总不符：%+v", got)
	}
	if got.AccuracyPct != 70 || got.AvgHeatMax != 25 || got.LeakedPerBattle != 1.3 || got.AvgDurationMs != 50000 {
		t.Fatalf("除零/浮点不符：%+v", got)
	}
	if len(got.Buckets) != 2 {
		t.Fatalf("应有 2 个日桶：%+v", got.Buckets)
	}
	b0, b1 := got.Buckets[0], got.Buckets[1]
	if b0.Bucket != "2026-11-23" || b0.Battles != 2 || b0.Shots != 10 || b0.Hits != 7 || b0.AccuracyPct != 70 {
		t.Fatalf("桶 0 不符：%+v", b0)
	}
	if b1.Bucket != "2026-11-24" || b1.Battles != 1 || b1.Shots != 0 || b1.Hits != 0 || b1.AccuracyPct != 0 {
		t.Fatalf("shots=0 桶命中率应归零：%+v", b1)
	}
	for _, v := range []float64{got.AccuracyPct, got.AvgHeatMax, got.LeakedPerBattle, got.AvgDurationMs} {
		if math.IsNaN(v) || math.IsInf(v, 0) {
			t.Fatalf("出现 NaN/Inf：%+v", got)
		}
	}
}

func TestAggregateBattleEfficiencyEmpty(t *testing.T) {
	now := time.Date(2026, 11, 24, 0, 0, 0, 0, time.UTC)
	got := AggregateBattleEfficiency(nil, now)
	if got.Battles != 0 || got.Shots != 0 || got.Hits != 0 {
		t.Fatalf("空态汇总应全零：%+v", got)
	}
	if got.AccuracyPct != 0 || got.AvgHeatMax != 0 || got.LeakedPerBattle != 0 || got.AvgDurationMs != 0 {
		t.Fatalf("空态除法应归零：%+v", got)
	}
	if got.Buckets == nil || len(got.Buckets) != 0 {
		t.Fatalf("空态 buckets 必须是空数组而非 null：%+v", got.Buckets)
	}
	if got.AIInsight != (BattleEfficiencyAIInsight{}) {
		t.Fatalf("空态 ai_insight 必须为空对象：%+v", got.AIInsight)
	}
	raw, err := json.Marshal(got)
	if err != nil {
		t.Fatalf("JSON 序列化失败：%v", err)
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatalf("JSON 反序列化失败：%v", err)
	}
	for _, k := range []string{"window_days", "window_from", "window_to", "battles", "shots", "hits", "accuracy_pct", "avg_heat_max", "leaked_per_battle", "avg_duration_ms", "buckets", "ai_insight"} {
		if _, ok := m[k]; !ok {
			t.Fatalf("JSON 缺字段 %s：%s", k, raw)
		}
	}
	if _, ok := m["buckets"].([]any); !ok {
		t.Fatalf("buckets 必须是 JSON 数组：%s", raw)
	}
	if v, ok := m["accuracy_pct"].(float64); !ok || math.IsNaN(v) {
		t.Fatalf("accuracy_pct 不得为 NaN：%s", raw)
	}
}

func TestAggregateBattleEfficiencyAIInsightFourCriteria(t *testing.T) {
	now := time.Date(2026, 11, 24, 0, 0, 0, 0, time.UTC)
	rows := []battleEfficiencyRow{
		{Bucket: "2026-11-23", Battles: 1, Shots: 8, Hits: 6, HeatMax: 20, Leaked: 1, DurationMs: 60000},
		{Bucket: "2026-11-24", Battles: 1, Shots: 4, Hits: 1, HeatMax: 90, Leaked: 2, DurationMs: 60000},
	}
	got := AggregateBattleEfficiency(rows, now)
	ai := got.AIInsight
	if ai.Title != "AI · 战斗效率诊断" {
		t.Fatalf("AI 标题必须含可见汉字：%q", ai.Title)
	}
	if ai.Conclusion != "近 30 天命中率 58.3%，效率呈下降趋势，峰值热量均值 55.0，建议先改善 heat 管理" {
		t.Fatalf("AI 真实数据结论不符：%q", ai.Conclusion)
	}
	if ai.Evidence != "依据：近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准" {
		t.Fatalf("AI 依据行不符：%q", ai.Evidence)
	}
	if ai.CTA == "" {
		t.Fatal("AI CTA 不得为空")
	}
}
