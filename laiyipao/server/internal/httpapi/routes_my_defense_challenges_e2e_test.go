package httpapi

// 「我的防线被挑战」历史端点的端到端测试（2026-10-31 新一轮第 3 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 有记录 → 200 + items 包含被挑战记录（挑战者昵称/胜负/偷取）；
//   - 无记录 → 200 + 空 items。

import (
	"context"
	"encoding/json"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMyDefenseChallengesRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/me/defense/challenges", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EMyDefenseChallengesSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	ownerID, ownerTok := e.newPlayer(t, "defch_owner")
	challengerID, _ := e.newPlayer(t, "defch_challenger")

	// 防线（owner）
	var defID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '我家防线', 999, '{}'::jsonb, 'snap')
		 RETURNING id`, ownerID).Scan(&defID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}
	// 挑战记录（challenger 挑战 owner，挑战者获胜并偷到宝石）
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms, hp_left_pct, loot, settled)
		 VALUES ($1, $2, 1, TRUE, 1500, 90, '{"gem":5}'::jsonb, TRUE)`,
		defID, challengerID); err != nil {
		t.Fatalf("插入挑战记录失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/defense/challenges", nil)
	req.Header.Set("Authorization", "Bearer "+ownerTok)
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
		t.Errorf("won 应为 true（挑战者获胜），实际 %v", item["won"])
	}
	if item["defense_name"] != "我家防线" {
		t.Errorf("defense_name = %v，期望 我家防线", item["defense_name"])
	}
	if item["challenger_nickname"] == "" || item["challenger_nickname"] == nil {
		t.Error("challenger_nickname 不应为空")
	}
	loot, ok := item["loot"].(map[string]any)
	if !ok || loot["gem"] != float64(5) {
		t.Errorf("loot 应含 gem:5，实际 %v", item["loot"])
	}
}

func TestE2EMyDefenseChallengesEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	ownerID, ownerTok := e.newPlayer(t, "defch_empty_owner")
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '无人来犯', 999, '{}'::jsonb, 'snap')`, ownerID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/defense/challenges", nil)
	req.Header.Set("Authorization", "Bearer "+ownerTok)
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
	if body.Items == nil || len(body.Items) != 0 {
		t.Errorf("无记录应返回空 items，实际 %v", body.Items)
	}
}