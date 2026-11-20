// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

// mulDiv 溢出边界回归。
//
// 行为锚点：damage.go 里 mulDiv 的注释写「内部用 int64 防溢出」，
// 而 miniapp/src/game/fixed.ts 的文件头写两侧函数「必须逐行等价」——
// TS 侧用 bigint，(a*b)/c 永不溢出。
// 因此即便当前战斗内容表的量级碰不到该边界，函数的**契约**
// （防溢出、跨端一致）必须先于实现正确：
// 对同一个输入，Go 必须给出与 TS BigInt 相同的结果。
//
// 取 a = 6e18、b = 1450、c = 1000：
//   真实结果 = 8.7e18，在 int64 范围内（MaxInt64 ≈ 9.22e18）——
//   精确值 8700000000000000000 确实 ≤ 9223372036854775807；
//   而中间乘积 a*b = 8.7e21 远超 int64，Go 的 (a*b)/c 静默回绕成错值。
// 这就是 engine_pure_contract_test.go 注释里说的
// 「结果装得下但中间装不下」区间 —— 一旦底伤上限调高就会踩到。

import (
	"math"
	"testing"
)

func TestMulDivNoIntermediateOverflow(t *testing.T) {
	// 契约向量：结果装得下，中间乘积装不下。
	cases := []struct {
		a, b, c, want int64
		why           string
	}{
		{6_000_000_000_000_000_000, 1450, 1000, 8_700_000_000_000_000_000,
			"中间乘积 8.7e21 溢出，但商 8.7e18 在 int64 内；TS BigInt 得精确商"},
		{math.MaxInt64, 2, 2, math.MaxInt64,
			"MaxInt64*2 回绕成 -2，再除 2 得 -1；恒等必须把值原样带回"},
		{math.MinInt64, 2, 2, math.MinInt64,
			"负边界同样不能被中间回绕改变"},
	}
	for _, tc := range cases {
		if got := mulDiv(tc.a, tc.b, tc.c); got != tc.want {
			t.Errorf("mulDiv(%d,%d,%d) = %d，期望 %d（%s）",
				tc.a, tc.b, tc.c, got, tc.want, tc.why)
		}
	}
}
