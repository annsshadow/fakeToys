// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R21 后端纯逻辑契约：真实 battle_records 聚合、连续失败阈值、分母归零与确定性教练建议。
// test-first：先写测试锁定口径，再写实现；不碰结算/replay_hash。

import (
	"encoding/json"
	"fmt"
	"math"
	"reflect"
	"strings"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

func TestAggregateLevelCoachFieldsAndStuck(t *testing.T) {
	if domain.StuckThreshold != 2 {
		t.Fatalf("基线 StuckThreshold 必须为 2，实际 %d", domain.StuckThreshold)
	}
	rows := []levelCoachRow{
		{LevelID: 1, Attempts: 4, Wins: 2, BestWave: 7, WaveSum: 21, LastResult: "lose", RecentResults: []string{"lose", "win", "lose", "lose"}},
		{LevelID: 2, Attempts: 3, Wins: 0, BestWave: 3, WaveSum: 8, LastResult: "lose", RecentResults: []string{"lose", "lose", "lose"}},
	}
	now := time.Date(2026, 11, 24, 12, 0, 0, 0, time.UTC)
	got := AggregateLevelCoach(rows, now)
	if got.WindowDays != 30 || got.WindowFrom != "2026-10-25" || got.WindowTo != "2026-11-24" {
		t.Fatalf("窗口不符：%+v", got)
	}
	if len(got.Items) != 2 {
		t.Fatalf("应有两行：%+v", got.Items)
	}
	it1, it2 := got.Items[0], got.Items[1]
	if it1.LevelID != 1 || it1.Attempts != 4 || it1.Wins != 2 || it1.BestWave != 7 || it1.AvgWave != 5.3 || it1.LastResult != "lose" || it1.Stuck {
		t.Fatalf("关卡1 不符（遇 win 即停，不应 stuck）：%+v", it1)
	}
	if it2.LevelID != 2 || it2.Attempts != 3 || it2.Wins != 0 || it2.BestWave != 3 || it2.AvgWave != 2.7 || it2.LastResult != "lose" || !it2.Stuck {
		t.Fatalf("关卡2 不符（连续失败应 stuck）：%+v", it2)
	}
	if it1.Coach.Title == "" || it1.Coach.Detail == "" || len(it1.Coach.Suggestions) == 0 || it1.Coach.Confidence <= 0 {
		t.Fatalf("coach 四字段不得为空：%+v", it1.Coach)
	}
	if !strings.Contains(it1.Coach.Detail, fmt.Sprintf("%d 次", it1.Attempts)) || !strings.Contains(it1.Coach.Detail, fmt.Sprintf("%d 波", it1.BestWave)) {
		t.Fatalf("coach 必须由真实数据确定性推导：%+v", it1.Coach)
	}
	again := AggregateLevelCoach(rows, now)
	if !reflect.DeepEqual(got, again) {
		t.Fatal("同输入必须同输出（确定性教练建议）")
	}
	raw, err := json.Marshal(got)
	if err != nil {
		t.Fatalf("序列化失败：%v", err)
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatalf("反序列化失败：%v", err)
	}
	items, ok := m["items"].([]any)
	if !ok || len(items) != 2 {
		t.Fatalf("items 必须是数组：%s", raw)
	}
	row0, _ := items[0].(map[string]any)
	for _, k := range []string{"level_id", "attempts", "wins", "best_wave", "avg_wave", "last_result", "stuck", "coach"} {
		if _, ok := row0[k]; !ok {
			t.Fatalf("八字段缺 %s：%s", k, raw)
		}
	}
}

func TestAggregateLevelCoachEmptyAndZeroDivision(t *testing.T) {
	now := time.Date(2026, 11, 24, 0, 0, 0, 0, time.UTC)
	got := AggregateLevelCoach(nil, now)
	if got.Items == nil || len(got.Items) != 0 {
		t.Fatalf("空结果 items 必须是 []：%+v", got.Items)
	}
	// 防御性：attempts=0 的异常行不得产生 NaN，也不得产出 coach。
	zero := AggregateLevelCoach([]levelCoachRow{{LevelID: 9, Attempts: 0}}, now)
	if len(zero.Items) != 1 || zero.Items[0].AvgWave != 0 || math.IsNaN(zero.Items[0].AvgWave) || zero.Items[0].Coach.Title != "" {
		t.Fatalf("分母为 0 必须归零且无 coach：%+v", zero.Items)
	}
	raw, _ := json.Marshal(got)
	var m map[string]any
	_ = json.Unmarshal(raw, &m)
	if items, ok := m["items"].([]any); !ok || len(items) != 0 {
		t.Fatalf("空 items JSON 必须是 []：%s", raw)
	}
}
