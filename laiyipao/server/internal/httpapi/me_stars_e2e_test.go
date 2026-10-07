package httpapi

// 第 139 轮：GET /me/stars 把玩家各关历史最好星级下发给客户端。
// 判据：响应体是刚写入 level_stars 的实际行（11 关 2 星、12 关 3 星），
// 形状为 { "stars": { "<关卡ID>": 星数 } }；无记录时空对象；未登录 401。

import (
	"context"
	"testing"
	"time"
)

func TestE2EMeStars(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "mestars")

	// 无记录 → 200 + 空对象（客户端按缺省 0 处理）
	status, body := e.get(t, "/api/v1/me/stars", tok)
	if status != 200 {
		t.Fatalf("GET /me/stars 应 200，实际 %d（body=%v）", status, body)
	}
	if _, ok := body["stars"].(map[string]any); !ok {
		t.Fatalf("响应应带 stars 对象，实际 %v", body)
	}

	// 未登录 → 401
	if s, _ := e.get(t, "/api/v1/me/stars", ""); s != 401 {
		t.Errorf("无令牌 GET /me/stars 应 401，实际 %d", s)
	}

	// 模拟结算写入（与 SettleBattle 的 upsert 同语义）
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	upsert := `
		INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
		VALUES ($1,$2,$3,0,1,0)
		ON CONFLICT (user_id, level_id) DO UPDATE SET
			stars = GREATEST(level_stars.stars, EXCLUDED.stars),
			clears = level_stars.clears + EXCLUDED.clears`
	for _, row := range [][]any{
		{uid, 11, 2},
		{uid, 12, 3},
	} {
		if _, err := e.pool.Exec(ctx, upsert, row...); err != nil {
			t.Fatalf("写星级行失败：%v", err)
		}
	}

	status, body = e.get(t, "/api/v1/me/stars", tok)
	if status != 200 {
		t.Fatalf("GET /me/stars 应 200，实际 %d（body=%v）", status, body)
	}
	stars, _ := body["stars"].(map[string]any)
	if int(stars["11"].(float64)) != 2 {
		t.Errorf("第 11 关应下发 2 星，实际 %v", stars)
	}
	if int(stars["12"].(float64)) != 3 {
		t.Errorf("第 12 关应下发 3 星，实际 %v", stars)
	}
	if len(stars) != 2 {
		t.Errorf("只应下发有记录的关，实际 %d 条：%v", len(stars), stars)
	}
}
