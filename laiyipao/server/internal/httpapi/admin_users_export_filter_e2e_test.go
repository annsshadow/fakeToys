// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 用户导出 CSV 的筛选口径必须与列表一致（R29）。
//
// 运营的工作流是「列表里筛出问题人群 → 导出交给人处理」。
// 若导出少一个筛选条件，导出的就是**人群外的全量数据** —— 而且没有任何提示。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"testing"
	"time"
)

func TestAdminExportUsersIsGuestFilterE2E(t *testing.T) {
	e := newE2E(t)
	ctx := context.Background()
	_, tok := e.newAdmin(t, "export-guest")

	tag := fmt.Sprintf("EXP%d", time.Now().UnixNano())
	// 一个游客（nickname = E2E_<tag>_g）
	e.newPlayer(t, tag+"_g")
	// 一个非游客账号（nickname = username）
	acct := "E2E_" + tag + "_acct"
	if _, err := e.svc.EnsureBootstrapPlayer(ctx, acct, "test-password-12345"); err != nil {
		t.Fatalf("创建账号失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = e.pool.Exec(ctx, `DELETE FROM users WHERE nickname = $1`, acct)
	})

	for _, want := range []string{"true", "false"} {
		_, list := e.get(t, fmt.Sprintf("/api/v1/admin/users?keyword=%s&is_guest=%s&limit=50", tag, want), tok)
		total, ok := list["total"].(float64)
		if !ok {
			t.Fatalf("列表 total 缺失：%v", list)
		}
		status, raw := e.getRaw(t, fmt.Sprintf("/api/v1/admin/users/export?keyword=%s&is_guest=%s", tag, want), tok)
		if status != 200 {
			t.Fatalf("导出 is_guest=%s 应 200，实际 %d", want, status)
		}
		records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(raw, []byte("\xEF\xBB\xBF")))).ReadAll()
		if err != nil {
			t.Fatalf("解析 CSV 失败：%v", err)
		}
		if got := len(records) - 1; float64(got) != total {
			t.Fatalf("is_guest=%s 导出 %d 行，列表 total=%v（口径不一致）", want, got, total)
		}
		if total < 1 {
			t.Fatalf("is_guest=%s 夹具失效：两个桶都应有匹配用户", want)
		}
	}
}
