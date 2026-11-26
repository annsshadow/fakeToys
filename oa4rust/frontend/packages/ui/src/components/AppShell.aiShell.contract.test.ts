// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R300 B1 AI 应用外壳契约：侧栏品牌渐变、面包屑层级、活动页签、
 * Ctrl+K 命令面板（键盘可达）与主题切换必须真正落地并基于 --ai-* 令牌，
 * 同时不破坏原有导航/搜索标记/键盘可达语义。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')

describe('R300 B1 PC AI 应用外壳契约', () => {
  it('侧栏品牌与激活导航消费 AI 渐变令牌', () => {
    expect(source).toMatch(/\.logo-icon\s*\{[^}]*background:\s*var\(--ai-gradient-brand\)/)
    expect(source).toMatch(/\.nav-item\.active\s*\{[^}]*var\(--ai-gradient-surface\)/)
  })

  it('顶栏提供多段面包屑、页签条与命令面板入口', () => {
    expect(source).toContain('aria-label="面包屑"')
    expect(source).toContain('breadcrumb-trail')
    expect(source).toContain('aria-label="已打开的页签"')
    expect(source).toContain('data-testid="workspace-tabs"')
    expect(source).toContain('data-testid="command-palette-trigger"')
  })

  it('Ctrl+K 打开命令面板，面板用 AI 令牌且支持键盘选择与关闭', () => {
    expect(source).toContain('@keydown.ctrl.k.prevent="openCommandPalette"')
    expect(source).toContain('@keydown.meta.k.prevent="openCommandPalette"')
    expect(source).toContain('role="dialog"')
    expect(source).toContain('aria-modal="true"')
    expect(source).toContain('data-testid="command-palette"')
    expect(source).toContain('@keydown.esc.prevent="closeCommandPalette"')
    expect(source).toContain('@keydown.down.prevent="moveCommandSelection(1)"')
    expect(source).toContain('@keydown.up.prevent="moveCommandSelection(-1)"')
    expect(source).toContain('var(--ai-cmd-width)')
    expect(source).toContain('var(--ai-cmd-item-height)')
    expect(source).toContain('var(--ai-cmd-backdrop)')
    expect(source).toContain('var(--ai-shadow-3)')
  })

  it('主题切换具名且使用 spring/快速动效令牌', () => {
    expect(source).toMatch(/class="theme-toggle"[^>]*aria-label="切换主题"/)
    expect(source).toMatch(/\.theme-toggle[\s\S]*?var\(--ai-motion-fast\)[\s\S]*?var\(--ai-ease-spring\)/)
  })

  it('全局搜索回车跳转到真实全局搜索页面且不丢失标记/快捷键契约', () => {
    expect(source).toContain('@keydown.enter.prevent="submitGlobalSearch"')
    expect(source).toContain("router.push({ path: '/app/search', query: { q: query } })")
    expect(source).toContain('data-oa-global-search')
    expect(source).toContain('@keydown.ctrl.k.prevent')
  })
})
