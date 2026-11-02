// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 22 轮：用户搜索大小写不敏感。
//
// PostgreSQL 的 `LIKE` 区分大小写：搜 "alice" 找不到昵称 "Alice" 的玩家。
// 防线名 / 兑换码搜索早已用 `ILIKE`（adminapi.go），用户列表/导出是漏网的
// 两处 —— 同一个后台里两种大小写语义，运营只会以为「搜不到=不存在」。
// 本测试钉住：大小写变体与大小写混合关键词都必须命中。

import (
	"context"
	"testing"
)

func TestAdminListUsersKeywordCaseInsensitive(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	// 直接改昵称为大小写混合（newUser 生成的昵称不可控）。
	ts.exec(t, `UPDATE users SET nickname = 'Alice_Wonder' WHERE id = $1`, uid)

	for _, kw := range []string{"alice", "ALICE", "Alice", "WONDER", "wonder"} {
		users, _, err := ts.AdminListUsers(ctx, kw, 50, 0, nil, nil, nil, nil, nil, nil)
		if err != nil {
			t.Fatalf("搜索 %q 失败：%v", kw, err)
		}
		found := false
		for _, u := range users {
			if u.ID == uid {
				found = true
			}
		}
		if !found {
			t.Errorf("大小写不敏感搜索 %q 应命中 Alice_Wonder（id=%d），实际未命中", kw, uid)
		}
	}
}
