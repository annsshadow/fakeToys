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

// 连接池必须真的把 statement_timeout 下发到会话上，并且超时由 PG 取消。
//
// 只断言「配置结构体里有这个值」不算证据：RuntimeParams 拼错 key 时
// 结构体照样是对的，会话上却没有限制。
func TestStatementTimeoutIsAppliedAndCancels(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBStatementTimeout = 2 * time.Second
	ctx := context.Background()

	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	var shown string
	if err := db.Pool.QueryRow(ctx, `SHOW statement_timeout`).Scan(&shown); err != nil {
		t.Fatalf("读取 statement_timeout 失败：%v", err)
	}
	if !strings.Contains(shown, "2s") {
		t.Fatalf("会话 statement_timeout = %q，期望 2s", shown)
	}

	// 真跑一条超时查询：pg_sleep(3) 在 2s 上限下必须被 PG 取消。
	start := time.Now()
	_, err = db.Pool.Exec(ctx, `SELECT pg_sleep(3)`)
	if err == nil {
		t.Fatal("pg_sleep(3) 在有 2s 上限时应被取消，实际成功返回")
	}
	var pgErr *pgconn.PgError
	if !errors.As(err, &pgErr) {
		t.Fatalf("期望 PG 错误（57014），实际 %T：%v", err, err)
	}
	if pgErr.Code != "57014" {
		t.Fatalf("错误码 = %s，期望 57014（query_canceled）", pgErr.Code)
	}
	// 若没有真正生效，客户端会等满 3s 才返回；2.5s 是「确实被提前取消」的判据。
	if elapsed := time.Since(start); elapsed > 2500*time.Millisecond {
		t.Fatalf("超时未生效：耗时 %v（应 < 2.5s）", elapsed)
	}
}

// 0 = 关闭：显式关掉时不能把 statement_timeout 写成 0 以外的值，
// 也不能因为「没设」而意外继承一个限制。
func TestStatementTimeoutZeroLeavesNoLimit(t *testing.T) {
	cfg := openCfg(scratchDB(t))
	cfg.DBStatementTimeout = 0
	ctx := context.Background()

	db, err := Open(ctx, cfg)
	if err != nil {
		t.Fatalf("打开测试库失败：%v", err)
	}
	defer db.Close()

	var shown string
	if err := db.Pool.QueryRow(ctx, `SHOW statement_timeout`).Scan(&shown); err != nil {
		t.Fatalf("读取 statement_timeout 失败：%v", err)
	}
	if shown != "0" {
		t.Fatalf("关闭时 statement_timeout = %q，期望 0", shown)
	}
}
