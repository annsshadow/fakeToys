// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R72：登录 / 兑换 / 结算的**全局层**限流行为证据。
//
// 改前实测（探针，单 IP 单账号连打 200 次）：窄层（per-IP / per-user）都在拦
// —— 登录 401×10+429×190、建号 200×60+429×140、兑换 400×5+429×195、
// 结算 200×60+429×10。但 3 账号各打 40 次结算（各自低于单用户额度）=
// **120 次全部 200、0 次 429**，聚合流量没有任何闸。
//
// 这里证明改后：① 全局桶真实存在且跨 IP/跨账号共享；② 超限响应是
// 429 + code=rate_limited + Retry-After: 60（R68 契约）；③ 正常节奏不触发。
//
// 阈值变量是 var：测试在装配路由前改小（个位数即可证明拦截），
// 本包无 t.Parallel，改完用 t.Cleanup 还原，安全。

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/httpapi/httperr"
)

func mustJSON(t *testing.T, body any) io.Reader {
	t.Helper()
	raw, err := json.Marshal(body)
	if err != nil {
		t.Fatalf("编码请求体失败：%v", err)
	}
	return bytes.NewReader(raw)
}

func decodeJSON(t *testing.T, r io.Reader, out *map[string]any) {
	t.Helper()
	if err := json.NewDecoder(r).Decode(out); err != nil {
		*out = map[string]any{"_decode_error": err.Error()}
	}
}

// setGlobalLimitsForTest 把全局阈值改小，装配完 app 后由 Cleanup 还原。
func setGlobalLimitsForTest(t *testing.T, login, guest, redeem, settle int) {
	t.Helper()
	oldL, oldG, oldR, oldS :=
		globalAuthLoginLimit, globalAuthGuestLimit, globalRedeemLimit, globalSettleLimit
	globalAuthLoginLimit, globalAuthGuestLimit, globalRedeemLimit, globalSettleLimit =
		login, guest, redeem, settle
	t.Cleanup(func() {
		globalAuthLoginLimit, globalAuthGuestLimit, globalRedeemLimit, globalSettleLimit =
			oldL, oldG, oldR, oldS
	})
}

// hitRaw 打一次请求并保留响应头（e.do 丢掉了 Retry-After）。
func hitRaw(t *testing.T, app *fiber.App, method, url, token string, body any) (int, string, map[string]any) {
	t.Helper()
	req := httptest.NewRequest(method, url, nil)
	if body != nil {
		req = httptest.NewRequest(method, url, mustJSON(t, body))
		req.Header.Set("Content-Type", "application/json")
	}
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	resp, err := app.Test(req, 10000)
	if err != nil {
		t.Fatalf("%s %s 执行失败：%v", method, url, err)
	}
	defer resp.Body.Close()
	var m map[string]any
	decodeJSON(t, resp.Body, &m)
	return resp.StatusCode, resp.Header.Get("Retry-After"), m
}

// assertRateLimited 断言这是契约里的 429（码 + Retry-After）。
// 响应形状是 {"error":{"code","message"}}（httperr.APIError），码在 error 下。
func assertRateLimited(t *testing.T, where string, st int, ra string, body map[string]any) {
	t.Helper()
	if st != fiber.StatusTooManyRequests {
		t.Fatalf("%s 应 429，实际 %d body=%v", where, st, body)
	}
	errObj, _ := body["error"].(map[string]any)
	if errObj["code"] != httperr.CodeRateLimited {
		t.Errorf("%s error.code=%v，期望 %s（body=%v）", where, errObj["code"], httperr.CodeRateLimited, body)
	}
	if ra != "60" {
		t.Errorf("%s Retry-After=%q，期望 60（限流窗口 1 分钟）", where, ra)
	}
}

// TestGlobalLimiterMechanism：机制本身——常量 key 即全局桶，超限 429 + 契约头。
func TestGlobalLimiterMechanism(t *testing.T) {
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render, DisableStartupMessage: true})
	app.Get("/x", globalLimiter("probe", 3), func(c *fiber.Ctx) error { return c.SendString("ok") })

	for i := 1; i <= 5; i++ {
		st, ra, _ := hitRaw(t, app, http.MethodGet, "/x", "", nil)
		if i <= 3 && st != fiber.StatusOK {
			t.Fatalf("第 %d 次应放行，实际 %d", i, st)
		}
		if i > 3 {
			if st != fiber.StatusTooManyRequests || ra != "60" {
				t.Fatalf("第 %d 次应 429+Retry-After:60，实际 %d RA=%q", i, st, ra)
			}
		}
	}
}

// TestGlobalSettleLimitSpansAccounts：全局结算桶跨账号共享。
//
// 改前同样的打法（3 账号 × 40）拿到 120×200 / 0×429；改后把全局阈值
// 压到 3：两个账号合计只能过 3 次，其余 429，且落库行数恰好 = 放行次数。
func TestGlobalSettleLimitSpansAccounts(t *testing.T) {
	setGlobalLimitsForTest(t, 600, 1200, 120, 3)
	e := newE2E(t)

	uidA, tokA := e.newPlayer(t, "r72ga")
	uidB, tokB := e.newPlayer(t, "r72gb")
	e.exec(t, `UPDATE user_wallets SET energy = 100000 WHERE user_id IN ($1,$2)`, uidA, uidB)

	settle := func(tok string) (int, string, map[string]any) {
		st, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
		if st != 200 {
			t.Fatalf("取结算 token 失败：%d %v", st, body)
		}
		tokenID := int64(body["token_id"].(float64))
		return hitRaw(t, e.app, http.MethodPost, "/api/v1/battle/settle", tok, e.settleBody(t, tokenID))
	}

	// 交替打 5 次：A B A B A —— 谁先谁后不该影响总量。
	var ok, limited int
	for i, tok := range []string{tokA, tokB, tokA, tokB, tokA} {
		st, ra, body := settle(tok)
		switch {
		case st == fiber.StatusOK:
			ok++
		case st == fiber.StatusTooManyRequests:
			limited++
			assertRateLimited(t, fmt.Sprintf("第%d次结算", i+1), st, ra, body)
		default:
			t.Fatalf("第 %d 次结算意外状态 %d body=%v", i+1, st, body)
		}
	}
	if ok != 3 || limited != 2 {
		t.Errorf("放行=%d 429=%d，期望恰好 3 放行（全局阈值）+ 2 拦截", ok, limited)
	}
	rows := e.scalarInt(t, `SELECT COUNT(*) FROM battle_records WHERE user_id IN ($1,$2)`, uidA, uidB)
	if rows != 3 {
		t.Errorf("落库战报 = %d，期望 3（与放行次数一致，429 不多写）", rows)
	}
}

// TestGlobalLoginLimitBelowPerIP：登录全局层在 per-IP（10/分）之下也能先到顶。
// 单 IP 连打 6 次：per-IP 额度没用完，全局阈值 3 先满 —— 证明这层真实存在。
func TestGlobalLoginLimitBelowPerIP(t *testing.T) {
	setGlobalLimitsForTest(t, 3, 1200, 120, 1200)
	e := newE2E(t)

	var unauthorized, limited int
	for i := 1; i <= 6; i++ {
		st, ra, body := hitRaw(t, e.app, http.MethodPost, "/api/v1/auth/login", "",
			map[string]any{"username": "r72_no_such_user", "password": "x"})
		switch st {
		case fiber.StatusUnauthorized:
			unauthorized++
		case fiber.StatusTooManyRequests:
			limited++
			assertRateLimited(t, fmt.Sprintf("第%d次登录", i), st, ra, body)
		default:
			t.Fatalf("第 %d 次登录意外状态 %d body=%v", i, st, body)
		}
	}
	if unauthorized != 3 || limited != 3 {
		t.Errorf("401=%d 429=%d，期望 3×401（全局阈值内）+ 3×429（全局满）", unauthorized, limited)
	}
}

// TestGlobalRedeemLimitSpansAccounts：兑换全局桶跨账号共享。
// 两个账号从同一 IP 各打几次（per-IP 5 没动），全局阈值 3 先满。
func TestGlobalRedeemLimitSpansAccounts(t *testing.T) {
	setGlobalLimitsForTest(t, 600, 1200, 3, 1200)
	e := newE2E(t)
	_, tokA := e.newPlayer(t, "r72ra")
	_, tokB := e.newPlayer(t, "r72rb")

	var bad, limited int
	for i, tok := range []string{tokA, tokB, tokA, tokB, tokA} {
		st, ra, body := hitRaw(t, e.app, http.MethodPost, "/api/v1/redeem", tok,
			map[string]any{"code": "R72-NOPE"})
		switch st {
		case fiber.StatusBadRequest, fiber.StatusNotFound:
			bad++
		case fiber.StatusTooManyRequests:
			limited++
			assertRateLimited(t, fmt.Sprintf("第%d次兑换", i+1), st, ra, body)
		default:
			t.Fatalf("第 %d 次兑换意外状态 %d body=%v", i+1, st, body)
		}
	}
	if bad != 3 || limited != 2 {
		t.Errorf("坏码=%d 429=%d，期望 3×坏码（全局阈值内）+ 2×429（全局满）", bad, limited)
	}
}

// TestNormalPaceNotRateLimited：不误伤证据。
//
// 用**真实默认阈值**（装配前不改任何变量），模拟正常玩家的一分钟：
// 冷启动游客登录（newPlayer 已含）→ 连打 3 局（含网络重试式连点）→
// 兑 2 个码 → 账号登录试 2 次。全部不得出现 429。
//
// 依据的量级：一局战斗最短约 30s（正常 ≤2 次/分），兑换一天 1~2 个码，
// 登录一次冷启动一次 —— 这个节奏离任何一层阈值都差一个数量级以上。
func TestNormalPaceNotRateLimited(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "r72normal")
	e.exec(t, `UPDATE user_wallets SET energy = 100000 WHERE user_id = $1`, uid)

	for i := 1; i <= 3; i++ {
		st, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
		if st != 200 {
			t.Fatalf("第 %d 局取 token 意外状态 %d（不得 429）body=%v", i, st, body)
		}
		tokenID := int64(body["token_id"].(float64))
		st, _, body = hitRaw(t, e.app, http.MethodPost, "/api/v1/battle/settle", tok, e.settleBody(t, tokenID))
		if st == fiber.StatusTooManyRequests {
			t.Fatalf("第 %d 局结算被限流 —— 正常节奏不应触发 429", i)
		}
		if st != fiber.StatusOK {
			t.Fatalf("第 %d 局结算意外状态 %d body=%v", i, st, body)
		}
	}
	for i := 1; i <= 2; i++ {
		st, _, body := hitRaw(t, e.app, http.MethodPost, "/api/v1/redeem", tok,
			map[string]any{"code": fmt.Sprintf("R72-NOPE-%d", i)})
		if st == fiber.StatusTooManyRequests {
			t.Fatalf("第 %d 次兑换被限流 —— 正常节奏不应触发 429", i)
		}
		_ = body
	}
	for i := 1; i <= 2; i++ {
		st, _, _ := hitRaw(t, e.app, http.MethodPost, "/api/v1/auth/login", "",
			map[string]any{"username": "r72_normal_pace", "password": "x"})
		if st == fiber.StatusTooManyRequests {
			t.Fatalf("第 %d 次登录被限流 —— 正常节奏不应触发 429", i)
		}
	}
}
