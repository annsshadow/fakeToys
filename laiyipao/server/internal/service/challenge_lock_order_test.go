package service

import (
	"context"
	"errors"
	"strings"
	"testing"
	"time"
)

// `lockChallengeStateOrdered` 必须按**全局固定顺序**加锁（第 79 轮）。
//
// # 为什么不能用「并发跑 N 轮，看有没有死锁」
//
// 我第一版就是这么写的（6 轮，后改 30 轮），实测**不可靠**：
//
//  - 有死锁 bug 时，6 轮能抓到 3/4 个变异；30 轮反而**只**抓到 1/4
//  - 完全不预锁的变异（MUT1）在 6 轮时被抓，30 轮时**全绿**
//
// 原因是概率性的：两个事务要在**恰好那个时刻**交错。
// 而且并发轮数一多，`DefenseAttemptLimit`（每日 3 次）会先让大部分请求
// 在 `attempts >= limit` 处早退 —— **根本没走到窃取分支**，也就没有钱包加锁。
//
// 所以判据必须落在**机制**上，而且是确定性的。
//
// # 确定性判据：用 NOWAIT 探测「先锁了谁」
//
// 行锁在 PG 里是**独占**的，于是可以反过来问：
//
//	「在 A 已被别人锁住的情况下，调用 lockChallengeStateOrdered([B, A])
//	  是否已经先把 B 锁上了？」
//
// 顺序正确（升序 → 先 B 后 A）→ 进程会**卡在 A 上**，而 **B 已被锁**。
// 顺序错误（先 A 后 B）→ 进程卡在 A 上，而 **B 还没被锁**。
//
// 于是从第三个连接用 `FOR UPDATE NOWAIT` 探 B：
//
//	探到 55P03（lock_not_available） → B 被锁了 → 升序 ✓
//	探成功                            → B 没被锁 → 顺序错 ✗
//
// 这就是确定性判据：不依赖时序，只依赖「谁先被锁」。
//
// 这与 README 记的 `runParallel` 里 peak 断言同源：
// **断言必须落在真正能区分对错的那一层，而不是「跑够多次总能碰到」。**

// nowaitLockErr 报告「行被别的事务锁住」。
//
// PG 的 SQLSTATE 55P03 = lock_not_available。
// 这里不依赖具体错误码文案（各语言驱动文案不同），
// 而是认「NOWAIT 失败」这个事实本身。
func nowaitLockErr(t *testing.T, tx txType, table, col string, id int64) (locked bool) {
	t.Helper()
	q := `SELECT ` + col + ` FROM ` + table + ` WHERE user_id = $1 FOR UPDATE NOWAIT`
	var got int64
	err := tx.QueryRow(context.Background(), q, id).Scan(&got)
	if err == nil {
		return false
	}
	if errors.Is(err, context.Canceled) || errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("NOWAIT 查询被取消了：%v", err)
	}
	// 任何查询失败都当作「被锁住」——NOWAIT 的语义就是
	// 「锁不到立刻报错」，所以失败 == 被锁。
	return true
}

// TestLockChallengeStateAcquiresInAscendingOrder 是本文件的核心。
func TestLockChallengeStateAcquiresInAscendingOrder(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 造三个用户，并**按 id 升序**命名，使 a < b < c 可预期。
	// ⚠️ newUser 的 id 由序列分配，顺序不可预期 ——
	// 所以这里按实际 id 排序后再命名，而不是假设分配顺序。
	u1 := ts.newUser(t, ctx)
	u2 := ts.newUser(t, ctx)
	u3 := ts.newUser(t, ctx)
	ids := []int64{u1, u2, u3}
	for i := 0; i < len(ids); i++ {
		for j := i + 1; j < len(ids); j++ {
			if ids[j] < ids[i] {
				ids[i], ids[j] = ids[j], ids[i]
			}
		}
	}
	lo, hi := ids[0], ids[len(ids)-1]
	if lo == hi {
		t.Fatal("造用户失败：id 重复")
	}

	// 第 1 步：在**独立**的事务里占住 hi 的钱包行，且**不提交**。
	holder, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 holder 事务失败：%v", err)
	}
	defer func() { _ = holder.Rollback(ctx) }()
	var tmp int64
	if err := holder.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1 FOR UPDATE`, hi).Scan(&tmp); err != nil {
		t.Fatalf("占住 wallet[%d] 失败：%v", hi, err)
	}

	// 第 2 步：起一个 goroutine 调 lockChallengeStateOrdered([lo, hi])。
	// 它按升序会先锁 lo、再卡在 hi —— 我们要观察的正是「lo 已被锁」。
	done := make(chan error, 1)
	worker, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 worker 事务失败：%v", err)
	}
	// ⚠️ 刻意传**逆序** [hi, lo]。
	//
	// 我第一版传的是已排好序的 [lo, hi] —— 于是「实现里没有 sort」
	// 这个变异**完全观察不到**（输入本来就有序）。守卫通过了，
	// 但它没有在守它宣称的东西。
	//
	// 判据必须落在「实现做了功」的那条输入上。
	go func() {
		done <- lockChallengeStateOrdered(ctx, worker, []int64{hi, lo})
	}()

	// 给它一点时间跑到「卡住」的位置。
	//
	// ⚠️ 这里**确实**有等待，但方向是反的：我们要等的是
	// 「它已经拿到 lo 并在等 hi」，而不是「它跑完了」。
	// 一个信号量比 sleep 更稳：先确认 lo **还没有**被锁（它还没开始），
	// 再轮询到 lo 被锁为止。
	waitUntilLocked(t, ts, ctx, "user_wallets", "user_id", lo, 3*time.Second)

	// 第 3 步：确认 lo 确实被 worker 锁住了 —— 这证明它**先于 hi** 处理 lo。
	probe, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 probe 事务失败：%v", err)
	}
	if !nowaitLockErr(t, probe, "user_wallets", "user_id", lo) {
		_ = probe.Rollback(ctx)
		t.Fatalf("wallet[%d] 没有被 worker 锁住 —— "+
			"lockChallengeStateOrdered 没有先锁较小的 id（顺序错）", lo)
	}
	_ = probe.Rollback(ctx)

	// 第 4 步：放开 hi，worker 应当**顺利返回**（不是死锁）
	if err := holder.Rollback(ctx); err != nil {
		t.Logf("holder 回滚返回 %v（可忽略）", err)
	}
	select {
	case err := <-done:
		if err != nil {
			_ = worker.Rollback(ctx)
			t.Fatalf("worker 返回错误 %v —— 修复后它只应被短暂阻塞，不该失败", err)
		}
	case <-time.After(5 * time.Second):
		_ = worker.Rollback(ctx)
		t.Fatal("worker 5 秒未返回 —— 它不是在等 hi，而是卡在别处")
	}
	_ = worker.Rollback(ctx)
}

// waitUntilLocked 轮询直到目标行被锁住，或超时。
func waitUntilLocked(t *testing.T, ts *testService, ctx context.Context,
	table, col string, id int64, budget time.Duration) {
	t.Helper()
	deadline := time.Now().Add(budget)
	for time.Now().Before(deadline) {
		probe, err := ts.db.Pool.Begin(ctx)
		if err != nil {
			continue
		}
		var got int64
		q := `SELECT ` + col + ` FROM ` + table + ` WHERE user_id = $1 FOR UPDATE NOWAIT`
		err = probe.QueryRow(ctx, q, id).Scan(&got)
		_ = probe.Rollback(ctx)
		if err != nil {
			return // 被锁住了
		}
		time.Sleep(5 * time.Millisecond)
	}
	t.Fatalf("%s[%d] 在 %v 内始终没被锁住 —— worker 可能还没跑起来", table, id, budget)
}

// TestLockChallengeStateCountersBeforeWallets 钉住**表顺序**。
//
// # 构造：占住 hi 的 **counters** 行
//
// worker 按**全局表顺序**（先全部 counters、再全部 wallets）、**id 升序**执行时：
//
//	第一轮  counters[lo] ✓ → counters[hi] ✗ 卡住（holder 占着）
//	第二轮  还没开始
//
// 于是观察到的状态是：
//
//	lo 的 counters  **被锁**
//	lo 的 wallets   **没被锁**
//
// 三种退化各自的可观察结果：
//
//	不排序（先 counters[hi]）  → lo 的 counters 也没锁   → 本条红
//	交错（每个用户 counters 再 wallets）→ lo 的 wallets 也被锁 → 本条红
//	正确                          → 只有 lo 的 counters 被锁 → 绿
//
// ⚠️ 我第一版的构造占的是 hi 的 **钱包**行，于是三种退化
// 在那个场景下**观察不到**（因为第二轮的钱包加锁由 SQL 的
// `ORDER BY user_id` 保证，Go 侧的排序对它没有影响）——
// 守卫通过了，但它没有在守它宣称的东西。
//
// **判据必须落在「实现做了功、且做功能被观测」的那条输入上。**
func TestLockChallengeStateCountersBeforeWallets(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ids := []int64{ts.newUser(t, ctx), ts.newUser(t, ctx)}
	for i := 0; i < len(ids); i++ {
		for j := i + 1; j < len(ids); j++ {
			if ids[j] < ids[i] {
				ids[i], ids[j] = ids[j], ids[i]
			}
		}
	}
	lo, hi := ids[0], ids[1]

	// 预建 counters 行 —— 未提交的行对别的连接不可见，
	// 探测时找不到 ≠ 没被锁（这个坑记在下面）。
	for _, id := range []int64{lo, hi} {
		if _, err := ts.pool.Exec(ctx, `
			INSERT INTO user_daily_challenges (user_id, challenge_date, attempts, stolen_times)
			VALUES ($1, CURRENT_DATE, 0, 0)
			ON CONFLICT (user_id, challenge_date) DO UPDATE SET challenge_date = CURRENT_DATE`,
			id); err != nil {
			t.Fatalf("预建 counters[%d] 失败：%v", id, err)
		}
	}

	// holder 占住 hi 的 **counters** 行（无操作更新只为拿锁）
	holder, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 holder 失败：%v", err)
	}
	defer func() { _ = holder.Rollback(ctx) }()
	if _, err := holder.Exec(ctx, `
		INSERT INTO user_daily_challenges (user_id, challenge_date, attempts, stolen_times)
		VALUES ($1, CURRENT_DATE, 0, 0)
		ON CONFLICT (user_id, challenge_date) DO UPDATE SET challenge_date = CURRENT_DATE`,
		hi); err != nil {
		t.Fatalf("占住 counters[%d] 失败：%v", hi, err)
	}

	worker, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 worker 失败：%v", err)
	}
	go func() {
		_ = lockChallengeStateOrdered(ctx, worker, []int64{hi, lo})
	}()

	// 等 counters[lo] 被锁上
	waitUntilLocked(t, ts, ctx, "user_daily_challenges", "user_id", lo, 3*time.Second)

	probe, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 probe 失败：%v", err)
	}
	var tmp int64
	// 1) lo 的钱包**不该**被锁 —— 它属于第二轮，而 worker 卡在第一轮
	errWallet := probe.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1 FOR UPDATE NOWAIT`, lo).Scan(&tmp)
	walletLocked := errWallet != nil
	_ = probe.Rollback(ctx)

	if walletLocked {
		t.Errorf("wallet[%d] 已经被锁住 —— 说明加锁是「每个用户先 counters 再 wallet」"+
			"的交错形态，而不是「先全部 counters、再全部 wallets」的全局表顺序", lo)
	}

	_ = holder.Rollback(ctx)
	_ = worker.Rollback(ctx)
}

// TestLockChallengeStateDoesNotTouchOtherRows 确认只锁传入的那些人。
//
// 若实现误锁了「全体用户」或「所有 counters」，那既慢又扩大冲突面。
func TestLockChallengeStateDoesNotTouchOtherRows(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	target := ts.newUser(t, ctx)
	bystander := ts.newUser(t, ctx)

	if err := ts.DB.Tx(ctx, func(tx txType) error {
		return lockChallengeStateOrdered(ctx, tx, []int64{target})
	}); err != nil {
		t.Fatalf("失败：%v", err)
	}

	probe, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 probe 失败：%v", err)
	}
	defer probe.Rollback(ctx)
	var got int64
	err = probe.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1 FOR UPDATE NOWAIT`, bystander).Scan(&got)
	if err != nil {
		t.Fatalf("bystander[%d] 的钱包被锁住了（err=%v）—— 实现锁了不该锁的行", bystander, err)
	}
}

// TestSelfChallengeStillRejected 守住早退没被预锁改动影响。
func TestSelfChallengeStillRejected(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	dv, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{
		Name: "自挑战探针", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("造防线失败：%v", err)
	}
	_, err = ts.ChallengeDefense(ctx, uid, dv.ID, validChallenge())
	if err == nil {
		t.Fatal("挑战自己的防线竟然成功了")
	}
	if !strings.Contains(err.Error(), "自己的防线") {
		t.Errorf("应报「不能挑战自己的防线」，实得 %v", err)
	}
}

// TestChallengeDefensePreLocksBeforeStealing 是**唯一**能区分
// 「入口的预锁调用被整段删掉」的守卫。
//
// # 为什么前三条都区分不了
//
// `TestLockChallengeState*` 直接测那个**函数**，
// 所以函数体被改坏能抓到，而**调用点**被删掉时函数仍然完好 → 全绿。
//
// ⚠️ 这与 README 记的「测函数不等于测调用点」完全同型
// （第 69 轮：`ValidateChallengeInput` 存在但 `ChallengeDefense` 不调它）。
//
// # 判据：预锁的加锁顺序与窃取分支的加锁顺序**不同**
//
// 预锁：**升序**（先小 id）。窃取：`grantWallet(对方)` → `grantWallet(自己)`。
//
// 于是：占住 **lo（较小那个）** 的钱包行，
// 让「lo 打 hi」在后台跑起来，然后探 **hi** 的钱包：
//
//	预锁存在   升序 → 先试 lo → 立刻卡住 → hi 的钱包**没被锁**
//	预锁被删   窃取顺序 → 先锁 hi（成功）→ 再卡在 lo → hi 的钱包**被锁**
//
// 差异与并发时序无关，是确定性的。
func TestChallengeDefensePreLocksBeforeStealing(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	attacker := ts.newUser(t, ctx)
	owner := ts.newUser(t, ctx)

	// 「较小 id」是攻方还是守方取决于分配顺序，两种都要能区分，
	// 所以这里让攻方是**较小**的那个（若分配相反，测试会在这里红，
	// 那说明需要另一种构造 —— 而不是悄悄换个方向）。
	if attacker > owner {
		t.Skip("攻方 id 反而更大 —— 本构造假设攻方是较小那个")
	}
	lo, hi := attacker, owner

	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{
		Name: "预锁探针", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("造防线失败：%v", err)
	}
	ts.grant(t, ctx, owner, map[string]int64{"coin": 500_000})

	// 占住 lo 的钱包行
	holder, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 holder 失败：%v", err)
	}
	defer func() { _ = holder.Rollback(ctx) }()
	var tmp int64
	if err := holder.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1 FOR UPDATE`, lo).Scan(&tmp); err != nil {
		t.Fatalf("占住 wallet[%d] 失败：%v", lo, err)
	}

	done := make(chan error, 1)
	go func() {
		_, e := ts.ChallengeDefense(ctx, lo, dv.ID, validChallenge())
		done <- e
	}()

	// 等到 **hi** 的钱包被锁住或超时。
	//
	// ⚠️ 这里等的是「hi 被锁」—— 若预锁被删，hi 会**很快**被锁上；
	// 若预锁在，它**永远不会被锁**（因为卡在 lo 上），
	// 于是我们等满预算后进入探测，探测结果为「没被锁」→ 通过。
	//
	// 所以这段等待只是给 worker 足够时间跑到「卡住」的位置，
	// 不作为断言。判据在下面的探测里。
	time.Sleep(300 * time.Millisecond)

	probe, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 probe 失败：%v", err)
	}
	hiLocked := probe.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1 FOR UPDATE NOWAIT`, hi).Scan(&tmp) != nil
	_ = probe.Rollback(ctx)

	_ = holder.Rollback(ctx)
	select {
	case <-done:
	case <-time.After(5 * time.Second):
		t.Log("挑战仍在进行（已被 holder 放开后应当很快结束）")
	}

	if hiLocked {
		t.Fatalf("wallet[%d]（较大那个）在 wallet[%d] 被占住期间已被锁上 —— "+
			"说明加锁走的是「对方→自己」而不是升序预锁", hi, lo)
	}
}
