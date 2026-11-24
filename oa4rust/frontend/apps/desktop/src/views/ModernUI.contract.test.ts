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
  it('ModernUI.vue 提供根容器与默认插槽；scoped 样式仅作用于组件私有类，不局部化全局类', () => {
    const src = adminRead('ModernUI.vue')
    expect(src).toMatch(/class="modern-page[^"]*"/)
    expect(src).toMatch(/<slot\s*\/>/)
    expect(src).toMatch(/\.ai-b5-surface\s*\{/)
    // 队长 §219 裁决：组件私有类（如 .ai-b5-surface）允许 scoped；
    // 但 scoped 块内禁止定义 .modern-*/.glass-card/.admin-* 全局类，避免全局样式被局部化。
    const scopedCss = (src.match(/<style[^>]*\bscoped\b[^>]*>([\s\S]*?)<\/style>/g) ?? [])
      .map((block) => block.replace(/<style[^>]*>/, '').replace(/<\/style>/, ''))
      .join('\n')
    expect(scopedCss).not.toMatch(/\.modern-[a-z0-9-]+/)
    expect(scopedCss).not.toMatch(/\.glass-card\b/)
    expect(scopedCss).not.toMatch(/\.admin-[a-z0-9-]+/)
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
    expect(src).toMatch(/<h1[^>]*class="[^"]*admin-title/)
  })

  it('R450 .ai-b5-surface 的 background 使用正确令牌 --ai-gradient-surface，不含错误词序 --ai-surface-gradient', () => {
    const src = adminRead('ModernUI.vue')
    const rule = src.match(/\.ai-b5-surface\s*\{([\s\S]*?)\}/)
    expect(rule, '存在 .ai-b5-surface scoped 规则块').not.toBeNull()
    const body = rule?.[1] ?? ''
    expect(body).toMatch(/background:\s*var\(--ai-gradient-surface\)/)
    expect(body).not.toContain('--ai-surface-gradient')
  })
})
