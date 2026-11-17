// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { businessTodayYM, businessTodayYMD } from './date'

describe('业务今日 / 本月（固定 UTC+8 业务时区）', () => {
  // 业务日界固定在 UTC+8（与 timerStart.beijingNow、O26-1 一致），
  // 不能直接用 new Date().toISOString().slice(0,10)——那是 UTC 日，UTC+8 凌晨会落到昨天/上月。
  it('UTC+8 凌晨：业务今日/本月不能回退到 UTC 昨天/上月', () => {
    // 2026-01-01 00:30 北京时间 == 2025-12-31 16:30 UTC
    const now = new Date('2025-12-31T16:30:00Z')

    // 旧实现的实际输出（证明缺口真实存在）
    expect(now.toISOString().slice(0, 10)).toBe('2025-12-31')
    expect(now.toISOString().slice(0, 7)).toBe('2025-12')

    // 业务口径应为新的一天 / 新的一月
    expect(businessTodayYMD(now)).toBe('2026-01-01')
    expect(businessTodayYM(now)).toBe('2026-01')
  })

  it('与浏览器本地时区无关：同一时刻只取决于固定 UTC+8 业务时区', () => {
    const now = new Date('2025-12-31T16:30:00Z')
    expect(businessTodayYMD(now)).toBe('2026-01-01')
    expect(businessTodayYM(now)).toBe('2026-01')
  })

  it('UTC+8 白天与 UTC 日期一致（无副作用）', () => {
    // 北京时间 2026-06-15 12:00 == 04:00 UTC
    const now = new Date('2026-06-15T04:00:00Z')
    expect(businessTodayYMD(now)).toBe('2026-06-15')
    expect(businessTodayYM(now)).toBe('2026-06')
  })

  it('默认参数取当前时刻，且 YM 与 YMD 前缀一致', () => {
    const ymd = businessTodayYMD()
    expect(ymd).toMatch(/^\d{4}-\d{2}-\d{2}$/)
    expect(businessTodayYM()).toBe(ymd.slice(0, 7))
  })
})
