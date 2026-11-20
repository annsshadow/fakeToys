// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * R296 导航完整性契约：侧栏每个导航项必须指向已注册路由。
 * 缺陷证据：侧栏「管理」指向 /app/admin，但 main.ts 未注册该子路由，
 * 命中 /:pathMatch(.*)* -> /login 兜底，用户点击「管理」被误登出。
 */
const shell = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')
const mainTs = readFileSync(resolve(import.meta.dirname, '../../../../apps/desktop/src/main.ts'), 'utf8')

const navPaths = [...shell.matchAll(/path: '(\/app\/[^']+)'/g)].map((m) => m[1])

describe('AppShell 导航完整性（R296）', () => {
  it('提取到全部侧栏导航项', () => {
    expect(navPaths.length).toBeGreaterThanOrEqual(15)
  })

  it('每个导航项都指向 main.ts 已注册的子路由', () => {
    const missing = navPaths.filter((p) => {
      const segment = p.replace('/app/', '')
      return !new RegExp("path: '" + segment + "'").test(mainTs)
    })
    expect(missing, '无对应路由的导航项: ' + missing.join(', ')).toEqual([])
  })

  it('管理入口不再指向不存在的 /app/admin', () => {
    expect(shell).not.toMatch(/path: '\/app\/admin'/)
    expect(shell).toMatch(/label: '管理'/)
  })
})
