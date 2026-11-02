// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 后台用户列表按游客/账号筛选的测试（2026-11-01 新一轮第 37 轮）。

import (
	"bytes"
	"context"
	"encoding/csv"
	"strconv"
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

// 导出口径必须与列表一致：运营在列表里筛了「只看游客」，导出不能把账号也带上。
// 改前 AdminExportUsers 没有 is_guest 参数，这里两种筛选导出的行数都等于全量 →
// 与 AdminListUsers 的 total 对不上。
func TestAdminExportUsersIsGuestFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	newGuestUser(t, ts, "export_guest")
	if _, err := ts.EnsureBootstrapPlayer(ctx, "export_guest_acct", "test-password-12345"); err != nil {
		t.Fatalf("创建账号失败：%v", err)
	}

	for _, want := range []bool{true, false} {
		_, total, err := ts.AdminListUsers(ctx, "", 50, 0, nil, nil, boolPtr(want), nil, nil, nil)
		if err != nil {
			t.Fatalf("列表筛选 is_guest=%v 失败：%v", want, err)
		}
		data, err := ts.AdminExportUsers(ctx, "", nil, nil, boolPtr(want), nil, nil, nil)
		if err != nil {
			t.Fatalf("导出 is_guest=%v 失败：%v", want, err)
		}
		records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
		if err != nil {
			t.Fatalf("解析 CSV 失败：%v", err)
		}
		if got := int64(len(records) - 1); got != total {
			t.Fatalf("is_guest=%v 导出行数 %d 与列表 total %d 不一致", want, got, total)
		}
		if total == 0 {
			t.Fatalf("is_guest=%v 夹具失效：两个桶都应有用户", want)
		}
		for _, rec := range records[1:] {
			if rec[2] != strconv.FormatBool(want) {
				t.Errorf("is_guest=%v 导出了 is_guest=%s 的行：%v", want, rec[2], rec)
			}
		}
	}
}
