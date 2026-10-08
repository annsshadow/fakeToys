package service

import (
	"context"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

// 成就是**终身**累计的，不按天分行（第 93 轮）。
//
// # 缺陷：成就可以每天重复领取
//
// `user_tasks` 的主键是 `(user_id, task_id, task_date)` —— 每条进度按周期分行。
// 而 `periodStart(now, "achievement")` 原先落到 `default` 分支，即**今天零点**。
// 于是「成就」也成了每天一行：
//
//	今天：progress = 20，claimed_at = NULL  → 领取，写 claimed_at
//	明天：**新的一行**，progress = 20，claimed_at = **NULL**
//
// 而 `ClaimTask` 同样只读 `periodStart(now, scope)`（今天那一行），
// 于是明天再玩一次就能**再领一次**。
//
// # ⚠️ 更正：第 92 轮的探针结论写的是「跨日期领取被拒」—— 那是错的
//
// 那次探针手工插了「明天」的行，然后调 `ClaimTask(ctx, uid, id)`。
// 但 `ClaimTask` **内部自己算** `periodStart(time.Now(), scope)`、不接受传入日期，
// 于是它读到的是**今天**已领取的那一行 → 被拒。
//
// **探针没有制造出「明天」，它只是又查了一次今天。**
//
// 这与第 79 轮「未提交的 INSERT 在 READ COMMITTED 下不可见」、
// 第 89 轮「SET TIME ZONE 不跨连接」同族：
// **观测手段本身有前提。** 这次的前提是「被观测的函数不接受时间输入」。
func TestAchievementProgressIsLifetimeNotDaily(t *testing.T) {
	// 判据一：`periodStart` 对 achievement 返回**固定**日期
	t1 := periodStart(time.Now(), "achievement")
	t2 := periodStart(time.Now().AddDate(0, 0, 1), "achievement")
	t3 := periodStart(time.Now().AddDate(1, 0, 0), "achievement")
	if !t1.Equal(t2) || !t1.Equal(t3) {
		t.Errorf("periodStart(now, \"achievement\") 随日期变化：%s / %s / %s\\n"+
			"成就必须用**固定哨兵日期**，否则每天一行新记录、claimed_at 每天归零，"+
			"成就可以每天重复领取。",
			t1.Format("2006-01-02"), t2.Format("2006-01-02"), t3.Format("2006-01-02"))
	}
	if t1.IsZero() {
		t.Error("哨兵日期不应是零值 time.Time —— 它会被写成 0001-01-01，" +
			"与任何真实周期都不冲突但可读性差")
	}

	// 判据二：日/周**不能**被这个改动带偏
	d0 := periodStart(time.Date(2026, 3, 10, 15, 30, 0, 0, time.UTC), "daily")
	d1 := periodStart(time.Date(2026, 3, 11, 1, 0, 0, 0, time.UTC), "daily")
	if d0.Equal(d1) {
		t.Error("daily 的日界被哨兵日期带跑了 —— 每日任务必须每天不同")
	}
	w0 := periodStart(time.Date(2026, 3, 11, 23, 0, 0, 0, time.UTC), "weekly") // 周三
	w1 := periodStart(time.Date(2026, 3, 15, 1, 0, 0, 0, time.UTC), "weekly")  // 周日
	if !w0.Equal(w1) {
		t.Errorf("weekly 的周界不对：周三 %s vs 周日 %s，应当同属一周",
			w0.Format("2006-01-02"), w1.Format("2006-01-02"))
	}
	wSun := periodStart(time.Date(2026, 3, 15, 12, 0, 0, 0, time.UTC), "weekly")
	if !w0.Equal(wSun) {
		t.Errorf("周日应当与本周同起点：周三 %s vs 周日 %s", w0.Format("2006-01-02"), wSun.Format("2006-01-02"))
	}
}

// TestAchievementRowsAreSinglePerUser 是**行为**判据。
//
// 直接查库：同一个用户对同一条成就，无论过多少天，
// `user_tasks` 里**只能有一行**。
func TestAchievementRowsAreSinglePerUser(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	var taskID int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT id FROM tasks WHERE scope='achievement' LIMIT 1`).Scan(&taskID); err != nil {
		t.Skipf("没有成就任务：%v", err)
	}

	// 模拟三个不同日期的 bump
	for _, d := range []time.Time{
		time.Now(),
		time.Now().AddDate(0, 0, 1),
		time.Now().AddDate(0, 0, 2),
	} {
		dd := d
		if err := scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
			return scratch.bumpTasks(ctx, tx, uid, map[string]int64{"clears": 1}, dd)
		}); err != nil {
			t.Fatal(err)
		}
	}

	var n int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_tasks WHERE user_id = $1 AND task_id = $2`,
		uid, taskID).Scan(&n); err != nil {
		t.Fatal(err)
	}
	if n != 1 {
		var dates []string
		rows, err := scratch.pool.Query(ctx,
			`SELECT task_date::text FROM user_tasks WHERE user_id = $1 AND task_id = $2
			  ORDER BY task_date`, uid, taskID)
		if err != nil {
			t.Fatal(err)
		}
		defer rows.Close()
		for rows.Next() {
			var s string
			if err := rows.Scan(&s); err != nil {
				t.Fatal(err)
			}
			dates = append(dates, s)
		}
		t.Errorf("同一用户同一条成就出现了 %d 行（%v），应为 1 行\\n"+
			"多行意味着「每天一行、claimed_at 每天归零」→ 成就可以每天重复领取。",
			n, dates)
	}
}

// TestAchievementClaimedOnceStaysClaimed 钉住「领过一次就永远领不到」。
//
// ⚠️ 这条**不能**用「把日期拨到明天」来测 ——
// `ClaimTask` 内部自己算 `periodStart(time.Now(), …)`，不接受时间输入
// （第 92 轮探针正是因此得出错误结论）。
//
// 所以这里改成直接验算那条 SQL 语义：
// 哨兵行只有一个，`claimed_at` 一旦写入，条件 UPDATE 匹配不到行。
func TestAchievementClaimedOnceStaysClaimed(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	var id, target int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT id, target FROM tasks WHERE scope='achievement' AND target = 1 LIMIT 1`).
		Scan(&id, &target); err != nil {
		t.Skipf("没有 target=1 的成就：%v", err)
	}

	// ⚠️ 这一次**预期**失败（还没做完），不是 Fatal。
	// 我第一版写成 `t.Fatalf("首次领取应失败")` ——
	// Fatal 在这里意味着「期望的失败发生时反而红」，测试永远过不了。
	if _, err := scratch.ClaimTask(ctx, uid, id); err == nil {
		t.Fatal("还没做完就领取成功了 —— progress 预检失效")
	}

	// 造出「已完成但未领取」的哨兵行
	if err := scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
		return scratch.bumpTasks(ctx, tx, uid, map[string]int64{"clears": int64(target)}, time.Now())
	}); err != nil {
		t.Fatal(err)
	}

	if _, err := scratch.ClaimTask(ctx, uid, id); err != nil {
		t.Fatalf("应能领取：%v", err)
	}

	// 第二次：必须被拒
	_, err := scratch.ClaimTask(ctx, uid, id)
	if err == nil {
		t.Fatal("同一条成就领了两次 —— claimed_at 没有生效")
	}

	// 且哨兵行只有一行、claimed_at 非空
	var n int
	var claimed *time.Time
	if err := scratch.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_tasks WHERE user_id=$1 AND task_id=$2`,
		uid, id).Scan(&n); err != nil {
		t.Fatal(err)
	}
	if err := scratch.pool.QueryRow(ctx,
		`SELECT claimed_at FROM user_tasks WHERE user_id=$1 AND task_id=$2`,
		uid, id).Scan(&claimed); err != nil {
		t.Fatal(err)
	}
	if n != 1 {
		t.Errorf("成就行数 = %d，应为 1", n)
	}
	if claimed == nil {
		t.Error("claimed_at 为空 —— 领取没有落库")
	}
}

// TestAchievementMigrationConsolidatesRows 确认迁移 00013 的语义。
//
// ⚠️ 不能直接对生产迁移做「回滚再前滚」的测试 ——
// 那会改真实数据。改为在 scratch 库里造出「历史多行」的形态，
// 再执行**同样那条 SQL**，验证合并结果。
//
// 判据是**逐条**属性：
//
//	progress   取 MAX（终身最高，不是最后一天）
//	claimed_at 取 MIN（曾经领过就永久领过，**不是** now()）
func TestAchievementMigrationConsolidatesRows(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()

	var achID int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT id FROM tasks WHERE scope='achievement' LIMIT 1`).Scan(&achID); err != nil {
		t.Skipf("没有成就任务：%v", err)
	}
	u1 := scratch.newUser(t, ctx)
	u2 := scratch.newUser(t, ctx)

	// u1：三天三行，progress 递减（1 / 5 / 3），只有第一行已领取
	scratch.exec(t, `
		INSERT INTO user_tasks (user_id, task_id, task_date, progress, claimed_at)
		VALUES ($1, $2, CURRENT_DATE - 2, 1, TIMESTAMPTZ '2026-01-02 03:04:05+00'),
		       ($1, $2, CURRENT_DATE - 1, 5, NULL),
		       ($1, $2, CURRENT_DATE,     3, NULL)`, u1, achID)

	// 记下**迁移前**的 claimed_at —— 迁移不得改写它。
	//
	// ⚠️ 我第一版只断言「claimed_at 非空」，于是
	// 「`MIN(ut.claimed_at)`（保留原时间戳）」与
	// 「`CASE WHEN COUNT(...)>0 THEN now() END`（改写成迁移时刻）」
	// **两者都非空** → 变异全绿。
	//
	// 要分出它们必须比**时间戳本身**，而不是「有没有」。
	var claimedBefore time.Time
	if err := scratch.pool.QueryRow(ctx,
		`SELECT claimed_at FROM user_tasks
		  WHERE user_id = $1 AND task_id = $2 AND claimed_at IS NOT NULL`,
		u1, achID).Scan(&claimedBefore); err != nil {
		t.Fatal(err)
	}
	// u2：一行，未领取
	scratch.exec(t, `
		INSERT INTO user_tasks (user_id, task_id, task_date, progress, claimed_at)
		VALUES ($1, $2, CURRENT_DATE, 7, NULL)`, u2, achID)

	// —— 执行**迁移文件里那条真 SQL** ——
	//
	// ⚠️ 我第一版把 SQL **内联复制**了一份在这里。
	// 于是变异「把迁移文件里的 MIN(claimed_at) 改成 now()」→ **全绿** ——
	// 因为被测的根本不是那个文件。
	//
	// **复制一份 = 测的是副本，不是被测物。**
	// 判据必须从**被测文件**里取。
	upSQL := migrationUpSQL(t, "00013_achievement_epoch.sql")
	for _, stmt := range splitStatements(upSQL) {
		scratch.exec(t, stmt)
	}

	type row struct {
		n       int
		prog    int
		claimed int
	}
	read := func(uid int64) row {
		var r row
		var c *time.Time
		if err := scratch.pool.QueryRow(ctx,
			`SELECT COUNT(*)::int, COALESCE(MAX(progress),0)::int,
			        COALESCE(MAX((claimed_at IS NOT NULL)::int), 0)::int
			   FROM user_tasks WHERE user_id=$1 AND task_id=$2`, uid, achID).
			Scan(&r.n, &r.prog, &r.claimed); err != nil {
			t.Fatal(err)
		}
		_ = c
		return r
	}

	got := read(u1)
	if got.n != 1 {
		t.Errorf("u1 合并后有 %d 行，应为 1", got.n)
	}
	if got.prog != 5 {
		t.Errorf("u1 的 progress = %d，应为 MAX(1,5,3) = 5 —— "+
			"取最后一次的 3 会把「历史最高」压低", got.prog)
	}
	if got.claimed == 0 {
		t.Error("u1 的 claimed_at 丢了 —— " +
			"曾经领过就永久领过，迁移不得用 now() 改写这个事实")
	}

	// 关键：时间戳本身必须**逐位保持**。
	//
	// `MIN(claimed_at)` 保留原值；`now()` 会把它改写成迁移时刻。
	// 只断言「非空」分不出这两者 —— 那是本条守卫第一版的漏洞。
	var claimedAfter time.Time
	if err := scratch.pool.QueryRow(ctx,
		`SELECT claimed_at FROM user_tasks
		  WHERE user_id = $1 AND task_id = $2 AND task_date = DATE '1970-01-01'`,
		u1, achID).Scan(&claimedAfter); err != nil {
		t.Fatal(err)
	}
	if !claimedAfter.Equal(claimedBefore) {
		t.Errorf("claimed_at 被改写：迁移前 %s，迁移后 %s\n"+
			"迁移必须用 `MIN(claimed_at)` 保留既成事实的时间戳，"+
			"而不是 `now()` —— 后者抹掉了「何时领的」这条审计信息。",
			claimedBefore.Format(time.RFC3339), claimedAfter.Format(time.RFC3339))
	}

	got2 := read(u2)
	if got2.n != 1 {
		t.Errorf("u2 合并后有 %d 行，应为 1", got2.n)
	}
	if got2.prog != 7 {
		t.Errorf("u2 的 progress = %d，应为 7", got2.prog)
	}
	if got2.claimed != 0 {
		t.Error("u2 未领过却被标成已领取 —— MIN(claimed_at) 在全是 NULL 时必须是 NULL")
	}
}

// TestAchievementEpochIsNotARealDay 确认哨兵日期不会与真实周期撞上。
func TestAchievementEpochIsNotARealDay(t *testing.T) {
	ep := periodStart(time.Now(), "achievement")
	daily := periodStart(time.Now(), "daily")
	weekly := periodStart(time.Now(), "weekly")
	if ep.Equal(daily) || ep.Equal(weekly) {
		t.Errorf("哨兵日期与真实周期起点相同（%s）—— "+
			"那会让成就行与日/周任务行混在一起", ep.Format("2006-01-02"))
	}
	if ep.Year() > 2000 {
		t.Errorf("哨兵日期 %s 看起来像个真实日期 —— "+
			"它的作用是「一眼看出不是真实周期」", ep.Format("2006-01-02"))
	}
	t.Logf("哨兵日期 = %s（epoch 年 %d）", ep.Format("2006-01-02"), ep.Year())
	_ = strconv.Itoa(0)
}

/**
 * 守��本身**不许退化成空洞通过**（第 93 轮）。
 *
 * # 实际发生过
 *
 * 第 93 轮把 `periodStart(now, "achievement")` 改成哨兵日期之后，
 * `TestClearingLevelOneDoesNotUnlockReachAchievements` 立刻变绿 ——
 * 但那是**空洞通过**：
 *
 * ```sql
 * LEFT JOIN user_tasks ut
 *        ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
 * ```
 *
 * 那条用例的 `$2` 仍是 `periodStart(now, "daily")` = 今天零点，
 * 而成就行现在在 `1970-01-01` —— **JOIN 匹配不到任何行**，
 * `progress` 恒为 0，于是 `0 >= target` 恒假，断言恒成立。
 *
 * **空洞通过的守卫比没有守卫更危险**：它看起来在守着某件事，
 * 会在出问题时给出「守卫在、所以没问题」的错误结论。
 *
 * # 判据
 *
 * 用 `INNER JOIN` 问同一个问题：
 * 「通关第 1 关之后，成就行**确实存在**」。
 *
 * 只要行不存在就红 —— 那样外层那条 LEFT JOIN 断言就没有意义。
 * 而目标恰好是 20 的 `ach_reach_20` **必然有行**
 * （`bumpTasks` 对每个匹配 metric 的任务都插一行），
 * 所以「不存在」只可能是关联键写错了。
 */
func TestReachAchievementRowActuallyExistsAfterOneClear(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	if err := scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
		return scratch.bumpTasks(ctx, tx, uid, map[string]int64{
			"max_stage": 1, "clears": 1,
		}, time.Now())
	}); err != nil {
		t.Fatal(err)
	}

	// 用 achievement 自己的周期起点做 **INNER** JOIN
	var progress int
	err := scratch.pool.QueryRow(ctx,
		`SELECT ut.progress
		   FROM tasks t JOIN user_tasks ut
		          ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
		  WHERE t.code = 'ach_reach_20'`, uid, periodStart(time.Now(), "achievement")).
		Scan(&progress)
	if err != nil {
		if err == pgx.ErrNoRows {
			t.Fatal("通关第 1 关之后**没有** ach_reach_20 的进度行\n\n" +
				"这说明 TestClearingLevelOneDoesNotUnlockReachAchievements 已经是**空洞通过** ——\n" +
				"它的 LEFT JOIN 匹配不到行，progress 恒为 0，于是断言恒真。\n" +
				"空洞通过的守卫比没有守卫更危险。")
		}
		t.Fatal(err)
	}
	if progress != 1 {
		t.Errorf("ach_reach_20 的 progress = %d，通关第 1 关后应为 1（关号）", progress)
	}
}

/**
 * migrationUpSQL 读出迁移文件里 `-- +goose Up` 与 `-- +goose Down` 之间的部分。
 *
 * ⚠️ 为什么必须从文件读而不是内联复制：
 * 内联的那份是**副本**，改迁移文件时测试照样绿
 * （第 93 轮实测：把 `MIN(claimed_at)` 改成 `now()`，内联版的测试全绿）。
 *
 * **测副本等于没测。**
 */
func migrationUpSQL(t *testing.T, name string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "..", "migrations", name))
	if err != nil {
		t.Fatalf("读迁移 %s 失败：%v", name, err)
	}
	text := string(raw)
	const upMark = "-- +goose Up"
	const downMark = "-- +goose Down"
	i := strings.Index(text, upMark)
	if i < 0 {
		t.Fatalf("%s 里找不到 %s", name, upMark)
	}
	rest := text[i+len(upMark):]
	j := strings.Index(rest, downMark)
	if j < 0 {
		return rest
	}
	return rest[:j]
}

/** splitStatements 按分号切分 SQL，并丢掉空语句与注释行。 */
func splitStatements(sql string) []string {
	var out []string
	for _, raw := range strings.Split(sql, ";") {
		var keep []string
		for _, line := range strings.Split(raw, "\n") {
			t := strings.TrimSpace(line)
			if t == "" || strings.HasPrefix(t, "--") {
				continue
			}
			keep = append(keep, line)
		}
		joined := strings.TrimSpace(strings.Join(keep, "\n"))
		if joined != "" {
			out = append(out, joined)
		}
	}
	return out
}
