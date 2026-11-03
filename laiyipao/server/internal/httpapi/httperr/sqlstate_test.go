// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httperr_test

// R69：SQLSTATE 映射的**正负锚点全表**。
//
// 为什么不能只测分类函数：
//
//	分类函数对了，但 Render 的分支顺序错了（比如结算拒绝放在瞬时故障前面，
//	或瞬时故障放在 500 前面却漏了写 Retry-After），对外行为仍然是错的。
//
// 所以这里的每个锚点都走一次**真实的 Render**（经 Fiber 的 ErrorHandler），
//	断言最终 status / code / Retry-After，而不是断言 TransientDependency 的返回值。
//
// 两类触发：
//   - 注入：`*pgconn.PgError` 直接构造（08 类、57P02/57P03、53300、53200 这类
//     在测试机上一次难以真实制造的服务端故障）；
//   - 真库：23505、42P01、57014、57P01 用一次性库真实触发（超出注入的可信度）。
//
// 负锚点的意义与正锚点同等重要：23505/42P01 若被判成 503，客户端会无限重试
// 一个「重试也没有用」的错误；context.Canceled 判成 503 会制造误导性告警。

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"

	"github.com/laiyipao/server/internal/httpapi/httperr"
	"github.com/laiyipao/server/internal/testutil/scratchdb"
)

// renderOnce 把错误交给真实的 httperr.Render，返回对外可见的三元组。
func renderOnce(t *testing.T, err error) (status int, code, retryAfter string) {
	t.Helper()
	app := fiber.New(fiber.Config{
		ErrorHandler:          httperr.Render,
		DisableStartupMessage: true,
	})
	app.Get("/x", func(c *fiber.Ctx) error { return err })
	resp, testErr := app.Test(httptest.NewRequest("GET", "/x", nil))
	if testErr != nil {
		t.Fatalf("请求执行失败：%v", testErr)
	}
	defer resp.Body.Close()
	var body httperr.APIError
	if decErr := json.NewDecoder(resp.Body).Decode(&body); decErr != nil {
		t.Fatalf("响应不是 JSON（status=%d）：%v", resp.StatusCode, decErr)
	}
	return resp.StatusCode, body.Error.Code, resp.Header.Get(fiber.HeaderRetryAfter)
}

// assertRender 是一条锚点的判据。
func assertRender(t *testing.T, name string, err error, wantStatus int, wantCode, wantRetry string) {
	t.Helper()
	status, code, retry := renderOnce(t, err)
	if status != wantStatus || code != wantCode || retry != wantRetry {
		t.Errorf("%s：Render 得到 (%d, %q, Retry-After=%q)，期望 (%d, %q, %q)",
			name, status, code, retry, wantStatus, wantCode, wantRetry)
	}
}

// TestSQLSTATEAnchorTable 是注入触发的正负锚点全表。
func TestSQLSTATEAnchorTable(t *testing.T) {
	cases := []struct {
		name       string
		err        error
		wantStatus int
		wantCode   string
		wantRetry  string
	}{
		// ---- 正锚点：连接异常（class 08）----
		// 08 类每个子码都代表「连接层坏了」，客户端等一会儿重连可能成功。
		{"08001 无法建立连接", &pgconn.PgError{Code: "08001"}, 503, "service_unavailable", "5"},
		{"08003 连接不存在", &pgconn.PgError{Code: "08003"}, 503, "service_unavailable", "5"},
		{"08006 连接失败", &pgconn.PgError{Code: "08006"}, 503, "service_unavailable", "5"},
		{"08P01 协议破坏", &pgconn.PgError{Code: "08P01"}, 503, "service_unavailable", "5"},

		// ---- 正锚点：服务端临时不可用 ----
		{"57P01 后台终止", &pgconn.PgError{Code: "57P01"}, 503, "service_unavailable", "5"},
		{"57P02 服务端崩溃", &pgconn.PgError{Code: "57P02"}, 503, "service_unavailable", "5"},
		{"57P03 暂不可连接", &pgconn.PgError{Code: "57P03"}, 503, "service_unavailable", "5"},
		{"53300 连接数已满", &pgconn.PgError{Code: "53300"}, 503, "service_unavailable", "5"},
		{"53200 内存不足", &pgconn.PgError{Code: "53200"}, 503, "service_unavailable", "5"},

		// ---- 正锚点：语句被取消 ----
		// 57014 是本仓库 DB_STATEMENT_TIMEOUT 的实际错误码。
		{"57014 语句超时", &pgconn.PgError{Code: "57014"}, 503, "service_unavailable", "5"},

		// ---- 负锚点：重试没有意义 ----
		{"23505 唯一冲突", &pgconn.PgError{Code: "23505"}, 500, "internal_error", ""},
		{"42P01 表不存在", &pgconn.PgError{Code: "42P01"}, 500, "internal_error", ""},
		{"context.Canceled（客户端断开）", context.Canceled, 500, "internal_error", ""},
		// 对照：未登记的 SQLSTATE 不能被「兜底当 503」。
		{"XX000 内部错误", &pgconn.PgError{Code: "XX000"}, 500, "internal_error", ""},
		{"42601 语法错误", &pgconn.PgError{Code: "42601"}, 500, "internal_error", ""},
	}

	for _, tc := range cases {
		// 包装一层：production 里的错误几乎总是被 %w 包过的，
		// 若分类只认顶层类型，这里会全部退化成 500。
		// 这同时验证 errors.Is / errors.As 的穿透。
		wrapped := fmt.Errorf("repo 查询失败: %w", tc.err)
		assertRender(t, tc.name, wrapped, tc.wantStatus, tc.wantCode, tc.wantRetry)
	}
}

// TestSQLSTATEAnchorsFromRealDatabase 用一次性库真实触发四个可稳定复现的锚点。
//
// 注入能证明「分类逻辑」，只有真库能证明「pgx 实际交出来的错误形状
// 与我们假设的一致」—— 例如 57014 在 pgx v5 里到底是 PgError 还是
// 包装过的 context 错误，只有跑一次才知道。
func TestSQLSTATEAnchorsFromRealDatabase(t *testing.T) {
	dsn := scratchdb.DSN(t, "laiyipao_httpapi_e2e_")
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	conn, err := pgx.Connect(ctx, dsn)
	if err != nil {
		t.Fatalf("连接一次性库失败：%v", err)
	}
	t.Cleanup(func() { _ = conn.Close(context.Background()) })

	// --- 42P01：查询不存在的表 ---
	_, err = conn.Exec(ctx, `SELECT * FROM __r69_no_such_table`)
	if err == nil {
		t.Fatal("查询不存在的表竟然成功")
	}
	assertRender(t, "真库 42P01", err, 500, "internal_error", "")

	// --- 23505：唯一约束冲突 ---
	if _, err := conn.Exec(ctx, `CREATE TEMP TABLE r69_uniq (k int UNIQUE)`); err != nil {
		t.Fatalf("建临时表失败：%v", err)
	}
	if _, err := conn.Exec(ctx, `INSERT INTO r69_uniq VALUES (1)`); err != nil {
		t.Fatalf("首次插入失败：%v", err)
	}
	_, err = conn.Exec(ctx, `INSERT INTO r69_uniq VALUES (1)`)
	if err == nil {
		t.Fatal("重复插入竟然成功")
	}
	assertRender(t, "真库 23505", err, 500, "internal_error", "")

	// --- 57014：statement_timeout 打断 pg_sleep ---
	if _, err := conn.Exec(ctx, `SET statement_timeout = 80`); err != nil {
		t.Fatalf("设置 statement_timeout 失败：%v", err)
	}
	_, err = conn.Exec(ctx, `SELECT pg_sleep(0.5)`)
	if err == nil {
		t.Fatal("pg_sleep 未被 statement_timeout 打断")
	}
	assertRender(t, "真库 57014", err, 503, "service_unavailable", "5")
	if _, err := conn.Exec(ctx, `SET statement_timeout = 0`); err != nil {
		t.Fatalf("恢复 statement_timeout 失败：%v", err)
	}

	// --- 57P01：从另一个连接杀掉本连接的后端 ---
	victim, err := pgx.Connect(ctx, dsn)
	if err != nil {
		t.Fatalf("连接 victim 失败：%v", err)
	}
	t.Cleanup(func() { _ = victim.Close(context.Background()) })
	var victimPID int
	if err := victim.QueryRow(ctx, `SELECT pg_backend_pid()`).Scan(&victimPID); err != nil {
		t.Fatalf("读 victim pid 失败：%v", err)
	}
	if _, err := conn.Exec(ctx, `SELECT pg_terminate_backend($1)`, victimPID); err != nil {
		t.Fatalf("终止 victim 失败：%v", err)
	}
	// 被终止的连接下一次使用时的错误形状由 pgx 决定：
	// 可能是 PgError 57P01，也可能只是「连接已关闭」。
	// 两种都必须落进 503 —— 这正是本条要钉住的。
	_, err = victim.Exec(ctx, `SELECT 1`)
	if err == nil {
		t.Fatal("被终止的连接仍然可用")
	}
	var pgErr *pgconn.PgError
	if errors.As(err, &pgErr) {
		if pgErr.Code != "57P01" {
			t.Logf("pgx 交出的 SQLSTATE 是 %s（不是 57P01），仍按瞬时故障断言", pgErr.Code)
		}
	} else {
		t.Logf("pgx 未交出 PgError，实际错误：%v（按瞬时故障断言）", err)
	}
	assertRender(t, "真库 57P01", err, 503, "service_unavailable", "5")

	// --- context.Canceled：真实取消一个在途查询 ---
	cancelConn, err := pgx.Connect(ctx, dsn)
	if err != nil {
		t.Fatalf("连接 cancelConn 失败：%v", err)
	}
	t.Cleanup(func() { _ = cancelConn.Close(context.Background()) })
	runCtx, runCancel := context.WithCancel(context.Background())
	go func() {
		time.Sleep(50 * time.Millisecond)
		runCancel()
	}()
	_, err = cancelConn.Exec(runCtx, `SELECT pg_sleep(0.5)`)
	if err == nil {
		t.Fatal("查询未被取消")
	}
	if !errors.Is(err, context.Canceled) && !strings.Contains(err.Error(), "cancel") {
		t.Logf("取消后 pgx 实际错误：%v", err)
	}
	assertRender(t, "真库 context.Canceled", err, 500, "internal_error", "")
}
