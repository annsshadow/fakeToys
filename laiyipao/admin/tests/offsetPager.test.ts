// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 后台 offset 分页上界（R97）。
 *
 * 后端在 handler 里拒绝 offset > 25,000；前端必须在**发请求之前**停住，
 * 而不是让用户点一下看到 400。这里锁死两个函数在边界上的行为。
 */
import { describe, expect, it } from 'vitest'
import { MAX_OFFSET, canGoNext, cappedTotal } from '@/utils/offsetPager'

describe('cappedTotal', () => {
  it('真实总量远大于上界时，只排到「最后一页起点仍合法」为止', () => {
    // limit=50 → 封顶 25,050 → 501 页；第 501 页 offset=25,000 合法，
    // 不存在第 502 页（它的 offset=25,050 会被后端 400）。
    expect(cappedTotal(1_000_000, 50)).toBe(MAX_OFFSET + 50)
    expect(Math.ceil(cappedTotal(1_000_000, 50) / 50)).toBe(501)
  })

  it('总量本身没超上界时原样返回（不改动正常列表的页数）', () => {
    expect(cappedTotal(120, 50)).toBe(120)
    expect(cappedTotal(0, 50)).toBe(0)
  })

  it('异常值（NaN / 负数）不因为封顶而变形', () => {
    expect(cappedTotal(Number.NaN, 50)).toBeNaN()
    expect(cappedTotal(-1, 50)).toBe(-1)
  })
})

describe('canGoNext', () => {
  it('下一档恰好落在 25,000 时仍可翻（上界是包含的）', () => {
    expect(canGoNext(24_950, 50, 1_000_000)).toBe(true)
  })

  it('下一档会到 25,050（即 offset 25,001 之后）时禁用', () => {
    expect(canGoNext(25_000, 50, 1_000_000)).toBe(false)
  })

  it('本来就没有下一页时禁用（与既有语义一致）', () => {
    expect(canGoNext(100, 50, 120)).toBe(false)
    expect(canGoNext(50, 50, 120)).toBe(true)
  })
})
