// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import "fmt"

// 管理端 CSV 导出的单次行数上限。
//
// 上限本身是需要的：导出把全量结果物化进内存，无上限时一次请求就能把
// API 进程打爆。但**达到上限不能静默截断**——运营拿到一份缺行的 CSV，
// 会当成全量证据做对账/补偿。此前 10 处导出 SQL 都写死 `LIMIT 100000`，
// 超过 10 万行时返回的是「成功 + 少了一截」。
//
// 现在的约定：SQL 用 `LIMIT adminExportProbeRows`（多取一行），迭代时
// 数行数；读到超过上限的行就返回 ErrExportTooLarge，让调用方拿到明确的
// 「结果太大，请缩小筛选」，而不是一份被截断的成功响应。
const (
	adminExportMaxRows   = 100000
	adminExportProbeRows = adminExportMaxRows + 1
)

// exportTooLarge 返回「导出结果超过上限」的可分类错误。
//
// n 是本次实际读到的行数；因为 SQL 只多取一行，n 的真实含义是
// 「至少 n 行」，文案里要按「至少」表述，避免给出错误的总数。
func exportTooLarge(n int) error {
	return fmt.Errorf("%w：本次至少命中 %d 行，单次最多导出 %d 行；请缩小筛选范围或分批导出",
		ErrExportTooLarge, n, adminExportMaxRows)
}
