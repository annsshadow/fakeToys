package service

// SignInStatus 复用签到日历缓存的集成判据（2026-10-31 新一轮第 5 轮）。
//
// 修前 `SignInStatus` 每次请求都要 `SELECT COUNT(*) FROM sign_in_calendar`
// 单独查一次库；修后复用 `SignInCalendar` 的 TTL 缓存（seeder 静态配置）。
//
// 判据沿用本仓的「改底表」手法：**首次调用成功之后，把 sign_in_calendar 改名**。
// 第二次调用若仍成功，说明周期长度来自缓存 —— 那正是「复用缓存」的定义。
// 墙钟断言在 CI 上易抖，而「表没了还能返回」是确定性的。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip —— 那一刻这条断言是
// **未执行**，不是「通过」。

import (
	"context"
	"testing"
)

func TestSignInStatusReusesCalendarCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)

	// 首次调用：加载日历并填充缓存
	if _, err := ts.SignInStatus(ctx, uid); err != nil {
		t.Fatalf("首次 SignInStatus 失败：%v", err)
	}

	// 把签到日历表改名 —— 之后任何真实查询都必然报错
	ts.renameTable(t, "sign_in_calendar", "sign_in_calendar_bak")
	defer ts.renameTable(t, "sign_in_calendar_bak", "sign_in_calendar")

	// TTL 内第二次调用应命中日历缓存（user_sign_ins 查询仍正常）
	st, err := ts.SignInStatus(ctx, uid)
	if err != nil {
		t.Fatalf("TTL 内第二次调用应命中日历缓存（日历表已改名，查库必失败），实际报错：%v", err)
	}
	// 新用户：claimed=0、今日未签、周期长来自缓存（7）→ 可签。
	// 若缓存未生效，日历行数会读成 0/报错，can_sign 会错。
	if !st.CanSign {
		t.Errorf("新用户在 7 天周期内且今日未签应可签，实际 %+v", st)
	}
}