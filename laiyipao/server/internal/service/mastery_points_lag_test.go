package service

import (
	"context"
	"testing"
)

// 专精点必须在**跨过门槛的那一局**就到账（第 103 轮）——这是钉住一个
// **刻意写出来的内联补偿**。
//
// # 缺陷：PG 的 SET 表达式读的是**旧值**
//
// ```sql
// UPDATE user_progress
//
//	SET level_exp     = level_exp + 50 + $4,
//	    mastery_points = 3 + FLOOR((level_exp + 50 + $4) / 1000)::int
//
// ```
//
// PostgreSQL 里一条 UPDATE 的**所有** SET 表达式都读**同一份旧行**，
// 所以这里的 `level_exp` 是**本局之前**的值。
//
// 而 `level_exp` 那一行确实加上了 `50 + $4` ——
// 于是 `mastery_points` 依据的是「本局之前」的 exp。
//
// # 后果：专精点晚一局到账
//
//	起始 exp = 950，本局 win、kills = 0
//	本局结束：level_exp = 950 + 50 + 0 = 1000   ← 跨过 1000
//	          mastery_points = 3 + FLOOR(950 / 1000) = 3   ← **没加**
//	下一局：  level_exp = 1050
//	          mastery_points = 3 + FLOOR(1050 / 1000) = 4   ← 这时才加
//
// 迁移里的注释写着 `mastery_points … 初始 3，每 10 级 +1`，
// 语义上「每 10 级」应当按**本局之后**的等级算。
//
// # 为什么测试抓不到
//
// 既有测试都是从 exp = 0 起算、连打很多局，
// 而 `FLOOR` 的误差恰好被后面的局吸收了 ——
// 单看最终值完全正确，只有**在跨门槛的那一局**断言才看得出。
func TestMasteryPointsArriveOnTheCrossingBattle(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	const kills = 0
	const expPerBattle = 50 + kills // 与 SQL 里的 `50 + $4` 一致

	// 把 exp 停在「再打一局就跨过 1000」的位置
	// 950 + 50 = 1000 → 本局跨线
	ts.exec(t, `UPDATE user_progress
	               SET level_exp = 950, mastery_points = 3
	             WHERE user_id = $1`, uid)

	readPoints := func() int {
		var n int
		if err := ts.pool.QueryRow(ctx,
			`SELECT mastery_points FROM user_progress WHERE user_id = $1`, uid).Scan(&n); err != nil {
			t.Fatalf("读专精点失败：%v", err)
		}
		return n
	}
	readExp := func() int64 {
		var n int64
		if err := ts.pool.QueryRow(ctx,
			`SELECT level_exp FROM user_progress WHERE user_id = $1`, uid).Scan(&n); err != nil {
			t.Fatalf("读经验失败：%v", err)
		}
		return n
	}

	beforeExp := readExp()
	_ = readPoints()

	// 打一局：跨过 1000
	if err := ts.DB.Tx(ctx, func(tx pgxTx) error {
		_, err := ts.applyProgress(ctx, tx, uid, 1, true, kills)
		return err
	}); err != nil {
		t.Fatalf("applyProgress 失败：%v", err)
	}

	afterExp := readExp()
	after := readPoints()

	// 本局之后应当按新 exp 算
	wantExp := beforeExp + int64(expPerBattle)
	if afterExp != wantExp {
		t.Fatalf("exp 应从 %d 变成 %d，实际 %d", beforeExp, wantExp, afterExp)
	}
	wantPoints := 3 + int(wantExp/1000)
	if after != wantPoints {
		t.Errorf("本局把 exp 推到 %d（跨过 1000），专精点应是 %d，实际 %d\n\n"+
			"若少 1，说明 `mastery_points` 依据的是**本局之前**的 exp ——\n"+
			"PostgreSQL 里一条 UPDATE 的所有 SET 表达式都读**同一份旧行**。\n"+
			"修法：把 exp 的增量先算进一个 CTE / 子查询，让两者基于同一份新值。",
			afterExp, wantPoints, after)
	}
}
