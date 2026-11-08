// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const SRC = resolve(import.meta.dirname, '..')

function walk(dir: string): string[] {
  const out: string[] = []
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name)
    if (entry.isDirectory()) out.push(...walk(full))
    else if (/\.(ts|vue)$/.test(entry.name)) out.push(full)
  }
  return out
}

const sources = walk(SRC).filter((file) => !/\.test\.ts$/.test(file) && !/utils[/\\]datetime\.ts$/.test(file))

describe('移动端服务端时间解析契约（R154）', () => {
  it('全仓不再用 new Date 隐式解析服务端字符串', () => {
    const offenders: string[] = []
    for (const file of sources) {
      const source = readFileSync(file, 'utf8')
      if (/new Date\(\s*String\(/.test(source)) offenders.push(`${file}: new Date(String(...))`)
      if (/new Date\(\s*['"`]/.test(source)) offenders.push(`${file}: new Date(字符串字面量)`)
      if (/Date\.parse\(/.test(source)) offenders.push(`${file}: Date.parse(...)`)
    }
    expect(offenders).toEqual([])
  })

  it('日历/考勤服务端时间继续走 parseServerDate，不回退隐式解析', () => {
    for (const name of ['pages/calendar/calendar.vue', 'pages/attendance/attendance.vue']) {
      const source = readFileSync(resolve(SRC, name), 'utf8')
      expect(source).toContain("import { parseServerDate } from '@/utils/datetime'")
      expect(source.split('parseServerDate(').length - 1).toBeGreaterThan(0)
      expect(source).not.toMatch(/new Date\(\s*(?:raw|value|v)\b/)
      expect(source).not.toContain('new Date(String(')
    }
  })
})
