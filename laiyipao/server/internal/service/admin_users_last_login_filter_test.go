package service

// 后台用户列表/导出按最后登录时间范围过滤的测试（2026-11-01 新一轮第 46 轮）。
//
// 运营此前只能按注册日期筛用户 —— 但「昨天还在线的玩家」与
// 「昨天注册的玩家」是两批完全不同的人：注册日期筛选回答不了活跃度问题。
// 新增可选 last_login_from / last_login_to。

import (
	"context"
	"strings"
	"testing"
	"time"
)

// newUserWithLastLogin 造一个用户并单独设置最后登录时间。
func newUserWithLastLogin(t *testing.T, ts *testService, nickname string, lastLogin time.Time) int64 {
	t.Helper()
	ctx := context.Background()
	now := time.Now()
	id := newUserWithCreatedAt(t, ts, nickname, now)
	if _, err := ts.db.Pool.Exec(ctx,
		`UPDATE users SET last_login_at = $2 WHERE id = $1`, id, lastLogin); err != nil {
		t.Fatalf("设置最后登录时间失败：%v", err)
	}
	return id
}

func TestAdminListUsersLastLoginFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库的跨测试数据污染
	ctx := context.Background()

	now := time.Now()
	oldID := newUserWithLastLogin(t, ts, "ll_old", now.Add(-3*time.Hour))
	midID := newUserWithLastLogin(t, ts, "ll_mid", now.Add(-2*time.Hour))
	newID := newUserWithLastLogin(t, ts, "ll_new", now.Add(-1*time.Hour))

	from := now.Add(-150 * time.Minute) // -2.5h
	to := now.Add(-90 * time.Minute)    // -1.5h

	// 只传 last_login_from：包含 -2h / -1h，不含 -3h
	list, total, err := ts.AdminListUsers(ctx, "", 50, 0, nil, nil, nil, &from, nil)
	if err != nil {
		t.Fatalf("last_login_from 过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("last_login_from=-2.5h 应剩 2 条，实际 %d", total)
	}
	for _, u := range list {
		if u.ID == oldID {
			t.Errorf("last_login_from 过滤不应包含 -3h 登录的用户")
		}
	}

	// 只传 last_login_to（开区间 <）：应只含 -3h 登录的用户
	list, total, err = ts.AdminListUsers(ctx, "", 50, 0, nil, nil, nil, nil, &from)
	if err != nil {
		t.Fatalf("last_login_to 过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != oldID {
		t.Errorf("last_login_to=-2.5h 应只含 -3h 用户，实际 total=%d", total)
	}

	// 双端 [from, to)：应只含 -2h 登录的用户
	list, total, err = ts.AdminListUsers(ctx, "", 50, 0, nil, nil, nil, &from, &to)
	if err != nil {
		t.Fatalf("双端过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != midID {
		t.Errorf("双端 [-2.5h,-1.5h) 应只含 -2h 用户，实际 total=%d", total)
	}

	// 新用户（-1h）在开区间外（date_to 开区间）
	for _, u := range list {
		if u.ID == newID {
			t.Errorf("双端过滤不应包含 -1h 登录的用户")
		}
	}
	_ = ctx
}

func TestAdminExportUsersLastLoginFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	now := time.Now()
	_ = newUserWithLastLogin(t, ts, "ll_exp_old", now.Add(-3*time.Hour))
	_ = newUserWithLastLogin(t, ts, "ll_exp_new", now.Add(-1*time.Hour))

	from := now.Add(-2 * time.Hour)
	data, err := ts.AdminExportUsers(ctx, "", nil, nil, &from, nil)
	if err != nil {
		t.Fatalf("last_login_from 导出失败：%v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Errorf("last_login_from=-2h 导出行数应为 2（1 表头 + 1 数据），实际 %d", len(lines))
	}

	// 空筛选：两条都在
	data, err = ts.AdminExportUsers(ctx, "", nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("空筛选导出失败：%v", err)
	}
	lines = strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 3 {
		t.Errorf("空筛引导出行数应为 3（1 表头 + 2 数据），实际 %d", len(lines))
	}
}