package service

import (
	"context"
	"testing"
)

// 体力被动回复**不得因为读取频繁而丢量**（第 86 轮）。
//
// # 缺陷：每次推进都把不足一格的余数丢掉
//
// 原写法里 `energy_updated_at` 推进到 `now()`：
//
//	energy_updated_at = CASE WHEN elapsed >= interval THEN now() ELSE energy_updated_at END
//
// `now()` 落在**下一次读取的时刻**上，于是两次读取之间不足一格的余数
// 被永久丢弃。
//
// 而客户端每次进页面都 `fetchWallet()`（`index.vue` / `bag.vue` /
// `me.vue` / `battle.vue` 结算后都会调），所以读取间隔（3~5 分钟）
// **短于**回复间隔（6 分钟）—— 正是最容易丢的形态。
//
// 按 5 分钟读一次的账：
//
//	读点    已过    应回    旧实现实得
//	300s    300     0       0
//	600s    600     1       1   ← 余数 240 在这里被吞
//	900s    900     2       1
//	1200s   1200    3       1   ← 又吞 240
//	…
//	3600s   3600    10      6
//
// **少回 40%。**
//
// # 判据：逐帧比对「累计值 == FLOOR(已过秒数 / 间隔)」
//
// ⚠️ 判据不能只断言「单次读取回 1 点」—— 那条在修复前后都成立，
// 一次通过是**巧合**（读得够久就直接回满一格）。
//
// 期望值在两种实现下分别是 6 与 10，所以它落在判据真正分界的地方。
//
// # 为什么逐帧而不是只看末尾
//
// 我第一版写的是「把起点拨到 1 小时前，然后读 12 次，比总量」——
// 实测得 9、期望 10，**差的那 1 点是真实执行耗时**：
//
// 每次迭代我用 `UPDATE … energy_updated_at = energy_updated_at + 300s`
// 模拟时钟前进，而 `elapsed = now() - at` 里 `now()` 也在真实前进，
// 于是 3600s 的窗口被削掉约 1.5s，FLOOR 从 10 掉到 9。
//
// **那是我的判据有缺陷，不是实现有缺陷。**
//
// 更糟的是我第一版的方向就是反的：`at` 往前推会让 `elapsed` **变小**
// （`elapsed = now() - at`），而我要的是模拟 `now()` 前进。
// 改成每次把 `at` **往回拨** 300s，方向才对。
//
// 逐帧比对还带来一个额外好处：一旦某一步对不上，
// 报错直接指向那一步，而不是「1 小时后少了 1 点」这种难定位的数字。
func TestEnergyRegenDoesNotLoseFractionalRemainder(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	const intervalSecs = 360 // 6 分钟，与 LoadWallet 一致
	const readEvery = 300    // 5 分钟读一次 —— 比间隔短，是最易丢的形态
	const steps = 24         // 覆盖 2 小时，跨越至少一个「卡在格点」的读点

	// 起点：`energy_updated_at = now()`（已过 0 秒），能量清零。
	if _, err := ts.pool.Exec(ctx,
		`UPDATE user_wallets
		    SET energy = 0, energy_updated_at = now()
		  WHERE user_id = $1`, uid); err != nil {
		t.Fatalf("布置起点失败：%v", err)
	}

	for i := 1; i <= steps; i++ {
		// 模拟「第 i 次读取发生在起点后 300*i 秒」。
		//
		// ⚠️ 方向：`elapsed = now() - energy_updated_at`，
		// 所以要模拟时间前进必须把 `at` **往回拨**，不是往前推。
		// 往前推会让 elapsed 变小 —— 我第一版就搞反了这一点，
		// 结果测出「少回 1 点」并差点去改一个没坏的实现。
		if _, err := ts.pool.Exec(ctx,
			`UPDATE user_wallets
			    SET energy_updated_at = energy_updated_at - make_interval(secs => $2::double precision)
			  WHERE user_id = $1`, uid, readEvery); err != nil {
			t.Fatalf("推进时间轴失败（第 %d 次）：%v", i, err)
		}
		if _, err := ts.LoadWallet(ctx, uid); err != nil {
			t.Fatalf("第 %d 次 LoadWallet 失败：%v", i, err)
		}

		elapsed := int64(i * readEvery)
		want := elapsed / intervalSecs
		got := ts.wallet(t, ctx, uid).Energy
		if got != want {
			t.Fatalf("第 %d 次读取（已过 %ds）后体力 = %d，期望 %d\n"+
				"（区间 %ds，期望按 %ds 一格回 %d 点）\n\n"+
				"原因：`energy_updated_at` 被推到 `now()`，两次读取之间"+
				"不足一格的余数被永久丢弃。\n"+
				"修法：推进量 = 整格数 × 间隔，让余数留在时间戳里继续累积。",
				i, elapsed, got, want, elapsed, intervalSecs, want)
		}
	}
}

// TestEnergyRegenRemainderCarriesOver 单独钉住「余数要留下来」。
//
// 这条比上面那条更尖锐：它只做**一次**跨格读取，
// 然后验证时间戳确实只前进了**一格**而不是「跳到读取时刻」。
//
// 两条分开的原因是：上面那条可能因为别的巧合通过，
// 这条不可能 —— 它直接断言时间戳的推进量。
func TestEnergyRegenRemainderCarriesOver(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	const intervalSecs = 360

	// 起点：410 秒前（1 格 + 50 秒余数），能量清零。
	if _, err := ts.pool.Exec(ctx,
		`UPDATE user_wallets
		    SET energy = 0,
		        energy_updated_at = now() - make_interval(secs => 410.0)
		  WHERE user_id = $1`, uid); err != nil {
		t.Fatalf("布置起点失败：%v", err)
	}

	if _, err := ts.LoadWallet(ctx, uid); err != nil {
		t.Fatalf("LoadWallet 失败：%v", err)
	}

	w := ts.wallet(t, ctx, uid)
	if w.Energy != 1 {
		t.Errorf("410 秒应回 1 点体力（余数不够一格），实际 %d", w.Energy)
	}

	// ⚠️ 关键断言：时间戳只应前进 **1 格**，不能前进 410 秒。
	// 前进 410 秒就意味着那 50 秒余数被吞掉了。
	var remaining float64
	if err := ts.pool.QueryRow(ctx,
		`SELECT EXTRACT(EPOCH FROM (now() - energy_updated_at))
		   FROM user_wallets WHERE user_id = $1`, uid).Scan(&remaining); err != nil {
		t.Fatalf("读时间戳失败：%v", err)
	}
	// 余数应约等于 50 秒（加上这次 LoadWallet 本身的执行耗时）。
	if remaining < 45 || remaining > 55 {
		t.Errorf("读取后余数应约 50 秒（410 - 360），实测 %.1f 秒\n"+
			"若接近 0 说明 `energy_updated_at` 被推到了 now() —— "+
			"余数被吞，长时间看会少回体力。", remaining)
	}
}

// TestEnergyRegenStillCapsAtMax 确认修复没把上限搞坏。
func TestEnergyRegenStillCapsAtMax(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 上限是 120。起点：9999 秒前（27 格）+ 能量 100 → 应夹在 120。
	if _, err := ts.pool.Exec(ctx,
		`UPDATE user_wallets
		    SET energy = 100,
		        energy_updated_at = now() - make_interval(secs => 9999.0)
		  WHERE user_id = $1`, uid); err != nil {
		t.Fatalf("布置起点失败：%v", err)
	}
	if _, err := ts.LoadWallet(ctx, uid); err != nil {
		t.Fatalf("LoadWallet 失败：%v", err)
	}
	if e := ts.wallet(t, ctx, uid).Energy; e != 120 {
		t.Errorf("体力应夹在上限 120，实际 %d", e)
	}
}

// TestEnergyRegenNoDoubleCount 确认反复读不重复加点。
//
// ⚠️ 这是「余数累积」最常见的失败形态：修好了余数，却让
// 每次读取都加 1 —— 于是「每进一次页面就 +1 体力」，
// 那是刷体力漏洞。
func TestEnergyRegenNoDoubleCount(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 起点：100 秒前（不够一格），能量 10。
	if _, err := ts.pool.Exec(ctx,
		`UPDATE user_wallets
		    SET energy = 10,
		        energy_updated_at = now() - make_interval(secs => 100.0)
		  WHERE user_id = $1`, uid); err != nil {
		t.Fatalf("布置起点失败：%v", err)
	}

	// 连读 50 次，一次都不推进时间轴 —— 一格都没过，不该加任何点。
	for i := 0; i < 50; i++ {
		if _, err := ts.LoadWallet(ctx, uid); err != nil {
			t.Fatalf("第 %d 次 LoadWallet 失败：%v", i, err)
		}
	}
	if e := ts.wallet(t, ctx, uid).Energy; e != 10 {
		t.Errorf("未满一格时连读 50 次，体力从 10 变成 %d —— 重复加点了", e)
	}
}
