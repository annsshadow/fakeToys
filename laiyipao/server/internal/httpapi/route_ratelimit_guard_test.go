package httpapi

import (
	"strings"
	"testing"
)

// TestEconomyWriteRoutesAreRateLimited 锁住「写/铸造端点必须带限流」。
//
// # 为什么用源码扫描而不是行为测试
//
// 限流是**安全属性**：把某条路由的 `authRateLimit` 删掉，编译照过、现有测试
// 全绿，而「一个账号能打多猛」的顶悄悄没了 —— 这正是本仓库反复要消灭的
// 「看起来在防、其实没防」的静默失效。行为测试要真打满限额（需起服务+计时），
// 本机跑不起；源码扫描把它变成**结构约束**：谁删限流，这里立刻红。
//
// 覆盖的是 2026-10-23（LY-3）接上限流的六条：`/battle/token`（铸造，体力可再生
// → 无界写）+ 五条无界状态写（loadout/equip/defenses save/challenge）。资源消耗
// 类端点（buy/claim/signin/...）花的是有限持有物、经济上自限，刻意不限 —— 见
// middleware.go 的 economyWriteLimit 注释。
func TestEconomyWriteRoutesAreRateLimited(t *testing.T) {
	required := []string{
		`battle.Post("/token", authRateLimit(`,
		`battle.Post("/abandon", authRateLimit(`,
		`me.Put("/loadout", authRateLimit(`,
		`me.Put("/equipment", authRateLimit(`,
		`defenses.Post("/save", authRateLimit(`,
		`defenses.Post("/:id/challenge", authRateLimit(`,
	}
	src := stripHTTPComments(readHTTPFile(t, "routes.go"))
	for _, want := range required {
		if !strings.Contains(src, want) {
			t.Errorf("写/铸造路由缺少 authRateLimit 限流：%q —— 限流是安全属性，删它必须让这里红", want)
		}
	}
}
