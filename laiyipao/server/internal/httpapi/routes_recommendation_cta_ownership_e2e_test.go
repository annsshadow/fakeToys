// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R7-BUG-2：推荐 CTA 的 id/replace 必须来自玩家**已拥有**集合。
// 只按配置数值挑候选会让两步确认弹层指向写端必然拒绝的 id：
// SaveLoadout / SetEquippedEquipment 都硬校验 user_skills / user_equipment 的归属。
// 这些用例先用未拥有的候选坐实缺陷，再授予真实归属并走真实写入接口验收。

import (
	"strings"
	"testing"
)

func recCTAParamsAndBasis(t *testing.T, body map[string]any, context string) (map[string]any, string) {
	t.Helper()
	params := recImpactParams(t, body, context)
	insights, ok := body["insights"].([]any)
	if !ok || len(insights) == 0 {
		t.Fatalf("%s 推荐没有 insight：%v", context, body["insights"])
	}
	raw, _ := insights[0].(map[string]any)["basis"].([]any)
	parts := make([]string, 0, len(raw))
	for _, item := range raw {
		if s, ok := item.(string); ok {
			parts = append(parts, s)
		}
	}
	return params, strings.Join(parts, " ")
}

func recCTAWithBattle(t *testing.T, e *e2e, tag string) (int64, string) {
	t.Helper()
	uid, token := e.newPlayer(t, tag)
	e.exec(t, `INSERT INTO user_progress (user_id, max_stage) VALUES ($1, 60)
		ON CONFLICT (user_id) DO UPDATE SET max_stage = GREATEST(user_progress.max_stage, EXCLUDED.max_stage)`, uid)
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
		VALUES ($1, 5, 'win', 3, 10, 120, 8, now())`, uid)
	return uid, token
}

func TestE2ERecommendationCTASkipsUnownedSkill(t *testing.T) {
	e := newE2E(t)
	uid, token := recCTAWithBattle(t, e, "cta_own_skill")
	// 新号只拥有 1/4/7；配置里 id=2（燃油弹 140 > 干冰弹 90）数值更高但未拥有。
	if owned := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id = $1 AND skill_id = 2`, uid); owned != 0 {
		t.Fatalf("前置条件失败：玩家已拥有技能 2，count=%d", owned)
	}
	status, body := e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != 200 {
		t.Fatalf("skills 推荐 = %d %v", status, body)
	}
	params, _ := recCTAParamsAndBasis(t, body, "skills")
	if id, replaced := num(params, "id"), num(params, "replace"); id != 0 || replaced != 0 {
		t.Fatalf("未拥有技能却被下发为 CTA：id=%d replace=%d（应不下发，前端才能不展示不可达收益卡）", id, replaced)
	}

	// 真实授予归属后，同一条推荐才应下发 id/replace。
	e.exec(t, `INSERT INTO user_skills (user_id, skill_id) VALUES ($1, 2) ON CONFLICT DO NOTHING`, uid)
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id = $1 AND skill_id = 2`, uid); got != 1 {
		t.Fatalf("授予归属失败：user_skills count=%d", got)
	}
	status, body = e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != 200 {
		t.Fatalf("授予归属后 skills 推荐 = %d %v", status, body)
	}
	params, basis := recCTAParamsAndBasis(t, body, "skills")
	if id, replaced := num(params, "id"), num(params, "replace"); id != 2 || replaced != 4 {
		t.Fatalf("拥有技能 2 后应推荐 2 替换 4，实际 id=%d replace=%d；basis=%s", id, replaced, basis)
	}
	if !strings.Contains(basis, "已拥有") {
		t.Fatalf("basis 没反映真实拥有情况：%s", basis)
	}
	requireReachableImpact(t, e, token, "skill", params)

	// 两步确认的真实写入：PUT /me/loadout 会把候选写进 user_skill_slots。
	slot := e.scalarInt(t, `SELECT slot FROM user_skill_slots WHERE user_id = $1 AND skill_id = 4`, uid)
	if slot < 0 || slot > 2 {
		t.Fatalf("被替换技能 4 的槽位异常：%d", slot)
	}
	next := []int{0, 0, 0}
	next[slot] = 2
	status, written := e.put(t, "/api/v1/me/loadout", token, map[string]any{"skill_ids": next})
	if status != 200 {
		t.Fatalf("按 CTA 保存 loadout = %d %v", status, written)
	}
	if e.scalarInt(t, `SELECT COUNT(*) FROM user_skill_slots WHERE user_id = $1 AND skill_id = 2`, uid) != 1 {
		t.Fatal("两步确认后候选技能 2 未写进出战槽位")
	}
	if e.scalarInt(t, `SELECT COUNT(*) FROM user_skill_slots WHERE user_id = $1 AND skill_id = 4`, uid) != 0 {
		t.Fatal("两步确认后被替换技能 4 仍占槽")
	}
}

func TestE2ERecommendationCTASkipsUnownedEquipment(t *testing.T) {
	e := newE2E(t)
	uid, token := recCTAWithBattle(t, e, "cta_own_equip")
	// 新号只拥有装备 1(weapon) / 10(gloves)；id=11（磁力手套 100‰ > 40‰）未拥有。
	if owned := e.scalarInt(t, `SELECT COUNT(*) FROM user_equipment WHERE user_id = $1 AND equipment_id = 11`, uid); owned != 0 {
		t.Fatalf("前置条件失败：玩家已拥有装备 11，count=%d", owned)
	}
	status, body := e.get(t, "/api/v1/me/recommendations?context=equipment", token)
	if status != 200 {
		t.Fatalf("equipment 推荐 = %d %v", status, body)
	}
	params, _ := recCTAParamsAndBasis(t, body, "equipment")
	if id, replaced := num(params, "id"), num(params, "replace"); id != 0 || replaced != 0 {
		t.Fatalf("未拥有装备却被下发为 CTA：id=%d replace=%d（应不下发）", id, replaced)
	}

	e.exec(t, `INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		VALUES ($1, 11, 'gloves', FALSE) ON CONFLICT (user_id, equipment_id) DO NOTHING`, uid)
	status, body = e.get(t, "/api/v1/me/recommendations?context=equipment", token)
	if status != 200 {
		t.Fatalf("授予归属后 equipment 推荐 = %d %v", status, body)
	}
	params, basis := recCTAParamsAndBasis(t, body, "equipment")
	if id, replaced := num(params, "id"), num(params, "replace"); id != 11 || replaced != 10 {
		t.Fatalf("拥有装备 11 后应推荐 11 替换 10，实际 id=%d replace=%d", id, replaced)
	}
	if !strings.Contains(basis, "已拥有") {
		t.Fatalf("basis 没反映真实拥有情况：%s", basis)
	}
	requireReachableImpact(t, e, token, "equipment", params)

	// 两步确认的真实写入：PUT /me/equipment 走完整归属校验。
	status, written := e.put(t, "/api/v1/me/equipment", token,
		map[string]any{"equipment_ids": []int{1, 11}})
	if status != 200 {
		t.Fatalf("按 CTA 保存 equipment = %d %v", status, written)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_equipment WHERE user_id = $1 AND equipment_id = 11 AND equipped`, uid); got != 1 {
		t.Fatalf("两步确认后候选装备 11 未装上：count=%d", got)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_equipment WHERE user_id = $1 AND equipment_id = 10 AND equipped`, uid); got != 0 {
		t.Fatalf("两步确认后被替换装备 10 仍装着：count=%d", got)
	}
}
