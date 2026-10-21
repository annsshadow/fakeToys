package service

import (
	"context"
	"errors"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestAbandonBattleRefundsEnergy 钉住 P4-5。
//
// # 背景
//
// `StartBattle` 在**开战时**扣体力，而客户端此前**没有**任何放弃路径。
// 于是玩家中途退出：体力扣了、没有战报、没有奖励 ——
// 「什么都没发生，但资源没了」。
//
// # 四条判据
//
//  1. 放弃后体力**全额退回**；
//  2. 放弃后那枚凭证**不能再结算**（否则就等于「打完一次还能再打一次」）；
//  3. **同一枚凭证不能放弃两次**（否则能退两次体力）；
//  4. 放弃别人的凭证 → 404。
func TestAbandonBattleRefundsEnergy(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	before, err := ts.LoadWallet(ctx, uid)
	if err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	gl := domain.GenerateLevel(1)
	mid, err := ts.LoadWallet(ctx, uid)
	if err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}
	if mid.Energy != before.Energy-int64(gl.EnergyCost) {
		t.Fatalf("开战应扣 %d 体力：%d → %d", gl.EnergyCost, before.Energy, mid.Energy)
	}

	// 1) 放弃 → 体力全额退回
	refunded, err := ts.AbandonBattle(ctx, uid, tp.TokenID)
	if err != nil {
		t.Fatalf("放弃失败：%v", err)
	}
	if refunded != int64(gl.EnergyCost) {
		t.Errorf("应退回 %d 体力，实际 %d", gl.EnergyCost, refunded)
	}
	after, err := ts.LoadWallet(ctx, uid)
	if err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}
	if after.Energy != before.Energy {
		t.Errorf("放弃后体力应回到 %d，实际 %d", before.Energy, after.Energy)
	}

	// 2) 那枚凭证不能再结算
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err == nil {
		t.Error("放弃过的凭证不应还能结算")
	}

	// 3) 同一枚凭证不能放弃两次（否则能退两次体力）
	if _, err := ts.AbandonBattle(ctx, uid, tp.TokenID); err == nil {
		t.Error("同一枚凭证不应能放弃两次")
	}
	after2, err := ts.LoadWallet(ctx, uid)
	if err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}
	if after2.Energy != before.Energy {
		t.Errorf("重复放弃把体力退了两遍：%d，应为 %d", after2.Energy, before.Energy)
	}

	// 4) 别人的凭证 → 404
	other := ts.newUser(t, ctx)
	tp2, err := ts.StartBattle(ctx, other, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	if _, err := ts.AbandonBattle(ctx, uid, tp2.TokenID); !errors.Is(err, ErrNotFound) {
		t.Errorf("放弃别人的凭证应 ErrNotFound，实际 %v", err)
	}
}
