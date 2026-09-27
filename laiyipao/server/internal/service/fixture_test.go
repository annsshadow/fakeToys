package service

// 覆盖率攻坚用的两个夹具变体。
//
// openTestService（concurrency_test.go）给出"健康库"环境；
// 这里补两个确定性故障环境：
//
//   - openBrokenService：连接池已关闭。所有查询立刻失败，
//     用于覆盖各函数的"数据库故障"错误分支 —— 这些分支在健康库上
//     永远走不到，但恰恰是出错时唯一被用户看到的东西。
//   - openScratchService：一次性 scratch 库（建库→迁移→种子）。
//     用于"单表故障"场景：把某张表改名，让特定 SQL 必然失败，
//     覆盖"前一步成功、这一步失败"的中间分支（closed pool 只能覆盖第一步）。

import (
	"context"
	"fmt"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/seeder"
	"github.com/laiyipao/server/internal/store"
)

// openBrokenService 返回一个底层连接池已关闭的 Service。
func openBrokenService(t *testing.T) *Service {
	t.Helper()
	ts := openTestService(t) // 连不上库会 Skip
	ts.db.Close()            // pgxpool.Close 幂等，cleanup 里的二次 Close 无害
	return ts.Service
}

// openScratchService 返回跑在一次性库上的 Service（已迁移+种子）。
// 库在测试结束时整体 DROP，因此本文件里的"改名/删表"不需要恢复。
func openScratchService(t *testing.T) *testService {
	t.Helper()
	dsn := scratchDSN(t)
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()

	cfg := config.Load()
	cfg.DatabaseURL = dsn
	cfg.DBMaxConns = 4
	cfg.DBMinConns = 1

	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("scratch 库迁移失败：%v", err)
	}
	if _, err := seeder.Run(ctx, db.Pool); err != nil {
		t.Fatalf("scratch 库种子失败：%v", err)
	}
	return &testService{Service: New(db, cfg), db: db}
}

// scratchDSN 建一次性库并返回其 DSN（与 seeder/store 包同思路，测试夹具不能跨包）。
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

	// 借用一个临时池来 CREATE/DROP DATABASE
	pool, err := pgxpool.New(ctx, admin)
	if err != nil {
		t.Skipf("连不上管理库，跳过：%v", err)
	}
	defer pool.Close()
	if err := pool.Ping(ctx); err != nil {
		t.Skipf("管理库不可达，跳过：%v", err)
	}

	name := fmt.Sprintf("laiyipao_svc_test_%d", time.Now().UnixNano()%1_000_000_000)
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

// (ts) renameTable 临时把表改名。scratch 库专用 —— 不恢复，
// 但同一用例内继续跑的步骤必须知晓这一点。
func (ts *testService) renameTable(t *testing.T, from, to string) {
	t.Helper()
	if _, err := ts.pool.Exec(context.Background(),
		fmt.Sprintf(`ALTER TABLE %s RENAME TO %s`, from, to)); err != nil {
		t.Fatalf("改名 %s → %s 失败：%v", from, to, err)
	}
}

// (ts) exec 执行任意 SQL（数据准备）。
func (ts *testService) exec(t *testing.T, q string, args ...any) {
	t.Helper()
	if _, err := ts.pool.Exec(context.Background(), q, args...); err != nil {
		t.Fatalf("执行失败（%s）：%v", q, err)
	}
}

// mustErr 断言 err != nil 且错误信息包含 want 子串（want 为空则只要求非 nil）。
func mustErr(t *testing.T, err error, want string) {
	t.Helper()
	if err == nil {
		t.Fatalf("应返回错误（期望含 %q）却成功了", want)
	}
	if want != "" && !strings.Contains(err.Error(), want) {
		t.Fatalf("错误 %q 不包含期望片段 %q", err.Error(), want)
	}
}
