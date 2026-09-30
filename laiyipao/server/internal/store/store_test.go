package store

// store 包的集成测试。
//
// 为什么打真库而不是 mock：Migrate 的全部价值就在于"goose 能吃下
// migrations.FS 并把库升到最新"，mock 掉 goose 后测试只剩空转。
//
// ⚠️ 关键决策：**不用本机 laiyipao 库做 MigrateDown**。
// MigrateDown 会回滚最新一个迁移（删表/删列），打到开发库上等于毁数据。
// 所以这里自建一次性 scratch 库：CREATE DATABASE → 迁移到顶 → 回滚一步
// → DROP DATABASE，全程与开发库隔离。
// 连不上 PostgreSQL 时 Skip（与 service 包 openTestService 同一约定）。

import (
	"context"
	"fmt"
	"net/url"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/laiyipao/server/migrations"

	"github.com/laiyipao/server/internal/config"
)

// adminDSN 返回连到 postgres 系统库的 DSN（用于 CREATE/DROP DATABASE）。
func adminDSN() string {
	if dsn := os.Getenv("TEST_DATABASE_URL"); dsn != "" {
		return strings.Replace(dsn, "/laiyipao", "/postgres", 1)
	}
	return "postgres://postgres@127.0.0.1:5432/postgres?sslmode=disable"
}

// withDBName 把 DSN 的库名替换为 name。
// 必须走 url 解析而不是字符串替换："/postgres" 在 `postgres://postgres@…`
// 里出现两次，盲替换会改掉用户名段。
func withDBName(dsn, name string) string {
	u, err := url.Parse(dsn)
	if err != nil {
		return dsn
	}
	u.Path = "/" + name
	return u.String()
}

// scratchDB 建一个一次性数据库并返回它的 DSN。
// 名字带纳秒时间戳，避免并行会话/上次残留撞名。
func scratchDB(t *testing.T) string {
	t.Helper()
	admin := adminDSN()
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

	name := fmt.Sprintf("laiyipao_store_test_%d", time.Now().UnixNano()%1_000_000_000)
	if _, err := pool.Exec(ctx, `CREATE DATABASE `+name); err != nil {
		t.Skipf("无权创建测试库，跳过：%v", err)
	}
	t.Cleanup(func() {
		dropCtx, dropCancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer dropCancel()
		// DROP 需要独占：先掐断可能残留的连接
		_, _ = pool.Exec(dropCtx,
			`SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1`, name)
		_, _ = pool.Exec(dropCtx, `DROP DATABASE IF EXISTS `+name)
	})
	return withDBName(admin, name)
}

// openCfg 组装指向指定 DSN 的配置。
func openCfg(dsn string) config.Config {
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	cfg.DBMaxConns = 4
	cfg.DBMinConns = 1
	return cfg
}

func TestOpenRejectsBadDSN(t *testing.T) {
	// 解析失败的 DSN 必须在 Open 内部报错，而不是把垃圾配置带进连接池
	cfg := openCfg("::not a dsn::")
	_, err := Open(context.Background(), cfg)
	if err == nil {
		t.Fatal("非法 DSN 应返回错误")
	}
	if !strings.Contains(err.Error(), "parse DATABASE_URL") {
		t.Errorf("错误应说明是 DSN 解析失败：%v", err)
	}
}

func TestOpenRejectsUnreachableDB(t *testing.T) {
	// 端口 1 几乎必然拒绝连接：验证"探测失败就关门"，不泄漏连接池
	cfg := openCfg("postgres://postgres@127.0.0.1:1/none?sslmode=disable&connect_timeout=2")
	db, err := Open(context.Background(), cfg)
	if err == nil {
		db.Close()
		t.Fatal("不可达数据库应返回错误")
	}
	if !strings.Contains(err.Error(), "ping database") {
		t.Errorf("错误应说明是连通性探测失败：%v", err)
	}
}

// TestOpenRejectsBadPoolConfig 构造 pgxpool.NewWithConfig 的失败分支：
// DSN 本身合法（ParseConfig 通过），但 Open 随后把 MaxConns 覆写成 0，
// pgxpool 在建池时校验 "MaxConns 必须 > 0" 直接报错。
// 这条分支此前无人覆盖，且无需真实数据库 —— 校验发生在建立连接之前。
// 意义：配置装配的错误必须在 Open 里被"create pool"包装并上抛，
// 而不是带着非法池配置继续启动服务。
func TestOpenRejectsBadPoolConfig(t *testing.T) {
	cfg := openCfg("postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable")
	cfg.DBMaxConns = 0 // 合法 DSN + 非法池参数
	db, err := Open(context.Background(), cfg)
	if err == nil {
		db.Close()
		t.Fatal("MaxConns=0 应让建池失败")
	}
	if !strings.Contains(err.Error(), "create pool") {
		t.Errorf("错误应说明是建池失败（而非 DSN 解析或连通性）：%v", err)
	}
}

// TestMigrateUpAndDownOnScratchDB 是本文件的核心：
// 迁移到顶 → 校验业务表存在且 goose 记录到最新 → 回滚一步 → 校验回滚生效。
// 全程在 scratch 库上，绝不碰开发库。
func TestMigrateUpAndDownOnScratchDB(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	db, err := Open(ctx, openCfg(dsn))
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)

	// Ping 是 /healthz 的依赖，顺带在此验证
	if err := db.Ping(ctx); err != nil {
		t.Fatalf("Ping 失败：%v", err)
	}

	// 迁移到顶
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("Migrate 失败：%v", err)
	}
	// 幂等：再跑一次必须无错（服务每次启动都会跑 Migrate）
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("Migrate 重复执行应幂等：%v", err)
	}

	// 核心业务表必须存在（这里挑两张分属不同迁移文件的关键表）
	for _, table := range []string{"users", "user_wallets", "levels", "shop_items", "defenses"} {
		var n int
		if err := db.Pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM information_schema.tables WHERE table_name = $1`, table).
			Scan(&n); err != nil || n != 1 {
			t.Errorf("迁移后应存在表 %s（count=%d err=%v）", table, n, err)
		}
	}
	// goose 版本记录应停在最新（00009_redeem_code_identity → 9）
	var version int64
	if err := db.Pool.QueryRow(ctx, `SELECT MAX(version_id) FROM goose_db_version`).Scan(&version); err != nil {
		t.Fatalf("读 goose 版本失败：%v", err)
	}
	// ⚠️ 这里**不能**硬编码版本号。
	//
	// 原来写的是 `version != 9` —— 于是每加一条迁移，这个测试就红一次，
	// 而红的原因与「迁移是否可用」毫无关系。
	// 硬编码版本号的测试守的是「迁移文件恰好有 9 个」，
	// 不是「迁移能把库升到最新」。
	//
	// 改成从**嵌入的迁移集**数出来：真正的语义是
	// 「goose 记录的版本 = 迁移文件的数量」。
	if want := migrationCount(t); version != int64(want) {
		t.Errorf("迁移后 goose 版本应等于迁移文件数 %d，实际 %d", want, version)
	}

	// 回滚一步：版本必须**恰好减一**。
	//
	// ⚠️ 原来这里断言的是「00009 的 down 把 redeem_codes.id 的 IDENTITY 去掉」。
	// 那条断言只在「00009 恰好是最后一条迁移」时成立 ——
	// 我加了 00010 之后它就在测错东西了（回滚的是 00010，不是 00009）。
	//
	// 换成与编号无关的性质：回滚一步 = 版本 -1。
	// 具体某条迁移的 down 效果，由那条迁移自己的测试负责。
	if err := db.MigrateDown(ctx); err != nil {
		t.Fatalf("MigrateDown 失败：%v", err)
	}
	var after int64
	if err := db.Pool.QueryRow(ctx, `SELECT MAX(version_id) FROM goose_db_version`).
		Scan(&after); err != nil {
		t.Fatalf("读回滚后 goose 版本失败：%v", err)
	}
	if after != version-1 {
		t.Errorf("回滚一步后版本应从 %d 变成 %d", version, after)
	}
}

// TestMigrateFailsOnCancelledContext 构造 goose.UpContext 的失败路径：
// 传入已取消的 context，迁移必须把错误包成 "goose up: ..." 向上返回，
// 而不是静默当作"迁移完成"。这是 store.go 里唯一可构造的迁移失败分支
// （SetDialect("postgres") 是硬编码合法方言，永不可达）。
func TestMigrateFailsOnCancelledContext(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	db, err := Open(ctx, openCfg(dsn))
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)

	cancelled, cancelFn := context.WithCancel(context.Background())
	cancelFn() // 先取消再调用
	err = db.Migrate(cancelled)
	if err == nil {
		t.Fatal("已取消的 context 必须让 Migrate 失败")
	}
	if !strings.Contains(err.Error(), "goose up") {
		t.Errorf("错误应说明是迁移执行失败：%v", err)
	}
}

// TestTxCommitAndRollback 锁住 Tx 的两条语义：
// fn 返回 nil → 提交；fn 返回 error → 整笔回滚。
// 用 announcements 表（迁移自带、无业务含义）做写入探针。
func TestTxCommitAndRollback(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	db, err := Open(ctx, openCfg(dsn))
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("Migrate 失败：%v", err)
	}

	count := func() int {
		t.Helper()
		var n int
		if err := db.Pool.QueryRow(ctx, `SELECT COUNT(*) FROM announcements`).Scan(&n); err != nil {
			t.Fatalf("计数失败：%v", err)
		}
		return n
	}

	// 提交路径
	if err := db.Tx(ctx, func(tx pgx.Tx) error {
		_, err := tx.Exec(ctx, `INSERT INTO announcements (title, body, published) VALUES ('t1','b1',false)`)
		return err
	}); err != nil {
		t.Fatalf("提交事务失败：%v", err)
	}
	if count() != 1 {
		t.Fatalf("fn 返回 nil 后应已提交，实际 %d 行", count())
	}

	// 回滚路径
	if err := db.Tx(ctx, func(tx pgx.Tx) error {
		if _, err := tx.Exec(ctx, `INSERT INTO announcements (title, body, published) VALUES ('t2','b2',false)`); err != nil {
			return err
		}
		return fmt.Errorf("故意失败")
	}); err == nil {
		t.Fatal("fn 返回 error 时 Tx 应向上传播错误")
	}
	if n := count(); n != 1 {
		t.Fatalf("fn 失败后必须回滚，实际 %d 行（期望仍为 1）", n)
	}
}

// TestCloseNilSafety Close 必须容忍零值与 nil 接收者 ——
// run() 的 defer db.Close() 在 Open 失败路径上会拿到半初始化对象。
func TestCloseNilSafety(t *testing.T) {
	var nilDB *DB
	nilDB.Close() // 不应 panic
	(&DB{}).Close()
}

// migrationCount 数嵌入的迁移文件数量。
//
// 它的存在是为了让「goose 版本」断言不依赖硬编码编号 ——
// 每加一条迁移就要改一次测试的那种断言守不到任何东西。
func migrationCount(t *testing.T) int {
	t.Helper()
	entries, err := migrations.FS.ReadDir(".")
	if err != nil {
		t.Fatalf("读迁移目录失败：%v", err)
	}
	n := 0
	for _, e := range entries {
		if !e.IsDir() && strings.HasSuffix(e.Name(), ".sql") {
			n++
		}
	}
	if n == 0 {
		t.Fatal("迁移文件数为 0 —— 嵌入是否失效？")
	}
	return n
}
