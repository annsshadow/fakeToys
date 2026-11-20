// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 现代管理 UI 共享基座契约：
 * 1) .modern-* 全局类由 packages/ui 的 modern.css 定义并由 ui barrel 入口导入，任意页面模板可用；
 * 2) ModernUI.vue 提供轻量包装组件并在 admin barrel 导出；
 * 3) AdminLayout 通过 provide/inject 开关允许自带标题的页面隐藏布局大标题。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (from: string, name: string): string => readFileSync(resolve(import.meta.dirname, from, name), 'utf8')
const uiRoot = '../../../../packages/ui/src'
const uiRead = (name: string): string => read(uiRoot, name)
const adminRead = (name: string): string => read('../components/admin', name)

const REQUIRED_CLASSES = [
  'modern-page',
  'modern-hero',
  'modern-hero-eyebrow',
  'modern-hero-actions',
  'modern-section',
  'modern-section-title',
  'modern-grid',
  'modern-card',
  'modern-empty',
  'modern-chip',
  'modern-stat',
  'modern-btn',
]

describe('R298 现代管理 UI 全局样式契约', () => {
  it('packages/ui barrel 导入 modern.css（全局生效入口）', () => {
    const entry = uiRead('index.ts')
    expect(entry).toContain("import './theme/modern.css'")
  })

  it('modern.css 定义页面使用的全部 .modern-* 类', () => {
    const css = uiRead('theme/modern.css')
    for (const cls of REQUIRED_CLASSES) {
      expect(css, cls).toMatch(new RegExp(`\\.${cls}\\b`))
    }
  })

  it('modern.css 只用设计令牌，不定义裸 hex 颜色，也不重复定义 glass-card', () => {
    const css = uiRead('theme/modern.css')
    expect(css).toMatch(/var\(--(space|color|bg|text|border|radius|shadow|font-size|transition)-[a-z0-9-]+\)/)
    expect(css).not.toMatch(/#[0-9a-fA-F]{3,8}\b/)
    expect(css).not.toMatch(/\.glass-card\s*\{/)
  })
})

describe('R298 ModernUI 共享组件与 AdminLayout 标题开关契约', () => {
  it('ModernUI.vue 提供根容器与默认插槽，样式非 scoped（不局部化全局类）', () => {
    const src = adminRead('ModernUI.vue')
    expect(src).toMatch(/class="modern-page[^"]*"/)
    expect(src).toMatch(/<slot\s*\/>/)
    expect(src).not.toMatch(/<style[^>]*scoped/)
  })

  it('barrel 导出 ModernUI 组件与 AdminLayout 标题隐藏 injection key', () => {
    const src = adminRead('index.ts')
    expect(src).toContain("export { default as ModernUI } from './ModernUI.vue'")
    expect(src).toContain('ADMIN_LAYOUT_HIDE_TITLE')
  })

  it('AdminLayout 通过 injection key 提供并消费标题隐藏开关，默认不隐藏', () => {
    const src = adminRead('AdminLayout.vue')
    expect(src).toContain('ADMIN_LAYOUT_HIDE_TITLE')
    expect(src).toMatch(/provide\(ADMIN_LAYOUT_HIDE_TITLE/)
    expect(src).toMatch(/inject\(ADMIN_LAYOUT_HIDE_TITLE/)
    expect(src).toMatch(/v-if="!hideTitle"/)
    expect(src).toMatch(/<h1[^>]*class="admin-title"/)
  })
})
