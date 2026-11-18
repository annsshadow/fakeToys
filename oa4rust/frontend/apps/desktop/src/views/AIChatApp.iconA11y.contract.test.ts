// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R219 AIChat 图标按钮可访问名契约测试（WCAG 4.1.2 Name/Role/Value）。
 *
 * AIChatApp.vue 侧栏与聊天输入区用纯字形按钮执行破坏性或关键操作：
 *   - btn-del-sm  删对话（✕，零名称）
 *   - attach-x    移除已附文件（✕，零名称）
 *   - btn-config  打开 MCP 配置（⚙，仅 title）
 *   - btn-attach  上传附件（📎，仅 title）
 * 纯 ✕ 不是可读名称——读屏器只会念「x / 按钮」，用户无法区分「删除对话」与「移除附件」；
 * title 属弱兜底、多数读屏器不读。这些控件必须显式提供 aria-label。
 * 源码级契约扫描按钮开始标签，钉住 class → aria-label 的对应关系。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AIChatApp.vue'), 'utf8')

function buttonTagWithClass(cls: string): string {
  const re = new RegExp('<button\\b[^>]*class="[^"]*\\b' + cls + '\\b[^"]*"[^>]*>')
  const match = source.match(re)
  expect(match, `未找到 button.${cls}`).not.toBeNull()
  return match![0]
}

describe('AIChat 图标按钮可访问名（R219 / WCAG 4.1.2）', () => {
  it.each(['btn-del-sm', 'attach-x', 'btn-config', 'btn-attach'])('button.%s 显式提供 aria-label', (cls) => {
    expect(buttonTagWithClass(cls), `button.${cls} 缺少 aria-label`).toMatch(/\baria-label=/)
  })

  it('删除对话按钮的 aria-label 带上对话标题以区分列表项', () => {
    const tag = buttonTagWithClass('btn-del-sm')
    expect(tag).toMatch(/:aria-label=/)
    expect(tag).toMatch(/删除对话/)
  })
})
