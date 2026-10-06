// Package store 负责 pgx 连接池与数据库迁移。
package store

import (
	"context"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/jackc/pgx/v5/stdlib"
	"github.com/pressly/goose/v3"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/migrations"
)

// DB 包装 pgx 连接池，向上层暴露统一的 Query/Exec/事务接口。
type DB struct {
	Pool *pgxpool.Pool
}

// defaultDBTimeZone 是会话时区的兜底值。
//
// ⚠️ 选 Asia/Shanghai 而不是 UTC：业务是中文小程序，
// 「今天」应当等于**玩家所在时区的自然日**。
// 用 UTC 会让每日任务/签到在北京时间 08:00 翻页。
//
// 想改就在环境变量 `DB_TIMEZONE` 上改，不必改代码。
const defaultDBTimeZone = "Asia/Shanghai"

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
	return &DB{Pool: pool}, nil
}

// Close 释放连接池。
func (d *DB) Close() {
	if d != nil && d.Pool != nil {
		d.Pool.Close()
	}
}

// Migrate 执行 migrations 包内嵌的 goose 迁移。
//
// 两点说明：
//  1. goose 的 Provider API 只接受 *sql.DB，因此用 pgx 官方的 stdlib 适配层
//     把连接池包一层；连接池本身不受影响，stdlib 复用同一批底层连接。
//  2. migrations.FS 用 //go:embed *.sql 嵌入，文件位于 FS 根目录，
//     因此 goose 的目录参数是 "." 而不是 "migrations"。
func (d *DB) Migrate(ctx context.Context) error {
	goose.SetBaseFS(migrations.FS)
	if err := goose.SetDialect("postgres"); err != nil {
		return fmt.Errorf("goose dialect: %w", err)
	}
	sqlDB := stdlib.OpenDBFromPool(d.Pool)
	defer sqlDB.Close()
	if err := goose.UpContext(ctx, sqlDB, "."); err != nil {
		return fmt.Errorf("goose up: %w", err)
	}
	return nil
}

// MigrateDown 回滚一步，仅供本地调试使用。
func (d *DB) MigrateDown(ctx context.Context) error {
	goose.SetBaseFS(migrations.FS)
	if err := goose.SetDialect("postgres"); err != nil {
		return err
	}
	sqlDB := stdlib.OpenDBFromPool(d.Pool)
	defer sqlDB.Close()
	return goose.DownContext(ctx, sqlDB, ".")
}

// Tx 在事务中执行 fn，出错自动回滚。
func (d *DB) Tx(ctx context.Context, fn func(pgx.Tx) error) error {
	return pgx.BeginFunc(ctx, d.Pool, fn)
}

// Ping 供 /healthz 使用。
func (d *DB) Ping(ctx context.Context) error { return d.Pool.Ping(ctx) }
