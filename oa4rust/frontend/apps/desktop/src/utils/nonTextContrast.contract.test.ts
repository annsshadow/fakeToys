// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R189 全局主题样式标准契约：UI 组件边界与状态指示对比度（WCAG 2.1 AA / 1.4.11，≥3:1）。
 *
 * --border-subtle / --border-active 是全仓复用 200+ 次的组件边界 token：输入框、按钮、
 * 复选框、标签选中态、卡片与 skip-link 的边框都依赖它们。旧值带低 alpha（dark subtle
 * rgba(0,212,255,.12)、active rgba(0,212,255,.35)；light subtle rgba(0,0,0,.08)、
 * active rgba(37,99,235,.4)），与实际背景合成后对比度只有 1.19–2.30:1，远低于非文本
 * 对比度要求的 3:1——键盘/低视力用户看不清控件边界与选中状态。因为 token 定义在全局
 * 主题层，必须在 dark.css 一处抬升，而不是逐视图改 200+ 处。
 *
 * 本测试解析 dark.css 两个主题块，把 rgba 按 WCAG 规范与背景做 alpha 合成后，用 sRGB
 * 相对亮度公式实算对比度：既钉死所有边界 token 在 base/surface/elevated 三种背景上
 * ≥3:1，也验证 active 的层级强于 subtle，防止把两者调成同一视觉权重。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
const themeCss = readFileSync(path.join(frontendRoot, 'packages/ui/src/theme/dark.css'), 'utf8')

const AA_NON_TEXT = 3

type Rgb = [number, number, number]
type Paint = { rgb: Rgb; alpha: number }

/** 取 [data-theme="x"] 的花括号块内容。 */
function themeBlock(theme: 'dark' | 'light'): string {
  const start = themeCss.indexOf(`[data-theme="${theme}"]`)
  expect(start, `主题块 ${theme} 不存在`).toBeGreaterThan(-1)
  const open = themeCss.indexOf('{', start)
  const close = themeCss.indexOf('}', open)
  return themeCss.slice(open + 1, close)
}

/** 从主题块读取一个 CSS 变量，支持 #hex 与 rgba()/rgb() 两种写法。 */
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
      rgb: [0, 2, 4].map((i) => parseInt(full.slice(i, i + 2), 16)) as Rgb,
      alpha: 1,
    }
  }
  const m = value.match(/rgba?\(([^)]+)\)/)
  if (!m) throw new Error(`无法解析 CSS 变量 ${name} 的颜色值: ${value}`)
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

/** WCAG 2.1 相对亮度（sRGB，D65）。 */
function luminance(rgb: Rgb): number {
  return 0.2126 * srgbToLinear(rgb[0]) + 0.7152 * srgbToLinear(rgb[1]) + 0.0722 * srgbToLinear(rgb[2])
}

/** 把带 alpha 的边界色按 WCAG 规范合成到实背景上。 */
function composite(paint: Paint, bg: Rgb): Rgb {
  return paint.rgb.map((channel, i) => Math.round(channel * paint.alpha + bg[i] * (1 - paint.alpha))) as Rgb
}

function contrastRatio(a: Rgb, b: Rgb): number {
  const la = luminance(a)
  const lb = luminance(b)
  return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05)
}

const BACKGROUNDS = ['--bg-base', '--bg-surface', '--bg-elevated'] as const

/** 某个边界 token 在三种背景上的最小对比度。 */
function minBoundaryRatio(theme: 'dark' | 'light', varName: string): number {
  const block = themeBlock(theme)
  const paint = cssPaint(block, varName)
  return Math.min(
    ...BACKGROUNDS.map((bgVar) => {
      const bg = cssPaint(block, bgVar).rgb
      return contrastRatio(composite(paint, bg), bg)
    }),
  )
}

describe('全局主题的 UI 组件边界对比度标准（WCAG 2.1 AA / 1.4.11）', () => {
  for (const theme of ['dark', 'light'] as const) {
    for (const varName of ['--border-subtle', '--border-active']) {
      it(`${theme} 主题 ${varName} 在 base/surface/elevated 三种背景上均 ≥ ${AA_NON_TEXT}:1`, () => {
        const block = themeBlock(theme)
        const paint = cssPaint(block, varName)
        for (const bgVar of BACKGROUNDS) {
          const bg = cssPaint(block, bgVar).rgb
          const effective = composite(paint, bg)
          const ratio = contrastRatio(effective, bg)
          expect(
            ratio,
            `${theme}: ${varName} 合成到 ${bgVar} = ${ratio.toFixed(2)}:1 (< ${AA_NON_TEXT})`,
          ).toBeGreaterThanOrEqual(AA_NON_TEXT)
        }
      })
    }

    it(`${theme} 主题保留层级：active 边界比 subtle 边界更醒目`, () => {
      const subtle = minBoundaryRatio(theme, '--border-subtle')
      const active = minBoundaryRatio(theme, '--border-active')
      expect(
        active,
        `${theme}: active ${active.toFixed(2)}:1 未强于 subtle ${subtle.toFixed(2)}:1，层级被抹平`,
      ).toBeGreaterThan(subtle)
    })
  }
})
