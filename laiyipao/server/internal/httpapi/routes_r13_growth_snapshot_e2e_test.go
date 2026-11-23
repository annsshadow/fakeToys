// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R13：冷启动成长候选 + 升级战力前后快照。
// 真实 Service + 真实 PostgreSQL + Fiber app.Test；先 RED 再实现。

import (
	"fmt"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/domain"
)

func coldStartOf(t *testing.T, body map[string]any) map[string]any {
	t.Helper()
	cs, ok := body["cold_start"].(map[string]any)
	if !ok {
		t.Fatalf("响应缺少 cold_start 对象：%v", body)
	}
	return cs
}

func TestE2EColdStartGrowthCandidates(t *testing.T) {
	e := newE2E(t)
	uid, token := e.newPlayer(t, "r13_coldstart")

	// 0 场战报：现有 rec-v1 空态保持，但必须附规则候选。
	status, body := e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != fiber.StatusOK {
		t.Fatalf("冷启动推荐 = %d %v", status, body)
	}
	reason, _ := body["empty_reason"].(map[string]any)
	if reason == nil || reason["code"] != "NO_HISTORY" {
		t.Fatalf("零战报仍应保留 NO_HISTORY 诚实空态：%v", body["empty_reason"])
	}
	cs := coldStartOf(t, body)
	if !strings.Contains(fmt.Sprint(cs["label"]), "基于规则而非你的历史战报") {
		t.Fatalf("cold_start 必须明确标注规则来源，实际 label=%v", cs["label"])
	}
	if num(cs, "battle_count") != 0 || num(cs, "threshold") != 3 {
		t.Fatalf("冷启动阈值/战报数不真实：battle_count=%v threshold=%v", cs["battle_count"], cs["threshold"])
	}
	cfgVer := fmt.Sprint(cs["config_version"])
	if cfgVer == "" || cfgVer == "<nil>" {
		t.Fatalf("cold_start 必须带真实配置版本：%v", cs["config_version"])
	}
	candidates, ok := cs["candidates"].([]any)
	if !ok || len(candidates) == 0 {
		t.Fatalf("冷启动必须给可执行候选：%v", cs["candidates"])
	}
	for i, raw := range candidates {
		c, _ := raw.(map[string]any)
		if c == nil || fmt.Sprint(c["id"]) == "" || fmt.Sprint(c["headline"]) == "" {
			t.Fatalf("候选 %d 缺 id/headline：%v", i, c)
		}
		if basis, ok := c["basis"].([]any); !ok || len(basis) == 0 {
			t.Fatalf("候选 %d 缺真实依据：%v", i, c["basis"])
		}
		cta, _ := c["cta"].(map[string]any)
		if cta == nil || fmt.Sprint(cta["route"]) == "" {
			t.Fatalf("候选 %d 缺 CTA：%v", i, c["cta"])
		}
		if params, ok := cta["params"].(map[string]any); !ok || len(params) == 0 {
			t.Fatalf("候选 %d 的 CTA 不可执行（params 为空）：%v", i, cta["params"])
		}
		src, _ := c["source"].(map[string]any)
		if src == nil || fmt.Sprint(src["rule"]) == "" || fmt.Sprint(src["config_version"]) != cfgVer {
			t.Fatalf("候选 %d 缺规则名/配置版本来源：%v", i, c["source"])
		}
	}

	// 1~2 场：仍在冷启动阈值内。
	for i := 0; i < 2; i++ {
		e.exec(t, `INSERT INTO battle_records
   (user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
   VALUES ($1, 5, 'win', 3, 10, 120, 8, now())`, uid)
	}
	status, body = e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != fiber.StatusOK {
		t.Fatalf("2 场战报推荐 = %d %v", status, body)
	}
	cs = coldStartOf(t, body)
	if num(cs, "battle_count") != 2 {
		t.Fatalf("2 场战报时 battle_count 应为 2：%v", cs)
	}

	// 第 3 场：回到既有 rec-v1 行为，不再下发冷启动候选。
	e.exec(t, `INSERT INTO battle_records
  (user_id, level_id, result, stars, wave_reached, hits, reactions, created_at)
  VALUES ($1, 5, 'win', 3, 10, 120, 8, now())`, uid)
	status, body = e.get(t, "/api/v1/me/recommendations?context=skills", token)
	if status != fiber.StatusOK {
		t.Fatalf("3 场战报推荐 = %d %v", status, body)
	}
	if body["cold_start"] != nil {
		t.Fatalf("达到阈值后不应再下发 cold_start：%v", body["cold_start"])
	}
	if insights, ok := body["insights"].([]any); !ok || len(insights) == 0 {
		t.Fatalf("有历史后应恢复既有 insights：%v", body["insights"])
	}
}

func TestE2EUpgradeSkillPowerSnapshot(t *testing.T) {
	e := newE2E(t)
	uid, token := e.newPlayer(t, "r13_power")
	status, beforeBody := e.get(t, "/api/v1/me/", token)
	if status != fiber.StatusOK {
		t.Fatalf("me profile = %d %v", status, beforeBody)
	}
	beforePower := int64(num(beforeBody, "power"))

	cost := domain.DefaultSkillRules().CostFrom(1)
	e.exec(t, `UPDATE user_wallets SET coin = $2 WHERE user_id = $1`, uid, cost+500)
	status, body := e.post(t, "/api/v1/me/skills/1/upgrade", token, nil)
	if status != fiber.StatusOK {
		t.Fatalf("技能升级 = %d %v", status, body)
	}
	snap, ok := body["power_snapshot"].(map[string]any)
	if !ok {
		t.Fatalf("升级响应缺少 power_snapshot：%v", body)
	}
	if snap["applicable"] != true {
		t.Fatalf("技能升级应有战力公式，applicable=true：%v", snap)
	}
	if int64(num(snap, "before")) != beforePower {
		t.Fatalf("power_before 应来自同一战力函数（me.profile power=%d），实际 %v", beforePower, snap["before"])
	}
	if int64(num(snap, "after")) <= int64(num(snap, "before")) {
		t.Fatalf("升级后战力应上升：%v", snap)
	}
	if !strings.Contains(fmt.Sprint(snap["method"]), "ComputePower") {
		t.Fatalf("必须说明战力计算口径：%v", snap["method"])
	}
	if int64(num(body, "power_before")) != beforePower {
		t.Fatalf("契约字段 power_before 必须与同一战力函数一致：%v", body["power_before"])
	}
	if int64(num(body, "power_after")) <= int64(num(body, "power_before")) {
		t.Fatalf("契约字段 power_after 必须反映升级后的真实战力：%v", body)
	}
	if !strings.Contains(fmt.Sprint(body["power_method"]), "ComputePower") {
		t.Fatalf("契约字段 power_method 必须说明计算口径：%v", body["power_method"])
	}
}

func TestE2ECosmeticUpgradePowerNotApplicable(t *testing.T) {
	e := newE2E(t)
	uid, token := e.newPlayer(t, "r13_cosmetic")
	// 游客转正是账号面变更，不改构筑；战力必须诚实标注不适用，不能填 0。
	status, body := e.post(t, "/api/v1/me/upgrade", token, map[string]any{
		"username": fmt.Sprintf("r13_%d", uid),
		"password": "r13-test-only-password",
	})
	if status != fiber.StatusOK {
		t.Fatalf("账号升级 = %d %v", status, body)
	}
	snap, ok := body["power_snapshot"].(map[string]any)
	if !ok {
		t.Fatalf("账号升级响应缺少 power_snapshot：%v", body)
	}
	if snap["applicable"] != false || snap["before"] != nil || snap["after"] != nil {
		t.Fatalf("无战力公式的升级必须 before/after=null 且 applicable=false：%v", snap)
	}
	if !strings.Contains(fmt.Sprint(snap["note"]), "不适用") {
		t.Fatalf("不适用分支必须给出说明：%v", snap["note"])
	}
	if body["power_before"] != nil || body["power_after"] != nil {
		t.Fatalf("无战力公式的升级不得编造 power_before/after：%v", body)
	}
	if !strings.Contains(fmt.Sprint(body["power_method"]), "not_applicable") {
		t.Fatalf("不适用分支的 power_method 必须标注 not_applicable：%v", body["power_method"])
	}
}
