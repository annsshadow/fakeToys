// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 15 轮：OfflineRewardStatus 读路径不再写库。
//
// 此前每次 GET 都跑 data-modifying CTE（INSERT ON CONFLICT DO NOTHING），
// 我的页面每次打开都产生一次写（WAL / 行锁 / 元组版本）。
// ⚠️ 判据用 **xmin**（行版本号）：断言字段没变证明不了「没有写」，
// 这里要的正是「没有产生新行版本」（TestLoadWalletDoesNotWriteWhenNoRegen 同款）。
// 顺带钉住老用户缺行的补行路径仍然可用（首次访问不报 404）。

import (
	"context"
	"testing"
)

func TestOfflineRewardStatusDoesNotWrite(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	xmin := func() string {
		t.Helper()
		var v string
		if err := ts.pool.QueryRow(ctx,
			`SELECT xmin::text FROM offline_rewards WHERE user_id = $1`, uid).Scan(&v); err != nil {
			t.Fatalf("读行版本失败：%v", err)
		}
		return v
	}

	// 预热一次（走正常路径），然后连续读：行版本必须不变。
	if _, err := ts.OfflineRewardStatus(ctx, uid); err != nil {
		t.Fatalf("首次读取失败：%v", err)
	}
	before := xmin()
	for i := 0; i < 3; i++ {
		if _, err := ts.OfflineRewardStatus(ctx, uid); err != nil {
			t.Fatalf("第 %d 次读取失败：%v", i+2, err)
		}
	}
	if after := xmin(); after != before {
		t.Errorf("读端点不应产生新行版本：xmin %s → %s", before, after)
	}
}

func TestOfflineRewardStatusBackfillsMissingRow(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 模拟「迁移上线前注册」的老用户：行不存在。
	ts.exec(t, `DELETE FROM offline_rewards WHERE user_id = $1`, uid)

	st, err := ts.OfflineRewardStatus(ctx, uid)
	if err != nil {
		t.Fatalf("缺行用户首次访问应补行而不是报错：%v", err)
	}
	if st.CapHours != ts.Cfg().OfflineCapHours {
		t.Errorf("CapHours 应来自配置，实际 %v", st.CapHours)
	}
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM offline_rewards WHERE user_id = $1`, uid).Scan(&n); err != nil || n != 1 {
		t.Fatalf("补行应落库（n=%d err=%v）", n, err)
	}
}
