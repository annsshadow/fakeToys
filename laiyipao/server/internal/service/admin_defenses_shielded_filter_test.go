// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 后台防线列表按护盾状态筛选的测试（2026-11-01 新一轮第 100 轮）。
//
// 有护盾 = shielded_until 非空且未过期；无护盾 = 空或已过期。
// 三态参数：nil 不过滤。

import (
	"context"
	"testing"
	"time"
)

func TestAdminListDefensesShieldedFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库数据污染
	ctx := context.Background()

	// 每个用户最多一条防线（idx_defenses_owner 唯一），用三个用户。
	uidA, _ := newGuestUser(t, ts, "def_shield_a")
	uidB, _ := newGuestUser(t, ts, "def_shield_b")
	uidC, _ := newGuestUser(t, ts, "def_shield_c")
	activeID := seedDefense(t, ts, uidA, "护盾防线")
	_ = seedDefense(t, ts, uidB, "无护盾防线")
	expiredID := seedDefense(t, ts, uidC, "过期防线")
	now := time.Now()
	if _, err := ts.db.Pool.Exec(ctx,
		`UPDATE defenses SET shielded_until = $2 WHERE id = $1`, activeID, now.Add(24*time.Hour)); err != nil {
		t.Fatalf("设置有效护盾失败：%v", err)
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`UPDATE defenses SET shielded_until = $2 WHERE id = $1`, expiredID, now.Add(-time.Hour)); err != nil {
		t.Fatalf("设置过期护盾失败：%v", err)
	}

	// shielded=true：只含有护盾的防线
	items, total, err := ts.AdminListDefenses(ctx, 50, 0, "", 0, boolPtr(true))
	if err != nil {
		t.Fatalf("shielded=true 查询失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].ID != activeID {
		t.Errorf("shielded=true 应只含有护盾防线，实际 total=%d items=%+v", total, items)
	}

	// shielded=false：含无护盾与过期防线（2 条）
	items, total, err = ts.AdminListDefenses(ctx, 50, 0, "", 0, boolPtr(false))
	if err != nil {
		t.Fatalf("shielded=false 查询失败：%v", err)
	}
	if total != 2 {
		t.Errorf("shielded=false 应有 2 条（无护盾+过期），实际 %d", total)
	}
	for _, d := range items {
		if d.ID == activeID {
			t.Error("shielded=false 不应包含护盾有效的防线")
		}
	}

	// shielded=nil：全部 3 条
	items, total, err = ts.AdminListDefenses(ctx, 50, 0, "", 0, nil)
	if err != nil {
		t.Fatalf("shielded=nil 查询失败：%v", err)
	}
	if total != 3 {
		t.Errorf("shielded=nil 应有 3 条，实际 %d", total)
	}
}
