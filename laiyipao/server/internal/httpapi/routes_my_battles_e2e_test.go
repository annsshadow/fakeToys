package httpapi

// 玩家战报历史端点的端到端测试（2026-10-30 实质优化第 19 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 有记录 → 200 + items 包含战报（关卡/结果/星级/时间）；
//   - 无记录 → 200 + 空 items。

import (
	"context"
	"encoding/json"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMyBattleHistoryRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/me/battles", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EMyBattleHistorySucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "bhist")
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, stars, score, kills, leaked, replay_hash)
		 VALUES ($1, 5, 'win', 3, 888, 12, 1, 'e2e_hash')`, uid); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/battles", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
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
	if item["level_id"] != float64(5) {
		t.Errorf("level_id 应为 5，实际 %v", item["level_id"])
	}
	if item["result"] != "win" {
		t.Errorf("result 应为 win，实际 %v", item["result"])
	}
	if item["stars"] != float64(3) {
		t.Errorf("stars 应为 3，实际 %v", item["stars"])
	}
	if item["score"] != float64(888) {
		t.Errorf("score 应为 888，实际 %v", item["score"])
	}
}

func TestE2EMyBattleHistoryEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "bhist_empty")
	req, _ := http.NewRequest("GET", "/api/v1/me/battles", nil)
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
