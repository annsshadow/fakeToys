package service

// 看板「今日签到/今日兑换」指标的集成测试（2026-10-31 实质优化第 28 轮）。
//
// 关键行为锚点：
//   - 造一条今日签到记录 → SignInsToday >= 1；
//   - 造一条今日兑换记录 → RedeemsToday >= 1；
//   - 昨日记录不计入今日指标。

import (
	"context"
	"testing"
)

func TestDashboardSignInsAndRedeemsToday(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "dash_sr")
	// 初始化钱包（兑换发币需要）
	if err := ts.initNewUser(ctx, uid); err != nil {
		t.Fatalf("初始化新用户失败：%v", err)
	}

	// 今日签到
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward)
		 VALUES ($1, CURRENT_DATE, 1, '{"coin":100}'::jsonb)`, uid); err != nil {
		t.Fatalf("插入签到失败：%v", err)
	}
	// 昨日签到（不应计入今日）
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward)
		 VALUES ($1, CURRENT_DATE - 1, 1, '{"coin":100}'::jsonb)`, uid); err != nil {
		t.Fatalf("插入昨日签到失败：%v", err)
	}

	// 今日兑换：需要 redeem_codes + redeem_usages
	// （redeem_usages 有 UNIQUE(code_id, user_id)，今日/昨日必须用不同码）
	var codeID int
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ('DASH_SR', '{"coin":100}'::jsonb, 100)
		 RETURNING id`).Scan(&codeID); err != nil {
		t.Fatalf("插入今日兑换码失败：%v", err)
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_usages (code_id, user_id, created_at)
		 VALUES ($1, $2, now())`, codeID, uid); err != nil {
		t.Fatalf("插入今日兑换记录失败：%v", err)
	}
	// 昨日兑换（不应计入今日）—— 用另一个码
	var yesterdayCodeID int
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ('DASH_SR_Y', '{"coin":100}'::jsonb, 100)
		 RETURNING id`).Scan(&yesterdayCodeID); err != nil {
		t.Fatalf("插入昨日兑换码失败：%v", err)
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_usages (code_id, user_id, created_at)
		 VALUES ($1, $2, now() - interval '1 day')`, yesterdayCodeID, uid); err != nil {
		t.Fatalf("插入昨日兑换记录失败：%v", err)
	}

	dash, err := ts.AdminDashboard(ctx)
	if err != nil {
		t.Fatalf("查看看板失败：%v", err)
	}
	if dash.Economy.SignInsToday < 1 {
		t.Errorf("SignInsToday 应 >= 1（今日签到），实际 %d", dash.Economy.SignInsToday)
	}
	if dash.Economy.SignInsToday > 1 {
		t.Errorf("SignInsToday 不应计入昨日记录，实际 %d", dash.Economy.SignInsToday)
	}
	if dash.Economy.RedeemsToday < 1 {
		t.Errorf("RedeemsToday 应 >= 1（今日兑换），实际 %d", dash.Economy.RedeemsToday)
	}
	if dash.Economy.RedeemsToday > 1 {
		t.Errorf("RedeemsToday 不应计入昨日记录，实际 %d", dash.Economy.RedeemsToday)
	}
}
