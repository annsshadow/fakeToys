// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R222 字形导航按钮可访问名契约（WCAG 4.1.2 Name/Role/Value、2.4.6 Headings and Labels）。
 *
 * 日历上/下月与论坛/考勤/查询分页使用纯字形按钮：
 *   <button class="nav-btn" @click="prevMonth">‹</button>
 *   <button class="pgb" @click="page--">‹</button>
 * 读屏器只会念「左尖括号 / 按钮」，用户无法得知这是「上一月」还是「上一页」。
 * 这些关键导航按钮必须显式提供可读的 aria-label。源码级契约扫描按钮起始标签，
 * 钉住 click 处理器到可读名称的对应关系，防止后续改动回退成零名称字形按钮。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const viewsDir = path.join(path.dirname(fileURLToPath(import.meta.url)), '../views')
function readView(file: string): string {
  return readFileSync(path.join(viewsDir, file), 'utf8')
}

/** 取含指定 @click 处理器的 <button>...</button> 全文。 */
function glyphButton(source: string, handler: string): string {
  const hook = `@click="${handler}"`
  const idx = source.indexOf(hook)
  expect(idx, `未找到 ${hook}`).toBeGreaterThan(-1)
  const start = source.lastIndexOf('<button', idx)
  expect(start, `${handler} 前应存在 <button`).toBeGreaterThan(-1)
  const end = source.indexOf('</button>', idx)
  expect(end, `${handler} 所在 button 未闭合`).toBeGreaterThan(idx)
  return source.slice(start, end + '</button>'.length)
}

interface Case {
  file: string
  handler: string
  glyph: string
  label: string
}
const GLYPH_CASES: Case[] = [
  { file: 'CalendarApp.vue', handler: 'prevMonth', glyph: '‹', label: '上一月' },
  { file: 'CalendarApp.vue', handler: 'nextMonth', glyph: '›', label: '下一月' },
  { file: 'AttendanceApp.vue', handler: 'page--', glyph: '‹', label: '上一页' },
  { file: 'AttendanceApp.vue', handler: 'page++', glyph: '›', label: '下一页' },
  { file: 'BBSForum.vue', handler: 'page--', glyph: '‹', label: '上一页' },
  { file: 'BBSForum.vue', handler: 'page++', glyph: '›', label: '下一页' },
  { file: 'QueryStatementDesigner.vue', handler: 'page--', glyph: '‹', label: '上一页' },
  { file: 'QueryStatementDesigner.vue', handler: 'page++', glyph: '›', label: '下一页' },
]

describe('字形导航按钮可访问名（R222 / WCAG 4.1.2、2.4.6）', () => {
  it.each(GLYPH_CASES)('$file 的 $handler 字形按钮提供可读 aria-label', ({ file, handler, glyph, label }) => {
    const btn = glyphButton(readView(file), handler)
    // 该按钮确实是字形按钮（内容只有尖括号），不是已有文本的按钮。
    expect(btn, `${file} ${handler}`).toContain(`>${glyph}</button>`)
    // 关键：必须有可读的 aria-label（读屏器据此命名，而非念出尖括号）。
    expect(btn, `${file} ${handler} 缺少可访问名`).toContain(`aria-label="${label}"`)
  })
})
