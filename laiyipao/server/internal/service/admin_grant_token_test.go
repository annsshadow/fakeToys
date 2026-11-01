package service

// 第 21 轮：后台发放道具（revive_token / gem_wash_token / mastery_reset）。
//
// grantWallet 自第 74 轮就把 isWalletToken 转发到 grantToken —— 发放**一直
// 能成功**；缺口在两处可见性：服务端响应的 out 只装四货币（运营看不到
// 道具余额）、admin UI 的货币白名单把客服挡在门外（只能让玩家自己买）。
// 本测试钉住服务端那半：发放生效 + 余额回显 + 回收路径。

import (
	"context"
	"testing"
)

func TestAdminGrantCurrencyToken(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 发放：余额回显到 out[currency]。
	out, err := ts.AdminGrantCurrency(ctx, uid, "revive_token", 3)
	if err != nil {
		t.Fatalf("发放道具失败：%v", err)
	}
	if out["revive_token"] != 3 {
		t.Fatalf("道具发放应回显余额 3，实际 %v", out)
	}
	var stored int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT balance FROM user_tokens WHERE user_id = $1 AND token = 'revive_token'`,
		uid).Scan(&stored); err != nil {
		t.Fatalf("道具余额应落库：%v", err)
	}

	// 追加发放：回显累计值。
	out, err = ts.AdminGrantCurrency(ctx, uid, "revive_token", 2)
	if err != nil {
		t.Fatalf("再次发放失败：%v", err)
	}
	if out["revive_token"] != 5 {
		t.Fatalf("道具应累计到 5，实际 %v", out)
	}

	// 回收（负数）：余额扣减并回显。
	out, err = ts.AdminGrantCurrency(ctx, uid, "revive_token", -2)
	if err != nil {
		t.Fatalf("回收道具失败：%v", err)
	}
	if out["revive_token"] != 3 {
		t.Fatalf("回收后应剩 3，实际 %v", out)
	}

	// 三种道具全部可发（UI 白名单与服务端 walletTokens 同源）。
	for _, token := range []string{"gem_wash_token", "mastery_reset"} {
		if _, err := ts.AdminGrantCurrency(ctx, uid, token, 1); err != nil {
			t.Errorf("发放 %s 失败：%v", token, err)
		}
	}
}
