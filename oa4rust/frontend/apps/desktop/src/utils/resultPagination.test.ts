// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { resultPageView } from './resultPagination'

const rows = Array.from({ length: 100 }, (_, i) => ({ id: i, name: `row-${i}` }))
const byId42 = (r: { id: number }) => r.id === 42

describe('结果筛选与分页：页数必须按过滤后行数计算', () => {
  it('无筛选：100 行每页 50 → 共 2 页，第 2 页为后 50 行', () => {
    const v = resultPageView(rows, () => true, 2, 50)
    expect(v.totalPages).toBe(2)
    expect(v.page).toBe(2)
    expect(v.rows.map((r) => r.id)).toEqual(Array.from({ length: 50 }, (_, i) => i + 50))
  })

  it('翻到第 2 页后筛选只剩 1 行：页数变 1、页码夹到 1，能看到该行而非空白页', () => {
    const v = resultPageView(rows, byId42, 2, 50)
    expect(v.totalPages).toBe(1)
    expect(v.page).toBe(1)
    expect(v.rows.map((r) => r.id)).toEqual([42])
  })

  it('筛选结果为空：仍在第 1 页 / 共 1 页，不出现「第 2 / 0 页」', () => {
    const v = resultPageView(rows, () => false, 2, 50)
    expect(v.totalPages).toBe(1)
    expect(v.page).toBe(1)
    expect(v.rows).toEqual([])
  })

  it('筛选后 60 行：共 2 页，第 3 页被夹到第 2 页且只显示剩余 10 行', () => {
    const v = resultPageView(rows, (r) => r.id < 60, 3, 50)
    expect(v.totalPages).toBe(2)
    expect(v.page).toBe(2)
    expect(v.rows.map((r) => r.id)).toEqual(Array.from({ length: 10 }, (_, i) => i + 50))
  })

  it('排序在分页之前生效：降序时第 1 页首行是最大值', () => {
    const v = resultPageView(
      rows,
      () => true,
      1,
      50,
      (a, b) => b.id - a.id,
    )
    expect(v.rows[0].id).toBe(99)
    expect(v.rows).toHaveLength(50)
  })

  it('非法每页条数归一到 1，避免切出空页', () => {
    const v = resultPageView(rows, () => true, 1, 0)
    expect(v.rows).toHaveLength(1)
  })
})
