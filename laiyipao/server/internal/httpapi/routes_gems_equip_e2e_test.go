// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 宝石佩戴端点的端到端测试（第 20 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 佩戴 → equipped_slot 非空、/me/gems 可见；卸下 → 复原（幂等）；
//   - 别人的宝石 → 404；
//   - 超过佩戴上限 → 400。

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EGemEquipRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	for _, tc := range []struct {
		method, path string
	}{
		{"POST", "/api/v1/me/gems/1/equip"},
		{"POST", "/api/v1/me/gems/1/unequip"},
	} {
		req, _ := testReq(tc.method, tc.path, nil)
		resp, err := e.app.Test(req, 10000)
		if err != nil {
			t.Fatalf("%s %s 请求失败：%v", tc.method, tc.path, err)
		}
		defer resp.Body.Close()
		if resp.StatusCode != fiber.StatusUnauthorized {
			t.Errorf("%s %s 未认证应 401，实际 %d", tc.method, tc.path, resp.StatusCode)
		}
	}
}

func TestE2EGemEquipUnequipRoundtrip(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "gem_equip")

	// 直接造一颗宝石（商城限购不挡 e2e 流程）。
	var gemID int64
	if err := e.pool.QueryRow(context.Background(), `
		INSERT INTO user_gems (user_id, gem_id, quality, affixes)
		VALUES ($1, 1, 'rare', '[{"affix":"attack","value":10}]'::jsonb)
		RETURNING id`, uid).Scan(&gemID); err != nil {
		t.Fatalf("造宝石失败：%v", err)
	}

	authed := func(method, path string) (*http.Response, error) {
		req, _ := testReq(method, path, nil)
		req.Header.Set("Authorization", "Bearer "+tok)
		return e.app.Test(req, 10000)
	}

	// 佩戴
	resp, err := authed("POST", fmt.Sprintf("/api/v1/me/gems/%d/equip", gemID))
	if err != nil {
		t.Fatalf("equip 失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("equip 应 200，实际 %d", resp.StatusCode)
	}
	var wear struct {
		Gem struct {
			Equipped string `json:"equipped_slot"`
		} `json:"gem"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&wear); err != nil {
		t.Fatalf("解析 equip 响应失败：%v", err)
	}
	if wear.Gem.Equipped == "" {
		t.Fatal("佩戴后 equipped_slot 必须非空")
	}

	// 卸下（幂等：卸两次都 200）
	for i := 0; i < 2; i++ {
		resp2, err := authed("POST", fmt.Sprintf("/api/v1/me/gems/%d/unequip", gemID))
		if err != nil {
			t.Fatalf("unequip 失败：%v", err)
		}
		defer resp2.Body.Close()
		if resp2.StatusCode != fiber.StatusOK {
			t.Fatalf("unequip 应 200，实际 %d", resp2.StatusCode)
		}
	}

	// 列表确认复原
	var equipped string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT equipped_slot FROM user_gems WHERE id = $1`, gemID).Scan(&equipped); err != nil {
		t.Fatal(err)
	}
	if equipped != "" {
		t.Errorf("卸下后 equipped_slot 应为空，实际 %q", equipped)
	}
}

func TestE2EGemEquipForeignAndLimit(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	owner, _ := e.newPlayer(t, "gem_owner")
	attacker, tok := e.newPlayer(t, "gem_attacker")

	// owner 一颗 + attacker 8 颗（超过上限 6）。
	var ownerGem int64
	if err := e.pool.QueryRow(context.Background(), `
		INSERT INTO user_gems (user_id, gem_id, quality, affixes)
		VALUES ($1, 1, 'rare', '[{"affix":"attack","value":10}]'::jsonb)
		RETURNING id`, owner).Scan(&ownerGem); err != nil {
		t.Fatal(err)
	}
	for i := 0; i < 8; i++ {
		if _, err := e.pool.Exec(context.Background(), `
			INSERT INTO user_gems (user_id, gem_id, quality, affixes)
			VALUES ($1, 1, 'rare', '[{"affix":"attack","value":10}]'::jsonb)`, attacker); err != nil {
			t.Fatal(err)
		}
	}

	// 别人的宝石 → 404。
	req, _ := testReq("POST", fmt.Sprintf("/api/v1/me/gems/%d/equip", ownerGem), nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Errorf("佩戴他人宝石应 404，实际 %d", resp.StatusCode)
	}

	// 自己的 8 颗：前 6 颗成功，第 7 颗 400。
	for i := 0; i < 8; i++ {
		var id int64
		if err := e.pool.QueryRow(context.Background(),
			`SELECT id FROM user_gems WHERE user_id = $1 ORDER BY id LIMIT 1 OFFSET $2`,
			attacker, i).Scan(&id); err != nil {
			t.Fatal(err)
		}
		req2, _ := testReq("POST", fmt.Sprintf("/api/v1/me/gems/%d/equip", id), nil)
		req2.Header.Set("Authorization", "Bearer "+tok)
		resp2, err := e.app.Test(req2, 10000)
		if err != nil {
			t.Fatal(err)
		}
		defer resp2.Body.Close()
		want := fiber.StatusOK
		if i >= 6 {
			want = fiber.StatusBadRequest
		}
		if resp2.StatusCode != want {
			t.Errorf("第 %d 颗佩戴应 %d，实际 %d", i+1, want, resp2.StatusCode)
		}
	}
}
