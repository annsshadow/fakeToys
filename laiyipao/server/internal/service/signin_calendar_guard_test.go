package service

// 第 135 轮：签到日历作为**权威来源**下发给客户端（服务端 SignInCalendar）。
// 判据：返回的是库里 sign_in_calendar 的实际内容（顺序、天数、奖励形状），
// 而不是客户端本地硬编码公式的副本。

import (
	"context"
	"testing"
)

func TestSignInCalendarServiceMethod(t *testing.T) {
	ts := openScratchService(t) // 已灌种子，含 sign_in_calendar
	ctx := context.Background()

	days, err := ts.SignInCalendar(ctx)
	if err != nil {
		t.Fatalf("读取签到日历失败：%v", err)
	}
	// 结构不变量：七行、按天升序、每天奖励非空对象
	if len(days) != 7 {
		t.Fatalf("签到日历应为 7 天，实际 %d", len(days))
	}
	for i, d := range days {
		if d.Day != i+1 {
			t.Errorf("第 %d 行 day_index=%d（应为 %d）", i, d.Day, i+1)
		}
		if len(d.Reward) == 0 {
			t.Errorf("第 %d 天奖励为空 —— 日历行不完整", d.Day)
		}
	}
	// 与 seeder 契约：第 1 天给金币（coin），第 7 天额外给体力（energy）
	if days[0].Reward["coin"] == 0 {
		t.Errorf("第 1 天应有金币奖励，实际 %v", days[0].Reward)
	}
	if days[6].Reward["energy"] == 0 {
		t.Errorf("第 7 天应有体力奖励，实际 %v", days[6].Reward)
	}
}
