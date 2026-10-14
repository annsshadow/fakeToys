package httpapi

// 本轮安全审计的 HTTP 层守卫。
//
// 判据统一是「**改回去会不会红**」，不是「覆盖到了吗」。

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

// TestSecurityHeadersAreSentOnEveryResponse 是点击劫持那条的守卫。
//
// # 为什么它必须是「每条响应」而不是「admin 页面」
//
// 后台的写操作全是单次点击即生效（发币 / 封号 / 改价 / 再生成 100 关），
// 没有二次确认弹窗。缺了这两个头时，攻击者可以：
//
//	evil.com → <iframe src="https://后台域名/#/admin/users">
//	         → 盖透明遮罩，把假按钮对齐到 iframe 内的真按钮
//
// 已登录运营点一下假按钮，请求就**真的**带上了他自己 localStorage 里的
// `Authorization` 头发出去 —— 跨域读不到 token 也不影响，
// 因为执行的是他自己页面里的代码。结果落进审计日志，
// 看起来与运营亲手点的完全一样。
//
// 这是当前**唯一一条**「不需要任何服务端漏洞、只要运营点一下就能在后台
// 执行写操作」的路径。
//
// # 为什么 X-Frame-Options 与 frame-ancestors 两个都要断言
//
// `frame-ancestors` 只在 CSP2+ 生效，`X-Frame-Options` 是老浏览器的唯一手段。
// 少任何一个就有一批浏览器失去防护 —— 而只断言其中一个的测试会绿。
func TestSecurityHeadersAreSentOnEveryResponse(t *testing.T) {
	e := newE2E(t)
	for _, path := range []string{"/healthz", "/readyz", "/api/v1/config", "/api/v1/leaderboard"} {
		resp, err := e.app.Test(httptest.NewRequest(http.MethodGet, path, nil), 10000)
		if err != nil {
			t.Fatalf("%s 请求失败：%v", path, err)
		}
		if got := resp.Header.Get("X-Frame-Options"); got != "DENY" {
			t.Errorf("%s 的 X-Frame-Options = %q，期望 DENY", path, got)
		}
		if got := resp.Header.Get("Content-Security-Policy"); !strings.Contains(got, "frame-ancestors 'none'") {
			t.Errorf("%s 的 CSP 缺 frame-ancestors 'none'，实际 = %q", path, got)
		}
		if got := resp.Header.Get("X-Content-Type-Options"); got != "nosniff" {
			t.Errorf("%s 的 X-Content-Type-Options = %q，期望 nosniff", path, got)
		}
		if got := resp.Header.Get("Referrer-Policy"); got != "no-referrer" {
			t.Errorf("%s 的 Referrer-Policy = %q，期望 no-referrer", path, got)
		}
	}
}

// TestSecurityHeadersAreSentOnErrorResponses 守住错误路径。
//
// 为什么必须单独一条：把 `Set` 挪到 handler **之后**的实现也能让上一条变绿，
// 而那样恰好是最危险的形态 —— 出错时页面最容易被 iframe 套。
func TestSecurityHeadersAreSentOnErrorResponses(t *testing.T) {
	e := newE2E(t)
	resp, err := e.app.Test(httptest.NewRequest(http.MethodGet, "/api/v1/nope", nil), 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	if resp.StatusCode == 200 {
		t.Fatalf("未知路径应非 200，实际 = %d", resp.StatusCode)
	}
	if got := resp.Header.Get("X-Frame-Options"); got != "DENY" {
		t.Errorf("错误响应也必须带 X-Frame-Options，实际 = %q", got)
	}
	if got := resp.Header.Get("Content-Security-Policy"); !strings.Contains(got, "frame-ancestors 'none'") {
		t.Errorf("错误响应也必须带 frame-ancestors，实际 = %q", got)
	}
}

// TestSecurityHeadersOnPreflight 断言预检（OPTIONS）也带。
//
// 这一条是防「限流中间件提前短路 OPTIONS」的实现：
// CORS 预检被挡掉的表现是「整批请求发不出去」，
// 很难与网络故障区分 —— 本项目踩过一次同型的坑
// （加一个自定义请求头就得同步改预检白名单，忘改的症状是整批请求发不出去）。
func TestSecurityHeadersOnPreflight(t *testing.T) {
	e := newE2E(t)
	req := httptest.NewRequest(http.MethodOptions, "/api/v1/config", nil)
	req.Header.Set("Origin", "https://admin.example.com")
	req.Header.Set("Access-Control-Request-Method", "GET")
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("预检失败：%v", err)
	}
	if got := resp.Header.Get("X-Frame-Options"); got != "DENY" {
		t.Errorf("预检响应缺 X-Frame-Options，实际 = %q", got)
	}
}

// TestRateLimitedAuthEndpointsAreActuallyLimited 是限流那条的守卫。
//
// 判据是**实测请求序列**，不是「中间件被挂上了」：
// 一个挂了 limiter 但 `Max` 写成 100000 的实现能让「挂上了」这个断言绿，
// 却挡不住任何东西。所以这里真的打第 N+1 次，断言它被拒。
//
// ⚠️ 关于「按 IP 计」这个前提：`app.Test` 把所有请求放在同一个远端地址上，
// 所以这里不需要（也不应该）去伪造不同 IP —— 全部落在同一个桶里
// 正是「按 IP 限流」应有的表现。
func TestRateLimitedAuthEndpointsAreActuallyLimited(t *testing.T) {
	cases := []struct {
		name  string
		limit int
		path  string
		body  string
	}{
		// 后台登录：全站**最贵的一次未认证请求**（bcrypt cost 10，
		// 单次约 50~100ms 纯 CPU）。无限流时未认证攻击者打满并发
		// 就能让正常运营完全登不进后台，同时顺便无限次口令喷洒。
		{"admin/login", adminLoginLimit, "/api/v1/admin/login",
			`{"username":"nobody","password":"x"}`},
		// 游客建号：无需任何凭证，一次调用写 7 行库。
		{"auth/guest", guestLoginLimit, "/api/v1/auth/guest", `{"guest_token":""}`},
		// 刷新令牌：长期凭证（默认 30 天）的续命口。
		{"auth/refresh", refreshLimit, "/api/v1/auth/refresh", `{"refresh_token":"x"}`},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			// 1) 额度内的首次请求**不得**被限流
			//    （这条是「限流不能把正常玩家挡在门外」的可执行版本）
			fresh := newE2E(t)
			if code := postOnce(fresh.app, tc.path, tc.body); code == http.StatusTooManyRequests {
				t.Fatalf("%s 第一次请求就被限流 —— 额度判定写错了", tc.name)
			}

			// 2) 连打 limit+5 次必须被限住，且**持续**被拒
			//    （「持续」这一半是防「随机限流」的实现）
			e := newE2E(t)
			limited := 0
			for i := 0; i < tc.limit+5; i++ {
				if postOnce(e.app, tc.path, tc.body) == http.StatusTooManyRequests {
					limited++
				}
			}
			if limited == 0 {
				t.Errorf("%s 连打 %d 次都没被限流 —— 限流没有生效", tc.name, tc.limit+5)
			}
			if limited != 5 {
				t.Errorf("%s 应恰好在第 %d 次起被拒（连打 %d 次应拒 %d 次），实际拒了 %d 次",
					tc.name, tc.limit+1, tc.limit+5, 5, limited)
			}
		})
	}
}

// postOnce 打一个 POST 并返回状态码。
func postOnce(app interface {
	Test(*http.Request, ...int) (*http.Response, error)
}, path, body string) int {
	req := httptest.NewRequest(http.MethodPost, path, strings.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	resp, err := app.Test(req, 10000)
	if err != nil {
		return 0
	}
	return resp.StatusCode
}

// TestRateLimiterDoesNotBlockPublicReads 守住「限流不能误伤」。
//
// ⚠️ 这一条与「限流生效」同等重要：一次把 `/config` 或 `/leaderboard`
// 也限住的实现，能让上面几条全绿，却让正常玩家**用不了游戏**。
//
// 判据是「这些端点不在任何限流中间件后面」——
// 通过连打远高于任何额度的次数来证明。
func TestRateLimiterDoesNotBlockPublicReads(t *testing.T) {
	e := newE2E(t)
	for _, path := range []string{"/api/v1/config", "/api/v1/leaderboard", "/healthz"} {
		for i := 0; i < 80; i++ {
			resp, err := e.app.Test(httptest.NewRequest(http.MethodGet, path, nil), 10000)
			if err != nil {
				t.Fatalf("%s 第 %d 次请求失败：%v", path, i+1, err)
			}
			if resp.StatusCode == http.StatusTooManyRequests {
				t.Fatalf("%s 第 %d 次请求被限流 —— 公开读端点不应受限", path, i+1)
			}
		}
	}
}
