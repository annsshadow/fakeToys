package service

import (
	"context"
	"encoding/json"
	"errors"
	"strings"
	"testing"
	"time"
)

// 周任务的可领取性（第 76 轮）。
//
// # 缺陷：`ClaimTask` 的 `task_date` 写死成 daily
//
// 原实现：
//
//	LEFT JOIN user_tasks ut
//	       ON ut.task_id = t.id AND ut.user_id = $1
//	      AND ut.task_date = periodStart(time.Now(), "daily")   ← 写死
//
// 而进度是按**任务自己的 scope** 记的：
//
//	bumpTasks:  day := periodStart(now, t.scope)     // weekly → 本周一
//
// 于是周任务的进度行 `task_date = 本周一`，
// 而 ClaimTask 去找 `task_date = 今天零点` → 找不到
// → `COALESCE(progress,0) = 0` → `progress < target`
// → 报「任务未完成（0/N）」，**一次都领不到**。
//
// # 为什么它「看起来是能领的」
//
// `LoadTasks(ctx, uid, "weekly")` 用的是 `periodStart(now, scope)`（正确），
// 所以 UI 上那个周任务显示 `progress = target`、可领取 ——
// 点下去报「未完成」。
//
// **判别依据必须是端到端的一致性，不是某一个函数算对了。**
// 三处 periodStart（bump / load / claim）必须给出同一个 day，
// 而这一点只有跑通「记进度 -> 列任务 -> 领取」全链路才验得出来。

// upsertWeeklyTask 造一条 weekly 任务，返回 task_id。
func upsertWeeklyTask(t *testing.T, ts *testService, target int, reward map[string]int) int {
	t.Helper()
	raw, err := json.Marshal(reward)
	if err != nil {
		t.Fatal(err)
	}
	var id int
	if err := ts.pool.QueryRow(context.Background(),
		`INSERT INTO tasks (id, code, name, scope, target, metric, reward)
		 VALUES ($1, $2, $3, 'weekly', $4, 'kills', $5) RETURNING id`,
		nextTaskID(t, ts), "probe_w_"+randomProbeName(), "周任务探针",
		target, raw).Scan(&id); err != nil {
		t.Fatalf("造 weekly 任务失败：%v", err)
	}
	return id
}

func randomProbeName() string {
	return itoa64(time.Now().UnixNano())
}

// nextTaskID 造一个不与现有任务冲突的 id。
//
// ⚠️ `tasks.id` 是 `INTEGER PRIMARY KEY` 而不是 `SERIAL` —— 种子数据
// 显式给了 id（1..N）。第一版我写了 `RETURNING id` 让 PG 自己分配，
// PG 于是往 NOT NULL 的 id 上塞了个 NULL：
//
//	null value in column "id" of relation "tasks" violates not-null constraint
//
// 也就是说「让数据库代劳」在这张表上根本不成立 —— 与 `shop_items` 不同。
func nextTaskID(t *testing.T, ts *testService) int {
	t.Helper()
	var maxID int
	if err := ts.pool.QueryRow(context.Background(),
		`SELECT COALESCE(MAX(id), 0) FROM tasks`).Scan(&maxID); err != nil {
		t.Fatalf("读 max(task.id) 失败：%v", err)
	}
	return maxID + 1
}

// bumpDirectly 模拟 battle.go 里 bumpTasks 的行为：
// 按任务自己的 scope 算周期起点，写入进度。
func bumpDirectly(t *testing.T, ts *testService, userID int64, taskID, value int) {
	t.Helper()
	day := periodStart(time.Now(), "weekly")
	if _, err := ts.pool.Exec(context.Background(),
		`INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		 VALUES ($1,$2,$3,$4)
		 ON CONFLICT (user_id, task_id, task_date)
		 DO UPDATE SET progress = GREATEST(user_tasks.progress, EXCLUDED.progress)`,
		userID, taskID, day, value); err != nil {
		t.Fatalf("写进度失败：%v", err)
	}
}

// TestWeeklyTaskIsClaimable 是本文件的核心 —— 端到端。
func TestWeeklyTaskIsClaimable(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	beforeCoin := ts.wallet(t, ctx, uid).Coin

	const target = 5
	id := upsertWeeklyTask(t, ts, target, map[string]int{"coin": 1234})
	bumpDirectly(t, ts, uid, id, target)

	// 第一步：LoadTasks 必须显示「已完成」
	// —— 这条在修复前**本来就是绿的**（它算对了 day）。
	// 留着它是为了记录「UI 与领取曾不一致」这个事实。
	views, err := ts.LoadTasks(ctx, uid, "weekly")
	if err != nil {
		t.Fatalf("LoadTasks 失败：%v", err)
	}
	var found *TaskView
	for i := range views {
		if views[i].ID == id {
			found = &views[i]
		}
	}
	if found == nil {
		t.Fatalf("weekly 任务列表里没有 id=%d 的那条", id)
	}
	if !found.Done {
		t.Fatalf("LoadTasks 说它没完成：progress=%d target=%d —— "+
			"而 ClaimTask 会用另一个 day 去找，两处不一致", found.Progress, found.Target)
	}

	// 第二步：领取必须成功
	got, err := ts.ClaimTask(ctx, uid, id)
	if err != nil {
		t.Fatalf("周任务领取失败：%v\n"+
			"这正是修复前的症状：JOIN 用 daily 的 day 去匹配 weekly 的进度行", err)
	}
	if got["coin"] != 1234 {
		t.Errorf("奖励 = %v，期望 coin=1234", got)
	}

	// 第三步：查余额确认真的发了
	//
	// ⚠️ 必须量**增量**而不是绝对值 —— `newUser` 会送一笔初始余额
	// （实测 2000），我第一版断言 `== 1234`，红得莫名其妙。
	if got := ts.wallet(t, ctx, uid).Coin - beforeCoin; got != 1234 {
		t.Errorf("余额增量 = %d，期望 1234（奖励没真的到账）", got)
	}

	// 第四步：不能重复领
	if _, err := ts.ClaimTask(ctx, uid, id); !errors.Is(err, ErrForbidden) {
		t.Errorf("重复领取应报 ErrForbidden，实得 %v", err)
	}
	var flows int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1 AND reason = 'task_reward'`,
		uid).Scan(&flows); err != nil {
		t.Fatal(err)
	}
	if flows != 1 {
		t.Errorf("task_reward 流水有 %d 条，期望 1 —— 重复领取漏了", flows)
	}
}

// TestDailyTaskStillClaimable 确认没把日任务弄坏。
//
// ⚠️ 这条**必须在**：修复动的是共用代码路径，
// 而「日任务还正常」不是周任务测试能证明的。
func TestDailyTaskStillClaimable(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	raw, _ := json.Marshal(map[string]int{"coin": 77})
	var id int
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO tasks (id, code, name, scope, target, metric, reward)
		 VALUES ($1,$2,'日任务探针','daily',2,'kills',$3) RETURNING id`,
		nextTaskID(t, ts), "probe_d_"+randomProbeName(), raw).Scan(&id); err != nil {
		t.Fatalf("造 daily 任务失败：%v", err)
	}

	day := periodStart(time.Now(), "daily")
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		 VALUES ($1,$2,$3,2)`, uid, id, day); err != nil {
		t.Fatal(err)
	}
	beforeCoin := ts.wallet(t, ctx, uid).Coin
	if _, err := ts.ClaimTask(ctx, uid, id); err != nil {
		t.Fatalf("日任务领取失败：%v", err)
	}
	if got := ts.wallet(t, ctx, uid).Coin - beforeCoin; got != 77 {
		t.Errorf("余额增量 = %d，期望 77", got)
	}
}

// TestWeeklyTaskNotClaimableBeforeProgress 守住「没做完就是不能领」。
//
// 修复的方向性风险：把 day 改成 weekly 之后，
// 会不会变成「只要有 weekly 行就能领」？这条排除那种退化。
func TestWeeklyTaskNotClaimableBeforeProgress(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	const target = 5
	beforeCoin := ts.wallet(t, ctx, uid).Coin
	id := upsertWeeklyTask(t, ts, target, map[string]int{"coin": 999})
	bumpDirectly(t, ts, uid, id, target-1)

	_, err := ts.ClaimTask(ctx, uid, id)
	if !errors.Is(err, ErrForbidden) {
		t.Fatalf("未完成（%d/%d）时应当报 ErrForbidden，实得 %v", target-1, target, err)
	}
	// 错误信息必须带真实进度，而不是被 day 错配出来的 0
	if err != nil && !strings.Contains(err.Error(), "4/5") {
		t.Errorf("错误信息应报告真实进度 4/5，实得 %v —— "+
			"报 0/N 说明又退回到了「找不到行」的形态", err)
	}
	if got := ts.wallet(t, ctx, uid).Coin - beforeCoin; got != 0 {
		t.Errorf("未完成却发了奖励，余额增量 %d", got)
	}
}

// TestClaimIgnoresOtherPeriodsRow 覆盖「周期隔离」。
//
// 上一周（对 weekly 就是 7 天前）的进度行还在库里。
// 领取必须**只看本周期**那一行，否则上周做完这周白送。
func TestClaimIgnoresOtherPeriodsRow(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	const target = 5
	id := upsertWeeklyTask(t, ts, target, map[string]int{"coin": 555})

	// 只写「上周」的进度
	oldDay := periodStart(time.Now(), "weekly").AddDate(0, 0, -7)
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		 VALUES ($1,$2,$3,$4)`, uid, id, oldDay, target); err != nil {
		t.Fatal(err)
	}

	if _, err := ts.ClaimTask(ctx, uid, id); !errors.Is(err, ErrForbidden) {
		t.Fatalf("只有上周的进度时不该能领，实得 %v", err)
	}

	// 本周期补上进度后就能领
	beforeCoin := ts.wallet(t, ctx, uid).Coin
	bumpDirectly(t, ts, uid, id, target)
	if _, err := ts.ClaimTask(ctx, uid, id); err != nil {
		t.Fatalf("本周期完成后领取失败：%v", err)
	}
	if got := ts.wallet(t, ctx, uid).Coin - beforeCoin; got != 555 {
		t.Errorf("余额增量 = %d，期望 555", got)
	}
}

// TestClaimWritesClaimedAtOnThePeriodRow 钉住「领了哪一行」。
//
// 修复后 UPDATE 的 `$3` 是 scope 对应的 day。
// 若它退回 daily，周任务会「领了但 claimed_at 写到另一行」——
// 于是下次 ClaimTask 又看到 claimed_at = nil，**能重复领**。
func TestClaimWritesClaimedAtOnThePeriodRow(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	const target = 5
	id := upsertWeeklyTask(t, ts, target, map[string]int{"coin": 333})
	bumpDirectly(t, ts, uid, id, target)

	// 同时造一行 daily 的干扰数据（task_date = 今天零点）
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		 VALUES ($1,$2,$3,$4)
		 ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET progress = 999`,
		uid, id, periodStart(time.Now(), "daily"), target); err != nil {
		t.Fatal(err)
	}

	if _, err := ts.ClaimTask(ctx, uid, id); err != nil {
		t.Fatalf("领取失败：%v", err)
	}

	var claimedAt *time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT claimed_at FROM user_tasks
		  WHERE user_id = $1 AND task_id = $2 AND task_date = $3`,
		uid, id, periodStart(time.Now(), "weekly")).Scan(&claimedAt); err != nil {
		t.Fatalf("读周任务那一行失败：%v", err)
	}
	if claimedAt == nil {
		t.Fatal("claimed_at 写到了错误的行 —— 本周期的行仍是未领取，" +
			"下次 ClaimTask 会再发一次奖励")
	}
}
