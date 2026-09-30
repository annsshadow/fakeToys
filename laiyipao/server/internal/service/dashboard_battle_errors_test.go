package service

// 覆盖率收尾第三批：看板分区错误与战斗链路的剩余中段失败。
//
// AdminDashboard 串了 9 段统计查询，各查各的表 —— 把对应表改名，
// 就能让看板在**那一段**失败，覆盖各自的错误出口。
// battle_records 被三段共用，只有最先用到它的那段的分支可达。

import (
	"context"
	"testing"
)

func TestAdminDashboardSectionFailures(t *testing.T) {
	ctx := context.Background()

	// 每个用例：scratch 库上改一张表 → 看板必须把错误带出来
	//（任何一个分区静默吞错，后台看板就会用假 0 误导运营）。
	sections := []struct{ name, table string }{
		{"users", "users"},
		{"battles", "battle_records"},
		{"progression", "user_progress"},
		{"economy", "wallet_flows"},
		{"orders", "orders"},
		{"verification", "replay_verifications"},
	}
	for _, s := range sections {
		t.Run(s.name, func(t *testing.T) {
			ts := openScratchService(t)
			ts.renameTable(t, s.table, s.table+"_bak")
			_, err := ts.AdminDashboard(ctx)
			ts.renameTable(t, s.table+"_bak", s.table)
			if err == nil {
				t.Fatalf("表 %s 缺失时看板应报错，实际静默成功", s.table)
			}
		})
	}
}

// TestStartBattleMidwayFailures StartBattle 的中段失败。
func TestStartBattleMidwayFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("battle_tokens写入失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "battle_tokens", "battle_tokens_bak")
		_, err := ts.StartBattle(ctx, uid, 1)
		ts.renameTable(t, "battle_tokens_bak", "battle_tokens")
		if err == nil {
			t.Fatal("应报错")
		}
	})

	t.Run("凭证发放后快照失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_equipment", "user_equipment_bak")
		_, err := ts.StartBattle(ctx, uid, 1)
		ts.renameTable(t, "user_equipment_bak", "user_equipment")
		if err == nil {
			t.Fatal("应报错")
		}
		// 事务已提交（token 落库、体力已扣）—— 恢复表后重试必须成功，
		// 证明失败点确实在快照而不是扣费。
		if _, err := ts.StartBattle(ctx, uid, 1); err != nil {
			t.Fatalf("恢复后开局应成功：%v", err)
		}
	})
}

// TestSettleAndEconomyBroken 故障态直调（此前只经 httpapi 覆盖，包内需自证）。
func TestSettleAndEconomyBroken(t *testing.T) {
	broken := openBrokenService(t)
	ctx := context.Background()

	if _, err := broken.SettleBattle(ctx, 1, 1, loseSettleInput()); err == nil {
		t.Error("SettleBattle 故障态应报错")
	}
	if _, err := broken.SignIn(ctx, 1); err == nil {
		t.Error("SignIn 故障态应报错")
	}
	if _, err := broken.LoadShop(ctx, 1); err == nil {
		t.Error("LoadShop 故障态应报错")
	}
	if _, err := broken.Diagnose(ctx, 1, 1, 1); err == nil {
		t.Error("Diagnose 故障态应报错")
	}
	if _, err := broken.SaveDefense(ctx, 1, SaveDefenseInput{Skills: []int{1}}); err == nil {
		t.Error("SaveDefense 故障态应报错")
	}
	if _, err := broken.ChallengeDefense(ctx, 1, 1, ChallengeInput{Won: true}); err == nil {
		t.Error("ChallengeDefense 故障态应报错")
	}
	if _, err := broken.VerifyReplay(ctx, 1, 1, "x"); err == nil {
		t.Error("VerifyReplay 故障态应报错")
	}
	if _, err := broken.Leaderboard(ctx, "stage", 10); err == nil {
		t.Error("Leaderboard(stage) 故障态应报错")
	}
	if _, err := broken.AdminSetUserStatus(ctx, 1, true, "x"); err == nil {
		t.Error("AdminSetUserStatus 故障态应报错")
	}
	if _, err := broken.AdminUpdateShopItem(ctx, 1, map[string]any{"name": "x"}); err == nil {
		t.Error("AdminUpdateShopItem 故障态应报错")
	}
}
