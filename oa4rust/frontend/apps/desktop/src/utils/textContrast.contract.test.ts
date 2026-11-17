// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 全局主题样式标准契约：正文文字对比度达到 WCAG 2.1 AA（1.4.3，≥4.5:1）。
 *
 * --text-muted 是全仓复用 500+ 次的正文 token：副标题、表头、空状态、时间、
 * 元数据、标签都用它。旧值（dark #4a5568 / light #94a3b8）相对各自背景只有
 * 2.42:1 / 2.56:1，远低于 AA 的 4.5:1——属于用户真实可读性问题，且因为 token
 * 在全局主题层，必须在主题文件一处抬升才能全局生效，而不是逐视图改 500 处。
 * 本测试直接解析 dark.css 的两个主题块并计算 WCAG 相对亮度对比度，把这条标准
 * 钉死：任何把 muted 色调回低对比的改动都会立刻让本测试变红。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
const themeCss = readFileSync(path.join(frontendRoot, 'packages/ui/src/theme/dark.css'), 'utf8')

/** 取 [data-theme="x"] 的花括号块内容。 */
function themeBlock(theme: 'dark' | 'light'): string {
  const start = themeCss.indexOf(`[data-theme="${theme}"]`)
  expect(start, `主题块 ${theme} 不存在`).toBeGreaterThan(-1)
  const open = themeCss.indexOf('{', start)
  const close = themeCss.indexOf('}', open)
  return themeCss.slice(open + 1, close)
}

/** 从主题块中取一个十六进制 CSS 变量值。 */
function cssVar(block: string, name: string): string {
  const match = block.match(new RegExp(`${name}:\\s*(#[0-9a-fA-F]{3,8})`))
  if (!match) throw new Error(`主题块缺少 CSS 变量 ${name}`)
  return match[1]
}

function srgbToLinear(channel: number): number {
  const c = channel / 255
  return c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4
}

/** WCAG 2.1 相对亮度（sRGB，D65）。 */
function luminance(hex: string): number {
  const raw = hex.replace('#', '')
  const full =
    raw.length === 3
      ? raw
          .split('')
          .map((c) => c + c)
          .join('')
      : raw
  const r = srgbToLinear(parseInt(full.slice(0, 2), 16))
  const g = srgbToLinear(parseInt(full.slice(2, 4), 16))
  const b = srgbToLinear(parseInt(full.slice(4, 6), 16))
  return 0.2126 * r + 0.7152 * g + 0.0722 * b
}

function contrastRatio(a: string, b: string): number {
  const la = luminance(a)
  const lb = luminance(b)
  const lighter = Math.max(la, lb)
  const darker = Math.min(la, lb)
  return (lighter + 0.05) / (darker + 0.05)
}

const AA_NORMAL_TEXT = 4.5

describe('全局主题的正文文字对比度标准（WCAG 2.1 AA / 1.4.3）', () => {
  for (const theme of ['dark', 'light'] as const) {
    it(`${theme} 主题下 --text-muted 在 base/surface/elevated 三种背景上均 ≥ ${AA_NORMAL_TEXT}:1`, () => {
      const block = themeBlock(theme)
      const muted = cssVar(block, '--text-muted')
      for (const bgVar of ['--bg-base', '--bg-surface', '--bg-elevated']) {
        const background = cssVar(block, bgVar)
        const ratio = contrastRatio(muted, background)
        expect(
          ratio,
          `${theme}: --text-muted ${muted} on ${bgVar} ${background} = ${ratio.toFixed(2)}:1 (< ${AA_NORMAL_TEXT})`,
        ).toBeGreaterThanOrEqual(AA_NORMAL_TEXT)
      }
    })
  }

  it('暗色主题的 muted 仍比 secondary 更弱，保留文字层级', () => {
    const dark = themeBlock('dark')
    expect(luminance(cssVar(dark, '--text-muted'))).toBeLessThan(luminance(cssVar(dark, '--text-secondary')))
  })

  it('亮色主题的 muted 仍比 secondary 更弱，保留文字层级', () => {
    const light = themeBlock('light')
    expect(luminance(cssVar(light, '--text-muted'))).toBeGreaterThan(luminance(cssVar(light, '--text-secondary')))
  })
})
