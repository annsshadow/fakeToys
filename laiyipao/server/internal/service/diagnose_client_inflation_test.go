package service

import (
	"context"
	"testing"
)

// TestDiagnoseIgnoresInflatedClientFailedTimes 钉住「客户端不能自己编失败次数」。
//
// # 背景（2026-10-21 二轮审计 P2）
//
// `Diagnose` 的连续失败次数是**服务端算**的（`battle_records` 记着每局结果），
// 但客户端仍然会传一个 `?failed_times=`。
//
// 当时的实现是「客户端报得大就认它的」：
//
//	if failedTimes > realFailed {
//	    realFailed = failedTimes
//	}
//
// 于是客户端传 `failed_times=999999` 就能把计数抬到任意大 ——
// **服务端的权威计数被客户端的猜测推翻**，
// 于是第一次挑战就能拿到「卡关诊断」卡（`stuck=true`）。
//
// 这正是 P2-7 要消灭的东西（「让客户端自己报」= 让它自己编）。
//
// # 判据
//
// 允许的比我们多**一局**（在途的那一局还没写进 battle_records），
// 多于 1 的一律按服务端的算。
func TestDiagnoseIgnoresInflatedClientFailedTimes(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	const levelID = 1

	// 从没打过这关：真实连续失败数 = 0
	d, err := ts.Diagnose(ctx, uid, levelID, 0)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 0 || d.Stuck {
		t.Fatalf("没打过这关不该卡住：failed_times=%d stuck=%v", d.FailedTimes, d.Stuck)
	}

	// 客户端报一个巨大的数 → 必须被忽略
	d, err = ts.Diagnose(ctx, uid, levelID, 999999)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes > 1 {
		t.Errorf("客户端把 failed_times 抬到 %d，服务端的权威计数被推翻了 —— "+
			"它只该被允许比我们多 1 局（在途的那一局）", d.FailedTimes)
	}
	if d.Stuck {
		t.Error("没打过这关不该说「卡住了」—— 客户端报个大数就信，那等于让它自己编")
	}

	// 安全性判据（必须在**还没输过**时验）：**客户端最多只能把计数抬到
	// real+1**，而「卡住」的阈值是 2（domain.StuckThreshold）。
	// 于是「从没输过」（real=0）的玩家无论报多大都**跨不过**阈值 ——
	// 这就是「客户端不能自己编出「卡住了」」的完整保证。
	d, err = ts.Diagnose(ctx, uid, levelID, 1)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 1 {
		t.Errorf("客户端报 1（real=0）应被接受为 1，实际 %d", d.FailedTimes)
	}
	if d.Stuck {
		t.Error("客户端报 1 不足以跨过阈值 2 —— 不该说「卡住了」")
	}

	// 输两局 → 真实计数 2
	for i := 0; i < 2; i++ {
		ts.settleLoss(t, ctx, uid, levelID)
	}
	d, err = ts.Diagnose(ctx, uid, levelID, 0)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 2 || !d.Stuck {
		t.Fatalf("连输两局应判定为卡住：failed_times=%d stuck=%v", d.FailedTimes, d.Stuck)
	}

	// 客户端报 999999 → 按「比我们多 1」夹紧（real=2 → 3），而不是 999999
	d, err = ts.Diagnose(ctx, uid, levelID, 999999)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 3 {
		t.Errorf("客户端报了 999999，服务端应夹到自己的计数 +1 = 3，实际 %d", d.FailedTimes)
	}
	if !d.Stuck {
		t.Error("连输两局应判定为卡住")
	}

	// 允许「比我们多 1」（在途的那一局）：2 → 3
	d, err = ts.Diagnose(ctx, uid, levelID, 3)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.FailedTimes != 3 {
		t.Errorf("客户端比服务端多报 1 局（在途的那一局）应被接受，实际 %d", d.FailedTimes)
	}
}
