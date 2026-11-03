// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package store

// acquire_timeout_test.go — R90：池耗尽时 Acquire 等待必须有上限。
//
// # 为什么（R89 F2 / R90 探针实测）
//
// 池满且无空闲连接时，pgxpool.Acquire 用请求 ctx 无限排队：请求挂起
// 时长完全由故障持有时间决定（实测 20/241 个请求挂起 15.0s、
// p95=15032ms），期间没有任何一层掐断。DB_ACQUIRE_TIMEOUT 让排队超限
// 变成 ctx.DeadlineExceeded —— 上游 httperr 既有映射 503 + Retry-After
// （R67 契约，零改动）。
//
// 这里的测试全部走**真实 scratch 库 + 真实占满池**（两条 pg_sleep 占住
// 全部连接），不用 mock：Acquire 排队是 pgx/puddle 的内部行为，只有
// 真池真占满才算证据。

import (
	"context"
	"errors"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

// occupyPool 起 n 个 goroutine 各持一条连接跑 pg_sleep，
// 等池被占满（AcquiredConns == n）后返回。
func occupyPool(t *testing.T, db *DB, n int32, sleepSec int) *sync.WaitGroup {
	t.Helper()
	var wg sync.WaitGroup
	for i := int32(0); i < n; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			_, _ = db.Pool.Exec(context.Background(), `SELECT pg_sleep($1)`, sleepSec)
		}()
	}
	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		if db.Pool.Stat().AcquiredConns() == n {
			return &wg
		}
		time.Sleep(20 * time.Millisecond)
	}
	t.Fatalf("池未在 5s 内被占满（acquired=%d，期望 %d）", db.Pool.Stat().AcquiredConns(), n)
	return nil
}

// TestAcquireTimeoutFiresUnderExhaustion 核心断言：池被占满时，第三个
// 请求的 Acquire 必须在 DBAcquireTimeout 后失败（错误链含
// context.DeadlineExceeded 且点名 acquire），而不是挂到占用者结束。
func TestAcquireTimeoutFiresUnderExhaustion(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBMaxConns = 2
	cfg.DBMinConns = 1
	cfg.DBAcquireTimeout = 600 * time.Millisecond
	ctx := context.Background()
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	occupyPool(t, db, 2, 5) // 两条连接各挂 5s

	start := time.Now()
	var one int
	err = db.Pool.QueryRow(ctx, `SELECT 1`).Scan(&one)
	elapsed := time.Since(start)
	if err == nil {
		t.Fatal("池满时第三个请求应因 Acquire 超时失败，实际成功")
	}
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("错误链应含 context.DeadlineExceeded（httperr 据此映 503），实际：%v", err)
	}
	if !strings.Contains(err.Error(), "acquire db connection") {
		t.Errorf("错误应点名是 Acquire 阶段（区别于语句执行超时），实际：%v", err)
	}
	// 占用者要 5s 才放；1.5s 是「确实被自己的超时提前打断」的判据。
	if elapsed > 1500*time.Millisecond {
		t.Fatalf("Acquire 超时未生效：耗时 %v（应 ≈600ms）", elapsed)
	}
}

// TestAcquireTimeoutLeavesNoOrphanTx 用户要求 5 的边界断言：
// Acquire 超时发生在**事务开始之前**，超时后库里绝不能出现
// 「事务已开但没人收尾」的 idle in transaction 孤儿。
func TestAcquireTimeoutLeavesNoOrphanTx(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBMaxConns = 2
	cfg.DBMinConns = 1
	cfg.DBAcquireTimeout = 500 * time.Millisecond
	ctx := context.Background()
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	occupyPool(t, db, 2, 5)

	// Begin 的 Acquire 在 BEGIN 之前：超时 = 还没拿到连接 = 事务未开。
	_, err = db.Pool.Begin(ctx)
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("池满时 Begin 应因 Acquire 超时失败，实际：%v", err)
	}
	// 事务 wrapper（db.Tx → pgx.BeginFunc → pool.Begin）同路径。
	fnRan := false
	err = db.Tx(ctx, func(tx pgx.Tx) error {
		fnRan = true // 不应执行：Begin 阶段已超时
		return nil
	})
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("池满时 db.Tx 应因 Acquire 超时失败，实际：%v", err)
	}
	if fnRan {
		t.Fatal("Acquire 超时后事务函数绝不应执行")
	}
	// 孤儿事务断言（边界）：超时后库里不能出现 idle in transaction。
	// 用第三条连接直查（测试库独占，池外连接不受占满影响）。
	admin, err := pgx.Connect(ctx, cfg.DatabaseURL)
	if err != nil {
		t.Fatalf("直查连接失败：%v", err)
	}
	defer admin.Close(ctx)
	var orphans int
	if err := admin.QueryRow(ctx,
		`SELECT count(*) FROM pg_stat_activity
		  WHERE datname = current_database() AND state = 'idle in transaction'`).Scan(&orphans); err != nil {
		t.Fatalf("查孤儿事务失败：%v", err)
	}
	if orphans != 0 {
		t.Fatalf("Acquire 超时后出现 %d 个 idle in transaction 孤儿（应为 0）", orphans)
	}
}

// TestAcquireTimeoutCancelAfterSuccess 钉住 puddle 语义：Acquire 的 ctx
// 只作用于等待阶段，拿到连接后 actx 被 defer cancel 不影响连接使用。
// wrapper 的每次成功调用都在隐含验证这一点——显式命名防止未来有人
// 把 actx 存起来用。
func TestAcquireTimeoutCancelAfterSuccess(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBAcquireTimeout = 50 * time.Millisecond // 极短：若 actx 泄漏到执行阶段，正常查询也会被误杀
	ctx := context.Background()
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	var one int
	if err := db.Pool.QueryRow(ctx, `SELECT 1`).Scan(&one); err != nil || one != 1 {
		t.Fatalf("QueryRow 在 50ms acquire 超时下应正常（执行不受限）：one=%d err=%v", one, err)
	}
	rows, err := db.Pool.Query(ctx, `SELECT generate_series(1, 3)`)
	if err != nil {
		t.Fatalf("Query 应正常：%v", err)
	}
	n := 0
	for rows.Next() {
		n++
	}
	rows.Close()
	if n != 3 {
		t.Fatalf("Query 应读 3 行，实际 %d", n)
	}
	if _, err := db.Pool.Exec(ctx, `SELECT 1`); err != nil {
		t.Fatalf("Exec 应正常：%v", err)
	}
	tx, err := db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("Begin 应正常：%v", err)
	}
	if err := tx.Rollback(ctx); err != nil {
		t.Fatalf("Rollback 应正常：%v", err)
	}
}

// TestAcquireTimeoutPingCoversReadiness：池满时 Ping 也必须超时返回
// （readyz 不该在池耗尽时挂起）。
func TestAcquireTimeoutPingCoversReadiness(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBMaxConns = 2
	cfg.DBMinConns = 1
	cfg.DBAcquireTimeout = 500 * time.Millisecond
	ctx := context.Background()
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	occupyPool(t, db, 2, 5)

	start := time.Now()
	err = db.Pool.Ping(ctx)
	if err == nil {
		t.Fatal("池满时 Ping 应超时失败")
	}
	if elapsed := time.Since(start); elapsed > 1500*time.Millisecond {
		t.Fatalf("Ping 超时未生效：耗时 %v", elapsed)
	}
}

// TestAcquireTimeoutRecoveryAfterRelease：超时不是终局——占用者释放后
// 请求立即恢复（故障解除语义，与 R89 恢复测量同形状）。
func TestAcquireTimeoutRecoveryAfterRelease(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBMaxConns = 2
	cfg.DBMinConns = 1
	cfg.DBAcquireTimeout = 500 * time.Millisecond
	ctx := context.Background()
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	wg := occupyPool(t, db, 2, 2) // 2s 后自动释放

	_, err = db.Pool.Exec(ctx, `SELECT 1`)
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("占满期应超时，实际：%v", err)
	}
	wg.Wait() // 等占用者释放
	var one int
	if err := db.Pool.QueryRow(ctx, `SELECT 1`).Scan(&one); err != nil || one != 1 {
		t.Fatalf("释放后应立即恢复：one=%d err=%v", one, err)
	}
}
