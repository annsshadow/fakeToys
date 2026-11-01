package httpapi

// 玩家兑换记录端点的端到端测试（2026-10-30 实质优化第 21 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 有记录 → 200 + items 包含兑换记录（码/奖励快照/时间）；
//   - 无记录 → 200 + 空 items。

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EMyRedemptionsRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/me/redemptions", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EMyRedemptionsSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "redemptions")
	code := fmt.Sprintf("E2E_%d", time.Now().UnixNano())
	var codeID int
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ($1, '{"gem":10}'::jsonb, 100)
		 RETURNING id`, code).Scan(&codeID); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, codeID, uid); err != nil {
		t.Fatalf("插入兑换记录失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/me/redemptions", nil)
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
	if item["code"] != code {
		t.Errorf("code 应为 %s，实际 %v", code, item["code"])
	}
	reward, ok := item["reward"].(map[string]any)
	if !ok || reward["gem"] != float64(10) {
		t.Errorf("reward 应为 {gem:10}，实际 %v", item["reward"])
	}
}

func TestE2EMyRedemptionsEmpty(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "redemptions_empty")
	req, _ := http.NewRequest("GET", "/api/v1/me/redemptions", nil)
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
