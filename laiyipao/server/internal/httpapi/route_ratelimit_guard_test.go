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

// TestGlobalRateLimitWired 锁住 R72 的全局限流层（登录/建号/兑换/结算）。
//
// 全局层同样是**安全属性**：删掉某条路由的 globalLimiter，编译照过、
// 窄层测试全绿，而「分布式刷量无总闸」这个缺口就悄悄回来了
// （改前实测：3 账号 × 40 次结算 = 120×200、0×429）。
// 行为证据在 route_global_ratelimit_test.go；这里把它变成结构约束。
//
// 每个模式都编码了**顺序**：窄层（per-IP/per-user）在前，全局层在后 ——
// 反过来的话一个滥用 IP 会把全站共享预算吃完（见 globalLimiter 注释）。
func TestGlobalRateLimitWired(t *testing.T) {
	src := readHTTPFile(t, "routes.go")
	// 压平空白：路由装配可能折行，模式匹配不该依赖排版。
	flat := strings.Join(strings.Fields(stripHTTPComments(src)), " ")

	required := []string{
		`authAPI.Post("/guest", perIPLimiter("guest", guestLoginLimit), globalLimiter("auth_guest", globalAuthGuestLimit), s.guestLogin)`,
		`authAPI.Post("/wechat", perIPLimiter("wechat", guestLoginLimit), globalLimiter("auth_wechat", globalAuthGuestLimit), s.wechatLogin)`,
		`authAPI.Post("/login", perIPLimiter("playerlogin", playerLoginLimit), globalLimiter("auth_login", globalAuthLoginLimit), s.playerLogin)`,
		`battle.Post("/settle", authRateLimit("settle", settleLimit), globalLimiter("settle", globalSettleLimit), s.settleBattle)`,
		`redeem.Post("/", perIPLimiter("redeem", redeemLimit), authRateLimit("redeem", perUserRedeem), globalLimiter("redeem", globalRedeemLimit), s.redeem)`,
	}
	for _, want := range required {
		if !strings.Contains(flat, want) {
			t.Errorf("路由缺少全局层限流（或窄层/全局层顺序被颠倒）：%q —— 分布式刷量将无总闸", want)
		}
	}
	if n := strings.Count(flat, "globalLimiter("); n < 5 {
		t.Errorf("routes.go 里 globalLimiter 只有 %d 处（应 ≥5）—— 守卫可能扫错文件或全局层被拆走", n)
	}
}
