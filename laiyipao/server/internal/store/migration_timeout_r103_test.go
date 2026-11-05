// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package store

// R103 的回归：迁移的超时/锁预算必须与运行期解耦。
//
// 两条用例都来自真实复现（见台账 R103 的改前原文）：
//   1. 65s 的迁移被启动 ctx 在 60.09s 砍断（context deadline exceeded）；
//   2. 持锁 DDL 被运行期 lock_timeout 在 2.07s 砍断（55P03）。
//
// 这里把「砍断」改成「成功完成」，并额外用迁移内部的 current_setting
// 钉住迁移连接真的是 statement_timeout=0 / lock_timeout=0 ——
// 只看「成功」不够：把超时调大也能成功，但那样运行期预算仍然泄漏到迁移里。

import (
	"context"
	"testing"
	"testing/fstest"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/pressly/goose/v3"
)

// upOnly 把 runMigrations 的注入点收窄成「只做 Up」。
func upOnly(mctx context.Context, p *goose.Provider) error {
	_, err := p.Up(mctx)
	return err
}

func slowMigrationFS() fstest.MapFS {
	return fstest.MapFS{
		"00001_slow.sql": &fstest.MapFile{
			Data: []byte("-- +goose Up\nSELECT pg_sleep(63);\n\n-- +goose Down\nSELECT 1;\n"),
		},
	}
}

// lockMigrationFS 的 Up 先做需要 ACCESS EXCLUSIVE 的 DDL，再把迁移会话的
// 两个设置写进一张表，供测试断言。
func lockMigrationFS() fstest.MapFS {
	return fstest.MapFS{
		"00001_alter.sql": &fstest.MapFile{
			Data: []byte(`-- +goose Up
ALTER TABLE r103_lock_target ADD COLUMN note int;
CREATE TABLE r103_migration_settings AS
  SELECT current_setting('lock_timeout') AS lock_timeout,
         current_setting('statement_timeout') AS statement_timeout;

-- +goose Down
DROP TABLE r103_migration_settings;
ALTER TABLE r103_lock_target DROP COLUMN note;
`),
		},
	}
}

// TestMigrateOutlivesCallerDeadlineR103：调用方 ctx 60s + 迁移 63s。
// 修前必红（60s 处 context deadline exceeded）；修后必须成功。
func TestMigrateOutlivesCallerDeadlineR103(t *testing.T) {
	dsn := scratchDB(t)
	// 调用方的 deadline 故意短于迁移耗时：它正是修前砍断迁移的那个 60s 启动 ctx。
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	cfg := openCfg(dsn)
	cfg.MigrateTimeout = 0 // 0 = 不限，交给迁移自己跑完
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)

	start := time.Now()
	if err := db.runMigrations(ctx, slowMigrationFS(), upOnly); err != nil {
		t.Fatalf("R103：迁移不应被调用方 60s deadline 砍断：%v", err)
	}
	if elapsed := time.Since(start); elapsed < 61*time.Second {
		t.Fatalf("迁移只用了 %v（应 >60s）—— 用例没有真正越过 60s 边界", elapsed)
	}
}

// TestMigrateIgnoresRuntimeLockBudgetR103：外部会话持 users 类的表锁 5s，
// 迁移 DDL 必须等到锁释放并成功，而不是在 2s 的 lock_timeout 上失败。
func TestMigrateIgnoresRuntimeLockBudgetR103(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()

	cfg := openCfg(dsn)
	cfg.DBLockTimeout = 2 * time.Second
	cfg.DBStatementTimeout = 30 * time.Second
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)

	if _, err := db.Pool.Exec(ctx, "CREATE TABLE r103_lock_target (id int PRIMARY KEY)"); err != nil {
		t.Fatalf("建被锁表失败：%v", err)
	}

	// 外部会话持 ACCESS EXCLUSIVE 锁 5s —— 超过运行期 lock_timeout(2s)。
	conn, err := pgx.Connect(ctx, dsn)
	if err != nil {
		t.Fatalf("外部连接失败：%v", err)
	}
	tx, err := conn.Begin(ctx)
	if err != nil {
		t.Fatalf("外部事务开始失败：%v", err)
	}
	if _, err := tx.Exec(ctx, "LOCK TABLE r103_lock_target IN ACCESS EXCLUSIVE MODE"); err != nil {
		t.Fatalf("持锁失败：%v", err)
	}
	released := make(chan struct{})
	go func() {
		defer close(released)
		time.Sleep(5 * time.Second)
		_ = tx.Rollback(context.Background())
	}()
	t.Cleanup(func() {
		<-released
		_ = conn.Close(context.Background())
	})

	start := time.Now()
	if err := db.runMigrations(ctx, lockMigrationFS(), upOnly); err != nil {
		t.Fatalf("R103：迁移不应被运行期 lock_timeout(2s) 砍断：%v", err)
	}
	if elapsed := time.Since(start); elapsed < 4*time.Second {
		t.Fatalf("迁移只用了 %v —— 外部锁没有生效，用例没测到等待", elapsed)
	}

	var lockTimeout, statementTimeout string
	if err := db.Pool.QueryRow(ctx,
		"SELECT lock_timeout, statement_timeout FROM r103_migration_settings").
		Scan(&lockTimeout, &statementTimeout); err != nil {
		t.Fatalf("读迁移会话设置失败：%v", err)
	}
	if lockTimeout != "0" || statementTimeout != "0" {
		t.Fatalf("迁移会话预算未关闭：lock_timeout=%q statement_timeout=%q",
			lockTimeout, statementTimeout)
	}

	// 迁移池关闭后，运行期池必须仍是原来的预算（迁移不得影响请求路径）。
	var runtimeLock, runtimeStatement string
	if err := db.Pool.QueryRow(ctx, "SHOW lock_timeout").Scan(&runtimeLock); err != nil {
		t.Fatalf("读运行期 lock_timeout 失败：%v", err)
	}
	if err := db.Pool.QueryRow(ctx, "SHOW statement_timeout").Scan(&runtimeStatement); err != nil {
		t.Fatalf("读运行期 statement_timeout 失败：%v", err)
	}
	if runtimeLock != "2s" || runtimeStatement != "30s" {
		t.Fatalf("运行期池预算被迁移影响：lock_timeout=%q statement_timeout=%q",
			runtimeLock, runtimeStatement)
	}
}
