package service

// 第 141 轮：签到页的初始态（已签天数 / 今日可否签）必须由服务端回读（SignInStatus）。
// 判据：
//  1. 新用户 → 0 天、未签今日、可签
//  2. 「昨天签过」不得算成「今日已签」—— 日界口径必须与 SignIn 的 INSERT 完全一致
//  3. 今日已签 → 不可再签
//  4. 周期签满（claimed 达到 sign_in_calendar 行数）→ 不可再签

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestSignInStatusServiceMethod(t *testing.T) {
	ts := openScratchService(t) // 一次性库，已种子（sign_in_calendar 7 行）
	ctx := context.Background()

	var uid int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,'signin_status_probe',TRUE) RETURNING id`,
		fmt.Sprintf("signinstatus_%d", time.Now().UnixNano())).Scan(&uid); err != nil {
		t.Fatalf("造用户失败：%v", err)
	}

	today := periodStart(time.Now(), "daily")
	insert := func(dayIdx int, when time.Time) {
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward) VALUES ($1,$2,$3,$4)`,
			uid, when.Format("2006-01-02"), dayIdx, `{"coin":1000}`); err != nil {
			t.Fatalf("写签到记录失败：%v", err)
		}
	}

	// 新用户
	st, err := ts.SignInStatus(ctx, uid)
	if err != nil {
		t.Fatalf("查签到状态失败：%v", err)
	}
	if st.ClaimedCount != 0 || st.SignedToday || !st.CanSign {
		t.Errorf("新用户应为 0 天/未签今日/可签，实际 %+v", st)
	}

	// 签过两天，都在今天之前 → claimed=2、不算今日、今天仍可签
	insert(1, today.AddDate(0, 0, -2))
	insert(2, today.AddDate(0, 0, -1))
	st, _ = ts.SignInStatus(ctx, uid)
	if st.ClaimedCount != 2 {
		t.Errorf("应已签 2 天，实际 %d", st.ClaimedCount)
	}
	if st.SignedToday {
		t.Errorf("只签过今天之前的日子，不得判为今日已签（日界口径与 INSERT 不一致）：%+v", st)
	}
	if !st.CanSign {
		t.Errorf("周期未满且今日未签 → 应可签，实际 %+v", st)
	}

	// 今天签了 → 不可再签
	insert(3, today)
	st, _ = ts.SignInStatus(ctx, uid)
	if !st.SignedToday || st.CanSign {
		t.Errorf("今日已签：signed_today=true 且 can_sign=false，实际 %+v", st)
	}

	// 补满 7 天（周期完成）→ 不可再签
	for d := 4; d <= 7; d++ {
		insert(d, today.AddDate(0, 0, d-3))
	}
	st, _ = ts.SignInStatus(ctx, uid)
	if st.ClaimedCount != 7 || st.CanSign {
		t.Errorf("周期签满：claimed=7 且 can_sign=false，实际 %+v", st)
	}
}
