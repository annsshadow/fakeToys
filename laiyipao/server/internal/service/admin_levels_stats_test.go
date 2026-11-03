// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R64：`/admin/levels` 从「每次全表聚合 battle_records」改为
// 「读 dashboard_battle_stats 增量表」（迁移 00025/00026）。
//
// 这是本轮按 `pg_stat_statements` 总耗时排序的第一名：
// 单次 245ms / 并发 10 时 501ms。
//
// 改写的风险是口径漂移，所以这里不比「接口 200」，而是把同一份夹具上
// 增量表读出的 attempts / clears / avg_wave 与**直接聚合 battle_records**
// 的真值逐行比对，并覆盖触发器的三个分支：
//
//	INSERT → 计数增加
//	UPDATE 只改 wave_reached → avg_wave 必须跟着变（旧函数漏这个判据）
//	DELETE → 计数回扣
//
// 任何一条漂移都会让运营看到的通关率与真实战报对不上。

import (
	"context"
	"math"
	"testing"
)

func TestAdminLevelsStatsMatchBattleRecords(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	var uid int64
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO users (guest_token, nickname, is_guest, status, created_at, last_login_at)
		VALUES ('r64_levels', 'R64', true, 1, now(), now())
		RETURNING id`).Scan(&uid); err != nil {
		t.Fatalf("建用户失败：%v", err)
	}

	insertBattle := func(level int, result string, dur, wave int) int64 {
		t.Helper()
		var id int64
		if err := ts.pool.QueryRow(ctx, `
			INSERT INTO battle_records (user_id, level_id, result, duration_ms, wave_reached, created_at)
			VALUES ($1, $2, $3, $4, $5, now())
			RETURNING id`, uid, level, result, dur, wave).Scan(&id); err != nil {
			t.Fatalf("建战报失败：%v", err)
		}
		return id
	}
	// 关卡 1：3 战 2 胜，波次 10/3/8 → avg_wave = 7
	insertBattle(1, "win", 1000, 10)
	insertBattle(1, "lose", 2000, 3)
	insertBattle(1, "win", 1500, 8)
	// 关卡 2：1 战 0 胜
	insertBattle(2, "lose", 900, 1)

	// 直接聚合的真值（与旧 SQL 一致）。
	direct := func(level int) (attempts, clears int64, avg float64) {
		t.Helper()
		if err := ts.pool.QueryRow(ctx, `
			SELECT COUNT(*), COUNT(*) FILTER (WHERE result = 'win'),
			       COALESCE(AVG(wave_reached), 0)
			  FROM battle_records WHERE level_id = $1`, level).
			Scan(&attempts, &clears, &avg); err != nil {
			t.Fatalf("直接聚合失败：%v", err)
		}
		return
	}

	check := func(stage string) {
		t.Helper()
		list, total, err := ts.AdminListLevels(ctx, 0, "", 0, 0)
		if err != nil {
			t.Fatalf("%s：读列表失败：%v", stage, err)
		}
		if int64(len(list)) != total {
			t.Fatalf("%s：列表 %d 行与 total %d 不符", stage, len(list), total)
		}
		for _, lv := range list {
			wantA, wantC, wantAvg := direct(lv.ID)
			if lv.Attempts != wantA || lv.Clears != wantC ||
				math.Abs(lv.AvgWave-wantAvg) > 1e-9 {
				t.Fatalf("%s：关卡 %d 增量表 (%d/%d/%.6f) 与战报真值 (%d/%d/%.6f) 不一致 —— "+
					"运营看到的通关率会与真实战报对不上",
					stage, lv.ID, lv.Attempts, lv.Clears, lv.AvgWave, wantA, wantC, wantAvg)
			}
		}
		// 单行读回必须与列表同一行一致（改关卡的写后回读也走这条路）。
		for _, id := range []int{1, 2, 3} {
			row, err := ts.AdminLevelRow(ctx, id)
			if err != nil {
				t.Fatalf("%s：AdminLevelRow(%d) 失败：%v", stage, id, err)
			}
			wantA, wantC, wantAvg := direct(id)
			if row.Attempts != wantA || row.Clears != wantC ||
				math.Abs(row.AvgWave-wantAvg) > 1e-9 {
				t.Fatalf("%s：单行关卡 %d (%d/%d/%.6f) 与真值 (%d/%d/%.6f) 不一致",
					stage, id, row.Attempts, row.Clears, row.AvgWave, wantA, wantC, wantAvg)
			}
		}
	}

	check("插入后")

	// UPDATE 只改 wave_reached（不改 result/duration/level_id）：
	// 旧触发器的判据漏掉这一列，avg_wave 会停在旧值。
	if _, err := ts.pool.Exec(ctx,
		`UPDATE battle_records SET wave_reached = 12 WHERE level_id = 1 AND wave_reached = 3`); err != nil {
		t.Fatalf("更新波次失败：%v", err)
	}
	check("只改波次后")

	// DELETE 必须回扣。
	if _, err := ts.pool.Exec(ctx,
		`DELETE FROM battle_records WHERE level_id = 2`); err != nil {
		t.Fatalf("删除战报失败：%v", err)
	}
	check("删除后")
}
