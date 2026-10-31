package service

// 看板「防线总数/今日挑战」指标的集成测试（2026-10-31 实质优化第 39 轮）。
//
// 关键行为锚点：
//   - 造一条防线 → Defenses.Total >= 1；
//   - 造一条今日挑战记录 → ChallengesToday >= 1；
//   - 昨日挑战记录不计入今日。

import (
	"context"
	"testing"
)

func TestDashboardDefensesMetrics(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "dash_def")
	defID := seedDefense(t, ts, ownerID, "看板防线")

	// 今日挑战（挑战者 = owner 自己？不行，ChallengeDefense 会拒绝自己。
	// 直接插入 defense_challenges 记录（不经过业务校验）。
	challengerID, _ := newGuestUser(t, ts, "dash_def_ch")
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms, hp_left_pct, settled, created_at)
		 VALUES ($1, $2, 1, TRUE, 1000, 90, TRUE, now())`, defID, challengerID); err != nil {
		t.Fatalf("插入今日挑战失败：%v", err)
	}
	// 昨日挑战（不应计入今日）
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms, hp_left_pct, settled, created_at)
		 VALUES ($1, $2, 1, FALSE, 1000, 50, TRUE, now() - interval '1 day')`, defID, challengerID); err != nil {
		t.Fatalf("插入昨日挑战失败：%v", err)
	}

	dash, err := ts.AdminDashboard(ctx)
	if err != nil {
		t.Fatalf("查看看板失败：%v", err)
	}
	if dash.Defenses.Total < 1 {
		t.Errorf("Defenses.Total 应 >= 1（有防线），实际 %d", dash.Defenses.Total)
	}
	if dash.Defenses.ChallengesToday < 1 {
		t.Errorf("ChallengesToday 应 >= 1（今日挑战），实际 %d", dash.Defenses.ChallengesToday)
	}
	if dash.Defenses.ChallengesToday > 1 {
		t.Errorf("ChallengesToday 不应计入昨日记录，实际 %d", dash.Defenses.ChallengesToday)
	}
}
