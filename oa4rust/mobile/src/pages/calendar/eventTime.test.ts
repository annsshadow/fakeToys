// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { validateEventTimeRange } from './eventTime'

describe('validateEventTimeRange', () => {
  it('正常起止顺序返回 null', () => {
    expect(validateEventTimeRange('2026-09-22 09:00', '2026-09-22 10:00')).toBeNull()
  })

  it('结束早于开始返回 end-before-start', () => {
    expect(validateEventTimeRange('2026-09-22 10:00', '2026-09-22 09:00')).toBe('end-before-start')
  })

  it('相等时间（零时长）允许', () => {
    expect(validateEventTimeRange('2026-09-22 09:00', '2026-09-22 09:00')).toBeNull()
  })

  it('跨天正常顺序允许', () => {
    expect(validateEventTimeRange('2026-09-22 23:00', '2026-09-23 01:00')).toBeNull()
  })

  it('不可解析的时间返回 invalid-format', () => {
    expect(validateEventTimeRange('not-a-date', '2026-09-22 10:00')).toBe('invalid-format')
    expect(validateEventTimeRange('2026-09-22 09:00', '')).toBe('invalid-format')
  })

  it('不存在的日期与越界时分秒返回 invalid-format', () => {
    expect(validateEventTimeRange('2026-02-30 09:00', '2026-02-30 10:00')).toBe('invalid-format')
    expect(validateEventTimeRange('2026-09-22 25:00', '2026-09-22 26:00')).toBe('invalid-format')
  })
})

describe('日历新建事件接入时间校验（R245）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'calendar.vue'), 'utf8')
  // 只截取必填校验之后、调用创建接口之前：确保校验发生在提交前而非事后。
  const submitBody = source.slice(
    source.indexOf('async function submitEvent'),
    source.indexOf('calendarApi.coreEventCreate'),
  )

  it('提交前调用 validateEventTimeRange，命中错误即短路返回', () => {
    expect(source).toContain("import { validateEventTimeRange } from './eventTime'")
    expect(submitBody).toContain('validateEventTimeRange(f.startTime, f.endTime)')
    expect(submitBody).toContain('if (timeRangeError)')
    // 校验调用之后、创建接口之前必须存在 return 短路。
    const callAt = submitBody.indexOf('validateEventTimeRange(f.startTime, f.endTime)')
    const guardAt = submitBody.indexOf('if (timeRangeError)', callAt)
    expect(guardAt).toBeGreaterThan(callAt)
    expect(submitBody.indexOf('return', guardAt)).toBeGreaterThan(guardAt)
  })

  it('倒序与格式错误分别有明确 toast 文案', () => {
    expect(submitBody).toContain('结束时间不能早于开始时间')
    expect(submitBody).toContain('时间格式不正确')
  })
})
