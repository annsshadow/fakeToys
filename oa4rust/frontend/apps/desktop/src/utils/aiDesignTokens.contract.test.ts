// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R300 AI 风格令牌层契约：确保 AI-native 设计令牌可落地且不被回归破坏。
 *
 * ai-tokens.css 是新增的令牌层（--ai-* + .ai-*），不改 dark.css 的受测色值。
 * 本测试钉死三条意图：① 令牌层经 ui 入口产出；② 双主题各自定义品牌渐变/8px 栅格/
 * 克制玻璃面板；③ 组件类只消费令牌、骨架屏动画走令牌缓动（可被全局 reduced-motion 收敛）。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
const aiCss = readFileSync(path.join(frontendRoot, 'packages/ui/src/theme/ai-tokens.css'), 'utf8')
const uiIndex = readFileSync(path.join(frontendRoot, 'packages/ui/src/index.ts'), 'utf8')

function themeBlock(theme: 'dark' | 'light'): string {
  const start = aiCss.indexOf(`[data-theme="${theme}"]`)
  expect(start, `ai-tokens.css 缺少 ${theme} 主题块`).toBeGreaterThan(-1)
  const open = aiCss.indexOf('{', start)
  const close = aiCss.indexOf('}', open)
  return aiCss.slice(open + 1, close)
}

function cssVar(block: string, name: string): string {
  const match = block.match(new RegExp(`${name}:\\s*([^;]+)`))
  if (!match) throw new Error(`主题块缺少 CSS 变量 ${name}`)
  return match[1].trim()
}

describe('R300 AI 风格令牌层可落地契约', () => {
  it('ai-tokens.css 经 ui 包入口导入，随 style.css 产出', () => {
    expect(uiIndex).toMatch(/import\s+['"]\.\/theme\/ai-tokens\.css['"]/)
  })

  for (const theme of ['dark', 'light'] as const) {
    it(`${theme} 主题定义品牌渐变、8px 栅格、克制玻璃面板与命令面板令牌`, () => {
      const block = themeBlock(theme)
      expect(cssVar(block, '--ai-gradient-brand')).toContain('linear-gradient')
      expect(cssVar(block, '--ai-grid-unit')).toBe('8px')
      expect(cssVar(block, '--ai-glass-bg')).toMatch(/rgba?\(/)
      expect(cssVar(block, '--ai-indigo')).toMatch(/^#/)
      expect(cssVar(block, '--ai-cyan')).toMatch(/^#/)
      expect(cssVar(block, '--ai-cmd-item-height')).toBe('44px')
    })
  }

  it('8px 栅格令牌全部为 8 的整数倍，与 4px 基础步进兼容', () => {
    const block = themeBlock('dark')
    for (const name of ['--ai-grid-1', '--ai-grid-2', '--ai-grid-3', '--ai-grid-4', '--ai-grid-5', '--ai-grid-6', '--ai-grid-8']) {
      const value = cssVar(block, name)
      expect(value, `${name} 应为 px`).toMatch(/^\d+px$/)
      expect(Number.parseInt(value, 10) % 8, `${name} 应为 8 的整数倍`).toBe(0)
    }
  })

  it('提供可复用 AI 组件类：克制玻璃面板/渐变文字/品牌填充/骨架屏', () => {
    for (const cls of ['.ai-panel', '.ai-gradient-text', '.ai-brand-fill', '.ai-skeleton-line']) {
      expect(aiCss, `缺少 ${cls}`).toContain(cls)
    }
  })

  it('AI 组件类只消费令牌，不写死十六进制色值', () => {
    for (const cls of ['.ai-panel', '.ai-gradient-text', '.ai-brand-fill', '.ai-skeleton-line']) {
      const start = aiCss.indexOf(`${cls} {`)
      expect(start, `缺少 ${cls} 规则`).toBeGreaterThan(-1)
      const rule = aiCss.slice(start, aiCss.indexOf('}', start))
      expect(rule, `${cls} 出现硬编码色值`).not.toMatch(/#[0-9a-fA-F]{3,8}\b/)
    }
  })

  it('骨架屏动画走令牌缓动与 shimmer，可被全局 reduced-motion 守卫收敛', () => {
    const start = aiCss.indexOf('.ai-skeleton-line {')
    const rule = aiCss.slice(start, aiCss.indexOf('}', start))
    expect(rule).toMatch(/animation:\s*ai-shimmer/)
    expect(rule).toMatch(/var\(--ai-ease-spring\)/)
    expect(aiCss).toMatch(/@keyframes\s+ai-shimmer/)
  })
})
