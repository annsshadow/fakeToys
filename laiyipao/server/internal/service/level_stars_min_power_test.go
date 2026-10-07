package service

// 第 143 轮：level_stars.min_power_clear 是效率榜（L-2）的唯一数据源。
// 修前结算 upsert 恒写 0 且 ON CONFLICT 不更新它 → 效率榜
// `WHERE min_power_clear > 0` 恒零行，整个榜单自出生起就是死代码。
//
// 判据：
//  1. 胜利后 min_power_clear == 通关当时构筑的战力（不是 0）
//  2. 失败局不覆盖旧值
//  3. 再通关取「更低战力」（效率=用最低战力打过去）
//  4. 效率榜能真正取到行（用户可见症状的回归）

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

func TestLevelStarsMinPowerClear(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 5000})

	// 授一个技能，保证战力 > 0（效率榜要求 min_power_clear > 0 才可见）
	ts.exec(t, `INSERT INTO user_skills (user_id, skill_id, level) VALUES ($1, 1, 3) ON CONFLICT (user_id, skill_id) DO NOTHING`, uid)
	ts.exec(t, `INSERT INTO user_skill_slots (user_id, slot, skill_id) VALUES ($1, 0, 1) ON CONFLICT (user_id, slot) DO NOTHING`, uid)

	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读构筑快照失败：%v", err)
	}
	power := ts.ComputePowerFor(build)
	if power <= 0 {
		t.Fatalf("前置不成立：授技能后战力应 > 0，实际 %d", power)
	}

	start := func() int64 {
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Fatalf("开局失败：%v", err)
		}
		return tp.TokenID
	}
	winIn := func() domain.SettleInput {
		in := loseSettleInput()
		in.Result = "win"
		in.Kills = domain.MaxKillsFor(domain.GenerateLevel(1))
		in.Score = 2000
		return in
	}
	readMinPower := func() int64 {
		var mpc int64
		if err := ts.pool.QueryRow(ctx,
			`SELECT min_power_clear FROM level_stars WHERE user_id = $1 AND level_id = 1`, uid).Scan(&mpc); err != nil {
			t.Fatalf("读 level_stars 失败：%v", err)
		}
		return mpc
	}

	// 1) 胜利后 min_power_clear == 通关战力（修前这里是 0）
	if _, err := ts.SettleBattle(ctx, uid, start(), winIn()); err != nil {
		t.Fatalf("胜利结算失败：%v", err)
	}
	if got := readMinPower(); got != power {
		t.Errorf("min_power_clear 应等于通关战力 %d，实际 %d", power, got)
	}

	// 4) 效率榜取到该用户，分数就是这份战力
	lb, err := ts.Leaderboard(ctx, "efficiency", 200)
	if err != nil {
		t.Fatalf("查效率榜失败：%v", err)
	}
	found := false
	for _, e := range lb {
		if e.UserID == uid {
			found = true
			if e.Score != power {
				t.Errorf("效率榜分数应为通关战力 %d，实际 %d", power, e.Score)
			}
		}
	}
	if !found {
		t.Errorf("效率榜应含用户 %d（min_power_clear=%d>0）——修前榜单恒空", uid, readMinPower())
	}

	// 2) 失败局不覆盖旧值
	if _, err := ts.SettleBattle(ctx, uid, start(), loseSettleInput()); err != nil {
		t.Fatalf("失败结算失败：%v", err)
	}
	if got := readMinPower(); got != power {
		t.Errorf("失败局不得改动 min_power_clear（应仍为 %d），实际 %d", power, got)
	}

	// 3) 再通关：战力仍同值（同一构筑）→ 取 LEAST 后不变
	if _, err := ts.SettleBattle(ctx, uid, start(), winIn()); err != nil {
		t.Fatalf("再通关结算失败：%v", err)
	}
	if got := readMinPower(); got != power {
		t.Errorf("同构筑再通关 LEAST 后应仍为 %d，实际 %d", power, got)
	}
}
