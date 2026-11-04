// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R96 P1：把此前 0% 覆盖的六个玩家侧端点接到真实链路上。
//
// 这六个 handler 都只有「解析 → 调 service → 转响应」几行，但它们此前
// **一次都没被 e2e 走过**：`/me`、`/me/logout`、`/me/gems`、
// `/me/equipment`（GET/PUT）、`/battle/abandon`、`/me/skills/synthesize`。
// 对薄层来说，「没被测过」的典型后果不是计算错，而是**路由没挂上、
// JSON tag 拼错、空切片序列化成 null** —— 这些恰好只有真 app 才抓得住。
//
// 断言落点都是玩家能观察到的后果：登出后凭证真的换了、宝石真的列得出、
// 装备真的能整套替换、放弃战斗真的退体力、熔炼真的消耗材料并发放产物。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// findItem 在读模型响应里按某个整型字段找一项。
func findItem(t *testing.T, items []any, key string, want int64) map[string]any {
	t.Helper()
	for _, raw := range items {
		m, ok := raw.(map[string]any)
		if !ok {
			continue
		}
		if v, ok := m[key].(float64); ok && int64(v) == want {
			return m
		}
	}
	return nil
}

func TestE2EMeProfileAndLogoutRotatesCredential(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "me")

	status, body := e.get(t, "/api/v1/me", tok)
	if status != fiber.StatusOK {
		t.Fatalf("GET /me = %d %v", status, body)
	}
	if num(body, "user_id") != uid {
		t.Fatalf("/me 的 user_id 应为 %d，实际 %v", uid, body["user_id"])
	}
	if _, ok := body["max_stage"].(float64); !ok {
		t.Errorf("/me 应返回 max_stage（未通关的新号为 0），实际 %T", body["max_stage"])
	}
	build, ok := body["build"].(map[string]any)
	if !ok || build == nil {
		t.Fatalf("/me 应返回 build 快照（map），实际 %T %v", body["build"], body["build"])
	}
	if _, ok := body["build_rating"].(map[string]any); !ok {
		t.Errorf("/me 应返回 build_rating，实际 %T", body["build_rating"])
	}
	if power := num(body, "power"); power <= 0 {
		t.Errorf("带默认技能的新号战力应 > 0，实际 %d", power)
	}
	if _, ok := body["account"].(map[string]any); !ok {
		t.Errorf("/me 应返回 account（注册/最后登录），实际 %T", body["account"])
	}

	// 登出必须**轮换游客凭证**：旧凭证留着等于没登出（拿它还能换到同一账号的令牌）。
	var before string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT guest_token FROM users WHERE id = $1`, uid).Scan(&before); err != nil {
		t.Fatalf("读登出前 guest_token 失败：%v", err)
	}

	status, body = e.post(t, "/api/v1/me/logout", tok, nil)
	if status != fiber.StatusOK {
		t.Fatalf("POST /me/logout = %d %v", status, body)
	}
	issued, _ := body["guest_token"].(string)
	if issued == "" {
		t.Fatal("登出响应里必须有新的 guest_token（客户端要存下来）")
	}
	if num(body, "revoked") < 1 {
		t.Errorf("登录过就会有 refresh token，登出应至少吊销 1 条，实际 %v", body["revoked"])
	}

	var after string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT guest_token FROM users WHERE id = $1`, uid).Scan(&after); err != nil {
		t.Fatalf("读登出后 guest_token 失败：%v", err)
	}
	if after == before {
		t.Fatal("登出后 guest_token 必须变 —— 没变等于旧凭证仍可换令牌")
	}
	if issued != after {
		t.Errorf("响应里的 guest_token 应与库里一致：响应 %q，库 %q", issued, after)
	}
}

func TestE2EGemsListedAndEmptyIsArray(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "gems")

	// 没有宝石时必须是 `[]` 而不是 `null`：客户端 `gems.map()` 会在 null 上炸。
	status, body := e.get(t, "/api/v1/me/gems", tok)
	if status != fiber.StatusOK {
		t.Fatalf("GET /me/gems = %d %v", status, body)
	}
	gems, ok := body["gems"].([]any)
	if !ok {
		t.Fatalf("空宝石列表应为 JSON 数组，实际 %T %v", body["gems"], body["gems"])
	}
	if len(gems) != 0 {
		t.Fatalf("新号不该有宝石，实际 %d 颗", len(gems))
	}

	var gemID int64
	if err := e.pool.QueryRow(context.Background(), `
		INSERT INTO user_gems (user_id, gem_id, quality, affixes)
		VALUES ($1, 1, 'rare', '[{"affix":"attack","value":10}]'::jsonb)
		RETURNING id`, uid).Scan(&gemID); err != nil {
		t.Fatalf("造宝石失败：%v", err)
	}

	status, body = e.get(t, "/api/v1/me/gems", tok)
	if status != fiber.StatusOK {
		t.Fatalf("GET /me/gems = %d %v", status, body)
	}
	gems, ok = body["gems"].([]any)
	if !ok {
		t.Fatalf("宝石列表应为数组，实际 %T", body["gems"])
	}
	item := findItem(t, gems, "id", gemID)
	if item == nil {
		t.Fatalf("列表里找不到刚造的宝石 %d：%v", gemID, gems)
	}
	if name, _ := item["name"].(string); name == "" {
		t.Errorf("宝石应带内容表里的名字，实际 %v", item["name"])
	}
	affixes, ok := item["affixes"].([]any)
	if !ok {
		t.Fatalf("affixes 应序列化成数组（脏 null 会让客户端整页崩），实际 %T", item["affixes"])
	}
	if len(affixes) != 1 {
		t.Errorf("应回 1 条词条，实际 %d", len(affixes))
	}
}

func TestE2EEquipmentCatalogAndSave(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "equip")

	// 挑两件**该玩家尚未拥有**的装备（新号礼包会发初始装备，
	// 直接用目录第一件会撞主键，且会让 owned 断言失去意义）。
	var eqID int64
	var slot string
	if err := e.pool.QueryRow(context.Background(), `
		SELECT e.id, e.slot FROM equipment e
		 WHERE NOT EXISTS (SELECT 1 FROM user_equipment ue
		                    WHERE ue.user_id = $1 AND ue.equipment_id = e.id)
		 ORDER BY e.id LIMIT 1`, uid).Scan(&eqID, &slot); err != nil {
		t.Fatalf("找不到未拥有的装备（种子可能缺失）：%v", err)
	}
	var otherID int64
	if err := e.pool.QueryRow(context.Background(), `
		SELECT e.id FROM equipment e
		 WHERE e.id <> $2
		   AND NOT EXISTS (SELECT 1 FROM user_equipment ue
		                    WHERE ue.user_id = $1 AND ue.equipment_id = e.id)
		 ORDER BY e.id LIMIT 1`, uid, eqID).Scan(&otherID); err != nil {
		t.Fatalf("找不到第二件未拥有的装备：%v", err)
	}

	// 给玩家发这一件（其余目录项仍应是 owned=false）。
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO user_equipment (user_id, equipment_id, slot) VALUES ($1,$2,$3)`,
		uid, eqID, slot); err != nil {
		t.Fatalf("发装备失败：%v", err)
	}

	status, body := e.get(t, "/api/v1/me/equipment", tok)
	if status != fiber.StatusOK {
		t.Fatalf("GET /me/equipment = %d %v", status, body)
	}
	items, ok := body["items"].([]any)
	if !ok {
		t.Fatalf("items 应为数组，实际 %T", body["items"])
	}
	mine := findItem(t, items, "id", eqID)
	if mine == nil {
		t.Fatalf("目录里找不到 %d", eqID)
	}
	if owned, _ := mine["owned"].(bool); !owned {
		t.Errorf("刚发的装备应 owned=true，实际 %v", mine["owned"])
	}
	if equipped, _ := mine["equipped"].(bool); equipped {
		t.Errorf("还没装配，equipped 应为 false，实际 %v", mine["equipped"])
	}
	if notOwned := findItem(t, items, "id", otherID); notOwned != nil {
		if owned, _ := notOwned["owned"].(bool); owned {
			t.Errorf("未发放的装备 %d 不该 owned=true", otherID)
		}
	}

	// 整组替换：装上这一件
	status, body = e.put(t, "/api/v1/me/equipment", tok, map[string]any{
		"equipment_ids": []int{int(eqID)},
	})
	if status != fiber.StatusOK {
		t.Fatalf("PUT /me/equipment = %d %v", status, body)
	}
	items, _ = body["items"].([]any)
	if got := findItem(t, items, "id", eqID); got == nil {
		t.Fatalf("装配响应里找不到 %d", eqID)
	} else if equipped, _ := got["equipped"].(bool); !equipped {
		t.Errorf("装配后 equipped 应为 true，实际 %v", got["equipped"])
	}

	// 目录外的 id → 400
	if status, body := e.put(t, "/api/v1/me/equipment", tok, map[string]any{
		"equipment_ids": []int{9999},
	}); status != fiber.StatusBadRequest {
		t.Fatalf("目录外装备应 400，实际 %d %v", status, body)
	}
	// 目录里但没拥有 → 400（不能凭空装配）
	if status, body := e.put(t, "/api/v1/me/equipment", tok, map[string]any{
		"equipment_ids": []int{int(otherID)},
	}); status != fiber.StatusBadRequest {
		t.Fatalf("未拥有的装备应 400，实际 %d %v", status, body)
	}
}

func TestE2EAbandonBattleRefundsEnergyOnce(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newPlayer(t, "abandon")

	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusOK {
		t.Fatalf("开局失败 = %d %v", status, body)
	}
	tokenID := num(body, "token_id")
	if tokenID <= 0 {
		t.Fatalf("开局应返回 token_id，实际 %v", body)
	}

	status, body = e.post(t, "/api/v1/battle/abandon", tok, map[string]any{"token_id": tokenID})
	if status != fiber.StatusOK {
		t.Fatalf("放弃战斗 = %d %v", status, body)
	}
	if abandoned, _ := body["abandoned"].(bool); !abandoned {
		t.Errorf("放弃响应应 abandoned=true，实际 %v", body["abandoned"])
	}
	if refunded := num(body, "refunded_energy"); refunded <= 0 {
		t.Errorf("放弃应退回本局消耗的体力，实际 %d", refunded)
	}

	// 同一局不能退两次 —— 否则刷体力。
	if status, body := e.post(t, "/api/v1/battle/abandon", tok, map[string]any{
		"token_id": tokenID,
	}); status != fiber.StatusBadRequest {
		t.Fatalf("重复放弃应 400，实际 %d %v", status, body)
	}
}

func TestE2ESynthesizeSkillsConsumesMaterials(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "synth")

	// 配方从库里取（与 internal/service 的 synthesis_test 同口径）。
	var aID, bID, outID, need int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT a_skill_id, b_skill_id, output_skill_id FROM skill_recipes
		  WHERE output_skill_id = 31`).Scan(&aID, &bID, &outID); err != nil {
		t.Fatalf("读配方 31 失败（种子可能缺失）：%v", err)
	}
	if err := e.pool.QueryRow(context.Background(),
		`SELECT unlock_level FROM skills WHERE id = $1`, outID).Scan(&need); err != nil {
		t.Fatalf("读产物等级门槛失败：%v", err)
	}
	// 两份材料都要有（新号礼包未必包含配方里的那一枚）。
	if _, err := e.pool.Exec(context.Background(), `
		INSERT INTO user_skills (user_id, skill_id) VALUES ($1,$2),($1,$3)
		ON CONFLICT (user_id, skill_id) DO NOTHING`, uid, aID, bID); err != nil {
		t.Fatalf("发材料技能失败：%v", err)
	}
	// 等级 = level_exp/100，恰好达标。
	if _, err := e.pool.Exec(context.Background(),
		`UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, need*100); err != nil {
		t.Fatalf("设置等级失败：%v", err)
	}

	status, body := e.post(t, "/api/v1/me/skills/synthesize", tok, map[string]any{
		"a": aID, "b": bID,
	})
	if status != fiber.StatusOK {
		t.Fatalf("熔炼 = %d %v", status, body)
	}
	if got := num(body, "output"); got != outID {
		t.Fatalf("产物应为 %d，实际 %d（%v）", outID, got, body)
	}
	if _, ok := body["build"].(map[string]any); !ok {
		t.Errorf("熔炼应回带刷新后的 build 快照，实际 %T", body["build"])
	}

	// 材料必须真的被熔掉，产物真的到账。
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id=$1 AND skill_id=$2`, uid, aID); got != 0 {
		t.Errorf("材料 %d 应被消耗，实际仍有 %d 条", aID, got)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id=$1 AND skill_id=$2`, uid, bID); got != 0 {
		t.Errorf("材料 %d 应被消耗，实际仍有 %d 条", bID, got)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM user_skills WHERE user_id=$1 AND skill_id=$2`, uid, outID); got != 1 {
		t.Errorf("产物 %d 应到账 1 条，实际 %d", outID, got)
	}

	// 非法输入：同一个技能不能自己熔自己；无配方组合必须 400。
	if status, body := e.post(t, "/api/v1/me/skills/synthesize", tok, map[string]any{
		"a": aID, "b": aID,
	}); status != fiber.StatusBadRequest {
		t.Fatalf("同 id 熔炼应 400，实际 %d %v", status, body)
	}
	if status, body := e.post(t, "/api/v1/me/skills/synthesize", tok, map[string]any{
		"a": 1000, "b": 1001,
	}); status != fiber.StatusBadRequest {
		t.Fatalf("无配方组合应 400，实际 %d %v", status, body)
	}
}
