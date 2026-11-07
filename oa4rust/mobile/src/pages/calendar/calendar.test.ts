// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

// mobile vitest 环境未装配 .vue 插件，按仓内源码断言惯例守卫。
const source = readFileSync(resolve(import.meta.dirname, 'calendar.vue'), 'utf8')

describe('日历页事件并行拉取', () => {
  it('各日历事件用 allSettled 并行拉取，不再逐日历串行 await', () => {
    // 多日历用户原为 N 次串行往返（每次一个 RTT）；allSettled 并行后单日历失败仍不影响其余。
    expect(source).toContain('Promise.allSettled')
    expect(source).not.toContain('await calendarApi.eventsFilter')
  })
})

describe('日历时间显示（R152，iOS 兼容）', () => {
  it('经 parseServerDate 解析，不再用 new Date 直接吞服务端字符串', () => {
    // iOS WebView 里 new Date('2026-11-07 10:30:00') 是 Invalid Date，时间会退化成原始字符串。
    expect(source).toContain("import { parseServerDate } from '@/utils/datetime'")
    expect(source).toContain('const d = parseServerDate(v)')
    expect(source).not.toContain('const d = new Date(String(v))')
  })
})
