// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { beijingNow, beijingWallClockToUtcNaive, buildTimerStartPayload } from './timerStart'

// O27 守卫：定时器按钮此前提交空 body（{}）→ 后端 workId/fireAt 缺失，功能恒不可用。
// 这里锁定三件事：①本地北京时间 → 后端 UTC naive 契约的换算；②非法输入不发出半成品请求；
// ③请求体字段名与后端 `start_timer` 读取的键（workId / fireAt）一致。
// 破坏方式：把 `- BUSINESS_OFFSET_MS` 改成 0 或 `+` → 下面所有有效用例红。

interface ConversionCase {
  input: string
  expected: string
  why: string
}

const VALID_CASES: ConversionCase[] = [
  {
    input: '2026-11-05 12:00:00',
    expected: '2026-11-05 04:00:00',
    why: '北京正午 = UTC 04:00（-8h 基本盘）',
  },
  {
    input: '2026-11-05 07:30:00',
    expected: '2026-11-04 23:30:00',
    why: '北京清晨 = UTC 前一天（跨日界）',
  },
  {
    input: '2026-01-01 00:00:00',
    expected: '2025-12-31 16:00:00',
    why: '北京跨年零点 = UTC 上一年（跨年界）',
  },
  {
    input: '2024-02-29 23:59:59',
    expected: '2024-02-29 15:59:59',
    why: '闰日边界不得被日期校验误拒',
  },
]

const INVALID_CASES: Array<{ input: string; why: string }> = [
  { input: '2026-02-30 10:00:00', why: '不存在的日期（Date.UTC 会顺延到 03-02，必须拒绝）' },
  { input: '2026-11-05 24:00:00', why: '小时越界' },
  { input: '2026-11-5 12:00:00', why: '月份/日未补零，不符合后端格式' },
  { input: '', why: '空输入' },
]

describe('O27 定时器按钮：北京时间 → 后端 UTC 契约', () => {
  it('用例表规模下限（防止守卫被悄悄删到只剩 1 条）', () => {
    expect(VALID_CASES.length).toBeGreaterThanOrEqual(4)
    expect(INVALID_CASES.length).toBeGreaterThanOrEqual(4)
  })

  it.each(VALID_CASES)('$input → $expected（$why）', ({ input, expected }) => {
    expect(beijingWallClockToUtcNaive(input)).toBe(expected)
  })

  it.each(INVALID_CASES)('$input 必须拒绝（$why）', ({ input }) => {
    expect(beijingWallClockToUtcNaive(input)).toBeNull()
  })

  it('请求体只含后端读取的两个键，且 workId 去空白', () => {
    const built = buildTimerStartPayload('  work-1  ', '2026-11-05 12:00:00')
    expect(built.ok).toBe(true)
    if (!built.ok) return
    expect(Object.keys(built.payload).sort()).toEqual(['fireAt', 'workId'])
    expect(built.payload).toEqual({
      workId: 'work-1',
      fireAt: '2026-11-05 04:00:00',
    })
  })

  it('workId 为空或时间非法时返回错误，不产生半成品载荷', () => {
    const noWorkId = buildTimerStartPayload('   ', '2026-11-05 12:00:00')
    expect(noWorkId.ok).toBe(false)
    if (!noWorkId.ok) expect(noWorkId.error).toContain('流程实例 ID')

    const badTime = buildTimerStartPayload('work-1', '2026-02-30 10:00:00')
    expect(badTime.ok).toBe(false)
    if (!badTime.ok) expect(badTime.error).toContain('YYYY-MM-DD HH:MM:SS')
  })

  it('beijingNow 不依赖浏览器时区：固定 UTC 时刻 → 固定的北京墙钟', () => {
    expect(beijingNow(new Date('2026-11-05T00:00:00Z'))).toBe('2026-11-05 08:00:00')
  })
})
