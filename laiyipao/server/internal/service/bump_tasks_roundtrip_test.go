// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R147：bumpTasks 的往返次数守卫。
//
// # 为什么要有这条守卫
//
// 改前：命中 N 个任务 → N 次 `tx.Exec`（每个任务一次往返）。
// 1100 万用户库实测（c=8）：tx_tasks 段落 11.7ms，而
// `pg_stat_statements` 里那条 upsert SQL 只有 0.035–0.039ms × 11 ≈ 0.41ms
// ——**问题不在 SQL，在往返次数**。
//
// 合并成一条多行 upsert 后，往返固定为 1，与命中任务数无关。
// 这条用例把「与命中数无关的常数往返」钉住：有人把 SQL 改回循环时，
// execs 会重新变成命中数（种子下是 11），必红。
//
// ⚠️ 破坏实验（改前应红、改后绿）：
// 把本文件断言里的 `execs > 1` 判据拿去跑旧实现 —— 11 次 Exec 直接触发。

import (
	"context"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
)

// countingTx 只统计「发了多少次请求」，不做语义拦截。
type countingTx struct {
	pgx.Tx
	execs   int
	queries int
	batches int
}

func (c *countingTx) Exec(ctx context.Context, sql string, args ...any) (pgconn.CommandTag, error) {
	c.execs++
	return c.Tx.Exec(ctx, sql, args...)
}

func (c *countingTx) Query(ctx context.Context, sql string, args ...any) (pgx.Rows, error) {
	c.queries++
	return c.Tx.Query(ctx, sql, args...)
}

func (c *countingTx) SendBatch(ctx context.Context, b *pgx.Batch) pgx.BatchResults {
	c.batches++
	return c.Tx.SendBatch(ctx, b)
}

// settleMetrics 是结算路径实际传的四个 metric（game.go SettleBattle）。
var settleMetrics = []string{"kills", "clears", "reactions", "max_stage"}

// TestBumpTasksOneRoundTripPerSettle：一次结算的任务进度写入必须是
// **一条语句**（往返 1），且命中多个任务行。
func TestBumpTasksOneRoundTripPerSettle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 预热任务配置缓存：否则 taskConfigsForTx 会多一条 Query（那是缓存冷时的
	// 正常成本，与本次要钉住的「写入往返」无关）。
	configs, err := ts.loadTaskConfigs(ctx)
	if err != nil {
		t.Fatalf("加载任务配置失败：%v", err)
	}
	ts.tasksCache.put(configs, time.Now())

	// 前置：本轮种子下四个 metric 命中多少任务行 —— 必须 > 1，
	// 否则「N → 1」这个判据是空洞的。
	var hits int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM tasks WHERE enabled AND metric = ANY($1)`, settleMetrics).
		Scan(&hits); err != nil {
		t.Fatalf("统计命中任务失败：%v", err)
	}
	if hits < 2 {
		t.Fatalf("结算四个 metric 只命中 %d 个任务 —— 本用例前提不成立（无法区分 1 次与 N 次往返）", hits)
	}

	tx, err := ts.pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开事务失败：%v", err)
	}
	defer tx.Rollback(ctx)

	ct := &countingTx{Tx: tx}
	if err := ts.bumpTasks(ctx, ct, uid, map[string]int64{
		"kills": 3, "clears": 1, "reactions": 1, "max_stage": 1,
	}, time.Now()); err != nil {
		t.Fatalf("bumpTasks 失败：%v", err)
	}

	if ct.execs != 1 || ct.queries != 0 || ct.batches != 0 {
		t.Errorf("bumpTasks 应只用 1 条语句（往返 1）：实际 Exec=%d Query=%d Batch=%d（命中 %d 个任务）。\n"+
			"往返数必须与命中数无关；回到逐任务循环时这里会变成 %d 次 Exec。",
			ct.execs, ct.queries, ct.batches, hits, hits)
	}
}

// TestBumpTasksMergedSemantics：合并成一条语句后，两类语义必须**逐行**保持：
//
//   - achievement：GREATEST（可超过 target，progress 是「历史最大值」）
//   - daily/weekly：累加并截断到**每行自己的** target
//     （不能串到别的任务上 —— cap 按 task_id 对齐取，不是按行位置）
//
// ⚠️ 这条能抓到的典型错法：cap 数组与输出行错位 ——
// e.g. daily_kills_30 拿到 weekly_kills_200 的 cap=200。
func TestBumpTasksMergedSemantics(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	now := time.Now()

	bump := func(d map[string]int64) {
		t.Helper()
		if err := ts.DB.Tx(ctx, func(tx pgx.Tx) error {
			return ts.bumpTasks(ctx, tx, uid, d, now)
		}); err != nil {
			t.Fatalf("bumpTasks(%v) 失败：%v", d, err)
		}
	}

	bump(map[string]int64{"kills": 31, "clears": 1, "reactions": 150, "max_stage": 20})
	bump(map[string]int64{"kills": 100, "clears": 1, "reactions": 1, "max_stage": 60})

	// 读回该用户所有进度行（含 scope，用来在 Go 侧按 periodStart 过滤，
	// 不把周期规则复制进 SQL）。
	rows, err := ts.pool.Query(ctx, `
		SELECT t.code, t.scope, ut.task_date, ut.progress
		  FROM tasks t
		  JOIN user_tasks ut ON ut.task_id = t.id
		 WHERE ut.user_id = $1`, uid)
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()
	got := map[string]int{}
	for rows.Next() {
		var code, scope string
		var day time.Time
		var progress int
		if err := rows.Scan(&code, &scope, &day, &progress); err != nil {
			t.Fatal(err)
		}
		wantDay := periodStart(now, scope)
		// task_date 是 date 列，扫出来是 UTC 午夜；periodStart 返回业务时区午夜。
		// 比日历日（不是比瞬时），避免时区把同一行判成「非本期」。
		if day.Format("2006-01-02") != wantDay.Format("2006-01-02") {
			continue // 非本期行（换日边界等）不参与断言
		}
		got[code] = progress
	}
	if err := rows.Err(); err != nil {
		t.Fatal(err)
	}

	want := map[string]int{
		"daily_kills_30":       30,  // 31 → 截断 30；+100 仍是 30
		"weekly_kills_200":     131, // 31+100，未到 200
		"daily_clears_3":       2,   // 1+1
		"weekly_clears_20":     2,   // 1+1
		"ach_first_clear":      1,   // GREATEST(1,1)：成就不累加
		"ach_reactions_100":    150, // GREATEST(150,1)：**超过 target 也不截断**
		"ach_reach_20":         60,  // GREATEST(20,60)
		"ach_reach_60":         60,  // 成就不按 target 截断
		"ach_reach_100":        60,  // **60 < target 100 也照样写 60**（无 cap）
		"daily_reactions_20":   20,  // 150 已截断到 20；+1 仍是 20
		"weekly_reactions_150": 150, // 150 已达 cap；+1 仍是 150
		"daily_signin":         0,   // 未签到：不应被结算写入
	}
	for code, w := range want {
		if got[code] != w {
			t.Errorf("%s: progress=%d（缺行时为 0），期望 %d", code, got[code], w)
		}
	}
}
