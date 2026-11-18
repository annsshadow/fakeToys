// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R205 导航高亮边界契约：侧边栏同一时刻只能有一个导航项处于 active。
 *
 * 缺陷证据：AppShell.vue 的 isActive() 用 route.path.startsWith(navPath)。路由表同时
 * 注册了 /app/file（文件管理）与 /app/fileinfo（文件信息），访问 /app/fileinfo 时
 * '/app/fileinfo'.startsWith('/app/file') === true，两个导航项同时亮起，用户分不清
 * 当前所在模块。
 *
 * 契约：导航项路径是路由的「段」边界，只有当前路径等于导航路径或以 navPath + '/'
 * 开头时才算命中；既不能错配兄弟路由（file/fileinfo），也不能漏掉合法子路由。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { isNavActive } from './navActive'

describe('侧边栏导航高亮段边界契约', () => {
  it('访问 /app/fileinfo 时，/app/file 不得高亮（前缀相同但属于不同路由段）', () => {
    expect(isNavActive('/app/fileinfo', '/app/file')).toBe(false)
  })

  it('访问 /app/fileinfo 时，/app/fileinfo 自己必须高亮', () => {
    expect(isNavActive('/app/fileinfo', '/app/fileinfo')).toBe(true)
  })

  it('精确命中的导航项仍然高亮', () => {
    expect(isNavActive('/app/file', '/app/file')).toBe(true)
    expect(isNavActive('/app/dashboard', '/app/dashboard')).toBe(true)
  })

  it('真实子路由（navPath + /...）仍然高亮', () => {
    expect(isNavActive('/app/process/detail/42', '/app/process')).toBe(true)
    expect(isNavActive('/app/process/', '/app/process')).toBe(true)
  })

  it('兄弟路由即使共享前缀也不得高亮', () => {
    expect(isNavActive('/app/formdesigner', '/app/form')).toBe(false)
    expect(isNavActive('/app/viewer', '/app/view')).toBe(false)
  })

  it('AppShell 必须委托给段边界函数，不得回退到裸 startsWith', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')
    expect(source).toMatch(/isNavActive\(route\.path, path\)/)
    expect(source).not.toMatch(/return route\.path\.startsWith\(path\)/)
  })

  it('同一时刻 active 的导航项至多一个（真实导航路径全量互不误匹配）', () => {
    const navPaths = [
      '/app/dashboard',
      '/app/org',
      '/app/directory',
      '/app/process',
      '/app/im',
      '/app/calendar',
      '/app/meeting',
      '/app/file',
      '/app/bbs',
      '/app/admin',
      '/app/mind',
      '/app/document',
      '/app/program',
      '/app/queryview',
      '/app/recycle',
      '/app/server',
      '/app/unit',
      '/app/form',
      '/app/view',
      '/app/fileinfo',
    ]
    for (const current of navPaths) {
      const active = navPaths.filter((p) => isNavActive(current, p))
      expect(active, `路径 ${current} 命中了多个导航项: ${active.join(', ')}`).toEqual([current])
    }
  })
})
