package service

// 后台按 user_id 精确搜索的服务层测试（2026-10-31 实质优化第 31 轮）。
//
// 关键行为锚点：
//   - keyword 是数字时，`AdminListUsers` 精确匹配该 user_id；
//   - 非数字 keyword 不误匹配 id（仍走昵称模糊）；
//   - 数字 keyword 不存在时返回空。

import (
	"context"
	"fmt"
	"strconv"
	"testing"
	"time"
)

func TestAdminListUsersSearchByID(t *testing.T) {
	// 用一次性 scratch 库：共享库的残留用户会让 total==1 断言不可靠。
	ts := openScratchService(t)
	ctx := context.Background()

	// 两个用户：一个昵称含 "searchme"，一个不含
	usernameA := fmt.Sprintf("idsearch_a_%d", time.Now().UnixNano())
	usernameB := fmt.Sprintf("idsearch_b_%d", time.Now().UnixNano())
	if _, err := ts.EnsureBootstrapPlayer(ctx, usernameA, "test-password-12345"); err != nil {
		t.Fatalf("引导账号 A 失败：%v", err)
	}
	if _, err := ts.EnsureBootstrapPlayer(ctx, usernameB, "test-password-12345"); err != nil {
		t.Fatalf("引导账号 B 失败：%v", err)
	}

	var idA int64
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT id FROM users WHERE username = $1`, usernameA).Scan(&idA); err != nil {
		t.Fatalf("查询账号 A 失败：%v", err)
	}
	// 把 B 的昵称改成不含数字的纯文本，避免被 A 的 id（数字）模糊匹配命中。
	var idB int64
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT id FROM users WHERE username = $1`, usernameB).Scan(&idB); err != nil {
		t.Fatalf("查询账号 B 失败：%v", err)
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`UPDATE users SET nickname = '用户B' WHERE id = $1`, idB); err != nil {
		t.Fatalf("修改 B 昵称失败：%v", err)
	}

	// 按 id 精确搜索 → 只返回 A
	items, total, err := ts.AdminListUsers(ctx, strconv.FormatInt(idA, 10), 10, 0, nil, nil, nil)
	if err != nil {
		t.Fatalf("按 id 搜索失败：%v", err)
	}
	if total != 1 {
		t.Errorf("total 应为 1，实际 %d", total)
	}
	if len(items) != 1 || items[0].ID != idA {
		t.Errorf("应精确匹配 user_id=%d，实际 %+v", idA, items)
	}
}

func TestAdminListUsersIDSearchNoMatch(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 不存在的 id → 空结果
	items, total, err := ts.AdminListUsers(ctx, "999999999", 10, 0, nil, nil, nil)
	if err != nil {
		t.Fatalf("按 id 搜索失败：%v", err)
	}
	if total != 0 || len(items) != 0 {
		t.Errorf("不存在的 id 应返回空，实际 total=%d items=%+v", total, items)
	}
}

func TestAdminListUsersNonNumericKeywordStillSearchesNickname(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("nicksearch_%d", time.Now().UnixNano())
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, "test-password-12345"); err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}

	// 非数字 keyword 不应触发 id 匹配，仍按昵称模糊搜索（昵称=用户名）
	items, total, err := ts.AdminListUsers(ctx, "nicksearch_", 10, 0, nil, nil, nil)
	if err != nil {
		t.Fatalf("按昵称搜索失败：%v", err)
	}
	if total < 1 {
		t.Errorf("按昵称搜索应有结果，实际 total=%d", total)
	}
	found := false
	for _, it := range items {
		if it.Nickname == username {
			found = true
		}
	}
	if !found {
		t.Error("搜索结果应包含测试用户")
	}
}
