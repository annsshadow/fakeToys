// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/config"
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
	// ⚠️ R92：范围从「service 包」扩到**整个 internal 包树**。
	//
	// 第 99 轮把它从「两个文件」扩到整个 service 包，理由是对的 ——
	// 「只有这两处决定业务日界」会失效。R92 又发现同一形状的下一层：
	// **「只有 service 包写生产 SQL」同样会失效**。新增的 store 代码、
	// httpapi 里的查询，只要粘一句 `created_at >= CURRENT_DATE` 就绕过了守卫。
	//
	// 所以判据是「internal 下所有非测试 Go 源码」，不是「我已知的几处」。
	//
	// 例外（**有意不扫**，各写理由）：
	//
	//   - `migrations/*.sql`：一次性历史脚本，执行时的会话时区由 store.Open
	//     钉成业务时区；且它们不改写「当前请求」的日界。
	//   - `cmd/playertest`：压测工具，不跑生产流量；它写的假数据不进业务统计。
	//
	// ⚠️ 测试夹具由 TestTaskFixturesUseServiceDay 单独钉住
	// （只禁「写当期行」的 CURRENT_DATE，允许 `CURRENT_DATE - N` 造历史数据）。

	// 禁止的写法。全部是「日界由 PG 会话时区决定」的标记。
	//
	// 若某处**确实**需要 SQL 侧日界，正确写法是显式时区：
	// `(now() AT TIME ZONE 'Asia/Shanghai')::date` —— 不依赖会话配置，
	// 且能被 TestStatsDayBoundaryUsesExplicitBusinessZone 对拍。
	var offenders []string
	checked := 0

	walkErr := filepath.Walk("..", func(p string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}
		name := info.Name()
		if !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			return nil
		}
		checked++

		code := stripGoComments(readFileInPackage(t, p))
		serviceDir := filepath.Join("..", "service")
		for i, ln := range strings.Split(code, "\n") {
			low := strings.ToLower(ln)
			bad := ""
			switch {
			case strings.Contains(low, "current_date"):
				bad = "CURRENT_DATE（日界由 PG 会话时区决定）"
			case strings.Contains(low, "date_trunc") &&
				strings.Contains(low, "now()") && strings.Contains(low, "'day'"):
				bad = "date_trunc('day', now())（日界由 PG 会话时区决定）"
			case strings.Contains(low, "now()::date") || strings.Contains(low, "(now())::date"):
				bad = "now()::date（日界由 PG 会话时区决定）"
			}
			if bad != "" {
				offenders = append(offenders,
					p+":"+itoa(int64(i+1))+": "+bad+" → "+strings.TrimSpace(ln))
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
		if filepath.Dir(p) == serviceDir {
			for _, ln := range strings.Split(code, "\n") {
				if !strings.Contains(ln, "today :=") {
					continue
				}
				if !strings.Contains(ln, "periodStart(") {
					offenders = append(offenders,
						p+": today 赋值没有用 periodStart： "+strings.TrimSpace(ln)+
							"（`Truncate` 是 UTC 对齐的，北京时间 00:00–08:00 会比本地日早一天）")
				}
			}
		}
		return nil
	})
	if walkErr != nil {
		t.Fatalf("遍历 internal 目录失败：%v", walkErr)
	}

	// 自证：**扫过的文件数**必须 > 0，否则「零违规」与「目录读空了」无法区分。
	//
	// ⚠️ 与第 91 轮那条同源：守卫可以被「扫得少」而失效，
	// 而失效方式与「全部合规」一模一样。
	//
	// 「扫得少」与「全部合规」在输出里**一模一样** ——
	// 所以要有**下限**：internal 下的非测试源文件远多于这个数，
	// 缩到几个说明范围被人为收窄了。
	const minExpectedFiles = 30
	if checked < minExpectedFiles {
		t.Fatalf("只扫了 %d 个文件，期望至少 %d 个\n"+
			"「扫得少」与「全部合规」在输出里一模一样 —— "+
			"变异「把范围缩回几个已知文件」是全绿的。"+
			"新增源文件时这个下限要相应提高。",
			checked, minExpectedFiles)
	}

	if len(offenders) > 0 {
		sort.Strings(offenders)
		t.Errorf("以下 %d 处仍在 SQL 里决定业务「哪一天」（已扫 %d 个文件）：\n  %s\n\n"+
			"`CURRENT_DATE` / `date_trunc('day', now())` / `now()::date` "+
			"都由 **PG 会话时区**决定，而任务/签到/商城/挑战用 Go 的 "+
			"`periodStart(now, \"daily\")`（业务时区 Asia/Shanghai）。\n"+
			"容器里会话时区或宿主机时区一旦不是 Asia/Shanghai，"+
			"两者在北京时间 00:00–08:00 就差一天 —— 同一个自然日可以签到两次。\n"+
			"修法：Go 侧算出日期字面量传参；必须用 SQL 侧日界时显式写 "+
			"`AT TIME ZONE 'Asia/Shanghai'`。",
			len(offenders), checked, strings.Join(offenders, "\n  "))
	}
}

// TestStatsDayBoundaryUsesExplicitBusinessZone 钉住**SQL 侧日界**的唯一合法写法（R92）。
//
// # 改前
//
// `stats.go` 有 6 行 `CURRENT_DATE`（今日新增 / 今日战斗 / 今日挑战 /
// 今日签到 / 今日兑换 / 近 14 天曲线）。它由 **PG 会话时区**决定，
// 而游戏内的「今日」由 Go 的 `periodStart` 决定 —— 两个日界来源。
// 当时把它列为「运营看板口径，有意不改」，但那个豁免遮住了真正的问题：
// 只要会话时区与业务时区不一致（或 Go 进程时区是 UTC），
// 看板与游戏就落在不同的自然日，且**没人能看出来**。
//
// # 改后
//
// SQL 仍然自己取日界（看板的「今日」必须由数据库的 now() 决定，不能信
// 应用传入的日期字面量 —— 否则跨时区部署时看板会跟着服务器漂），
// 但**必须显式写业务时区**，不能依赖会话配置。
//
// 本用例做两件事：
//  1. 禁止 `CURRENT_DATE` 回来；
//  2. 所有 `AT TIME ZONE '...'` 的时区必须等于 `config.BusinessTimeZone`，
//     防止有人手写成 UTC 或另一个城市的日界。
func TestStatsDayBoundaryUsesExplicitBusinessZone(t *testing.T) {
	code := stripGoComments(readFileInPackage(t, "stats.go"))

	if n := strings.Count(code, "CURRENT_DATE"); n > 0 {
		t.Errorf("stats.go 仍有 %d 处裸 CURRENT_DATE —— "+
			"它由 PG 会话时区决定，与游戏内 periodStart 的业务日界是两套口径。\n"+
			"改成 `(now() AT TIME ZONE '%s')::date`。", n, config.BusinessTimeZone)
	}

	zoneExpr := "AT TIME ZONE '" + config.BusinessTimeZone + "'"
	n := strings.Count(code, zoneExpr)
	// 下限：今日新增 / 今日战斗 / 今日挑战 / 今日签到 / 今日兑换 / 14 天曲线(2)。
	// 至少 6 个才算「全部日界都显式带上了时区」；
	// 数量忽然变少说明有人删掉了其中一处 —— 那正是要发现的。
	const minExpected = 6
	if n < minExpected {
		t.Errorf("stats.go 只有 %d 处 `%s`，期望至少 %d 处\n"+
			"SQL 侧日界必须显式带业务时区，不能依赖会话配置。", n, zoneExpr, minExpected)
	}

	// 不允许出现别的时区字面量：`AT TIME ZONE 'UTC'` 这类写法
	// 在编译期看不出来，在跨时区部署时又是另一个「今日」。
	for i, ln := range strings.Split(code, "\n") {
		idx := strings.Index(ln, "AT TIME ZONE '")
		if idx < 0 {
			continue
		}
		rest := ln[idx+len("AT TIME ZONE '"):]
		end := strings.Index(rest, "'")
		if end < 0 {
			continue
		}
		if zone := rest[:end]; zone != config.BusinessTimeZone {
			t.Errorf("stats.go:%d 用了时区 %q，业务时区必须是 %q\n  %s",
				i+1, zone, config.BusinessTimeZone, strings.TrimSpace(ln))
		}
	}

	t.Logf("stats.go 有 %d 处显式业务时区（%s）—— SQL 侧日界不再依赖会话配置。",
		n, config.BusinessTimeZone)
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

// TestTaskFixturesUseServiceDay 把「日界口径」守卫从产码扩到**测试夹具**。
//
// # 为什么需要（2026-10-25 实测红过一次）
//
// `TestServiceHasNoCurrentDateInBusinessDaySQL` 只扫非 `_test.go`，于是夹具
// 可以继续用 `CURRENT_DATE` 写「今天」的 `user_tasks` 行 —— 而 `ClaimTask`
// 按 Go 的 `periodStart(nowFunc(), "daily")` 读同一行。
//
// 本环境 PG 会话是 `Asia/Shanghai`、Go 本地是 UTC，**16:00–24:00 UTC 这 8 小时
// 两者差一天**：夹具按 PG 日写、服务端按 Go 日读 → 读到 progress 0 →
// 报「任务未完成（0/30）」而不是用例期望的业务错误。
// 2026-10-25 16:00 UTC 的 CI 就是在这个窗口里红的（`TestTxMidwayFailures/ClaimTask`）。
//
// # 范围（刻意窄）
//
// 只钉「**写当期行**」这一种形态：`INSERT INTO user_tasks / user_task_events /
// user_daily_challenges` 且语句里带 `CURRENT_DATE`。
// 用 `CURRENT_DATE - N` **造历史数据**是合法的（`achievement_epoch_test` 就是），
// 不在本守卫范围 —— 所以判据不是「测试里不许出现 CURRENT_DATE」。
func TestTaskFixturesUseServiceDay(t *testing.T) {
	// 当期行落在「哪一天」必须与产码同源的表
	currentPeriodTables := []string{"user_tasks", "user_task_events", "user_daily_challenges"}

	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("读包目录失败：%v", err)
	}

	var offenders []string
	scanned := 0

	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, "_test.go") {
			continue
		}
		scanned++
		code := stripGoComments(readFileInPackage(t, name))

		for _, table := range currentPeriodTables {
			needle := "INSERT INTO " + table
			idx := 0
			for {
				at := strings.Index(code[idx:], needle)
				if at < 0 {
					break
				}
				start := idx + at
				idx = start + len(needle)
				// 取语句窗口：INSERT 起 400 字符内（够覆盖跨行的列清单与 VALUES）
				end := start + 400
				if end > len(code) {
					end = len(code)
				}
				if strings.Contains(code[start:end], "CURRENT_DATE") {
					line := strings.Count(code[:start], "\n") + 1
					offenders = append(offenders,
						name+":"+itoa(int64(line))+": INSERT INTO "+table+" 用了 CURRENT_DATE")
				}
			}
		}
	}

	if scanned == 0 {
		t.Fatal("没有扫到任何 _test.go —— 守卫失效（可能工作目录不对）")
	}
	if len(offenders) > 0 {
		t.Errorf("测试夹具用 CURRENT_DATE 写「当期」行，与服务端 Go 日口径不一致：\n  %s\n\n"+
			"改法：用 periodStart(nowFunc(), \"daily\") 算日期并作为参数传入。\n"+
			"（本环境 PG=Asia/Shanghai、Go=UTC，16:00–24:00 UTC 差一天，\n"+
			" 夹具写进去的行服务端读不到 → 报「任务未完成（0/30）」。）",
			strings.Join(offenders, "\n  "))
	}
}
