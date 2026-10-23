package httpapi

import (
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/service"
)

// TestRenderCacheMetrics：/metrics 输出形状与数值。
//
// 判据：Prometheus 文本格式要求每个指标都有 `# HELP` 与 `# TYPE ... counter`
// 两行元数据，且样本行是 `name value`。缺元数据的输出多数抓取端会拒收
// 或告警 —— 那是指标「看起来有、实际抓不到」的静默失效，与本仓库反复
// 要消灭的形态同源。纯函数、无 DB，是本机真正能跑的指标判据。
func TestRenderCacheMetrics(t *testing.T) {
	st := service.CacheStats{
		DashboardHits: 3, DashboardMisses: 1,
		LeaderboardHits: 10, LeaderboardMisses: 2,
	}
	out := renderCacheMetrics(st)

	samples := map[string]bool{
		"laiyipao_dashboard_cache_hits_total 3":     false,
		"laiyipao_dashboard_cache_misses_total 1":   false,
		"laiyipao_leaderboard_cache_hits_total 10":  false,
		"laiyipao_leaderboard_cache_misses_total 2": false,
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

	// 每个指标都要有 HELP 与 TYPE 两行元数据
	for _, name := range []string{
		"laiyipao_dashboard_cache_hits_total",
		"laiyipao_leaderboard_cache_misses_total",
	} {
		if !strings.Contains(out, "# HELP "+name+" ") {
			t.Errorf("缺少 %s 的 HELP 行:\n%s", name, out)
		}
		if !strings.Contains(out, "# TYPE "+name+" counter") {
			t.Errorf("缺少 %s 的 TYPE 行:\n%s", name, out)
		}
	}
}
