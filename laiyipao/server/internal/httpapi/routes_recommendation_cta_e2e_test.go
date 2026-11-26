// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R7-BUG-1：技能/装备推荐卡的 CTA 必须给出能被收益端点消费的 id/replace。
// 只断言"有 CTA"不够：前端会拿这两个参数直接请求 /recommendations/impact，
// 参数缺失就让收益弹层永远不可达。

import (
	"fmt"
	"testing"
)

func recImpactParams(t *testing.T, body map[string]any, context string) map[string]any {
	t.Helper()
	requireRecEnvelope(t, body, context)
	insights, ok := body["insights"].([]any)
	if !ok || len(insights) == 0 {
		t.Fatalf("%s 推荐没有 insight：%v", context, body["insights"])
	}
	cta, ok := insights[0].(map[string]any)["cta"].(map[string]any)
	if !ok {
		t.Fatalf("%s 推荐没有 CTA：%v", context, insights[0])
	}
	params, ok := cta["params"].(map[string]any)
	if !ok {
		t.Fatalf("%s 推荐 CTA 没有 params：%v", context, cta)
	}
	return params
}

func requireReachableImpact(t *testing.T, e *e2e, token string, kind string, params map[string]any) {
	t.Helper()
	id, replaced := num(params, "id"), num(params, "replace")
	if id <= 0 || replaced <= 0 {
		t.Fatalf("%s 推荐 CTA 缺少可用 id/replace，收益弹层不可达：%v", kind, params)
	}
	path := fmt.Sprintf("/api/v1/me/recommendations/impact?kind=%s&id=%d&replace=%d", kind, id, replaced)
	status, body := e.get(t, path, token)
	if status != 200 {
		t.Fatalf("按 %s 推荐 CTA 请求 impact = %d %v", kind, status, body)
	}
	impact, ok := body["impact"].(map[string]any)
	if !ok {
		t.Fatalf("%s 推荐 CTA 指向的 impact 没有真实预估：%v", kind, body)
	}
	if num(impact, "candidate_id") != id || num(impact, "replaced_id") != replaced {
		t.Fatalf("impact 回显的替换关系与 CTA 不一致：%v，CTA=%v", impact, params)
	}
}

func TestE2ERecommendationImpactCTAIsReachable(t *testing.T) {
	e := newE2E(t)
	uid, token := e.newPlayer(t, "cta_impact")
	// 有历史且已解锁中高阶配置，推荐才应给出具体替换方案。
	e.exec(t, `INSERT INTO user_progress (user_id, max_stage) VALUES ($1, 60)
		ON CONFLICT (user_id) DO UPDATE SET max_stage = GREATEST(user_progress.max_stage, EXCLUDED.max_stage)`, uid)
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
		VALUES ($1, 5, 'win', 3, 10, 120, 8, now())`, uid)
	// R7-BUG-2：CTA 只从玩家已拥有集合推导；新号默认未持有 2 / 11。
	e.exec(t, `INSERT INTO user_skills (user_id, skill_id) VALUES ($1, 2) ON CONFLICT DO NOTHING`, uid)
	e.exec(t, `INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		VALUES ($1, 11, 'gloves', FALSE) ON CONFLICT (user_id, equipment_id) DO NOTHING`, uid)

	status, body := e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != 200 {
		t.Fatalf("skills 推荐 = %d %v", status, body)
	}
	requireReachableImpact(t, e, token, "skill", recImpactParams(t, body, "skills"))

	status, body = e.get(t, "/api/v1/me/recommendations?context=equipment", token)
	if status != 200 {
		t.Fatalf("equipment 推荐 = %d %v", status, body)
	}
	requireReachableImpact(t, e, token, "equipment", recImpactParams(t, body, "equipment"))
}

func TestE2ERecommendationImpactCTAOmitsInvalidIds(t *testing.T) {
	e := newE2E(t)
	uid, token := e.newPlayer(t, "cta_invalid")
	// 新号进度为 0：配置里有更优装备，但玩家尚未解锁，不能下发不可用的替换 id。
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
		VALUES ($1, 5, 'win', 3, 10, 120, 8, now())`, uid)
	status, body := e.get(t, "/api/v1/me/recommendations?context=equipment", token)
	if status != 200 {
		t.Fatalf("equipment 推荐 = %d %v", status, body)
	}
	params := recImpactParams(t, body, "equipment")
	if num(params, "id") != 0 || num(params, "replace") != 0 {
		t.Fatalf("未解锁候选不得下发 id/replace：%v", params)
	}
}
