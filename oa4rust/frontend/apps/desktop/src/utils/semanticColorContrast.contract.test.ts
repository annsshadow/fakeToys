// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R191 全局主题样式标准契约：亮色语义色作正文色的对比度（WCAG 2.1 AA 1.4.3，≥4.5:1）。
 */
import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'

const CSS_PATH = new URL('../../../../packages/ui/src/theme/dark.css', import.meta.url)

function readCss(): string {
  return readFileSync(CSS_PATH, 'utf8')
}

/** 截取 [data-theme=...] 规则块的花括号内容 */
function themeBlock(css: string, name: 'dark' | 'light'): string {
  const selector = `[data-theme="${name}"]`
  const start = css.indexOf(selector)
  if (start < 0) throw new Error(`未找到 ${selector} 规则块`)
  const open = css.indexOf('{', start)
  const close = css.indexOf('}', open)
  if (open < 0 || close < 0) throw new Error(`[data-theme=${name}] 规则块括号不完整`)
  return css.slice(open + 1, close)
}

function cssVar(block: string, name: string): string {
  const at = block.indexOf(`${name}:`)
  if (at < 0) throw new Error(`未找到 CSS 变量 ${name}`)
  const rest = block.slice(at + name.length + 1)
  const end = rest.indexOf(';')
  if (end < 0) throw new Error(`CSS 变量 ${name} 缺少分号`)
  return rest.slice(0, end).trim()
}

function hexToRgb(hex: string): [number, number, number] {
  const value = hex.replace('#', '')
  if (value.length !== 6) throw new Error(`期望 6 位十六进制颜色值，实际: ${hex}`)
  return [
    Number.parseInt(value.slice(0, 2), 16),
    Number.parseInt(value.slice(2, 4), 16),
    Number.parseInt(value.slice(4, 6), 16),
  ]
}

function relativeLuminance([r, g, b]: [number, number, number]): number {
  const channel = (v: number) => {
    const c = v / 255
    return c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4
  }
  return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

function contrastRatio(fg: string, bg: string): number {
  const l1 = relativeLuminance(hexToRgb(fg))
  const l2 = relativeLuminance(hexToRgb(bg))
  return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05)
}

describe('R191 亮色语义色正文对比度 (WCAG 2.1 AA 1.4.3)', () => {
  const light = themeBlock(readCss(), 'light')
  const semanticColors = ['--color-success', '--color-warning', '--color-error'] as const
  const backgrounds = ['--bg-base', '--bg-surface', '--bg-elevated'] as const

  for (const colorVar of semanticColors) {
    const color = cssVar(light, colorVar)
    describe(`${colorVar} (${color}) 作为文字色`, () => {
      for (const bgVar of backgrounds) {
        const bg = cssVar(light, bgVar)
        it(`在 ${bgVar} (${bg}) 上对比度 >= 4.5:1`, () => {
          expect(contrastRatio(color, bg)).toBeGreaterThanOrEqual(4.5)
        })
      }
    })
  }
})
