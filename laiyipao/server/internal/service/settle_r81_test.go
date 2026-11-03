// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// settle_r81_test.go — R81 两条改动的行为测试：
//
//  1. 战报 INSERT 挪到事务尾（nextval 预取 id）：功能性不变量 —
//     wallet_flows.ref_id 必须仍指向本局战报（预取值与落库行一致）；
//  2. 事务后构筑不再重读第二遍：响应的 Rating/Power 必须与
//    「结算后最新状态重算」逐位一致（等价性），跨级时例外走重读。
//
// 这些断言盯的是「一致性不受影响」（R81 要求 2 的前置证明），
// 不是性能数字 —— 性能数字由 r81 探针在 1M 库上量。

import (
	"context"
	"reflect"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// r81SettleResp 与 r80Settle 同一条真实结算路径，但带回完整响应。
func r81SettleResp(t *testing.T, ts *testService, ctx context.Context, uid int64) SettleResp {
	t.Helper()
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	var seedText string
	if err := ts.pool.QueryRow(ctx, `SELECT seed FROM battle_tokens WHERE id=$1`, tp.TokenID).Scan(&seedText); err != nil {
		t.Fatalf("读 seed 失败：%v", err)
	}
	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读构筑失败：%v", err)
	}
	out := domain.Replay(domain.GenerateLevel(1), buildSnapshotForReplay(t, build), seedText,
		[]int{0, 1, -1}, "", domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules())
	if out.Err != nil {
		t.Fatalf("重算跑不起来：%v", out.Err)
	}
	resp, err := ts.SettleBattle(ctx, uid, tp.TokenID, domain.SettleInput{
		Result: "lose", Score: 1000, DurationMs: 30_000,
		Shots: 200, Hits: 100, Reactions: 20, HeatMax: 50,
		HPLeft: 500, WaveReached: 1, Kills: 30, Leaked: 0,
		ReplayHash:    out.ComputedHash,
		ElementsUsed:  map[string]int{"fire": 3},
		ReactionsUsed: map[string]int{"overheat": 20},
		TerrainUsed:   []string{},
		CardPicks:     []int{0, 1, -1},
	})
	if err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	return resp
}

// TestSettleWalletFlowRefMatchesBattleID：INSERT 挪尾 + id 预取之后，
// 掉落流水的 ref_id 仍必须指向本局战报（链路不能断）。
func TestSettleWalletFlowRefMatchesBattleID(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	resp := r81SettleResp(t, ts, ctx, uid)

	var refID int64
	if err := ts.pool.QueryRow(ctx, `
		SELECT ref_id FROM wallet_flows
		 WHERE user_id = $1 AND reason = 'battle_reward'
		 ORDER BY id DESC LIMIT 1`, uid).Scan(&refID); err != nil {
		t.Fatalf("读掉落流水失败：%v", err)
	}
	if refID != resp.BattleID {
		t.Errorf("掉落流水 ref_id=%d ≠ 本局战报 id=%d —— id 预取/插入链路断了", refID, resp.BattleID)
	}
	// 战报行本身必须存在且 id 就是预取的那个。
	var cnt int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM battle_records WHERE id = $1`, resp.BattleID).Scan(&cnt); err != nil {
		t.Fatalf("读战报失败：%v", err)
	}
	if cnt != 1 {
		t.Errorf("战报 id=%d 应有 1 行，实际 %d", resp.BattleID, cnt)
	}
}

// TestSettleTailReuseEquivalentWhenNoLevelUp：非跨级结算的响应评分
// 必须与「结算后重读构筑再算」逐位一致 —— 这是「复用事务前快照」
// 的正确性定义（等价性，不是性能）。
func TestSettleTailReuseEquivalentWhenNoLevelUp(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
	// 新号 exp=0，本局 +80 远低于任何跨级线 —— 保证走「复用」分支。

	pre, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读事务前构筑失败：%v", err)
	}

	resp := r81SettleResp(t, ts, ctx, uid)

	// 结算后重读构筑 + 库内最新点数，独立重算。
	post, parts, err := ts.loadBuildSnapshotFull(ctx, uid)
	if err != nil {
		t.Fatalf("读结算后构筑失败：%v", err)
	}
	if got, want := computePower(post), resp.Power; got != want {
		t.Errorf("响应 Power=%d ≠ 结算后重算 %d —— 复用快照与最新状态不一致", want, got)
	}
	wantRating := computeRating(post, parts.masteryPoints)
	if !reflect.DeepEqual(resp.Rating, wantRating) {
		t.Errorf("响应 Rating=%+v ≠ 结算后重算 %+v（点数=%d）", resp.Rating, wantRating, parts.masteryPoints)
	}
	// 非跨级时「事务前快照」与「结算后重读」的构筑本体必须逐位一致
	// （否则复用前提本身被种子/并发改动破坏，等价断言失去意义）。
	if computePower(pre) != computePower(post) {
		t.Errorf("非跨级下事务前后构筑的 Power 变了（%d → %d）—— 复用前提不成立",
			computePower(pre), computePower(post))
	}
}

// TestSettleTailReloadsWhenLevelUp：跨级结算解锁新技能，
// 响应必须反映**重读后**的构筑（含新解锁行），不是事务前的旧快照。
func TestSettleTailReloadsWhenLevelUp(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	// 把经验垫到「6 级差 1 点」：本局 +80 必跨级。
	d := int64(domain.PlayerLevelDivisor)
	ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, 6*d-1)

	// 前置：确实存在会在本次跨级中新解锁的基础技能
	// （否则「重读 ≠ 复用」无鉴别力 —— 那是种子数据变了，必须吵起来）。
	var unlockable int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM skills s
		 WHERE s.unlock_level > 1 AND s.unlock_level <= 6
		   AND s.id NOT IN (SELECT output_skill_id FROM skill_recipes)
		   AND s.id NOT IN (SELECT skill_id FROM user_skills WHERE user_id = $1)`, uid).
		Scan(&unlockable); err != nil {
		t.Fatalf("查可解锁技能失败：%v", err)
	}
	if unlockable == 0 {
		t.Fatal("等级 2..6 没有可新解锁的基础技能 —— 种子数据变了，本测试失去鉴别力")
	}

	preSkills, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读事务前构筑失败：%v", err)
	}
	preCount := len(preSkills["skills"].(map[string]any))

	resp := r81SettleResp(t, ts, ctx, uid)

	post, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读结算后构筑失败：%v", err)
	}
	postCount := len(post["skills"].(map[string]any))
	if postCount <= preCount {
		t.Fatalf("跨级后技能数应增加（%d → %d）—— 解锁没发生，测试前提被破坏", preCount, postCount)
	}
	// 关键断言：响应 Power 必须等于**重读后**算出的值。
	// 若实现误用事务前快照（不含新解锁技能），这里不等。
	if got, want := computePower(post), resp.Power; got != want {
		t.Errorf("跨级结算响应 Power=%d ≠ 重读构筑算出的 %d —— 响应用了旧快照", want, got)
	}
}
