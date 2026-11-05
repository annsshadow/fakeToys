// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// Package store 负责 pgx 连接池与数据库迁移。
package store

import (
	"context"
	"fmt"
	"io/fs"
	"strconv"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/jackc/pgx/v5/stdlib"
	"github.com/pressly/goose/v3"
	"github.com/pressly/goose/v3/lock"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/migrations"
)

// DB 包装 pgx 连接池，向上层暴露统一的 Query/Exec/事务接口。
//
// Pool 是 *store.Pool（R90 起）：方法签名与 *pgxpool.Pool 一致，但
// Acquire 等待有 DB_ACQUIRE_TIMEOUT 上限。需要原始池的启动期一次性
// 使用者（seeder / stdlib 迁移）走 Pool.Raw()。
type DB struct {
	Pool *Pool
	// cfg 保留构造期配置：迁移要建**独立连接池**（R103），需要 DSN /
	// 时区 / 迁移预算，不能从已建好的运行期池里反推。
	cfg config.Config
}

// defaultDBTimeZone 是会话时区的兜底值。
//
// ⚠️ 选 Asia/Shanghai 而不是 UTC：业务是中文小程序，
// 「今天」应当等于**玩家所在时区的自然日**。
// 用 UTC 会让每日任务/签到在北京时间 08:00 翻页。
//
// 想改就在环境变量 `DB_TIMEZONE` 上改，不必改代码。
//
// R92：默认值直接引用 config.BusinessTimeZone —— 「业务日界」只有一个来源。
const defaultDBTimeZone = config.BusinessTimeZone

// Open 建立连接池并做一次连通性探测。探测失败直接返回错误，
// 不做"先启动后重试"——后端起不来时必须立刻让人知道原因。
func Open(ctx context.Context, cfg config.Config) (*DB, error) {
	poolCfg, err := pgxpool.ParseConfig(cfg.DatabaseURL)
	if err != nil {
		return nil, fmt.Errorf("parse DATABASE_URL: %w", err)
	}
	poolCfg.MaxConns = cfg.DBMaxConns
	poolCfg.MinConns = cfg.DBMinConns
	poolCfg.MaxConnLifetime = cfg.DBConnLifetime
	// 空闲连接回收：pgx 默认 30m，显式接配置是为了让「托管 PG 主动断开
	// 空闲连接」的环境能把回收时间调到断连阈值以内，避免反复用坏连接。
	if cfg.DBConnIdleTime > 0 {
		poolCfg.MaxConnIdleTime = cfg.DBConnIdleTime
	}

	// ⚠️ 第 90 轮：把会话时区**钉死**在连接上。
	//
	// # 为什么必须钉
	//
	// 不钉的话，会话时区由**数据库服务器的配置**决定：
	// postgresql.conf 的 TimeZone，或托管实例的默认（常见是 UTC）。
	// 而这个值会静默影响**所有**依赖日期折算的 SQL：
	//
	//	`sign_date DATE` 的 timestamptz -> date
	//	`to_char(created_at, 'YYYY-MM-DD')`
	//	任何 `::date` 转换
	//
	// 换句话说「今天几号」这个业务概念由**部署配置**决定，
	// 而部署配置不在代码里、也不在测试里。
	//
	// 第 89 轮已经在签到上撞到过一次：Go 用 `periodStart`（本地时区），
	// PG 用会话时区，两者各算一半。那里改成传日期字面量绕开了，
	// 但**其余任何**新增的日期折算还会再踩一次。
	//
	// 钉死之后，行为不再依赖服务器配置 ——
	// 「在开发机上跑通」与「在托管实例上跑通」是同一件事。
	//
	// ⚠️ 用 RuntimeParams 而不是 AfterConnect + SET：
	// 后者在**每条连接**上多一次往返，且忘记设就静默失效；
	// RuntimeParams 随连接握手一次性发出，不存在「漏掉某条连接」的可能。
	//
	// ⚠️ 空值不能写进去：PG 收到 `TimeZone=''` 会用服务器默认，
	// 那等于没钉。所以空值直接退回配置里的默认值。
	if poolCfg.ConnConfig.RuntimeParams == nil {
		poolCfg.ConnConfig.RuntimeParams = map[string]string{}
	}
	tz := cfg.DBTimeZone
	if tz == "" {
		tz = defaultDBTimeZone
	}
	poolCfg.ConnConfig.RuntimeParams["TimeZone"] = tz

	// ⚠️ 单条 SQL 的服务端执行上限（第 R3 轮）。
	//
	// 没有它时，一条写坏的查询（漏了 WHERE、笛卡尔积、锁等待）会
	// **无限期**占着一条连接：池只有 MaxConns 条，几条这样的查询就能把
	// API 拖到全员超时 —— 而且从应用侧看起来只是「变慢」，没有报错。
	// 设上界后，超时由 PG 主动取消（57014），连接立刻归还，
	// 错误也能进日志、被 fail-loud 上抛。
	//
	// ⚠️ 迁移**不再**复用这个池（R103）：`Migrate` 建一个独立的迁移连接池，
	// 显式 `statement_timeout=0` / `lock_timeout=0`，迁移结束即关闭。
	// 所以「大回填要设 DB_STATEMENT_TIMEOUT=0」这条旧建议已经失效 ——
	// 这两个变量只作用于运行期池；迁移的上限改由 `MIGRATE_TIMEOUT` 单独给。
	// 用 RuntimeParams 的理由与 TimeZone 相同：随握手一次性下发，
	// 不存在「某条连接漏设」的可能，也不多一次往返。
	if cfg.DBStatementTimeout > 0 {
		ms := cfg.DBStatementTimeout.Milliseconds()
		if ms < 1 {
			// sub-ms 的配置若原样下发会变成 0，而 0 在 PG 里表示**关闭** ——
			// 「设了很短的超时」反而变成「完全不超时」。
			ms = 1
		}
		poolCfg.ConnConfig.RuntimeParams["statement_timeout"] = strconv.FormatInt(ms, 10)
	}

	// ⚠️ 锁等待预算（R94）：与语句执行预算**分开**。
	//
	// statement_timeout 管「整条语句」，默认 30s 是为了容下看板聚合与大导出；
	// 用它管锁等待，故障期每个请求要挂 30s 才终局 —— 远超客户端 10s 超时。
	//
	// R89 F2 实测就是这个形态：锁 users 15s，241 个请求挂起 p95=15032ms，
	// 全部等到锁释放（15s < 30s statement_timeout，根本没触发取消）。
	// 终局到来时客户端早已断开，503 无人接收。
	//
	// lock_timeout 只计量「等锁」这段：正常短事务持锁毫秒级，
	// 2s 预算 ~3 个数量级余量，只有真故障（长事务/死锁边缘/DDL）才触达。
	// 超时错误码同样是 57014，R67 的 IsTransientDependency 已映射 503，
	// 错误路径零改动。
	//
	// ⚠️ 迁移不经过这个预算（R103）：`Migrate` 的独立迁移池显式
	// `lock_timeout=0`；迁移外的请求仍受这里约束。
	if cfg.DBLockTimeout > 0 {
		ms := cfg.DBLockTimeout.Milliseconds()
		if ms < 1 {
			ms = 1
		}
		poolCfg.ConnConfig.RuntimeParams["lock_timeout"] = strconv.FormatInt(ms, 10)
	}

	pool, err := pgxpool.NewWithConfig(ctx, poolCfg)
	if err != nil {
		return nil, fmt.Errorf("create pool: %w", err)
	}

	pingCtx, cancel := context.WithTimeout(ctx, 5*time.Second)
	defer cancel()
	if err := pool.Ping(pingCtx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("ping database: %w", err)
	}
	return &DB{Pool: WrapPool(pool, cfg.DBAcquireTimeout), cfg: cfg}, nil
}

// Close 释放连接池。
func (d *DB) Close() {
	if d != nil && d.Pool != nil {
		d.Pool.Close()
	}
}

// Migrate 执行 migrations 包内嵌的 goose 迁移。
//
// 三点说明：
//  1. goose 的 Provider API 只接受 *sql.DB，因此用 pgx 官方的 stdlib 适配层
//     把连接池包一层；连接池本身不受影响，stdlib 复用同一批底层连接。
//  2. migrations.FS 用 //go:embed *.sql 嵌入，文件位于 FS 根目录，
//     因此 goose 的目录参数是 "." 而不是 "migrations"。
//  3. **必须使用会话级 advisory lock**（R54）。
//
// # 为什么并发迁移必须上锁
//
// 工业部署里多个实例会同时启动，而 `cmd/api` 的启动序就包含 `db.Migrate`。
// 没有锁时每个实例都会读到同一个「当前版本」，然后各自跑同一批迁移。
// 实测 8 个实例对同一个空库同时迁移：**1 个成功、7 个失败**，错误为
// `relation "goose_db_version" does not exist`（42P01）与
// `duplicate key ... pg_typename_nsp_index`（23505）—— 失败实例在 main
// 里直接 FATAL 退出，滚动发布时表现为「只有一个副本能起来」。
//
// goose 的 `WithSessionLocker` 用 `pg_try_advisory_lock` 串行化整个迁移过程：
// 抢到锁的实例迁移，其余实例在锁上等待，拿到锁后读到已是最新版本、直接空转。
//
// ⚠️ 会话锁会**独占一个连接**直到迁移结束，所以迁移至少需要 2 个连接
// （锁 1 个 + goose 查询 1 个）。`DB_MAX_CONNS=1` 会在这里报错而不是
// 卡到 context 超时 —— 后者看起来只是「迁移很慢」，很难排查。
//
// # R103：迁移的超时与连接预算必须独立
//
// 修前直接 `provider.Up(启动 ctx)`，且迁移复用运行期池 —— 两个硬上限
// 都落在迁移头上：
//
//	| 上限 | 来源 | 实测原文 |
//	|------|------|----------|
//	| 60s  | 启动 ctx（连库/探测用的 deadline） | `goose up: ... timeout: context deadline exceeded`，60.09s 退出 |
//	| 30s  | 运行期池的 statement_timeout | 同上形态，只是更早；单条大 UPDATE/回填会被 57014 掐断 |
//	| 2s   | 运行期池的 lock_timeout（R94） | `ERROR: canceling statement due to lock timeout (SQLSTATE 55P03)`，2.07s 退出 |
//
// 前两条在日志里都贴着 "timeout"，与「SQL 写错」很难分辨 —— R103 的
// 实测两条复现都落在 timeout 分支，运维第一反应会是「再等等/重跑」，
// 而不是「迁移里有条语句错了」。
//
// 所以这里：
//   - 迁移用**独立连接池**（不与运行期池共享会话参数），显式
//     `statement_timeout=0` / `lock_timeout=0`，迁移结束即关闭；
//   - 迁移用 `context.WithoutCancel(调用方 ctx)` 起底，只由
//     `MIGRATE_TIMEOUT`（默认 0 = 不限）给预算 —— 调用方（启动 ctx /
//     cmd-migrate 的 2min ctx）的 deadline 不再决定迁移能活多久。
//
// ⚠️ 代价是「迁移卡住不会自动退出」（MIGRATE_TIMEOUT=0 时）。迁移是
// 部署动作，不是请求：把停机预算套在它身上的代价是**砍断合法回填**，
// 那比「运维显式处理卡住的迁移」更糟。
func (d *DB) Migrate(ctx context.Context) error {
	return d.runMigrations(ctx, migrations.FS, func(mctx context.Context, p *goose.Provider) error {
		if _, err := p.Up(mctx); err != nil {
			return fmt.Errorf("goose up: %w", err)
		}
		return nil
	})
}

// MigrateDown 回滚一步，仅供本地调试使用。
//
// 同样上会话锁：本地/CI 偶尔会并行跑通道，没有锁时回滚与升级可能交错。
func (d *DB) MigrateDown(ctx context.Context) error {
	return d.runMigrations(ctx, migrations.FS, func(mctx context.Context, p *goose.Provider) error {
		if _, err := p.Down(mctx); err != nil {
			return fmt.Errorf("goose down: %w", err)
		}
		return nil
	})
}

// runMigrations 建独立迁移池、套独立超时，然后交给 run 执行 goose。
// fsys 是测试注入临时迁移的入口（生产固定为 migrations.FS）。
func (d *DB) runMigrations(
	ctx context.Context,
	fsys fs.FS,
	run func(context.Context, *goose.Provider) error,
) error {
	if max := d.Pool.Config().MaxConns; max < 2 {
		return fmt.Errorf("goose: 迁移需要至少 2 个连接（会话锁独占 1 个），当前 DB_MAX_CONNS=%d", max)
	}
	mctx, cancel := d.migrationContext(ctx)
	defer cancel()

	pool, err := d.migrationPool(mctx)
	if err != nil {
		return err
	}
	defer pool.Close()

	locker, err := lock.NewPostgresSessionLocker()
	if err != nil {
		return fmt.Errorf("goose session locker: %w", err)
	}
	sqlDB := stdlib.OpenDBFromPool(pool)
	defer sqlDB.Close()
	provider, err := goose.NewProvider(goose.DialectPostgres, sqlDB, fsys,
		goose.WithSessionLocker(locker))
	if err != nil {
		return fmt.Errorf("goose provider: %w", err)
	}
	return run(mctx, provider)
}

// migrationContext 返回迁移自己的 ctx：继承调用方的值（tracing 等），
// 但不继承它的 deadline/cancel —— 只由 MIGRATE_TIMEOUT 给上限。
func (d *DB) migrationContext(ctx context.Context) (context.Context, context.CancelFunc) {
	base := context.WithoutCancel(ctx)
	if d.cfg.MigrateTimeout > 0 {
		return context.WithTimeout(base, d.cfg.MigrateTimeout)
	}
	return base, func() {}
}

// migrationPool 建一个**只给迁移用**的连接池。
//
// 为什么不是「在运行期池上 SET」：`SET` 只作用于执行它的那条会话，
// 而 goose 的会话锁会另占一条连接、迁移语句又可能落在第三条 ——
// 逐条 SET 漏一条就静默失效。独立池用 RuntimeParams 随握手下发，
// 不存在漏设；池在迁移结束后关闭，运行期连接池的参数完全不受影响。
func (d *DB) migrationPool(ctx context.Context) (*pgxpool.Pool, error) {
	if d.cfg.DatabaseURL == "" {
		return nil, fmt.Errorf("goose: 迁移需要 DATABASE_URL（当前 DB 不是由 store.Open 创建）")
	}
	poolCfg, err := pgxpool.ParseConfig(d.cfg.DatabaseURL)
	if err != nil {
		return nil, fmt.Errorf("parse DATABASE_URL（迁移池）: %w", err)
	}
	// 会话锁独占 1 条 + goose 查询至少 1 条。这里不沿用运行期池的
	// MaxConns（可能被配成 1），但仍尊重配置的上限语义：至少 2。
	maxConns := d.cfg.DBMaxConns
	if maxConns < 2 {
		maxConns = 2
	}
	poolCfg.MaxConns = maxConns
	poolCfg.MinConns = 0

	if poolCfg.ConnConfig.RuntimeParams == nil {
		poolCfg.ConnConfig.RuntimeParams = map[string]string{}
	}
	tz := d.cfg.DBTimeZone
	if tz == "" {
		tz = defaultDBTimeZone
	}
	poolCfg.ConnConfig.RuntimeParams["TimeZone"] = tz
	// 迁移专用预算：显式关掉。0 在 PG 里表示「不限」，并在迁移池的
	// 生命周期内生效；运行期池的 statement_timeout/lock_timeout 不受影响。
	poolCfg.ConnConfig.RuntimeParams["statement_timeout"] = "0"
	poolCfg.ConnConfig.RuntimeParams["lock_timeout"] = "0"

	pool, err := pgxpool.NewWithConfig(ctx, poolCfg)
	if err != nil {
		return nil, fmt.Errorf("create migration pool: %w", err)
	}
	return pool, nil
}

// Tx 在事务中执行 fn，出错自动回滚。
func (d *DB) Tx(ctx context.Context, fn func(pgx.Tx) error) error {
	return pgx.BeginFunc(ctx, d.Pool, fn)
}

// Ping 供 /healthz 使用。
func (d *DB) Ping(ctx context.Context) error { return d.Pool.Ping(ctx) }
