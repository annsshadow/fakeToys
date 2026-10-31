package httpapi

// 离线（挂机）收益端点的端到端测试（2026-10-31 新一轮第 7 轮）。
//
// 覆盖：
//   - 未认证 → 401（GET 与 POST）；
//   - 有累计 → GET 返回可领金币、POST 领取后钱包增加且清零；
//   - 无累计 → GET coin=0、POST 返回 0 且不清零。

import (
	"context"
	"encoding/json"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EOfflineRewardRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	for _, tc := range []struct {
		method string
		path   string
	}{
		{"GET", "/api/v1/me/offline-reward"},
		{"POST", "/api/v1/me/offline-reward/claim"},
	} {
		req, _ := http.NewRequest(tc.method, tc.path, nil)
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

func TestE2EOfflineRewardStatusAndClaim(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "offline_reward")

	// 造 2 小时累计
	if _, err := e.pool.Exec(context.Background(),
		`UPDATE offline_rewards SET last_claim_at = $1 WHERE user_id = $2`,
		time.Now().Add(-2*time.Hour), uid); err != nil {
		t.Fatalf("设置 last_claim_at 失败：%v", err)
	}

	// GET：应返回可领 1200 金币（600/h × 2h）
	req, _ := http.NewRequest("GET", "/api/v1/me/offline-reward", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("GET 失败：%v", err)
	}
	defer resp.Body.Close()
	var status struct {
		Coin int64 `json:"coin"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&status); err != nil {
		t.Fatalf("解析 GET 响应失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("GET 应 200，实际 %d", resp.StatusCode)
	}
	if status.Coin != 1200 {
		t.Fatalf("挂机 2 小时应可领 1200 金币，实际 %d", status.Coin)
	}

	// 领取前的钱包余额
	var coinBefore int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT coin FROM user_wallets WHERE user_id = $1`, uid).Scan(&coinBefore); err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}

	// POST：领取
	req2, _ := http.NewRequest("POST", "/api/v1/me/offline-reward/claim", nil)
	req2.Header.Set("Authorization", "Bearer "+tok)
	resp2, err := e.app.Test(req2, 10000)
	if err != nil {
		t.Fatalf("POST 失败：%v", err)
	}
	defer resp2.Body.Close()
	var claim struct {
		Coin int64 `json:"coin"`
	}
	if err := json.NewDecoder(resp2.Body).Decode(&claim); err != nil {
		t.Fatalf("解析 POST 响应失败：%v", err)
	}
	if resp2.StatusCode != fiber.StatusOK {
		t.Fatalf("POST 应 200，实际 %d", resp2.StatusCode)
	}
	if claim.Coin != 1200 {
		t.Errorf("应领到 1200 金币，实际 %d", claim.Coin)
	}
	var coinAfter int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT coin FROM user_wallets WHERE user_id = $1`, uid).Scan(&coinAfter); err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}
	if coinAfter-coinBefore != 1200 {
		t.Errorf("钱包应增加 1200 金币，实际 +%d", coinAfter-coinBefore)
	}

	// 领取后 GET：清零
	req3, _ := http.NewRequest("GET", "/api/v1/me/offline-reward", nil)
	req3.Header.Set("Authorization", "Bearer "+tok)
	resp3, err := e.app.Test(req3, 10000)
	if err != nil {
		t.Fatalf("GET（领取后）失败：%v", err)
	}
	defer resp3.Body.Close()
	var statusAfter struct {
		Coin int64 `json:"coin"`
	}
	if err := json.NewDecoder(resp3.Body).Decode(&statusAfter); err != nil {
		t.Fatalf("解析 GET 响应失败：%v", err)
	}
	if statusAfter.Coin != 0 {
		t.Errorf("领取后应清零，实际 %d", statusAfter.Coin)
	}
}

func TestE2EOfflineRewardClaimZero(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "offline_reward_zero")

	// 新用户无累计：GET coin=0
	req, _ := http.NewRequest("GET", "/api/v1/me/offline-reward", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("GET 失败：%v", err)
	}
	defer resp.Body.Close()
	var status struct {
		Coin int64 `json:"coin"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&status); err != nil {
		t.Fatalf("解析 GET 响应失败：%v", err)
	}
	if status.Coin != 0 {
		t.Errorf("新用户应 0 收益，实际 %d", status.Coin)
	}

	// POST 也返回 0
	req2, _ := http.NewRequest("POST", "/api/v1/me/offline-reward/claim", nil)
	req2.Header.Set("Authorization", "Bearer "+tok)
	resp2, err := e.app.Test(req2, 10000)
	if err != nil {
		t.Fatalf("POST 失败：%v", err)
	}
	defer resp2.Body.Close()
	var claim struct {
		Coin int64 `json:"coin"`
	}
	if err := json.NewDecoder(resp2.Body).Decode(&claim); err != nil {
		t.Fatalf("解析 POST 响应失败：%v", err)
	}
	if claim.Coin != 0 {
		t.Errorf("无收益领取应返回 0，实际 %d", claim.Coin)
	}
}