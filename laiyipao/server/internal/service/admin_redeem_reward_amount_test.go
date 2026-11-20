// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R281：兑换码奖励数量必须为正整数。
//
// 修前 validateRedeemReward 只校验键名白名单（walletColumns ∪ walletTokens），
// 不校验数量 v。运营经 HTTP 直接 POST JSON（body.Reward map[string]int）即可
// 绕过前端 parseReward 的非负正则，建出 {"coin": -100} 这种码；玩家兑换时
// Redeem 把 v 原样转 int64 delta 交 grantWallet，delta<0 走条件 UPDATE
// （WHERE coin >= -delta）→ 玩家被**倒扣** 100 coin。道具键同理走 grantToken
// 扣减分支。这是一条从运营输入到玩家资产损失的完整可达链。
//
// 判据：三入口（单建/批量/编辑）拒绝负值与零值且零落库；正值仍可建可兑。

import (
	"context"
	"errors"
	"testing"
)

func TestAdminRedeemRewardRejectsNonPositiveAmount(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 用户初始 coin=2000，作为正常兑换的基准。
	uid := ts.newUser(t, ctx)
	before := ts.scalar(t, ctx, `SELECT coin FROM user_wallets WHERE user_id=$1`, uid)
	if before != 2000 {
		t.Fatalf("新用户初始 coin 应为 2000，实际 %d", before)
	}

	// ① 单建：负数量必须被拒，且零落库
	if _, err := ts.AdminCreateRedeemCode(ctx, "R281NEG", map[string]int{"coin": -100}, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("负数量 coin 建码应返回 ErrBadInput，实际 %v", err)
	}
	if n := ts.scalar(t, ctx, `SELECT COUNT(*) FROM redeem_codes WHERE code='R281NEG'`); n != 0 {
		t.Fatalf("被拒的负数量兑换码不应落库，实存 %d 行", n)
	}

	// ② 负数量道具同样必须被拒（grantToken 也有扣减分支）
	if _, err := ts.AdminCreateRedeemCode(ctx, "R281NEGTOKEN", map[string]int{"revive_token": -1}, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("负数量道具建码应返回 ErrBadInput，实际 %v", err)
	}

	// ③ 零数量也必须被拒（无意义，且会掩盖录入错误）
	if _, err := ts.AdminCreateRedeemCode(ctx, "R281ZERO", map[string]int{"coin": 0}, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("零数量建码应返回 ErrBadInput，实际 %v", err)
	}

	// ④ 批量：负数量整批拒绝，零落库
	beforeCount := ts.scalar(t, ctx, `SELECT COUNT(*) FROM redeem_codes`)
	if _, err := ts.AdminCreateRedeemCodes(ctx, 5, map[string]int{"coin": -1}, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("批量负数量建码应返回 ErrBadInput，实际 %v", err)
	}
	if after := ts.scalar(t, ctx, `SELECT COUNT(*) FROM redeem_codes`); after != beforeCount {
		t.Fatalf("批量被拒后不应新增兑换码：前 %d，后 %d", beforeCount, after)
	}

	// ⑤ 编辑：把合法码改成负数量必须被拒
	if _, err := ts.AdminCreateRedeemCode(ctx, "R281OK", map[string]int{"coin": 50}, 1, nil); err != nil {
		t.Fatalf("建合法码失败：%v", err)
	}
	id := ts.scalar(t, ctx, `SELECT id FROM redeem_codes WHERE code='R281OK'`)
	if _, err := ts.AdminUpdateRedeemCode(ctx, id, map[string]int{"coin": -50}, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("编辑成负数量应返回 ErrBadInput，实际 %v", err)
	}

	// ⑥ 正值仍可建可兑：修复不破坏正常路径，且余额按奖励增加
	if _, err := ts.AdminCreateRedeemCode(ctx, "R281POS", map[string]int{"coin": 100}, 1, nil); err != nil {
		t.Fatalf("正值建码失败：%v", err)
	}
	if _, err := ts.Redeem(ctx, uid, "R281POS"); err != nil {
		t.Fatalf("正值兑换失败：%v", err)
	}
	if after := ts.scalar(t, ctx, `SELECT coin FROM user_wallets WHERE user_id=$1`, uid); after != before+100 {
		t.Fatalf("兑换 +100 coin 后余额应为 %d，实际 %d", before+100, after)
	}

	// ⑦ 负数量码根本不存在 → 无法触发倒扣
	if _, err := ts.Redeem(ctx, uid, "R281NEG"); !errors.Is(err, ErrBadInput) {
		t.Fatalf("被拒的码不应能兑换，实际 %v", err)
	}
}
