package service

import (
	"context"
	"regexp"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

// 「达到 N 关」类成就的 `target` 必须等于 **N**，而不是 1（第 92 轮）。
//
// # 缺陷：通关第 1 关解锁「达到 100 关」，白送 710 钻石
//
// `max_stage` 这个 metric 的 bump 值是**刚结算那一关的关号**
// （`game.go` 结算路径传 `"max_stage": int64(gl.ID)`），
// 而 `bumpTasks` 对 `achievement` scope 走
// `GREATEST(progress, EXCLUDED.progress)` ——
//
//	progress 的语义是「**历史上到达过的最高关号**」
//
// 于是「达到 20 关」的达成条件应当是 `progress >= 20`。
// 而种子里 target 写的是 **1**，条件变成 `progress >= 1` ——
//
//	**通关第 1 关就解锁。**
//
// 实测（只结算第 1 关，修复前）：
//
//	ach_reach_20   progress=1 target=1   可领取
//	ach_reach_60   progress=1 target=1   可领取
//	ach_reach_100  progress=1 target=1   可领取   ← 500 钻石
//
// 三项合计 **710 钻石**，代价是通关第 1 关。
//
// # 为什么没被测出来
//
// 既有测试都是「造一行 progress >= target 再 ClaimTask」——
// **从不检查 target 与 metric 语义是否匹配**。
// 而 `ach_first_clear`（target=1, metric=clears）与
// `ach_reactions_100`（target=100, metric=reactions）的 target **是对的**，
// 所以「所有 achievement 的 target 都等于 1」这个假设在它们上面也成立。
func TestReachLevelAchievementTargetsMatchTheirNames(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()

	// code 形如 ach_reach_20 —— 数字就在 code 里，这是**可判定的**。
	re := regexp.MustCompile(`^ach_reach_(\d+)$`)

	rows, err := scratch.pool.Query(ctx,
		`SELECT code, name, target, metric FROM tasks
		  WHERE scope = 'achievement' AND code LIKE 'ach_reach_%' ORDER BY code`)
	if err != nil {
		t.Fatalf("读成就任务失败：%v", err)
	}
	defer rows.Close()

	bad := []string{}
	n := 0
	for rows.Next() {
		var code, name, metric string
		var target int
		if err := rows.Scan(&code, &name, &target, &metric); err != nil {
			t.Fatal(err)
		}
		n++
		m := re.FindStringSubmatch(code)
		if m == nil {
			bad = append(bad, code+": code 不符合 ach_reach_<N> 形态，无法从它推出 target")
			continue
		}
		want, err := strconv.Atoi(m[1])
		if err != nil {
			bad = append(bad, code+": "+err.Error())
			continue
		}
		if target != want {
			bad = append(bad, code+": target="+strconv.Itoa(target)+
				"，但 code 里写的是 "+strconv.Itoa(want))
		}
		if metric != "max_stage" {
			bad = append(bad, code+": metric="+metric+
				"。「达到 N 关」必须用 max_stage —— 该 metric 的值是**关号**，"+
				"用累加型的 metric（如 clears）会让「达到 100 关」变成「通关 100 次」")
		}
		if !strings.Contains(name, m[1]) {
			bad = append(bad, code+": 名字「"+name+"」里没有出现 "+m[1]+
				" —— code / name / target 三者对不上，改一个就会漂")
		}
	}
	if err := rows.Err(); err != nil {
		t.Fatal(err)
	}
	if n == 0 {
		t.Fatal("一条 ach_reach_* 任务都没有 —— 判据前提不成立")
	}
	if len(bad) > 0 {
		t.Errorf("以下「达到 N 关」成就的 target 与 code 不一致：\n  %s\n\n"+
			"后果：target=1 时达成条件退化成 progress>=1，"+
			"**通关第 1 关就解锁**（修复前实测 710 钻石）。\n"+
			"`max_stage` 的 progress 语义是「历史最高关号」，所以 target 必须是 N。",
			strings.Join(bad, "\n  "))
	}
}

// TestClearingLevelOneDoesNotUnlockReachAchievements 是行为判据。
//
// 上一条是**读种子表**，这一条是**走完整链路** ——
// 它能抓到的不只是 target 写错，还包括「bump 传的关号不对」
// 「GREATEST 语义被改成累加」这类形态。
func TestClearingLevelOneDoesNotUnlockReachAchievements(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 只结算第 1 关
	if err := scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
		return scratch.bumpTasks(ctx, tx, uid, map[string]int64{
			"max_stage": 1, "clears": 1, "kills": 3, "reactions": 1,
		}, time.Now())
	}); err != nil {
		t.Fatal(err)
	}

	// ⚠️ 第 93 轮：成就的周期起点已是**哨兵日期**，不再是今天。
	// 这里原来写的是 `periodStart(now, "daily")` —— 改完之后 LEFT JOIN 匹配不到任何行，
	// progress 恒为 0，于是「通关第 1 关不得解锁」那条断言变成**空洞通过**。
	//
	// **空洞通过的守卫比没有守卫更危险**：它看起来在守着某件事。
	day := periodStart(time.Now(), "achievement")

	// ⚠️ 自证：**本条断言用的关联键必须真的能匹配到行**。
	//
	// 第 93 轮实测：把这里的 `"achievement"` 改回 `"daily"`，
	// LEFT JOIN 就匹配不到任何行，`progress` 恒为 0，
	// 下面那条「不得解锁」的断言变成**空洞通过**而全绿。
	//
	// 所以先验一次：通关第 1 关后 `ach_reach_20` 的行**必须存在**。
	var exists int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM tasks t JOIN user_tasks ut
		        ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
		  WHERE t.code = 'ach_reach_20'`, uid, day).Scan(&exists); err != nil {
		t.Fatal(err)
	}
	if exists == 0 {
		t.Fatalf("用 task_date=%s 关联不到 ach_reach_20 的进度行 —— "+
			"本用例的关联键写错了，下面那条断言已是**空洞通过**。\n"+
			"成就的周期起点是 periodStart(now, \"achievement\")（哨兵日期），不是 \"daily\"。",
			day.Format("2006-01-02"))
	}

	rows, err := scratch.pool.Query(ctx,
		`SELECT t.code, t.target, COALESCE(ut.progress, 0)
		   FROM tasks t
		   LEFT JOIN user_tasks ut
		          ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
		  WHERE t.scope = 'achievement' AND t.code LIKE 'ach_reach_%'
		  ORDER BY t.code`, uid, day)
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()

	completed := []string{}
	total := 0
	for rows.Next() {
		var code string
		var target, progress int
		if err := rows.Scan(&code, &target, &progress); err != nil {
			t.Fatal(err)
		}
		total++
		if progress >= target {
			completed = append(completed, code+"（progress="+strconv.Itoa(progress)+
				" >= target="+strconv.Itoa(target)+"）")
		}
	}
	if err := rows.Err(); err != nil {
		t.Fatal(err)
	}
	if total == 0 {
		t.Fatal("一条 ach_reach_* 都没有 —— 判据前提不成立")
	}
	if len(completed) > 0 {
		t.Errorf("只通关了第 1 关，下列「达到 N 关」成就却被判定完成：\n  %s\n\n"+
			"通关第 1 关不该解锁「达到 20/60/100 关」。\n"+
			"若 `ach_first_clear`（target=1）也算，那是**应该**完成的 —— "+
			"它不在本次断言范围内。",
			strings.Join(completed, "\n  "))
	}
}

// TestReachAchievementCompletesAtItsOwnLevel 确认改完之后**仍能达成**。
//
// ⚠️ 这条不许被「干脆把成就删了」或「把 target 设成 999」绕过 ——
// 那是把一个 bug 换成另一个。
func TestReachAchievementCompletesAtItsOwnLevel(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 恰好推到第 20 关（**不**推过）
	//
	// ⚠️ 我第一版推到了 25，然后断言 `progress == target` ——
	// 而 `GREATEST` 会保留更高的值，progress=25、target=20，
	// 「progress == target」是错的断言，不是代码的错。
	for _, lvl := range []int64{5, 12, 20} {
		l := lvl
		if err := scratch.DB.Tx(ctx, func(tx pgx.Tx) error {
			return scratch.bumpTasks(ctx, tx, uid, map[string]int64{"max_stage": l}, time.Now())
		}); err != nil {
			t.Fatal(err)
		}
	}

	day := periodStart(time.Now(), "achievement")
	for code, wantComplete := range map[string]bool{
		"ach_reach_20": true, "ach_reach_60": false, "ach_reach_100": false,
	} {
		var target, progress int
		if err := scratch.pool.QueryRow(ctx,
			`SELECT t.target, COALESCE(ut.progress, 0)
			   FROM tasks t
			   LEFT JOIN user_tasks ut
			          ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
			  WHERE t.code = $3`, uid, day, code).Scan(&target, &progress); err != nil {
			t.Fatalf("读 %s 失败：%v", code, err)
		}
		complete := progress >= target
		if complete != wantComplete {
			t.Errorf("%s: progress=%d target=%d 完成=%v，期望 %v",
				code, progress, target, complete, wantComplete)
		}
		// 「恰好到达 N 关」时 progress 应当**正好是 N** ——
		// 这条同时钉住 metric 传的是关号、且 GREATEST 语义正确。
		//
		// ⚠️ 不能断言 `progress == target` 在「推过 N」之后也成立：
		// GREATEST 会保留更高的值（推过 20 后 progress 是 25，那是正确的）。
		if code == "ach_reach_20" && progress != 20 {
			t.Errorf("%s: 恰好到达第 20 关时 progress 应为 20，实际 %d", code, progress)
		}
	}
}
