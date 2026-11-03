// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R67：数据库断开/超时的错误语义探针。
//
// 判据：客户端必须能区分「重试有意义」（503 + Retry-After）与
// 「重试无意义」（500 / 422 / 400）。因此这里模拟连接池被关闭，
// 断言 DB 依赖端点返回 503，而不是兜底的 500。

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net"
	"net/http"
	"net/http/httptest"
	"syscall"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/puddle/v2"

	"github.com/laiyipao/server/internal/httpapi/httperr"
)

// TestTransientDependencyErrorClassification 钉住分类边界：
// 瞬时故障归 503，真 bug（约束冲突/表不存在/普通错误）必须留在 500，
// 客户端主动取消不能被算成「服务不可用」。
func TestTransientDependencyErrorClassification(t *testing.T) {
	cases := []struct {
		name string
		err  error
		want bool
	}{
		{"连接池关闭", puddle.ErrClosedPool, true},
		{"包装后的连接池关闭", fmt.Errorf("query: %w", puddle.ErrClosedPool), true},
		{"加载超时", context.DeadlineExceeded, true},
		{"客户端取消", context.Canceled, false},
		{"语句超时 57014", &pgconn.PgError{Code: "57014"}, true},
		{"连接异常 08P01", &pgconn.PgError{Code: "08P01"}, true},
		{"连接数满 53300", &pgconn.PgError{Code: "53300"}, true},
		{"后台终止 57P01", &pgconn.PgError{Code: "57P01"}, true},
		{"唯一冲突 23505", &pgconn.PgError{Code: "23505"}, false},
		{"表不存在 42P01", &pgconn.PgError{Code: "42P01"}, false},
		{"网络不可达", &net.OpError{Op: "dial", Net: "tcp", Err: syscall.ECONNREFUSED}, true},
		{"普通错误", errors.New("boom"), false},
		{"nil", nil, false},
	}
	for _, tc := range cases {
		if got := httperr.IsTransientDependency(tc.err); got != tc.want {
			t.Errorf("%s：transientDependencyError=%v，期望 %v", tc.name, got, tc.want)
		}
	}
}

func TestDBUnavailableMapsTo503(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	if got := callAs(t, e.app, http.MethodGet, "/api/v1/leaderboard?type=power", ""); got != http.StatusOK {
		t.Fatalf("预热：正常路径应为 200，实际 %d", got)
	}

	// 模拟数据库断开。关闭后任何 Acquire 都返回 puddle.ErrClosedPool，
	// 与真实断连（连接被回收/网络不可达）在 HTTP 层的处理应当是同一类。
	e.svc.DB.Pool.Close()
	// 榜单有 15s 缓存，必须失效后再打，否则命中的是断连前快照（200），
	// 探针会误判成「错误语义没问题」。顺带确认：断连期间读缓存本身仍可用。
	e.svc.FlushStatsCache()

	req := httptest.NewRequest(http.MethodGet, "/api/v1/leaderboard?type=power", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != fiber.StatusServiceUnavailable {
		var body struct {
			Error struct {
				Code string `json:"code"`
			} `json:"error"`
		}
		_ = json.NewDecoder(resp.Body).Decode(&body)
		t.Fatalf("DB 不可用应返回 503，实际 %d（code=%q）；客户端无法区分可重试",
			resp.StatusCode, body.Error.Code)
	}
	if got := resp.Header.Get("Retry-After"); got != "5" {
		t.Fatalf("503 必须带 Retry-After: 5，实际 %q", got)
	}
	var body struct {
		Error struct {
			Code string `json:"code"`
		} `json:"error"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("503 响应体不是 JSON：%v", err)
	}
	if body.Error.Code != "service_unavailable" {
		t.Fatalf("503 响应 code 应为 service_unavailable，实际 %q", body.Error.Code)
	}
}
