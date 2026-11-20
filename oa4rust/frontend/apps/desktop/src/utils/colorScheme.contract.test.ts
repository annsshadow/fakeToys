// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R190 全局主题样式标准契约：原生表单控件配色跟随主题（CSS color-scheme）。
 *
 * 主题通过 <html data-theme="dark|light"> 切换，但整套 dark.css 只定义了
 * 自定义属性与 scoped 样式，从未声明 color-scheme。浏览器因此不知道页面是深色
 * 还是浅色：<select> 下拉、<input type="date"> 日历、input autofill 背景、
 * Firefox 滚动条与原生 checkbox/radio 都会按系统默认（通常浅色）渲染，
 * 在深色主题下出现刺眼的浅色原生控件——真实的一致性与可读性缺陷。
 * 注意这与 prefers-color-scheme 媒体查询（仅检测系统偏好）完全不同。
 * 本测试钉死两个主题块各自声明正确的 color-scheme，防止回归。
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

describe('全局主题的原生控件配色标准（CSS color-scheme）', () => {
  it('dark 主题块声明 color-scheme: dark，让原生控件渲染为深色', () => {
    const block = themeBlock('dark')
    expect(block, 'dark 主题缺少 color-scheme: dark').toMatch(/color-scheme:\s*dark\s*;/)
  })

  it('light 主题块声明 color-scheme: light，让原生控件渲染为浅色', () => {
    const block = themeBlock('light')
    expect(block, 'light 主题缺少 color-scheme: light').toMatch(/color-scheme:\s*light\s*;/)
  })

  it('两个主题块的 color-scheme 值不同，切换主题时原生控件同步切换', () => {
    const dark = themeBlock('dark').match(/color-scheme:\s*(\w+)/)?.[1]
    const light = themeBlock('light').match(/color-scheme:\s*(\w+)/)?.[1]
    expect(dark).toBe('dark')
    expect(light).toBe('light')
    expect(dark).not.toBe(light)
  })
})
