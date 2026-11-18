// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 复活局的 replay_hash 契约缺口（第 40 轮）：
//
//	客户端开局拿到 extra_base_hp > 0，engine.ts 把它计进 replay_hash 的
//	|X{extraBaseHp} 段；但服务端重算走 domain.Replay 时从未把复活加成
//	传进 BattleConfig.ExtraBaseHp，于是重算哈希的 X 段恒为 0。
//	enforce 档下，合法复活胜利局因此被误判「重放不符」而拒发奖励。
//
// 本测试走真实重算路径，判据是「合法复活上报被接受」，
// 不扫描任何字符串。

import (
	"context"
	"encoding/json"
	"strconv"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// revivedClientHash 复刻客户端「带额外血量」的 replay_hash。
//
// 用与 domain.Replay 完全相同的构筑 / 种子 / 选牌脚本建引擎，
// 唯一差别是把 ExtraBaseHp 设成复活加成 —— 这正是客户端
// engine.ts 里 cfg.extraBaseHp 的语义（|X 段读它）。
func revivedClientHash(t *testing.T, buildRaw []byte, seed string,
	cardPicks []int, extra int64) string {
	t.Helper()

	var bs domain.BuildSnapshot
	if err := json.Unmarshal(buildRaw, &bs); err != nil {
		t.Fatalf("构筑快照解码失败：%v", err)
	}
	seedVal, err := strconv.ParseInt(seed, 10, 64)
	if err != nil {
		t.Fatalf("种子 %q 非法：%v", seed, err)
	}
	skills := domain.SkillsByID()
	equipped := domain.EquippedFromSnapshot(bs, skills)
	att, err := domain.AttackerFromSnapshot(bs)
	if err != nil {
		t.Fatalf("还原攻方失败：%v", err)
	}
	if len(equipped) == 0 {
		t.Fatal("构筑快照没有已装备技能，无法构造客户端哈希")
	}

	e := domain.NewBattleEngine(domain.BattleConfig{
		Level: domain.GenerateLevel(1), Enemies: domain.EnemiesByID(),
		Skills: skills, Equipped: equipped, Attacker: att, Seed: seedVal,
		ActiveSlots: bs.ActiveSlots, ScoreRules: domain.DefaultScoreRules(),
		ExtraBaseHp: extra,
	})
	if len(cardPicks) > 0 {
		e.SetReplayScript(cardPicks)
	}
	e.Start()
	domain.RunToEnd(e, domain.MaxTicksPerReplay)
	return e.ReplayHash()
}

// TestRevivedSettleRecomputeMatchesClientHash 钉死复活局的验真闭环。
//
// 红：客户端上报带 |X{extra} 的哈希，服务端重算按 X0 算 → 不符 → 拒发 + 台账。
// 绿：重算把 tok.Revived 换算成 ReviveExtraBaseHP 传进 Replay → 逐位一致 → 放行。
func TestRevivedSettleRecomputeMatchesClientHash(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	ts.Service.Cfg().SettleRecomputeMode = RecomputeEnforce
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	in, tp, err := ts.realHashSettle(t, ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}

	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读构筑快照失败：%v", err)
	}
	buildRaw, err := json.Marshal(build)
	if err != nil {
		t.Fatalf("构筑快照编码失败：%v", err)
	}

	gl := domain.GenerateLevel(1)
	extra := domain.ReviveExtraBaseHP(gl)
	if extra <= 0 {
		t.Fatalf("第 1 关复活加成 = %d，期望 > 0 —— 夹具失效", extra)
	}

	// 客户端上报的是「带额外血量」的哈希。
	in.ReplayHash = revivedClientHash(t, buildRaw, tp.Seed, in.CardPicks, extra)

	tok := domain.BattleToken{
		ID: tp.TokenID, UserID: uid, LevelID: 1,
		Seed: mustSeed(t, tp.Seed), Revived: true,
	}

	ts.resetEngineCounters()
	before := ts.countRecomputesOf(t, ctx, uid)
	err = ts.recomputeEngine(ctx, uid, tok, gl, buildRaw, in)

	if err != nil {
		t.Errorf("合法复活局被拒发：%v —— 客户端 |X 段计入额外血量，服务端重算也必须计入", err)
	}
	if c := ts.EngineCounters(); c.Verified != 1 || c.Mismatch != 0 {
		t.Errorf("重算计数器异常：verified=%d mismatch=%d（期望 1/0）", c.Verified, c.Mismatch)
	}
	if got := ts.countRecomputesOf(t, ctx, uid); got != before {
		t.Errorf("台账多出 %d 行 —— 合法复活局不该记不符", got-before)
	}
}

// TestRevivedSettleBattleEndToEnd 走真实结算入口 SettleBattle。
//
// 上一条直接调 recomputeEngine，手动构造 tok.Revived=true；
// 这里验证生产链路：开局真正消耗复活币 → battle_tokens.revived 落库 →
// SettleBattle 从库里读出标记 → 重算同口径 → 放行。
// 任何一环断了（DB 没存、SELECT 没读、没传进重算）都会红。
func TestRevivedSettleBattleEndToEnd(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	ts.Service.Cfg().SettleRecomputeMode = RecomputeEnforce
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})
	ts.grant(t, ctx, uid, map[string]int64{"revive_token": 1})

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	if !tp.Revived || tp.ExtraBaseHP <= 0 {
		t.Fatalf("复活开局响应异常：revived=%t extra=%d", tp.Revived, tp.ExtraBaseHP)
	}
	extra := domain.ReviveExtraBaseHP(domain.GenerateLevel(1))
	if tp.ExtraBaseHP != extra {
		t.Fatalf("下发额外血量 %d，期望 %d", tp.ExtraBaseHP, extra)
	}

	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读构筑快照失败：%v", err)
	}
	buildRaw, err := json.Marshal(build)
	if err != nil {
		t.Fatalf("构筑快照编码失败：%v", err)
	}

	in := loseSettleInput()
	in.ReplayHash = revivedClientHash(t, buildRaw, tp.Seed, in.CardPicks, extra)

	ts.resetEngineCounters()
	before := ts.countRecomputesOf(t, ctx, uid)
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Errorf("真实复活结算被拒发：%v", err)
	}
	if c := ts.EngineCounters(); c.Verified != 1 || c.Mismatch != 0 {
		t.Errorf("重算计数器异常：verified=%d mismatch=%d（期望 1/0）", c.Verified, c.Mismatch)
	}
	if got := ts.countRecomputesOf(t, ctx, uid); got != before {
		t.Errorf("台账多出 %d 行 —— 合法复活局不该记不符", got-before)
	}
}
