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
	"github.com/laiyipao/server/internal/testutil/scratchdb"
)

// scratchPrefix 是本包一次性库的命名前缀。
// 改动必须同步登记到 scratchdb.Prefixes（见 scratchdb 的文档注释）。
const scratchPrefix = "laiyipao_seedcmd_test_"

// TestMain 跑测试前清掉本包前缀下的残留库（否则孤儿库累积，
// 症状是「跑得越多越红」，而那与真实缺陷无关）。
func TestMain(m *testing.M) {
	if n, err := scratchdb.SweepPrefix(scratchPrefix); err != nil {
		fmt.Fprintf(os.Stderr, "[scratchdb] 清理残留库失败（不阻断）：%v\n", err)
	} else if n > 0 {
		fmt.Fprintf(os.Stderr, "[scratchdb] 已清理 %d 个残留测试库（前缀 %s）\n", n, scratchPrefix)
	}
	os.Exit(m.Run())
}

// scratchDSN 建一次性库并返回其 DSN（第 150 轮收敛到 internal/testutil/scratchdb）。
func scratchDSN(t *testing.T) string {
	t.Helper()
	return scratchdb.DSN(t, scratchPrefix)
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
