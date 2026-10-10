// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { monthEndYMD } from './date'

describe('monthEndYMD（月末 YMD，UTC 无时区依赖）', () => {
  it('普通月取当月最后一天', () => {
    expect(monthEndYMD('2026-10')).toBe('2026-10-31')
    expect(monthEndYMD('2026-03')).toBe('2026-03-31')
    expect(monthEndYMD('2026-06')).toBe('2026-06-30')
  })

  it('闰年 2 月 29 天、平年 2 月 28 天', () => {
    expect(monthEndYMD('2028-02')).toBe('2028-02-29')
    expect(monthEndYMD('2026-02')).toBe('2026-02-28')
    expect(monthEndYMD('2000-02')).toBe('2000-02-29') // 世纪闰年
    expect(monthEndYMD('1900-02')).toBe('1900-02-28') // 非 400 倍数非闰
  })

  it('输出恒为零填充的 YYYY-MM-DD（输入月未补零也归一）', () => {
    expect(monthEndYMD('2026-9')).toBe('2026-09-30')
    expect(monthEndYMD('2026-12')).toBe('2026-12-31')
  })
})
