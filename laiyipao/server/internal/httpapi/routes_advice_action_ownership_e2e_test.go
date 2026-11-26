// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R14：/me/advice/action 必须把「建议元素」翻译成服务端权威的替换 id。
// 未拥有候选时任何 id/replace 都不能下发；授予真实归属后才允许出现可执行 CTA。

import (
	"strings"
	"testing"
)

func adviceActionWithBattles(t *testing.T, e *e2e, tag string, count int) (int64, string) {
	t.Helper()
	uid, token := e.newPlayer(t, tag)
	e.exec(t, `INSERT INTO user_progress (user_id, max_stage) VALUES ($1, 60)
		ON CONFLICT (user_id) DO UPDATE SET max_stage = GREATEST(user_progress.max_stage, EXCLUDED.max_stage)`, uid)
	for i := 0; i < count; i++ {
		e.exec(t, `INSERT INTO battle_records
			(user_id, level_id, result, stars, wave_reached, hits, reactions, elements_used, reactions_used, created_at)
			VALUES ($1, 5, 'win', 3, 10, 120, 8, '{"fire":4}'::jsonb, '{"steam_burst":2}'::jsonb, now())`, uid)
	}
	return uid, token
}

func adviceEmptyCode(t *testing.T, body map[string]any) string {
	t.Helper()
	reason, ok := body["empty_reason"].(map[string]any)
	if !ok {
		t.Fatalf("响应缺少 empty_reason：%v", body)
	}
	code, _ := reason["code"].(string)
	return code
}

func adviceActionParams(t *testing.T, body map[string]any) map[string]any {
	t.Helper()
	params, ok := body["params"].(map[string]any)
	if !ok {
		t.Fatalf("成功响应缺少 params：%v", body)
	}
	return params
}

func TestE2EAdviceActionSkipsUnownedCandidate(t *testing.T) {
	e := newE2E(t)
	uid, token := adviceActionWithBattles(t, e, "advice_own_skill", 3)
	if owned := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id = $1 AND skill_id = 2`, uid); owned != 0 {
		t.Fatalf("前置条件失败：玩家已拥有火系技能 2，count=%d", owned)
	}
	status, body := e.get(t, "/api/v1/me/advice/action?element=fire&source=battle", token)
	if status != 200 {
		t.Fatalf("未拥有候选的 advice/action = %d %v", status, body)
	}
	if body["schema_version"] != "advice-action-v1" || body["badge"] != "AI" {
		t.Fatalf("响应缺少 advice-action-v1 契约：%v", body)
	}
	if body["element"] != "fire" {
		t.Fatalf("element 必须回显权威入参：%v", body["element"])
	}
	if code := adviceEmptyCode(t, body); code != "NO_OWNED_CANDIDATE" {
		t.Fatalf("未拥有候选应回 NO_OWNED_CANDIDATE，实际 %s：%v", code, body)
	}
	if id, replaced := num(body, "candidate_id"), num(body, "replaced_id"); id != 0 || replaced != 0 || body["params"] != nil {
		t.Fatalf("空态不得下发非零 id/replace/params：id=%d replace=%d params=%v", id, replaced, body["params"])
	}

	e.exec(t, `INSERT INTO user_skills (user_id, skill_id) VALUES ($1, 2) ON CONFLICT DO NOTHING`, uid)
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id = $1 AND skill_id = 2`, uid); got != 1 {
		t.Fatalf("授予归属失败：user_skills count=%d", got)
	}
	status, body = e.get(t, "/api/v1/me/advice/action?element=fire&source=battle", token)
	if status != 200 {
		t.Fatalf("授予归属后 advice/action = %d %v", status, body)
	}
	if body["empty_reason"] != nil {
		t.Fatalf("拥有候选后不应再是空态：%v", body["empty_reason"])
	}
	params := adviceActionParams(t, body)
	if id, replaced := num(params, "id"), num(params, "replace"); id != 2 || replaced != 4 {
		t.Fatalf("应给出已拥有的火系候选 2 替换 4，实际 id=%d replace=%d：%v", id, replaced, params)
	}
	if num(body, "candidate_id") != 2 || num(body, "replaced_id") != 4 || body["kind"] != "skill" {
		t.Fatalf("六字段与 params 不一致：%v", body)
	}
	if params["kind"] != "skill" || body["preview_route"] != "/pages/skills/skills" {
		t.Fatalf("params/preview_route 必须能被现有 R7 流程消费：%v", body)
	}
	impact, ok := body["impact"].(map[string]any)
	if !ok || num(impact, "candidate_id") != 2 || num(impact, "replaced_id") != 4 {
		t.Fatalf("impact 必须服务端同步算好且与 CTA 一致：%v", body["impact"])
	}
	raw, _ := body["based_on"].([]any)
	parts := make([]string, 0, len(raw))
	for _, item := range raw {
		if s, ok := item.(string); ok {
			parts = append(parts, s)
		}
	}
	joined := strings.Join(parts, " ")
	if !strings.Contains(joined, "battle_records.elements_used") || !strings.Contains(joined, "user_skills") {
		t.Fatalf("based_on 未指向真实事实源：%s", joined)
	}
}

func TestE2EAdviceActionEquipmentOwnedCandidate(t *testing.T) {
	e := newE2E(t)
	uid, token := adviceActionWithBattles(t, e, "advice_own_equip", 3)
	status, body := e.get(t, "/api/v1/me/advice/action?element=lightning", token)
	if status != 200 || adviceEmptyCode(t, body) != "NO_OWNED_CANDIDATE" {
		t.Fatalf("未拥有电系装备时 = %d %v", status, body)
	}
	e.exec(t, `INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		VALUES ($1, 11, 'gloves', FALSE) ON CONFLICT (user_id, equipment_id) DO NOTHING`, uid)
	status, body = e.get(t, "/api/v1/me/advice/action?element=lightning", token)
	if status != 200 || body["empty_reason"] != nil {
		t.Fatalf("授予归属后 advice/action = %d %v", status, body)
	}
	params := adviceActionParams(t, body)
	if num(params, "id") != 11 || num(params, "replace") != 10 || body["kind"] != "equipment" {
		t.Fatalf("应给出已拥有装备 11 替换 10：%v", body)
	}
	impact, _ := body["impact"].(map[string]any)
	if num(impact, "candidate_id") != 11 || num(impact, "replaced_id") != 10 {
		t.Fatalf("装备 impact 与 CTA 不一致：%v", body["impact"])
	}
}

func TestE2EAdviceActionHonestEmptyStates(t *testing.T) {
	e := newE2E(t)
	_, token := adviceActionWithBattles(t, e, "advice_invalid", 3)
	for _, tc := range []struct {
		name, url, token, code string
		status                 int
	}{
		{"非法元素", "/api/v1/me/advice/action?element=plasma", token, "INVALID_ELEMENT", 200},
		{"游客", "/api/v1/me/advice/action?element=fire", "", "NOT_AUTHENTICATED", 200},
		{"禁止伪造 id", "/api/v1/me/advice/action?element=fire&id=2&replace=4", token, "NO_OWNED_CANDIDATE", 200},
	} {
		status, body := e.get(t, tc.url, tc.token)
		if status != tc.status || adviceEmptyCode(t, body) != tc.code {
			t.Fatalf("%s = %d，code=%s，期望 %d/%s：%v", tc.name, status, adviceEmptyCode(t, body), tc.status, tc.code, body)
		}
		if id, replaced := num(body, "candidate_id"), num(body, "replaced_id"); id != 0 || replaced != 0 || body["params"] != nil {
			t.Fatalf("%s 空态不得带 id/replace/params：%v", tc.name, body)
		}
	}

	_, noBattle := adviceActionWithBattles(t, e, "advice_no_history", 0)
	status, body := e.get(t, "/api/v1/me/advice/action?element=fire", noBattle)
	if status != 200 || adviceEmptyCode(t, body) != "NO_HISTORY" {
		t.Fatalf("零战报应回 NO_HISTORY：%d %v", status, body)
	}
	_, few := adviceActionWithBattles(t, e, "advice_few", 1)
	status, body = e.get(t, "/api/v1/me/advice/action?element=fire", few)
	if status != 200 || adviceEmptyCode(t, body) != "NOT_ENOUGH_DATA" {
		t.Fatalf("样本不足应回 NOT_ENOUGH_DATA：%d %v", status, body)
	}
}
