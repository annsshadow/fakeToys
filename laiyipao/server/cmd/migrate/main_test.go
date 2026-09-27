package main

// migrate 命令的集成测试：run() 在一次性 scratch 库上跑迁移升降级。
// main 本身只做 flag 解析与 os.Exit（进程级副作用），按豁免处理。

import (
	"context"
	"fmt"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/config"
)

// scratchDSN 建一次性库并返回其 DSN（与 store 包同思路，夹具不能跨包导入）。
func scratchDSN(t *testing.T) string {
	t.Helper()
	admin := "postgres://postgres@127.0.0.1:5432/postgres?sslmode=disable"
	if dsn := os.Getenv("TEST_DATABASE_URL"); dsn != "" {
		if i := strings.LastIndex(dsn, "/"); i >= 0 {
			if j := strings.Index(dsn[i+1:], "?"); j >= 0 {
				admin = dsn[:i+1] + "postgres" + dsn[i+1+j:]
			} else {
				admin = dsn[:i+1] + "postgres"
			}
		}
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	pool, err := pgxpool.New(ctx, admin)
	if err != nil {
		t.Skipf("连不上管理库，跳过：%v", err)
	}
	defer pool.Close()
	if err := pool.Ping(ctx); err != nil {
		t.Skipf("管理库不可达，跳过：%v", err)
	}

	name := fmt.Sprintf("laiyipao_mig_test_%d", time.Now().UnixNano()%1_000_000_000)
	if _, err := pool.Exec(ctx, `CREATE DATABASE `+name); err != nil {
		t.Skipf("无权创建测试库，跳过：%v", err)
	}
	t.Cleanup(func() {
		dropCtx, dropCancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer dropCancel()
		_, _ = pool.Exec(dropCtx,
			`SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1`, name)
		_, _ = pool.Exec(dropCtx, `DROP DATABASE IF EXISTS `+name)
	})

	if i := strings.LastIndex(admin, "/"); i >= 0 {
		if j := strings.Index(admin[i+1:], "?"); j >= 0 {
			return admin[:i+1] + name + admin[i+1+j:]
		}
		return admin[:i+1] + name
	}
	return admin
}

func testCfg(dsn string) config.Config {
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	cfg.DBMaxConns = 4
	cfg.DBMinConns = 1
	return cfg
}

// TestRunMigrateUpAndDown 升级到顶 → 回滚一步，全程 scratch 库。
// ⚠️ 绝不能对本机开发库跑 down —— 那会删掉最新迁移的表/列。
func TestRunMigrateUpAndDown(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()
	dsn := scratchDSN(t)

	if err := run(ctx, testCfg(dsn), false); err != nil {
		t.Fatalf("迁移升级失败：%v", err)
	}
	// 幂等：重复执行必须成功
	if err := run(ctx, testCfg(dsn), false); err != nil {
		t.Fatalf("重复迁移应幂等：%v", err)
	}
	// 回滚一步
	if err := run(ctx, testCfg(dsn), true); err != nil {
		t.Fatalf("回滚失败：%v", err)
	}
}

func TestRunBadDSN(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if err := run(ctx, testCfg("::bad dsn::"), false); err == nil {
		t.Fatal("非法 DSN 应报错")
	}
}
