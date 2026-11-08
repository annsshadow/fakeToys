// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { formatServerDateTime, monthEndYMD, parseServerDate } from './date'

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

describe('服务端时间解析（R154，iOS 兼容）', () => {
  it('空格分隔的「日期 时间」按本地时间解析（iOS 上不再 Invalid）', () => {
    const d = parseServerDate('2026-11-07 10:30:00')
    expect(d).not.toBeNull()
    expect(d?.getFullYear()).toBe(2026)
    expect(d?.getMonth()).toBe(10)
    expect(d?.getDate()).toBe(7)
    expect(d?.getHours()).toBe(10)
    expect(d?.getMinutes()).toBe(30)
    expect(d?.getSeconds()).toBe(0)
  })

  it('支持 T 分隔、秒可省略、斜杠日期与纯日期', () => {
    expect(parseServerDate('2026-11-07T09:05')?.getMinutes()).toBe(5)
    expect(parseServerDate('2026/01/02 08:00')?.getMonth()).toBe(0)
    const day = parseServerDate('2026-11-07')
    expect(day?.getHours()).toBe(0)
    expect(day?.getMinutes()).toBe(0)
  })

  it('带时区标记的字符串交给标准解析', () => {
    const d = parseServerDate('2026-11-07T02:30:00Z')
    expect(d?.getTime()).toBe(Date.UTC(2026, 10, 7, 2, 30, 0))
  })

  it('非法/空/不存在的日期返回 null', () => {
    expect(parseServerDate('')).toBeNull()
    expect(parseServerDate('   ')).toBeNull()
    expect(parseServerDate('not-a-date')).toBeNull()
    expect(parseServerDate('2026-02-30 10:00')).toBeNull()
    expect(parseServerDate(null)).toBeNull()
    expect(parseServerDate(undefined)).toBeNull()
    expect(parseServerDate(Number.NaN)).toBeNull()
    expect(parseServerDate(Number.POSITIVE_INFINITY)).toBeNull()
  })

  it('数字时间戳与 Date 实例直接可用', () => {
    const ms = Date.UTC(2026, 10, 7, 2, 30, 0)
    expect(parseServerDate(ms)?.getTime()).toBe(ms)
    expect(parseServerDate(new Date(ms))?.getTime()).toBe(ms)
    expect(parseServerDate(new Date(Number.NaN))).toBeNull()
  })
})

describe('formatServerDateTime（R154，解析失败回落原始值）', () => {
  it('服务端空格时间不再输出 Invalid Date', () => {
    const text = formatServerDateTime('2026-11-07 10:30:00')
    expect(text).not.toContain('Invalid')
    expect(text).toMatch(/11\/07|11-07/)
  })

  it('无法解析时回落原始字符串', () => {
    expect(formatServerDateTime('not-a-date')).toBe('not-a-date')
  })

  it('默认格式为两位月日与时分', () => {
    const text = formatServerDateTime('2026-11-07 10:30:00')
    expect(text).toMatch(/11\/07|11-07/)
    expect(text).toMatch(/10:30/)
  })
})
