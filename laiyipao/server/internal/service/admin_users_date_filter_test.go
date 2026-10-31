package service

// 后台用户列表/导出按注册日期范围过滤的测试（2026-10-31 新一轮第 15 轮）。
//
// 运营想查「今天/某时间段注册的新用户」，此前只能靠看板计数，
// 无法进入列表逐条处理。新增可选 date_from / date_to。

import (
	"context"
	"strings"
	"testing"
	"time"
)

func TestAdminListUsersDateFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库的跨测试数据污染
	ctx := context.Background()

	// 三个用户：注册于 3 小时前 / 2 小时前 / 1 小时前
	now := time.Now()
	oldID := newUserWithCreatedAt(t, ts, "udate_old", now.Add(-3*time.Hour))
	midID := newUserWithCreatedAt(t, ts, "udate_mid", now.Add(-2*time.Hour))
	newID := newUserWithCreatedAt(t, ts, "udate_new", now.Add(-1*time.Hour))
	_ = midID
	_ = newID

	from := now.Add(-150 * time.Minute) // -2.5h
	to := now.Add(-90 * time.Minute)    // -1.5h

	// 只传 date_from：包含 -2h / -1h，不含 -3h
	list, total, err := ts.AdminListUsers(ctx, "", 50, 0, &from, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("date_from 过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("date_from=-2.5h 应剩 2 条，实际 %d", total)
	}
	for _, u := range list {
		if u.ID == oldID {
			t.Errorf("date_from 过滤不应包含 -3h 注册的用户")
		}
	}

	// 只传 date_to（开区间 <）：应只含 -3h 注册的用户
	list, total, err = ts.AdminListUsers(ctx, "", 50, 0, nil, &from, nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("date_to 过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != oldID {
		t.Errorf("date_to=-2.5h 应只含 -3h 用户，实际 total=%d", total)
	}

	// 双端 [from, to)：应只含 -2h 注册的用户（-1h 在开区间外）
	list, total, err = ts.AdminListUsers(ctx, "", 50, 0, &from, &to, nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("双端过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != midID {
		t.Errorf("双端 [-2.5h,-1.5h) 应只含 -2h 用户，实际 total=%d", total)
	}
}

func TestAdminExportUsersDateFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	now := time.Now()
	newUserWithCreatedAt(t, ts, "udate_exp_old", now.Add(-3*time.Hour))
	newUserWithCreatedAt(t, ts, "udate_exp_mid", now.Add(-2*time.Hour))
	newUserWithCreatedAt(t, ts, "udate_exp_new", now.Add(-1*time.Hour))

	from := now.Add(-150 * time.Minute)
	data, err := ts.AdminExportUsers(ctx, "", &from, nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("date_from 导出失败：%v", err)
	}
	// CSV 首行是表头；范围内应有 2 个用户
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 3 {
		t.Errorf("date_from=-2.5h 导出行数应为 3（1 表头 + 2 数据），实际 %d", len(lines))
	}
}

// newUserWithCreatedAt 直接插入用户并覆盖注册时间。
func newUserWithCreatedAt(t *testing.T, ts *testService, nickname string, at time.Time) int64 {
	t.Helper()
	ctx := context.Background()
	var id int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest, created_at, last_login_at)
		 VALUES ($1, $2, TRUE, $3, $3)
		 RETURNING id`,
		"gt_"+nickname, nickname, at).Scan(&id); err != nil {
		t.Fatalf("插入用户失败：%v", err)
	}
	return id
}