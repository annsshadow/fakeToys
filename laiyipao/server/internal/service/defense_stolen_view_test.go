// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"testing"
)

// TestListDefensesMyStolenToday 校验 GET /api/v1/defenses 的
// mine.my_stolen_today 必须等于玩家当天防线被偷的真实次数。
//
// 服务端契约三处互相印证：
//   - DefenseView 声明 MyStolenToday（json:my_stolen_today）；
//   - API 参考 05-api-reference.md 把 mine.my_stolen_today 列为响应字段；
//   - ListDefenses 开头 dailyChallengeCounters 已读到当日 stolen_times，
//     却用 `_` 丢弃；ChallengeDefense 每次胜利给 owner 的 stolen_times+1，
//     并以 ChallengeResult.OwnerStolenToday 回显同一计数器。
//
// 行为断言：真实挑战两次（Won:true）后，防守方视角的 mine.my_stolen_today
// 必须是 2。若列表不回填，该字段恒为 0 —— 防守方永远看不到自己当天被偷几次。
func TestListDefensesMyStolenToday(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	foe := ts.newUser(t, ctx)

	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "被挑战", Skills: []int{1, 4, 7}})
	if err != nil {
		t.Fatalf("owner 建防线失败：%v", err)
	}
	t.Cleanup(func() { ts.exec(t, "DELETE FROM defenses WHERE owner_id = $1", owner) })

	in := ChallengeInput{Seed: "1", Won: true, DurationMs: 60_000, HPLeftPct: 100, ReplayHash: "0000000000000000"}
	for i := 1; i <= 2; i++ {
		res, err := ts.ChallengeDefense(ctx, foe, dv.ID, in)
		if err != nil {
			t.Fatalf("第 %d 次挑战失败：%v", i, err)
		}
		// OwnerStolenToday 是本次结算前 owner 已被偷的次数（0、1）
		if res.OwnerStolenToday != i-1 {
			t.Fatalf("第 %d 次挑战的 owner_stolen_today 应为 %d，实际 %d", i, i-1, res.OwnerStolenToday)
		}
	}

	mine, _, err := ts.ListDefenses(ctx, owner)
	if err != nil {
		t.Fatalf("ListDefenses 失败：%v", err)
	}
	if mine == nil {
		t.Fatal("owner 已保存防线，mine 不应为 nil")
	}
	if mine.MyStolenToday != 2 {
		t.Errorf("mine.my_stolen_today 应等于今日真实被偷的 2 次，实际 %d", mine.MyStolenToday)
	}
}
