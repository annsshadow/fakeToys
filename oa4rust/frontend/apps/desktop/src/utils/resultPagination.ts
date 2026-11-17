// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

export interface ResultPageView<T> {
  /** 夹取后的实际页码，始终落在 [1, totalPages] */
  page: number
  /** 按过滤后行数计算的总页数，无结果时为 1 */
  totalPages: number
  /** 当前页应显示的行（已按 compare 排序、再分页） */
  rows: T[]
}

/**
 * 结果集「先过滤、再排序、最后分页」的统一视图计算。
 *
 * QueryStatementDesigner 的旧实现把「总页数」按未过滤的全量 resultData 算、
 * 「当前页行」却从过滤后的 sortedResult 里 slice，且筛选变化不归位页码：
 * 用户翻到第 2 页再筛选到少量行时，分页器仍显示「第 2 / 2 页」，表格却空白。
 * 本函数把三件事收敛到一次计算，保证页码、总页数与可见行始终互相一致。
 */
export function resultPageView<T>(
  allRows: readonly T[],
  matches: (row: T) => boolean,
  page: number,
  pageSize: number,
  compare?: (a: T, b: T) => number,
): ResultPageView<T> {
  const size = Number.isFinite(pageSize) && pageSize > 0 ? Math.floor(pageSize) : 1
  const filtered = allRows.filter(matches)
  const ordered = compare ? [...filtered].sort(compare) : filtered
  const totalPages = Math.max(1, Math.ceil(ordered.length / size))
  const p = Number.isFinite(page) ? Math.floor(page) : 1
  const clamped = Math.min(Math.max(p, 1), totalPages)
  return {
    page: clamped,
    totalPages,
    rows: ordered.slice((clamped - 1) * size, clamped * size),
  }
}
