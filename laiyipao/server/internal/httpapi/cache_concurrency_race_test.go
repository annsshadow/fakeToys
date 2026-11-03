// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R66：共享缓存出参的并发审计（`go test -race` 跑）。
//
// # 负载构成（写进用例里，避免「全绿只是因为没打对地方」）
//
//   - 32 个读 goroutine，每个 20 次请求，轮流打 12 个端点；
//   - 端点覆盖：/config、三个榜单、公告、签到日历、签到状态、任务、商城、
//     后台看板、后台经济页、后台玩家列表；
//   - 2 个写者与读**同时**运行：
//     ① 每 200ms 调一次 `FlushStatsCache()`（强制看板/榜单与读并发重载）；
//     ② 每 5ms 调一次 `ReloadConfig` 并翻转 `PlayerLoginRequired`
//     （强制配置快照与 /config 字节缓存换代）；
//   - 所有缓存 TTL 压到 10ms，TTL 边界的高频重载/单飞路径会被反复走到。
//
// 如果某个出参与缓存共享 map 且 handler 就地写入，这里会触发
// `fatal error: concurrent map iteration and map write`（进程直接退出，
// 用例不可能「变红」而是整包崩）或 `-race` 报 data race。

import (
	"fmt"
	"net/http"
	"net/http/httptest"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
)

// cacheRaceGet 是并发用例专用的请求函数：不在 goroutine 里调用 t.Fatal
// （FailNow 只能在测试主 goroutine 里用），只回传状态码与错误。
func cacheRaceGet(app *fiber.App, url, token string) (int, error) {
	req := httptest.NewRequest(http.MethodGet, url, nil)
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	resp, err := app.Test(req, 10000)
	if err != nil {
		return 0, err
	}
	defer resp.Body.Close()
	return resp.StatusCode, nil
}

func TestCacheSharedStateRace(t *testing.T) {
	// 迁移到顶的一次性库：共享开发库可能还是旧 schema，缺 00025 的增量表，
	// 会把「库没迁移」误判成并发缺陷（42P01）。
	e2eScratchDB(t)
	short := 10 * time.Millisecond
	e := newE2EWith(t, func(cfg *config.Config) {
		cfg.DashboardCacheTTL = short
		cfg.LeaderboardCacheTTL = short
		cfg.SignInCalendarCacheTTL = short
		cfg.AnnouncementsCacheTTL = short
		cfg.ShopCacheTTL = short
		cfg.TasksCacheTTL = short
		cfg.EconomyCacheTTL = short
		cfg.AdminUsersCountCacheTTL = short
	})
	_, tok := e.newPlayer(t, "cache_race")
	_, adminTok := e.newAdmin(t, "cache_race")

	endpoints := []struct{ url, token string }{
		{"/api/v1/config", tok},
		{"/api/v1/leaderboard?type=power", ""},
		{"/api/v1/leaderboard?type=stage", ""},
		{"/api/v1/leaderboard?type=efficiency", ""},
		{"/api/v1/announcements?limit=5", ""},
		{"/api/v1/signin/calendar", tok},
		{"/api/v1/signin/status", tok},
		{"/api/v1/tasks?scope=daily", tok},
		{"/api/v1/shop", tok},
		{"/api/v1/admin/dashboard", adminTok},
		{"/api/v1/admin/economy", adminTok},
		{"/api/v1/admin/users?limit=5", adminTok},
	}

	// 预热：先确认 12 个端点在当前夹具下都可用，避免把「路由没挂」误判成并发问题。
	for _, ep := range endpoints {
		code, err := cacheRaceGet(e.app, ep.url, ep.token)
		if err != nil || code != http.StatusOK {
			t.Fatalf("预热 %s 失败：status=%d err=%v", ep.url, code, err)
		}
	}

	const readers = 32
	const iterations = 20

	// 写者与读者在同一时间窗内运行。
	stop := make(chan struct{})
	var writers sync.WaitGroup
	writers.Add(2)
	go func() { // 写者①：缓存失效，强制重载与读并发
		defer writers.Done()
		for {
			select {
			case <-stop:
				return
			default:
				e.svc.FlushStatsCache()
				time.Sleep(200 * time.Millisecond)
			}
		}
	}()
	go func() { // 写者②：配置快照换代（含 /config 的 require_login 切换）
		defer writers.Done()
		base := *e.svc.Cfg()
		for i := 0; ; i++ {
			select {
			case <-stop:
				return
			default:
				next := base
				next.PlayerLoginRequired = i%2 == 0
				e.svc.ReloadConfig(next)
				time.Sleep(5 * time.Millisecond)
			}
		}
	}()

	var (
		wg       sync.WaitGroup
		failures atomic.Int64
		errMu    sync.Mutex
		firstErr string
	)
	record := func(s string) {
		failures.Add(1)
		errMu.Lock()
		if firstErr == "" {
			firstErr = s
		}
		errMu.Unlock()
	}

	for g := 0; g < readers; g++ {
		wg.Add(1)
		go func(g int) {
			defer wg.Done()
			for i := 0; i < iterations; i++ {
				ep := endpoints[(g+i)%len(endpoints)]
				code, err := cacheRaceGet(e.app, ep.url, ep.token)
				if err != nil {
					record(fmt.Sprintf("%s: %v", ep.url, err))
					continue
				}
				if code != http.StatusOK {
					record(fmt.Sprintf("%s: status=%d", ep.url, code))
				}
			}
		}(g)
	}
	wg.Wait()
	close(stop)
	writers.Wait()

	if n := failures.Load(); n > 0 {
		errMu.Lock()
		msg := firstErr
		errMu.Unlock()
		t.Fatalf("%d 个请求失败（首个：%s）", n, msg)
	}
}
