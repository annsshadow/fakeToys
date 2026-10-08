package service

import (
	"context"
	"sort"
	"strings"
	"testing"
	"time"
)

// 任务周期的**权威清单**必须在三处保持一致（第 106 轮）。
//
// # 缺陷（HTTP 层）
//
// ```go
// scope := c.Query("scope", "daily")   // 从不校验
// items, err := s.Svc.LoadTasks(ctx, uid, scope)
// ```
//
// 而 `LoadTasks` 的过滤是 `WHERE t.scope = $3` → 匹配不到任何行 →
// **200 + 空列表**，且响应里 `"scope"` 仍**回显那个未知的值**。
//
// ## 实测（修前）
//
// ```
// ?scope=daily      status=200 len(items)=104  回显=daily
// ?scope=daliy      status=200 len(items)=0    回显=daliy
// ?scope=Daily      status=200 len(items)=0    回显=Daily
// ```
//
// # 为什么这比「返回错数据」更糟
//
// 玩家看到的是「**今天没有任务**」—— 一个**看起来完全合理**的答案。
// 而且它按玩家、按天出现，很可能永远不会被发现。
//
// （`leaderboard` 的 `type` 是同一个毛病，第 104 轮已修。）
//
// # 三处一致，缺一处就会漂移
//
//	1. `TaskScopes`          —— 权威清单（本轮新增）
//	2. `periodStart` 的 case —— 给清单加一项却忘加 case，它就会**静默落进 default**
//	3. `tasks` 表的 DISTINCT —— 种子数据新增第四种 scope 时，本轮加的校验会把它挡掉
//
// 第 2 条最危险：`periodStart` 的 `default` 分支**不报错**，
// 它只是悄悄按 daily 语义算 —— 而 HTTP 层会放行这个 scope
// （因为它在清单里），于是玩家拿到**日期错**的任务进度。

// TestTaskScopeListMatchesSeedData 确认权威清单与 `tasks` 表的取值**完全一致**。
//
// ⚠️ 差集要双向打出来：少一个（表里有、清单没有 → HTTP 层会挡掉真实数据）
// 与多一个（清单有、表里没有 → 玩家请求它得到空列表）都是错。
func TestTaskScopeListMatchesSeedData(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	rows, err := ts.pool.Query(ctx, `SELECT DISTINCT scope FROM tasks`)
	if err != nil {
		t.Fatalf("读 tasks.scope 失败：%v", err)
	}
	defer rows.Close()

	var inDB []string
	for rows.Next() {
		var s string
		if err := rows.Scan(&s); err != nil {
			t.Fatalf("扫描失败：%v", err)
		}
		inDB = append(inDB, s)
	}
	if err := rows.Err(); err != nil {
		t.Fatalf("遍历失败：%v", err)
	}

	if len(inDB) == 0 {
		t.Skip("tasks 表为空（未播种）—— 本条断言未执行，不是「通过」")
	}
	sort.Strings(inDB)
	list := append([]string(nil), TaskScopes...)
	sort.Strings(list)

	if strings.Join(inDB, ",") != strings.Join(list, ",") {
		t.Errorf("权威清单与 tasks 表不一致\n"+
			"  表里有：%v\n  清单是：%v\n\n"+
			"多出来的 scope 会被 HTTP 层的校验挡掉，玩家请求它只会拿到空列表；"+
			"漏掉的 scope 则**根本取不到**。\n"+
			"改 `TaskScopes` 时记得同步 seeder，两边都改。", inDB, list)
	}
}

// TestPeriodStartHandlesEveryKnownScope 用**期望表**逐项对拍。
//
// # 为什么不用「不能等于 default 分支的结果」
//
// `periodStart` 的结构是
//
//	case "achievement": …
//	case "weekly":      …
//	default:            return d      ← **daily 就是这个分支**
//
// 所以 `periodStart(wed, "daily")` 天然等于 default 的结果 ——
// 我第一版的判据对 `daily` **必然失败**，而且它给不出
// 「那 daily 到底该是什么」的答案。
//
// 换成期望表之后有两个好处：
//
//  1. 直接陈述每种周期的**正确答案**，而不是一个否定式
//  2. **给 `TaskScopes` 加一项时这张表必须同步** ——
//     忘了加，它会明确报「未知 scope」，而不是给出一个含糊的失败
func TestPeriodStartHandlesEveryKnownScope(t *testing.T) {
	// 2026-10-07 是**周三**。刻意避开周一：
	// daily 与 weekly 在周一是同一个日期，那样就分不出「忘加 weekly case」。
	wed := time.Date(2026, 10, 7, 15, 30, 0, 0, time.Local)
	mon := time.Date(2026, 10, 5, 15, 30, 0, 0, time.Local)
	dayOf := func(tm time.Time) time.Time {
		return time.Date(tm.Year(), tm.Month(), tm.Day(), 0, 0, 0, 0, tm.Location())
	}

	want := map[string]time.Time{
		"daily":       dayOf(wed),
		"weekly":      dayOf(mon), // 周一的零点
		"achievement": achievementEpoch,
	}

	for _, sc := range TaskScopes {
		expected, known := want[sc]
		if !known {
			t.Errorf("`TaskScopes` 新增了 %q，但期望表里没有它。\n\n"+
				"这是**故意的**：新增 scope 必须同时告诉 periodStart 该怎么算，"+
				"否则它会静默落进 default 分支按 daily 语义算 —— "+
				"而 HTTP 层会放行（它在清单里），玩家拿到的是**日期错**的任务进度。\n\n"+
				"请在这张表里补上它的期望日期。", sc)
			continue
		}
		got := periodStart(wed, sc)
		if !got.Equal(expected) {
			t.Errorf("scope=%q 在周三应得 %v，实际 %v", sc, expected, got)
		}
		if got.Weekday() != expected.Weekday() {
			t.Errorf("scope=%q 的星期不对：期望 %v，实际 %v", sc, expected.Weekday(), got.Weekday())
		}
	}

	// 清单里不得有重复项
	seenNames := map[string]bool{}
	for _, sc := range TaskScopes {
		if seenNames[sc] {
			t.Errorf("清单里有重复项 %q", sc)
		}
		seenNames[sc] = true
	}

	// 期望表里不该有清单外的项（否则那张表会假装覆盖一个不存在的 scope）
	for sc := range want {
		if !ValidTaskScope(sc) {
			t.Errorf("期望表里有 %q，但它不在 `TaskScopes` 里 —— 期望表与清单漂移了", sc)
		}
	}

	// 未在清单里的 scope 会落进 default（= daily 语义）。
	// 这里**把这件事写成断言**而不是假装它不存在：
	// 它是 `periodStart` 的公开行为，将来若改成 panic 或报错，这条会红。
	for _, unknown := range []string{"", "bogus", "Daily", "\x00"} {
		if got := periodStart(wed, unknown); !got.Equal(dayOf(wed)) {
			t.Errorf("未知 scope %q 落进 default 后应等于今日零点，实际 %v\n"+
				"⚠️ 若 `periodStart` 的 default 语义被改动（改成 panic / 报错），"+
				"请连同 HTTP 层的校验一起更新 —— 那是**有意的行为变更**。",
				unknown, got)
		}
	}

	// 周一是 weekly 的边界，也是上面刻意避开的日子 —— 单独确认它不是零值/未来
	for _, sc := range TaskScopes {
		d := periodStart(mon, sc)
		if d.IsZero() {
			t.Errorf("scope=%q 在周一返回了零值时间", sc)
		}
		if d.After(mon) {
			t.Errorf("scope=%q 在周一的起点 %v **晚于**当天 %v —— periodStart 应当只向前取整",
				sc, d, mon)
		}
	}
}

// TestValidTaskScope 覆盖 ValidTaskScope 的逐项行为。
//
// 与 HTTP 层的行为判据**分开**：
// 那是「端点返回什么」，这里是「判定函数本身对不对」。
// 混在一起时，端点的 400 可能是判定写错，也可能是别的 —— 分不出原因。
func TestValidTaskScope(t *testing.T) {
	for _, ok := range TaskScopes {
		if !ValidTaskScope(ok) {
			t.Errorf("%q 在清单里却判为非法", ok)
		}
	}
	for _, bad := range []string{
		"", " ", "Daily", "DAILY", "daliy", "dailies", "achievment",
		"\x00", "daily ", " daily", "周", "achievements",
	} {
		if ValidTaskScope(bad) {
			t.Errorf("%q 应当判为非法", bad)
		}
	}
}
