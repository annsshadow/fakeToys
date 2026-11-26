// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R302 t29 DR-05: 路由切换播报必须 sr-only（读屏可播、视觉隐藏）。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const announcer = readFileSync(resolve(__dirname, './routeAnnouncer.ts'), 'utf8')

describe('R302 t29 路由切换 sr-only 播报', () => {
  it('live region 为 status/polite/atomic 且样式为 sr-only', () => {
    expect(announcer).toMatch(/setAttribute\('role',\s*'status'\)/)
    expect(announcer).toMatch(/setAttribute\('aria-live',\s*'polite'\)/)
    expect(announcer).toMatch(/setAttribute\('aria-atomic',\s*'true'\)/)
    expect(announcer).toMatch(/const SR_ONLY_STYLE\s*=[\s\S]*?position:absolute;width:1px;height:1px[\s\S]*?clip:rect\(0,0,0,0\)/)
    expect(announcer).toMatch(/setAttribute\('style',\s*SR_ONLY_STYLE\)/)
  })
})
