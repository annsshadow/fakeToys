package httpapi

import (
	"fmt"
	"strings"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/service"
)

// renderCacheMetrics 把缓存计数渲染成 Prometheus 文本格式（v0.0.4）。
//
// 刻意做成**纯函数**：不碰 DB、不碰 fiber.Ctx，于是无 PG 环境也能单测
// 「输出形状是否合法、数值是否对」。若把它塞进 handler 里，就只能起服务
// 才能验，而本机常常连不上库。
//
// 手写而不引 client_golang：只需要 4 个 counter，引一整个注册表/采集器
// 是把「看板缓存命中率」这件事的成本放大了一个数量级。
func renderCacheMetrics(st service.CacheStats) string {
	var b strings.Builder
	emit := func(name, help string, v int64) {
		fmt.Fprintf(&b, "# HELP %s %s\n# TYPE %s counter\n%s %d\n", name, help, name, name, v)
	}
	emit("laiyipao_dashboard_cache_hits_total", "看板缓存命中总数", st.DashboardHits)
	emit("laiyipao_dashboard_cache_misses_total", "看板缓存未命中总数", st.DashboardMisses)
	emit("laiyipao_leaderboard_cache_hits_total", "榜单缓存命中总数", st.LeaderboardHits)
	emit("laiyipao_leaderboard_cache_misses_total", "榜单缓存未命中总数", st.LeaderboardMisses)
	return b.String()
}

// metrics 暴露进程内缓存计数（Prometheus 文本格式）。
//
// 仅在 `METRICS_ENABLED=true` 时注册（见 Register）。内容是命中/未命中
// 计数，非敏感；但默认不公开，见 config.MetricsEnabled 的理由。
func (s *Server) metrics(c *fiber.Ctx) error {
	c.Set("Content-Type", "text/plain; version=0.0.4; charset=utf-8")
	return c.SendString(renderCacheMetrics(s.Svc.CacheStats()))
}
