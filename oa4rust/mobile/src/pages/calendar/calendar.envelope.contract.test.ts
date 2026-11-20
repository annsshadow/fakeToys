// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * P0 回归契约（t25 / C1 全页实测 P0-2）：
 * 真接口 GET /calendar/list/my 返回的 data 是对象
 * { myCalendars: [], unitCalendars: [] }，旧代码 (cals.data ?? []) as CalendarRow[]
 * 只是类型断言，运行期对象没有 .map，抛 "S.value.map is not a function"，日历整页不可用。
 * 这里对归一化函数做行为断言，而非源码字符串匹配：给定对象形 data 必须返回合并后的数组。
 */
import { describe, expect, it } from 'vitest'
import { normalizeCalendarRows } from './calendarEnvelope'

describe('日历 data 信封运行时归一化（对象形 data 不再 .map 崩溃）', () => {
  it('RED核心：对象形 {myCalendars, unitCalendars} 合并为扁平数组，可直接 .map 渲染', () => {
    const mine = [{ id: 'mine-1' }, { id: 'mine-2' }]
    const unit = [{ id: 'unit-1' }]
    const result = normalizeCalendarRows({ myCalendars: mine, unitCalendars: unit })
    expect(Array.isArray(result)).toBe(true)
    expect(result.map((row) => row.id)).toEqual(['mine-1', 'mine-2', 'unit-1'])
  })

  it('数组形 data 原样透传（兼容旧后端/单测桩）', () => {
    const arr = [{ id: 'a' }, { id: 'b' }]
    expect(normalizeCalendarRows(arr)).toEqual(arr)
  })

  it('空值或畸形信封退化为 []，不再抛 .map is not a function', () => {
    expect(normalizeCalendarRows(null)).toEqual([])
    expect(normalizeCalendarRows(undefined)).toEqual([])
    expect(normalizeCalendarRows({})).toEqual([])
    expect(normalizeCalendarRows({ myCalendars: 'not-an-array' })).toEqual([])
  })
})
