package service

import (
	"context"
	"encoding/json"
	"sync"
	"testing"
	"time"
)

// `ClaimTask` 在**周任务**上的并发互斥（第 76 轮）。
//
// # 为什么已有守卫不够
//
// 既有守卫 `TestConcurrentClaimTaskOnlyOnce` 确实覆盖了
// 「删掉 `if tag.RowsAffected() == 0` 会导致并发重复发奖」——
// 变异实测它会红。**所以那部分覆盖是既有的，不是我补的。**
//
// 它缺的是**周任务**这一条路径：
// 该守卫的任务来自 `SELECT id FROM tasks WHERE enabled AND scope = 'daily'`，
// 是日任务。而第 76 轮动的正是 `day` 的计算（scope 相关），
// 若其中一处退回 daily，周任务会「一个请求写到 daily 行、
// 另一个匹配到 weekly 行」——**在日任务的守卫里完全看不出来**。
//
// 这是本文件存在的唯一理由：把同一个不变量在**两条 scope 路径**上都钉一遍。
//
// # 一条变异结果如实记录（无害）
//
// 删掉 `if claimedAt != nil` 的预检 → **全部守卫仍绿**。
// 因为那个预检只是快路径：真正��互斥点是下面的条件 UPDATE
// （`... AND claimed_at IS NULL` + `RowsAffected`）。
// 预检删掉后，重复领取会走到 UPDATE、拿到 0 行、被正确拒绝 —
// 只是多一次数据库往返。
//
// 变异确实无害，如实记录，不粉饰。它与 README 记的
// 「去掉雪崩只留 levelID × 奇数 → 均匀度测试全绿」同类。
//
// # 峰值断言是必需的
//
// `close(start)` 只是**释放门**，不是会合点：排在 close 之后
// 才被调度的 goroutine 会直接通过，没有任何重叠保证。
// 纯内存的 fn 很容易整体串行执行，而「只发一次」那条断言
// 在串行下**照样通过** —— 于是测试变成「可能撞上竞态的探测器」。
//
// 有了 peak，每条用例都能断言 peak >= 2，从而证明自己确实在并发。

// TestConcurrentClaimGrantsRewardOnce 并发领取只发一次奖励。
func TestConcurrentClaimGrantsRewardOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	raw, _ := json.Marshal(map[string]int{"coin": 1000})
	var taskID int
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO tasks (id, code, name, scope, target, metric, reward)
		 VALUES ($1,$2,'并发领取探针','daily',1,'kills',$3) RETURNING id`,
		nextTaskID(t, ts), "probe_conc_"+randomProbeName(), raw).Scan(&taskID); err != nil {
		t.Fatalf("造任务失败：%v", err)
	}
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		 VALUES ($1,$2,$3,1)`,
		uid, taskID, periodStart(time.Now(), "daily")); err != nil {
		t.Fatal(err)
	}

	const workers = 8
	before := ts.wallet(t, ctx, uid).Coin

	var wg sync.WaitGroup
	results := make([]error, workers)
	start := make(chan struct{})
	peak := make(chan int, workers)
	var mu sync.Mutex
	var inFlight, peakDepth int

	for i := 0; i < workers; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			<-start
			mu.Lock()
			inFlight++
			if inFlight > peakDepth {
				peakDepth = inFlight
			}
			mu.Unlock()

			_, err := ts.ClaimTask(ctx, uid, taskID)

			mu.Lock()
			inFlight--
			mu.Unlock()
			results[idx] = err
		}(i)
	}
	close(start)
	wg.Wait()
	peak <- peakDepth

	// ⚠️ 先断言「确实并发过」—— 否则本测试可能变成一个串行的空转。
	// `close(start)` 只是释放门，不是会合点。
	if p := <-peak; p < 2 {
		t.Fatalf("峰值并发 %d < 2 —— 本用例没有真的并发，"+
			"下面的断言可能是在串行下通过的（那不能证明互斥）", p)
	}

	ok := 0
	for _, err := range results {
		if err == nil {
			ok++
		}
	}
	if ok != 1 {
		t.Errorf("%d 个并发请求里有 %d 个成功领到奖励，期望恰好 1 个", workers, ok)
	}

	after := ts.wallet(t, ctx, uid).Coin
	if got := after - before; got != 1000 {
		t.Errorf("余额增量 = %d，期望 1000 —— 并发下奖励被发了 %d 次",
			got, got/1000)
	}

	var flows int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM wallet_flows
		  WHERE user_id = $1 AND reason = 'task_reward' AND ref_id = $2`,
		uid, taskID).Scan(&flows); err != nil {
		t.Fatal(err)
	}
	if flows != 1 {
		t.Errorf("task_reward 流水 %d 条，期望 1 条", flows)
	}
}

// TestConcurrentClaimOnWeeklyTaskIsAlsoOnce 覆盖周任务的并发。
//
// ⚠️ 必须单独一条：修复后进度查询与 UPDATE 的 `$3` 都是 scope 对应的 day。
// 若其中一处退回 daily，并发下会「一个请求写到 daily 行、
// 另一个匹配到 weekly 行」—— 而那在串行测试里完全看不出来。
func TestConcurrentClaimOnWeeklyTaskIsAlsoOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	raw, _ := json.Marshal(map[string]int{"coin": 1000})
	var taskID int
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO tasks (id, code, name, scope, target, metric, reward)
		 VALUES ($1,$2,'并发周领取探针','weekly',1,'kills',$3) RETURNING id`,
		nextTaskID(t, ts), "probe_concw_"+randomProbeName(), raw).Scan(&taskID); err != nil {
		t.Fatalf("造 weekly 任务失败：%v", err)
	}
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		 VALUES ($1,$2,$3,1)`,
		uid, taskID, periodStart(time.Now(), "weekly")); err != nil {
		t.Fatal(err)
	}

	const workers = 8
	before := ts.wallet(t, ctx, uid).Coin

	var wg sync.WaitGroup
	results := make([]error, workers)
	var mu sync.Mutex
	var inFlight, peakDepth int
	start := make(chan struct{})

	for i := 0; i < workers; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			<-start
			mu.Lock()
			inFlight++
			if inFlight > peakDepth {
				peakDepth = inFlight
			}
			mu.Unlock()

			_, err := ts.ClaimTask(ctx, uid, taskID)

			mu.Lock()
			inFlight--
			mu.Unlock()
			results[idx] = err
		}(i)
	}
	close(start)
	wg.Wait()

	if peakDepth < 2 {
		t.Fatalf("峰值并发 %d < 2 —— 本用例没有真的并发", peakDepth)
	}
	ok := 0
	for _, err := range results {
		if err == nil {
			ok++
		}
	}
	if ok != 1 {
		t.Errorf("%d 个并发请求里有 %d 个成功，期望恰好 1 个", workers, ok)
	}
	if got := ts.wallet(t, ctx, uid).Coin - before; got != 1000 {
		t.Errorf("余额增量 = %d，期望 1000", got)
	}

	// 本周期内，这个 (user, task) 应该**只有一行**进度记录。
	//
	// ⚠️ 旧写法按 `task_date = dailyDay` 查「daily 那一行是否存在」，
	// 但在**周一** daily 与 weekly 的 periodStart 完全相等，
	// 那条 SQL 会把 weekly 行自己也数进去 → 必然红。
	// （第 150 轮踩过：README 记的「全绿」那天是周六。）
	//
	// 改成「总行数 == 1」后：
	//   - 周一：weekly 行就是 daily 行，总数 1，断言通过；
	//   - 其余 6 天：若代码某处把 day 写死成 daily，会多出一行
	//     → 总数 2 → 断言红。
	// 周一「多写一行 daily」与 weekly 行撞主键 (user,task,date)，
	// 无法被行数观测 —— 这条 Monday 盲区由
	// `TestPeriodStartWeeklyNeverCollapsesToDaily` 单独钉住。
	//
	// 多一行 daily 的另一种形态（只写 daily、不写 weekly）会被
	// 上面的 `ok != 1`（领取失败）先抓住，这里不必重复。
	var totalRows int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_tasks
		  WHERE user_id = $1 AND task_id = $2`,
		uid, taskID).Scan(&totalRows); err != nil {
		t.Fatal(err)
	}
	if totalRows != 1 {
		t.Errorf("user_tasks 里出现了 %d 行（期望 1）—— "+
			"代码某处为 weekly 任务多写了一行（可能把 day 写死成了 daily）",
			totalRows)
	}
}

// TestPeriodStartWeeklyNeverCollapsesToDaily 把「weekly 与 daily 的周期起点不同」
// 这件事本身钉成不变量，而不是让它散落在各处的调用方心里。
//
// 这条**直接**排除掉第 150 轮那个「周一必然红」的前提：
// 在**非周一**的时间点上，weekly 起点必须严格早于 daily 起点。
//
// 为什么用函数指针注入时间：`periodStart` 读的是传进来的 `t`，
// 所以这里根本不需要改生产代码签名 ——
// 直接给一个固定的周二即可，连时钟都不用 mock。
func TestPeriodStartWeeklyNeverCollapsesToDaily(t *testing.T) {
	// 覆盖一整周七天，逐日断言而不只看今天 ——
	// 只测「今天」的话，周一那天必然漏掉（那正是原缺陷藏身的地方）。
	base := time.Date(2026, 10, 12, 13, 45, 0, 0, time.UTC) // 周一
	for d := 0; d < 7; d++ {
		now := base.AddDate(0, 0, d)
		daily := periodStart(now, "daily")
		weekly := periodStart(now, "weekly")
		if weekly.After(daily) {
			t.Errorf("%s（周%s）：weekly 起点 %s 晚于 daily 起点 %s",
				now.Format("2006-01-02"), now.Weekday(), weekly, daily)
		}
		if d == 0 {
			// 周一：两者**必须**相等 —— 这正是原缺陷成立的条件。
			// 把它写成断言（而不是只写注释），是为了让"周一相等"这条事实
			// 在有人改动 periodStart 时立刻响，而不是悄悄变成另一种形态。
			if !weekly.Equal(daily) {
				t.Errorf("周一的 weekly/daily 起点应相等，实际 %s vs %s", weekly, daily)
			}
		} else {
			if weekly.Equal(daily) {
				t.Errorf("%s（周%s）：weekly 起点与 daily 起点相同 —— "+
					"「daily 行不存在」类断言在这一天不可判定", now.Format("2006-01-02"), now.Weekday())
			}
			// weekly 必须落在**之前**的某个周一
			if diff := daily.Sub(weekly); diff <= 0 || diff > 7*24*time.Hour {
				t.Errorf("%s：weekly 起点 %s 与 daily 相差 %v，不在 (0, 7天] 内",
					now.Format("2006-01-02"), weekly, diff)
			}
		}
	}
}
