package service

// 经济域（economy.go）的服务层测试：榜单、商店、兑换、验真。
// 正常链路打健康库；错误分支走 closed pool / scratch。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"
)

func TestLeaderboardKinds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 造一点进度：通关记录 + 经验
	ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
	            VALUES ($1, 1, 3, 5000, 1, 100)
	            ON CONFLICT (user_id, level_id) DO UPDATE SET clears = 1, min_power_clear = 100`, uid)
	ts.exec(t, `UPDATE user_progress SET level_exp = level_exp + 777 WHERE user_id = $1`, uid)

	// 三种榜单都必须能跑且包含自己
	for _, kind := range []string{"power", "stage", "efficiency"} {
		items, err := ts.Leaderboard(ctx, kind, 50)
		if err != nil {
			t.Fatalf("%s 榜失败：%v", kind, err)
		}
		found := false
		for _, it := range items {
			if it.UserID == uid {
				found = true
				// rank 必须从 1 连续递增
			}
		}
		if !found {
			t.Errorf("%s 榜应包含用户 %d", kind, uid)
		}
	}
	// rank 编号连续
	items, _ := ts.Leaderboard(ctx, "power", 50)
	for i, it := range items {
		if it.Rank != i+1 {
			t.Fatalf("rank 应从 1 连续递增，第 %d 项 rank=%d", i, it.Rank)
		}
	}

	// 未知 kind 落到 power 分支（不报错）
	if _, err := ts.Leaderboard(ctx, "bogus", 50); err != nil {
		t.Errorf("未知 kind 应回落默认榜：%v", err)
	}
	// limit 夹紧：0 与超大值都合法
	if _, err := ts.Leaderboard(ctx, "power", 0); err != nil {
		t.Errorf("limit=0 应回落 50：%v", err)
	}
	if _, err := ts.Leaderboard(ctx, "power", 9999); err != nil {
		t.Errorf("limit=9999 应回落 50：%v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.Leaderboard(ctx, "power", 10); err == nil {
		t.Error("故障态应报错")
	}
}

func TestSignInCalendar(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 首签：第 1 天
	res, err := ts.SignIn(ctx, uid)
	if err != nil {
		t.Fatalf("首签失败：%v", err)
	}
	if res.Already || res.DayIndex != 1 {
		t.Errorf("首签应为第 1 天：%+v", res)
	}
	if res.Wallet.Coin == 0 {
		t.Error("签到响应必须带更新后的钱包")
	}
	// 钱包必须真的到账（第 1 天 1000 金币）
	if w := ts.wallet(t, ctx, uid); w.Coin < 2000+1000 {
		t.Errorf("签到金币未到账：%d", w.Coin)
	}

	// 当天二签：唯一键（user_id, sign_date）冲突 → 业务错误
	if _, err := ts.SignIn(ctx, uid); !errors.Is(err, ErrForbidden) {
		t.Errorf("当天重复签到应 ErrForbidden，实际 %v", err)
	}

	// 七天全签完后 → Already=true（日历耗尽，而不是报错）
	for day := 1; day <= 7; day++ {
		ts.exec(t, `INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward)
		            VALUES ($1, CURRENT_DATE + $2::int, $3, '{}'::jsonb)
		            ON CONFLICT (user_id, sign_date) DO NOTHING`, uid, day, day)
	}
	res2, err := ts.SignIn(ctx, uid)
	if err != nil {
		t.Fatalf("签满 7 天后再签应不报错：%v", err)
	}
	if !res2.Already {
		t.Errorf("日历耗尽应 Already=true：%+v", res2)
	}
}

func TestRedeemLifecycle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 无效码
	if _, err := ts.Redeem(ctx, uid, "NOPE"); !errors.Is(err, ErrBadInput) {
		t.Errorf("无效码应 ErrBadInput，实际 %v", err)
	}

	code := fmt.Sprintf("SVCRDM%d", time.Now().UnixNano()%1_000_000)
	ts.exec(t, `INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
	            VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), $1, $2, 5, 0, true)`,
		code, []byte(`{"coin":250}`))
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `DELETE FROM redeem_usages WHERE code_id IN (SELECT id FROM redeem_codes WHERE code=$1)`, code)
		_, _ = ts.pool.Exec(ctx, `DELETE FROM redeem_codes WHERE code=$1`, code)
	})

	reward, err := ts.Redeem(ctx, uid, code)
	if err != nil || reward["coin"] != 250 {
		t.Fatalf("兑换应成功：%v, %v", reward, err)
	}
	if w := ts.wallet(t, ctx, uid); w.Coin < 2000+250 {
		t.Errorf("兑换奖励未到账：%d", w.Coin)
	}
	// 同用户重复 → ErrForbidden
	if _, err := ts.Redeem(ctx, uid, code); !errors.Is(err, ErrForbidden) {
		t.Errorf("重复兑换应 ErrForbidden，实际 %v", err)
	}

	// 用尽：used_count 打满后其他人兑换 → ErrBadInput
	other := ts.newUser(t, ctx)
	ts.exec(t, `UPDATE redeem_codes SET used_count = max_uses WHERE code = $1`, code)
	if _, err := ts.Redeem(ctx, other, code); !errors.Is(err, ErrBadInput) {
		t.Errorf("用尽的码应 ErrBadInput，实际 %v", err)
	}

	// 奖励字段未知 → grantWallet 业务错误（而不是 500）
	badCode := code + "B"
	ts.exec(t, `INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
	            VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), $1, $2, 5, 0, true)`,
		badCode, []byte(`{"diamonds":5}`))
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `DELETE FROM redeem_codes WHERE code=$1`, badCode)
	})
	if _, err := ts.Redeem(ctx, other, badCode); !errors.Is(err, ErrBadInput) {
		t.Errorf("未知奖励币种应 ErrBadInput，实际 %v", err)
	}
}

func TestLoadShopAndBuy(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	items, err := ts.LoadShop(ctx, uid)
	if err != nil || len(items) == 0 {
		t.Fatalf("商城应非空：%v", err)
	}
	for _, it := range items {
		if it.Price == nil || it.Payload == nil {
			t.Errorf("商城条目 %d 缺 price/payload：%+v", it.ID, it)
		}
	}

	// 商品不存在
	if _, err := ts.Buy(ctx, uid, 99999); !errors.Is(err, ErrNotFound) {
		t.Errorf("未知商品应 ErrNotFound，实际 %v", err)
	}

	// 首充礼包 0 元购成功 + 限购
	granted, err := ts.Buy(ctx, uid, 7)
	if err != nil || granted["gem"] != 300 {
		t.Fatalf("首充礼包应发放 gem300：%v, %v", granted, err)
	}
	if _, err := ts.Buy(ctx, uid, 7); !errors.Is(err, ErrForbidden) {
		t.Errorf("超出限购应 ErrForbidden，实际 %v", err)
	}

	// 余额不足
	ts.exec(t, `UPDATE user_wallets SET gem = 0 WHERE user_id = $1`, uid)
	if _, err := ts.Buy(ctx, uid, 1); !errors.Is(err, ErrBadInput) {
		t.Errorf("余额不足应 ErrBadInput，实际 %v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.LoadShop(ctx, uid); err == nil {
		t.Error("故障态应报错")
	}
}

func TestVerifyReplayAndParseCardPicks(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	in := loseSettleInput()
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	// 拿 battle_id
	var battleID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).Scan(&battleID); err != nil {
		t.Fatalf("读战报失败：%v", err)
	}

	// 匹配
	res, err := ts.VerifyReplay(ctx, uid, battleID, in.ReplayHash)
	if err != nil {
		t.Fatalf("验真失败：%v", err)
	}
	if !res.Matched || res.Seed == "" || res.RecordedAt == "" {
		t.Errorf("验真结果不完整：%+v", res)
	}
	// 留痕
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1 AND verifier_id = $2`, battleID, uid).
		Scan(&n); err != nil || n != 1 {
		t.Errorf("验真必须留痕（%d, %v）", n, err)
	}
	// 不匹配
	res2, err := ts.VerifyReplay(ctx, uid, battleID, "ffffffffffffffff")
	if err != nil || res2.Matched {
		t.Errorf("不同哈希应 matched=false：%+v, %v", res2, err)
	}
	// 未知战报
	if _, err := ts.VerifyReplay(ctx, uid, 999999999, "x"); !errors.Is(err, ErrNotFound) {
		t.Errorf("未知战报应 ErrNotFound，实际 %v", err)
	}

	// GetReplay：结构完整性 + settle_input 剥离 + card_picks 解析
	info, err := ts.GetReplay(ctx, uid, battleID)
	if err != nil {
		t.Fatalf("回放失败：%v", err)
	}
	// seed 必须是十进制数字串（parseSeed 内部做形状校验）
	_ = parseSeed(t, info.Seed)
	if info.CardPicks == nil || len(info.CardPicks) != 2 || info.CardPicks[1] != -1 {
		t.Errorf("card_picks 应解析为 [0,-1]：%v", info.CardPicks)
	}
	if _, has := info.Build["settle_input"]; has {
		t.Error("回放绝不能带 settle_input")
	}
	if _, has := info.Build["attacker"]; !has {
		t.Error("回放必须带 attacker")
	}
	// 未知战报
	if _, err := ts.GetReplay(ctx, uid, 999999999); !errors.Is(err, ErrNotFound) {
		t.Errorf("未知回放应 ErrNotFound，实际 %v", err)
	}

	// parseCardPicks 纯函数
	cases := []struct {
		in   string
		want []int
	}{
		{"", nil},
		{"   ", nil},
		{"0", []int{0}},
		{"0,1,-1", []int{0, 1, -1}},
		{" 0 , 1 ", []int{0, 1}},
		{"a,1,b,-1", []int{1, -1}}, // 脏值跳过，不整条作废
	}
	for _, c := range cases {
		got := parseCardPicks(c.in)
		if len(got) != len(c.want) {
			t.Errorf("parseCardPicks(%q) = %v，期望 %v", c.in, got, c.want)
			continue
		}
		for i := range got {
			if got[i] != c.want[i] {
				t.Errorf("parseCardPicks(%q) = %v，期望 %v", c.in, got, c.want)
			}
		}
	}

	// parseInt64
	if v, err := parseInt64("123"); err != nil || v != 123 {
		t.Errorf("parseInt64(123) = %d, %v", v, err)
	}
	if _, err := parseInt64("x"); err == nil {
		t.Error("parseInt64 非数字应报错")
	}
}

// parseSeed 辅助：把字符串种子转回数字（形状校验用）。
func parseSeed(t *testing.T, s string) int64 {
	t.Helper()
	var v int64
	for i := 0; i < len(s); i++ {
		if s[i] < '0' || s[i] > '9' {
			t.Fatalf("seed 应是十进制数字串：%q", s)
		}
		v = v*10 + int64(s[i]-'0')
	}
	return v
}

// TestGetReplayErrorBranch 故障态下 GetReplay / VerifyReplay 的错误路径。
func TestReplayErrorBranches(t *testing.T) {
	broken := openBrokenService(t)
	ctx := context.Background()
	if _, err := broken.GetReplay(ctx, 1, 1); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.VerifyReplay(ctx, 1, 1, "x"); err == nil {
		t.Error("故障态应报错")
	}
}
