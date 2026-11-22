// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R10 审计：推荐卡有历史统计却没有 id/replace，收益弹层停在「接口未提供真实预估」。
// 决定性对照证明这不是新的服务端缺陷：缺 id 是因为更优候选玩家未拥有；
// 强行下发配置候选虽能预览 impact，但两步确认写入会被归属校验拒绝（t77 安全修复）。
import (
	"strings"
	"testing"
)

func TestE2ER10RecommendationCTAReasonAudit(t *testing.T) {
	e := newE2E(t)
	uid, token := recCTAWithBattle(t, e, "r10_cta_audit")
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id = $1 AND skill_id = 2`, uid); got != 0 {
		t.Fatalf("前置条件失败：新号不应拥有技能 2，count=%d", got)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_equipment WHERE user_id = $1 AND equipment_id = 11`, uid); got != 0 {
		t.Fatalf("前置条件失败：新号不应拥有装备 11，count=%d", got)
	}

	// ① 有战报但未拥有更优候选：CTA 必须诚实缺 id/replace，并在卡片依据里说明原因。
	status, body := e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != 200 {
		t.Fatalf("skills 推荐 = %d %v", status, body)
	}
	params, basis := recCTAParamsAndBasis(t, body, "skills")
	if id, replaced := num(params, "id"), num(params, "replace"); id != 0 || replaced != 0 {
		t.Fatalf("未拥有技能 2 却下发了 CTA：id=%d replace=%d", id, replaced)
	}
	if !strings.Contains(basis, "已拥有技能里没有同为主战且基础伤害高于当前") {
		t.Fatalf("卡片依据未解释收益不可用的真实原因：%s", basis)
	}
	status, body = e.get(t, "/api/v1/me/recommendations?context=equipment", token)
	if status != 200 {
		t.Fatalf("equipment 推荐 = %d %v", status, body)
	}
	params, basis = recCTAParamsAndBasis(t, body, "equipment")
	if id, replaced := num(params, "id"), num(params, "replace"); id != 0 || replaced != 0 {
		t.Fatalf("未拥有装备 11 却下发了 CTA：id=%d replace=%d", id, replaced)
	}
	if !strings.Contains(basis, "已拥有装备里没有同槽且加成高于当前") {
		t.Fatalf("卡片依据未解释收益不可用的真实原因：%s", basis)
	}

	// ② 强行套用 R10 猜测的「配置候选」：impact 预览能算出数，但真实写入必被拒。
	status, forced := e.get(t, "/api/v1/me/recommendations/impact?kind=skill&id=2&replace=4", token)
	if status != 200 {
		t.Fatalf("强制技能 impact = %d %v", status, forced)
	}
	slot := e.scalarInt(t, `SELECT slot FROM user_skill_slots WHERE user_id = $1 AND skill_id = 4`, uid)
	next := []int{0, 0, 0}
	if slot < 0 || slot > 2 {
		t.Fatalf("被替换技能 4 的槽位异常：%d", slot)
	}
	next[slot] = 2
	status, written := e.put(t, "/api/v1/me/loadout", token, map[string]any{"skill_ids": next})
	if status != 400 {
		t.Fatalf("未拥有的技能 2 应被写入端拒绝（400），实际 %d %v", status, written)
	}
	status, forced = e.get(t, "/api/v1/me/recommendations/impact?kind=equipment&id=11&replace=10", token)
	if status != 200 {
		t.Fatalf("强制装备 impact = %d %v", status, forced)
	}
	status, written = e.put(t, "/api/v1/me/equipment", token, map[string]any{"equipment_ids": []int{1, 11}})
	if status != 400 {
		t.Fatalf("未拥有的装备 11 应被写入端拒绝（400），实际 %d %v", status, written)
	}

	// ③ 授予真实归属后，同一接口才下发权威 id/replace，客户端门槛自然打开。
	e.exec(t, `INSERT INTO user_skills (user_id, skill_id) VALUES ($1, 2) ON CONFLICT DO NOTHING`, uid)
	e.exec(t, `INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		VALUES ($1, 11, 'gloves', FALSE) ON CONFLICT (user_id, equipment_id) DO NOTHING`, uid)
	status, body = e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != 200 {
		t.Fatalf("授予归属后 skills 推荐 = %d %v", status, body)
	}
	requireReachableImpact(t, e, token, "skill", recImpactParams(t, body, "skills"))
	status, body = e.get(t, "/api/v1/me/recommendations?context=equipment", token)
	if status != 200 {
		t.Fatalf("授予归属后 equipment 推荐 = %d %v", status, body)
	}
	requireReachableImpact(t, e, token, "equipment", recImpactParams(t, body, "equipment"))
}
