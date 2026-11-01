package service

// 签到日历缓存的集成判据（2026-10-30 实质优化第 17 轮）。
//
// 与看板/榜单缓存同一招：第一次调用成功后把 `sign_in_calendar` 表改名，
// 第二次若仍成功返回相同数据 → 证明没有查库（缓存命中）。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip —— 那一刻这些断言是
// **未执行**，不是「通过」。

import (
	"context"
	"testing"
)

func TestSignInCalendarServedFromCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	first, err := ts.SignInCalendar(ctx)
	if err != nil {
		t.Fatalf("首次 SignInCalendar 失败：%v", err)
	}
	if len(first) != 7 {
		t.Fatalf("夹具自检失败：种子日历应有 7 天，实际 %d", len(first))
	}

	// 把日历表改名 —— 之后任何一次真实查询都必然报错
	ts.renameTable(t, "sign_in_calendar", "sign_in_calendar_bak")
	defer ts.renameTable(t, "sign_in_calendar_bak", "sign_in_calendar")

	second, err := ts.SignInCalendar(ctx)
	if err != nil {
		t.Fatalf("TTL 内第二次调用应命中缓存（表已改名，查库必失败），实际报错：%v", err)
	}
	if len(second) != len(first) {
		t.Errorf("缓存快照应等于首次结果：first %d 天，second %d 天", len(first), len(second))
	}
	for i := range first {
		if second[i].Day != first[i].Day || len(second[i].Reward) != len(first[i].Reward) {
			t.Errorf("第 %d 天缓存快照不一致：first=%+v second=%+v", i, first[i], second[i])
		}
	}

	// 统计：第一次 miss，第二次 hit
	hits, misses := ts.signInCalendarCache.Stats()
	if hits < 1 {
		t.Errorf("第二次调用应计命中，实际 hits=%d", hits)
	}
	if misses < 1 {
		t.Errorf("第一次调用应计未命中，实际 misses=%d", misses)
	}

	// 手动失效后必须重新查库（此时表还是改名的，所以必须报错）
	ts.signInCalendarCache.flush()
	if _, err := ts.SignInCalendar(ctx); err == nil {
		t.Fatal("flush 后应重新查库（表已改名，必须报错）")
	}
}

func TestSignInCalendarCacheTTLZeroDisables(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 0 = 关闭缓存（与看板/榜单同一条运维命令）
	ts.Cfg().SignInCalendarCacheTTL = 0

	if _, err := ts.SignInCalendar(ctx); err != nil {
		t.Fatalf("首次 SignInCalendar 失败：%v", err)
	}
	ts.renameTable(t, "sign_in_calendar", "sign_in_calendar_bak")
	defer ts.renameTable(t, "sign_in_calendar_bak", "sign_in_calendar")

	if _, err := ts.SignInCalendar(ctx); err == nil {
		t.Fatal("TTL=0 时缓存关闭，表已改名，第二次调用必须查库报错")
	}
}

func TestSignInCalendarCacheConcurrentSingleflight(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// singleflight 的「loader 只跑一次」已在 ttlcache_test.go 覆盖。
	// 这里只验证缓存可被并发读：所有请求都成功、结果一致。
	// misses 按**请求**计数（每个在 TTL 之外到达的请求都计一次），
	// 不是按实际查询次数计 —— 见 ttlCache.getOrLoad 的注释。
	done := make(chan error, 8)
	results := make([][]SignInCalendarDay, 8)
	for i := 0; i < 8; i++ {
		go func(i int) {
			res, err := ts.SignInCalendar(ctx)
			results[i] = res
			done <- err
		}(i)
	}
	for i := 0; i < 8; i++ {
		if err := <-done; err != nil {
			t.Fatalf("并发 SignInCalendar 失败：%v", err)
		}
	}
	for i := 1; i < 8; i++ {
		if len(results[i]) != len(results[0]) {
			t.Errorf("并发结果不一致：第 %d 个 %d 天 vs 第 0 个 %d 天",
				i, len(results[i]), len(results[0]))
		}
	}
	_, misses := ts.signInCalendarCache.Stats()
	if misses != 8 {
		t.Errorf("8 个并发请求都应计未命中（按请求计），实际 misses=%d", misses)
	}
}
