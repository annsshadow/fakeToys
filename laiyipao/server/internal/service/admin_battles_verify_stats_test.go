// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 13 轮：AdminListBattles 普通翻页改两阶段聚合（先页、后按页内 battle_id
// 回填报数），COUNT 摘掉只在 onlyMismatched 才需要的聚合 JOIN。
//
// 本测试钉住两阶段回填的正确性：同一局验真两次（匹配/不匹配）→
// 普通列表的 checked/mismatched 必须精确；onlyMismatched 路径
// （单查询 JOIN）也顺带验证语义不变。

import (
	"context"
	"testing"
)

func TestAdminListBattlesVerifyStatsFilled(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	in := loseSettleInput()
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	var battleID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).Scan(&battleID); err != nil {
		t.Fatalf("读战报失败：%v", err)
	}

	// 同一局验真两次：一次匹配、一次不匹配。
	if _, err := ts.VerifyReplay(ctx, uid, battleID, in.ReplayHash); err != nil {
		t.Fatalf("匹配验真失败：%v", err)
	}
	if _, err := ts.VerifyReplay(ctx, uid, battleID, "ffffffffffffffff"); err != nil {
		t.Fatalf("不匹配验真失败：%v", err)
	}

	// 普通翻页（两阶段回填路径）。
	rows, _, err := ts.AdminListBattles(ctx, 0, 0, 50, false, 0, nil, nil, "")
	if err != nil {
		t.Fatalf("列表失败：%v", err)
	}
	found := false
	for _, b := range rows {
		if b.ID != battleID {
			continue
		}
		found = true
		if b.VerifyChecked != 2 {
			t.Errorf("验真次数应为 2，实际 %d", b.VerifyChecked)
		}
		if b.VerifyMismatched != 1 {
			t.Errorf("不匹配次数应为 1，实际 %d", b.VerifyMismatched)
		}
	}
	if !found {
		t.Fatalf("列表结果里没有刚结算的战报 %d（共 %d 行）", battleID, len(rows))
	}

	// onlyMismatched（单查询 JOIN 路径）：该局有一次不匹配，必须命中。
	mis, _, err := ts.AdminListBattles(ctx, 0, 0, 50, true, 0, nil, nil, "")
	if err != nil {
		t.Fatalf("onlyMismatched 列表失败：%v", err)
	}
	hit := false
	for _, b := range mis {
		if b.ID == battleID {
			hit = true
			if b.VerifyMismatched != 1 {
				t.Errorf("onlyMismatched 路径不匹配计数应为 1，实际 %d", b.VerifyMismatched)
			}
		}
	}
	if !hit {
		t.Fatal("onlyMismatched 列表应包含有 不匹配验真 的战报")
	}
}
