package service

// 进程内缓存的纯逻辑守卫（2026-10-23）。
//
// 这一组**不需要数据库**，所以它是本轮唯一能在无 PG 环境下真正执行的
// 缓存判据 —— 其余（看板命中、榜单命中）都要查库，本机只能 Skip。
//
// 为什么值得单独测：缓存的失效方式全是静默的 ——
//   · TTL 判断写反 → 永远不命中（性能问题，没人报错）
//   · 过期不失效   → 永远命中（运营看到的是启动时的快照）
//   · `ttl<=0` 不短路 → 「关闭缓存」这个开关变成摆设

import (
	"testing"
	"time"
)

func TestTTLCacheHitWithinTTL(t *testing.T) {
	var c ttlCache[int]
	now := time.Now()
	if _, ok := c.get(now, time.Second); ok {
		t.Fatal("空缓存不该命中")
	}
	c.put(42, now)
	if v, ok := c.get(now.Add(500*time.Millisecond), time.Second); !ok || v != 42 {
		t.Fatalf("TTL 内应命中 42，实际 v=%d ok=%v", v, ok)
	}
}

func TestTTLCacheExpires(t *testing.T) {
	var c ttlCache[string]
	now := time.Now()
	c.put("x", now)
	// 恰好等于 TTL 也算过期（`>=`）—— 边界取「不命中」是刻意的：
	// 取「命中」会让 TTL 实际变成 TTL+1 个时钟粒度，且难以解释
	if _, ok := c.get(now.Add(time.Second), time.Second); ok {
		t.Fatal("到达 TTL 应视为过期")
	}
}

func TestTTLCacheDisabledWhenTTLIsZeroOrNegative(t *testing.T) {
	for _, ttl := range []time.Duration{0, -time.Second} {
		var c ttlCache[int]
		now := time.Now()
		c.put(7, now)
		if _, ok := c.get(now, ttl); ok {
			t.Fatalf("ttl=%v 应恒不命中（关闭缓存），实际命中", ttl)
		}
	}
}

func TestTTLCacheFlush(t *testing.T) {
	var c ttlCache[int]
	now := time.Now()
	c.put(1, now)
	c.flush()
	if _, ok := c.get(now, time.Minute); ok {
		t.Fatal("flush 后不该命中")
	}
	// flush 后再 put 应恢复正常
	c.put(2, now)
	if v, ok := c.get(now, time.Minute); !ok || v != 2 {
		t.Fatalf("flush 后重新 put 应命中 2，实际 v=%d ok=%v", v, ok)
	}
}

// TestSliceLeaderboard 截断不复制、不越界。
func TestSliceLeaderboard(t *testing.T) {
	all := []LeaderboardEntry{{UserID: 1}, {UserID: 2}, {UserID: 3}}
	if got := sliceLeaderboard(all, 2); len(got) != 2 || got[1].UserID != 2 {
		t.Fatalf("limit=2 应取前两条，实际 %+v", got)
	}
	// limit 大于长度：原样返回（不 panic、不补零）
	if got := sliceLeaderboard(all, 50); len(got) != 3 {
		t.Fatalf("limit=50 应返回全部 3 条，实际 %d 条", len(got))
	}
}

// TestLeaderboardCacheKindsCoverTheThreeBoards 缓存键必须覆盖三个榜。
//
// 这条守的是「加榜时忘了改缓存初始化」：`leaderboardCache[kind]` 取不到时
// 代码会走 `ok == false` 分支**静默跳过缓存**（不报错、功能正常、只是慢）。
// 那种失效没有任何症状，只能靠名单对拍发现。
func TestLeaderboardCacheKindsCoverTheThreeBoards(t *testing.T) {
	got := LeaderboardCacheKinds()
	want := map[string]bool{"power": false, "stage": false, "efficiency": false}
	for _, k := range got {
		if _, ok := want[k]; !ok {
			t.Errorf("缓存里出现了未知榜名 %q", k)
			continue
		}
		want[k] = true
	}
	for k, seen := range want {
		if !seen {
			t.Errorf("榜 %q 没有缓存条目 —— 它会静默地每次都查库", k)
		}
	}
}
