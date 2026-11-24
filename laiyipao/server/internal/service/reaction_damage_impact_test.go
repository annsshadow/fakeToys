// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R18 后端反应伤害贡献聚合（test-first）。
// 契约：items 五字段 reaction/total_damage/damage_share_pct/events/count；
// window_days=30；count 必须来自 dashboard_reaction_counts 权威次数，不能用 events 冒充；
// 未知反应 key 丢弃；SUM=0 时占比全 0 不产生 NaN；空结果 items=[] 不是 null。

import (
	"encoding/json"
	"math"
	"strings"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

func TestR18AggregateReactionDamagePure(t *testing.T) {
	now := time.Date(2026, 11, 24, 12, 0, 0, 0, time.UTC)
	valid := []domain.ReactionTimelineEvent{
		{Wave: 1, Tick: 1, Reaction: "overheat", EnemyUID: 1, Damage: 30},
		{Wave: 1, Tick: 2, Reaction: "overheat", EnemyUID: 2, Damage: 20},
		{Wave: 2, Tick: 3, Reaction: "steam_burst", EnemyUID: 3, Damage: 50},
		{Wave: 2, Tick: 4, Reaction: "unknown_key", EnemyUID: 4, Damage: 999},
	}
	// 权威次数来自物化视图：overheat 5 次、steam_burst 1 次。
	authCounts := map[string]int64{"overheat": 5, "steam_burst": 1}
	res := AggregateReactionDamageImpact(valid, authCounts, 2, now)
	if res.WindowDays != 30 {
		t.Fatalf("window_days 应为 30，实际 %d", res.WindowDays)
	}
	if res.WindowFrom != "2026-10-25" || res.WindowTo != "2026-11-24" {
		t.Fatalf("窗口日期不符：%s ~ %s", res.WindowFrom, res.WindowTo)
	}
	if res.ExcludedOldBattles != 2 {
		t.Fatalf("excluded_old_battles 应为 2，实际 %d", res.ExcludedOldBattles)
	}
	if len(res.Items) != 2 {
		t.Fatalf("未知 key 应丢弃，期望 2 行，实际 %d：%+v", len(res.Items), res.Items)
	}
	byKey := map[string]ReactionDamageImpactItem{}
	for _, it := range res.Items {
		byKey[it.Reaction] = it
	}
	ov := byKey["overheat"]
	if ov.TotalDamage != 50 || ov.Events != 2 || ov.Count != 5 {
		t.Fatalf("overheat 聚合不符（total/events/count）：%+v", ov)
	}
	if math.Abs(ov.DamageSharePct-50.0) > 1e-9 {
		t.Fatalf("overheat 占比应为 50%%，实际 %v", ov.DamageSharePct)
	}
	st := byKey["steam_burst"]
	if st.TotalDamage != 50 || st.Events != 1 || st.Count != 1 {
		t.Fatalf("steam_burst 聚合不符（total/events/count）：%+v", st)
	}
}

func TestR18AggregateReactionDamageZeroAndEmpty(t *testing.T) {
	now := time.Date(2026, 11, 24, 0, 0, 0, 0, time.UTC)
	zero := []domain.ReactionTimelineEvent{
		{Wave: 1, Tick: 1, Reaction: "overheat", EnemyUID: 1, Damage: 0},
	}
	res := AggregateReactionDamageImpact(zero, map[string]int64{"overheat": 3}, 0, now)
	if len(res.Items) != 1 || res.Items[0].DamageSharePct != 0 {
		t.Fatalf("SUM=0 时占比应为 0 不产生 NaN：%+v", res.Items)
	}
	empty := AggregateReactionDamageImpact(nil, nil, 0, now)
	if empty.Items == nil || len(empty.Items) != 0 {
		t.Fatalf("空结果 items 必须是非 nil 空数组：%+v", empty.Items)
	}
	if empty.ExcludedOldBattles != 0 {
		t.Fatalf("空输入排除数应为 0，实际 %d", empty.ExcludedOldBattles)
	}
}

func TestR18ReactionDamageImpactJSONShape(t *testing.T) {
	// JSON 形状：snake_case 五字段 + 窗口/排除字段都要出现在对外响应里。
	res := ReactionDamageImpactResp{
		WindowDays: 30, WindowFrom: "2026-10-25", WindowTo: "2026-11-24", ExcludedOldBattles: 2,
		Items: []ReactionDamageImpactItem{{Reaction: "overheat", TotalDamage: 50, DamageSharePct: 50, Events: 2, Count: 5}},
	}
	b, err := json.Marshal(res)
	if err != nil {
		t.Fatal(err)
	}
	for _, key := range []string{"window_days", "window_from", "window_to", "excluded_old_battles", "items", "reaction", "total_damage", "damage_share_pct", "events", "count"} {
		if !strings.Contains(string(b), "\""+key+"\"") {
			t.Fatalf("JSON 缺字段 %s：%s", key, b)
		}
	}
}
