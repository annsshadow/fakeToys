// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * R297 导航归属契约（承接 R296 死链缺陷）+ R298 管理区重构：
 * 1) PC 侧栏收敛为 4 个员工分组 + 1 个「系统管理」单入口（分组菜单移入 AdminLayout）；
 * 2) 每个导航项都在 main.ts 有对应注册路由，绝不命中兜底 → /login；
 * 3) 管理/设计器页统一嵌套在 /app/admin/*（AdminLayout children），无死链；
 * 4) 移除 disabled 的「工具」占位项；
 * 5) 移动端仅保留业务页。
 */
const shell = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')
const mainTs = readFileSync(resolve(import.meta.dirname, '../../../../apps/desktop/src/main.ts'), 'utf8')

const navPaths = [...new Set([...shell.matchAll(/path: '(\/app\/[^']+)'/g)].map((m) => m[1]))]
const groupHeaders = [...shell.matchAll(/label: '([^']+)',\s*icon: '[^']+',(\s*admin: true,)?\s*items: \[/g)].map(
  (m) => ({ label: m[1], admin: Boolean(m[2]) }),
)

// R298：admin/* 路由嵌套在 AdminLayout 父路由（path: 'admin'）的 children 里，路径为相对短名。
const adminBlockStart = mainTs.indexOf("path: 'admin',")
const adminBlockEnd = mainTs.indexOf('\n      ],', adminBlockStart)
const adminBlock = adminBlockStart > -1 ? mainTs.slice(adminBlockStart, adminBlockEnd) : ''

const adminComponentPaths = new Set(
  [...adminBlock.matchAll(/\{\s*path: '([^']+)',\s*name: '[^']+'/g)].map((m) => `/app/admin/${m[1]}`),
)

const registeredPaths = new Set<string>()
for (const m of mainTs.matchAll(/path: '([^']+)'/g)) {
  const p = m[1]
  if (p.startsWith('/app/')) registeredPaths.add(p)
  else if (p === '/app') registeredPaths.add(p)
  else if (!p.startsWith('/') && p !== '' && !p.includes(':')) registeredPaths.add(`/app/${p}`)
}
for (const p of adminComponentPaths) registeredPaths.add(p)

describe('AppShell 导航归属与路由完整性（R297）', () => {
  it('PC 侧栏收敛为 4 个员工分组 + 1 个系统管理单入口分组', () => {
    expect(
      groupHeaders.map((g) => g.label),
      '分组结构必须是 工作台/沟通/协作/个人/系统管理',
    ).toEqual(['工作台', '沟通', '协作', '个人', '系统管理'])
    expect(groupHeaders[4]?.admin, '最后一个分组必须是管理员区').toBe(true)
    expect(
      groupHeaders.slice(0, 4).some((g) => g.admin),
      '员工分组不得标记为管理员区',
    ).toBe(false)
  })

  it('移除 disabled 的「工具」占位项', () => {
    expect(shell).not.toMatch(/label: '工具'/)
    expect(shell).not.toMatch(/disabled: true/)
  })

  it('全部导航项都在 main.ts 注册（含 /app/admin/*，无死链）', () => {
    const missing = navPaths.filter((p) => !registeredPaths.has(p))
    expect(missing, `无对应注册路由的导航项: ${missing.join(', ')}`).toEqual([])
  })

  it('R298：侧栏管理入口收敛为单项，分组导航由 AdminLayout 承载', () => {
    expect(shell).not.toMatch(/path: '\/app\/admin'/)
    const adminNavPaths = navPaths.filter((p) => p.startsWith('/app/admin/'))
    expect(adminNavPaths, '系统管理只保留控制面板单入口').toEqual(['/app/admin/control-panel'])
    const missingComponents = adminNavPaths.filter((p) => !adminComponentPaths.has(p))
    expect(missingComponents, `管理导航项没有真实组件路由: ${missingComponents.join(', ')}`).toEqual([])
    expect(adminComponentPaths.size, '嵌套管理路由必须保留全部 58 条').toBe(58)
    expect(mainTs).toContain('component: AdminLayout,')
  })

  it('移动端仅保留业务页且均为已注册路由', () => {
    const start = shell.indexOf('const mobileNavItems')
    const end = shell.indexOf('function isActive', start)
    expect(start, '未找到移动端导航定义').toBeGreaterThan(-1)
    const mobile = shell.slice(start, end)
    const mobilePaths = [...mobile.matchAll(/path: '(\/app\/[^']+)'/g)].map((m) => m[1])
    expect(mobilePaths).toEqual(['/app/dashboard', '/app/process', '/app/im', '/app/personal'])
    for (const p of mobilePaths) expect(registeredPaths.has(p), `${p} 未注册`).toBe(true)
  })
})
