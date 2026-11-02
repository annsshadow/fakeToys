// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R63：反应计数表（迁移 00025）的一致性护栏。
//
// 看板的反应分布从「每次刷新扫全表拆 jsonb」改成「写入时由触发器累加」。
// 这类结构最危险的不是慢，而是**悄悄漂移**：计数与战报对不上时，
// 看板会长期显示一个没人能解释的数字，而且没有任何测试会发现。
//
// 所以这里不看「接口 200」，而是每一步都与**直接聚合 battle_records**
// 的真值逐键比对，覆盖四种改变计数的写入：
//   INSERT / DELETE / UPDATE（含 level_id 变化）/ 删用户级联。

import (
	"context"
	"testing"
)

func TestDashboardReactionCountsMatchAggregate(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	mustExec := func(q string, args ...any) {
		t.Helper()
		if _, err := ts.pool.Exec(ctx, q, args...); err != nil {
			t.Fatalf("夹具写入失败：%v\nSQL: %s", err, q)
		}
	}

	// directAgg 是改前的口径：直接从 battle_records 拆 jsonb 求和。
	directAgg := func() map[string]int64 {
		t.Helper()
		rows, err := ts.pool.Query(ctx, `
			SELECT e.key, SUM(e.value::bigint)
			  FROM battle_records b,
			       LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
			 WHERE jsonb_typeof(b.reactions_used) = 'object'
			 GROUP BY e.key`)
		if err != nil {
			t.Fatalf("直接聚合失败：%v", err)
		}
		defer rows.Close()
		out := map[string]int64{}
		for rows.Next() {
			var k string
			var n int64
			if err := rows.Scan(&k, &n); err != nil {
				t.Fatalf("扫描直接聚合失败：%v", err)
			}
			out[k] = n
		}
		if err := rows.Err(); err != nil {
			t.Fatalf("直接聚合读取失败：%v", err)
		}
		return out
	}

	// counterAgg 是计数器表的口径（读侧实际取的就是它）。
	counterAgg := func() map[string]int64 {
		t.Helper()
		rows, err := ts.pool.Query(ctx, `SELECT reaction, SUM(count) FROM dashboard_reaction_counts GROUP BY reaction`)
		if err != nil {
			t.Fatalf("读计数表失败：%v", err)
		}
		defer rows.Close()
		// 计数归零后行会保留 count=0（触发器不做删除，避免热行反复拔插）。
		// 语义上与「没有这个键」等价，比对前先剔掉零值。
		out := map[string]int64{}
		for rows.Next() {
			var k string
			var n int64
			if err := rows.Scan(&k, &n); err != nil {
				t.Fatalf("扫描计数表失败：%v", err)
			}
			if n > 0 {
				out[k] = n
			}
		}
		if err := rows.Err(); err != nil {
			t.Fatalf("计数表读取失败：%v", err)
		}
		return out
	}

	assertSame := func(step string) {
		t.Helper()
		direct := directAgg()
		counter := counterAgg()
		if len(direct) != len(counter) {
			t.Fatalf("%s：键集不一致 直接=%v 计数=%v", step, direct, counter)
		}
		for k, want := range direct {
			if got := counter[k]; got != want {
				t.Fatalf("%s：反应 %q 计数 %d，直接聚合真值 %d", step, k, got, want)
			}
		}
	}

	var uid int64
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO users (guest_token, nickname, is_guest, status, created_at, last_login_at)
		VALUES ('r63_u1', 'R63客1', TRUE, 1, now(), now()) RETURNING id`).Scan(&uid); err != nil {
		t.Fatalf("建用户失败：%v", err)
	}

	insertBattle := func(level int, reactions string) int64 {
		t.Helper()
		var id int64
		if err := ts.pool.QueryRow(ctx, `
			INSERT INTO battle_records (user_id, level_id, result, duration_ms, reactions_used)
			VALUES ($1, $2, 'win', 1000, $3::jsonb) RETURNING id`, uid, level, reactions).Scan(&id); err != nil {
			t.Fatalf("建战报失败：%v", err)
		}
		return id
	}

	// 1) 三种输入形状：正常对象、空对象、jsonb null（null 必须被跳过）。
	insertBattle(1, `{"steam_burst":2,"overheat":1}`)
	insertBattle(2, `{}`)
	insertBattle(1, `null`)
	for _, level := range []int{1, 2} {
		var n int64
		if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM dashboard_reaction_counts WHERE level_id = $1`, level).Scan(&n); err != nil {
			t.Fatalf("查计数行失败：%v", err)
		}
		if level == 1 && n != 2 {
			t.Fatalf("关卡 1 应有 steam_burst/overheat 两行计数，实际 %d", n)
		}
		if level == 2 && n != 0 {
			t.Fatalf("关卡 2 不应有计数行（空对象），实际 %d", n)
		}
	}
	assertSame("插入后")

	// 2) 读侧（看板）与直接聚合一致。
	got, err := ts.adminDashboardUncached(ctx)
	if err != nil {
		t.Fatalf("看板刷新失败：%v", err)
	}
	direct := directAgg()
	seen := map[string]int64{}
	for _, rc := range got.ReactionUsage {
		seen[rc.Reaction] = rc.Count
	}
	if seen["steam_burst"] != direct["steam_burst"] || seen["overheat"] != direct["overheat"] {
		t.Fatalf("看板反应分布与真值不一致：看板=%v 真值=%v", seen, direct)
	}

	// 3) UPDATE 改变 level_id 与反应内容：旧增量必须被扣回。
	var bid int64
	if err := ts.pool.QueryRow(ctx, `
		SELECT id FROM battle_records WHERE level_id = 1 AND reactions_used = '{"steam_burst":2,"overheat":1}'::jsonb
		LIMIT 1`).Scan(&bid); err != nil {
		t.Fatalf("取战报 id 失败：%v", err)
	}
	mustExec(`UPDATE battle_records SET level_id = 2, reactions_used = '{"flash_freeze":4}'::jsonb WHERE id = $1`, bid)
	assertSame("更新后")
	if v := counterAgg()["steam_burst"]; v != 0 {
		t.Fatalf("更新后 steam_burst 应回到 0，实际 %d（旧增量没扣回就是这条断言要抓的）", v)
	}

	// 4) DELETE：计数必须扣回。
	mustExec(`DELETE FROM battle_records WHERE id = $1`, bid)
	assertSame("删除后")
	if v := counterAgg()["flash_freeze"]; v != 0 {
		t.Fatalf("删除后 flash_freeze 应回到 0，实际 %d", v)
	}

	// 5) 删用户级联删战报：计数同样要扣回，且表里不留非零或负值。
	insertBattle(3, `{"steam_burst":1}`)
	assertSame("级联前")
	mustExec(`DELETE FROM users WHERE id = $1`, uid)
	assertSame("级联删除后")
	var nonzero, negative int64
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FILTER (WHERE count <> 0), COUNT(*) FILTER (WHERE count < 0)
		  FROM dashboard_reaction_counts`).Scan(&nonzero, &negative); err != nil {
		t.Fatalf("查计数表失败：%v", err)
	}
	if nonzero != 0 || negative != 0 {
		t.Fatalf("级联删除后计数表应全部归零且无负值，实际非零 %d 行、负值 %d 行，残留 %v",
			nonzero, negative, counterAgg())
	}
}
