// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package store

// R54：并发多实例同时跑 goose 的回归测试。
//
// 工业部署里 cmd/api 的启动序包含 Migrate，滚动发布时多个副本会同时启动。
// 没有会话锁时实测 8 个实例对同一个空库同时迁移：1 个成功、7 个失败
// （42P01 goose_db_version does not exist / 23505 重复键），失败副本直接 FATAL。
// 现在要求全部成功，且最终版本恰好是 migrations 目录里的最大编号。

import (
	"context"
	"fmt"
	"strconv"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/laiyipao/server/migrations"
)

// latestMigrationVersion 从嵌入的迁移文件里取最大版本号。
func latestMigrationVersion(t *testing.T) int64 {
	t.Helper()
	entries, err := migrations.FS.ReadDir(".")
	if err != nil {
		t.Fatalf("读迁移目录失败：%v", err)
	}
	var maxVer int64
	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".sql") {
			continue
		}
		prefix, _, ok := strings.Cut(name, "_")
		if !ok {
			t.Fatalf("迁移文件名不符合 NNNNN_name.sql：%s", name)
		}
		v, err := strconv.ParseInt(prefix, 10, 64)
		if err != nil {
			t.Fatalf("迁移文件名前缀不是数字：%s", name)
		}
		if v > maxVer {
			maxVer = v
		}
	}
	if maxVer == 0 {
		t.Fatal("没有找到任何迁移文件")
	}
	return maxVer
}

func TestConcurrentMigrateIsSerialized(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 90*time.Second)
	defer cancel()

	const n = 6
	errs := make([]error, n)
	start := make(chan struct{})
	var wg sync.WaitGroup
	for i := range n {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			db, err := Open(ctx, openCfg(dsn))
			if err != nil {
				errs[i] = fmt.Errorf("实例 %d 打开连接失败：%w", i, err)
				return
			}
			defer db.Close()
			<-start
			errs[i] = db.Migrate(ctx)
		}(i)
	}
	close(start)
	wg.Wait()

	for i, err := range errs {
		if err != nil {
			t.Errorf("并发迁移下实例 %d 失败（会话锁应让后来者等待后空转）：%v", i, err)
		}
	}

	db, err := Open(ctx, openCfg(dsn))
	if err != nil {
		t.Fatalf("复查连接失败：%v", err)
	}
	defer db.Close()

	var gotVer, applied int64
	if err := db.Pool.QueryRow(ctx, `SELECT MAX(version_id) FROM goose_db_version`).Scan(&gotVer); err != nil {
		t.Fatalf("读版本失败：%v", err)
	}
	if err := db.Pool.QueryRow(ctx, `SELECT COUNT(*) FROM goose_db_version WHERE is_applied`).Scan(&applied); err != nil {
		t.Fatalf("读版本行数失败：%v", err)
	}

	wantVer := latestMigrationVersion(t)
	if gotVer != wantVer {
		t.Errorf("最终版本 = %d，期望 %d", gotVer, wantVer)
	}
	// goose 会为版本 0 写一条基线行，之后每个版本恰好一行。
	if applied != wantVer+1 {
		t.Errorf("已应用版本行 = %d，期望 %d（版本 0 基线 + %d 个迁移）；出现重复行说明迁移被执行了多次",
			applied, wantVer+1, wantVer)
	}
}

// TestMigrateRejectsSingleConnectionPool：会话锁独占一个连接，池只有 1 个连接时
// 必须**立刻报错**，而不是卡到 context 超时（后者看起来只是「迁移很慢」）。
func TestMigrateRejectsSingleConnectionPool(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	cfg := openCfg(dsn)
	cfg.DBMaxConns = 1
	cfg.DBMinConns = 0
	db, err := Open(ctx, cfg)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	defer db.Close()

	if err := db.Migrate(ctx); err == nil {
		t.Fatal("DB_MAX_CONNS=1 应被拒绝（会话锁需要额外一个连接）")
	} else if !strings.Contains(err.Error(), "至少 2 个连接") {
		t.Errorf("错误应说明连接数要求：%v", err)
	}
}
