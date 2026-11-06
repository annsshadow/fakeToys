// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 146 轮：直方图分片（迁移 00029）的守卫。
//
// # 这条测试守的是什么
//
// 分片的正确性前提只有一条：**同一个用户的 +1 与 -1 必须落在同一行**。
// 所以分片键只能是 user_id（% 16），绝不能掺入 score —— 一旦掺入，
// 结算时「旧档 -1」会打到不存在的那份分片上，静默少计，榜单人数只涨不跌，
// 而且没有任何报错。
//
// 判据不是「再算一遍同样的 SQL」，而是：
//   - 32 个**连续** user_id → 每个分片恰好 2 人，直方图必须给出 16 行、每行 cnt=2；
//   - 每个 (kind, score) 的 SUM(cnt) 必须等于该分数的真实人数；
//   - 把一个用户的分数从 A 改到 B，A 分片正好 -1、B 分片正好 +1，其余分片不动。
//
// 破坏实验（必须红）：把函数里的 `v_shard` 改成常量 0（等价于回退迁移前的
// 单行直方图），上面第一组判据立刻变成「1 行 cnt=32」——测试必须失败。

import (
	"context"
	"testing"
)

// TestLeaderboardHistogramShard 校验分片分布与增减的跨分片一致性。
func TestLeaderboardHistogramShard(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	const n = 32
	const scoreA int64 = 77777
	const scoreB int64 = 77778

	ids := make([]int64, n)
	for i := range ids {
		ids[i] = ts.newUser(t, ctx)
		ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, ids[i], scoreA)
	}

	// ① 16 个分片各 2 人：分片键必须是 user_id，不能被分数或随机数替代。
	type shardRow struct {
		shard int
		cnt   int64
	}
	rows := func() []shardRow {
		t.Helper()
		rs, err := ts.pool.Query(ctx,
			`SELECT shard, cnt FROM leaderboard_score_counts
			  WHERE kind = 'power' AND score = $1 ORDER BY shard`, scoreA)
		if err != nil {
			t.Fatalf("查直方图失败：%v", err)
		}
		defer rs.Close()
		var out []shardRow
		for rs.Next() {
			var r shardRow
			if err := rs.Scan(&r.shard, &r.cnt); err != nil {
				t.Fatalf("扫描直方图失败：%v", err)
			}
			out = append(out, r)
		}
		return out
	}

	got := rows()
	if len(got) != 16 {
		t.Fatalf("同一分数应落在 16 个分片上（连续 user_id 每片 2 人），实际 %d 行：%+v", len(got), got)
	}
	var sum int64
	for _, r := range got {
		if r.cnt != 2 {
			t.Fatalf("分片 %d 人数应为 2，实际 %d（分片键掺了非 user_id 的量？）", r.shard, r.cnt)
		}
		sum += r.cnt
	}
	if sum != n {
		t.Fatalf("直方图人数合计应为 %d，实际 %d", n, sum)
	}

	// ② 跨分片迁移：A 档 -1、B 档 +1 各只落在该用户自己的分片上。
	uid := ids[0]
	uidShard := int(uid % 16)
	ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, scoreB)

	dec := rows()
	if len(dec) != 16 {
		t.Fatalf("减档后 A 档应仍覆盖 16 个分片，实际 %d 行", len(dec))
	}
	for _, r := range dec {
		want := int64(2)
		if r.shard == uidShard {
			want = 1
		}
		if r.cnt != want {
			t.Fatalf("A 档分片 %d 人数应为 %d，实际 %d（-1 打到了错误的分片？）",
				r.shard, want, r.cnt)
		}
	}

	var incSum int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT COALESCE(SUM(cnt),0) FROM leaderboard_score_counts
		  WHERE kind = 'power' AND score = $1`, scoreB).Scan(&incSum); err != nil {
		t.Fatalf("查 B 档失败：%v", err)
	}
	if incSum != 1 {
		t.Fatalf("B 档人数应为 1，实际 %d", incSum)
	}

	// ③ 人数口径与实时定义一致（把分片加总后必须仍是精确值）。
	var total int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT COALESCE(SUM(cnt),0) FROM leaderboard_score_counts WHERE kind = 'power'`).
		Scan(&total); err != nil {
		t.Fatalf("查 power 总人数失败：%v", err)
	}
	var realTotal int64
	if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM user_progress`).Scan(&realTotal); err != nil {
		t.Fatalf("查实时总人数失败：%v", err)
	}
	if total != realTotal {
		t.Fatalf("直方图总人数 %d 与 user_progress 行数 %d 不一致（分片丢了计数）",
			total, realTotal)
	}
}

// TestLeaderboardShardMigrationRoundTrip 校验 00029 能 goose 正向 + 回滚往返，
// 且两个方向都保持「同一用户只落一个分片、SUM 等于实时值」。
//
// 与上面那条守的区别：上面守的是**运行时触发器**的增减路径；这里守的是
// **迁移重建**（00029 Up 的 INSERT ... GROUP BY、Down 的合并且丢 shard 列）。
// 两条都必须能独立失败：把 Up 的 `(user_id % 16)` 改成常量 0 → 本测试红，
// 但运行时测试仍绿（触发器是对的）；把函数的 `v_shard` 改错 → 上面红、本测试绿。
func TestLeaderboardShardMigrationRoundTrip(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	gooseVersion := func() int64 {
		t.Helper()
		var v int64
		if err := ts.pool.QueryRow(ctx,
			`SELECT COALESCE(MAX(version_id),0) FROM goose_db_version`).Scan(&v); err != nil {
			t.Fatalf("查 goose 版本失败：%v", err)
		}
		return v
	}
	hasShardColumn := func() bool {
		t.Helper()
		var n int
		if err := ts.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM information_schema.columns
			  WHERE table_schema = 'public' AND table_name = 'leaderboard_score_counts'
			    AND column_name = 'shard'`).Scan(&n); err != nil {
			t.Fatalf("查 shard 列失败：%v", err)
		}
		return n == 1
	}
	// 三个榜的 SUM 都必须等于各自的实时口径（power 含 0 分；另两个排除 <=0）。
	assertCountsEqualRealtime := func(stage string) {
		t.Helper()
		cases := []struct {
			kind string
			real string
		}{
			{"power", `SELECT COUNT(*) FROM user_progress`},
			{"stage", `SELECT COUNT(*) FROM user_progress WHERE clear_stage_max > 0`},
			{"efficiency", `SELECT COUNT(*) FROM user_progress WHERE min_power_clear_ever > 0`},
		}
		for _, c := range cases {
			var got, want int64
			if err := ts.pool.QueryRow(ctx,
				`SELECT COALESCE(SUM(cnt),0) FROM leaderboard_score_counts WHERE kind = $1`,
				c.kind).Scan(&got); err != nil {
				t.Fatalf("%s：查直方图 SUM 失败：%v", c.kind, err)
			}
			if err := ts.pool.QueryRow(ctx, c.real).Scan(&want); err != nil {
				t.Fatalf("%s：查实时人数失败：%v", c.kind, err)
			}
			if got != want {
				t.Fatalf("%s 迁移后 SUM=%d，实时=%d（%s 不一致）", c.kind, got, want, stage)
			}
		}
	}
	// 每个用户一个**唯一**分数 → 该分数在直方图里只允许一行，
	// 且 shard 必须等于 user_id % 16。若分片键算错（同一用户跨两片），
	// 该分数会出现 2 行（SUM=2），这里直接红。
	assertShardPlacement := func(ids, scores []int64, stage string) {
		t.Helper()
		for i, uid := range ids {
			var shard, cnt int64
			var rowsN int
			rs, err := ts.pool.Query(ctx,
				`SELECT shard, cnt FROM leaderboard_score_counts
				  WHERE kind = 'power' AND score = $1`, scores[i])
			if err != nil {
				t.Fatalf("%s：查 uid=%d 的分数 %d 失败：%v", stage, uid, scores[i], err)
			}
			for rs.Next() {
				if err := rs.Scan(&shard, &cnt); err != nil {
					rs.Close()
					t.Fatalf("%s：扫描失败：%v", stage, err)
				}
				rowsN++
			}
			rs.Close()
			if rowsN != 1 {
				t.Fatalf("%s：uid=%d（分数 %d）落在 %d 个分片行上，应为 1（分片键算错？）",
					stage, uid, scores[i], rowsN)
			}
			if shard != uid%16 {
				t.Fatalf("%s：uid=%d 应在分片 %d，实际 %d", stage, uid, uid%16, shard)
			}
			if cnt != 1 {
				t.Fatalf("%s：uid=%d 的分数 cnt 应为 1，实际 %d", stage, uid, cnt)
			}
		}
	}

	// 16 个连续用户 → 16 个不同分片；每个用户一个唯一分数。
	base := int64(90001)
	ids := make([]int64, 16)
	scores := make([]int64, 16)
	for i := range ids {
		ids[i] = ts.newUser(t, ctx)
		scores[i] = base + int64(i)*7
		ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, ids[i], scores[i])
	}

	upVersion := gooseVersion()
	if !hasShardColumn() {
		t.Fatalf("迁移到最新后缺少 shard 列（版本 %d）", upVersion)
	}
	assertShardPlacement(ids, scores, "正向迁移后")
	assertCountsEqualRealtime("正向迁移后")

	// 回滚一步：shard 列消失、旧 PK/旧函数恢复（旧函数 ON CONFLICT (kind,score) 依赖它），
	// 且合并后的 SUM 仍必须等于实时值（回滚不能丢计数）。
	if err := ts.db.MigrateDown(ctx); err != nil {
		t.Fatalf("goose down 00029 失败：%v", err)
	}
	if got := gooseVersion(); got != upVersion-1 {
		t.Fatalf("回滚后版本应为 %d，实际 %d", upVersion-1, got)
	}
	if hasShardColumn() {
		t.Fatalf("回滚后 shard 列仍然存在")
	}
	// 真正走一次旧写入路径：分数 +1（旧函数若没配旧 PK，这里会报错）。
	ts.exec(t, `UPDATE user_progress SET level_exp = level_exp + 1 WHERE user_id = $1`, ids[0])
	scores[0]++
	assertCountsEqualRealtime("回滚后")

	// 再正向：重建必须按当前 user_progress 重新落分片（不是沿用回滚前的旧行）。
	if err := ts.db.Migrate(ctx); err != nil {
		t.Fatalf("goose up 00029 失败：%v", err)
	}
	if got := gooseVersion(); got != upVersion {
		t.Fatalf("再次正向迁移后版本应为 %d，实际 %d", upVersion, got)
	}
	if !hasShardColumn() {
		t.Fatalf("再次正向迁移后缺少 shard 列")
	}
	assertShardPlacement(ids, scores, "回滚再正向迁移后")
	assertCountsEqualRealtime("回滚再正向迁移后")
}
