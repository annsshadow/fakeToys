// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"fmt"
	"testing"
)

// R33：刷新令牌的「吊销旧 token」与「写入新 token」必须同生同死。
//
// 修前是两条独立 autocommit：先 UPDATE 吊销旧 token，再 INSERT 新 token。
// 第二条失败（临时故障、唯一约束、连接中断）时旧 token 已经作废、新 token
// 也没发出去 —— 客户端手里只剩一枚已经无效的 refresh token，
// 用户被强制重新登录，而且这个「失败」发生在服务端一次瞬时错误之后。
//
// 判据用故障注入（scratch 库专用触发器，只拦 INSERT、放行 UPDATE）：
//   - 写新 token 失败时 Refresh 必须报错；
//   - 旧 token 必须仍然有效（有效 token 数回到基线 1）；
//   - 故障解除后，同一枚旧 token 必须还能换出新令牌。
func TestRefreshRotationIsAtomic(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	tp, err := ts.GuestLogin(ctx, "", "刷新原子性")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	uid := tp.User.ID
	oldRefresh := tp.RefreshToken
	if oldRefresh == "" {
		t.Fatal("GuestLogin 必须返回 refresh token")
	}

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

	// 故障注入：只让「写新 token」这一步失败；吊销旧 token 的 UPDATE 正常执行。
	ts.exec(t, `CREATE OR REPLACE FUNCTION reject_new_refresh() RETURNS trigger AS $$
		BEGIN
			RAISE EXCEPTION 'injected refresh insert failure';
		END;
	$$ LANGUAGE plpgsql`)
	ts.exec(t, fmt.Sprintf(`CREATE TRIGGER reject_new_refresh_trg BEFORE INSERT ON refresh_tokens
		FOR EACH ROW WHEN (NEW.user_id = %d) EXECUTE FUNCTION reject_new_refresh()`, uid))

	if _, err := ts.Refresh(ctx, oldRefresh); err == nil {
		t.Fatal("写新 token 失败时 Refresh 必须报错（不能只吊销旧 token 就回成功）")
	}
	// 关键断言：旧 token 的吊销必须随整笔事务回滚。
	if got := activeTokens(); got != 1 {
		t.Errorf("注入失败后有效 token 应回到基线 1，实际 %d（旧 token 被吊销了）", got)
	}

	ts.exec(t, `DROP TRIGGER reject_new_refresh_trg ON refresh_tokens`)

	// 故障解除后，同一枚旧 token 必须还能完成轮换。
	if _, err := ts.Refresh(ctx, oldRefresh); err != nil {
		t.Fatalf("注入失败后旧 refresh token 必须仍可换新令牌，实际：%v", err)
	}
	if got := activeTokens(); got != 1 {
		t.Errorf("成功轮换后应仍是 1 张有效 token（旧吊销 + 新签发），实际 %d", got)
	}
}
