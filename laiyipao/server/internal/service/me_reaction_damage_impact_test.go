// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R19 玩家反应伤害贡献聚合（test-first）。
// count 取 reactions_used（权威结算次数），不得用 events 冒充；
// 旧战报无 timeline 但次数仍入 count；SUM=0 占比归零；空结果 items=[]。

import (
	"encoding/json"
	"math"
	"testing"
	"time"
)

func TestR19AggregateMeReactionDamageImpactPure(t *testing.T) {
	now := time.Date(2026, 11, 24, 12, 0, 0, 0, time.UTC)
	damages := []meDamageRow{
		{Reaction: "overheat", TotalDamage: 50, Events: 2},
		{Reaction: "steam_burst", TotalDamage: 50, Events: 1},
		{Reaction: "unknown_key", TotalDamage: 999, Events: 1},
	}
	// 权威次数来自 reactions_used（结算写入），不是 events 条数。
	authCounts := map[string]int64{"overheat": 5, "steam_burst": 1}
	res := AggregateMeReactionDamageImpact(damages, authCounts, 0, now)
	if res.WindowDays != 30 {
		t.Fatalf("window_days 应为 30，实际 %d", res.WindowDays)
	}
	if res.WindowFrom != "2026-10-25" || res.WindowTo != "2026-11-24" {
		t.Fatalf("窗口日期不符：%s ~ %s", res.WindowFrom, res.WindowTo)
	}
	if len(res.Items) != 2 {
		t.Fatalf("未知 key 应丢弃，期望 2 行，实际 %d：%+v", len(res.Items), res.Items)
	}
	byKey := map[string]MeReactionDamageImpactItem{}
	for _, it := range res.Items {
		byKey[it.Reaction] = it
	}
	ov := byKey["overheat"]
	if ov.TotalDamage != 50 || ov.Events != 2 || ov.Count != 5 {
		t.Fatalf("overheat 聚合不符（damage/events/count）：%+v", ov)
	}
	if math.Abs(ov.DamageSharePct-50) > 1e-9 {
		t.Fatalf("overheat 占比应为 50%%，实际 %v", ov.DamageSharePct)
	}
	st := byKey["steam_burst"]
	if st.TotalDamage != 50 || st.Events != 1 || st.Count != 1 {
		t.Fatalf("steam_burst 聚合不符（damage/events/count）：%+v", st)
	}
}

func TestR19OldBattleCountStillCounted(t *testing.T) {
	now := time.Date(2026, 11, 24, 12, 0, 0, 0, time.UTC)
	// 无 timeline 伤害，但权威次数仍在 → items 也要有该 reaction，count 保留。
	res := AggregateMeReactionDamageImpact(nil, map[string]int64{"overheat": 5}, 2, now)
	if len(res.Items) != 1 {
		t.Fatalf("旧战报次数应占一行，实际 %d：%+v", len(res.Items), res.Items)
	}
	it := res.Items[0]
	if it.Reaction != "overheat" || it.Count != 5 || it.TotalDamage != 0 || it.Events != 0 {
		t.Fatalf("旧战报次数/空伤害不符：%+v", it)
	}
	if it.DamageSharePct != 0 || math.IsNaN(it.DamageSharePct) {
		t.Fatalf("无伤害占比应为 0：%+v", it)
	}
	if res.ExcludedOldBattles != 2 {
		t.Fatalf("excluded_old_battles 应为 2，实际 %d", res.ExcludedOldBattles)
	}
}

func TestR19ZeroAndEmpty(t *testing.T) {
	now := time.Date(2026, 11, 24, 12, 0, 0, 0, time.UTC)
	zero := []meDamageRow{{Reaction: "overheat", TotalDamage: 0, Events: 3}}
	res := AggregateMeReactionDamageImpact(zero, map[string]int64{"overheat": 3}, 0, now)
	if len(res.Items) != 1 || res.Items[0].DamageSharePct != 0 || math.IsNaN(res.Items[0].DamageSharePct) {
		t.Fatalf("SUM=0 时占比应为 0 且不产 NaN：%+v", res.Items)
	}
	empty := AggregateMeReactionDamageImpact(nil, nil, 0, now)
	if empty.Items == nil || len(empty.Items) != 0 {
		t.Fatalf("空结果 items 必须是非 nil 空数组：%+v", empty.Items)
	}
}

func TestR19JSONShape(t *testing.T) {
	res := MeReactionDamageImpactResp{
		WindowDays: 30, WindowFrom: "2026-10-25", WindowTo: "2026-11-24", ExcludedOldBattles: 2,
		Items: []MeReactionDamageImpactItem{{Reaction: "overheat", TotalDamage: 50, DamageSharePct: 50, Events: 2, Count: 5}},
	}
	b, err := json.Marshal(res)
	if err != nil {
		t.Fatal(err)
	}
	var m map[string]any
	if err := json.Unmarshal(b, &m); err != nil {
		t.Fatal(err)
	}
	for _, key := range []string{"window_days", "window_from", "window_to", "excluded_old_battles", "items"} {
		if _, ok := m[key]; !ok {
			t.Fatalf("JSON 缺外层字段 %s：%s", key, b)
		}
	}
	items, _ := m["items"].([]any)
	if len(items) != 1 {
		t.Fatalf("items 应 1 行：%s", b)
	}
	item0, _ := items[0].(map[string]any)
	for _, key := range []string{"reaction", "total_damage", "damage_share_pct", "events", "count"} {
		if _, ok := item0[key]; !ok {
			t.Fatalf("items 行缺字段 %s：%s", key, b)
		}
	}
}
