// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"fmt"
	"strings"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/service"
)

// renderMetrics 把缓存计数与连接池快照渲染成 Prometheus 文本格式。
//
// 刻意做成**纯函数**：不碰 DB、不碰 fiber.Ctx，于是无 PG 环境也能单测
// 「输出形状是否合法、数值是否对」。若把它塞进 handler 里，就只能起服务
// 才能验，而本机常常连不上库。
//
// 手写而不引 client_golang：只需要几个指标，引一整个注册表/采集器
// 是把「看板缓存命中率」这件事的成本放大了一个数量级。
//
// R50 增加连接池 gauge：池耗尽不会报错，只会让所有接口变慢，
// `acquired / max` 是唯一能提前看出来的信号。
func renderMetrics(st service.CacheStats, ps service.PoolStats) string {
	var b strings.Builder
	emit := func(name, help string, v int64) {
		fmt.Fprintf(&b, "# HELP %s %s\n# TYPE %s counter\n%s %d\n", name, help, name, name, v)
	}
	emit("laiyipao_dashboard_cache_hits_total", "看板缓存命中总数", st.DashboardHits)
	emit("laiyipao_dashboard_cache_misses_total", "看板缓存未命中总数", st.DashboardMisses)
	emit("laiyipao_leaderboard_cache_hits_total", "榜单缓存命中总数", st.LeaderboardHits)
	emit("laiyipao_leaderboard_cache_misses_total", "榜单缓存未命中总数", st.LeaderboardMisses)

	gauge := func(name, help string, v int32) {
		fmt.Fprintf(&b, "# HELP %s %s\n# TYPE %s gauge\n%s %d\n", name, help, name, name, v)
	}
	gauge("laiyipao_db_pool_total_conns", "连接池当前连接总数", ps.TotalConns)
	gauge("laiyipao_db_pool_idle_conns", "连接池空闲连接数", ps.IdleConns)
	gauge("laiyipao_db_pool_acquired_conns", "连接池被请求占用的连接数", ps.AcquiredConns)
	gauge("laiyipao_db_pool_constructing_conns", "连接池正在建立的连接数", ps.ConstructingConns)
	gauge("laiyipao_db_pool_max_conns", "连接池上限（DB_MAX_CONNS）", ps.MaxConns)
	return b.String()
}

// metrics 暴露进程内缓存计数与连接池快照（Prometheus 文本格式）。
//
// 仅在 `METRICS_ENABLED=true` 时注册（见 Register）。内容是非敏感的
// 计数与池容量，但默认不公开，见 config.MetricsEnabled 的理由。
func (s *Server) metrics(c *fiber.Ctx) error {
	c.Set("Content-Type", "text/plain; version=0.0.4; charset=utf-8")
	return c.SendString(renderMetrics(s.Svc.CacheStats(), s.Svc.PoolStats()))
}
