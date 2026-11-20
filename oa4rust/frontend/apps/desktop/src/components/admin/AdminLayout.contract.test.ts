// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 AdminLayout 契约：分组侧栏 + 面包屑/页标题 + 右侧 <router-view>。
 * 视觉走设计令牌与全局 glass-card，禁止在组件内重复定义 .glass-card。
 * 导航项用 router-link（原生键盘可达）+ isNavActive 派生 aria-current。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AdminLayout.vue'), 'utf8')

describe('R298 AdminLayout 结构契约', () => {
  it('左侧分组侧栏消费 adminNavGroups，右侧渲染 <router-view>', () => {
    expect(source).toContain("import { adminNavGroups } from './adminNav'")
    expect(source).toMatch(/v-for="group in adminNavGroups"/)
    expect(source).toMatch(/v-for="item in group\.items"/)
    expect(source).toMatch(/<router-view\s*\/>/)
  })

  it('侧栏是带 aria-label 的 nav landmark，导航项为 router-link 且派生 aria-current', () => {
    expect(source).toMatch(/<nav[^>]*aria-label="管理区导航"/)
    expect(source).toMatch(/<router-link[^>]*:to="item\.path"/)
    expect(source).toMatch(/:aria-current="isActive\(item\.path\) \? 'page' : undefined"/)
    expect(source).not.toMatch(/aria-current="page"/)
    expect(source).toContain("import { isNavActive } from '@oa4rust/ui/components/navActive'")
  })

  it('顶部面包屑与页标题来自当前路由（meta.title 同源）', () => {
    expect(source).toMatch(/class="admin-breadcrumb"/)
    expect(source).toContain('系统管理')
    expect(source).toMatch(/route\.meta\.title/)
    expect(source).toMatch(/<h1[^>]*class="admin-title"/)
  })

  it('视觉全部走设计令牌，不重复定义 glass-card，不写裸 hex 颜色', () => {
    expect(source).toMatch(/var\(--space-\d+\)/)
    expect(source).toMatch(/var\(--radius-(sm|md|lg)\)/)
    expect(source).toMatch(/var\(--(color|text|bg)-[a-z-]+\)/)
    expect(source).not.toMatch(/\.glass-card\s*\{/)
    expect(source).not.toMatch(/#[0-9a-fA-F]{3,8}\b/)
  })
})
