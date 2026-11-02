// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import "testing"

// TestClampLimit 分页条数的三态语义：非正数回落缺省、越界夹到上限、
// 区间内原样生效。
//
// 意图（不是「测一个三元 switch」）：`?limit=9999` 必须拿到**上限**，
// 不能与 `?limit=0` 得到同一个结果 —— 那意味着客户端「要更多」反而
// 拿到更少的行，且不报错（此前 19 处全是这种回落写法）。
func TestClampLimit(t *testing.T) {
	cases := []struct {
		in, def, max, want int
	}{
		{0, 50, 200, 50},
		{-3, 50, 200, 50},
		{1, 50, 200, 1},
		{49, 50, 200, 49},
		{50, 50, 200, 50},
		{200, 50, 200, 200},
		{201, 50, 200, 200},
		{999999, 50, 200, 200},
		{0, 20, 50, 20},
		{51, 20, 50, 50},
	}
	for _, c := range cases {
		if got := clampLimit(c.in, c.def, c.max); got != c.want {
			t.Errorf("clampLimit(%d, %d, %d) = %d，期望 %d", c.in, c.def, c.max, got, c.want)
		}
	}
}
