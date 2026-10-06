package service

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// 商城的「今日」必须与每日任务的「今日」是**同一套口径**（第 97 轮）。
//
// # 缺陷形态：两套日界并存
//
// | 路径 | 日界来源 |
// |---|---|
// | 每日任务 | Go 的 `periodStart(now, "daily")`（用 `time.Local`） |
// | 每日签到 | 第 89 轮已统一到 Go |
// | **商城限购** | **PG 会话时区**（`CURRENT_DATE`） |
//
// 商城三处（`LoadShop` 计数 / `Buy` 计数 / `Buy` 写入）**都用** `CURRENT_DATE`，
// 所以商城**内部是自洽的** —— 这也是它一直没被发现的原因：
// 单独测商城永远自洽，只有把商城和任务放在一起才看得出两边日界不同。
//
// # 为什么第 90 轮没解决
//
// 第 90 轮把 PG 会话时区钉成 `Asia/Shanghai`，但**没有钉 Go 那一侧** ——
// 容器里 `time.Local` 常常是 UTC。
//
// 两者不一致时（Go=UTC、PG=Asia/Shanghai）：
//
//	每日任务：00:00 **UTC** = 北京 08:00 刷新
//	商城限购：00:00 Asia/Shanghai = 北京 00:00 刷新
//
// 于是玩家早上 8 点看到任务已刷新，商城却还显示**昨天**的限购次数。
//
// # 修法：与第 89 轮签到同一手法 —— 传**日期字面量**
//
// 让日界只由 Go 一处决定，PG 完全不参与换算。
func TestShopDayBoundaryMatchesTaskDayBoundary(t *testing.T) {
	// 行为判据：买到的记录落在**任务口径**的那一天
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 找一个不限购的商品
	var itemID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM shop_items WHERE enabled ORDER BY id LIMIT 1`).Scan(&itemID); err != nil {
		t.Skipf("商城是空的：%v", err)
	}

	ts.grant(t, ctx, uid, map[string]int64{"coin": 100_000_000, "gem": 100_000})

	if _, err := ts.Buy(ctx, uid, itemID); err != nil {
		t.Fatalf("购买失败：%v", err)
	}

	// 读回的 purchase_date 必须等于 Go 口径的今天
	var got string
	if err := ts.pool.QueryRow(ctx,
		`SELECT purchase_date::text FROM user_purchases
		  WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).Scan(&got); err != nil {
		t.Fatal(err)
	}
	want := periodStart(time.Now(), "daily").Format("2006-01-02")
	if got != want {
		t.Errorf("purchase_date = %s，任务口径的今天是 %s\n\n"+
			"两者不一致意味着商城的「今日」与任务的「今日」在不同时刻翻页。",
			got, want)
	}

	// ⚠️ 关键：**即使 PG 会话时区被改成别的，结论也必须成立**。
	//
	// 这里无法真的改会话时区（`SET TIME ZONE` 不跨连接，
	// 第 89/90 轮都踩过），所以用源码判据补上。
}

// TestShopHasNoCurrentDate 是**源码**判据：service 包里不得再出现 `CURRENT_DATE`。
//
// ⚠️ 为什么这条比行为判据更强：
// 行为判据在「PG 会话时区 == Go 本地」时**观察不到差别** ——
// 那正是本环境的现状（第 90 轮实测 PG 是 Asia/Shanghai）。
//
// 而 `CURRENT_DATE` 这个**写法本身**就是「日界由 PG 决定」的标记，
// 它可判定、不依赖环境。
//
// ⚠️ 必须剥注释 —— 否则上面那段解释 `CURRENT_DATE` 的注释会被当成代码。
// 本项目已因「文本扫描撞注释」踩坑四次（第 83/88/94 轮）。
func TestShopHasNoCurrentDate(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(".", "economy.go"))
	if err != nil {
		t.Fatalf("读 economy.go 失败：%v", err)
	}
	code := stripGoComments(string(raw))

	// ⚠️ 变异测试实测：把 `periodStart(time.Now(), "daily")` 换成
	// `time.Now().Truncate(24 * time.Hour)` → **全绿通过**。
	//
	// 原因与第 89 轮那条完全相同：`Truncate` 按**零时刻起算、忽略 Location**，
	// 它对齐的是 **UTC 午夜**，而结果仍带 `+08:00` Location。
	// 于是它**只在部分时段与本地日一致**：
	//
	// | 北京时间 | Truncate 结果 | 与本地日 |
	// |---|---|---|
	// | 08:00–24:00 | 同一天 | 一致 ← 测不出来 |
	// | **00:00–08:00** | **前一天** | **差一天** ❌ |
	//
	// 当前实测时刻在「一致」那一栏 —— **测试结果随运行时刻变化**。
	// 这是判据踩在分界线上的第三种形态：
	// 第 85 轮「输入落不到分界线」，第 92 轮「输入恰好落在安全区」，
	// 这次是「**时刻**恰好落在安全区」。
	//
	// 所以判据不比较结果，直接要求那行**必须**调用 `periodStart` ——
	// 那是与每日任务共用口径的唯一入口。
	for _, line := range strings.Split(code, "\n") {
		if !strings.Contains(line, "today :=") {
			continue
		}
		if !strings.Contains(line, "periodStart(") {
			t.Errorf("economy.go 里的 today 赋值是：\n\t%s\n\n"+
				"它必须用 `periodStart(time.Now(), \"daily\")` —— 与每日任务同一个口径。\n\n"+
				"`time.Now().Truncate(24*time.Hour)` 是 **UTC 对齐**的"+
				"（按零时刻起算、忽略 Location），\n"+
				"在北京时间 **00:00–08:00** 那 8 小时里它会比本地日早一天 —— "+
				"而当前时刻恰好落在「一致」那一栏，所以行为判据看不出来。",
				strings.TrimSpace(line))
		}
	}

	if strings.Contains(code, "CURRENT_DATE") {
		// 给出具体行号，让人一眼看到是哪一处
		lines := strings.Split(code, "\n")
		bad := []string{}
		for i, ln := range lines {
			if strings.Contains(ln, "CURRENT_DATE") {
				bad = append(bad, itoa(int64(i+1))+": "+strings.TrimSpace(ln))
			}
		}
		t.Errorf("economy.go 里还有 %d 处 `CURRENT_DATE`（已剥注释）：\n  %s\n\n"+
			"商城与每日任务必须共用同一个日界口径 ——\n"+
			"`CURRENT_DATE` 由 **PG 会话时区**决定，而任务/签到用 Go 的 `periodStart`。\n"+
			"请改成传日期字面量（与第 89 轮签到、LoadShop 的写法一致）。",
			len(bad), strings.Join(bad, "\n  "))
	}
}

// stripGoComments 剥掉 Go 的行注释与块注释。
//
// 文本级剥离：会误伤字符串字面量里的 `//`。
// 那类写法在本文件里不存在，且误伤代价是「有人要来解释这条为什么红了」。
func stripGoComments(src string) string {
	var out []string
	inBlock := false
	for _, line := range strings.Split(src, "\n") {
		l := line
		if inBlock {
			if i := strings.Index(l, "*/"); i >= 0 {
				l = l[i+2:]
				inBlock = false
			} else {
				continue
			}
		}
		if i := strings.Index(l, "/*"); i >= 0 {
			if j := strings.Index(l[i:], "*/"); j >= 0 {
				l = l[:i] + " " + l[i+j+2:]
			} else {
				l = l[:i]
				inBlock = true
			}
		}
		if i := strings.Index(l, "//"); i >= 0 {
			l = l[:i]
		}
		out = append(out, l)
	}
	return strings.Join(out, "\n")
}

// TestStripGoCommentsWorks 确认剥离器本身可用 ——
// 「扫描器是否可用」与「代码是否合规」必须分开回答。
func TestStripGoCommentsWorks(t *testing.T) {
	in := []string{
		"x := 1 // 注释里的 CURRENT_DATE",
		"/*",
		" 块注释里的 CURRENT_DATE",
		"*/",
		"y := 2",
	}
	got := stripGoComments(strings.Join(in, "\n"))

	// ⚠️ 断言必须是「注释里的那些字面量消失了」，
	// 而不是「整个文件里没有 CURRENT_DATE」——
	// 后者要求把**真代码**里出现的也剥掉，那是错的。
	if strings.Contains(got, "注释里的") || strings.Contains(got, "块注释里的") {
		t.Errorf("剥离器没剥干净：\n%s", got)
	}
	if !strings.Contains(got, "x := 1") || !strings.Contains(got, "y := 2") {
		t.Errorf("剥离器把真代码也剥掉了：\n%s", got)
	}
}

func TestStripGoCommentsKeepsRealCode(t *testing.T) {
	// 正向断言：**真代码里的** CURRENT_DATE 必须留着。
	//
	// ⚠️ 没有这一条时，上面那条断言可以用「把真代码也剥掉」的方式通过 ——
	// 那是另一种失效形态：守卫为了通过而把被测对象一起吃了。
	in := "y := CURRENT_DATE"
	if got := stripGoComments(in); got != in {
		t.Errorf("真代码被误剥：%q -> %q", in, got)
	}
}
