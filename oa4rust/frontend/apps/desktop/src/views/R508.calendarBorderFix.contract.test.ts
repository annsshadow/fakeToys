// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R508 日历新建表单边界对比度修复契约（WCAG 2.1 AA 1.4.11）：
 * 三控件 .cal-create-input/.cal-create-color/.cal-create-cancel 的 border 颜色令牌
 * 由低 alpha 的 --ai-glass-border 换成契约达标令牌 --border-subtle；装饰性表单容器 L816 不动。
 * 真父 RED、修复版 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'CalendarApp.vue'), 'utf8')
const darkCss = readFileSync(resolve(import.meta.dirname, '../../../../packages/ui/src/theme/dark.css'), 'utf8')
const aiTokensCss = readFileSync(resolve(import.meta.dirname, '../../../../packages/ui/src/theme/ai-tokens.css'), 'utf8')
const NL = String.fromCharCode(10)

function lineWith(fragment: string): string {
  const i = source.indexOf(fragment)
  expect(i).toBeGreaterThanOrEqual(0)
  return source.slice(i, source.indexOf(NL, i))
}

describe('R508 日历新建表单 border 对比度修复契约', () => {
  it('核心：三个功能控件的 border 颜色改用达标令牌 --border-subtle', () => {
    const inputDeclaration = lineWith('ai-page .cal-create-color {')
    expect(inputDeclaration).toContain('border: 1px solid var(--border-subtle)')
    expect(source).toContain('.ai-page .cal-create-input,' + NL + '.ai-page .cal-create-color { border: 1px solid var(--border-subtle)')
    expect(lineWith('.ai-page .cal-create-cancel {')).toContain('border: 1px solid var(--border-subtle)')
  })
  it('核心：达标令牌在 dark.css 双主题均存在且经对比度契约钉死', () => {
    expect(darkCss).toContain('--border-subtle: rgba(0, 212, 255, 0.5)')
    expect(darkCss).toContain('--border-subtle: rgba(0, 0, 0, 0.44)')
    expect(darkCss).toContain('nonTextContrast.contract.test.ts')
  })
  it('守卫：装饰性表单容器 L816 仍用 --ai-glass-border，三控件其他声明零改', () => {
    expect(lineWith('.ai-page .cal-create-form {')).toContain('border: 1px solid var(--ai-glass-border)')
    const input = lineWith('ai-page .cal-create-color {')
    expect(input).toContain('border-radius: var(--ai-grid-1)')
    expect(input).toContain('padding: 6px 10px')
    expect(input).toContain('background: var(--ai-glass-bg)')
    expect(input).toContain('color: var(--text-primary)')
    const cancel = lineWith('.ai-page .cal-create-cancel {')
    expect(cancel).toContain('border-radius: var(--ai-grid-1)')
    expect(cancel).toContain('padding: 7px 12px')
    expect(cancel).toContain('background: transparent')
    expect(cancel).toContain('color: var(--text-secondary)')
  })
  it('守卫：模板结构/事件/数据绑定/data-testid/业务处理器零改', () => {
    expect(source).toContain('data-testid="calendar-create-entry"')
    expect(source).toContain('@click="calCreateOpen = !calCreateOpen"')
    expect(source).toContain('data-testid="calendar-create-form"')
    expect(source).toContain('@submit.prevent="calWriteCE(' + String.fromCharCode(39) + 'calCreate' + String.fromCharCode(39) + ')"')
    expect(source).toContain('v-model="calCreateForm.name"')
    expect(source).toContain('v-model="calCreateForm.color"')
    expect(source).toContain('@click="calCreateOpen = false"')
  })
  it('守卫：全局 token 文件未改 --ai-glass-border 原值', () => {
    expect(aiTokensCss).toContain('--ai-glass-border: rgba(148, 163, 184, 0.16)')
    expect(aiTokensCss).toContain('--ai-glass-border: rgba(15, 23, 42, 0.1)')
  })
})
