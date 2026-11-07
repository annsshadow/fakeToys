// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { parseServerDate } from './datetime'

describe('服务端时间解析（R152，iOS 兼容）', () => {
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
