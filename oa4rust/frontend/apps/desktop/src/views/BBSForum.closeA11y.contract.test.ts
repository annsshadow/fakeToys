// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R240 BBSForum 弹窗关闭按钮可访问名契约（WCAG 4.1.2 Name, Role, Value / Level A）。
 *
 * 缺陷证据：BBSForum.vue 中四个弹窗关闭按钮仅含字形 ✕，既无 aria-label 也无 title：
 *   <button class="close-btn" @click="showNewTopic = false">✕</button>
 *   <button class="close-btn" @click="viewingTopic = null">✕</button>
 *   <button class="close-btn" @click="closeSectionModal">✕</button>
 *   <button class="close-btn" @click="sectionInfo.open=false">✕</button>
 * 读屏用户只会听到「乘号 / 按钮」，无法得知关闭的是发帖弹窗、帖子详情、
 * 版块编辑弹窗还是版块详情——四个语义完全不同的控件被压成同一个无名按钮。
 * 同文件其余图标按钮（新增版块 ℹ ✎ ✕、分页 ‹›）都带 aria-label/title，
 * 唯独这四个关闭按钮是真实的名称缺失缺口。
 *
 * 现有增强器不覆盖此缺口：labelA11y 只处理 <label> 与表单控件，
 * formErrorA11y 只关联字段错误，modalFocus 只补弹窗标题名与焦点陷阱，
 * glyphButtonA11y 契约只钉住导航尖括号按钮，FileManager.closeA11y 只覆盖 FileManager.vue——
 * 均不处理 BBSForum 的这四个 ✕。
 *
 * 契约要求：所有字形 ✕ 关闭按钮必须提供可读、语义明确的 aria-label。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const source = readFileSync(path.join(here, 'BBSForum.vue'), 'utf8')

/** 取出含指定 @click 处理器的 <button>...</button> 全文。 */
function closeButton(handler: string): string {
  const hook = `@click="${handler}"`
  const idx = source.indexOf(hook)
  expect(idx, `未找到 ${hook}`).toBeGreaterThan(-1)
  const start = source.lastIndexOf('<button', idx)
  expect(start, `${handler} 前应存在 <button`).toBeGreaterThan(-1)
  const end = source.indexOf('</button>', idx)
  expect(end, `${handler} 所在 button 未闭合`).toBeGreaterThan(idx)
  return source.slice(start, end + '</button>'.length)
}

/** 所有内容仅为 ✕ 的字形关闭按钮（开标签 + 内容 + 闭标签）。 */
function glyphCloseButtons(): string[] {
  return Array.from(source.matchAll(/<button\b[^>]*>[\s\S]*?<\/button>/g))
    .map((m) => m[0])
    .filter((btn) => />\s*✕\s*<\/button>/.test(btn))
}

interface Case {
  handler: string
  label: string
}
const CLOSE_CASES: Case[] = [
  { handler: 'showNewTopic = false', label: '关闭发帖弹窗' },
  { handler: 'closeTopic()', label: '关闭帖子详情' },
  { handler: 'closeSectionModal', label: '关闭版块编辑弹窗' },
  { handler: 'sectionInfo.open=false', label: '关闭版块详情' },
]

describe('BBSForum 关闭按钮可访问名（R240 / WCAG 4.1.2）', () => {
  it('存在至少四个字形 ✕ 关闭按钮（防止契约因模板改动而空跑）', () => {
    expect(glyphCloseButtons().length).toBeGreaterThanOrEqual(4)
  })

  it.each(CLOSE_CASES)('$handler 关闭按钮提供可读 aria-label', ({ handler, label }) => {
    const btn = closeButton(handler)
    expect(btn, `${handler} 按钮应为字形 ✕`).toMatch(/>\s*✕\s*<\/button>/)
    const aria = btn.match(/aria-label\s*=\s*"([^"]+)"/)?.[1]
    expect(aria, `${handler} 关闭按钮缺少 aria-label：读屏用户只会遇到「乘号按钮」`).toBeTruthy()
    expect(aria!.trim(), 'aria-label 不能为空').not.toBe('')
    expect(aria, 'aria-label 必须是可读文字，不能是字形本身').toMatch(/[\u4e00-\u9fffA-Za-z]/)
    expect(aria, `aria-label 应表达「${label}」的用途`).toContain(label)
  })

  it('所有字形 ✕ 关闭按钮都不得只有字形而无名称', () => {
    const buttons = glyphCloseButtons()
    for (const btn of buttons) {
      const openTag = btn.slice(0, btn.indexOf('>'))
      expect(openTag, `关闭按钮缺少可访问名: ${btn}`).toMatch(/\baria-label="[^"]+"/)
    }
  })
})
