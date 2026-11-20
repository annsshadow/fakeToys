// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R294 弹层表面与列表行选中态统一契约。
 *
 * 缺陷证据：
 *   1) 39 处视图各自定义 .modal-overlay，遮罩浓度 rgba(0,0,0,0.6)/rgba(0,0,0,.7) 混用，
 *      且浅色主题下弹层遮罩是突兀的纯黑——dark.css 双主题早已定义 --bg-overlay 令牌却未被使用；
 *   2) AppShell 用户下拉面板没有任何背景/边框（完全透明），展开时与底层内容叠影
 *      （R293 评审 t5 记录的在案问题）；
 *   3) 多个视图的列表行选中态用 border-left: 3px 实现指示条，选中切换时内容横移 3px
 *      （与 R293 已在 AppShell 导航修复的盒宽跳动同源）。
 *
 * 契约：
 *   a) dark.css 双主题均定义 --bg-overlay，且所有视图 .modal-overlay 的 background
 *      一律使用 var(--bg-overlay)，禁止裸 rgba(0,0,0,…) 遮罩；
 *   b) .user-dropdown 必须具备玻璃表面（--bg-glass + --glass-blur + --glass-border）；
 *   c) 列表行选中指示器必须用 inset box-shadow，不得再用 border-left: 3px。
 */
import { readFileSync, readdirSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const viewsDir = resolve(import.meta.dirname)
const uiTheme = resolve(viewsDir, '../../../../packages/ui/src/theme/dark.css')
const appShell = resolve(viewsDir, '../../../../packages/ui/src/components/AppShell.vue')

const read = (p: string): string => readFileSync(p, 'utf8')

/** 提取单个 .selector{…} 规则体 */
const ruleOf = (css: string, selector: string): string => {
  const escaped = selector.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  return css.match(new RegExp(`${escaped}\\s*\\{([^}]*)\\}`))?.[1] ?? ''
}

describe('R294 弹层表面统一', () => {
  it('dark.css 双主题都定义 --bg-overlay 遮罩令牌', () => {
    const css = read(uiTheme)
    expect(css).toMatch(/\[data-theme="dark"\]\s*\{[^}]*--bg-overlay:/)
    expect(css).toMatch(/\[data-theme="light"\]\s*\{[^}]*--bg-overlay:/)
  })

  it('所有视图 .modal-overlay 的 background 一律用 var(--bg-overlay)', () => {
    const offenders: string[] = []
    for (const file of readdirSync(viewsDir).filter((f) => f.endsWith('.vue'))) {
      const css = read(resolve(viewsDir, file))
      for (const m of css.matchAll(/\.modal-overlay\s*\{([^}]*)\}/g)) {
        const body = m[1]
        if (!/background\s*:/.test(body)) continue
        if (!body.includes('var(--bg-overlay)')) {
          offenders.push(`${file}: ${body.trim().slice(0, 60)}`)
        }
      }
    }
    expect(offenders, '弹层遮罩必须统一到 --bg-overlay 令牌').toEqual([])
  })

  it('AppShell 用户下拉具备玻璃表面，不再透明叠影', () => {
    const dropdown = ruleOf(read(appShell), '.user-dropdown')
    expect(dropdown).toContain('var(--bg-glass)')
    expect(dropdown).toContain('var(--glass-blur)')
    expect(dropdown).toContain('var(--glass-border)')
  })
})

describe('R294 列表行选中指示器统一', () => {
  const cases: [string, string][] = [
    ['BBSForum.vue', '.section-item.active'],
    ['ConfigDesignerApp.vue', '.sb-item.active'],
    ['FormDesigner.vue', '.list-item.active'],
    ['IMChat.vue', '.conv-item.active'],
    ['QueryManagerDeep.vue', '.qs-item.active'],
    ['QueryStatementDesigner.vue', '.sb-item.active'],
  ]

  it.each(cases)('%s 的 %s 用 inset box-shadow 指示选中', (file, selector) => {
    const css = read(resolve(viewsDir, file))
    const escaped = selector.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
    const matches = [...css.matchAll(new RegExp(`${escaped}\\s*\\{([^}]*)\\}`, 'g'))]
    expect(matches.length, '选择器必须存在').toBeGreaterThan(0)
    for (const m of matches) {
      expect(m[1]).toContain('box-shadow: inset 3px 0 0 var(--color-primary)')
      expect(m[1]).not.toContain('border-left')
    }
  })
})
