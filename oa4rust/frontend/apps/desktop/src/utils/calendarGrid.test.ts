// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { calendarBusinessToday, calendarCellKey, shiftCalendarMonth } from './calendarGrid'
import { businessTodayYMD } from './date'

describe('日历今天（R169：固定 UTC+8 业务日，不随浏览器时区漂移）', () => {
  it('UTC+8 跨日：北京时间已是次日，日历不能停留在 UTC 的昨天', () => {
    // 北京时间 2026-01-01 00:30 == 2025-12-31 16:30Z
    const now = new Date('2025-12-31T16:30:00Z')
    expect(businessTodayYMD(now)).toBe('2026-01-01')
    expect(calendarBusinessToday(now)).toEqual({ year: 2026, month: 1, day: 1 })
  })

  it('UTC+8 跨月：业务月已切换为 2 月', () => {
    const now = new Date('2026-01-31T16:30:00Z')
    expect(calendarBusinessToday(now)).toEqual({ year: 2026, month: 2, day: 1 })
  })

  it('与 date.ts 业务日口径逐点一致', () => {
    for (const iso of [
      '2026-06-15T04:00:00Z',
      '2025-12-31T15:59:59Z',
      '2025-12-31T16:00:00Z',
      '2031-12-31T16:00:00Z',
    ]) {
      const now = new Date(iso)
      const p = calendarBusinessToday(now)
      expect(`${p.year}-${String(p.month).padStart(2, '0')}-${String(p.day).padStart(2, '0')}`).toBe(
        businessTodayYMD(now),
      )
    }
  })
})

describe('日历网格键与跨年换月（R169）', () => {
  it('单元格键恒定补零，跨月格与当月格格式一致', () => {
    expect(calendarCellKey(2031, 1, 5)).toBe('2031-01-05')
    expect(calendarCellKey(2031, 12, 31)).toBe('2031-12-31')
  })

  it('一月上一月回到去年 12 月，十二月下一月进入次年 1 月', () => {
    expect(shiftCalendarMonth(2031, 1, -1)).toEqual({ year: 2030, month: 12 })
    expect(shiftCalendarMonth(2031, 12, 1)).toEqual({ year: 2032, month: 1 })
    expect(shiftCalendarMonth(2031, 6, -1)).toEqual({ year: 2031, month: 5 })
    expect(shiftCalendarMonth(2031, 6, 1)).toEqual({ year: 2031, month: 7 })
  })

  it('跨年相邻格用真实年月生成键，不出现 0 月/13 月', () => {
    const prev = shiftCalendarMonth(2031, 1, -1)
    const next = shiftCalendarMonth(2031, 12, 1)
    expect(calendarCellKey(prev.year, prev.month, 31)).toBe('2030-12-31')
    expect(calendarCellKey(next.year, next.month, 1)).toBe('2032-01-01')
  })
})
