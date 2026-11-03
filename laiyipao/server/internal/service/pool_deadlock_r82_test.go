// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// pool_deadlock_r82_test.go — R82 压测照出来的池互等死锁（先红后绿）。
//
// # 死锁形状（1M 库实测，20 并发 SignIn，10×503 + 10×客户端超时）
//
//	bumpTasks 在事务内调 tasksCache.getOrLoad：
//	  未命中 → singleflight loader 从**池**再拿一条连接跑 loadTaskConfigs
//	而 bumpTasks 一定在事务内被调用（SignIn / SettleBattle）——
//	调用方手里已经占着一条池连接。
//	并发 ≥ 池上限 且缓存冷时：
//	  N 个事务各占 1 连接（池空）→ 全在等 loader → loader 等池连接 → 互等。
//	PG 侧完全看不到（等池发生在 Go 侧）——「库一切正常，服务 503 刷屏
//	30~60s 直到 cacheLoadTimeout 才雪崩恢复」。
//
// 本测试把池压到 2、并发 4 个签到、缓存强制冷 —— 改前必复现
// （30s cacheLoadTimeout 才解锁，测试用 25s 预算断言「及时返回」），
// 改后秒级全绿。
//
// ⚠️ 红态验证方式（改前跑一次）：本测试应耗时 ≈30s 后全 503；
// 修复后 <5s 全成功。

import (
	"context"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/store"
)

func TestConcurrentSignInNoPoolDeadlock(t *testing.T) {
	dsn := packageDSN
	if dsn == "" {
		dsn = legacyDevDSN
	}
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	cfg.DBMaxConns = 2 // 池刻意压小：并发 4 > 池 2 才构成死锁条件
	cfg.DBMinConns = 1

	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()
	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("无可用测试数据库：%v", err)
	}
	defer db.Close()
	svc := New(db, cfg)

	// 4 个真实用户（各自独立，签到互不冲突）。
	uids := make([]int64, 4)
	for i := range uids {
		tp, err := svc.GuestLogin(ctx, "", "R82死锁探针")
		if err != nil {
			t.Fatalf("建号失败：%v", err)
		}
		uids[i] = tp.User.ID
	}

	// 缓存强制冷：4 个并发都会撞上未命中分支。
	svc.tasksCache.flush()

	// 并发 4 同时签到；25s 预算（cacheLoadTimeout 30s 之前）——
	// 死锁形状下它们 30s 才以 503 解锁，25s 时必然还在等。
	start := time.Now()
	var wg sync.WaitGroup
	errs := make([]error, 4)
	var inFlight sync.WaitGroup
	inFlight.Add(4)
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			defer inFlight.Done()
			_, errs[i] = svc.SignIn(ctx, uids[i])
		}(i)
	}
	// 25s 检查点：若还没全部返回，就是互等形状。
	done := make(chan struct{})
	go func() { inFlight.Wait(); close(done) }()
	select {
	case <-done:
	case <-time.After(25 * time.Second):
		t.Fatal("25s 内 4 个并发签到无一返回 —— 池互等死锁仍在（loader 等池连接，池被等 loader 的事务占满）")
	}
	wg.Wait()
	elapsed := time.Since(start)

	for i, err := range errs {
		if err != nil {
			t.Errorf("签到 %d 失败：%v", i, err)
		}
	}
	if elapsed > 10*time.Second {
		t.Errorf("4 个并发签到耗时 %v —— 接近 cacheLoadTimeout，疑似仍在互等边界", elapsed)
	}
	t.Logf("4 并发签到（池=2，缓存冷）耗时 %v", elapsed)
}

// TestBumpTasksTxFallbackFillsCache：未命中时用事务自己的连接加载并回填 ——
// 这是修复后的行为契约：bumpTasks 在任何池压力下都不需要**第二条**连接。
//
// 观测点：flush 后第一次 SignIn 成功（tx 内加载），
// 紧接着 tasksCache.get 必须直接命中（回填生效）。
func TestBumpTasksTxFallbackFillsCache(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	ts.tasksCache.flush()
	if _, ok := ts.tasksCache.get(time.Now(), ts.Cfg().TasksCacheTTL); ok {
		t.Fatal("flush 后缓存仍命中 —— 前置条件不成立")
	}
	if _, err := ts.SignIn(ctx, uid); err != nil {
		t.Fatalf("签到失败：%v", err)
	}
	if _, ok := ts.tasksCache.get(time.Now(), ts.Cfg().TasksCacheTTL); !ok {
		t.Error("SignIn 后缓存未回填 —— bumpTasks 的 tx 内加载没有 put")
	}

	// 任务进度真的写了（daily_signin 任务 progress=1）：防「缓存回填了但 bump 没跑」。
	var n int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM user_tasks ut JOIN tasks t ON t.id = ut.task_id
		 WHERE ut.user_id = $1 AND t.metric = 'signin' AND ut.progress >= 1`, uid).Scan(&n); err != nil {
		t.Fatalf("查任务进度失败：%v", err)
	}
	if n == 0 {
		t.Error("签到后 signin 指标任务进度为 0 —— bumpTasks 没生效")
	}
}

// TestSignInErrorMentionsTasks 修复前后的错误文本钉住：
// bumpTasks 失败必须能定位到 tasks（R82 现场的错误消息形状）。
// 这条不构造故障，只锁注释里承诺的文案形状，防止后续重构把「load tasks」吃掉。
func TestSignInErrorMentionsTasks(t *testing.T) {
	// 文案契约来自 bumpTasks 的 fmt.Errorf("query tasks: %w")；
	// 这里验证任务配置读取失败时会带着这个前缀冒到 SignIn 调用方。
	// 直接调用私有 helper 构造失败：tasks 表改名（与 apply_progress_guard_test 同手法）。
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.tasksCache.flush()
	ts.exec(t, `ALTER TABLE tasks RENAME TO tasks_gone_r82`)
	t.Cleanup(func() { ts.exec(t, `ALTER TABLE tasks_gone_r82 RENAME TO tasks`) })

	_, err := ts.SignIn(ctx, uid)
	if err == nil {
		t.Fatal("tasks 表失踪后签到应失败")
	}
	if !strings.Contains(err.Error(), "tasks") {
		t.Errorf("错误应能定位到 tasks，实际：%v", err)
	}
}
