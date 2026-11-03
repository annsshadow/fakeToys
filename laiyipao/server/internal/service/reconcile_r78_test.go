// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R78：一致性对账的守卫测试。
//
// 两类证据缺一不可：
//  1. 干净库全绿（没有误报 —— 合法漂移源已按 reconcile.go 文件头的清单排除）；
//  2. 注入漂移必须报出（否则对账只是又一层永远绿的东西）。
//     五类注入一次打完：钱包扣币、道具余额、增量表 wave_sum、反应计数、
//     兑换 used_count、重复结算、未标记结算。

import (
	"context"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// r78SettleInput 一份最小的失败局结算输入（带一次 overheat 反应，
// 让 dashboard_reaction_counts 真的有数可核）。
func r78SettleInput() domain.SettleInput {
	return domain.SettleInput{
		Result: "lose", Score: 100, DurationMs: 30_000,
		Shots: 100, Hits: 50, Reactions: 2, HeatMax: 50,
		HPLeft: 900, WaveReached: 1,
		ElementsUsed:  map[string]int{"fire": 3},
		ReactionsUsed: map[string]int{"overheat": 2},
		TerrainUsed:   []string{},
		ReplayHash:    "",
	}
}

func TestReconcileCleanDBIsGreen(t *testing.T) {
	ts := openScratchService(t)
	rep, err := ts.AdminReconcile(context.Background())
	if err != nil {
		t.Fatalf("对账执行失败：%v", err)
	}
	if rep.Checks != 9 {
		t.Errorf("应跑 9 项检查，实际 %d", rep.Checks)
	}
	if !rep.OK || len(rep.Diffs) != 0 {
		t.Errorf("干净库应全绿，实际 ok=%v diffs=%v counts=%v", rep.OK, rep.Diffs, rep.ViolationCounts)
	}
	if len(rep.Notes) == 0 {
		t.Error("notes 不应为空（合法漂移源说明必须随报告下发）")
	}
}

func TestReconcileDetectsInjectedDrift(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 基线：一个真实用户 + 真实结算 + 真实兑换（都是绿的机制路径）。
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100, "gem_wash_token": 2})
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, int64(tp.TokenID), r78SettleInput()); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	var codeID int
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
		 VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), 'R78-DRIFT', '{"coin":1}', 5, 0, true)
		 RETURNING id`).Scan(&codeID); err != nil {
		t.Fatalf("造码失败：%v", err)
	}
	if _, err := ts.Redeem(ctx, uid, "R78-DRIFT"); err != nil {
		t.Fatalf("兑换失败：%v", err)
	}

	// 先证明注入前是绿的（否则后面分不清是注入生效还是本来就脏）。
	rep, err := ts.AdminReconcile(ctx)
	if err != nil {
		t.Fatalf("对账失败：%v", err)
	}
	if !rep.OK {
		t.Fatalf("注入前应全绿，实际 %v", rep.ViolationCounts)
	}

	// —— 注入五类漂移（每类都绕过应用层直改库，模拟「机制静默失效」）——
	ts.exec(t, `UPDATE user_wallets SET coin = coin + 7 WHERE user_id = $1`, uid)
	ts.exec(t, `UPDATE user_tokens SET balance = balance + 1 WHERE user_id = $1 AND token = 'gem_wash_token'`, uid)
	ts.exec(t, `UPDATE dashboard_battle_stats SET wave_sum = wave_sum + 1 WHERE level_id = 1`)
	ts.exec(t, `UPDATE dashboard_reaction_counts SET count = count + 1 WHERE level_id = 1 AND reaction = 'overheat'`)
	ts.exec(t, `UPDATE redeem_codes SET used_count = used_count + 1 WHERE id = $1`, codeID)
	// 重复结算：同一 token 两张战报（绕过 used_at 互斥，直插）。
	ts.exec(t, `INSERT INTO battle_records (user_id, level_id, result, battle_token_id, replay_hash)
			 VALUES ($1, 1, 'lose', $2, 'r78_dup1'), ($1, 1, 'lose', $2, 'r78_dup2')`, uid, tp.TokenID)
	// 未标记结算：战报引用一个 used_at 仍为 NULL 的 token。
	tp2, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("二次开局失败：%v", err)
	}
	ts.exec(t, `INSERT INTO battle_records (user_id, level_id, result, battle_token_id, replay_hash)
			 VALUES ($1, 1, 'lose', $2, 'r78_unmarked')`, uid, tp2.TokenID)

	rep, err = ts.AdminReconcile(ctx)
	if err != nil {
		t.Fatalf("注入后对账失败：%v", err)
	}
	if rep.OK {
		t.Fatal("注入了 7 处漂移，报告却全绿 —— 对账是摆设")
	}
	want := map[string]int64{
		"wallet_coin":               1,
		"wallet_tokens":             1,
		"dashboard_battle_stats":    1,
		"redeem_used_count":         1,
		"battle_double_settle":      1,
		"battle_unmarked_settle":    1,
		"dashboard_reaction_counts": 1,
	}
	for check, n := range want {
		if rep.ViolationCounts[check] != n {
			t.Errorf("检查 %s：违规数 = %d，期望 %d", check, rep.ViolationCounts[check], n)
		}
	}
	// 抽查两条样本的内容精度（报告必须能指出「哪个键、差在哪」）。
	assertSample := func(check, keyPart string) {
		t.Helper()
		for _, d := range rep.Diffs {
			if d.Check == check && strings.Contains(d.Key, keyPart) {
				return
			}
		}
		t.Errorf("样本里找不到 %s/%s：%v", check, keyPart, rep.Diffs)
	}
	assertSample("dashboard_battle_stats", "level#1")
	assertSample("redeem_used_count", "R78-DRIFT")
	assertSample("battle_double_settle", "token#")
}
