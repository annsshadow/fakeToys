// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 管理区重构契约：分组侧栏数据与 main.ts 嵌套路由必须一一对应。
 *
 * 缺陷证据（用户 turn 27）：/app/admin/* 约 58 个管理页在 AppShell 侧栏堆叠/折叠混杂，
 * 「人根本没法用」。重构为 AdminLayout 六组分组侧栏 + 嵌套路由。
 *
 * 契约：
 * 1) adminNav.ts 恰好六组（组织与权限/内容管理/流程管理/查询中心/门户管理/系统运维）；
 * 2) 分组条目与 main.ts AdminLayout 嵌套 children 的 58 条路由双向全覆盖、无重复；
 * 3) 条目标签与路由 meta.title 一致（面包屑/侧栏/文档标题三处同源）；
 * 4) main.ts 嵌套结构：import AdminLayout + path 'admin' + '' redirect 控制面板；
 * 5) 58 条 R297 旧路径 redirect 别名与 admin 角色守卫逐字保留。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { adminNavGroups } from './adminNav'

const mainTs = readFileSync(resolve(import.meta.dirname, '../../main.ts'), 'utf8')

function adminChildrenBlock(): string {
  const start = mainTs.indexOf("path: 'admin',")
  expect(start, 'main.ts 必须有嵌套管理区父路由 path: admin').toBeGreaterThan(-1)
  const end = mainTs.indexOf('\n      ],', start)
  expect(end, '管理区 children 数组必须闭合').toBeGreaterThan(start)
  return mainTs.slice(start, end)
}

const EXPECTED_GROUPS: Record<string, string[]> = {
  组织与权限: ['role', 'three-member', 'appinfo'],
  内容管理: [
    'document',
    'category',
    'hotpic',
    'template',
    'collect',
    'form',
    'view',
    'fileinfo',
    'cms-column',
    'cms-column-manager',
    'cms-index',
    'cms-dict-designer',
    'cms-form-designer',
    'cms-view-designer',
    'cms-script-designer',
    'cms-xform',
    'cms-module',
  ],
  流程管理: [
    'process-designer',
    'form-designer',
    'process-task-center',
    'process-manager',
    'process-form-designer',
    'process-dict-designer',
    'process-script-designer',
    'process-xform',
    'process-application',
    'service-invoke-designer',
  ],
  查询中心: [
    'query',
    'queryview',
    'query-query',
    'query-explorer',
    'query-manager-deep',
    'query-designer',
    'query-statement-designer',
    'query-table-designer',
    'query-view-designer',
    'query-stat-designer',
    'query-importer-designer',
    'find-designer',
  ],
  门户管理: [
    'portal',
    'portal-designer',
    'portal-dict-designer',
    'portal-page-designer',
    'portal-script-designer',
    'portal-widget-designer',
  ],
  系统运维: [
    'control-panel',
    'design-center',
    'program',
    'server',
    'config-designer',
    'deployment',
    'log-viewer',
    'jpush',
    'pdf-viewer',
    'common',
  ],
}

describe('R298 管理区分组侧栏与嵌套路由契约', () => {
  it('恰好六组，组名与归类符合定稿', () => {
    expect(adminNavGroups.map((g) => g.label)).toEqual([
      '组织与权限',
      '内容管理',
      '流程管理',
      '查询中心',
      '门户管理',
      '系统运维',
    ])
    for (const group of adminNavGroups) {
      expect(group.icon, group.label + ' 组必须有图标').toBeTruthy()
      expect(
        group.items.map((i) => i.path.replace('/app/admin/', '')).sort(),
        group.label + ' 组成员必须符合定稿归类',
      ).toEqual([...EXPECTED_GROUPS[group.label]].sort())
    }
  })

  it('每个条目都有 /app/admin/ 前缀、中文标签与图标，且无重复', () => {
    const items = adminNavGroups.flatMap((g) => g.items)
    const paths = items.map((i) => i.path)
    expect(new Set(paths).size, '分组条目不得重复').toBe(paths.length)
    for (const item of items) {
      expect(item.path.startsWith('/app/admin/'), item.path).toBe(true)
      expect(item.label, item.path + ' 标签').toBeTruthy()
      expect(item.icon, item.path + ' 图标').toBeTruthy()
    }
  })

  it('分组条目与 main.ts 嵌套 children 的 58 条路由双向全覆盖，标签与 meta.title 同源', () => {
    const block = adminChildrenBlock()
    const routes = [
      ...block.matchAll(
        /\{\s*path: '([^']+)',\s*name: '[^']+',\s*component: \(\) => import\('\.\/views\/[^']+'\),\s*meta: \{ title: '([^']+)' \},?\s*\}/g,
      ),
    ].map((m) => ({ path: m[1], title: m[2] }))
    expect(routes.length, '嵌套管理区必须保留全部 58 条路由').toBe(58)
    const navItems = new Map(adminNavGroups.flatMap((g) => g.items).map((i) => [i.path, i.label]))
    for (const route of routes) {
      const full = '/app/admin/' + route.path
      expect(navItems.has(full), full + ' 必须出现在分组侧栏').toBe(true)
      expect(navItems.get(full), full + ' 侧栏标签必须等于 meta.title').toBe(route.title)
    }
    const routePaths = new Set(routes.map((r) => '/app/admin/' + r.path))
    for (const path of navItems.keys()) {
      expect(routePaths.has(path), path + ' 必须有嵌套注册路由').toBe(true)
    }
  })

  it('main.ts 嵌套结构与入口 redirect 正确', () => {
    expect(mainTs).toContain("import AdminLayout from './components/admin/AdminLayout.vue'")
    expect(mainTs).toContain('component: AdminLayout,')
    expect(adminChildrenBlock()).toContain("{ path: '', redirect: '/app/admin/control-panel' }")
    expect(mainTs, '不得残留平铺 admin/* 路由').not.toMatch(/path: 'admin\/[^']+'/)
  })

  it('58 条 R297 旧路径 redirect 别名逐条保留', () => {
    const aliases = [...mainTs.matchAll(/\{ path: '([^']+)', redirect: '\/app\/admin\/([^']+)' \}/g)]
    expect(aliases.length).toBe(58)
    for (const m of aliases) expect(m[1]).toBe(m[2])
    const block = adminChildrenBlock()
    for (const m of aliases) {
      expect(block.includes("{ path: '" + m[2] + "',"), '别名 ' + m[1] + ' 的目标必须已注册').toBe(true)
    }
  })

  it('admin 角色守卫与登录守卫逐字保留', () => {
    expect(mainTs).toContain("if (to.path.startsWith('/app/admin')) {")
    expect(mainTs).toContain('const roles = session.state.user?.roles ?? []')
    expect(mainTs).toContain("const isAdmin = roles.some((role) => role.toLowerCase() === 'admin')")
    expect(mainTs).toContain("if (!isAdmin) return { name: 'Dashboard' }")
    expect(mainTs).toContain("return { name: 'Login', query: { redirect: to.fullPath } }")
  })
})
