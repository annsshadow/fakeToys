package service

// 第 126 轮：ChallengeDefense 的 `user_daily_challenges` 行锁必须
// 由**计数器读取之前**的全局升序预锁取得。
//
// 第 79 轮修的是**钱包**锁序，但当时把预锁放在 `dailyChallengeCountersTx`
// **之后**——而两次计数器读取本身就会按「自己 → 对方」拿行锁。
// 残留窗口：两个玩家**当日第一次**互打（今天的行还没被任何已提交事务
// 建出来）时：
//
//	Tx1（A 打 B）先锁 counters[A] → 等 counters[B]（B 的未提交 INSERT）
//	Tx2（B 打 A）先锁 counters[B] → 等 counters[A]
//
// `ON CONFLICT DO UPDATE` 会等对面未提交的 INSERT → 成环 →
// PG 40P01 → 两个诚实玩家各收一个 500（自伤型）。
//
// # 判据（与 challenge_lock_order_test.go 同款的 NOWAIT 探测法，确定性）
//
// 方向取**降序**（attacker = 大 id，owner = 小 id）—— 这是新旧顺序
// 唯一会分岔的方向。holder 占住 attacker 的计数器行不提交；
// worker（attacker → owner）尚未返回期间：
//
//	新顺序（升序）：先锁 owner 的计数器行 → 探到 55P03
//	旧顺序（自己先）：一直等 attacker 的行 → 探到 owner 行未被锁
//
// 计数器行必须**预先建好**（已提交）—— 未提交的 INSERT 对别的连接
// 不可见，探测会假阴性。

import (
	"context"
	"testing"
	"time"
)

func TestChallengeDefenseLocksCountersInAscendingOrder(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	u1 := ts.newUser(t, ctx)
	u2 := ts.newUser(t, ctx)
	// attacker 恒取大 id、owner 取小 id（两种分配顺序都能钉死方向）
	var attacker, owner int64
	if u1 < u2 {
		attacker, owner = u2, u1
	} else {
		attacker, owner = u1, u2
	}

	// owner 要有防线（被挑战）与金币（可被偷）
	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{
		Name: "锁序探针", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("造防线失败：%v", err)
	}
	ts.grant(t, ctx, owner, map[string]int64{"coin": 500_000})

	// 预建双方今日计数器行（已提交，跨连接可见）
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	for _, id := range []int64{attacker, owner} {
		if _, err := ts.pool.Exec(ctx, `
			INSERT INTO user_daily_challenges (user_id, challenge_date, attempts, stolen_times)
			VALUES ($1, $2, 0, 0)
			ON CONFLICT (user_id, challenge_date) DO UPDATE SET challenge_date = $2`,
			id, today); err != nil {
			t.Fatalf("预建 counters[%d] 失败：%v", id, err)
		}
	}

	// holder 占住 attacker 的计数器行（旧顺序的第一行；不提交）
	holder, err := ts.db.Pool.Begin(ctx)
	if err != nil {
		t.Fatalf("开 holder 失败：%v", err)
	}
	if _, err := holder.Exec(ctx, `
		UPDATE user_daily_challenges SET challenge_date = $2
		 WHERE user_id = $1 AND challenge_date = $2`,
		attacker, today); err != nil {
		t.Fatalf("占住 counters[%d] 失败：%v", attacker, err)
	}
	// ⚠️ 红灯路径（旧顺序）t.Fatalf 提前终止时，必须把 holder 的锁放开：
	// worker 的池连接正等这把锁，而 openTestService 的
	// `t.Cleanup(db.Close)` 会等**所有**池连接归还 —— 不放锁，
	// 整个测试进程就挂在收尾上（实测两次）。绿路径里 commit 后再
	// rollback 只会报「事务已结束」，被忽略。
	defer func() { _ = holder.Rollback(ctx) }()

	done := make(chan error, 1)
	go func() {
		_, e := ts.ChallengeDefense(ctx, attacker, dv.ID, validChallenge())
		done <- e
	}()

	// 新顺序：owner（较小 id）的计数器行**应当已**被 worker 锁住。
	// 3s 预算即旧顺序的确定性红灯窗口：旧顺序里 worker 全程等
	// attacker 的行，owner 的行永远不会被锁 → 超时报红。
	waitUntilLocked(t, ts, ctx, "user_daily_challenges", "user_id", owner, 3*time.Second)

	// 放开 holder → worker 应当正常走完全程（不是死锁 500）
	if err := holder.Commit(ctx); err != nil {
		t.Fatalf("holder 提交失败：%v", err)
	}
	select {
	case err := <-done:
		if err != nil {
			t.Fatalf("挑战在放开后仍报错（%v）—— 不该有死锁", err)
		}
	case <-time.After(10 * time.Second):
		t.Fatal("放开后 10s 内挑战仍未返回")
	}

	// 结算必须落库：一条挑战记录，attacker 的 attempts +1
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM defense_challenges
		 WHERE defense_id = $1 AND challenger_id = $2`,
		dv.ID, attacker).Scan(&n); err != nil || n != 1 {
		t.Fatalf("挑战行未落库（count=%d err=%v）", n, err)
	}
	var attempts int
	if err := ts.pool.QueryRow(ctx,
		`SELECT attempts FROM user_daily_challenges
		 WHERE user_id = $1 AND challenge_date = $2`,
		attacker, today).Scan(&attempts); err != nil || attempts != 1 {
		t.Fatalf("attacker attempts=%d（应为 1）err=%v", attempts, err)
	}
}
