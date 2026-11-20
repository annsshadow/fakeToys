// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R300 共享基座契约：管理页标题只由 AdminLayout 的 h1 渲染一次。
 * AdminLayout 默认 provide(ADMIN_HIDE_PAGE_TITLE, ref(true))，所有使用
 * AdminPageHeader 的页面默认隐藏其标题/副标题，但 actions 插槽必须保留；
 * 任何页面都不得强制 provide(ref(false)) 重新显示重复标题。
 */
import { readdirSync, readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')

const collectVue = (dir: string): string[] =>
  readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const full = resolve(dir, entry.name)
    if (entry.isDirectory()) return collectVue(full)
    return entry.name.endsWith('.vue') ? [full] : []
  })

describe('R300 管理页标题去重基座契约', () => {
  it('adminInjection 导出 ADMIN_HIDE_PAGE_TITLE 独立 injection key', () => {
    const src = read('adminInjection.ts')
    expect(src).toMatch(/export const ADMIN_HIDE_PAGE_TITLE/)
    expect(src).toMatch(/InjectionKey<Ref<boolean>>/)
  })

  it('index barrel 对外导出隐藏页标题 key 与 AdminPageHeader', () => {
    const src = read('index.ts')
    expect(src).toContain('ADMIN_HIDE_PAGE_TITLE')
    expect(src).toContain("export { default as AdminPageHeader } from './AdminPageHeader.vue'")
  })

  it('AdminLayout 默认提供隐藏页标题开关（ref(true)）', () => {
    const src = read('AdminLayout.vue')
    expect(src).toContain('ADMIN_HIDE_PAGE_TITLE')
    expect(src).toMatch(/provide\(ADMIN_HIDE_PAGE_TITLE, ref\(true\)\)/)
    expect(src).toMatch(/<h1[^>]*class="admin-title"/)
  })

  it('AdminPageHeader inject 开关后只隐藏标题/副标题，保留 actions 插槽', () => {
    const src = read('AdminPageHeader.vue')
    expect(src).toContain('ADMIN_HIDE_PAGE_TITLE')
    expect(src).toContain('inject(ADMIN_HIDE_PAGE_TITLE')
    expect(src).toMatch(/v-if="!hidePageTitle"/)
    expect(src).toMatch(/class="admin-page-title"/)
    expect(src).toMatch(/class="admin-page-subtitle"/)
    expect(src, '操作按钮区不得受隐藏开关影响').toMatch(
      /<div class="admin-page-actions">\s*<slot name="actions" \/>\s*<\/div>/,
    )
  })

  it('所有 AdminPageHeader 页默认隐藏标题/副标题：无页面强制 provide(ref(false))', () => {
    const viewsDir = resolve(import.meta.dirname, '../../views')
    const pages = collectVue(viewsDir).filter((p) => readFileSync(p, 'utf8').includes('<AdminPageHeader'))
    expect(pages.length, 'AdminPageHeader 页数量应覆盖全部管理页').toBeGreaterThanOrEqual(30)
    const forcedVisible = pages.filter((p) =>
      /provide\(ADMIN_HIDE_PAGE_TITLE,\s*ref\(false\)\)/.test(readFileSync(p, 'utf8')),
    )
    expect(forcedVisible, '不允许页面强制显示 AdminPageHeader 标题，避免与 AdminLayout h1 重复').toEqual([])
  })
})

describe('R302 系统管理区标题去重', () => {
  const shell = readFileSync(
    resolve(import.meta.dirname, '../../../../../packages/ui/src/components/AppShell.vue'),
    'utf8',
  )
  const layout = readFileSync(resolve(import.meta.dirname, 'AdminLayout.vue'), 'utf8')

  it('topbar currentTitle 在管理区路径下隐藏，员工页保留', () => {
    expect(shell).toMatch(/<div[^>]*v-if="!isAdminArea"[^>]*class="breadcrumb"/)
    expect(shell).toMatch(/isAdminArea/)
    expect(shell).toMatch(/route\.path\.startsWith\('\/app\/admin'\)/)
  })

  it('AdminLayout 面包屑只到分组，不再重复末级页名', () => {
    expect(layout).not.toMatch(/class="admin-breadcrumb-current"/)
    expect(layout).toMatch(/admin-breadcrumb-group/)
    expect(layout).toMatch(/<h1[^>]*class="admin-title"/)
  })
})
