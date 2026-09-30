package main

// seed 命令的集成测试：run() 在一次性 scratch 库上完成迁移+种子。
// 幂等性的细粒度验证在 seeder 包；这里验证命令入口的组装正确。

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

// scratchDSN 建一次性库并返回其 DSN（与 migrate 包同思路，夹具不能跨包导入）。
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

	name := fmt.Sprintf("laiyipao_seedcmd_test_%d", time.Now().UnixNano()%1_000_000_000)
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

func TestRunSeedsScratchDB(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 180*time.Second)
	defer cancel()
	dsn := scratchDSN(t)

	res, err := run(ctx, testCfg(dsn))
	if err != nil {
		t.Fatalf("run 失败：%v", err)
	}
	// 关键内容必须落库 —— 运营执行 cmd/seed 后玩家才能开局
	if res.Enemies == 0 || res.Skills == 0 || res.Levels == 0 || res.ShopItems == 0 {
		t.Fatalf("种子计数异常：%+v", res)
	}
	// 幂等：重复执行必须成功且计数一致
	res2, err := run(ctx, testCfg(dsn))
	if err != nil {
		t.Fatalf("重复 run 应幂等：%v", err)
	}
	if res != res2 {
		t.Fatalf("两次种子结果不一致：\n%+v\n%+v", res, res2)
	}
}

// TestRunSeedBadDSN 覆盖 run 的"连接数据库失败"分支。
func TestRunSeedBadDSN(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if _, err := run(ctx, testCfg("::bad dsn::")); err == nil {
		t.Fatal("非法 DSN 应报错")
	} else if !strings.Contains(err.Error(), "连接数据库失败") {
		t.Errorf("错误应说明是连接失败：%v", err)
	}
}

// TestRunSeedMigrateFails 覆盖 run 的"迁移失败"分支（Open 成功、Migrate 失败）。
// 构造：scratch 库预建 00001 会 CREATE 的 users 表 → goose up 撞表报错。
func TestRunSeedMigrateFails(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()
	dsn := scratchDSN(t)

	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	if _, err := pool.Exec(ctx, `CREATE TABLE users (id int)`); err != nil {
		pool.Close()
		t.Skipf("预建冲突表失败：%v", err)
	}
	pool.Close()

	if _, err := run(ctx, testCfg(dsn)); err == nil {
		t.Fatal("已存在 users 表时迁移应失败")
	} else if !strings.Contains(err.Error(), "迁移失败") {
		t.Errorf("错误应说明是迁移失败：%v", err)
	}
}
