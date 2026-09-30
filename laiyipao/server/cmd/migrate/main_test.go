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

// TestRunMigrateUpFails 覆盖 run 的"迁移失败"分支（Open 成功、Migrate 失败）。
// 构造：scratch 库里预建一张 00001 会 CREATE 的 users 表 → goose up 撞表报错。
// 意义：迁移执行失败必须被包装成 "迁移失败" 上抛，而不是当成完成。
func TestRunMigrateUpFails(t *testing.T) {
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

	if err := run(ctx, testCfg(dsn), false); err == nil {
		t.Fatal("已存在 users 表时迁移应失败")
	} else if !strings.Contains(err.Error(), "迁移失败") {
		t.Errorf("错误应说明是迁移失败：%v", err)
	}
}

// TestRunMigrateDownFails 覆盖 run 的"回滚失败"分支（Open 成功、MigrateDown 失败）。
// 构造：迁移到顶后删掉 00009 down 要改的 redeem_codes 表 → 回滚 SQL 找不到表报错。
func TestRunMigrateDownFails(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()
	dsn := scratchDSN(t)

	if err := run(ctx, testCfg(dsn), false); err != nil {
		t.Fatalf("先迁移到顶应成功：%v", err)
	}
	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	// 删掉最新迁移（00009）Down SQL 依赖的表 → 回滚必失败
	if _, err := pool.Exec(ctx, `DROP TABLE redeem_codes CASCADE`); err != nil {
		pool.Close()
		t.Skipf("删表失败：%v", err)
	}
	pool.Close()

	// ⚠️ 必须**循环回滚直到失败**，不能只回滚一步。
	//
	// 原来只调一次 `run(..., down=true)`，那时 00009 恰好是最后一条迁移，
	// 所以那一步就会失败。
	// 之后有人加了 00010（Down 是空操作），第一步回滚成功、第二步才失败 ——
	// 断言立刻红了，而红的原因与「MigrateDown 是否暴露错误」毫无关系。
	//
	// 硬编码「第 N 步会失败」和硬编码「第 9 条迁移」是同一类错误：
	// 都在断言一个会随别人改动而变化的事实。
	//
	// 这里断言的是**真正要守的性质**：只要还有一步会失败，
	// 就不该出现「一路回滚全成功」。
	const maxSteps = 100 // 迁移数量级的上限，防止死循环
	sawError := false
	for i := 0; i < maxSteps; i++ {
		err := run(ctx, testCfg(dsn), true)
		if err == nil {
			continue // 这一步回滚成功，继续往下
		}
		if !strings.Contains(err.Error(), "回滚失败") {
			t.Errorf("第 %d 步回滚应说明是回滚失败，实际：%v", i+1, err)
		}
		sawError = true
		break
	}
	if !sawError {
		t.Fatal("依赖表缺失时回滚本应失败，却一路回滚成功了")
	}
}
