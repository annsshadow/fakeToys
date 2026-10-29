package service

// 看板 / 榜单缓存的**集成**判据（2026-10-23）。
//
// 这两条用的是同一招：**第一次调用成功之后，把底表改名**。
// 第二次调用若仍然成功，说明它没有查库 —— 那正是「缓存命中」的定义。
//
// 为什么不用「计时」判断：墙钟断言在 CI 上天然易抖（本仓库在
// `perf_baseline.test.ts` 里已经为这件事写过一整段注释）。
// 而「表没了还能返回」是**确定性的**，它不可能因为机器慢而误报。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip —— 那一刻这些断言是
// **未执行**，不是「通过」。

import (
	"context"
	"testing"
	"time"
)

func TestDashboardServedFromCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// ⚠️ 必须先造一个用户。scratch 库是「迁移 + 种子」，**不含任何 users 行**，
	// 于是 `Users.Total` 恒为 0 —— 而第一版断言写的是 `Total < 0`（COUNT(*) 永不为负），
	// 那是一条**不可能失败**的断言，等于没验「缓存返回的是快照而不是零值」。
	ts.newUser(t, ctx)

	first, err := ts.AdminDashboard(ctx)
	if err != nil {
		t.Fatalf("首次 AdminDashboard 失败：%v", err)
	}
	if first.Users.Total < 1 {
		t.Fatalf("夹具自检失败：刚造了用户，Total 应 >= 1，实际 %d", first.Users.Total)
	}

	// 把看板要用的一张表改名 —— 之后任何一次真实查询都必然报错
	ts.renameTable(t, "users", "users_bak")
	defer ts.renameTable(t, "users_bak", "users")

	second, err := ts.AdminDashboard(ctx)
	if err != nil {
		t.Fatalf("TTL 内第二次调用应命中缓存（表已改名，查库必失败），实际报错：%v", err)
	}

	// 核心判据：命中的是**第一次的快照**，不是零值 Dashboard。
	// 零值会让「缓存」退化成「返回空看板」，而运营会看到一片 0。
	if second.Users.Total != first.Users.Total {
		t.Errorf("缓存快照应等于首次结果：first.Users.Total=%d second.Users.Total=%d",
			first.Users.Total, second.Users.Total)
	}
	// DAU 恒有 14 个日桶（generate_series 决定），零值 Dashboard 的 DailyActive 是 nil。
	// 这条同时证明「非零值」与「结构完整」。
	if len(second.DailyActive) != 14 {
		t.Errorf("缓存快照的 DailyActive 应有 14 个日桶，实际 %d 个 —— "+
			"说明 get 未命中时返回了零值却报告命中", len(second.DailyActive))
	}

	// 手动失效后必须重新查库（此时表还是改名的，所以必须报错）
	ts.FlushStatsCache()
	if _, err := ts.AdminDashboard(ctx); err == nil {
		t.Fatal("FlushStatsCache 之后应重新查库并因表缺失而报错")
	}
}

// TestSettleFlushesLeaderboardCache 结算成功后**榜单缓存必须失效**。
//
// # 为什么这条不能省
//
// 榜单缓存 15 秒，而 `/me/rank` 刻意不缓存（刚打完看旧名次最像 bug）。
// 两者一快一慢，同一屏上就会自相矛盾：「我的排名 第 5」配一份 15 秒前的列表。
// 结算是最关键的写路径 —— 它改变的正是榜单分数，所以必须由它来失效缓存。
//
// 判据：结算后把底表改名，`Leaderboard` 必须报错（说明它真的重新查库了）。
func TestSettleFlushesLeaderboardCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 先填充缓存
	if _, err := ts.Leaderboard(ctx, "power", 50); err != nil {
		t.Fatalf("首次 Leaderboard 失败：%v", err)
	}

	// 打一局并结算（失败局即可，走完整结算事务）
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
		t.Fatalf("结算失败：%v", err)
	}

	// 表没了 → 若缓存仍有效，Leaderboard 会「成功」；被 flush 了则必然报错
	ts.renameTable(t, "user_progress", "user_progress_bak")
	defer ts.renameTable(t, "user_progress_bak", "user_progress")

	if _, err := ts.Leaderboard(ctx, "power", 50); err == nil {
		t.Fatal("结算后榜单缓存应已失效（否则玩家刚打完会看到 15 秒前的旧榜），" +
			"实际仍返回成功 = 缓存没被 flush")
	}
}

// TestDashboardCacheDisabledByTTL `DASHBOARD_CACHE_TTL=0` 必须真的关掉缓存。
//
// 这条守的是「开关是摆设」：运维把 TTL 设成 0 期待「每次查库」，
// 若 `get` 没有对 `ttl <= 0` 短路，它反而会一直命中旧值 ——
// 而排障时最依赖的就是这个开关。
func TestDashboardCacheDisabledByTTL(t *testing.T) {
	ts := openScratchService(t)
	ts.Cfg().DashboardCacheTTL = 0
	ctx := context.Background()

	if _, err := ts.AdminDashboard(ctx); err != nil {
		t.Fatalf("首次 AdminDashboard 失败：%v", err)
	}
	ts.renameTable(t, "users", "users_bak")
	defer ts.renameTable(t, "users_bak", "users")

	if _, err := ts.AdminDashboard(ctx); err == nil {
		t.Fatal("TTL=0 时第二次调用必须查库（并因表缺失报错），实际成功 = 缓存没关掉")
	}
}

func TestLeaderboardCached(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 造一个能进榜的用户
	uid := ts.newUser(t, ctx)
	ts.exec(t, `UPDATE user_progress SET level_exp = 100 WHERE user_id = $1`, uid)

	first, err := ts.Leaderboard(ctx, "power", 50)
	if err != nil {
		t.Fatalf("首次 Leaderboard 失败：%v", err)
	}
	if len(first) == 0 {
		t.Fatal("刚造了用户，榜单不该为空")
	}

	ts.renameTable(t, "user_progress", "user_progress_bak")
	defer ts.renameTable(t, "user_progress_bak", "user_progress")

	second, err := ts.Leaderboard(ctx, "power", 50)
	if err != nil {
		t.Fatalf("TTL 内第二次应命中缓存（表已改名），实际报错：%v", err)
	}
	if len(second) != len(first) || second[0].UserID != first[0].UserID {
		t.Fatalf("缓存快照应与首次一致：first=%+v second=%+v", first, second)
	}

	ts.FlushStatsCache()
	if _, err := ts.Leaderboard(ctx, "power", 50); err == nil {
		t.Fatal("FlushStatsCache 之后应重新查库并因表缺失而报错")
	}
}

// TestLeaderboardLimitSlicesCachedRows 缓存的是 200 行，limit 只影响切片。
//
// 判据：同一份缓存下 limit=2 的结果必须是 limit=50 结果的前两条
// （而不是「另外查了一次 limit=2」—— 表已改名，真查必失败）。
func TestLeaderboardLimitSlicesCachedRows(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	for i := 0; i < 3; i++ {
		uid := ts.newUser(t, ctx)
		ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, int64(100-i*10))
	}
	if _, err := ts.Leaderboard(ctx, "power", 50); err != nil {
		t.Fatalf("首次 Leaderboard 失败：%v", err)
	}
	ts.renameTable(t, "user_progress", "user_progress_bak")
	defer ts.renameTable(t, "user_progress_bak", "user_progress")

	two, err := ts.Leaderboard(ctx, "power", 2)
	if err != nil {
		t.Fatalf("limit=2 应命中缓存，实际：%v", err)
	}
	if len(two) != 2 {
		t.Fatalf("limit=2 应返回 2 条，实际 %d 条", len(two))
	}
	if two[0].Rank != 1 || two[1].Rank != 2 {
		t.Fatalf("切片后名次应仍是 1,2，实际 %d,%d", two[0].Rank, two[1].Rank)
	}
}

// TestMyRankNotCached 「我的排名」刻意**不进缓存**。
//
// 理由：15 秒的陈旧对榜单可以接受，对「我刚打完一局，我的名次」不可接受 ——
// 那是玩家最容易当成 bug 的一处陈旧。
//
// 判据：MyRank 在表改名后必须报错（说明它每次都查库）。
func TestMyRankNotCached(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.exec(t, `UPDATE user_progress SET level_exp = 10 WHERE user_id = $1`, uid)

	if _, err := ts.MyRank(ctx, uid, "power"); err != nil {
		t.Fatalf("首次 MyRank 失败：%v", err)
	}
	ts.renameTable(t, "user_progress", "user_progress_bak")
	defer ts.renameTable(t, "user_progress_bak", "user_progress")

	if _, err := ts.MyRank(ctx, uid, "power"); err == nil {
		t.Fatal("MyRank 不该被缓存：表已改名却仍返回成功")
	}
}

// TestDashboardCacheTTLDefaultsAreSane 默认值必须是「有缓存但不长」。
func TestDashboardCacheTTLDefaultsAreSane(t *testing.T) {
	ts := openTestService(t)
	if ts.Cfg().DashboardCacheTTL <= 0 {
		t.Errorf("默认 DASHBOARD_CACHE_TTL 应 > 0，实际 %v", ts.Cfg().DashboardCacheTTL)
	}
	if ts.Cfg().DashboardCacheTTL > 5*time.Minute {
		t.Errorf("看板缓存超过 5 分钟就太旧了（运营会以为数据坏了），实际 %v",
			ts.Cfg().DashboardCacheTTL)
	}
	if ts.Cfg().LeaderboardCacheTTL <= 0 {
		t.Errorf("默认 LEADERBOARD_CACHE_TTL 应 > 0，实际 %v", ts.Cfg().LeaderboardCacheTTL)
	}
	// 榜单比看板更「玩家可感知」，所以它的缓存不该比看板长
	if ts.Cfg().LeaderboardCacheTTL > ts.Cfg().DashboardCacheTTL {
		t.Errorf("榜单缓存（%v）不该长于看板缓存（%v）：前者玩家能感知",
			ts.Cfg().LeaderboardCacheTTL, ts.Cfg().DashboardCacheTTL)
	}
}
