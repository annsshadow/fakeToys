// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

// mobile vitest 环境未装配 .vue 插件，按仓内源码断言惯例守卫。
const source = readFileSync(resolve(import.meta.dirname, 'attendance.vue'), 'utf8')

describe('考勤时间显示（R152，iOS 兼容）', () => {
  it('经 parseServerDate 解析，不再用 new Date 直接吞服务端字符串', () => {
    // iOS WebView 里 new Date('2026-11-07 10:30:00') 是 Invalid Date，打卡时间会原样显示服务端字符串。
    expect(source).toContain("import { parseServerDate } from '@/utils/datetime'")
    expect(source).toContain('const d = parseServerDate(value)')
    expect(source).not.toContain('new Date(value)')
  })
})
