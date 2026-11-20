// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { afterAll, beforeEach, describe, expect, it, vi } from 'vitest'

const OrigDateTimeFormat = Intl.DateTimeFormat
let ctorCount = 0

class CountingDateTimeFormat extends OrigDateTimeFormat {
  constructor(...args: ConstructorParameters<typeof OrigDateTimeFormat>) {
    super(...args)
    ctorCount++
  }
}

vi.stubGlobal('Intl', { ...Intl, DateTimeFormat: CountingDateTimeFormat })

let formatServerDateTime: typeof import('./date').formatServerDateTime

beforeEach(async () => {
  // 每个用例重新导入以拿全新 formatter 单例，避免跨用例共享干扰构造计数。
  vi.resetModules()
  ctorCount = 0
  formatServerDateTime = (await import('./date')).formatServerDateTime
})

afterAll(() => {
  vi.unstubAllGlobals()
})

describe('formatServerDateTime 格式化器复用契约（性能防回归）', () => {
  it('1000 行默认格式时间只构造 1 个 Intl.DateTimeFormat', () => {
    for (let i = 0; i < 1000; i++) {
      formatServerDateTime(`2026-11-07 10:${String(i % 60).padStart(2, '0')}:00`)
    }
    expect(ctorCount).toBe(1)
  })

  it('复用 formatter 的输出与 toLocaleString 默认选项完全一致', () => {
    const defaultOptions: Intl.DateTimeFormatOptions = {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    }
    const dates = [new Date(2026, 10, 7, 10, 30, 0), new Date(2026, 0, 1, 0, 5, 9), new Date(2026, 11, 31, 23, 59, 59)]
    for (const d of dates) {
      expect(formatServerDateTime(d)).toBe(d.toLocaleString('zh-CN', defaultOptions))
    }
  })

  it('显式 options 保留 toLocaleString 语义（{} 带时间），不新建缓存 formatter', () => {
    const d = new Date(2026, 10, 7, 10, 30, 5)
    formatServerDateTime(d)
    expect(ctorCount).toBe(1)
    expect(formatServerDateTime(d, {})).toBe(d.toLocaleString('zh-CN', {}))
    expect(formatServerDateTime(d, { year: 'numeric', month: '2-digit' })).toBe(
      d.toLocaleString('zh-CN', { year: 'numeric', month: '2-digit' }),
    )
    expect(ctorCount).toBe(1)
  })
})
