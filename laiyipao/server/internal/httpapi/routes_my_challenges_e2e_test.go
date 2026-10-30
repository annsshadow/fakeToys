package httpapi

// 玩家挑战历史端点的端到端测试（2026-10-30 实质优化第 18 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 有记录 → 200 + items 包含挑战历史（对手昵称/胜负/偷取）；
//   - 无记录 → 200 + 空 items。

import (
	"context"
	"encoding/json"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMyChallengeHistoryRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/me/challenges", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EMyChallengeHistorySucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	ownerID, ownerTok := e.newPlayer(t, "chist_owner")
	meID, meTok := e.newPlayer(t, "chist_me")

	// 防线（owner）
	var defID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '铁壁', 999, '{}'::jsonb, 'snap')
		 RETURNING id`, ownerID).Scan(&defID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}
	// 挑战记录（me 挑战 owner）
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms, hp_left_pct, loot, settled)
		 VALUES ($1, $2, 1, TRUE, 1500, 90, '{"gem":5}'::jsonb, TRUE)`,
		defID, meID); err != nil {
		t.Fatalf("插入挑战记录失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/challenges", nil)
	req.Header.Set("Authorization", "Bearer "+meTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	var body struct {
		Items []map[string]any `json:"items"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d", resp.StatusCode)
	}
	if len(body.Items) != 1 {
		t.Fatalf("应有 1 条记录，实际 %d", len(body.Items))
	}
	item := body.Items[0]
	if item["won"] != true {
		t.Errorf("won 应为 true，实际 %v", item["won"])
	}
	if item["defense_name"] != "铁壁" {
		t.Errorf("defense_name 应为 铁壁，实际 %v", item["defense_name"])
	}
	if item["owner_power"] != float64(999) {
		t.Errorf("owner_power 应为 999，实际 %v", item["owner_power"])
	}
	loot, ok := item["loot"].(map[string]any)
	if !ok || loot["gem"] != float64(5) {
		t.Errorf("loot 应为 {gem:5}，实际 %v", item["loot"])
	}

	// owner 视角也有权限边界：owner 的挑战历史是空的（他没挑战过自己）
	req2, _ := http.NewRequest("GET", "/api/v1/me/challenges", nil)
	req2.Header.Set("Authorization", "Bearer "+ownerTok)
	resp2, err := e.app.Test(req2, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp2.Body.Close()
	var body2 struct {
		Items []map[string]any `json:"items"`
	}
	if err := json.NewDecoder(resp2.Body).Decode(&body2); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if len(body2.Items) != 0 {
		t.Errorf("owner 无挑战记录，应返回空 items，实际 %d", len(body2.Items))
	}
}

func TestE2EMyChallengeHistoryEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "chist_empty")
	req, _ := http.NewRequest("GET", "/api/v1/me/challenges", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	var body struct {
		Items []any `json:"items"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if body.Items == nil || len(body.Items) != 0 {
		t.Errorf("无记录应返回空 items，实际 %v", body.Items)
	}
}
