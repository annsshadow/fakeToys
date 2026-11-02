// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// clampLimit 统一分页条数语义：非正数 → 缺省值；超过上限 → **夹到上限**。
//
// 不要写回 `if limit <= 0 || limit > maxLimit { limit = defLimit }`：
// 那样 `?limit=9999` 与 `?limit=0` 得到同一个结果，客户端「要更多」
// 反而拿到**比上限更少**的行，且不报错。公告列表早已按夹取口径实现
// （announcement.go 的 switch），这里把其余列表端点收口到同一语义。
func clampLimit(limit, defLimit, maxLimit int) int {
	switch {
	case limit <= 0:
		return defLimit
	case limit > maxLimit:
		return maxLimit
	default:
		return limit
	}
}
