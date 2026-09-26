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
