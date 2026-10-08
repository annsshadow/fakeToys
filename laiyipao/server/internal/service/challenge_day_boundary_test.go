package service

import (
	"context"
	"os"
	"sort"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

// 防线挑战的「今日」必须与每日任务的「今日」是**同一套口径**（第 98 轮）。
//
// # 缺陷形态：「今日」有两个定义
//
// | 路径 | 日界来源 |
// |---|---|
// | 每日任务（含「今日通关 3 关」） | Go 的 `periodStart(now, "daily")` |
// | **每日挑战次数上限** | **PG 会话时区**（`CURRENT_DATE`） |
//
// 而这两者**语义上必须对齐** ——
// 「今日通关 3 关」和「今日挑战次数上限」说的是同一个「今日」。
//
// 两者不同时刻翻页时（Go 本地 ≠ PG 会话时区，容器里 `time.Local` 常为 UTC）：
//
//	任务进度：00:00 UTC          = 北京 08:00 重置
//	挑战次数：00:00 Asia/Shanghai = 北京 00:00 重置
//
// 于是凌晨到早上做的通关，任务在 08:00 就清零了，
// 而挑战次数要等到北京午夜才重置 —— 玩家会看到「今天 0 次挑战」配「今天已完成 3 通关」。
func TestChallengeDayBoundaryMatchesTaskDayBoundary(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 行为判据：挑战计数行落在**任务口径**的那一天
	uid := ts.newUser(t, ctx)

	if err := ts.DB.Tx(ctx, func(tx pgxTx) error {
		_, _, err := ts.dailyChallengeCountersTx(ctx, tx, uid)
		return err
	}); err != nil {
		t.Fatalf("读挑战计数器失败：%v", err)
	}

	var got string
	if err := ts.pool.QueryRow(ctx,
		`SELECT challenge_date::text FROM user_daily_challenges WHERE user_id = $1`,
		uid).Scan(&got); err != nil {
		t.Fatal(err)
	}
	want := periodStart(time.Now(), "daily").Format("2006-01-02")
	if got != want {
		t.Errorf("challenge_date = %s，任务口径的今天是 %s\n\n"+
			"「今日挑战次数」与「今日通关任务」必须指同一个日子，"+
			"否则玩家会看到「今天 0 次挑战」配「今天已完成 3 通关」。",
			got, want)
	}
}

// TestServiceHasNoCurrentDateInBusinessDaySQL 是**源码**判据：
// 决定业务「哪一天」的 SQL 里不得再出现 `CURRENT_DATE`。
//
// ⚠️ 为什么源码判据比行为判据强：
// 行为判据在「PG 会话时区 == Go 本地」时**观察不到差别** ——
// 而那正是本环境的现状（第 90 轮实测 PG 是 `Asia/Shanghai`）。
//
// `CURRENT_DATE` 这个**写法本身**就是「日界由 PG 决定」的标记：
// 可判定、不依赖环境。
//
// ⚠️ 剥注释 —— 否则上面那段解释 `CURRENT_DATE` 的注释会被当成代码。
// 本项目已因「文本扫描撞注释」踩坑四次（第 83/88/94/95 轮）。
func TestServiceHasNoCurrentDateInBusinessDaySQL(t *testing.T) {
	// ⚠️ 第 99 轮：范围从「两个文件」扩到**整个 service 包**。
	//
	// 我第一版只查 `economy.go` + `progression.go` ——
	// 理由是「只有这两处决定业务日界」。但那个理由**会失效**：
	// 下一个人在新文件里写 `CURRENT_DATE` 时，
	// 这条守卫根本不看那个文件。
	//
	// 这与第 74 轮（道具名单只有 Buy 在用）、第 91 轮（metric 只有结算在 bump）
	// 是同一个形状：**守卫覆盖「我已知的几处」，而不是「这类情况」。**
	//
	// 唯一的例外是 `stats.go`（运营看板口径，**有意不改**），
	// 由 TestStatsDayBoundaryIsDocumented 单独钉住。

	// 白名单：文件名 -> 为什么它可以保留 CURRENT_DATE
	allowed := map[string]string{
		"stats.go": "运营看板口径（今日新增 / 14 天曲线）。改了会让历史报表不可比，" +
			"由 TestStatsDayBoundaryIsDocumented 钉住数量仍是 3",
	}

	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("读包目录失败：%v", err)
	}

	var offenders []string
	checked := 0

	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		if _, ok := allowed[name]; ok {
			continue
		}
		checked++

		code := stripGoComments(readFileInPackage(t, name))
		for i, ln := range strings.Split(code, "\n") {
			if strings.Contains(ln, "CURRENT_DATE") {
				offenders = append(offenders,
					name+":"+itoa(int64(i+1))+": "+strings.TrimSpace(ln))
			}
		}

		// 变异实测（第 97/98 轮各一次）：把 `periodStart` 换成
		// `time.Now().Truncate(24*time.Hour)` → 行为判据**全绿**。
		//
		// `Truncate` 按**零时刻起算、忽略 Location**，对齐 UTC 午夜，
		// 而结果仍带 `+08:00` —— 所以它只在部分时段与本地日一致：
		//
		// | 北京时间 | Truncate 结果 | 与本地日 |
		// |---|---|---|
		// | 08:00–24:00 | 同一天 | 一致 ← 测不出来 |
		// | **00:00–08:00** | **前一天** | **差一天** ❌ |
		//
		// 当前实测时刻落在「一致」那一栏，**测试结果随运行时刻变化**。
		// 判据不比较结果，直接要求实现**必须是** `periodStart`。
		for _, ln := range strings.Split(code, "\n") {
			if !strings.Contains(ln, "today :=") {
				continue
			}
			if !strings.Contains(ln, "periodStart(") {
				offenders = append(offenders,
					name+": today 赋值没有用 periodStart： "+strings.TrimSpace(ln)+
						"（`Truncate` 是 UTC 对齐的，北京时间 00:00–08:00 会比本地日早一天）")
			}
		}
	}

	// 自证：**扫过的文件数**必须 > 0，否则「零违规」与「目录读空了」无法区分。
	//
	// ⚠️ 与第 91 轮那条同源：守卫可以被「扫得少」而失效，
	// 而失效方式与「全部合规」一模一样。
	//
	// ⚠️ 但 `checked == 0` 不够：变异「把范围缩回 economy.go + progression.go」
	// → **全绿通过**（那两个文件本来就干净）。
	//
	// 「扫得少」与「全部合规」在输出里**一模一样** ——
	// 这与第 91 轮那条同源。
	//
	// 所以要有**下限**：service 包的源文件远多于这个数，
	// 缩到 2 个说明范围被人为收窄了。
	const minExpectedFiles = 5
	if checked < minExpectedFiles {
		t.Fatalf("只扫了 %d 个文件，期望至少 %d 个\n"+
			"「扫得少」与「全部合规」在输出里一模一样 —— "+
			"变异「把范围缩回两个已知文件」是全绿的。"+
			"新增源文件时这个下限要相应提高。",
			checked, minExpectedFiles)
	}

	// 豁免必须写理由，空理由等于没有豁免
	for f, why := range allowed {
		if strings.TrimSpace(why) == "" {
			offenders = append(offenders, f+" 在豁免表里但理由为空 —— "+
				"「这里可以保留 CURRENT_DATE」不留给沉默")
		}
		_ = f
	}

	if len(offenders) > 0 {
		sort.Strings(offenders)
		t.Errorf("以下 %d 处仍在 SQL 里决定业务「哪一天」"+
			"（已扫 %d 个文件，豁免 %d 个）：\n  %s\n\n"+
			"`CURRENT_DATE` 由 **PG 会话时区**决定，而任务/签到/商城/挑战"+
			"用 Go 的 `periodStart(now, \"daily\")`。\n"+
			"容器里 `time.Local` 常为 UTC，两者不同时刻翻页 —— "+
			"于是「今日」有两个定义。\n"+
			"修法：传日期字面量（第 89/97/98 轮都是这个手法）。\n"+
			"若某处**确实**该按看板口径走，请写进 allowed 并说明理由。",
			len(offenders), checked, len(allowed), strings.Join(offenders, "\n  "))
	}
}

// TestStatsDayBoundaryIsDocumented 记下 `stats.go` 的三处例外。
//
// ⚠️ 它们**故意不改**：那是运营看板的统计口径
// （今日新增、近 14 天曲线），改它会让历史报表不可比。
//
// 但「故意不改」必须被写下来并被检查 ——
// 否则下一个人会以为它漏掉了。
//
// 「故意不改」与「漏掉了」在代码里长得一模一样。
func TestStatsDayBoundaryIsDocumented(t *testing.T) {
	code := stripGoComments(readFileInPackage(t, "stats.go"))
	// ⚠️ 数**行**而不是出现次数。
	//
	// 第 193 行 `generate_series(CURRENT_DATE - INTERVAL '13 days', CURRENT_DATE, ...)`
	// 一行里有**两处**，而 `strings.Count` 数的是出现次数 → 得到 4 而不是 3，
	// 于是本用例报「数量变了」，看起来像有人改了统计口径。
	//
	// 「数量变了」这种失败信息会让人去查代码，而真正的原因在守卫自己身上。
	lines := 0
	for _, ln := range strings.Split(code, "\n") {
		if strings.Contains(ln, "CURRENT_DATE") {
			lines++
		}
	}
	n := lines
	if n == 0 {
		t.Skip("stats.go 里已无 CURRENT_DATE —— 若是有意改的，请把本用例与那条决定一起删掉")
	}
	if n != 3 {
		t.Errorf("stats.go 里有 %d 处 CURRENT_DATE，期望 3 处\n"+
			"（今日新增 x2 + 14 天曲线的 generate_series）。\n"+
			"数量变了说明有改动，请同步更新本用例与 README 的说明。", n)
	}
	t.Logf("stats.go 保留 %d 处 CURRENT_DATE —— 运营看板口径，**有意不改**。"+
		"它与游戏内「今日」可能不同时刻翻页，属已知取舍。", n)
}

// pgxTx 是 service 包里事务回调的形参类型（与 DB.Tx 的签名一致）。
type pgxTx = pgx.Tx

// readFileInPackage 读同包源码，供「扫源码」型守卫使用。
func readFileInPackage(t *testing.T, name string) string {
	t.Helper()
	raw, err := os.ReadFile(name)
	if err != nil {
		t.Fatalf("读 %s 失败：%v", name, err)
	}
	return string(raw)
}
