// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package store

import (
	"context"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/jackc/pgx/v5/pgxpool"
)

// Pool 是 pgxpool.Pool 的包装：方法签名与 pgxpool 一致（调用点零改动），
// 唯一行为差异是 **Acquire 等待有上限**（DB_ACQUIRE_TIMEOUT）。
//
// # 为什么需要它（R89 F2 实测）
//
// 池耗尽时（慢锁/慢查询占满全部连接），pgxpool.Acquire 用请求 ctx 无限
// 排队——请求挂起时长完全由故障持有时间决定（实测 20/241 个请求挂起
// 15.0s，p95=15032ms），挂起期间客户端/中间件/服务端没有任何一层掐断。
// 故障越久挂得越久，且挂起的请求占着 goroutine 与内存，把「慢」传染成
// 「全员等死」。
//
// 加上限后：排队超限 → ctx.DeadlineExceeded → httperr 既有映射
// 503 + Retry-After: 5（R67 契约，零改动）——客户端得到一个**可重试的、
// 快速的**失败，而不是十几秒的挂起。
//
// # 阈值为什么不许短（见 config 校验注释）
//
// 正常负载下 Acquire 排队几乎为零（R90 实测：20 并发抢 10 连接全链路
// p99=61ms）。超时设得太短（比如 100ms），合法突发就会把正常请求打成
// 503——客户端重试放大流量，是**假失败**，比挂起更糟。默认 5s 是正常
// p99 的 ~80 倍，只有真故障才会触达。
//
// ⚠️ Acquire 超时只覆盖「排队等连接」：拿到连接后的语句执行由
// DB_STATEMENT_TIMEOUT 管（PG 侧 57014），两者互不替代。
type Pool struct {
	inner          *pgxpool.Pool
	acquireTimeout time.Duration
}

// WrapPool 给已有池套上 Acquire 超时。acquireTimeout <= 0 时退化为直通
// （生产配置校验已拒绝 <=0，直通只为测试灵活性）。
func WrapPool(inner *pgxpool.Pool, acquireTimeout time.Duration) *Pool {
	return &Pool{inner: inner, acquireTimeout: acquireTimeout}
}

// Raw 返回底层池——给启动期一次性使用者（seeder 等）。它们不需要
// acquire 超时。
//
// ⚠️ 迁移不在这里（R103）：`Migrate` 建独立的迁移连接池，显式关掉
// statement_timeout / lock_timeout，不再借用运行期池。
func (p *Pool) Raw() *pgxpool.Pool { return p.inner }

func (p *Pool) acquire(ctx context.Context) (*pgxpool.Conn, error) {
	if p.acquireTimeout <= 0 {
		return p.inner.Acquire(ctx)
	}
	// puddle 的 ctx 只作用于「等待资源」阶段：拿到连接后与 ctx 脱钩，
	// 这里 defer cancel 不影响连接后续使用（破坏性测试
	// TestAcquireTimeoutCancelAfterSuccess 钉住这一点）。
	actx, cancel := context.WithTimeout(ctx, p.acquireTimeout)
	defer cancel()
	conn, err := p.inner.Acquire(actx)
	if err != nil {
		return nil, fmt.Errorf("acquire db connection: %w", err)
	}
	return conn, nil
}

// errRow 把 Acquire 失败包装成 pgx.Row（pgxpool 内部的 errRow 未导出）。
type errRow struct{ err error }

func (r errRow) Scan(...any) error { return r.err }

// errRows 把 Acquire 失败包装成 pgx.Rows。
type errRows struct{ err error }

func (errRows) Close()                                       {}
func (e errRows) Err() error                                 { return e.err }
func (errRows) CommandTag() pgconn.CommandTag                { return pgconn.CommandTag{} }
func (errRows) FieldDescriptions() []pgconn.FieldDescription { return nil }
func (errRows) Next() bool                                   { return false }
func (e errRows) Scan(...any) error                          { return e.err }
func (e errRows) Values() ([]any, error)                     { return nil, e.err }
func (errRows) RawValues() [][]byte                          { return nil }
func (errRows) Conn() *pgx.Conn                              { return nil }
func (errRows) TypeMap() *pgtype.Map                         { return nil }

// poolRows 在 Close（或行耗尽/扫错）时把连接还给池 —— 与 pgxpool 的
// 内部 poolRows 同语义。
//
// ⚠️ 这不是可选装饰：`pgxpool.Conn.Query` 返回的 rows **不负责** Release，
// 只有 `pgxpool.Pool.Query` 的包装才 Release。R90 首版 wrapper 直接返回
// 连接层的 rows，导致每次 Query 泄漏一条连接（TestAcquireTimeoutCancelAfterSuccess
// 在 db.Close() 处挂死即此证据）。
type poolRows struct {
	r   pgx.Rows
	c   *pgxpool.Conn
	err error
}

func (rows *poolRows) Close() {
	rows.r.Close()
	if rows.c != nil {
		rows.c.Release()
		rows.c = nil
	}
}

func (rows *poolRows) Err() error {
	if rows.err != nil {
		return rows.err
	}
	return rows.r.Err()
}

func (rows *poolRows) CommandTag() pgconn.CommandTag { return rows.r.CommandTag() }
func (rows *poolRows) FieldDescriptions() []pgconn.FieldDescription {
	return rows.r.FieldDescriptions()
}

func (rows *poolRows) Next() bool {
	if rows.err != nil {
		return false
	}
	n := rows.r.Next()
	if !n {
		rows.Close()
	}
	return n
}

func (rows *poolRows) Scan(dest ...any) error {
	err := rows.r.Scan(dest...)
	if err != nil {
		rows.Close()
	}
	return err
}

func (rows *poolRows) Values() ([]any, error) {
	v, err := rows.r.Values()
	if err != nil {
		rows.Close()
	}
	return v, err
}

func (rows *poolRows) RawValues() [][]byte  { return rows.r.RawValues() }
func (rows *poolRows) Conn() *pgx.Conn      { return rows.r.Conn() }
func (rows *poolRows) TypeMap() *pgtype.Map { return rows.r.TypeMap() }

// poolRow 在 Scan 时把连接还给池（含 Scan 内 panic 的路径）——
// 与 pgxpool 的内部 poolRow 同语义。
type poolRow struct {
	r   pgx.Row
	c   *pgxpool.Conn
	err error
}

func (row *poolRow) Scan(dest ...any) error {
	if row.err != nil {
		return row.err
	}
	panicked := true
	defer func() {
		if panicked && row.c != nil {
			row.c.Release()
		}
	}()
	err := row.r.Scan(dest...)
	panicked = false
	if row.c != nil {
		row.c.Release()
		row.c = nil
	}
	return err
}

// poolTx 在 Commit/Rollback 时把连接还给池 —— 与 pgxpool 的 Tx 同语义。
// 没有它，`db.Tx` 每次成功提交都会泄漏一条连接。
type poolTx struct {
	t pgx.Tx
	c *pgxpool.Conn
}

func (tx *poolTx) Begin(ctx context.Context) (pgx.Tx, error) { return tx.t.Begin(ctx) }

func (tx *poolTx) Commit(ctx context.Context) error {
	err := tx.t.Commit(ctx)
	if tx.c != nil {
		tx.c.Release()
		tx.c = nil
	}
	return err
}

func (tx *poolTx) Rollback(ctx context.Context) error {
	err := tx.t.Rollback(ctx)
	if tx.c != nil {
		tx.c.Release()
		tx.c = nil
	}
	return err
}

func (tx *poolTx) CopyFrom(ctx context.Context, tableName pgx.Identifier, columnNames []string, rowSrc pgx.CopyFromSource) (int64, error) {
	return tx.t.CopyFrom(ctx, tableName, columnNames, rowSrc)
}
func (tx *poolTx) SendBatch(ctx context.Context, b *pgx.Batch) pgx.BatchResults {
	return tx.t.SendBatch(ctx, b)
}
func (tx *poolTx) LargeObjects() pgx.LargeObjects { return tx.t.LargeObjects() }
func (tx *poolTx) Prepare(ctx context.Context, name, sql string) (*pgconn.StatementDescription, error) {
	return tx.t.Prepare(ctx, name, sql)
}
func (tx *poolTx) Exec(ctx context.Context, sql string, arguments ...any) (pgconn.CommandTag, error) {
	return tx.t.Exec(ctx, sql, arguments...)
}
func (tx *poolTx) Query(ctx context.Context, sql string, args ...any) (pgx.Rows, error) {
	return tx.t.Query(ctx, sql, args...)
}
func (tx *poolTx) QueryRow(ctx context.Context, sql string, args ...any) pgx.Row {
	return tx.t.QueryRow(ctx, sql, args...)
}
func (tx *poolTx) Conn() *pgx.Conn { return tx.t.Conn() }

// Query 与 pgxpool.Pool.Query 同语义：Acquire 后执行，Rows.Close() 时
// 自动 Release。
func (p *Pool) Query(ctx context.Context, sql string, args ...any) (pgx.Rows, error) {
	conn, err := p.acquire(ctx)
	if err != nil {
		return errRows{err}, err
	}
	rows, err := conn.Query(ctx, sql, args...)
	if err != nil {
		conn.Release()
		return errRows{err}, err
	}
	return &poolRows{r: rows, c: conn}, nil
}

// QueryRow 与 pgxpool.Pool.QueryRow 同语义：返回的 Row 在 Scan 时自动 Release。
func (p *Pool) QueryRow(ctx context.Context, sql string, args ...any) pgx.Row {
	conn, err := p.acquire(ctx)
	if err != nil {
		return errRow{err}
	}
	return &poolRow{r: conn.QueryRow(ctx, sql, args...), c: conn}
}

// Exec 与 pgxpool.Pool.Exec 同语义：Acquire → 执行 → Release。
func (p *Pool) Exec(ctx context.Context, sql string, args ...any) (pgconn.CommandTag, error) {
	conn, err := p.acquire(ctx)
	if err != nil {
		return pgconn.CommandTag{}, err
	}
	defer conn.Release()
	return conn.Exec(ctx, sql, args...)
}

// Begin 与 pgxpool.Pool.Begin 同语义：Acquire 后在连接上 BEGIN，
// Commit/Rollback 时 Release。
//
// ⚠️ R90 边界（用户要求 5）：Acquire 超时发生在**事务开始之前**——
// 超时即「还没拿到连接」，不存在「事务已开但等第二条连接」的半状态。
// 「事务内再取池」由 R83 的 AST 守卫覆盖（当前 0 违规）。
func (p *Pool) Begin(ctx context.Context) (pgx.Tx, error) {
	conn, err := p.acquire(ctx)
	if err != nil {
		return nil, err
	}
	tx, err := conn.Begin(ctx)
	if err != nil {
		conn.Release()
		return nil, err
	}
	return &poolTx{t: tx, c: conn}, nil
}

// BeginTx 与 pgxpool.Pool.BeginTx 同语义（pgx.BeginFunc 走这条）。
func (p *Pool) BeginTx(ctx context.Context, opts pgx.TxOptions) (pgx.Tx, error) {
	conn, err := p.acquire(ctx)
	if err != nil {
		return nil, err
	}
	tx, err := conn.BeginTx(ctx, opts)
	if err != nil {
		conn.Release()
		return nil, err
	}
	return &poolTx{t: tx, c: conn}, nil
}

// Acquire 直通底层池（带超时）。优先用 Query/Exec/Begin；直接 Acquire
// 的调用点必须 defer Release。
func (p *Pool) Acquire(ctx context.Context) (*pgxpool.Conn, error) {
	return p.acquire(ctx)
}

// Ping 带 Acquire 超时（readyz 不该在池耗尽时挂起）。
func (p *Pool) Ping(ctx context.Context) error {
	if p.acquireTimeout <= 0 {
		return p.inner.Ping(ctx)
	}
	actx, cancel := context.WithTimeout(ctx, p.acquireTimeout)
	defer cancel()
	return p.inner.Ping(actx)
}

// Stat / Config / Close 直通。
func (p *Pool) Stat() *pgxpool.Stat     { return p.inner.Stat() }
func (p *Pool) Config() *pgxpool.Config { return p.inner.Config() }
func (p *Pool) Close()                  { p.inner.Close() }
