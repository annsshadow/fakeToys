package service

// 宝石闭环的行为测试：商城买到 → 看得到 → 洗练扣券。
//
// # 为什么这三步必须一起测
//
// 单独测任何一步都会「看起来是对的」：
//
//	只测买 → 买了但玩家在界面上看不到（user_gems 此前**没有任何面向玩家的读取面**）
//	只测洗 → 洗得好好的，但玩家手里一颗宝石都没有（这正是第 150 轮之前的状态）
//	只测看 → 看得到，但没有任何途径能得到它
//
// 所以这里走完整链路：花钻石 → 手上多一颗 → 列得出来 → 洗 → 券被扣。

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// shopItemIDByCode 找商城商品 id，避免把 id 写死在测试里。
func shopItemIDByCode(t *testing.T, ts *testService, ctx context.Context, code string) int64 {
	t.Helper()
	var id int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM shop_items WHERE code = $1 AND enabled`, code).Scan(&id); err != nil {
		t.Fatalf("商城里找不到上架中的 %s：%v", code, err)
	}
	return id
}

// TestGemShopItemMintsAnOwnedInstance 买宝石商品 → 手上真的多一颗实例。
//
// 关键断言是「user_gems 里多了一行」而不是「Buy 返回了一个 map」——
// 后者曾经就足以让测试通过：早期 `Buy` 只按 payload 里有什么发什么，
// 而 `gem_instance` 会因为不是货币而整笔回滚。
func TestGemShopItemMintsAnOwnedInstance(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"gem": domain.GemShopPrice * 2})

	item := shopItemIDByCode(t, ts, ctx, "shop_gem")

	var before int
	if err := ts.pool.QueryRow(ctx,
		`SELECT count(*) FROM user_gems WHERE user_id = $1`, uid).Scan(&before); err != nil {
		t.Fatalf("数宝石失败：%v", err)
	}

	if _, err := ts.Buy(ctx, uid, item); err != nil {
		t.Fatalf("购买宝石失败：%v", err)
	}

	got, err := ts.ListGems(ctx, uid)
	if err != nil {
		t.Fatalf("列宝石失败：%v", err)
	}
	if len(got) != before+1 {
		t.Fatalf("买完后宝石数 = %d，买前 %d（应 +1）", len(got), before)
	}

	g := got[len(got)-1]
	// 品质必须在五档之内，且词条条数必须与该品质一致 ——
	// 这是「洗练/构筑」能不能用的前提，不能只是「有一行数据」。
	if domain.GemQualityMult(g.Quality) == 1000 && g.Quality != "gray" {
		t.Errorf("品质 %q 不在 gem_qualities 里", g.Quality)
	}
	if want := domain.GemQualityAffixCount(g.Quality); len(g.Affixes) != want {
		t.Errorf("品质 %q 应有 %d 条词条，实际 %d", g.Quality, want, len(g.Affixes))
	}
	for _, af := range g.Affixes {
		if af.Value <= 0 {
			t.Errorf("词条 %q 的值为 %d —— 正加成不该出现非正值", af.Affix, af.Value)
		}
	}
	if g.Name == "" {
		t.Error("宝石名称为空 —— 后台与客户端会显示一个空条目")
	}
}

// TestBuyingGemPaysPriceAndRespectsDailyLimit 买了要真扣钻石，且限次生效。
func TestBuyingGemPaysPriceAndRespectsDailyLimit(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"gem": domain.GemShopPrice * 5})
	item := shopItemIDByCode(t, ts, ctx, "shop_gem")

	walletGem := func() int64 {
		var n int64
		if err := ts.pool.QueryRow(ctx,
			`SELECT gem FROM user_wallets WHERE user_id = $1`, uid).Scan(&n); err != nil {
			t.Fatalf("读钱包失败：%v", err)
		}
		return n
	}

	before := walletGem()
	if _, err := ts.Buy(ctx, uid, item); err != nil {
		t.Fatalf("购买失败：%v", err)
	}
	if got, want := walletGem(), before-int64(domain.GemShopPrice); got != want {
		t.Errorf("买完钻石 = %d，期望 %d（原 %d − 售价 %d）",
			got, want, before, domain.GemShopPrice)
	}

	// 连买到超出限次 —— 最后一次必须被限次挡住，且**不再扣钱**
	const limit = 3
	for i := 1; i < limit; i++ {
		if _, err := ts.Buy(ctx, uid, item); err != nil {
			t.Fatalf("第 %d 次购买应成功：%v", i+1, err)
		}
	}
	beforeLimit := walletGem()
	if _, err := ts.Buy(ctx, uid, item); err == nil {
		t.Errorf("第 %d 次购买应被限次挡住（limit_per_day=%d）", limit+1, limit)
	}
	if after := walletGem(); after != beforeLimit {
		t.Errorf("被限次挡下的那次仍扣了钱：%d → %d", beforeLimit, after)
	}
}

// TestWashGemSpendsTokenAndKeepsQuality 洗练：扣 1 枚券，品质不变。
func TestWashGemSpendsTokenAndKeepsQuality(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"gem": domain.GemShopPrice})
	ts.grant(t, ctx, uid, map[string]int64{"gem_wash_token": 2})

	item := shopItemIDByCode(t, ts, ctx, "shop_gem")
	if _, err := ts.Buy(ctx, uid, item); err != nil {
		t.Fatalf("购买宝石失败：%v", err)
	}
	gems, err := ts.ListGems(ctx, uid)
	if err != nil || len(gems) != 1 {
		t.Fatalf("应当正好一颗宝石，实际 %d（err=%v）", len(gems), err)
	}
	g := gems[0]

	washed, err := ts.WashGem(ctx, uid, g.ID)
	if err != nil {
		t.Fatalf("洗练失败：%v", err)
	}
	if washed.Quality != g.Quality {
		t.Errorf("洗练把品质从 %q 改成了 %q —— 洗的是词条不是品质，"+
			"升品质必须是另一条途径", g.Quality, washed.Quality)
	}
	if len(washed.Affixes) != len(g.Affixes) {
		t.Errorf("洗练后词条数 = %d，洗前 %d（品质不变则条数不该变）",
			len(washed.Affixes), len(g.Affixes))
	}

	bal := reviveBalanceLike(t, ts, ctx, uid, "gem_wash_token")
	if bal != 1 {
		t.Errorf("洗练券余额 = %d，期望 1（洗一次扣 1）", bal)
	}

	// 再洗一次仍应成功（还有 1 枚）
	if _, err := ts.WashGem(ctx, uid, g.ID); err != nil {
		t.Fatalf("第二次洗练失败：%v", err)
	}
	if bal := reviveBalanceLike(t, ts, ctx, uid, "gem_wash_token"); bal != 0 {
		t.Errorf("洗两次后余额 = %d，期望 0", bal)
	}
	// 第三次必须失败，且**不消耗任何东西**
	if _, err := ts.WashGem(ctx, uid, g.ID); err == nil {
		t.Error("没有洗练券时洗练应失败")
	}
}

// TestWashGemRejectsForeignGem 别人的宝石洗不了。
//
// 归属校验必须在**扣券之前** —— 否则「拿别人的宝石 id 反复请求」虽然会失败，
// 但每一次失败请求都可能已经扣掉了券。
func TestWashGemRejectsForeignGem(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	mine := ts.newUser(t, ctx)
	other := ts.newUser(t, ctx)
	ts.grant(t, ctx, mine, map[string]int64{"gem": domain.GemShopPrice})
	ts.grant(t, ctx, other, map[string]int64{"gem": domain.GemShopPrice})
	ts.grant(t, ctx, other, map[string]int64{"gem_wash_token": 5})

	item := shopItemIDByCode(t, ts, ctx, "shop_gem")
	if _, err := ts.Buy(ctx, mine, item); err != nil {
		t.Fatalf("购买失败：%v", err)
	}
	gems, err := ts.ListGems(ctx, mine)
	if err != nil || len(gems) != 1 {
		t.Fatalf("应有恰好一颗宝石，实际 %d", len(gems))
	}

	if _, err := ts.WashGem(ctx, other, gems[0].ID); err == nil {
		t.Error("洗别人的宝石应失败")
	}
	if bal := reviveBalanceLike(t, ts, ctx, other, "gem_wash_token"); bal != 5 {
		t.Errorf("越权请求竟扣了券：余额 %d，期望仍是 5", bal)
	}
}

// reviveBalanceLike 读某种道具的余额（与 revive_battle_test 里的同名助手同义，
// 这里独立一份是为了让两个测试文件各自读得懂）。
func reviveBalanceLike(t *testing.T, ts *testService, ctx context.Context, uid int64, token string) int64 {
	t.Helper()
	var bal int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT balance FROM user_tokens WHERE user_id = $1 AND token = $2`,
		uid, token).Scan(&bal); err != nil {
		return 0
	}
	return bal
}
