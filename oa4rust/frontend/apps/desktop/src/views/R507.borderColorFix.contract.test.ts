// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R507 SQL语句设计器 --border-color 变量缺失修复契约（WCAG 2.1 AA 1.4.11）：
 * QSD scoped 域内须新增 --border-color: var(--border-subtle) 定义（挂组件根/浮层根，使参数与可视化浮层控件继承），
 * 令原 border:1px solid var(--border-color) 声明生效；模板/事件/业务/全局 token 零改。真父 RED、修复版 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryStatementDesigner.vue'), 'utf8')
const darkCss = readFileSync(resolve(import.meta.dirname, '../../../../packages/ui/src/theme/dark.css'), 'utf8')
const aiTokensCss = readFileSync(resolve(import.meta.dirname, '../../../../packages/ui/src/theme/ai-tokens.css'), 'utf8')

const styleBody = source.slice(source.indexOf('<style scoped>'))

describe('R507 QSD --border-color 变量定义契约', () => {
  it('核心：scoped 域内新增 --border-color 定义并引用达标令牌 --border-subtle', () => {
    expect(styleBody).toMatch(/--border-color\s*:\s*var\(--border-subtle\)/)
  })
  it('核心：定义挂在组件根或浮层根选择器上，使参数/可视化控件继承', () => {
    expect(styleBody).toMatch(/\.(?:mod-view|modal-overlay)[^{}]*\{[^}]*--border-color\s*:\s*var\(--border-subtle\)/)
  })
  it('守卫：原 border:1px solid var(--border-color) 声明保留（参数与可视化）', () => {
    expect(source).toContain('border:1px solid var(--border-color)')
    expect(source).toContain('.param-input{')
    expect(source).toContain('.ve-input-sm{')
  })
  it('守卫：模板事件/数据绑定/data-testid/业务处理器零改', () => {
    expect(source).toContain('@click="newStatement"')
    expect(source).toContain('v-if="showParamPanel"')
    expect(source).toContain('v-if="showVisualEditor"')
    expect(source).toContain('@click.self="showParamPanel=false"')
    expect(source).toContain('function selectStatement(s: Stmt) {')
  })
  it('守卫：全局 token 文件未新增 --border-color 定义', () => {
    expect(darkCss).not.toMatch(/--border-color\s*:/)
    expect(aiTokensCss).not.toMatch(/--border-color\s*:/)
  })
})