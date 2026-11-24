// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R460 AdminLayout 共享页标题渐变契约：
 * .admin-title.ai-gradient-text 规则块内 color: transparent 必须生效，
 * 不得被同块后置 color: var(--text-primary) 覆盖；其它规则块的文本色不受影响。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AdminLayout.vue'), 'utf8')

function ruleBody(src: string, selector: string): string {
  const start = src.indexOf(selector + ' {')
  if (start < 0) return ''
  const open = src.indexOf('{', start)
  if (open < 0) return ''
  const close = src.indexOf('}', open)
  if (close < 0) return ''
  return src.slice(open + 1, close)
}

describe('R460 AdminLayout 页标题渐变渲染契约', () => {
  const titleBody = ruleBody(source, '.admin-title.ai-gradient-text')

  it('标题规则块保留透明文字与渐变背景声明', () => {
    expect(titleBody).not.toBe('')
    expect(titleBody).toContain('color: transparent')
    expect(titleBody).toContain('background: var(--ai-gradient-text)')
    expect(titleBody).toMatch(/background-clip:\s*text/)
  })

  it('同一规则块内不得有后置文本色覆盖 transparent', () => {
    expect(titleBody).not.toMatch(/color:\s*var\(--text-primary\)/)
  })

  it('模板 h1 仍携带 admin-title 与 ai-gradient-text 类', () => {
    expect(source).toMatch(/<h1[^>]*class="[^"]*admin-title[^"]*ai-gradient-text[^"]*"/)
  })

  it('其它导航规则块的 text-primary 文本色不受影响', () => {
    expect(ruleBody(source, '.admin-sidebar-header')).toContain('color: var(--text-primary)')
    expect(ruleBody(source, '.admin-nav-item:hover')).toContain('color: var(--text-primary)')
    expect(ruleBody(source, '.admin-nav-item.active')).toContain('color: var(--text-primary)')
  })
})
