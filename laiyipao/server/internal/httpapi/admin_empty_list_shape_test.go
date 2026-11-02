// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 管理端「空列表」的 wire 形状契约。
//
// 后台列表的行类型来自 service 的 `var out []X`（零行时是 nil），
// handler 用 `NormalizeNilSlices(AdminXResp{...})` 归一后写出。
// 但该归一器对**传值**调用曾空转，于是空列表仍是 `"items": null` ——
// 前端按 `T[]` 调 .map/.length 直接 TypeError。
//
// 这里不造数据：用一个必然匹配不到任何行的筛选值，读真实端点。

import (
	"fmt"
	"testing"
	"time"
)

func TestAdminEmptyListIsArrayNotNull(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "empty-shape")

	// 该筛选值在开发库必然无匹配（含纳秒时间戳）。
	miss := fmt.Sprintf("__no_such_%d", time.Now().UnixNano())
	cases := []struct {
		url string
	}{
		{"/api/v1/admin/defenses?keyword=" + miss},
		{"/api/v1/admin/audit-logs?action=" + miss},
	}
	for _, c := range cases {
		status, body := e.get(t, c.url, tok)
		if status != 200 {
			t.Fatalf("%s 应 200，实际 %d", c.url, status)
		}
		items, ok := body["items"]
		if !ok {
			t.Fatalf("%s 响应缺 items（顶层键 %v）", c.url, keysOf(body))
		}
		if items == nil {
			t.Fatalf("%s 空列表编码成了 null，前端按数组用会崩", c.url)
		}
		if _, ok := items.([]any); !ok {
			t.Fatalf("%s items 不是数组，实际类型 %T", c.url, items)
		}
		if len(items.([]any)) != 0 {
			t.Fatalf("%s 夹具失效：筛选值命中了 %d 行", c.url, len(items.([]any)))
		}
	}
}
