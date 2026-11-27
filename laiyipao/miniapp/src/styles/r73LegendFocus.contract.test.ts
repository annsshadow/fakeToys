// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R73 战场图例弹层焦点管理与键盘闭环契约（test-first RED）。
 * legend-overlay 必须有 dialog 语义、ESC/Tab 守卫；help-chip 与 legend-close 必须键盘可达；
 * openHelp/closeHelp 必须有焦点移入/恢复；暂停恢复与互斥不变量、令牌锚点不得回退。
 * 真实读取 battle.vue / tokens.css，禁止内嵌实现副本。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

const NL = String.fromCharCode(10)

function blockAfter(text: string, keyword: string): { start: number; end: number; text: string } | null {
  const k = text.indexOf(keyword)
  if (k < 0) return null
  const open = text.indexOf('{', k)
  if (open < 0) return null
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch = text[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return { start: open, end: i, text: text.slice(open, i + 1) }
    }
  }
  return null
}

// 定位含指定类的 view 开标签块：从该行向上找最近的 <view，向下找开标签的 >
function viewTagBlock(text: string, classToken: string): string {
  const lines = text.split(NL)
  const idx = lines.findIndex(l => l.indexOf(classToken) > -1)
  if (idx < 0) return ''
  let start = idx
  while (start > 0 && lines[start].indexOf('<view') < 0) start--
  let end = idx
  while (end < lines.length && lines[end].indexOf('>') < 0) end++
  return lines.slice(start, end + 1).join(NL)
}

describe('R73 战场图例弹层焦点管理与键盘闭环契约', () => {
  const battle = readSrc('pages/battle/battle.vue')
  const tokens = readSrc('styles/tokens.css')

  it('it1 legend-overlay 含 role=dialog 与 aria-modal=true', () => {
    const overlay = battle.split(NL).find(l => l.indexOf('class="legend-overlay"') > -1)
    expect(overlay, '必须存在图例遮罩 legend-overlay').toBeTruthy()
    expect(overlay, 'legend-overlay 必须有 dialog 语义').toContain('role="dialog"')
    expect(overlay, 'legend-overlay 必须声明 aria-modal="true"').toContain('aria-modal="true"')
  })

  it('it2 help-chip 与 legend-close 键盘可达且复用 @click', () => {
    const chip = viewTagBlock(battle, 'class="help-chip"')
    const close = viewTagBlock(battle, 'class="legend-close"')
    expect(chip, '必须存在 help-chip').not.toBe('')
    expect(close, '必须存在 legend-close').not.toBe('')
    for (const [tag, handler, label] of [[chip, 'openHelp', 'help-chip'], [close, 'closeHelp', 'legend-close']] as const) {
      expect(tag, label + ' 必须有 role="button"').toContain('role="button"')
      expect(tag, label + ' 必须有 tabindex="0"').toContain('tabindex="0"')
      expect(tag, label + ' 必须有 @click 复用').toContain('@click="' + handler + '"')
      expect(tag, label + ' 必须有 @keydown.enter 复用 @click').toContain('@keydown.enter="' + handler + '"')
    }
  })

  it('it3 openHelp 焦点移入(nextTick+focus)且 closeHelp 焦点恢复(focus)', () => {
    const open = blockAfter(battle, 'function openHelp')
    const close = blockAfter(battle, 'function closeHelp')
    expect(open, 'openHelp 必须存在').not.toBeNull()
    expect(close, 'closeHelp 必须存在').not.toBeNull()
    expect(open!.text, 'openHelp 必须 nextTick 后移入焦点').toContain('nextTick')
    expect(open!.text, 'openHelp 必须调用 focus').toMatch(/focus\s*\(/)
    expect(close!.text, 'closeHelp 必须恢复焦点').toMatch(/focus\s*\(/)
  })

  it('it4 ESC 关闭守卫与 Tab 焦点陷阱守卫存在', () => {
    expect(battle, '必须有 ESC 关闭守卫(@keydown.esc 或 Escape 判断)').toMatch(/@keydown\.esc|['"]Escape['"]/)
    expect(battle, '必须有 Tab 焦点陷阱守卫(@keydown.tab 或 Tab/keyCode 或 trapFocus)').toMatch(/@keydown\.tab|['"]Tab['"]|keyCode\s*===?\s*9|trapFocus|focusTrap/)
  })

  it('it5 不变量：暂停恢复/互斥/令牌三锚点保留', () => {
    expect(battle, 'doPause 必须保留').toContain('doPause')
    expect(battle, 'doResume 必须保留').toContain('doResume')
    expect(battle, 'helpPausedByPage 记账必须保留').toContain('helpPausedByPage')
    const open = blockAfter(battle, 'function openHelp')
    expect(open, 'openHelp 必须存在').not.toBeNull()
    expect(open!.text, '图例与 R29 抽屉互斥必须保留').toContain('closeReactionTimeline()')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
  })

  it('it6 业务源码零 r73 视觉令牌', () => {
    expect(battle + tokens, '不得新增 --r73- 视觉令牌').not.toContain('--r73-')
  })
})
