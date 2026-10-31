package service

// 后台用户列表按游客/账号筛选的测试（2026-11-01 新一轮第 37 轮）。

import (
	"context"
	"testing"
)

func TestAdminListUsersIsGuestFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 一个游客（newGuestUser 直接插 users 行，is_guest 默认 TRUE）
	guestID, _ := newGuestUser(t, ts, "guest_f")
	// 一个账号（EnsureBootstrapPlayer 创建 is_guest=false）
	username := "guest_f_acct"
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, "test-password-12345"); err != nil {
		t.Fatalf("创建账号失败：%v", err)
	}

	// 只看游客 → 应包含 guestID，且账号不在其中
	items, total, err := ts.AdminListUsers(ctx, "", 50, 0, nil, nil, boolPtr(true), nil, nil, nil)
	if err != nil {
		t.Fatalf("游客筛选失败：%v", err)
	}
	if total < 1 {
		t.Errorf("游客筛选应至少 1 条，实际 %d", total)
	}
	foundGuest := false
	for _, u := range items {
		if u.ID == guestID {
			foundGuest = true
		}
		if !u.IsGuest {
			t.Errorf("游客筛选返回了非游客：%+v", u)
		}
	}
	if !foundGuest {
		t.Errorf("游客筛选应包含游客用户 %d", guestID)
	}

	// 只看账号 → 应包含账号，且游客不在其中
	items, total, err = ts.AdminListUsers(ctx, "", 50, 0, nil, nil, boolPtr(false), nil, nil, nil)
	if err != nil {
		t.Fatalf("账号筛选失败：%v", err)
	}
	if total < 1 {
		t.Errorf("账号筛选应至少 1 条，实际 %d", total)
	}
	for _, u := range items {
		if u.IsGuest {
			t.Errorf("账号筛选返回了游客：%+v", u)
		}
	}
}