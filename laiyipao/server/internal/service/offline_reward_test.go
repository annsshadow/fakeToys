package service

// 离线（挂机）收益的服务层测试（2026-10-31 新一轮第 7 轮）。
//
// 关键行为锚点：
//   - 新用户无可领收益（coin=0），但状态查询不报错；
//   - 收益按「距上次领取的时长 × 每小时速率」结算；
//   - 领取成功发金币、写流水、清零计时，钱包余额增加一致；
//   - 收益为 0 时**不清零**（否则挂机时间被吞）；
//   - 累计时长按 OFFLINE_CAP_HOURS 封顶；
//   - 老用户（无 offline_rewards 行）首次访问自动补行；
//   - 库故障必须报错而不是静默返回 0。

import (
	"context"
	"testing"
	"time"
)

func setLastClaimAt(t *testing.T, ts *testService, userID int64, at time.Time) {
	t.Helper()
	ctx := context.Background()
	if _, err := ts.db.Pool.Exec(ctx,
		`UPDATE offline_rewards SET last_claim_at = $1 WHERE user_id = $2`, at, userID); err != nil {
		t.Fatalf("设置 last_claim_at 失败：%v", err)
	}
}

func TestOfflineRewardStatusNewUser(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	st, err := ts.OfflineRewardStatus(ctx, uid)
	if err != nil {
		t.Fatalf("查询离线收益失败：%v", err)
	}
	if st.Coin != 0 {
		t.Errorf("新用户应无可领取收益，实际 %d", st.Coin)
	}
	if st.CapHours != 12 {
		t.Errorf("上限应 12 小时（默认），实际 %d", st.CapHours)
	}
	if st.ElapsedHours < 0 {
		t.Errorf("elapsed_hours 不应为负，实际 %f", st.ElapsedHours)
	}
}

func TestOfflineRewardStatusAccumulates(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	setLastClaimAt(t, ts, uid, time.Now().Add(-2*time.Hour))

	st, err := ts.OfflineRewardStatus(ctx, uid)
	if err != nil {
		t.Fatalf("查询离线收益失败：%v", err)
	}
	if st.Coin != 1200 {
		t.Errorf("挂机 2 小时应可领 1200 金币（600/h），实际 %d", st.Coin)
	}
	if st.ElapsedHours < 1.99 || st.ElapsedHours > 2.01 {
		t.Errorf("elapsed_hours 应约 2，实际 %f", st.ElapsedHours)
	}
}

func TestOfflineRewardClaimSucceeds(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	setLastClaimAt(t, ts, uid, time.Now().Add(-2*time.Hour))

	before := ts.wallet(t, ctx, uid)
	reward, err := ts.ClaimOfflineReward(ctx, uid)
	if err != nil {
		t.Fatalf("领取失败：%v", err)
	}
	if reward["coin"] != 1200 {
		t.Errorf("应领到 1200 金币，实际 %v", reward)
	}
	after := ts.wallet(t, ctx, uid)
	if after.Coin-before.Coin != 1200 {
		t.Errorf("钱包应增加 1200 金币，实际 +%d", after.Coin-before.Coin)
	}

	// 领取后清零：再查为 0，立即再领也为 0
	st, err := ts.OfflineRewardStatus(ctx, uid)
	if err != nil {
		t.Fatalf("查询离线收益失败：%v", err)
	}
	if st.Coin != 0 {
		t.Errorf("领取后应清零，实际 %d", st.Coin)
	}
	reward2, err := ts.ClaimOfflineReward(ctx, uid)
	if err != nil {
		t.Fatalf("再次领取失败：%v", err)
	}
	if reward2["coin"] != 0 {
		t.Errorf("刚领过再领应 0，实际 %v", reward2)
	}
}

func TestOfflineRewardClaimZeroDoesNotReset(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	// 距上次领取仅 3 秒 → 收益为 0（600/h 下不足 1 金币）
	setLastClaimAt(t, ts, uid, time.Now().Add(-3*time.Second))

	var before time.Time
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT last_claim_at FROM offline_rewards WHERE user_id = $1`, uid).Scan(&before); err != nil {
		t.Fatalf("读 last_claim_at 失败：%v", err)
	}

	reward, err := ts.ClaimOfflineReward(ctx, uid)
	if err != nil {
		t.Fatalf("领取失败：%v", err)
	}
	if reward["coin"] != 0 {
		t.Errorf("收益为 0 应返回 0，实际 %v", reward)
	}

	var after time.Time
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT last_claim_at FROM offline_rewards WHERE user_id = $1`, uid).Scan(&after); err != nil {
		t.Fatalf("读 last_claim_at 失败：%v", err)
	}
	if !after.Equal(before) {
		t.Errorf("收益为 0 时不应更新 last_claim_at（否则挂机时间被吞），before=%v after=%v", before, after)
	}
}

func TestOfflineRewardCap(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	// 100 小时前 → 按 12 小时上限封顶
	setLastClaimAt(t, ts, uid, time.Now().Add(-100*time.Hour))

	st, err := ts.OfflineRewardStatus(ctx, uid)
	if err != nil {
		t.Fatalf("查询离线收益失败：%v", err)
	}
	if st.Coin != 7200 {
		t.Errorf("100 小时应按 12 小时上限封顶为 7200 金币，实际 %d", st.Coin)
	}
	if st.ElapsedHours > 12.01 {
		t.Errorf("elapsed_hours 应被封顶到 12，实际 %f", st.ElapsedHours)
	}

	// 领取也只发封顶金额
	reward, err := ts.ClaimOfflineReward(ctx, uid)
	if err != nil {
		t.Fatalf("领取失败：%v", err)
	}
	if reward["coin"] != 7200 {
		t.Errorf("封顶领取应 7200 金币，实际 %v", reward)
	}
}

func TestOfflineRewardLegacyUserNoRow(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	// 模拟迁移上线前注册的老用户：没有 offline_rewards 行
	if _, err := ts.db.Pool.Exec(ctx,
		`DELETE FROM offline_rewards WHERE user_id = $1`, uid); err != nil {
		t.Fatalf("删除离线收益行失败：%v", err)
	}

	st, err := ts.OfflineRewardStatus(ctx, uid)
	if err != nil {
		t.Fatalf("老用户查询离线收益不应报错：%v", err)
	}
	if st.Coin != 0 {
		t.Errorf("老用户首次查询应 0 收益，实际 %d", st.Coin)
	}
	// 行已被幂等补回
	var n int
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM offline_rewards WHERE user_id = $1`, uid).Scan(&n); err != nil {
		t.Fatalf("统计离线收益行失败：%v", err)
	}
	if n != 1 {
		t.Errorf("查询后应自动补行（COUNT=1），实际 %d", n)
	}

	// 老用户领取也不报错
	reward, err := ts.ClaimOfflineReward(ctx, uid)
	if err != nil {
		t.Fatalf("老用户领取不应报错：%v", err)
	}
	if reward["coin"] != 0 {
		t.Errorf("老用户首次领取应 0，实际 %v", reward)
	}
}

func TestOfflineRewardBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.OfflineRewardStatus(ctx, 1); err == nil {
		t.Error("连接池已关闭时状态查询必须报错，不能静默返回 0")
	}
	if _, err := ts.ClaimOfflineReward(ctx, 1); err == nil {
		t.Error("连接池已关闭时领取必须报错，不能静默返回 0")
	}
}