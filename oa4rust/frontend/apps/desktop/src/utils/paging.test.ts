// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { clampPage } from './paging'

describe('clampPage：结果集缩小后页码必须夹取（R170）', () => {
  it('页码在合法范围内时保持不变', () => {
    expect(clampPage(3, 5)).toBe(3)
    expect(clampPage(1, 5)).toBe(1)
    expect(clampPage(2, 2)).toBe(2)
  })

  it('页码超过总页数时夹到最后一页（切到记录更少的月份/删到只剩一页）', () => {
    expect(clampPage(5, 2)).toBe(2)
    expect(clampPage(3, 1)).toBe(1)
  })

  it('结果为空（总页数 0）时回到第 1 页，绝不停留在空页', () => {
    expect(clampPage(4, 0)).toBe(1)
    expect(clampPage(2, -1)).toBe(1)
  })

  it('非法/非整数页码归一到合法值', () => {
    expect(clampPage(0, 3)).toBe(1)
    expect(clampPage(-2, 3)).toBe(1)
    expect(clampPage(Number.NaN, 3)).toBe(1)
    expect(clampPage(Number.POSITIVE_INFINITY, 3)).toBe(1)
    expect(clampPage(2.7, 5)).toBe(2)
  })

  it('夹取幂等：重复夹取不再变化（避免 queryFn 内改页码触发无限重取）', () => {
    for (const [p, t] of [
      [9, 1],
      [7, 2],
      [1, 0],
      [3, 8],
      [2.9, 2.1],
    ]) {
      const once = clampPage(p, t)
      expect(clampPage(once, t)).toBe(once)
    }
  })
})
