// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

export interface CalendarDateParts {
  year: number
  month: number
  day: number
}

export interface CalendarMonth {
  year: number
  month: number
}

// 固定按 UTC+8 业务日读取“今天”，与 date.ts businessTodayYMD 口径一致，
// 不随浏览器本地时区漂移；每次调用都读当前时刻，跨午夜自动生效。
export function calendarBusinessToday(now: Date = new Date()): CalendarDateParts {
  const shifted = new Date(now.getTime() + 8 * 60 * 60 * 1000)
  return {
    year: shifted.getUTCFullYear(),
    month: shifted.getUTCMonth() + 1,
    day: shifted.getUTCDate(),
  }
}

// 网格单元格键恒定补零，保证跨月格与当月格格式一致、Vue :key 稳定。
export function calendarCellKey(year: number, month: number, day: number): string {
  return `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`
}

// 换月统一走 1..12，跨年时进位/借位年份，避免出现 0 月或 13 月。
export function shiftCalendarMonth(year: number, month: number, delta: -1 | 1): CalendarMonth {
  const absolute = year * 12 + (month - 1) + delta
  return { year: Math.floor(absolute / 12), month: (((absolute % 12) + 12) % 12) + 1 }
}
