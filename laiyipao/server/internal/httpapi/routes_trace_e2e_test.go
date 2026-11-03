// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// routes_trace_e2e_test.go — R84 的端到端链路证明：
// 同一次 HTTP 请求，响应头的 X-Request-Id、访问日志的 req=、
// 慢结算日志的 req= 必须是**同一个值**。
//
// 中间件层的单测只能证明「中间件会写 Locals」；这条证明的是整条链：
// fiber Locals → *fasthttp.RequestCtx（作为 context.Context）→ service
// 层 TraceIDFrom → fmt.Printf —— 任何一环断了，慢日志里的 id
// 就不是响应头里那个，「按 id 拼故事」当场破功。

import (
	"io"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/config"
)

func TestTraceIDFlowsIntoSettleSlowLog(t *testing.T) {
	e := newE2EWith(t, func(c *config.Config) { c.SettleSlowMs = 1 })
	uid, token := e.newPlayer(t, "r84trace")
	e.exec(t, `UPDATE user_wallets SET energy = 100 WHERE user_id = $1`, uid)

	// 开局拿 token_id。
	st, body := e.post(t, "/api/v1/battle/token", token, map[string]any{"level_id": 1})
	if st != 200 {
		t.Fatalf("开局失败 status=%d body=%v", st, body)
	}
	tokenID := int64(body["token_id"].(float64))

	// 打结算：带一个**合法白名单**的 X-Request-Id，慢日志必须含同款 req=。
	const inherited = "r84e2e-link01"
	req := httptest.NewRequest("POST", "/api/v1/battle/settle", mustJSON(t, e.settleBody(t, tokenID)))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("X-Request-Id", inherited)

	logged := captureStdoutH(t, func() {
		resp, err := e.app.Test(req, 30000)
		if err != nil {
			t.Fatalf("结算请求失败：%v", err)
		}
		defer resp.Body.Close()
		if resp.StatusCode != 200 {
			t.Fatalf("结算 status=%d", resp.StatusCode)
		}
		if got := resp.Header.Get("X-Request-Id"); got != inherited {
			t.Errorf("响应头 X-Request-Id=%q，应继承 %q", got, inherited)
		}
	})

	var slowLine string
	for _, line := range strings.Split(logged, "\n") {
		if strings.Contains(line, "[settle-slow]") {
			slowLine = line
		}
	}
	if slowLine == "" {
		t.Fatalf("阈值 1ms 下没有慢结算日志：%q", logged)
	}
	if !strings.Contains(slowLine, "req="+inherited+" ") {
		t.Errorf("慢结算日志的 req= 应是继承的 %q，实际行: %q", inherited, slowLine)
	}
	// 禁词纪律（R80 列表）在新增 req= 字段后仍然成立。
	for _, banned := range []string{"replay", "seed", "hash=", "token=", "nickname", "昵称"} {
		if strings.Contains(strings.ToLower(slowLine), banned) {
			t.Errorf("慢结算日志含禁词 %q：%q", banned, slowLine)
		}
	}
}

// captureStdoutH 捕获 fn 期间的 stdout（httpapi 版；service 有同名夹具）。
func captureStdoutH(t *testing.T, fn func()) string {
	t.Helper()
	old := os.Stdout
	r, w, err := os.Pipe()
	if err != nil {
		t.Fatalf("建管道失败：%v", err)
	}
	os.Stdout = w
	defer func() { os.Stdout = old }()

	done := make(chan string)
	go func() {
		raw, _ := io.ReadAll(r)
		done <- string(raw)
	}()
	fn()
	_ = w.Close()
	out := <-done
	os.Stdout = old
	return out
}
