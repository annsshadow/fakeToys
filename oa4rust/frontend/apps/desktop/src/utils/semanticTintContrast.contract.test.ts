// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R192 全局主题样式标准契约：语义色文字与其同色半透明底色叠放时的对比度
 * （WCAG 2.1 AA / 1.4.3，≥4.5:1）。
 *
 * R191 已保证 success/warning/error 对 base/surface/elevated 裸背景达 AA，
 * 但全仓 16 处徽章/状态标签/删除按钮把同一前景色叠在该语义色的 glow/soft
 * 半透明底色上（如 .badge.rejected{background:var(--color-error-glow);
 * color:var(--color-error)}）。R191 之后亮色三语义色文字在这些底色上只有
 * 3.05~3.46:1，暗色 error/accent 只有 2.87~3.52:1，低视力用户读不清错误、
 * 迟到、审核状态。底色 token 定义在全局 dark.css，必须在主题层调低 alpha
 * 一处修复，而不是逐个改写 16 处视图。
 *
 * 本测试解析两个主题块的 token，按 WCAG sRGB 公式把半透明底色合成到三种
 * 背景后实算 8 组 «语义文字/语义底色» 在三种背景上的对比度；任何回退都会变红。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
const themeCss = readFileSync(path.join(frontendRoot, 'packages/ui/src/theme/dark.css'), 'utf8')

type Paint = { rgb: [number, number, number]; alpha: number }

/** 取 [data-theme="x"] 的花括号块内容。 */
function themeBlock(theme: 'dark' | 'light'): string {
  const start = themeCss.indexOf(`[data-theme="${theme}"]`)
  expect(start, `主题块 ${theme} 不存在`).toBeGreaterThan(-1)
  const open = themeCss.indexOf('{', start)
  const close = themeCss.indexOf('}', open)
  return themeCss.slice(open + 1, close)
}

/** 解析 #hex 或 rgba() 形式的 CSS 变量。 */
function cssPaint(block: string, name: string): Paint {
  const match = block.match(new RegExp(`${name}:\\s*([^;]+)`))
  if (!match) throw new Error(`主题块缺少 CSS 变量 ${name}`)
  const value = match[1].trim()
  if (value.startsWith('#')) {
    const raw = value.slice(1)
    const full =
      raw.length === 3
        ? raw
            .split('')
            .map((c) => c + c)
            .join('')
        : raw
    return {
      rgb: [0, 2, 4].map((i) => parseInt(full.slice(i, i + 2), 16)) as [number, number, number],
      alpha: 1,
    }
  }
  const m = value.match(/rgba?\(([^)]+)\)/)
  if (!m) throw new Error(`无法解析 CSS 变量 ${name}: ${value}`)
  const parts = m[1].split(',').map((x) => x.trim())
  return {
    rgb: [Number(parts[0]), Number(parts[1]), Number(parts[2])],
    alpha: parts.length > 3 ? Number(parts[3]) : 1,
  }
}

function srgbToLinear(channel: number): number {
  const c = channel / 255
  return c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4
}

function luminance(rgb: [number, number, number]): number {
  return 0.2126 * srgbToLinear(rgb[0]) + 0.7152 * srgbToLinear(rgb[1]) + 0.0722 * srgbToLinear(rgb[2])
}

/** 半透明底色按 WCAG 规范合成到背景。 */
function composite(paint: Paint, bg: [number, number, number]): [number, number, number] {
  return paint.rgb.map((c, i) => Math.round(c * paint.alpha + bg[i] * (1 - paint.alpha))) as [number, number, number]
}

function contrastRatio(a: [number, number, number], b: [number, number, number]): number {
  const la = luminance(a)
  const lb = luminance(b)
  return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05)
}

/** 全仓视图中真实出现的 «文字 token / 底色 token» 组合（见交付证据）。 */
const PAIRS: Record<'dark' | 'light', [string, string][]> = {
  dark: [
    ['--color-success', '--color-success-glow'],
    ['--color-warning', '--color-warning-glow'],
    ['--color-error', '--color-error-glow'],
    ['--color-accent', '--color-accent-soft'],
  ],
  light: [
    ['--color-success', '--color-success-glow'],
    ['--color-warning', '--color-warning-glow'],
    ['--color-error', '--color-error-glow'],
    ['--color-accent', '--color-accent-glow'],
  ],
}
const BACKGROUNDS = ['--bg-base', '--bg-surface', '--bg-elevated'] as const

describe('全局主题的语义文字/同色底色对比度标准（WCAG 2.1 AA / 1.4.3）', () => {
  for (const theme of ['dark', 'light'] as const) {
    const block = themeBlock(theme)
    for (const [fgVar, bgVar] of PAIRS[theme]) {
      it(`${theme} 主题 ${fgVar} 叠在 ${bgVar} 上，于 base/surface/elevated 均 ≥ 4.5:1`, () => {
        const fg = cssPaint(block, fgVar).rgb
        // light 块未重定义的 tint token 会继承 dark 根块的值（CSS 级联规则）。
        const tint = block.includes(`${bgVar}:`) ? cssPaint(block, bgVar) : cssPaint(themeBlock('dark'), bgVar)
        for (const bgName of BACKGROUNDS) {
          const bg = cssPaint(block, bgName).rgb
          const ratio = contrastRatio(fg, composite(tint, bg))
          expect(
            ratio,
            `${theme}: ${fgVar} on ${bgVar} over ${bgName} = ${ratio.toFixed(2)}:1 (< 4.5)`,
          ).toBeGreaterThanOrEqual(4.5)
        }
      })
    }
  }
})
