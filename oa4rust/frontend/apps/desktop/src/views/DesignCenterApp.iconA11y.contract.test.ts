// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R214 设计中心纯图标按钮可访问名称契约（WCAG 2.1 AA / 4.1.2 Name, Role, Value）。
 *
 * 模板里 <button>✕</button> 的可访问名只有字形本身，读屏器念出「乘号」而不知是
 * 「关闭统计」还是「关闭清单」；搜索框只有 placeholder，也不能作为可靠可访问名。
 * 本测试读取真实模板源码防止回归。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
const source = readFileSync(path.join(frontendRoot, 'apps/desktop/src/views/DesignCenterApp.vue'), 'utf8')

/** 找出行内只含符号（无字母/数字）的按钮，返回源码行与完整标签。 */
function glyphOnlyButtons(src: string): { line: number; tag: string; text: string }[] {
  const found: { line: number; tag: string; text: string }[] = []
  src.split('\n').forEach((lineText, index) => {
    const match = lineText.match(/<button\b[^>]*>([\s\S]*?)<\/button>/)
    if (!match) return
    const text = match[1].replace(/<[^>]+>/g, '').trim()
    if (!text || /[\p{L}\p{N}]/u.test(text)) return
    found.push({ line: index + 1, tag: match[0], text })
  })
  return found
}

describe('设计中心图标按钮可访问名称（WCAG 2.1 AA / 4.1.2）', () => {
  it('纯图标按钮必须声明 aria-label 或 title', () => {
    const offenders = glyphOnlyButtons(source).filter(
      (button) => !/aria-label=|aria-labelledby=|title=/.test(button.tag),
    )
    expect(
      offenders,
      `以下纯图标按钮只有字形，读屏器无法得知用途：\n${offenders
        .map((button) => `L${button.line}: ${button.tag}`)
        .join('\n')}`,
    ).toEqual([])
  })

  it('设计器搜索框必须声明 aria-label', () => {
    const tag = source.split('\n').find((line) => line.includes('class="search-input"')) ?? ''
    expect(tag, '搜索框只有 placeholder，屏幕阅读器无法得知搜索目标').toMatch(/aria-label=/)
  })
})
