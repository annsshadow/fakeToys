// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"testing"
	"time"
)

// R38：登出轮换游客凭证 + 响应丢失，不能把玩家锁成「新凭证没收到、
// 旧凭证已作废」，更不能让下一次冷启动给他造一个空号。
//
// 修前：Logout 只把 users.guest_token 换成新串，新串仅在响应体里；
// 客户端网络失败时保留旧串、清掉本地令牌。下次 GuestLogin 带着旧串，
// 服务端按「未知凭证 → 建新号」处理 —— 进度全部找不回。
//
// 修后：同一事务把旧串写入 guest_token_aliases（15 分钟），
// GuestLogin 命中别名时找回同一账号，并回传当前（轮换后的）凭证。
func TestGuestLogoutResponseLossRecoversSameAccount(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	tp, err := ts.GuestLogin(ctx, "", "宽限找回")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	uid := tp.User.ID
	oldToken := tp.GuestToken
	if oldToken == "" {
		t.Fatal("游客登录必须回传当前 guest_token（R38 起）")
	}

	usersCount := func() int {
		var n int
		if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM users`).Scan(&n); err != nil {
			t.Fatalf("统计用户失败：%v", err)
		}
		return n
	}
	baseUsers := usersCount()

	// 模拟「请求成功、响应丢了」：服务端已轮换，客户端只看见网络失败。
	res, err := ts.Logout(ctx, uid)
	if err != nil {
		t.Fatalf("登出失败：%v", err)
	}
	if res.GuestToken == "" || res.GuestToken == oldToken {
		t.Fatalf("登出必须轮换 guest_token（旧 %q 新 %q）", oldToken, res.GuestToken)
	}

	// 别名必须指向同一账号，且宽限窗口是未来的时间。
	var aliasUID int64
	var expiresAt time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT user_id, expires_at FROM guest_token_aliases WHERE token = $1`,
		oldToken).Scan(&aliasUID, &expiresAt); err != nil {
		t.Fatalf("登出必须为旧凭证写入宽限期别名：%v", err)
	}
	if aliasUID != uid {
		t.Errorf("别名应指向原账号 %d，实际 %d", uid, aliasUID)
	}
	if !expiresAt.After(time.Now()) {
		t.Errorf("别名到期时间应在未来，实际 %v", expiresAt)
	}

	// 客户端冷启动：用旧凭证登录，必须找回同一个号，而不是建新号。
	tp2, err := ts.GuestLogin(ctx, oldToken, "")
	if err != nil {
		t.Fatalf("旧凭证在宽限期内必须能找回账号，实际：%v", err)
	}
	if tp2.User.ID != uid {
		t.Errorf("宽限期内应找回账号 %d，实际建出 %d（账号被锁死）", uid, tp2.User.ID)
	}
	if got := usersCount(); got != baseUsers {
		t.Errorf("找回不应新增用户：基线 %d，实际 %d", baseUsers, got)
	}
	// 回传的必须是轮换后的当前凭证，客户端据此换掉本地失效的那一枚。
	if tp2.GuestToken != res.GuestToken {
		t.Errorf("找回时应回传当前 guest_token %q，实际 %q", res.GuestToken, tp2.GuestToken)
	}

	// 宽限期不是永久后门：过期后旧凭证不再找回（回到「未知凭证建新号」）。
	ts.exec(t, `UPDATE guest_token_aliases SET expires_at = now() - interval '1 second' WHERE token = $1`, oldToken)
	tp3, err := ts.GuestLogin(ctx, oldToken, "")
	if err != nil {
		t.Fatalf("过期后按新凭证登录应成功：%v", err)
	}
	if tp3.User.ID == uid {
		t.Errorf("别名过期后不应再找回原账号 %d", uid)
	}
	if got := usersCount(); got != baseUsers+1 {
		t.Errorf("过期后应建新号：期望 %d，实际 %d", baseUsers+1, got)
	}
}

// R38 的回滚证据：别名写入失败时，登出的**每一步**都必须回滚。
//
// 「轮换 guest_token」和「吊销 refresh token」都在同一事务里；
// 别名写入是新增的第三步。任何一步失败都不能留下
// 「凭证已换、旧凭证无从找回」或「令牌已吊销、凭证没换」的半成品。
func TestLogoutRollsBackEverythingWhenAliasWriteFails(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	tp, err := ts.GuestLogin(ctx, "", "登出回滚")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	uid := tp.User.ID
	oldToken := tp.GuestToken

	activeTokens := func() int {
		var n int
		if err := ts.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL`,
			uid).Scan(&n); err != nil {
			t.Fatalf("统计有效 token 失败：%v", err)
		}
		return n
	}
	if got := activeTokens(); got != 1 {
		t.Fatalf("建号后应有 1 张有效 token，实际 %d", got)
	}

	// 故障注入：让「写别名」这一步必然失败，前两步照常执行。
	ts.renameTable(t, "guest_token_aliases", "guest_token_aliases_hidden")

	if _, err := ts.Logout(ctx, uid); err == nil {
		t.Fatal("别名写入失败时 Logout 必须报错（不能回成功）")
	}

	// 关键断言：guest_token 未轮换、token 未被吊销 —— 客户端手里的凭证
	// 与令牌都还可用，玩家没有被锁在门外。
	var current string
	if err := ts.pool.QueryRow(ctx,
		`SELECT guest_token FROM users WHERE id = $1`, uid).Scan(&current); err != nil {
		t.Fatalf("回读 guest_token 失败：%v", err)
	}
	if current != oldToken {
		t.Errorf("登出失败后 guest_token 必须保持原值 %q，实际 %q", oldToken, current)
	}
	if got := activeTokens(); got != 1 {
		t.Errorf("登出失败后有效 token 应仍为 1，实际 %d（被吊销了）", got)
	}

	// 还原故障后同一账号可正常登出：旧凭证 → 别名 1 条，令牌全吊销。
	ts.renameTable(t, "guest_token_aliases_hidden", "guest_token_aliases")
	res, err := ts.Logout(ctx, uid)
	if err != nil {
		t.Fatalf("故障解除后登出应成功，实际：%v", err)
	}
	if res.GuestToken == oldToken {
		t.Errorf("登出成功后 guest_token 必须轮换")
	}
	var aliases int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM guest_token_aliases WHERE user_id = $1 AND token = $2`,
		uid, oldToken).Scan(&aliases); err != nil {
		t.Fatalf("统计别名失败：%v", err)
	}
	if aliases != 1 {
		t.Errorf("登出成功后应有 1 条旧凭证别名，实际 %d", aliases)
	}
	if got := activeTokens(); got != 0 {
		t.Errorf("登出成功后有效 token 应为 0，实际 %d", got)
	}
}
