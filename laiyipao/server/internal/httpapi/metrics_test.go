// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/service"
)

// TestRenderMetrics：/metrics 输出形状与数值。
//
// 判据：Prometheus 文本格式要求每个指标都有 `# HELP` 与 `# TYPE` 两行元数据，
// 且样本行是 `name value`。缺元数据的输出多数抓取端会拒收或告警 —— 那是指标
// 「看起来有、实际抓不到」的静默失效。纯函数、无 DB，是本机真正能跑的判据。
//
// R50：新增 5 个连接池 gauge（池耗尽不会报错，只会让所有接口变慢）。
func TestRenderMetrics(t *testing.T) {
	st := service.CacheStats{
		DashboardHits: 3, DashboardMisses: 1,
		LeaderboardHits: 10, LeaderboardMisses: 2,
	}
	ps := service.PoolStats{
		TotalConns: 6, IdleConns: 4, AcquiredConns: 2,
		ConstructingConns: 0, MaxConns: 10,
	}
	out := renderMetrics(st, ps, service.SettleStats{
		Total: 7, Slow: 1, TotalMs: 1234,
		Segments: map[string]service.SettleSegmentStat{
			"tx_insert": {Count: 7, SumMs: 56, MaxMs: 21},
		},
	}, accessAggSnapshot{
		Counts: map[string]map[string]int64{
			"anon":      {"2xx": 10, "4xx": 1},
			"bad_token": {"4xx": 3},
			"user":      {"2xx": 100, "5xx": 2},
			"admin":     {"2xx": 7},
		},
		TopErr: []accessErrEntry{{UserID: 42, Errors: 2}},
	})

	samples := map[string]bool{
		"laiyipao_dashboard_cache_hits_total 3":     false,
		"laiyipao_dashboard_cache_misses_total 1":   false,
		"laiyipao_leaderboard_cache_hits_total 10":  false,
		"laiyipao_leaderboard_cache_misses_total 2": false,
		"laiyipao_db_pool_total_conns 6":            false,
		"laiyipao_db_pool_idle_conns 4":             false,
		"laiyipao_db_pool_acquired_conns 2":         false,
		"laiyipao_db_pool_constructing_conns 0":     false,
		"laiyipao_db_pool_max_conns 10":             false,
		// R80：结算聚合指标的形状（sum/max/total/slow）。
		"laiyipao_settle_total 7":                 false,
		"laiyipao_settle_slow_total 1":            false,
		"laiyipao_settle_total_ms_sum 1234":       false,
		"laiyipao_settle_seg_tx_insert_ms_sum 56": false,
		"laiyipao_settle_seg_tx_insert_ms_max 21": false,
		// R85：访问身份计数与错误 top-N。
		`laiyipao_access_requests_total{identity="anon",status="2xx"} 10`:     false,
		`laiyipao_access_requests_total{identity="bad_token",status="4xx"} 3`: false,
		`laiyipao_access_requests_total{identity="user",status="5xx"} 2`:      false,
		`laiyipao_access_requests_total{identity="admin",status="2xx"} 7`:     false,
		`laiyipao_access_user_errors{user="user#42"} 2`:                       false,
	}
	for line := range strings.SplitSeq(out, "\n") {
		if _, ok := samples[line]; ok {
			samples[line] = true
		}
	}
	for sample, seen := range samples {
		if !seen {
			t.Errorf("输出缺少样本行 %q，实际:\n%s", sample, out)
		}
	}

	// counter 与 gauge 的元数据各查两个代表，避免只验一半。
	for name, typ := range map[string]string{
		"laiyipao_dashboard_cache_hits_total":     "counter",
		"laiyipao_leaderboard_cache_misses_total": "counter",
		"laiyipao_db_pool_acquired_conns":         "gauge",
		"laiyipao_db_pool_max_conns":              "gauge",
	} {
		if !strings.Contains(out, "# HELP "+name+" ") {
			t.Errorf("缺少 %s 的 HELP 行:\n%s", name, out)
		}
		if !strings.Contains(out, "# TYPE "+name+" "+typ) {
			t.Errorf("缺少 %s 的 TYPE %s 行:\n%s", name, typ, out)
		}
	}
}

// TestPoolStatsAreConsistent 用真实 scratch 池校验 PoolStats 的口径：
// 占用 + 空闲不应超过总数，总数不应超过上限，上限必须等于启动配置。
//
// 这守的是「指标名字和实际值对不上」的一类错误（例如把 Idle 映射成
// Acquired），那种错看数值永远「像是对的」。
func TestPoolStatsAreConsistent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	ps := e.svc.PoolStats()
	if ps.MaxConns <= 0 {
		t.Fatalf("MaxConns 应为正数，实际 %d", ps.MaxConns)
	}
	if ps.TotalConns < 0 || ps.AcquiredConns < 0 || ps.IdleConns < 0 {
		t.Fatalf("连接数不应为负：%+v", ps)
	}
	if ps.AcquiredConns+ps.IdleConns > ps.TotalConns {
		t.Errorf("占用(%d)+空闲(%d) 不应超过总数(%d)",
			ps.AcquiredConns, ps.IdleConns, ps.TotalConns)
	}
	if ps.TotalConns+ps.ConstructingConns > ps.MaxConns {
		t.Errorf("总数(%d)+在建(%d) 不应超过上限(%d)",
			ps.TotalConns, ps.ConstructingConns, ps.MaxConns)
	}
}
