// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"bytes"
	"fmt"
)

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

// exportBuffer 是带**字节预算**的 CSV 输出缓冲（R14）。
//
// # 为什么行数上限不够
//
// `adminExportMaxRows` 挡住了「行太多」，但挡不住「行太宽」：
// 100k 行 × 每行 1KB 就是 100MB，而导出是先把整份 CSV 物化进内存再写响应。
// 几次并发的大字段导出足以把进程推到 OOM，而它**没有超过任何行数上限**。
//
// # 为什么在 Write 层拦而不是导出完再查
//
// 事后检查 `buf.Len() > limit` 时内存已经分配掉了 —— 恰恰是这一轮要防的东西。
// 在写入前比较预算，超了就立刻返回可分类错误。
//
// 注意 `csv.Writer` 内部还有一层 `bufio.Writer`（默认 4 KiB）：它的单次
// 底层 Write 最多 4 KiB，所以实际峰值内存是 `limit` 再外加至多一个刷新块。
// 超预算的那个块会被**整块丢弃**，不会写进缓冲；错误随后由 csv.Writer 原样
// 返回，各导出末尾的 `Flush()/Error()` 检查会把它交回调用方。
//
// 实现成 io.Writer 而不是给 10 个导出各加计数：csv.Writer 会把底层 Write 的
// 错误原样返回，于是所有走 `writeCSVRow` 的导出**自动**获得预算约束 ——
// 新增导出只要用 `s.newExportBuffer()` 就不会漏。
type exportBuffer struct {
	bytes.Buffer
	limit int
	err   error
}

// newExportBuffer 按配置构造；limit <= 0 表示不限（与其它配置的 0 语义一致）。
func newExportBuffer(limit int) *exportBuffer {
	return &exportBuffer{limit: limit}
}

// Service.newExportBuffer 取一次配置快照后构造，避免预算在单次导出内被热加载换掉。
func (s *Service) newExportBuffer() *exportBuffer {
	return newExportBuffer(s.Cfg().ExportMaxBytes)
}

func (b *exportBuffer) Write(p []byte) (int, error) {
	if b.err != nil {
		return 0, b.err
	}
	// 用 Buffer.Len() 而不是自己计数：BOM 与表头走的是被提升的 WriteString，
	// 同样占用内存，必须计入预算。
	if b.limit > 0 && b.Buffer.Len()+len(p) > b.limit {
		b.err = fmt.Errorf("%w：本次输出将达 %d 字节，超过单次上限 %d 字节；请缩小筛选范围或分批导出",
			ErrExportTooLarge, b.Buffer.Len()+len(p), b.limit)
		return 0, b.err
	}
	return b.Buffer.Write(p)
}
