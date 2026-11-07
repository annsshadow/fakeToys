// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package store

// 第 149 轮：看板增量计数器分片（迁移 00030）的守卫。
//
// # 守什么
//
// 分片的正确性前提只有一条：**同一条战报的 +1 与 -1 必须落在同一分片**。
// 触发器 INSERT 用 NEW.id、DELETE/UPDATE 撤销用 OLD.id —— 一旦公式不一致，
// 删掉一条战报只会在原分片留下幽灵 +1，在另一分片留下 -1（或负数），
// 而全局 SUM 可能仍然「看起来对」。所以判据不能只看 SUM，必须看**每个分片**。
//
// 判据：
//   - 16 条 id 连续的战报 → 16 个分片各 1 行，SUM 与 battle_records 直接聚合逐项相等；
//   - 删除 id=5 后，shard 5 的 attempts/wins/duration/wave/反应计数全部归零，
//     其余分片不动，且没有负值；
//   - goose down→up 往返：down 后合并成单行且旧触发器仍可写，up 后按 id 重建；
//   - 回填后每个 (level) 的 SUM 仍等于直接聚合（数据一致性校验）。
//
// 破坏实验：把 DELETE 分支的分片写成 `((OLD.id + 1) % 16)` → 删除 id=5 后
// shard 5 仍是 1、shard 6 变 0（或负数），本用例必须红。

import (
	"context"
	"testing"
	"time"
)

func TestDashboardCounterShardMigration(t *testing.T) {
	dsn := scratchDB(t)
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()

	db, err := Open(ctx, openCfg(dsn))
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("Migrate 失败：%v", err)
	}

	var uid int64
	if err := db.Pool.QueryRow(ctx, `
		INSERT INTO users (guest_token, nickname, is_guest, status, created_at, last_login_at)
		VALUES ('r149_shard', 'R149', true, 1, now(), now())
		RETURNING id`).Scan(&uid); err != nil {
		t.Fatalf("建用户失败：%v", err)
	}

	insertBattle := func(id int64, level int, result string, dur, wave int, reactions string) {
		t.Helper()
		if _, err := db.Pool.Exec(ctx, `
			INSERT INTO battle_records
			    (id, user_id, level_id, result, duration_ms, wave_reached, reactions_used)
			VALUES ($1, $2, $3, $4, $5, $6, $7::jsonb)`,
			id, uid, level, result, dur, wave, reactions); err != nil {
			t.Fatalf("建战报失败 id=%d：%v", id, err)
		}
	}

	type agg struct {
		rows, attempts, wins, duration, wave int64
		shards                                []int32
	}
	readAgg := func(level int) agg {
		t.Helper()
		var a agg
		if err := db.Pool.QueryRow(ctx, `
			SELECT COUNT(*), COALESCE(SUM(attempts), 0), COALESCE(SUM(wins), 0),
			       COALESCE(SUM(duration_sum), 0), COALESCE(SUM(wave_sum), 0),
			       COALESCE(array_agg(shard ORDER BY shard), '{}'::int[])
			  FROM dashboard_battle_stats WHERE level_id = $1`, level).
			Scan(&a.rows, &a.attempts, &a.wins, &a.duration, &a.wave, &a.shards); err != nil {
			t.Fatalf("读增量表失败：%v", err)
		}
		return a
	}
	readShard := func(level, shard int) (attempts, wins, duration, wave int64) {
		t.Helper()
		if err := db.Pool.QueryRow(ctx, `
			SELECT COALESCE(SUM(attempts), 0), COALESCE(SUM(wins), 0),
			       COALESCE(SUM(duration_sum), 0), COALESCE(SUM(wave_sum), 0)
			  FROM dashboard_battle_stats WHERE level_id = $1 AND shard = $2`, level, shard).
			Scan(&attempts, &wins, &duration, &wave); err != nil {
			t.Fatalf("读分片 %d 失败：%v", shard, err)
		}
		return
	}
	readReaction := func(level int, reaction string) (rows, total int64) {
		t.Helper()
		if err := db.Pool.QueryRow(ctx, `
			SELECT COUNT(*), COALESCE(SUM(count), 0)
			  FROM dashboard_reaction_counts WHERE level_id = $1 AND reaction = $2`,
			level, reaction).Scan(&rows, &total); err != nil {
			t.Fatalf("读反应计数失败：%v", err)
		}
		return
	}
	directAgg := func(level int) (attempts, wins, duration, wave, overheat int64) {
		t.Helper()
		if err := db.Pool.QueryRow(ctx, `
			SELECT COUNT(*),
			       COUNT(*) FILTER (WHERE result = 'win'),
			       COALESCE(SUM(duration_ms), 0),
			       COALESCE(SUM(wave_reached), 0),
			       COALESCE(SUM((reactions_used->>'overheat')::bigint), 0)
			  FROM battle_records WHERE level_id = $1`, level).
			Scan(&attempts, &wins, &duration, &wave, &overheat); err != nil {
			t.Fatalf("直接聚合失败：%v", err)
		}
		return
	}
	assertNoNegative := func(step string) {
		t.Helper()
		var bad int64
		if err := db.Pool.QueryRow(ctx, `
			SELECT (SELECT COUNT(*) FROM dashboard_battle_stats
			         WHERE attempts < 0 OR wins < 0 OR duration_sum < 0 OR wave_sum < 0)
			     + (SELECT COUNT(*) FROM dashboard_reaction_counts WHERE count < 0)`).Scan(&bad); err != nil {
			t.Fatalf("%s：查负值失败：%v", step, err)
		}
		if bad != 0 {
			t.Fatalf("%s：分片计数出现 %d 行负值 —— +1/-1 落到了不同分片", step, bad)
		}
	}

	// 16 条 id=1..16 的战报覆盖全部分片；奇数局输、偶数局赢。
	for id := int64(1); id <= 16; id++ {
		result := "lose"
		if id%2 == 0 {
			result = "win"
		}
		insertBattle(id, 1, result, int(id*10), int(id), `{"overheat":1}`)
	}
	if _, err := db.Pool.Exec(ctx, `SELECT setval('battle_records_id_seq', 100, true)`); err != nil {
		t.Fatalf("推进序列失败：%v", err)
	}

	// ① 全分片分布 + SUM 与直接聚合一致。
	a := readAgg(1)
	if a.rows != 16 {
		t.Fatalf("应落在 16 个分片（每片 1 条），实际 %d 行：%+v", a.rows, a.shards)
	}
	for i, sh := range a.shards {
		if int(sh) != i {
			t.Fatalf("分片集合应为 0..15，实际 %v", a.shards)
		}
	}
	att, wins, dur, wave, overheat := directAgg(1)
	if a.attempts != att || a.wins != wins || a.duration != dur || a.wave != wave {
		t.Fatalf("SUM 口径 = {attempts:%d wins:%d dur:%d wave:%d}，直接聚合 = {%d %d %d %d}",
			a.attempts, a.wins, a.duration, a.wave, att, wins, dur, wave)
	}
	rows, total := readReaction(1, "overheat")
	if rows != 16 || total != overheat {
		t.Fatalf("反应分片 = %d 行 / %d，直接聚合 = %d 行 / %d", rows, total, 16, overheat)
	}
	assertNoNegative("插入后")

	// ② 删除 id=5（shard 5）：只能扣减 shard 5，不能留下幽灵 +1。
	if _, err := db.Pool.Exec(ctx, `DELETE FROM battle_records WHERE id = 5`); err != nil {
		t.Fatalf("删除战报失败：%v", err)
	}
	if sa, sw, sd, swv := readShard(1, 5); sa != 0 || sw != 0 || sd != 0 || swv != 0 {
		t.Fatalf("删除 id=5 后 shard 5 应为全零，实际 attempts=%d wins=%d dur=%d wave=%d（+1/-1 分片不一致）",
			sa, sw, sd, swv)
	}
	if rrows, rtotal := readReaction(1, "overheat"); rrows != 16 || rtotal != 15 {
		t.Fatalf("删除后反应计数应为 16 行（含零值）/ 15，实际 %d 行 / %d", rrows, rtotal)
	}
	att, wins, dur, wave, overheat = directAgg(1)
	if got := readAgg(1); got.attempts != att || got.wins != wins ||
		got.duration != dur || got.wave != wave || got.attempts != 15 {
		t.Fatalf("删除后 SUM 与直接聚合不一致：%+v vs {%d %d %d %d}", got, att, wins, dur, wave)
	}
	assertNoNegative("删除后")

	// ③ 回滚到 00029：合并回单行、旧触发器仍可写。
	downTo(t, db, ctx, 29)
	var shardCol int
	if err := db.Pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM information_schema.columns
		 WHERE table_name = 'dashboard_battle_stats' AND column_name = 'shard'`).Scan(&shardCol); err != nil {
		t.Fatalf("查列失败：%v", err)
	}
	if shardCol != 0 {
		t.Fatal("回滚后 shard 列仍存在")
	}
	// 回滚后没有 shard 列，不能用 readAgg（它按 shard 排序）。
	readSingle := func() (rows, attempts, wins, duration, wave int64) {
		t.Helper()
		if err := db.Pool.QueryRow(ctx, `
			SELECT COUNT(*), COALESCE(SUM(attempts), 0), COALESCE(SUM(wins), 0),
			       COALESCE(SUM(duration_sum), 0), COALESCE(SUM(wave_sum), 0)
			  FROM dashboard_battle_stats WHERE level_id = 1`).
			Scan(&rows, &attempts, &wins, &duration, &wave); err != nil {
			t.Fatalf("回滚后读增量表失败：%v", err)
		}
		return
	}
	if r, a, w, d, wv := readSingle(); r != 1 || a != 15 || w != 8 || d != 1310 || wv != 131 {
		t.Fatalf("回滚合并后 level 1 = {rows:%d attempts:%d wins:%d dur:%d wave:%d}，期望 {1 15 8 1310 131}",
			r, a, w, d, wv)
	}

	// 回滚期间写入：走旧触发器加到单行。
	insertBattle(101, 1, "win", 1000, 5, `{"overheat":2}`)
	if r, a, w, d, wv := readSingle(); r != 1 || a != 16 || w != 9 || d != 2310 || wv != 136 {
		t.Fatalf("回滚期写入后 level 1 = {rows:%d attempts:%d wins:%d dur:%d wave:%d}，期望 {1 16 9 2310 136}",
			r, a, w, d, wv)
	}

	// ④ 再升级：按 battle_records 重建分片（含回滚期间那条 id=101 → shard 5）。
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("重新 Migrate 失败：%v", err)
	}
	got := readAgg(1)
	if got.rows != 16 {
		t.Fatalf("重新升级后应重建 16 个分片，实际 %d 行：%v", got.rows, got.shards)
	}
	att, wins, dur, wave, overheat = directAgg(1)
	if got.attempts != att || got.wins != wins || got.duration != dur || got.wave != wave {
		t.Fatalf("重新升级后 SUM 口径 = %+v，直接聚合 = {%d %d %d %d}",
			got, att, wins, dur, wave)
	}
	if sa, sw, sd, swv := readShard(1, int(101%16)); sa != 1 || sw != 1 || sd != 1000 || swv != 5 {
		t.Fatalf("回滚期战报 id=101 应落在 shard 5 且计入 1 次，实际 %d %d %d %d", sa, sw, sd, swv)
	}
	if rrows, rtotal := readReaction(1, "overheat"); rrows != 16 || rtotal != overheat {
		t.Fatalf("重新升级后反应计数 = %d 行 / %d，直接聚合 = 16 行 / %d", rrows, rtotal, overheat)
	}
	assertNoNegative("重新升级后")

	// 兜底：分片数必须恰好是 16（改常量就会在这里红）。
	var maxShard int
	if err := db.Pool.QueryRow(ctx,
		`SELECT MAX(shard) FROM dashboard_battle_stats`).Scan(&maxShard); err != nil {
		t.Fatalf("查最大分片失败：%v", err)
	}
	if maxShard != 15 {
		t.Fatalf("分片上限应为 15（id%%16 的 16 片），实际 %d", maxShard)
	}
}
