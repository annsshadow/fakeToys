package service

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestDiagnoseStuckIsComputedByTheServer 钉住「卡没卡住」由服务端说了算。
//
// # 背景（P2-7）
//
// 客户端此前把 `failed_times` **硬编码成 2** 去请求诊断，
// 于是**每个玩家第一次进关**都能看到「卡关诊断」卡片 ——
// 一张对所有人说同一句话的卡片。
//
// 服务端本来就知道真实次数（battle_records 记着每一局的结果），
// 所以判据改成：服务端算出来的 `failed_times` / `stuck`。
//
// # 三条判据
//
//  1. 从没打过这关 → failed_times=0，stuck=false；
//  2. 连输两局 → failed_times=2，stuck=true；
//  3. 连输三局后又赢了一局 → 计数归零（赢过说明这关能过），
//     stuck 再次为 false。
func TestDiagnoseStuckIsComputedByTheServer(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	levelID := 1

	// 1) 从没打过 → 不卡
	d, err := ts.Diagnose(ctx, uid, levelID, 0)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 0 {
		t.Errorf("没打过这关时 failed_times 应为 0，实际 %d", d.FailedTimes)
	}
	if d.Stuck {
		t.Error("没打过这关不该说「卡住了」")
	}

	// 输两局
	for i := 0; i < 2; i++ {
		ts.settleLoss(t, ctx, uid, levelID)
	}
	d, err = ts.Diagnose(ctx, uid, levelID, 0)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 2 {
		t.Errorf("连输两局后 failed_times 应为 2，实际 %d", d.FailedTimes)
	}
	if !d.Stuck {
		t.Error("连输两局应判定为「卡住了」")
	}

	// 再输一局，然后赢一局 → 计数归零
	ts.settleLoss(t, ctx, uid, levelID)
	ts.settleWin(t, ctx, uid, levelID)
	d, err = ts.Diagnose(ctx, uid, levelID, 0)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 0 {
		t.Errorf("赢过一局后连续失败应归零，实际 %d", d.FailedTimes)
	}
	if d.Stuck {
		t.Error("赢过一局后不该说「卡住了」（赢过说明这关能过）")
	}
}

// settleLoss 打一局并结算成失败。
func (ts *testService) settleLoss(t *testing.T, ctx context.Context, uid int64, levelID int) {
	t.Helper()
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
	tp, err := ts.StartBattle(ctx, uid, levelID)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
		t.Fatalf("失败结算不应报错：%v", err)
	}
}

// settleWin 打一局并结算成胜利。
func (ts *testService) settleWin(t *testing.T, ctx context.Context, uid int64, levelID int) {
	t.Helper()
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
	tp, err := ts.StartBattle(ctx, uid, levelID)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	gl := domain.GenerateLevel(levelID)
	in := loseSettleInput()
	in.Result = "win"
	in.Kills = domain.MaxKillsFor(gl)
	in.Score = 2000
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("胜利结算失败：%v", err)
	}
}
