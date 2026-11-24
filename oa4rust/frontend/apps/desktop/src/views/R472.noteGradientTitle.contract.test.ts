// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R472 NoteApp 标题渐变渲染契约：
 * .view-header h1 规则块不得用实心 color 覆盖 .ai-gradient-text 的
 * color: transparent，否则渐变背景被遮蔽。真父版必 RED、修复版必 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'NoteApp.vue'), 'utf8')

/** 精确取选择器规则体（源码选择器后直接跟 {） */
function ruleBody(src: string, selector: string): string {
  const at = src.indexOf(selector + '{')
  if (at < 0) return ''
  const open = src.indexOf('{', at)
  const close = src.indexOf('}', open)
  if (open < 0 || close < 0) return ''
  return src.slice(open + 1, close)
}

describe('R472 NoteApp 页标题渐变渲染契约', () => {
  const titleBody = ruleBody(source, '.view-header h1')
  const gradientBody = ruleBody(source, '.ai-gradient-text')

  it('标题 h1 模板仍携带 ai-gradient-text 类', () => {
    expect(source).toMatch(/<h1[^>]*class="[^"]*ai-gradient-text[^"]*"/)
    expect(source).toContain('笔记管理')
  })

  it('.view-header h1 不得用实心 color 覆盖渐变 transparent（核心缺陷断言）', () => {
    expect(titleBody, '必须保留 .view-header h1 规则块').not.toBe('')
    expect(titleBody, '不得再有 color:var(--color-primary) 遮蔽渐变').not.toMatch(/color:\s*var\(--color-primary\)/)
  })

  it('渐变声明块保留 background 与 background-clip:text', () => {
    expect(gradientBody, '必须保留 .ai-gradient-text 规则块').not.toBe('')
    expect(gradientBody).toMatch(/background:\s*var\(--ai-gradient-text\)/)
    expect(gradientBody).toMatch(/background-clip:\s*text/)
    expect(gradientBody).toMatch(/color:\s*transparent/)
  })

  it('标题字体字号、间距与 text-shadow 零改', () => {
    expect(titleBody).toContain('font-family')
    expect(titleBody).toMatch(/font-size:\s*20px/)
    expect(titleBody).toMatch(/margin:\s*0 0 4px/)
    expect(titleBody).toMatch(/text-shadow:\s*0 0 15px var\(--color-primary-glow\)/)
  })

  it('按钮样式零改：.btn-primary 仍为原色值且渐变品牌样式保留', () => {
    const btn = ruleBody(source, '.btn-primary')
    expect(btn).toMatch(/background:\s*var\(--color-primary\)/)
    expect(source).toContain('.btn-primary,.btn-save{background: var(--ai-gradient-brand)')
  })
})
