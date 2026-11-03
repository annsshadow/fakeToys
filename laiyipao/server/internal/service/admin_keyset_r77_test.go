// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R77：keyset（游标）分页的正确性与并发写入一致性。
//
// # 改前探针（1M 仿真库，laiyipao_r77_perf，真实端点）
//
//   /admin/users   offset=10k→42ms / 100k→147ms / 500k→525ms / 900k→595ms
//   /admin/battles offset=100k→245ms / 500k→556ms
//
// EXPLAIN 归因：深页时优化器放弃 PK 索引，走 Parallel Seq Scan +
// Hash Join + external merge Sort（每 worker 磁盘排序 18–23MB）——
// 先把 50 万行排出来再丢掉。keyset（WHERE id < 游标）把这个变成
// 索引范围扫描（~1ms）。
//
// 前提澄清（如实）：管理端**没有** wallet_flows 的 OFFSET 翻页端点
// （只有 /admin/economy 聚合与按用户导出）——纯 SQL 测得它的 OFFSET
// 曲线（100k→17ms / 500k→57ms / 900k→129ms）作存档；真实退化面是
// users/battles 两个列表，keyset 落在它们上面。
//
// # 一致性（用户要求的核心）
//
// keyset 相对 OFFSET 的本质优势：翻页中途有新行插入时，游标钉住
// 「id < X」这个固定边界，新行（id 更大）排在 DESC 序的**前部**，
// 永远不会进入后续页 —— 不重不漏。本文件用真实行证明，
// 并同时证明 OFFSET 在同一场景确实重行。

import (
	"context"
	"fmt"
	"testing"
)

// r77SeedUsers 直插 n 个用户（不走 GuestLogin —— 那是 7 张表的慢路径），
// 返回它们的 id 集合。tag 防唯一键与别用例冲突。
func r77SeedUsers(t *testing.T, ts *testService, ctx context.Context, tag string, n int) map[int64]bool {
	t.Helper()
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO users (guest_token, nickname)
		 SELECT $1 || '_' || g, 'KS_' || g FROM generate_series(1, $2) g`,
		tag, n); err != nil {
		t.Fatalf("批量造用户失败：%v", err)
	}
	rows, err := ts.pool.Query(ctx,
		`SELECT id FROM users WHERE guest_token LIKE $1`, tag+"_%")
	if err != nil {
		t.Fatalf("读回 id 失败：%v", err)
	}
	defer rows.Close()
	ids := map[int64]bool{}
	for rows.Next() {
		var id int64
		if err := rows.Scan(&id); err != nil {
			t.Fatalf("scan id 失败：%v", err)
		}
		ids[id] = true
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM users WHERE guest_token LIKE $1`, tag+"_%")
	})
	return ids
}

// r77WalkUsersKeyset 用游标走完全部页，返回收集到的 id 列表。
// kw 限定 nickname 范围：scratch 库里还有种子/引导用户，不限定会把它走进来。
func r77WalkUsersKeyset(t *testing.T, ts *testService, ctx context.Context, kw string, limit int, midway func(page int)) []int64 {
	t.Helper()
	var got []int64
	var cursor int64
	for page := 1; ; page++ {
		items, _, err := ts.AdminListUsers(ctx, kw, limit, 0, nil, nil, nil, nil, nil, nil, cursor)
		if err != nil {
			t.Fatalf("第 %d 页失败：%v", page, err)
		}
		for _, it := range items {
			got = append(got, it.ID)
		}
		if midway != nil {
			midway(page)
		}
		if len(items) < limit {
			return got
		}
		cursor = items[len(items)-1].ID
	}
}

// r77WalkUsersOffset 用 OFFSET 走完全部页（对照组）。
func r77WalkUsersOffset(t *testing.T, ts *testService, ctx context.Context, kw string, limit int, midway func(page int)) []int64 {
	t.Helper()
	var got []int64
	for page := 0; ; page++ {
		items, _, err := ts.AdminListUsers(ctx, kw, limit, page*limit, nil, nil, nil, nil, nil, nil)
		if err != nil {
			t.Fatalf("第 %d 页失败：%v", page, err)
		}
		for _, it := range items {
			got = append(got, it.ID)
		}
		if midway != nil {
			midway(page + 1)
		}
		if len(items) < limit {
			return got
		}
	}
}

func r77DupCount(got []int64) int {
	seen := map[int64]bool{}
	dups := 0
	for _, id := range got {
		if seen[id] {
			dups++
		}
		seen[id] = true
	}
	return dups
}

// TestKeysetUsersNoDupNoMissUnderWrites：并发写入下 keyset 不重不漏，
// 同场景 OFFSET 确实重行（对照证明优势是真实的）。
func TestKeysetUsersNoDupNoMissUnderWrites(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	const total, limit = 120, 25
	base := r77SeedUsers(t, ts, ctx, "r77ks_main", total)

	// —— keyset：翻完 2 页后插入 5 个新用户（id 更大，DESC 序在头部）——
	var got []int64
	pages := 0
	got = r77WalkUsersKeyset(t, ts, ctx, "KS_", limit, func(page int) {
		pages = page
		if page == 2 {
			r77SeedUsers(t, ts, ctx, "r77ks_new", 5)
		}
	})
	if pages < 5 {
		t.Fatalf("只翻了 %d 页，插入未发生在中途（夹具失效）", pages)
	}
	if d := r77DupCount(got); d != 0 {
		t.Errorf("keyset 翻页出现 %d 个重复 id", d)
	}
	if len(got) != total {
		t.Fatalf("keyset 收到 %d 行，期望恰好 %d（中途插入的 5 行不该混进来）", len(got), total)
	}
	for _, id := range got {
		if !base[id] {
			t.Fatalf("keyset 收到了不属于初始集合的 id=%d（中途插入行混入）", id)
		}
	}
	// 集合相等 = 不漏
	for id := range base {
		found := false
		for _, g := range got {
			if g == id {
				found = true
				break
			}
		}
		if !found {
			t.Fatalf("keyset 漏了 id=%d", id)
		}
	}
	t.Logf("keyset：%d 行恰好走完 %d 个初始用户，0 重复 0 遗漏（中途 +5 行未混入）", len(got), total)

	// —— OFFSET 对照：同一场景必须出现重复（否则「优势」只是嘴上说）——
	gotOffset := r77WalkUsersOffset(t, ts, ctx, "KS_", limit, func(page int) {
		if page == 2 {
			r77SeedUsers(t, ts, ctx, "r77ks_new2", 5)
		}
	})
	if d := r77DupCount(gotOffset); d == 0 {
		t.Fatalf("OFFSET 对照组在中途插入后**没有**出现重复行 —— 对照失效，优势无证据")
	} else {
		t.Logf("OFFSET 对照：同场景重复 %d 行（窗口随插入平移）—— 这正是 keyset 要消灭的", d)
	}
}

// TestKeysetBattlesMatchesOffset：战报列表 keyset 与 OFFSET 同集合同序。
func TestKeysetBattlesMatchesOffset(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	const n = 60
	for i := range n {
		if _, err := ts.pool.Exec(ctx,
			`INSERT INTO battle_records (user_id, level_id, result, replay_hash)
			 VALUES ($1, 1, 'lose', $2)`, uid, fmt.Sprintf("r77ks_%d", i)); err != nil {
			t.Fatalf("造战报失败：%v", err)
		}
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM battle_records WHERE user_id=$1 AND replay_hash LIKE 'r77ks_%'`, uid)
	})

	const limit = 20
	var viaOffset, viaKeyset []int64
	for off := 0; ; off += limit {
		items, _, err := ts.AdminListBattles(ctx, uid, 0, limit, false, off, nil, nil, "")
		if err != nil {
			t.Fatalf("offset 翻页失败：%v", err)
		}
		for _, it := range items {
			viaOffset = append(viaOffset, it.ID)
		}
		if len(items) < limit {
			break
		}
	}
	var cursor int64
	for page := 0; ; page++ {
		items, _, err := ts.AdminListBattles(ctx, uid, 0, limit, false, 0, nil, nil, "", cursor)
		if err != nil {
			t.Fatalf("keyset 翻页失败：%v", err)
		}
		for _, it := range items {
			viaKeyset = append(viaKeyset, it.ID)
		}
		if len(items) < limit {
			break
		}
		cursor = items[len(items)-1].ID
	}

	if len(viaOffset) != n || len(viaKeyset) != n {
		t.Fatalf("条数不符：offset=%d keyset=%d 期望 %d", len(viaOffset), len(viaKeyset), n)
	}
	for i := range viaOffset {
		if viaOffset[i] != viaKeyset[i] {
			t.Fatalf("第 %d 位分叉：offset=%d keyset=%d", i, viaOffset[i], viaKeyset[i])
		}
	}
	// total 语义不受游标影响（游标是位置不是筛选）
	_, totalCursor, err := ts.AdminListBattles(ctx, uid, 0, limit, false, 0, nil, nil, "", viaKeyset[10])
	if err != nil {
		t.Fatalf("带游标取 total 失败：%v", err)
	}
	if totalCursor != int64(n) {
		t.Errorf("带游标时 total=%d，期望 %d（游标不该改变总数口径）", totalCursor, n)
	}
}

// TestKeysetUsersTotalUnchangedByCursor：users 侧同口径钉 total。
func TestKeysetUsersTotalUnchangedByCursor(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	r77SeedUsers(t, ts, ctx, "r77ks_total", 30)

	items, total, err := ts.AdminListUsers(ctx, "", 10, 0, nil, nil, nil, nil, nil, nil)
	if err != nil || len(items) == 0 {
		t.Fatalf("首页失败：%v", err)
	}
	_, total2, err := ts.AdminListUsers(ctx, "", 10, 0, nil, nil, nil, nil, nil, nil, items[9].ID)
	if err != nil {
		t.Fatalf("游标页失败：%v", err)
	}
	if total != total2 {
		t.Errorf("total 随游标变了：%d → %d", total, total2)
	}
}
