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

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/seeder"
	"github.com/laiyipao/server/internal/store"
	"github.com/laiyipao/server/internal/testutil/scratchdb"
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

// scratchPrefix 是本包一次性库的命名前缀。
// 改动必须同步登记到 scratchdb.Prefixes（见 scratchdb 的文档注释）。
const scratchPrefix = "laiyipao_svc_test_"

// legacyDevDSN 是退化路径：共用库建不起来时的兜底。
//
// ⚠️ 它同时也是**本机开发用的库** —— 所以只该在「没有 PG 可用」时落到这里，
// 而不是像第 150 轮之前那样把开发库当成默认测试库。
const legacyDevDSN = "postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"

// packageDSN 是本轮跑测共用的「健康库」。
//
// 空串 = 共用库没建起来，各用例会退到 `TEST_DATABASE_URL` / `legacyDevDSN`。
var packageDSN string

// TestMain 在任何测试跑之前做两件事：
//
//  1. 清掉本包前缀下的残留库（否则孤儿库累积，「跑得越多越红」）
//  2. 建一个**每轮一次性**的共用库并迁移 + 种子，供 `openTestService` 使用
//
// 第 2 点的原因见 `openTestService` 的注释：一句话是「别再往开发库里塞测试数据」。
func TestMain(m *testing.M) {
	if n, err := scratchdb.SweepPrefix(scratchPrefix); err != nil {
		// 清理失败不阻断跑测，但必须**说出来** —— 否则本轮结束后
		// 「为什么下一次这么慢」又变成一个查不到原因的问题。
		fmt.Fprintf(os.Stderr, "[scratchdb] 清理残留库失败（不阻断）：%v\n", err)
	} else if n > 0 {
		fmt.Fprintf(os.Stderr, "[scratchdb] 已清理 %d 个残留测试库（前缀 %s）\n", n, scratchPrefix)
	}

	// 共用库用独立前缀：它必须能与「每个用例各建一个」的库被分别清掉，
	// 否则前缀重叠时，一次清理会把另一个正在用的库删掉。
	sharedPrefix := scratchPrefix + "shared"
	if n, err := scratchdb.SweepPrefix(sharedPrefix); err == nil && n > 0 {
		fmt.Fprintf(os.Stderr, "[scratchdb] 已清理 %d 个残留共用库（前缀 %s）\n", n, sharedPrefix)
	}

	dsn, cleanup, err := scratchdb.Open(sharedPrefix)
	if err != nil {
		fmt.Fprintf(os.Stderr,
			"[fixture] 共用库建不起来，openTestService 将退回 %s / %s：%v\n",
			os.Getenv("TEST_DATABASE_URL"), legacyDevDSN, err)
		os.Exit(m.Run())
	}

	if err := migrateAndSeed(dsn); err != nil {
		// 库能建但灌不进去 → 本包所有 DB 用例都没有意义。说清楚再退化。
		fmt.Fprintf(os.Stderr, "[fixture] 共用库迁移/种子失败，本包将退回退化路径：%v\n", err)
		cleanup()
		os.Exit(m.Run())
	}

	packageDSN = dsn
	fmt.Fprintf(os.Stderr, "[fixture] 本轮共用库已就绪：%s\n", dsn)
	// os.Exit 会跳过 defer，所以这里必须显式顺序执行。
	code := m.Run()
	cleanup()
	os.Exit(code)
}

// migrateAndSeed 把一张空库升到最新并灌上内容种子。
func migrateAndSeed(dsn string) error {
	ctx, cancel := context.WithTimeout(context.Background(), 180*time.Second)
	defer cancel()
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	cfg.DBMaxConns = 4
	cfg.DBMinConns = 1
	db, err := store.Open(ctx, cfg)
	if err != nil {
		return fmt.Errorf("打开共用库：%w", err)
	}
	defer db.Close()
	if err := db.Migrate(ctx); err != nil {
		return fmt.Errorf("迁移共用库：%w", err)
	}
	if _, err := seeder.Run(ctx, db.Pool); err != nil {
		return fmt.Errorf("种子共用库：%w", err)
	}
	return nil
}

// scratchDSN 建一次性库并返回其 DSN（第 150 轮收敛到 internal/testutil/scratchdb）。
//
// 本仓库原先在 5 个包里各抄了一份，抄出了三个真实故障：
// 库名每秒重复、DROP 超时留孤儿、建库失败静默跳过。详见该包的文档注释。
func scratchDSN(t *testing.T) string {
	t.Helper()
	return scratchdb.DSN(t, scratchPrefix)
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
