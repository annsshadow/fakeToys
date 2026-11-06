// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 131 轮：榜单分数物化（迁移 00028）的**一致性**守卫。
//
// # 这条测试真正在守什么
//
// 迁移把「用户 → 该榜分数」写进 user_progress 两列，并由触发器在同一事务内
// 维护。于是正确性不再只取决于「SQL 写对了」，还取决于**每一个写路径都触发了
// 触发器**。漏一个（比如后台调分、批量导入、直接 SQL 修数据）的症状是静默的：
// 榜单和名次都不会报错，只是分数停在旧值。
//
// 判据是**拿物化结果和实时定义逐字段对拍**，而不是再算一遍同一份快照：
//   - 实时分数由 `level_stars` 重新聚合（`MAX(level_id) FILTER (WHERE clears>0)`、
//     `MIN(min_power_clear) FILTER (WHERE min_power_clear>0)`）—— 与迁移回填的
//     表达式逐字相同，但**不读物化列**；
//   - 名次由窗口函数 `ROW_NUMBER() OVER (ORDER BY score <dir>, user_id ASC)`
//     独立算一遍，覆盖「同分按 id 决胜」；
//   - 抽样 100 个用户，逐字段比对 score / rank / total，再比对榜单前 200 行。
//
// 数据里刻意安排大量并列（power 每 17 人一档、stage 每 20 人一档、
// efficiency 每 9 人一档），因为效率榜并列极常见，而并列正是
// 「同分但名次算错」最容易藏住的地方。
//
// # 过期窗口
//
// 触发器是**同事务**的，所以写后立刻读必须是精确值（误差上界 0）。
// 测试末尾专门做一次「改分 → 立刻读」，不 flush、不等待。
// 破坏实验：`ALTER TABLE level_stars DISABLE TRIGGER level_stars_leaderboard_sync`
// 之后再改分，本测试必须红。

import (
	"context"
	"sort"
	"testing"
)

// realtimeRankSQL 是三个榜的「实时定义」。
//
// ⚠️ 这里**不能**引用 clear_stage_max / min_power_clear_ever：
// 那正是被测的物化列，用它做基准等于自己和自己对拍。
var realtimeRankSQL = map[string]string{
	// power：level_exp，0 分也参与（与 leaderboardScoresSQL 的 participation 一致）
	"power": `
		SELECT user_id, level_exp::bigint AS score,
		       row_number() OVER (ORDER BY level_exp DESC, user_id ASC) AS rn,
		       count(*) OVER () AS total
		  FROM user_progress`,
	// stage：MAX(level_id) WHERE clears > 0，0 = 无成绩、不参与
	"stage": `
		WITH real AS (
		    SELECT p.user_id,
		           COALESCE(MAX(ls.level_id) FILTER (WHERE ls.clears > 0), 0) AS score
		      FROM user_progress p
		      LEFT JOIN level_stars ls ON ls.user_id = p.user_id
		     GROUP BY p.user_id)
		SELECT user_id, score,
		       row_number() OVER (ORDER BY score DESC, user_id ASC) AS rn,
		       count(*) OVER () AS total
		  FROM real WHERE score > 0`,
	// efficiency：MIN(min_power_clear) WHERE > 0，越低越好
	"efficiency": `
		WITH real AS (
		    SELECT p.user_id,
		           COALESCE(MIN(ls.min_power_clear) FILTER (WHERE ls.min_power_clear > 0), 0) AS score
		      FROM user_progress p
		      LEFT JOIN level_stars ls ON ls.user_id = p.user_id
		     GROUP BY p.user_id)
		SELECT user_id, score,
		       row_number() OVER (ORDER BY score ASC, user_id ASC) AS rn,
		       count(*) OVER () AS total
		  FROM real WHERE score > 0`,
}

type realtimeRow struct {
	score int64
	rank  int
	total int64
}

// loadRealtime 执行实时定义，返回 user → (score, rank, total)。
func loadRealtime(t *testing.T, ctx context.Context, ts *testService, kind string) map[int64]realtimeRow {
	t.Helper()
	rows, err := ts.pool.Query(ctx, realtimeRankSQL[kind])
	if err != nil {
		t.Fatalf("实时定义查询（%s）失败：%v", kind, err)
	}
	defer rows.Close()
	out := map[int64]realtimeRow{}
	for rows.Next() {
		var uid int64
		var score int64
		var rn int
		var total int64
		if err := rows.Scan(&uid, &score, &rn, &total); err != nil {
			t.Fatalf("扫描实时定义（%s）失败：%v", kind, err)
		}
		out[uid] = realtimeRow{score: score, rank: rn, total: total}
	}
	if err := rows.Err(); err != nil {
		t.Fatalf("遍历实时定义（%s）失败：%v", kind, err)
	}
	return out
}

// TestLeaderboardSnapshotMatchesRealtime 100 用户 × 三榜逐字段对拍。
func TestLeaderboardSnapshotMatchesRealtime(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	const n = 100
	ids := make([]int64, n)
	for i := range ids {
		ids[i] = ts.newUser(t, ctx)
		// power 刻意制造并列：每 17 人一档
		ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`,
			ids[i], int64((i%17)*100))
		// 每 10 人里留一个「从没通关」的：stage/efficiency 都不该把他算进人数
		if i%10 == 0 {
			continue
		}
		// 一条已通关：stage 取 level_id 最大值，efficiency 取 min_power_clear
		ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
		            VALUES ($1, $2, 3, 1000, 1, $3)`,
			ids[i], 10+i%20, int64(100+(i%9)*3))
		// 一条未通关：level_id 更大但 clears=0，不得把 stage 抬高；
		// min_power_clear=0，不得影响 efficiency
		ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
		            VALUES ($1, $2, 0, 0, 0, 0)`,
			ids[i], 40+i%20)
	}

	for _, kind := range []string{"power", "stage", "efficiency"} {
		t.Run(kind, func(t *testing.T) {
			real := loadRealtime(t, ctx, ts, kind)

			// ① 逐用户逐字段：score / rank / total
			for _, uid := range ids {
				mr, err := ts.MyRank(ctx, uid, kind)
				if err != nil {
					t.Fatalf("%s 榜 MyRank(user=%d) 失败：%v", kind, uid, err)
				}
				want, ok := real[uid]
				if !ok {
					// 实时定义里没有我 = 无有效成绩
					if mr.Rank != 0 || mr.Score != 0 {
						t.Errorf("%s 榜 user=%d 无成绩，MyRank 应给 rank=0/score=0，实际 rank=%d score=%d",
							kind, uid, mr.Rank, mr.Score)
					}
					continue
				}
				if mr.Score != want.score {
					t.Errorf("%s 榜 user=%d 分数不一致：快照 %d，实时 %d（触发器漏了？）",
						kind, uid, mr.Score, want.score)
				}
				if int64(mr.Rank) != int64(want.rank) {
					t.Errorf("%s 榜 user=%d 名次不一致：快照 %d，实时 %d",
						kind, uid, mr.Rank, want.rank)
				}
				if mr.Total != want.total {
					t.Errorf("%s 榜 user=%d 人数不一致：直方图 %d，实时 %d",
						kind, uid, mr.Total, want.total)
				}
			}

			// ② 榜单前 200 行：user_id / score / rank 必须与实时窗口逐行一致
			items, err := ts.Leaderboard(ctx, kind, 200)
			if err != nil {
				t.Fatalf("%s 榜 Leaderboard 失败：%v", kind, err)
			}
			type wantRow struct {
				uid   int64
				score int64
				rank  int
			}
			wantRows := make([]wantRow, 0, len(real))
			for uid, r := range real {
				if r.rank <= 200 {
					wantRows = append(wantRows, wantRow{uid: uid, score: r.score, rank: r.rank})
				}
			}
			sort.Slice(wantRows, func(i, j int) bool { return wantRows[i].rank < wantRows[j].rank })
			if len(items) != len(wantRows) {
				t.Fatalf("%s 榜行数不一致：物化 %d，实时 %d", kind, len(items), len(wantRows))
			}
			for i, w := range wantRows {
				got := items[i]
				if got.UserID != w.uid || got.Score != w.score || got.Rank != w.rank {
					t.Fatalf("%s 榜第 %d 行不一致：物化 (id=%d score=%d rank=%d)，实时 (id=%d score=%d rank=%d)",
						kind, i+1, got.UserID, got.Score, got.Rank, w.uid, w.score, w.rank)
				}
			}
		})
	}

	// ③ 过期窗口 = 0：写后**立刻**读，不 flush、不等待。
	t.Run("write-then-read", func(t *testing.T) {
		target := ids[5]
		before, err := ts.MyRank(ctx, target, "efficiency")
		if err != nil {
			t.Fatalf("写前 MyRank 失败：%v", err)
		}
		// 加一条「更省」的通关记录：效率榜应当立刻前进（分数变小、名次减小）
		ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
		            VALUES ($1, 3, 3, 1000, 1, 1)`, target)
		after, err := ts.MyRank(ctx, target, "efficiency")
		if err != nil {
			t.Fatalf("写后 MyRank 失败：%v", err)
		}
		if after.Score != 1 {
			t.Fatalf("写后分数应立刻变为 1，实际 %d（触发器非同步？）", after.Score)
		}
		if after.Rank != 1 {
			t.Fatalf("写后名次应立刻变为 1，实际 %d", after.Rank)
		}
		if after.Total != before.Total {
			t.Fatalf("新增一条记录不该改变人数：写前 %d，写后 %d", before.Total, after.Total)
		}
		// 删除该行：分数/名次必须立刻回退到写前
		ts.exec(t, `DELETE FROM level_stars WHERE user_id = $1 AND level_id = 3`, target)
		restored, err := ts.MyRank(ctx, target, "efficiency")
		if err != nil {
			t.Fatalf("删后 MyRank 失败：%v", err)
		}
		if restored.Score != before.Score || restored.Rank != before.Rank {
			t.Fatalf("删除后应回退到写前：before=(%d,%d) after=(%d,%d)",
				before.Score, before.Rank, restored.Score, restored.Rank)
		}
		if restored.Total != before.Total {
			t.Fatalf("删除后人数应回退：before=%d after=%d", before.Total, restored.Total)
		}
	})
}
