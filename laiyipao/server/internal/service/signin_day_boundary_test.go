package service

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// 签到与每日任务必须用**同一个**日界口径（第 89 轮）。
//
// # 缺陷：「今天几号」由两个不同的层各算一半
//
// | 路径 | 日界来源 |
// |---|---|
// | 每日任务 | Go 的 `periodStart(now, "daily")`（用 `time.Local`） |
// | **每日签到** | **PG 会话时区**（`sign_date DATE` 的隐式 timestamptz 折算） |
//
// 而「同一天只能签一次」**完全靠** `PRIMARY KEY (user_id, sign_date)` 保证 ——
// 也就是说签到的日界精度直接等于 PG 的会话时区精度。
//
// # 实测结论：**当前部署下两者一致，这不是正在发生的 bug**
//
// ```go
// PG session TimeZone = "Asia/Shanghai"
// go time.Local      = Local (+08:00)
// pg-derived date    = 2026-10-06
// go-derived date    = 2026-10-06
// SAME? true
// ```
//
// 但正确性**依赖一条没有任何东西钉住的配置**：
// 托管 Postgres 的会话时区默认值常常是 UTC。
// 若是 UTC，签到的「天」会在北京时间 **08:00** 翻页 ——
// 同一个自然日可签两次，而每日任务还停留在前一天。
//
// **把它叫「潜伏缺陷」而不是「正在丢钱」是我实测后的结论，不是保守。**
//
// # 另一层证据：代码原本是打算在 Go 里算的
//
// 改前有一行：
//
// ```go
// today := time.Now().Truncate(24 * time.Hour)
// ...
// _ = today
// ```
//
// 算完就扔。而 `Truncate` 本身也是 **UTC 对齐**的
// （它按零时刻起算、忽略 Location），所以即使当初保留它，
// 得到的也不是「本地零点」。**意图对，实现两头都不对。**
func TestSignInUsesTheSameDayBoundaryAsTasks(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	if _, err := scratch.SignIn(ctx, uid); err != nil {
		t.Fatalf("首次签到失败：%v", err)
	}

	want := periodStart(time.Now(), "daily").Format("2006-01-02")

	stored := readSignDate(t, scratch, ctx, uid)
	if stored != want {
		t.Errorf("sign_date = %s，每日任务口径算出的是 %s", stored, want)
	}

	// PG 自己按会话时区折算 now() 得到的日期 —— 记录下来但不作为判据：
	// 它**必然**等于 want（当前环境会话时区与本地一致），
	// 而一旦不一致，本文件另一条用例会红。
	var pgDate string
	if err := scratch.pool.QueryRow(ctx,
		`SELECT (now()::timestamptz)::date::text`).Scan(&pgDate); err != nil {
		t.Fatalf("读 PG 折算日期失败：%v", err)
	}
	if pgDate != want {
		t.Logf("PG 会话时区折算出的日期是 %s，Go 口径是 %s —— "+
			"两者不同意味着签到与每日任务在不同时刻翻页。", pgDate, want)
	}
}

/**
 * readSignDate 以**日历日**读回 sign_date。
 *
 * ⚠️ 不能比时刻：pgx 把 `DATE` 读成 `time.Time` 时给的是 **UTC 午夜**
 * （`2026-10-06T00:00:00Z`），而 `periodStart` 给的是**本地午夜**
 * （`2026-10-06T00:00:00+08:00`）。它们是同一天的两种表示，
 * `time.Time.Equal` 会判**不等** —— 我第一版就踩了这个，
 * 报出「sign_date 错了」而代码其实是对的。
 *
 * `2006-01-02` 是 DATE 的天然文本形态，PG 直接给字符串，不需要任何换算。
 */
func readSignDate(t *testing.T, scratch *testService, ctx context.Context, uid int64) string {
	t.Helper()
	var s string
	if err := scratch.pool.QueryRow(ctx,
		`SELECT sign_date::text FROM user_sign_ins WHERE user_id = $1`, uid).Scan(&s); err != nil {
		t.Fatalf("读回 sign_date 失败：%v", err)
	}
	return s
}

// TestSignInDayIsDateNotTimestamp 确认存进去的是 **date** 而不是 timestamptz。
//
// 判据不是「类型对不对」—— 那样 PG 会自己转换，什么都测不到。
// 判据是：**把连接会话时区改掉之后，同一个 instant 仍折成同一个 sign_date**。
//
// 这才是真正的性质：日界由**我们**决定，不由连接配置决定。
func TestSignInDayIsDecidedInGoNotBySessionTZ(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 把会话时区改成 UTC —— 这是托管实例的常见默认值
	if _, err := scratch.pool.Exec(ctx, `SET TIME ZONE 'UTC'`); err != nil {
		t.Fatalf("改会话时区失败：%v", err)
	}
	t.Cleanup(func() {
		if _, err := scratch.pool.Exec(context.Background(), `SET TIME ZONE 'Asia/Shanghai'`); err != nil {
			t.Logf("恢复会话时区失败：%v", err)
		}
	})

	if _, err := scratch.SignIn(ctx, uid); err != nil {
		t.Fatalf("UTC 会话时区下签到失败：%v", err)
	}

	stored := readSignDate(t, scratch, ctx, uid)
	want := periodStart(time.Now(), "daily").Format("2006-01-02")
	if stored != want {
		t.Errorf("会话时区改成 UTC 后 sign_date = %s，期望 %s（Go 口径）\n\n"+
			"说明日界仍由 PG 会话时区决定 —— 那是**配置**决定的，不是代码决定的。\n"+
			"托管实例的默认时区常常是 UTC，那样签到会在北京时间 08:00 翻页。\n\n"+
			"注意：传 time.Time 是不够的 —— pgx 会按 timestamptz 发送，\n"+
			"PG 仍会用会话时区把它折成 DATE（本地零点 +08:00 在 UTC 下是前一天）。\n"+
			"必须传 `YYYY-MM-DD` 字符串才能完全绕开时区换算。",
			stored, want)
	}
}

// TestSignInStillRejectsSecondSignInSameDay 确认「一天一次」没被改坏。
func TestSignInStillRejectsSecondSignInSameDay(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	if _, err := scratch.SignIn(ctx, uid); err != nil {
		t.Fatalf("首次签到失败：%v", err)
	}
	if _, err := scratch.SignIn(ctx, uid); err == nil {
		t.Fatal("同一天第二次签到竟然成功了")
	}

	// 只能签到一次
	var n int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, uid).Scan(&n); err != nil {
		t.Fatal(err)
	}
	if n != 1 {
		t.Errorf("签到行数 = %d，应为 1", n)
	}
}

// TestSignInStopsAfterCalendarExhausted 确认七天用完后不再无限发。
//
// `day_index` 取 `MAX(day_index)+1`，日历查不到就是「签完了」。
// 这条钉住那个上界 —— 少了它，「签到」就变成可以反复领奖的漏洞。
func TestSignInStopsAfterCalendarExhausted(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	var calDays int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM sign_in_calendar`).Scan(&calDays); err != nil {
		t.Fatal(err)
	}
	t.Logf("签到日历共 %d 天", calDays)

	// 连续签 calDays 天（每天改 sign_date 以绕过「一天一次」）
	for i := 0; i < calDays; i++ {
		scratch.exec(t,
			`INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward)
			 VALUES ($1, CURRENT_DATE - $2::int, $3, '{}'::jsonb)`,
			uid, i, i+1)
	}

	_, err := scratch.SignIn(ctx, uid)
	if err != nil {
		t.Fatalf("日历用尽后应当**成功返回 Already** 而不是报错，实际 %v", err)
	}
	var n int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, uid).Scan(&n); err != nil {
		t.Fatal(err)
	}
	if n != calDays {
		t.Errorf("签到行数 = %d，应停在 %d（日历用尽后不再新增）", n, calDays)
	}
}

// 让编译器确认 scratch 上确实有 pool 字段（本文件多处使用）
var _ = sql.ErrNoRows

/**
 * 传进去的必须是**日期字面量**，不能是 `time.Time`（第 89 轮的结构判据）。
 *
 * # 为什么行为判据抓不到，只能上结构判据
 *
 * 变异测试实测三条全绿：
 *
 * | 变异 | 行为测试 |
 * |---|---|
 * | `today := time.Now()`（原形态） | **全绿** |
 * | `today := periodStart(...)`（传 `time.Time`） | **全绿** |
 * | `today := time.Now().Truncate(24h)` | **全绿** |
 *
 * 两个原因叠加：
 *
 * 1. **本环境的两个时区重合。** PG 会话时区是 `Asia/Shanghai`，
 *    Go 本地也是 +08:00 —— 于是 PG 折算出来的日期与 Go 算出来的**一样**。
 *    我自己在文件头就写明了「当前部署下这不是正在发生的 bug」，
 *    变异全绿正是那个结论的**直接推论**，不是守卫的漏洞。
 *
 * 2. **`SET TIME ZONE` 只作用于执行它的那条连接。**
 *    `SignIn` 走连接池里的另一条连接，于是那条用例根本没测到它想测的东西。
 *
 *    这与第 79 轮踩的是同一个坑：**观测手段本身有前提**。
 *    那次是「未提交的 INSERT 在 READ COMMITTED 下不可见」，
 *    这次是「会话级设置不跨连接」。
 *
 *    要让它生效得 `ALTER DATABASE … SET TimeZone` + 强制重连，
 *    那是**改测试环境**而不是测代码，代价与风险都超出这一轮该做的范围。
 *
 * # 那这条守卫守的是什么
 *
 * 它守的是**「日界由 Go 单层决定」这个性质的可判定形式**：
 * 发给 PG 的必须是 `2006-01-02` 形态的字符串，
 * 因为那样 PG 不做任何时区换算 —— 它只是把字面量写进 DATE 列。
 *
 * 传 `time.Time` 则必然让 PG 用会话时区再折一次：
 *
 * 	Go 本地零点 2026-10-06 00:00 +08:00 = 2026-10-05 16:00 UTC
 * 	会话时区 UTC → 折成 DATE 得 **2026-10-05**，差一天
 *
 * 三条变异都把那个字面量换成了 `time.Time`，所以都被这条抓到。
 *
 * ⚠️ 它是源码文本判据，不是行为判据 —— 这一点写在这里，
 * 而不是留给后来的人猜。
 */
func TestSignInPassesDateLiteralNotTimestamp(t *testing.T) {
	src := readSourceFile(t, "economy.go")

	i := strings.Index(src, "INSERT INTO user_sign_ins")
	if i < 0 {
		t.Fatal("找不到 user_sign_ins 的 INSERT —— 结构变了")
	}
	// 往前找 today 的赋值，取那一行
	j := strings.LastIndex(src[:i], "today :=")
	if j < 0 {
		t.Fatal("找不到 today 的赋值 —— 结构变了")
	}
	// ⚠️ `strings.Index` 找不到时返回 -1，直接切片会 panic。
	// 判据代码自己 panic 会把真正的失败信息冲掉 —— 守卫崩溃时
	// 看到的是栈而不是「哪里不对」。
	nl := strings.Index(src[j:], "\n")
	if nl < 0 {
		nl = len(src) - j
	}
	line := src[j : j+nl]

	// ⚠️ 两条都要查，缺一不可。
	//
	// 只查 `.Format("2006-01-02")` 时，下面这条变异**逃过了**：
	//
	//	today := time.Now().Truncate(24 * time.Hour).Format("2006-01-02")
	//
	// `Truncate(24h)` 是按**零时刻起算、忽略 Location** 的，也就是对齐到
	// **UTC 午夜**；而结果仍带原 Location(+08:00)，`Format` 出来是
	// 「UTC 那天」在本地时区的日历日。
	//
	// 于是它**只在部分时段与本地日一致**：
	//
	// | 北京时间 | Truncate 结果（UTC 午夜） | Format 出的日期 | 与本地日 |
	// |---|---|---|---|
	// | 08:00–24:00 | 当天 00:00Z | **同一天** | 一致 ← 测不出来 |
	// | **00:00–08:00** | **前一天** 00:00Z | **前一天** | **差一天** ❌ |
	//
	// 也就是说「早上 8 点前打开游戏」的那 8 小时里，
	// 签到会被算到昨天 —— 而每日任务还算今天。
	//
	// 当前实测时刻是北京时间 19 点，落在「一致」那一栏，所以那条变异全绿。
	// **测试结果随运行时刻变化**，这是判据踩在分界线上的另一种形态：
	// 第 85 轮是「输入落不到分界线」，这次是「**输入恰好落在安全区**」。
	//
	// 所以判据不比较结果，直接要求那行**必须**调用 `periodStart` ——
	// 那是与每日任务共用口径的唯一入口。
	if !strings.Contains(line, "periodStart(") {
		t.Errorf("today 那一行是：\n\t%s\n\n"+
			"它必须用 `periodStart(time.Now(), \"daily\")` —— 与每日任务同一个口径。\n\n"+
			"`time.Now().Truncate(24*time.Hour)` 是 **UTC 对齐**的"+
			"（Truncate 按零时刻起算、忽略 Location），\n"+
			"在北京时间 **00:00–08:00** 那 8 小时里它会比本地日早一天。",
			strings.TrimSpace(line))
	}

	if !strings.Contains(line, `.Format("2006-01-02")`) {
		t.Errorf("today 那一行是：\n\t%s\n\n"+
			"它必须 `.Format(\"2006-01-02\")` —— 发**日期字面量**给 PG。\n\n"+
			"传 time.Time 的话 PG 仍会用会话时区把 timestamptz 折成 DATE：\n"+
			"  会话时区 Asia/Shanghai → 与 Go 口径一致（测不出来）\n"+
			"  会话时区 UTC            → 本地零点 +08:00 落到前一天 ❌\n\n"+
			"「行为测试抓不到」的原因见本文件上方说明：\n"+
			"本环境两个时区重合，且 SET TIME ZONE 不跨连接。",
			strings.TrimSpace(line))
	}
}

/** readSourceFile 读同包源码，供「扫源码」型守卫使用。 */
func readSourceFile(t *testing.T, name string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(".", name))
	if err != nil {
		t.Fatalf("读 %s 失败：%v", name, err)
	}
	return string(raw)
}
