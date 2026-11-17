// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { formatPercent } from './formatNumber'

describe('formatPercent：百分比统一格式（R164）', () => {
  it('把 0..1 比例转成带 % 的字符串，小数位固定', () => {
    expect(formatPercent(0)).toBe('0.0%')
    expect(formatPercent(0.5)).toBe('50.0%')
    expect(formatPercent(0.1234, 2)).toBe('12.34%')
    expect(formatPercent(1, 0)).toBe('100%')
    expect(formatPercent(0.6666, 0)).toBe('67%')
  })

  it('接受数值字符串，与后端数字字段保持一致', () => {
    expect(formatPercent('0.25', 0)).toBe('25%')
    expect(formatPercent('1', 0)).toBe('100%')
  })

  it('无数据/无效比例统一占位符，不再出现 NaN%/undefined%/误导性 100%', () => {
    for (const bad of [
      null,
      undefined,
      '',
      'abc',
      Number.NaN,
      Number.POSITIVE_INFINITY,
      Number.NEGATIVE_INFINITY,
      -0.1,
      -1,
    ]) {
      expect(formatPercent(bad)).toBe('—')
    }
    // 方向分布等场景总数 0 时 count/total 产生 NaN，必须占位而非 'NaN%'
    expect(formatPercent(0 / 0)).toBe('—')
  })
})
