// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package store

import (
	"context"
	"errors"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/pgconn"
)

// 连接池必须真的把 lock_timeout 下发到会话上，并且锁等待超时由 PG 取消。
//
// 只断言「配置结构体里有这个值」不算证据：RuntimeParams 拼错 key 时
// 结构体照样是对的，会话上却没有限制 —— 那正是 R89 F2 里
// 「241 个请求挂起 15s」的形态，而本测试要在它变成那样之前抓住它。
func TestLockTimeoutIsAppliedAndCancelsLockWait(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBLockTimeout = 1 * time.Second
	ctx := context.Background()

	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	// 会话参数必须真的生效（不是「结构体里有」）。
	var shown string
	if err := db.Pool.QueryRow(ctx, `SHOW lock_timeout`).Scan(&shown); err != nil {
		t.Fatalf("读取 lock_timeout 失败：%v", err)
	}
	if !strings.Contains(shown, "1s") {
		t.Fatalf("会话 lock_timeout = %q，期望 1s", shown)
	}

	// 真造一次锁等待：连接 A 锁住 users 表，连接 B 去查同一张表。
	// 没有 lock_timeout 时 B 会一直挂到 A 释放；1s 预算下必须被 PG 取消。
	if _, err := db.Pool.Exec(ctx, `CREATE TABLE IF NOT EXISTS lock_timeout_probe (id int)`); err != nil {
		t.Fatalf("建探针表失败：%v", err)
	}
	tx, err := db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("起锁事务失败：%v", err)
	}
	// 持锁事务**必须**显式关 lock_timeout：否则它自己也会在 1s 后被取消，
	// 锁就掉了，B 根本等不到锁。
	if _, err := tx.Exec(ctx, `SET LOCAL lock_timeout = 0`); err != nil {
		t.Fatalf("锁事务关 lock_timeout 失败：%v", err)
	}
	if _, err := tx.Exec(ctx, `LOCK TABLE lock_timeout_probe IN ACCESS EXCLUSIVE MODE`); err != nil {
		t.Fatalf("持锁失败：%v", err)
	}
	defer func() { _ = tx.Rollback(context.Background()) }()

	start := time.Now()
	_, err = db.Pool.Exec(ctx, `INSERT INTO lock_timeout_probe (id) VALUES (1)`)
	if err == nil {
		t.Fatal("锁等待在 1s 预算下应被 PG 取消，实际成功写入")
	}
	var pgErr *pgconn.PgError
	if !errors.As(err, &pgErr) {
		t.Fatalf("期望 PG 错误（55P03），实际 %T：%v", err, err)
	}
	// lock_timeout 的错误码是 55P03（lock_not_available），**不是** 57014。
	// R67 的 IsTransientDependency 已把 55P03 显式映射为 503；
	// 没映射时锁故障会以 500 返回 —— 那比「挂 15s」更糟。
	if pgErr.Code != "55P03" {
		t.Fatalf("错误码 = %s，期望 55P03（lock_not_available）", pgErr.Code)
	}
	// 没有真正生效时，客户端会挂到锁事务回滚才返回；
	// 2s 是「确实被提前取消」的判据（预算 1s + 余量）。
	if elapsed := time.Since(start); elapsed > 2*time.Second {
		t.Fatalf("锁等待未被提前取消：耗时 %v（应 < 2s）", elapsed)
	}
}

// 0 = 关闭：显式关掉时不能把 lock_timeout 写成 0 以外的值。
// 迁移复用同一池时必须能关，否则 goose 的长持锁迁移会被误杀。
func TestLockTimeoutZeroLeavesNoLimit(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBLockTimeout = 0
	ctx := context.Background()

	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	var shown string
	if err := db.Pool.QueryRow(ctx, `SHOW lock_timeout`).Scan(&shown); err != nil {
		t.Fatalf("读取 lock_timeout 失败：%v", err)
	}
	if shown != "0" {
		t.Fatalf("关闭时 lock_timeout = %q，期望 0", shown)
	}
}

// lock_timeout 不能影响「没有锁竞争」的语句：正常查询该多快还是多快。
// 这是防「加了预算把正常路径拖慢」的回归判据。
func TestLockTimeoutDoesNotAffectFastPath(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBLockTimeout = 5 * time.Second
	ctx := context.Background()

	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	// 连续 20 次无锁查询：任何一次被误杀都会直接失败。
	for i := 0; i < 20; i++ {
		var one int
		if err := db.Pool.QueryRow(ctx, `SELECT 1`).Scan(&one); err != nil {
			t.Fatalf("第 %d 次无锁查询被误杀：%v", i, err)
		}
	}
}
