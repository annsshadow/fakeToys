package seeder

// seeder 的集成测试。
//
// Run 的核心承诺是**幂等**：cmd/seed 会被运营反复执行，任何一次
// "upsert 写成了无条件 INSERT" 都会在第二次执行时炸掉（主键冲突），
// 或者更糟 —— 把玩家数据一起重置。所以这里的判据是：
//  1. 跑两次必须都成功，且两次的各类计数完全一致；
//  2. 库里实际行数与 SeedResult 一致（没有重复行）；
//  3. 关键内容真实落库（玩家路径依赖它们存在）。
//
// 与 store 包同理：在一次性 scratch 库上跑，不碰开发库。

import (
	"context"
	"fmt"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/store"
)

// 测试库 DSN：TEST_DATABASE_URL 可覆盖，缺省连本机管理库。
func testAdminDSN() string {
	if dsn := os.Getenv("TEST_DATABASE_URL"); dsn != "" {
		// 把业务库名换成系统库 postgres，用于 CREATE DATABASE
		if i := strings.LastIndex(dsn, "/"); i >= 0 {
			if j := strings.Index(dsn[i:], "?"); j >= 0 {
				return dsn[:i+1] + "postgres" + dsn[i+j:]
			}
			return dsn[:i+1] + "postgres"
		}
	}
	return "postgres://postgres@127.0.0.1:5432/postgres?sslmode=disable"
}

// withDBName 把 DSN 的库名段替换为 name（解析路径段，避免误伤用户名）。
func withDBName(dsn, name string) string {
	if i := strings.LastIndex(dsn, "/"); i >= 0 {
		if j := strings.Index(dsn[i+1:], "?"); j >= 0 {
			return dsn[:i+1] + name + dsn[i+1+j:]
		}
		return dsn[:i+1] + name
	}
	return dsn
}

// scratchDSN 建一次性库并返回其 DSN。
// 与 store 包的 helper 逻辑相同但独立一份 —— 测试辅助不能跨包导入。
func scratchDSN(t *testing.T) string {
	t.Helper()
	admin := testAdminDSN()
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

	name := fmt.Sprintf("laiyipao_seed_test_%d", time.Now().UnixNano()%1_000_000_000)
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
	return withDBName(admin, name)
}

// openScratch 打开一个已完成迁移的 scratch 库，返回连接池与其 DSN。
func openScratch(t *testing.T) (*pgxpool.Pool, string) {
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
	return db.Pool, dsn
}

func TestRunIsIdempotentAndComplete(t *testing.T) {
	pool, _ := openScratch(t)
	ctx := context.Background()

	first, err := Run(ctx, pool)
	if err != nil {
		t.Fatalf("首次 Run 失败：%v", err)
	}
	second, err := Run(ctx, pool)
	if err != nil {
		t.Fatalf("第二次 Run 失败 —— 幂等性被破坏（运营重复执行 cmd/seed 会炸）：%v", err)
	}

	// 两次计数必须一致：upsert 语义下重复执行不新增行
	if first != second {
		t.Fatalf("两次 Run 结果不一致：\n第一次 %+v\n第二次 %+v", first, second)
	}

	// 计数必须与内容表对齐 —— 这里的期望值来自 domain 真相源，
	// 写死 13/42 之类的数字会随内容增长变成假红。
	checks := []struct {
		name string
		want int
	}{
		{"enemies", len(domain.SeedEnemies)},
		{"skills", len(domain.SeedSkills) + len(domain.SeedCompositeSkills)},
		{"skill_recipes", len(domain.SeedRecipes)},
		{"equipment", len(domain.SeedEquipmentList)},
		{"gems", len(domain.SeedGems)},
		{"skins", len(domain.SeedSkins)},
		{"levels", domain.TotalLevels},
	}
	for _, c := range checks {
		var n int
		if err := pool.QueryRow(ctx, `SELECT COUNT(*) FROM `+c.name).Scan(&n); err != nil {
			t.Fatalf("数 %s 失败：%v", c.name, err)
		}
		if n != c.want {
			t.Errorf("%s 应有 %d 行（与 domain 内容表一致），实际 %d", c.name, c.want, n)
		}
	}

	// 其余计数只断言"非零"：种子内部自建的清单（签到/任务/商城/专精）
	// 没有导出为 domain 常量，断言具体数字会变成必须跟着内容改的脆测试。
	for name, got := range map[string]int{
		"sign_in_calendar": first.SignInDays,
		"tasks":            first.Tasks,
		"shop_items":       first.ShopItems,
		"mastery_nodes":    first.Mastery,
		"level_waves":      first.Waves,
	} {
		var n int
		if err := pool.QueryRow(ctx, `SELECT COUNT(*) FROM `+name).Scan(&n); err != nil {
			t.Fatalf("数 %s 失败：%v", name, err)
		}
		if n == 0 || got == 0 {
			t.Errorf("%s 落库 %d 行、SeedResult 记 %d —— 有一边是空的", name, n, got)
		}
		if n != got {
			t.Errorf("%s 落库 %d 行与 SeedResult %d 不一致", name, n, got)
		}
	}

	// 兑换码路径不返回计数，但必须落库且未被用过
	var used int
	if err := pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM redeem_codes WHERE used_count <> 0`).Scan(&used); err != nil {
		t.Fatalf("查兑换码失败：%v", err)
	}
	if used != 0 {
		t.Errorf("种子兑换码不应带使用记录，实际 %d 条", used)
	}
}

// TestRunWavesMatchGeneratedLevel 锁一个历史踩过的坑：
// level_waves 必须与 domain 生成器输出逐关对齐。
// 少写一波，客户端拿 /levels/:id 的 wave_count 建局，打到一半没有刷怪数据。
func TestRunWavesMatchGeneratedLevel(t *testing.T) {
	pool, _ := openScratch(t)
	ctx := context.Background()
	if _, err := Run(ctx, pool); err != nil {
		t.Fatalf("Run 失败：%v", err)
	}

	for _, id := range []int{1, 25, 50, 100} {
		var want, got int
		if err := pool.QueryRow(ctx,
			`SELECT wave_count FROM levels WHERE id = $1`, id).Scan(&want); err != nil {
			t.Fatalf("读关卡 %d 失败：%v", id, err)
		}
		if err := pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM level_waves WHERE level_id = $1`, id).Scan(&got); err != nil {
			t.Fatalf("读波次失败：%v", err)
		}
		if want != got {
			t.Errorf("关卡 %d：levels.wave_count=%d 但 level_waves=%d 行", id, want, got)
		}
	}
}

// TestRunFailsLoudWhenTargetMissing 覆盖每个 seed* 的写库失败分支。
//
// 手法：把目标表临时改名，让对应的 INSERT 必然失败，断言 Run 把错误
// 带出来（而不是静默吞掉继续跑）。**改回**后下一轮不受影响。
//
// 为什么值得逐一覆盖：Run 在一个事务里串 10 个 seeder，若某个分支把
// 错误吞了（或 return res 漏了 err），部分写入的库会被当成"种子完成"。
// 这些分支平时永远不会走 —— 正因为如此，出问题时没人见过它的样子。
func TestRunFailsLoudWhenTargetMissing(t *testing.T) {
	pool, _ := openScratch(t)
	ctx := context.Background()

	// 每个表名对应 Run 里第一个写它的 seeder。
	// enemy_element_resist 单列：同一 seeder 里敌人行写成功后抗性行写失败的分支。
	// level_waves 单列：关卡行写成功后波次行写失败的分支。
	targets := []string{
		"enemies", "enemy_element_resist", "skills", "skill_recipes",
		"equipment", "gems", "skins", "mastery_trees", "mastery_nodes",
		"levels", "level_waves", "sign_in_calendar", "tasks",
		"shop_items", "redeem_codes",
	}
	for _, tbl := range targets {
		t.Run(tbl, func(t *testing.T) {
			if _, err := pool.Exec(ctx, fmt.Sprintf(`ALTER TABLE %s RENAME TO %s_bak`, tbl, tbl)); err != nil {
				t.Fatalf("改名 %s 失败：%v", tbl, err)
			}
			_, runErr := Run(ctx, pool)
			// 无论断言成败都必须把表改回来，否则污染后续轮次
			if _, err := pool.Exec(ctx, fmt.Sprintf(`ALTER TABLE %s_bak RENAME TO %s`, tbl, tbl)); err != nil {
				t.Fatalf("改回 %s 失败：%v", tbl, err)
			}
			if runErr == nil {
				t.Errorf("表 %s 缺失时 Run 应报错，实际静默成功 —— 有分支吞了错误", tbl)
			}
		})
	}

	// 收尾验证：全部改回后 Run 必须恢复幂等成功，
	// 证明"改名-改回"没有在库里留下半残状态。
	if _, err := Run(ctx, pool); err != nil {
		t.Fatalf("恢复后 Run 应成功：%v", err)
	}
}

// TestRunFailsWhenPoolClosed 覆盖 Run 的事务开启失败分支。
func TestRunFailsWhenPoolClosed(t *testing.T) {
	pool, dsn := openScratch(t)

	// 独立开一个池再关掉：不影响主池
	closed, err := pgxpool.New(context.Background(), dsn)
	if err != nil {
		t.Skipf("开池失败，跳过：%v", err)
	}
	closed.Close()

	if _, err := Run(context.Background(), closed); err == nil {
		t.Error("连接池已关闭时 Run 应报 begin 错误")
	}
	_ = pool
}
