// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// idempotency_probe_test.go — R86 改前探针：六个写端点「同一请求打两次」
// 的实际后果（按危险度排序）。结论进台账；幂等键落地后，
// 前 3 条的断言会被「重放返回同响应」的新测试取代。

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

func TestR86IdempotencyProbe(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// ① 后台发币 ×2 —— 预期改前：余额 +200（重复实锤）
	uid := ts.newUser(t, ctx)
	before := ts.wallet(t, ctx, uid).Coin
	if _, err := ts.AdminGrantCurrency(ctx, uid, "coin", 100); err != nil {
		t.Fatalf("发币 1 失败：%v", err)
	}
	if _, err := ts.AdminGrantCurrency(ctx, uid, "coin", 100); err != nil {
		t.Fatalf("发币 2 失败：%v", err)
	}
	after := ts.wallet(t, ctx, uid).Coin
	t.Logf("① 后台发币 ×2：余额 %d → %d（%s）", before, after,
		map[bool]string{true: "重复执行实锤 +200", false: "幂等"}[after-before == 200])

	// ② 公告创建 ×2 —— 预期改前：两行公告（重复实锤）
	for i := 0; i < 2; i++ {
		if _, err := ts.AdminCreateAnnouncement(ctx, "R86探针标题", "R86探针正文", true); err != nil {
			t.Fatalf("公告 %d 失败：%v", i, err)
		}
	}
	var annCount int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM announcements WHERE title = 'R86探针标题'`).Scan(&annCount); err != nil {
		t.Fatalf("数公告失败：%v", err)
	}
	t.Logf("② 公告创建 ×2：%d 行（%s）", annCount,
		map[bool]string{true: "重复执行实锤 2 行", false: "幂等"}[annCount == 2])

	// ③ 批量兑换码 ×2 —— 预期改前：10 个码（重复实锤；随机码无唯一冲突）
	for i := 0; i < 2; i++ {
		if _, err := ts.AdminCreateRedeemCodes(ctx, 5, map[string]int{"coin": 1}, 1, nil); err != nil {
			t.Fatalf("批量码 %d 失败：%v", i, err)
		}
	}
	var codeCount int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM redeem_codes WHERE max_uses = 1 AND reward = '{"coin": 1}'`).Scan(&codeCount); err != nil {
		t.Fatalf("数码失败：%v", err)
	}
	t.Logf("③ 批量兑换码(5) ×2：%d 个码（%s）", codeCount,
		map[bool]string{true: "重复执行实锤 10 个", false: "幂等"}[codeCount == 10])

	// ④ 指定 code 创建 ×2 —— 预期：第二次被唯一约束挡（已幂等）
	if _, err := ts.AdminCreateRedeemCode(ctx, "R86FIXED1", map[string]int{"coin": 1}, 1, nil); err != nil {
		t.Fatalf("指定码 1 失败：%v", err)
	}
	_, err4 := ts.AdminCreateRedeemCode(ctx, "R86FIXED1", map[string]int{"coin": 1}, 1, nil)
	t.Logf("④ 指定 code 创建 ×2：第二次 err=%v（%s）", err4,
		map[bool]string{true: "已幂等（唯一约束）", false: "重复创建！"}[err4 != nil])

	// ⑤ 玩家兑换同码 ×2 —— 预期：第二次被 redeem_usages UNIQUE 挡（已幂等）
	uid5 := ts.newUser(t, ctx)
	if _, err := ts.Redeem(ctx, uid5, "R86FIXED1"); err != nil {
		t.Fatalf("兑换 1 失败：%v", err)
	}
	_, err5 := ts.Redeem(ctx, uid5, "R86FIXED1")
	t.Logf("⑤ 玩家兑换同码 ×2：第二次 err=%v（%s）", err5,
		map[bool]string{true: "已幂等（usages 唯一）", false: "重复发奖！"}[err5 != nil])

	// ⑥ 任务领奖 ×2 —— 预期：第二次被状态机挡（已幂等）
	uid6 := ts.newUser(t, ctx)
	var taskID, target int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id, target FROM tasks WHERE scope='daily' AND enabled ORDER BY id LIMIT 1`).
		Scan(&taskID, &target); err != nil {
		t.Fatalf("取任务失败：%v", err)
	}
	ts.exec(t, `INSERT INTO user_tasks (user_id, task_id, task_date, progress) VALUES ($1,$2,CURRENT_DATE,$3)`,
		uid6, taskID, target)
	if _, err := ts.ClaimTask(ctx, uid6, taskID); err != nil {
		t.Fatalf("领奖 1 失败：%v", err)
	}
	_, err6 := ts.ClaimTask(ctx, uid6, taskID)
	t.Logf("⑥ 任务领奖 ×2：第二次 err=%v（%s）", err6,
		map[bool]string{true: "已幂等（claimed_at 状态机）", false: "重复发奖！"}[err6 != nil])

	// ⑦ 结算同 token ×2 —— 预期：第二次被 used_at 抢占挡（已幂等）
	uid7 := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid7, map[string]int64{"energy": 100})
	tp, err := ts.StartBattle(ctx, uid7, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	in := settleInputForProbe(t, ts, ctx, uid7, int64(tp.TokenID))
	if _, err := ts.SettleBattle(ctx, uid7, int64(tp.TokenID), in); err != nil {
		t.Fatalf("结算 1 失败：%v", err)
	}
	_, err7 := ts.SettleBattle(ctx, uid7, int64(tp.TokenID), in)
	t.Logf("⑦ 结算同 token ×2：第二次 err=%v（%s）", err7,
		map[bool]string{true: "已幂等（used_at 抢占）", false: "重复发奖！"}[err7 != nil])

}

// settleInputForProbe 组装一次真实结算输入（与 r80Settle 同路径）。
func settleInputForProbe(t *testing.T, ts *testService, ctx context.Context, uid, tokenID int64) domain.SettleInput {
	t.Helper()
	var seedText string
	if err := ts.pool.QueryRow(ctx, `SELECT seed FROM battle_tokens WHERE id=$1`, tokenID).Scan(&seedText); err != nil {
		t.Fatalf("读 seed 失败：%v", err)
	}
	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读构筑失败：%v", err)
	}
	out := domain.Replay(domain.GenerateLevel(1), buildSnapshotForReplay(t, build), seedText,
		[]int{0, 1, -1}, "", domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules())
	if out.Err != nil {
		t.Fatalf("重算失败：%v", out.Err)
	}
	return domain.SettleInput{
		Result: "lose", Score: 1000, DurationMs: 30_000,
		Shots: 200, Hits: 100, Reactions: 20, HeatMax: 50,
		HPLeft: 500, WaveReached: 1, Kills: 30, Leaked: 0,
		ReplayHash:    out.ComputedHash,
		ElementsUsed:  map[string]int{"fire": 3},
		ReactionsUsed: map[string]int{"overheat": 20},
		TerrainUsed:   []string{},
		CardPicks:     []int{0, 1, -1},
	}
}
