// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
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
// → 无界写）+ 五条无界状态写（loadout/equip/defenses save/challenge）。
//
// 2026-10-30（实质优化第 7 轮）新增 `/me/password`：改密码是口令爆破面（拿到
// access token 后可以无限试旧密码），限流不可删 —— 见 passwordChangeLimit。
//
// 2026-10-30（实质优化第 8 轮）新增 `/me/upgrade`：游客转正是注册面（批量
// 占用户名 / 枚举用户名），限流不可删 —— 见 accountUpgradeLimit。
//
// 2026-10-30（实质优化第 9 轮）新增 `/me/nickname`：昵称进所有列表响应，
// 频繁改名可刷存在感，限流不可删 —— 见 nicknameChangeLimit。
//
// R52（B3 限流轮）：上面这段注释此前写着「资源消耗类端点（buy/claim/signin/...）
// 经济上自限，刻意不限」，而 middleware.go 的 `economyWriteLimit` 注释却写着
// 「买 / 领 / 签 / 洗 / 升 / 熔 / 分配 / 重铸 / 挑战 / 放弃 / 存配装 = 60/分钟」。
// **两份说明相互矛盾，实际实现只覆盖后者的一小半**（挑战/放弃/存配装）。
// 本轮以 `economyWriteLimit` 为准则（它给出了具体数字与理由），把缺失的八条补齐：
// buy / claim / signin / wash / upgrade / synthesize / allocate / reset。
// 判据不是「他们会不会亏」，而是「一个账号能多快把库写爆」——实测 70 次连续购买
// 落 70 行 user_purchases、70 行 wallet_flows，全程无 429。
func TestEconomyWriteRoutesAreRateLimited(t *testing.T) {
	required := []string{
		`battle.Post("/token", authRateLimit(`,
		`battle.Post("/abandon", authRateLimit(`,
		`me.Put("/loadout", authRateLimit(`,
		`me.Put("/equipment", authRateLimit(`,
		`defenses.Post("/save", authRateLimit(`,
		`defenses.Post("/:id/challenge", authRateLimit(`,
		`me.Post("/password", authRateLimit(`,
		`me.Post("/upgrade", authRateLimit(`,
		`me.Put("/nickname", authRateLimit(`,
		// R52：economyWriteLimit 注释里列出、但实现漏掉的八条经济写端点。
		`shop.Post("/:id/buy", authRateLimit(`,
		`tasks.Post("/:id/claim", authRateLimit(`,
		`signin.Post("/", authRateLimit(`,
		`me.Post("/gems/:id/wash", authRateLimit(`,
		`me.Post("/skills/:id/upgrade", authRateLimit(`,
		`me.Post("/skills/synthesize", authRateLimit(`,
		`mastery.Post("/allocate", authRateLimit(`,
		`mastery.Post("/reset", authRateLimit(`,
	}
	src := stripHTTPComments(readHTTPFile(t, "routes.go"))
	for _, want := range required {
		if !strings.Contains(src, want) {
			t.Errorf("写/铸造路由缺少 authRateLimit 限流：%q —— 限流是安全属性，删它必须让这里红", want)
		}
	}
}
