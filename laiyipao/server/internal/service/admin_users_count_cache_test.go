// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"fmt"
	"testing"
	"time"
)

// TestAdminUsersTotalCacheBoundedStaleness 守的是 R61 的核心承诺：
// 后台玩家列表的 total **可以旧几秒，但不会变成假数、也不会长期漂移**。
//
// 判据分四步（每一步都拿 `COUNT(*)` 真值对比）：
//  1. 首次查询：缓存值与真值一致；
//  2. 新增一个用户后立刻再查：允许仍返回旧值（证明命中缓存，而不是每次重算）；
//  3. `ADMIN_USERS_COUNT_CACHE_TTL=0`：必须返回真值（运维排查口径）；
//  4. TTL 到期后再查：必须重算到新值（证明不会长期漂移）。
func TestAdminUsersTotalCacheBoundedStaleness(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	tag := fmt.Sprintf("r61cache_%d", time.Now().UnixNano())
	insert := func(nick string) {
		t.Helper()
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO users (guest_token, nickname) VALUES ($1, $2)`,
			fmt.Sprintf("r61_%s_%d", nick, time.Now().UnixNano()), tag+"_"+nick); err != nil {
			t.Fatalf("插入用户失败：%v", err)
		}
	}
	fresh := func() int64 {
		t.Helper()
		var n int64
		if err := ts.db.Pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM users WHERE nickname LIKE $1`, tag+"%").Scan(&n); err != nil {
			t.Fatalf("读取真值失败：%v", err)
		}
		return n
	}
	t.Cleanup(func() {
		_, _ = ts.db.Pool.Exec(context.Background(), `DELETE FROM users WHERE nickname LIKE $1`, tag+"%")
	})

	insert("a")
	insert("b")

	// TTL 设得足够长，保证第 2 步一定命中缓存（不依赖机器快慢）。
	ts.Cfg().AdminUsersCountCacheTTL = 10 * time.Second
	call := func() int64 {
		t.Helper()
		_, total, err := ts.AdminListUsers(ctx, tag, 10, 0, nil, nil, nil, nil, nil, nil)
		if err != nil {
			t.Fatalf("AdminListUsers 失败：%v", err)
		}
		return total
	}

	// 1) 首次：缓存值 = 真值。
	if got, want := call(), fresh(); got != want {
		t.Fatalf("首次 total=%d，真值=%d", got, want)
	}

	// 2) 新增后立刻再查：仍返回旧值 2（缓存生效），而真值已是 3。
	insert("c")
	if got := call(); got != 2 {
		t.Fatalf("TTL 内 total=%d，期望仍为旧值 2（若为 3 说明缓存没生效）", got)
	}
	if truth := fresh(); truth != 3 {
		t.Fatalf("真值=%d，期望 3", truth)
	}

	// 3) 关闭缓存：必须每次真值。
	ts.Cfg().AdminUsersCountCacheTTL = 0
	if got, want := call(), fresh(); got != want {
		t.Fatalf("关闭缓存后 total=%d，真值=%d", got, want)
	}

	// 4) 重新开启短 TTL：过期后必须重算到新值，证明不会长期漂移。
	insert("d")
	ts.Cfg().AdminUsersCountCacheTTL = 50 * time.Millisecond
	time.Sleep(150 * time.Millisecond)
	if got, want := call(), fresh(); got != want || want != 4 {
		t.Fatalf("TTL 到期后 total=%d，真值=%d（期望 4）", got, want)
	}
}
