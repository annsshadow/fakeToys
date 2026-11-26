// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R6 §1E：真实数据驱动的推荐接口 E2E。
//
// 这些用例特意**不**只检查字段存在：它们先制造可核对的真实战报，再断言
// 响应里的胜率/反应名来自那些行；空库与越权同样是契约的一部分。
// 接口尚未实现时本文件必须 RED。

import (
	"fmt"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/config"
)

func requireRecEnvelope(t *testing.T, body map[string]any, wantContext string) {
	t.Helper()
	if body["schema_version"] != "rec-v1" {
		t.Fatalf("schema_version = %v，期望 rec-v1", body["schema_version"])
	}
	if body["context"] != wantContext {
		t.Fatalf("context = %v，期望 %s", body["context"], wantContext)
	}
	if s, _ := body["generated_at"].(string); len(s) < 20 {
		t.Fatalf("generated_at 不是 ISO8601：%q", s)
	}
	subject, ok := body["subject"].(map[string]any)
	if !ok {
		t.Fatalf("subject 缺失或类型错误：%#v", body["subject"])
	}
	key, _ := subject["user_key"].(string)
	if len(key) < 8 || strings.Contains(key, "e2e_") {
		t.Fatalf("user_key 必须是 hash，不能是账号名：%q", key)
	}
	snap, ok := body["input_snapshot"].(map[string]any)
	if !ok {
		t.Fatalf("input_snapshot 缺失：%#v", body["input_snapshot"])
	}
	if _, ok := snap["config_version"]; !ok {
		t.Fatalf("input_snapshot.config_version 缺失：%#v", snap)
	}
	if _, ok := snap["battle_count"]; !ok {
		t.Fatalf("input_snapshot.battle_count 缺失：%#v", snap)
	}
	if _, ok := snap["window_days"]; !ok {
		t.Fatalf("input_snapshot.window_days 缺失：%#v", snap)
	}
	if _, ok := body["model"].(map[string]any); !ok {
		t.Fatalf("model 缺失：%#v", body["model"])
	}
	if _, ok := body["evidence"].([]any); !ok {
		t.Fatalf("evidence 必须是数组：%#v", body["evidence"])
	}
	if _, ok := body["insights"].([]any); !ok {
		t.Fatalf("insights 必须是数组：%#v", body["insights"])
	}
}

func TestE2EMeRecommendationsFromRealBattles(t *testing.T) {
	e := newE2E(t)
	uid, token := e.newPlayer(t, "rec")

	// 1) 零战报：必须诚实空态，不得回退写死推荐。
	status, body := e.get(t, "/api/v1/me/recommendations?context=home", token)
	if status != 200 {
		t.Fatalf("空历史 GET recommendations = %d %v", status, body)
	}
	requireRecEnvelope(t, body, "home")
	if num(body["input_snapshot"].(map[string]any), "battle_count") != 0 {
		t.Fatalf("空历史 battle_count 应为 0：%v", body["input_snapshot"])
	}
	if body["empty_reason"] == nil {
		t.Fatalf("空历史必须给 empty_reason：%v", body)
	}
	reason := body["empty_reason"].(map[string]any)
	if reason["code"] != "NO_HISTORY" {
		t.Fatalf("空历史 code = %v，期望 NO_HISTORY", reason["code"])
	}
	if len(body["insights"].([]any)) != 0 {
		t.Fatalf("空历史 insights 必须为空：%v", body["insights"])
	}

	// 2) 真实战报：一胜一负、触发 steam_burst 两次。
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, reactions, elements_used, reactions_used, created_at)
		VALUES ($1, 5, 'win', 3, 10, 2,
		        '{"fire":2,"ice":1}'::jsonb, '{"steam_burst":2}'::jsonb, now())`, uid)
	e.exec(t, `INSERT INTO battle_records
		(user_id, level_id, result, stars, wave_reached, reactions, elements_used, reactions_used, created_at)
		VALUES ($1, 6, 'lose', 0, 4, 0,
		        '{}'::jsonb, '{}'::jsonb, now())`, uid)

	status, body = e.get(t, "/api/v1/me/recommendations?context=home", token)
	if status != 200 {
		t.Fatalf("有历史 GET recommendations = %d %v", status, body)
	}
	requireRecEnvelope(t, body, "home")
	if num(body["input_snapshot"].(map[string]any), "battle_count") != 2 {
		t.Fatalf("battle_count 应来自真实战报 =2：%v", body["input_snapshot"])
	}
	if body["empty_reason"] != nil {
		t.Fatalf("有历史时 empty_reason 应为 null：%v", body["empty_reason"])
	}
	insights := body["insights"].([]any)
	if len(insights) == 0 {
		t.Fatalf("有历史必须有洞察：%v", body)
	}
	evidence := body["evidence"].([]any)
	if len(evidence) == 0 {
		t.Fatalf("有历史必须有证据：%v", body)
	}
	winRateOK := false
	for _, raw := range evidence {
		ev, _ := raw.(map[string]any)
		metric, _ := ev["metric"].(string)
		value, _ := ev["value"].(float64)
		if strings.Contains(metric, "胜率") && value == 50 {
			winRateOK = true
		}
	}
	if !winRateOK {
		t.Fatalf("evidence 必须含真实胜率 50%%：%v", evidence)
	}
	first, _ := insights[0].(map[string]any)
	for _, field := range []string{"id", "headline"} {
		if s, _ := first[field].(string); s == "" {
			t.Fatalf("insight.%s 缺失：%v", field, first)
		}
	}
	if basis, ok := first["basis"].([]any); !ok || len(basis) == 0 {
		t.Fatalf("insight.basis 必须是非空数组：%v", first)
	}
	cta, ok := first["cta"].(map[string]any)
	if !ok || cta["label"] == "" || cta["route"] == "" {
		t.Fatalf("insight.cta 必须带 label/route：%v", first)
	}
	if first["confidence"] != "rule_based" && first["confidence"] != "statistical" {
		t.Fatalf("confidence 只能是文字标记：%v", first["confidence"])
	}

	// 3) 反应页应从 reactions_used 里挑出真实 top 反应，而不是写死文案。
	status, body = e.get(t, "/api/v1/me/recommendations?context=reactions", token)
	if status != 200 {
		t.Fatalf("reactions GET = %d %v", status, body)
	}
	joined := fmt.Sprint(body["insights"])
	if !strings.Contains(joined, "steam_burst") {
		t.Fatalf("反应洞察必须引用真实 reactions_used 键：%v", body["insights"])
	}

	// 4) 非法 context 与未登录都是硬失败，不能降级成默认页。
	status, _ = e.get(t, "/api/v1/me/recommendations?context=not_a_page", token)
	if status != 400 {
		t.Fatalf("非法 context 应 400，实际 %d", status)
	}
	status, _ = e.get(t, "/api/v1/me/recommendations?context=home", "")
	if status != 401 {
		t.Fatalf("未登录应 401，实际 %d", status)
	}
}

func TestE2EAdminRecommendationsFromDashboard(t *testing.T) {
	e := newE2EWith(t, func(cfg *config.Config) { cfg.DashboardCacheTTL = 0 })
	_, token := e.newAdmin(t, "recadm")
	status, body := e.get(t, "/api/v1/admin/recommendations/battles", token)
	if status != 200 {
		t.Fatalf("空看板 GET recommendations = %d %v", status, body)
	}
	requireRecEnvelope(t, body, "battles")

	// 制造一条真实战报后立即重读（DashboardCacheTTL=0）。
	e.exec(t, `INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash)
		VALUES ((SELECT id FROM users ORDER BY id LIMIT 1), 5, 'win', 3, 'recadmtest')`)
	status, body = e.get(t, "/api/v1/admin/recommendations/battles", token)
	if status != 200 {
		t.Fatalf("有战报 GET recommendations = %d %v", status, body)
	}
	requireRecEnvelope(t, body, "battles")
	if num(body["input_snapshot"].(map[string]any), "battle_count") < 1 {
		t.Fatalf("admin battle_count 必须来自看板真实战报：%v", body["input_snapshot"])
	}
	if len(body["insights"].([]any)) == 0 || len(body["evidence"].([]any)) == 0 {
		t.Fatalf("有数据时必须给洞察与证据：%v", body)
	}
	for _, scope := range []string{"dashboard", "levels", "economy", "skills", "users", "recomputes", "content", "defenses", "equipment", "wiki"} {
		status, body = e.get(t, "/api/v1/admin/recommendations/"+scope, token)
		if status != 200 {
			t.Fatalf("scope=%s 返回 %d %v", scope, status, body)
		}
		requireRecEnvelope(t, body, scope)
	}
	status, _ = e.get(t, "/api/v1/admin/recommendations/not_a_scope", token)
	if status != 400 {
		t.Fatalf("非法 scope 应 400，实际 %d", status)
	}
	status, _ = e.get(t, "/api/v1/admin/recommendations/battles", "")
	if status != 401 {
		t.Fatalf("未登录 admin 应 401，实际 %d", status)
	}
}
