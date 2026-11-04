// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package scratchdb

// 本文件测「建库 / 用库 / 清库」这条链路本身。
//
// 为什么这值得单独测：本包的三类故障都是**静默**的 ——
// 建库失败被当成「没权限」而跳过（测试全绿但用例没跑）、
// 清理失败只在孤儿库累积到几百个时才显形、
// 管理库不可达时 Sweep 必须返回 0 而不是制造假红。
// 上面的守卫测命名与登记；这里测真的 CREATE、真的连、真的 DROP。

import (
	"context"
	"errors"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
)

// TestAdminDSNRewritesDatabaseName 管理库 DSN 必须只换库名，其它部分逐字保留。
func TestAdminDSNRewritesDatabaseName(t *testing.T) {
	cases := []struct{ name, testURL, dataURL, want string }{
		{
			name:    "带查询参数",
			testURL: "postgres://u:p@127.0.0.1:5432/laiyipao_any?sslmode=disable",
			want:    "postgres://u:p@127.0.0.1:5432/postgres?sslmode=disable",
		},
		{
			name:    "不带查询参数",
			testURL: "postgres://u:p@127.0.0.1:5432/laiyipao_any",
			want:    "postgres://u:p@127.0.0.1:5432/postgres",
		},
		{
			name:    "TEST_DATABASE_URL 优先于 DATABASE_URL",
			testURL: "postgres://a@127.0.0.1:5432/laiyipao_test",
			dataURL: "postgres://b@127.0.0.1:5432/laiyipao_dev",
			want:    "postgres://a@127.0.0.1:5432/postgres",
		},
		{
			name: "两个变量都空时用本地默认管理库",
			want: "postgres://postgres@127.0.0.1:5432/postgres?sslmode=disable",
		},
		{
			name:    "没有斜杠的 key=value DSN 原样返回",
			testURL: "host=127.0.0.1 port=5432 dbname=laiyipao_test",
			want:    "host=127.0.0.1 port=5432 dbname=laiyipao_test",
		},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			t.Setenv("TEST_DATABASE_URL", c.testURL)
			t.Setenv("DATABASE_URL", c.dataURL)
			if got := AdminDSN(); got != c.want {
				t.Fatalf("AdminDSN() = %q，期望 %q", got, c.want)
			}
		})
	}
}

// TestSwapDBKeepsHostAndOptions 换库名时不能顺手改掉主机、端口或参数。
func TestSwapDBKeepsHostAndOptions(t *testing.T) {
	cases := []struct{ admin, name, want string }{
		{"postgres://u:p@127.0.0.1:5432/postgres?sslmode=disable", "laiyipao_x_1", "postgres://u:p@127.0.0.1:5432/laiyipao_x_1?sslmode=disable"},
		{"postgres://u:p@127.0.0.1:5432/postgres", "laiyipao_x_2", "postgres://u:p@127.0.0.1:5432/laiyipao_x_2"},
		{"no-slash-dsn", "laiyipao_x_3", "no-slash-dsn"},
	}
	for _, c := range cases {
		if got := swapDB(c.admin, c.name); got != c.want {
			t.Errorf("swapDB(%q, %q) = %q，期望 %q", c.admin, c.name, got, c.want)
		}
	}
}

// TestQuoteIdentEscapesDoubleQuotes DROP DATABASE 不能参数化，只能拼串 ——
// 于是库名里的双引号必须转义，否则一个带引号的名字就能拼出注入。
func TestQuoteIdentEscapesDoubleQuotes(t *testing.T) {
	cases := map[string]string{
		"laiyipao_x_1": `"laiyipao_x_1"`,
		`a"b`:          `"a""b"`,
		`a""b`:         `"a""""b"`,
	}
	for in, want := range cases {
		if got := quoteIdent(in); got != want {
			t.Errorf("quoteIdent(%q) = %q，期望 %q", in, got, want)
		}
	}
}

// TestPoolReportsUnreachableAdminDB 管理库连不上时，pool 必须返回错误，
// 且 SweepPrefix 必须把它当成「本机没条件」返回 (0, nil) —— 而不是制造假红。
func TestPoolReportsUnreachableAdminDB(t *testing.T) {
	// 127.0.0.1:1 是保留端口，不会有服务在听；连接会被立即拒绝。
	t.Setenv("TEST_DATABASE_URL", "postgres://postgres@127.0.0.1:1/postgres?sslmode=disable")
	t.Setenv("DATABASE_URL", "")

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if p, err := pool(ctx); err == nil {
		p.Close()
		t.Fatal("管理库不可达时 pool 仍返回了可用连接池")
	}

	n, err := SweepPrefix("laiyipao_never_exists_")
	if err != nil {
		t.Fatalf("管理库不可达时 SweepPrefix 应返回 (0, nil)，实际错误：%v", err)
	}
	if n != 0 {
		t.Fatalf("管理库不可达时不应清掉任何库，实际 n=%d", n)
	}
}

// TestSweepPrefixDropsEveryMatchingDatabase Sweep 必须真的清掉前缀下的残留库。
//
// 症状对照：清理只「看起来写了」时，残留量会悄悄回到几百个，
// 然后在某次 CREATE DATABASE 超时后表现为「测试莫名其妙变红」。
func TestSweepPrefixDropsEveryMatchingDatabase(t *testing.T) {
	// 用局部变量而不是前缀常量：本包的前缀常量守卫只认 const 声明。
	prefix := "laiyipao_sweep_self_"
	p := adminPoolOrSkip(t)
	defer p.Close()
	requireCreateDB(t, p)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	var created []string
	// 断言失败时也别留下孤儿库。
	// ⚠️ 这里不能复用 p：defer p.Close() 先于 t.Cleanup 执行，复用会拿到 closed pool
	// —— 那正是本包注释里记着的「每个库都留成孤儿」的成因。
	t.Cleanup(func() {
		cleanCtx, cleanCancel := context.WithTimeout(context.Background(), 30*time.Second)
		defer cleanCancel()
		dp, err := pool(cleanCtx)
		if err != nil {
			return
		}
		defer dp.Close()
		for _, name := range created {
			_, _ = dp.Exec(cleanCtx, `SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1`, name)
			_, _ = dp.Exec(cleanCtx, `DROP DATABASE IF EXISTS `+quoteIdent(name))
		}
	})

	for i := 0; i < 2; i++ {
		name := Name(prefix)
		if _, err := p.Exec(ctx, `CREATE DATABASE `+quoteIdent(name)); err != nil {
			t.Fatalf("造残留库失败：%v", err)
		}
		created = append(created, name)
	}

	got, err := SweepPrefix(prefix)
	if err != nil {
		t.Fatalf("SweepPrefix 失败：%v", err)
	}
	if got != len(created) {
		t.Fatalf("SweepPrefix 清了 %d 个库，期望 %d 个", got, len(created))
	}

	var left int
	if err := p.QueryRow(ctx,
		`SELECT COUNT(*) FROM pg_database WHERE datname LIKE $1`, prefix+"%").Scan(&left); err != nil {
		t.Fatalf("复查残留库失败：%v", err)
	}
	if left != 0 {
		t.Fatalf("Sweep 之后仍残留 %d 个库", left)
	}

	// 无残留时 Sweep 也必须安静通过（不因 n=0 报错）。
	Sweep(t, prefix)
}

// TestOpenCreatesUsableDatabaseAndCleanupDropsIt 建出来的库必须真的能连，
// cleanup 必须真的把它删掉 —— 而不是留着下轮再清。
func TestOpenCreatesUsableDatabaseAndCleanupDropsIt(t *testing.T) {
	prefix := "laiyipao_open_self_"
	admin := adminPoolOrSkip(t)
	defer admin.Close()
	requireCreateDB(t, admin)

	dsn, cleanup, err := Open(prefix)
	if err != nil || dsn == "" {
		if errors.Is(err, ErrNoPermission) {
			t.Skip(err.Error())
		}
		t.Fatalf("Open 失败：%v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// 真的连上去执行一条语句：DSN 拼错时这里会立刻红。
	conn, err := pgxpool.New(ctx, dsn)
	if err != nil {
		cleanup()
		t.Fatalf("用 Open 返回的 DSN 建连接池失败：%v", err)
	}
	if err := conn.Ping(ctx); err != nil {
		conn.Close()
		cleanup()
		t.Fatalf("Open 返回的 DSN 连不上：%v", err)
	}
	conn.Close()

	name := dbNameFromDSN(dsn)
	cleanup()

	var left int
	if err := admin.QueryRow(ctx,
		`SELECT COUNT(*) FROM pg_database WHERE datname = $1`, name).Scan(&left); err != nil {
		t.Fatalf("复查库是否已删失败：%v", err)
	}
	if left != 0 {
		t.Fatalf("cleanup 之后库 %s 仍然存在", name)
	}
}

// TestDSNReturnsUsableDatabase DSN 是带 *testing.T 的入口：成功时返回可连的 DSN，
// 且把 cleanup 挂到 t.Cleanup（这里只验证「能连」，回收由 t.Cleanup 兜底）。
func TestDSNReturnsUsableDatabase(t *testing.T) {
	prefix := "laiyipao_dsn_self_"
	// 先探管理库：Open 在管理库不可达时返回普通错误（DSN 会 Fatal），
	// 而本机没 PG 时本包应跳过而不是变红。
	admin := adminPoolOrSkip(t)
	admin.Close()

	dsn := DSN(t, prefix)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	conn, err := pgxpool.New(ctx, dsn)
	if err != nil {
		t.Fatalf("用 DSN 建连接池失败：%v", err)
	}
	defer conn.Close()
	if err := conn.Ping(ctx); err != nil {
		t.Fatalf("DSN 连不上：%v", err)
	}
}

// adminPoolOrSkip 打开管理库连接池；不可达时跳过（与本包 README 的约定一致）。
func adminPoolOrSkip(t *testing.T) *pgxpool.Pool {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	p, err := pool(ctx)
	if err != nil {
		t.Skipf("管理库不可达，跳过 DB 用例：%v", err)
	}
	return p
}

// requireCreateDB 探测 CREATEDB 权限；没有权限时跳过（这是「本机没条件」，不是故障）。
func requireCreateDB(t *testing.T, p *pgxpool.Pool) {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if !canCreateDatabases(ctx, p) {
		t.Skip("当前账号没有 CREATEDB 权限，跳过 DB 用例")
	}
}

// dbNameFromDSN 从 DSN 里取出库名（最后一个 / 与 ? 之间）。
func dbNameFromDSN(dsn string) string {
	i := strings.LastIndex(dsn, "/")
	if i < 0 {
		return ""
	}
	rest := dsn[i+1:]
	if j := strings.Index(rest, "?"); j >= 0 {
		return rest[:j]
	}
	return rest
}
