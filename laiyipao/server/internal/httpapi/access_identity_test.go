// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// access_identity_test.go — R85 访问日志身份标识的行为测试。
//
//  1. 三态分类：user#N / bad_token / anon；
//  2. 隐私边界：带 JWT、带 guest_token、带昵称（含纯数字昵称）的请求，
//     访问日志绝不出现这些原值 —— 只允许 user#N 这一种脱敏形态；
//  3. 运营聚合：身份类 × 状态段计数、错误 top-N、键数上限清空。

import (
	"fmt"
	"net/http"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/laiyipao/server/internal/httpapi/httperr"
)

// TestAccessIdentityClassification：三态判据（走真实中间件链）。
func TestAccessIdentityClassification(t *testing.T) {
	SetLogLevel("debug")
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
	app.Use(traceMiddleware())
	app.Use(requestLogger())
	// 模拟认证后路由：Locals 写入 user_id（requireUser 的效果）。
	app.Get("/authed", func(c *fiber.Ctx) error {
		c.Locals("user_id", int64(42))
		return c.SendString("ok")
	})
	// 模拟管理端路由：Locals 写入 admin_id（requireAdmin 的效果）。
	app.Get("/admined", func(c *fiber.Ctx) error {
		c.Locals("admin_id", int64(7))
		return c.SendString("ok")
	})
	app.Get("/pub", func(c *fiber.Ctx) error { return c.SendString("ok") })
	app.Get("/nf", func(c *fiber.Ctx) error {
		return httperr.New(fiber.StatusNotFound, httperr.CodeNotFound, "没有")
	})
	read := captureAccessLog(t)

	do := func(path, auth string) {
		req, _ := http.NewRequest(http.MethodGet, path, nil)
		if auth != "" {
			req.Header.Set("Authorization", auth)
		}
		resp, err := app.Test(req, 10000)
		if err != nil {
			t.Fatalf("请求失败：%v", err)
		}
		resp.Body.Close()
	}
	do("/authed", "Bearer valid-but-unparsed")  // Locals 由路由写入 → user#42
	do("/admined", "Bearer admin-token-string") // admin_id 写入 → admin#7
	do("/pub", "")                              // 无凭证 → anon
	do("/nf", "Bearer garbage-token-xyz")       // 坏凭证 → bad_token

	logged := read()
	if !strings.Contains(logged, "id=user#42 ") {
		t.Errorf("认证请求应记 id=user#42：%q", logged)
	}
	if !strings.Contains(logged, "id=admin#7 ") {
		t.Errorf("管理端请求应记 id=admin#7（R86 实抓：此前误标 bad_token）：%q", logged)
	}
	if !strings.Contains(logged, "id=anon ") {
		t.Errorf("无凭证请求应记 id=anon：%q", logged)
	}
	if !strings.Contains(logged, "id=bad_token ") {
		t.Errorf("坏凭证请求应记 id=bad_token：%q", logged)
	}
	// 坏令牌的内容绝不能出现在日志里。
	if strings.Contains(logged, "garbage-token-xyz") {
		t.Errorf("日志含令牌原值：%q", logged)
	}
}

// TestAccessLogPrivacyBoundary：真实 e2e 下的脱敏边界。
//
// 覆盖要求 3 的三种输入：JWT（Authorization）、guest_token（建号请求体）、
// 昵称（含**纯数字昵称**——它最容易被误读成 user# 字段）。
func TestAccessLogPrivacyBoundary(t *testing.T) {
	e := newE2E(t)
	// ⚠️ 顺序敏感：newE2E 装配时 Register 会用 config.LogLevel（默认 info）
	// 覆盖档位 —— debug 必须在装配**之后**设，否则 200 的访问行不可见。
	SetLogLevel("debug")
	t.Cleanup(func() { SetLogLevel("info") })
	read := captureAccessLog(t)

	// 建号请求体里带 guest_token 与昵称（访问日志绝不打 body —— 钉住）。
	guestTok := fmt.Sprintf("e2e_r85priv_%d", 12345)
	uid, token := e.loginGuest(t, guestTok, "12345") // 昵称就是纯数字

	// 认证请求：访问日志应是 id=user#<uid>。
	st, _ := e.get(t, "/api/v1/me", token)
	if st != 200 {
		t.Fatalf("/me status=%d", st)
	}
	// 坏 token 请求：id=bad_token，且令牌内容不落日志。
	badTok := "r85-badtoken-secret"
	req, _ := http.NewRequest(http.MethodGet, "/api/v1/me", nil)
	req.Header.Set("Authorization", "Bearer "+badTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("坏 token 请求失败：%v", err)
	}
	resp.Body.Close()

	logged := read()
	// 允许出现的唯一身份形态。
	if !strings.Contains(logged, fmt.Sprintf("id=user#%d ", uid)) {
		t.Errorf("访问日志应含 id=user#%d：%q", uid, logged)
	}
	if !strings.Contains(logged, "id=bad_token ") {
		t.Errorf("坏 token 请求应记 id=bad_token：%q", logged)
	}
	// 绝不出现的原值。
	for _, leaked := range []string{guestTok, token, badTok} {
		if leaked != "" && strings.Contains(logged, leaked) {
			t.Errorf("访问日志泄漏原值 %q：%q", leaked[:min(len(leaked), 16)]+"...", logged)
		}
	}
	// 纯数字昵称的反混淆：不得出现「id=12345」（裸数字被当成身份字段）。
	if strings.Contains(logged, "id=12345") {
		t.Errorf("纯数字昵称混进了身份字段：%q", logged)
	}
}

// TestAccessAggCountsAndTopN：聚合实例的计数、top-N 与键数上限。
func TestAccessAggCountsAndTopN(t *testing.T) {
	a := newAccessAgg()
	a.recordAccess("user#1", 200, 1)
	a.recordAccess("user#1", 500, 1)
	a.recordAccess("user#2", 403, 2)
	a.recordAccess("bad_token", 401, 0)
	a.recordAccess("anon", 200, 0)

	snap := a.snapshot()
	if got := snap.Counts["user#1"]["2xx"]; got != 1 {
		t.Errorf("user#1 2xx=%d，应 1", got)
	}
	if got := snap.Counts["user#1"]["5xx"]; got != 1 {
		t.Errorf("user#1 5xx=%d，应 1", got)
	}
	if got := snap.Counts["bad_token"]["4xx"]; got != 1 {
		t.Errorf("bad_token 4xx=%d，应 1", got)
	}
	// top-N：user#2 的 403 计入、anon/bad_token 不计（uid=0）。
	if len(snap.TopErr) != 2 {
		t.Fatalf("top 应有 2 条（user#1/user#2），实际 %d: %+v", len(snap.TopErr), snap.TopErr)
	}
	if snap.TopErr[0].UserID != 2 && snap.TopErr[1].UserID != 2 {
		t.Errorf("top 缺 user#2: %+v", snap.TopErr)
	}

	// 键数上限：灌满后整表清空（防无界增长）。
	for i := int64(100); i < 100+accessErrTopCap+5; i++ {
		a.recordAccess(fmt.Sprintf("user#%d", i), 500, i)
	}
	snap2 := a.snapshot()
	if len(snap2.TopErr) > 10 {
		t.Errorf("top 渲染只留前 10，实际 %d", len(snap2.TopErr))
	}
	// 清空过至少一次：errByUID 的实际键数应远小于灌入数。
	a.mu.Lock()
	keys := len(a.errByUID)
	a.mu.Unlock()
	if keys > accessErrTopCap {
		t.Errorf("errByUID 键数 %d 超过上限 %d", keys, accessErrTopCap)
	}
}
