// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 9 轮：bumpTasks 的任务配置改走 tasksCache（与 LoadTasks 同一 loader）。
//
// tasks 是 seeder 静态配置，此前**每次结算都在事务内**查一遍 `tasks` 表。
// 这条测试用「改底表」判据钉住缓存真的在被使用：TTL 内把 tasks 表改名，
// bumpTasks 仍必须成功且进度真实落库 —— 若有代码路径还在查表，改名后必红。

import (
	"context"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

func TestBumpTasksServedFromCache(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 预热前先取一个日常击杀任务的 id/target（改名后就查不到了）。
	var taskID, target int
	var scope string
	if err := scratch.pool.QueryRow(ctx,
		`SELECT id, target, scope FROM tasks WHERE metric = 'kills' AND scope = 'daily' LIMIT 1`,
	).Scan(&taskID, &target, &scope); err != nil {
		t.Fatalf("种子任务里应有日常击杀任务：%v", err)
	}
	if target < 3 {
		t.Fatalf("种子 target=%d，测试按 ≥3 设计", target)
	}

	bump := func() error {
		return scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
			return scratch.bumpTasks(ctx, tx, uid, map[string]int64{
				"max_stage": 1, "clears": 1, "kills": 3,
			}, time.Now())
		})
	}
	// 首次调用预热缓存（读 tasks 表并落 TTL 缓存）。
	if err := bump(); err != nil {
		t.Fatalf("预热 bumpTasks 失败：%v", err)
	}
	// 清掉预热写入的进度，让改名后的断言确定（只有一次 bump 的增量）。
	scratch.exec(t, `DELETE FROM user_tasks WHERE user_id = $1`, uid)

	// TTL 内把 tasks 表改名：若 bumpTasks 还在查表，这里必然失败。
	scratch.renameTable(t, "tasks", "tasks_gone_bump")
	if err := bump(); err != nil {
		t.Fatalf("缓存期内 bumpTasks 不应再查 tasks 表：%v", err)
	}

	// 进度必须真实落库 —— 缓存改变的只是配置的读取来源。
	day := periodStart(time.Now(), "daily")
	var progress int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT progress FROM user_tasks WHERE user_id = $1 AND task_id = $2 AND task_date = $3`,
		uid, taskID, day).Scan(&progress); err != nil {
		t.Fatalf("击杀任务进度应已落库：%v", err)
	}
	if progress != 3 {
		t.Errorf("kills 增量 3 应写入进度，实际 %d", progress)
	}
}
