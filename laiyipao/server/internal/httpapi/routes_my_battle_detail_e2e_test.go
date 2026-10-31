package httpapi

// 「我的战报详情」端点的端到端测试（2026-10-31 新一轮第 10 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 本人战报 → 200 + 完整详情（掉落/元素/反应/地形）；
//   - 别人的战报 → 404（与不存在同口径，不泄露存在性）。

import (
	"context"
	"encoding/json"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMyBattleDetailRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/me/battles/1", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EMyBattleDetailSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "bdetail")
	var battleID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, stars, score, kills, leaked,
		                             duration_ms, shots, hits, reactions, heat_max,
		                             elements_used, reactions_used, terrain_used, loot, replay_hash)
		 VALUES ($1, 7, 'win', 3, 1200, 15, 0, 12000, 250, 240, 6, 90,
		         '{"fire":10}'::jsonb, '{"steam_burst":3}'::jsonb, '["lava_pool"]'::jsonb,
		         '{"coin":900}'::jsonb, 'hash123')
		 RETURNING id`, uid).Scan(&battleID); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/battles/"+fmtInt(battleID), nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	var body map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d", resp.StatusCode)
	}
	if body["level_id"] != float64(7) || body["result"] != "win" || body["stars"] != float64(3) {
		t.Errorf("基本字段不完整：%v", body)
	}
	loot, ok := body["loot"].(map[string]any)
	if !ok || loot["coin"] != float64(900) {
		t.Errorf("loot 应含 coin:900，实际 %v", body["loot"])
	}
	elems, ok := body["elements_used"].(map[string]any)
	if !ok || elems["fire"] != float64(10) {
		t.Errorf("elements_used 应含 fire:10，实际 %v", body["elements_used"])
	}
	terrain, ok := body["terrain_used"].([]any)
	if !ok || len(terrain) != 1 || terrain[0] != "lava_pool" {
		t.Errorf("terrain_used 应为 [lava_pool]，实际 %v", body["terrain_used"])
	}
}

func TestE2EMyBattleDetailOwnershipIsolation(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	ownerID, _ := e.newPlayer(t, "bdetail_owner")
	_, otherTok := e.newPlayer(t, "bdetail_other")
	var battleID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash)
		 VALUES ($1, 1, 'win', 3, 'h')
		 RETURNING id`, ownerID).Scan(&battleID); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/battles/"+fmtInt(battleID), nil)
	req.Header.Set("Authorization", "Bearer "+otherTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Fatalf("别人的战报应 404，实际 %d", resp.StatusCode)
	}
}

// fmtInt 把 int64 转十进制字符串（避免 strconv 导入的噪音）。
func fmtInt(n int64) string {
	if n == 0 {
		return "0"
	}
	var buf [20]byte
	i := len(buf)
	for n > 0 {
		i--
		buf[i] = byte('0' + n%10)
		n /= 10
	}
	return string(buf[i:])
}