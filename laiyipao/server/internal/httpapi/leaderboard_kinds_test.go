package httpapi

// 榜名单的**双向对拍**（2026-10-23）。
//
// `leaderboardKinds`（本包，校验请求参数）与 `service.leaderboardCacheKinds`
// （缓存初始化）是两份独立的三个字符串。加第四个榜时只改一处不会报错：
//
//   - 只改本包  → 新榜能查，但没有缓存（每次都全表 GROUP BY）—— 静默变慢
//   - 只改 service → 新榜有缓存，但路由拒绝 —— 静默不可用
//
// 两种都不会让任何测试变红，所以必须显式对拍。
//
// 这条不需要数据库：两个名单都是编译期常量。

import (
	"sort"
	"testing"

	"github.com/laiyipao/server/internal/service"
)

func TestEveryLeaderboardKindHasCache(t *testing.T) {
	routeKinds := make([]string, 0, len(leaderboardKinds))
	for k := range leaderboardKinds {
		routeKinds = append(routeKinds, k)
	}
	cacheKinds := service.LeaderboardCacheKinds()

	sort.Strings(routeKinds)
	sort.Strings(cacheKinds)

	if len(routeKinds) != len(cacheKinds) {
		t.Fatalf("路由允许 %d 个榜 %v，缓存只覆盖 %d 个 %v —— 多出来的那个会静默地每次都查库",
			len(routeKinds), routeKinds, len(cacheKinds), cacheKinds)
	}
	for i := range routeKinds {
		if routeKinds[i] != cacheKinds[i] {
			t.Fatalf("榜名单不一致：路由侧 %v，缓存侧 %v", routeKinds, cacheKinds)
		}
	}
}
