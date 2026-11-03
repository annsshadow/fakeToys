// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package store

// 00026 的回滚验证：**必须有数据**，不能只在空库上升降。
//
// 为什么：
//
//  1. 00025/00026 建的是触发器函数与增量表。空库上没有行，
//     `UPDATE ... FROM battle_records` 的回填、`ON CONFLICT` 的累加、
//     以及 UPDATE 分支里「只改 wave_reached」的判据都不会执行 ——
//     空库跑通只能证明 SQL 语法对，证明不了**有数据时的语义**。
//  2. 真正的回滚场景恰恰是线上：库里已有 100 万战报，升级后发现问题要退。
//
// 判据：
//
//   - 降级后 `wave_sum` 列消失、`dashboard_battle_stats` 行与战报行数不变；
//   - 降级期间写入的战报，旧触发器仍能正确累加 attempts/wins/duration_sum；
//   - 重新升级后 `wave_sum` 按 battle_records 回填，含降级期间新增的那一行。
//
// ⚠️ 回滚顺序在真机上还有一条：新代码读 `wave_sum`，所以必须先回滚二进制
// 再回滚迁移。本用例只证明迁移本身可逆。

import (
	"context"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/stdlib"
	"github.com/laiyipao/server/migrations"
	"github.com/pressly/goose/v3"
)

// downTo 把库回滚到指定迁移版本（测试用，scratch 库独占，无需会话锁）。
func downTo(t *testing.T, d *DB, ctx context.Context, version int64) {
	t.Helper()
	sqlDB := stdlib.OpenDBFromPool(d.Pool)
	defer sqlDB.Close()
	p, err := goose.NewProvider(goose.DialectPostgres, sqlDB, migrations.FS)
	if err != nil {
		t.Fatalf("goose provider: %v", err)
	}
	if _, err := p.DownTo(ctx, version); err != nil {
		t.Fatalf("回滚到 %d 失败：%v", version, err)
	}
}

func TestDashboardWaveSumMigrationRollbackOnData(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 90*time.Second)
	defer cancel()

	db, err := Open(ctx, openCfg(dsn))
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("Migrate 失败：%v", err)
	}

	var uid int64
	if err := db.Pool.QueryRow(ctx, `
		INSERT INTO users (guest_token, nickname, is_guest, status, created_at, last_login_at)
		VALUES ('r64_migration', 'R64', true, 1, now(), now())
		RETURNING id`).Scan(&uid); err != nil {
		t.Fatalf("建用户失败：%v", err)
	}

	insertBattle := func(level int, result string, dur, wave int) {
		t.Helper()
		if _, err := db.Pool.Exec(ctx, `
			INSERT INTO battle_records (user_id, level_id, result, duration_ms, wave_reached)
			VALUES ($1, $2, $3, $4, $5)`, uid, level, result, dur, wave); err != nil {
			t.Fatalf("建战报失败：%v", err)
		}
	}
	insertBattle(1, "win", 1000, 10)
	insertBattle(1, "lose", 2000, 3)
	insertBattle(2, "win", 900, 5)

	// 升级态：wave_sum 已回填（10+3=13 for level 1；level 2 = 5）。
	type stats struct {
		attempts, wins, duration, wave int64
	}
	readStats := func(level int) stats {
		t.Helper()
		var s stats
		if err := db.Pool.QueryRow(ctx, `
			SELECT attempts, wins, duration_sum, wave_sum
			  FROM dashboard_battle_stats WHERE level_id = $1`, level).
			Scan(&s.attempts, &s.wins, &s.duration, &s.wave); err != nil {
			t.Fatalf("读增量表失败：%v", err)
		}
		return s
	}
	if got := readStats(1); got.attempts != 2 || got.wins != 1 || got.duration != 3000 || got.wave != 13 {
		t.Fatalf("升级后 level 1 增量表 = %+v，期望 {2 1 3000 13}", got)
	}

	// 回滚 00026（最新版本 26 → 25）。
	downTo(t, db, ctx, 25)

	// 列必须消失，战报行必须还在。
	var n int
	if err := db.Pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM information_schema.columns
		 WHERE table_name = 'dashboard_battle_stats' AND column_name = 'wave_sum'`).Scan(&n); err != nil {
		t.Fatalf("查列失败：%v", err)
	}
	if n != 0 {
		t.Fatal("回滚后 wave_sum 列仍存在")
	}
	if err := db.Pool.QueryRow(ctx, `SELECT COUNT(*) FROM battle_records`).Scan(&n); err != nil {
		t.Fatalf("查战报失败：%v", err)
	}
	if n != 3 {
		t.Fatalf("回滚后战报行数 = %d，期望 3（回滚不能动业务数据）", n)
	}
	// 只允许 4 参数版本，5 参数版本必须被删掉（否则同名重载会混淆调用）。
	var fns int
	var applyArgs string
	if err := db.Pool.QueryRow(ctx, `
		SELECT COUNT(*), COALESCE(MAX(pg_get_function_identity_arguments(p.oid)), '')
		  FROM pg_proc p WHERE p.proname = 'dashboard_battle_stats_apply'`).
		Scan(&fns, &applyArgs); err != nil {
		t.Fatalf("查函数失败：%v", err)
	}
	if fns != 1 {
		t.Fatalf("回滚后 dashboard_battle_stats_apply 应恰好 1 个，实际 %d 个重载", fns)
	}
	if strings.Contains(applyArgs, "p_wave") {
		t.Fatalf("回滚后仍留着收 wave 的 5 参数版本：%s", applyArgs)
	}

	// 降级期间写入仍被旧触发器累加（这正是「回滚后系统还能跑」的判据）。
	insertBattle(1, "win", 1234, 5)
	var attempts, wins, duration int64
	if err := db.Pool.QueryRow(ctx, `
		SELECT attempts, wins, duration_sum FROM dashboard_battle_stats WHERE level_id = 1`).
		Scan(&attempts, &wins, &duration); err != nil {
		t.Fatalf("读降级后增量表失败：%v", err)
	}
	if attempts != 3 || wins != 2 || duration != 4234 {
		t.Fatalf("降级后 level 1 增量表 = {%d %d %d}，期望 {3 2 4234}", attempts, wins, duration)
	}

	// 再升级：wave_sum 必须按 battle_records 重新回填，包含降级期间那一行。
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("重新 Migrate 失败：%v", err)
	}
	if got := readStats(1); got.attempts != 3 || got.wins != 2 || got.duration != 4234 || got.wave != 18 {
		t.Fatalf("回滚再升级后 level 1 = %+v，期望 {3 2 4234 18}", got)
	}
}
