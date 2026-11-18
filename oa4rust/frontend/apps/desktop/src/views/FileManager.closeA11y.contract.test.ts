// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R239 文件引用面板 / 文件详情弹窗关闭按钮可访问名契约（WCAG 4.1.2 Name, Role, Value / Level A）。
 *
 * 缺陷证据：FileManager.vue 中两个关闭按钮仅含字形 ✕：
 *   <button class="icon-action" @click="refPanel.open = false">✕</button>
 *   <button class="icon-action" @click="detail.open = false">✕</button>
 * 读屏用户只会听到「乘号 / 按钮」，无法得知这是「关闭引用面板」还是「关闭文件详情」。
 * 同文件的其它图标按钮都带 title（详情/下载/分享/删除），唯独这两个关闭按钮
 * 既无 aria-label 也无 title，是真实的名称缺失缺口。
 *
 * 现有增强器不覆盖此缺口：main.ts 安装的 labelA11y 只处理 <label> 与表单控件，
 * formErrorA11y 只关联字段错误，modalFocus 只补弹窗标题名与焦点陷阱，
 * glyphButtonA11y 契约只钉住上一月/上一页等导航尖括号按钮——均不处理这两个 ✕。
 *
 * 契约要求：所有字形 ✕ 关闭按钮必须提供可读的 aria-label（非 emoji/字形本身）。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const source = readFileSync(path.join(here, 'FileManager.vue'), 'utf8')

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

describe('FileManager 关闭按钮可访问名（R239 / WCAG 4.1.2）', () => {
  it('存在至少两个字形 ✕ 关闭按钮（防止契约因模板改动而空跑）', () => {
    expect(glyphCloseButtons().length).toBeGreaterThanOrEqual(2)
  })

  it('引用面板关闭按钮提供可读 aria-label', () => {
    const btn = closeButton('refPanel.open = false')
    const label = btn.match(/aria-label\s*=\s*"([^"]+)"/)?.[1]
    expect(label, '引用面板关闭按钮缺少 aria-label：读屏用户只会遇到「乘号按钮」').toBeTruthy()
    expect(label!.trim(), 'aria-label 不能为空').not.toBe('')
    expect(label, 'aria-label 必须是可读文字，不能是字形本身').toMatch(/[\u4e00-\u9fffA-Za-z]/)
  })

  it('文件详情弹窗关闭按钮提供可读 aria-label', () => {
    const btn = closeButton('detail.open = false')
    const label = btn.match(/aria-label\s*=\s*"([^"]+)"/)?.[1]
    expect(label, '文件详情关闭按钮缺少 aria-label：读屏用户只会遇到「乘号按钮」').toBeTruthy()
    expect(label!.trim(), 'aria-label 不能为空').not.toBe('')
    expect(label, 'aria-label 必须是可读文字，不能是字形本身').toMatch(/[\u4e00-\u9fffA-Za-z]/)
  })

  it('所有字形 ✕ 关闭按钮都不得只有字形而无名称', () => {
    const buttons = glyphCloseButtons()
    for (const btn of buttons) {
      const openTag = btn.slice(0, btn.indexOf('>'))
      expect(openTag, `关闭按钮缺少可访问名: ${btn}`).toMatch(/\baria-label="[^"]+"/)
    }
  })
})
