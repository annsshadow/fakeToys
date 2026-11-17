// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 移动端正文 token 对比度契约（WCAG 2.1 AA 1.4.3，正文≥4.5:1）。
 *
 * 与桌面 R185 同构：--color-text-muted / --color-text-secondary 在移动端用于
 * 空态提示、列表元数据、回复时间等真实正文（不是装饰）。旧值 #8a94a6/#90979f
 * 对页面底色 #f5f7fa 与卡片白 #ffffff只有 2.85~3.06:1，低视力用户在浅色
 * 背景下读不清。本测试按 sRGB 相对亮度公式实算两个最常用背景，钉死 ≥4.5:1
 * 与文字层级（body > secondary > muted），防回归。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const app = readFileSync(resolve(import.meta.dirname, 'App.vue'), 'utf8')

function token(name: string): string {
  const m = app.match(new RegExp(name.replace(/[-]/g, '\\-') + '\\s*:\\s*(#[0-9a-fA-F]{6})'))
  expect(m, 'App.vue 缺少 token ' + name).not.toBeNull()
  return m![1]
}

/** WCAG 2.x sRGB 相对亮度。 */
function luminance(hex: string): number {
  const c = hex.replace('#', '')
  const channel = (i: number) => {
    const v = parseInt(c.slice(i, i + 2), 16) / 255
    return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4)
  }
  return 0.2126 * channel(0) + 0.7152 * channel(2) + 0.0722 * channel(4)
}

function contrast(a: string, b: string): number {
  const l1 = luminance(a)
  const l2 = luminance(b)
  return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05)
}

const BACKGROUNDS = ['#f5f7fa', '#ffffff'] // 页面底 / 卡片表面

describe('移动端文字 token 对比度（WCAG 2.1 AA 1.4.3）', () => {
  for (const name of ['--color-text-muted', '--color-text-secondary', '--color-text-body']) {
    it(name + ' 对页面底与卡片表面均达正文 4.5:1', () => {
      const value = token(name)
      for (const bg of BACKGROUNDS) {
        expect(contrast(value, bg), name + ' ' + value + ' 对 ' + bg).toBeGreaterThanOrEqual(4.5)
      }
    })
  }

  it('保留文字层级：body 深于 secondary 深于 muted', () => {
    const body = luminance(token('--color-text-body'))
    const secondary = luminance(token('--color-text-secondary'))
    const muted = luminance(token('--color-text-muted'))
    expect(body).toBeLessThan(secondary) // 亮度越低看起来越深
    expect(secondary).toBeLessThan(muted)
  })
})
