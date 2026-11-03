// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// Package store 负责 pgx 连接池与数据库迁移。
package store

import (
	"context"
	"fmt"
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
	// ⚠️ 它同样作用于启动时的迁移（迁移复用同一个池）：大回填迁移如果
	// 可能超过上限，应在那次启动显式设 DB_STATEMENT_TIMEOUT=0。
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
	return &DB{Pool: WrapPool(pool, cfg.DBAcquireTimeout)}, nil
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
func (d *DB) Migrate(ctx context.Context) error {
	if max := d.Pool.Config().MaxConns; max < 2 {
		return fmt.Errorf("goose: 迁移需要至少 2 个连接（会话锁独占 1 个），当前 DB_MAX_CONNS=%d", max)
	}
	locker, err := lock.NewPostgresSessionLocker()
	if err != nil {
		return fmt.Errorf("goose session locker: %w", err)
	}
	sqlDB := stdlib.OpenDBFromPool(d.Pool.Raw())
	defer sqlDB.Close()
	provider, err := goose.NewProvider(goose.DialectPostgres, sqlDB, migrations.FS,
		goose.WithSessionLocker(locker))
	if err != nil {
		return fmt.Errorf("goose provider: %w", err)
	}
	if _, err := provider.Up(ctx); err != nil {
		return fmt.Errorf("goose up: %w", err)
	}
	return nil
}

// MigrateDown 回滚一步，仅供本地调试使用。
//
// 同样上会话锁：本地/CI 偶尔会并行跑通道，没有锁时回滚与升级可能交错。
func (d *DB) MigrateDown(ctx context.Context) error {
	if max := d.Pool.Config().MaxConns; max < 2 {
		return fmt.Errorf("goose: 迁移需要至少 2 个连接（会话锁独占 1 个），当前 DB_MAX_CONNS=%d", max)
	}
	locker, err := lock.NewPostgresSessionLocker()
	if err != nil {
		return fmt.Errorf("goose session locker: %w", err)
	}
	sqlDB := stdlib.OpenDBFromPool(d.Pool.Raw())
	defer sqlDB.Close()
	provider, err := goose.NewProvider(goose.DialectPostgres, sqlDB, migrations.FS,
		goose.WithSessionLocker(locker))
	if err != nil {
		return fmt.Errorf("goose provider: %w", err)
	}
	if _, err := provider.Down(ctx); err != nil {
		return fmt.Errorf("goose down: %w", err)
	}
	return nil
}

// Tx 在事务中执行 fn，出错自动回滚。
func (d *DB) Tx(ctx context.Context, fn func(pgx.Tx) error) error {
	return pgx.BeginFunc(ctx, d.Pool, fn)
}

// Ping 供 /healthz 使用。
func (d *DB) Ping(ctx context.Context) error { return d.Pool.Ping(ctx) }
