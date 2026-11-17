// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"testing"
)

// TestListDefensesMyAttemptsToday 校验 GET /api/v1/defenses 的
// mine.my_attempts_today 必须等于玩家当天真实消耗的挑战次数。
//
// 服务端契约三处互相印证：
//   - DefenseView 声明了 MyAttemptsToday（progression.go 的 DefenseView 结构体）；
//   - ListDefenses 开头就通过 dailyChallengeCounters 读到了 attempts，
//     并用它决定候选的 can_challenge（progression.go: attempts < DefenseAttemptLimit）；
//   - 挑战结算 ChallengeResult 回填同名字段 MyAttempts（progression.go: MyAttempts: attempts + 1）。
//
// 语义明确：把「我今天用了几次额度」告诉客户端。miniapp 的 me.vue 据此显示
// 「今日剩余 attempt_limit - my_attempts_today 次」。
// 若列表不回填，客户端永远读到缺省 0，显示满额 —— 玩家次数用尽后仍以为能挑战，
// 点下去才被服务端 403 拒绝，体验割裂。
//
// 这是行为断言（调真实 ListDefenses、真实消耗挑战次数），不是源码字符串扫描。
func TestListDefensesMyAttemptsToday(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	foe := ts.newUser(t, ctx)

	// owner 建一条防线供 foe 挑战
	odv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "被挑战", Skills: []int{1, 4, 7}})
	if err != nil {
		t.Fatalf("owner 建防线失败：%v", err)
	}
	t.Cleanup(func() { ts.exec(t, `DELETE FROM defenses WHERE owner_id = $1`, owner) })
	// foe 建自己的防线 —— ListDefenses 的 mine 才有对象
	if _, err := ts.SaveDefense(ctx, foe, SaveDefenseInput{Name: "我的防线", Skills: []int{1}}); err != nil {
		t.Fatalf("foe 建防线失败：%v", err)
	}
	t.Cleanup(func() { ts.exec(t, `DELETE FROM defenses WHERE owner_id = $1`, foe) })

	// foe 真实消耗 2 次挑战额度。用失败挑战（不窃取资源，减少副作用）。
	in := ChallengeInput{Seed: "1", Won: false, DurationMs: 60_000, HPLeftPct: 100, ReplayHash: "0000000000000000"}
	for i := 1; i <= 2; i++ {
		res, err := ts.ChallengeDefense(ctx, foe, odv.ID, in)
		if err != nil {
			t.Fatalf("第 %d 次挑战失败：%v", i, err)
		}
		if res.MyAttempts != i {
			t.Fatalf("挑战结果 MyAttempts 应为 %d，实际 %d", i, res.MyAttempts)
		}
	}

	// foe 视角的防线列表：mine 的 my_attempts_today 必须反映刚才的 2 次消耗
	mine, _, err := ts.ListDefenses(ctx, foe)
	if err != nil {
		t.Fatalf("ListDefenses 失败：%v", err)
	}
	if mine == nil {
		t.Fatal("foe 已保存防线，mine 不应为 nil")
	}
	if mine.MyAttemptsToday != 2 {
		t.Errorf("mine.my_attempts_today 应等于今日真实消耗的 2 次，实际 %d", mine.MyAttemptsToday)
	}
}
