// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R67：资源泄漏探针（goroutine 曲线 + 连接池回收）。
//
// 判据：负载结束后 goroutine 数必须回落到基线附近（允许固定后台 goroutine
// 的小幅抖动），且连接池 acquired 必须回到 0。若请求路径里有 goroutine
// 或连接忘记归还，曲线会单调上升，这里会直接失败。
//
// 负载：8 个并发 worker × 200 次 × 9 个端点（普通读 + 需鉴权读 + 后台读）。

import (
	"net/http"
	"runtime"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

func TestNoGoroutineGrowthUnderLoad(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "leak")
	_, adminTok := e.newAdmin(t, "leak")

	endpoints := []struct{ url, token string }{
		{"/api/v1/config", tok},
		{"/api/v1/leaderboard?type=power", ""},
		{"/api/v1/wallet", tok},
		{"/api/v1/me/stars", tok},
		{"/api/v1/me/rank", tok},
		{"/api/v1/shop", tok},
		{"/api/v1/tasks?scope=daily", tok},
		{"/api/v1/admin/dashboard", adminTok},
		{"/api/v1/admin/users?limit=5", adminTok},
	}
	for _, ep := range endpoints {
		if code, err := cacheRaceGet(e.app, ep.url, ep.token); err != nil || code != http.StatusOK {
			t.Fatalf("预热 %s 失败：status=%d err=%v", ep.url, code, err)
		}
	}
	// 让预热产生的短生命周期 goroutine 先退出，再取基线。
	time.Sleep(200 * time.Millisecond)
	baseline := runtime.NumGoroutine()

	const workers = 8
	const iterations = 200
	var (
		wg       sync.WaitGroup
		failures atomic.Int64
	)
	for w := 0; w < workers; w++ {
		wg.Add(1)
		go func(w int) {
			defer wg.Done()
			for i := 0; i < iterations; i++ {
				ep := endpoints[(w+i)%len(endpoints)]
				if code, err := cacheRaceGet(e.app, ep.url, ep.token); err != nil || code != http.StatusOK {
					failures.Add(1)
				}
			}
		}(w)
	}
	wg.Wait()

	if n := failures.Load(); n > 0 {
		t.Fatalf("压测期间 %d 个请求失败，泄漏结论不可信", n)
	}

	// 允许固定后台 goroutine（pgxpool 健康检查、limiter GC、Fiber 等）的
	// 小幅抖动；超过基线 +4 视为随请求增长。
	deadline := time.Now().Add(3 * time.Second)
	var final int
	for {
		final = runtime.NumGoroutine()
		if final <= baseline+4 || time.Now().After(deadline) {
			break
		}
		time.Sleep(50 * time.Millisecond)
	}
	if final > baseline+4 {
		t.Fatalf("goroutine 未回落：基线 %d，压测 %d 次后 %d（疑似随请求泄漏）",
			baseline, workers*iterations, final)
	}

	// 连接池：所有连接的 Acquire 都必须已归还。
	deadline = time.Now().Add(3 * time.Second)
	for {
		if e.svc.DB.Pool.Stat().AcquiredConns() == 0 || time.Now().After(deadline) {
			break
		}
		time.Sleep(50 * time.Millisecond)
	}
	if n := e.svc.DB.Pool.Stat().AcquiredConns(); n != 0 {
		t.Fatalf("压测结束后仍有 %d 个连接未归还", n)
	}
	t.Logf("泄漏曲线：基线 goroutine=%d，%d 次请求后=%d；AcquiredConns=%d",
		baseline, workers*iterations, final, e.svc.DB.Pool.Stat().AcquiredConns())
}
