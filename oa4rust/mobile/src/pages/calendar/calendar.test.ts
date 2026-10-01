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
