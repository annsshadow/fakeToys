// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R7 缺口3：推荐落地收益预估的 E2E 契约。
//
// 判据不是"字段存在"：先制造可核对的真实战报和默认构筑，再断言
// 预估里的命中/波、攻击变化、伤害变化都能从这些真实行与配置推出来。
// 空历史、非法参数、未登录同样是契约的一部分。

import (
	"math"
	"testing"
)

func requireImpactEnvelope(t *testing.T, body map[string]any, wantContext string) {
	t.Helper()
	requireRecEnvelope(t, body, wantContext)
	if _, ok := body["model"].(map[string]any); !ok {
		t.Fatalf("model 缺失：%v", body["model"])
	}
}

func TestE2EMeRecommendationImpactFromRealBattles(t *testing.T) {
	e := newE2E(t)
	uid, token := e.newPlayer(t, "impact")

	base := "/api/v1/me/recommendations/impact?kind=equipment&id=18&replace=1"

	// 1) 零战报：诚实空态，不能拿默认构筑编一个"预计提升"。
	status, body := e.get(t, base, token)
	if status != 200 {
		t.Fatalf("空历史 impact = %d %v", status, body)
	}
	requireImpactEnvelope(t, body, "equipment")
	if body["empty_reason"] == nil {
		t.Fatalf("空历史必须给 empty_reason：%v", body)
	}
	reason := body["empty_reason"].(map[string]any)
	if reason["code"] != "NO_HISTORY" {
		t.Fatalf("空历史 code = %v，期望 NO_HISTORY", reason["code"])
	}
	if body["impact"] != nil {
		t.Fatalf("空历史不应编造 impact：%v", body["impact"])
	}

	// 2) 真实战报：三局共 20 波、240 命中、两胜一负，但只有一局是三星。
	// "三星率"不能偷换成"胜率"——后者会把 2 星胜利也算进去。
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
		VALUES ($1, 5, 'win', 3, 10, 120, 8, now())`, uid)
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
		VALUES ($1, 6, 'lose', 0, 5, 60, 2, now())`, uid)
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
		VALUES ($1, 7, 'win', 2, 5, 60, 0, now())`, uid)

	status, body = e.get(t, base, token)
	if status != 200 {
		t.Fatalf("有历史 impact = %d %v", status, body)
	}
	requireImpactEnvelope(t, body, "equipment")
	if num(body["input_snapshot"].(map[string]any), "battle_count") != 3 {
		t.Fatalf("battle_count 应来自真实战报 =3：%v", body["input_snapshot"])
	}
	if body["empty_reason"] != nil {
		t.Fatalf("有历史 empty_reason 应为 null：%v", body["empty_reason"])
	}
	evidence := body["evidence"].([]any)
	hitsPerWaveOK := false
	for _, raw := range evidence {
		ev, _ := raw.(map[string]any)
		metric, _ := ev["metric"].(string)
		if metric == "历史命中/波" && ev["value"].(float64) == 12 {
			hitsPerWaveOK = true
		}
	}
	if !hitsPerWaveOK {
		t.Fatalf("evidence 必须含真实命中/波=12：%v", evidence)
	}

	impact, ok := body["impact"].(map[string]any)
	if !ok {
		t.Fatalf("有历史必须给 impact：%v", body)
	}
	if impact["kind"] != "equipment" {
		t.Fatalf("impact.kind = %v，期望 equipment", impact["kind"])
	}
	if num(impact, "candidate_id") != 18 || num(impact, "replaced_id") != 1 {
		t.Fatalf("候选/替换 id 应与参数一致：%v", impact)
	}
	baseline, ok1 := impact["baseline"].(map[string]any)
	candidate, ok2 := impact["candidate"].(map[string]any)
	changes, ok3 := impact["changes"].(map[string]any)
	if !ok1 || !ok2 || !ok3 {
		t.Fatalf("impact 三块缺失：%v", impact)
	}
	// 母核碎片 220‰ - 制式炮管 50‰ = +170‰，来自 config 种子数据。
	if num(changes, "attack_permille_delta") != 170 {
		t.Fatalf("attack_permille_delta = %v，期望 170", changes["attack_permille_delta"])
	}
	if num(changes, "damage_per_wave_delta") <= 0 || num(changes, "damage_per_wave_pct") <= 0 {
		t.Fatalf("伤害预估必须为正增量：%v", changes)
	}
	if num(baseline, "hits_per_wave") != 12 || num(candidate, "hits_per_wave") != 12 {
		t.Fatalf("命中/波必须复用真实战报：baseline=%v candidate=%v", baseline, candidate)
	}
	// 2 胜 / 3 场 = 66.7%，但三星只有 1 / 3 = 33.3%；必须报后者。
	if math.Abs(changes["star_rate_observed"].(float64)-100.0/3.0) > 0.01 {
		t.Fatalf("三星率应来自真实战报 =33.3%%（不是胜率 66.7%%）：%v", changes)
	}
	for _, raw := range evidence {
		ev, _ := raw.(map[string]any)
		if ev["metric"] == "历史三星率" && math.Abs(ev["value"].(float64)-100.0/3.0) > 0.01 {
			t.Fatalf("evidence 三星率不得写成胜率：%v", ev)
		}
	}
	if changes["star_probability_estimate"] != nil {
		t.Fatalf("没有因果模型时不得编造星级概率：%v", changes["star_probability_estimate"])
	}

	// 3) 技能替换同样可用，且上下文随 kind 变化。
	status, body = e.get(t, "/api/v1/me/recommendations/impact?kind=skill&id=2&replace=1", token)
	if status != 200 {
		t.Fatalf("skill impact = %d %v", status, body)
	}
	requireImpactEnvelope(t, body, "skill")
	impact = body["impact"].(map[string]any)
	if impact["kind"] != "skill" || num(impact, "candidate_id") != 2 {
		t.Fatalf("技能 impact 参数回显错误：%v", impact)
	}

	// 4) 非法参数与越权。
	for _, q := range []string{
		"kind=weapon&id=18&replace=1",
		"kind=equipment&id=abc&replace=1",
		"kind=equipment&replace=1",
		"kind=equipment&id=18&replace=9999",
		"kind=equipment&id=18",
		"kind=skill&id=2&replace=1&id=3",
	} {
		status, body = e.get(t, "/api/v1/me/recommendations/impact?"+q, token)
		if status != 400 {
			t.Fatalf("非法参数 %q = %d %v，期望 400", q, status, body)
		}
	}
	status, body = e.get(t, base, "")
	if status != 401 {
		t.Fatalf("未登录 impact = %d %v，期望 401", status, body)
	}
}
