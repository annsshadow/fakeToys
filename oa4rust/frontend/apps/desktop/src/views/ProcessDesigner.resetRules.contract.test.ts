// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R607b contract: the "重置默认" button must bind to a handler defined in the script.
// Father binds resetRules() which is never defined -> ReferenceError at click (RED).
// Fix rebinds to the existing resetConnectionRules() implementation -> GREEN.

import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const src = readFileSync(path.join(here, 'ProcessDesigner.vue'), 'utf8')

// Only the script section counts as a definition site; a template-only name cannot pass.
const scriptStart = src.indexOf('<script')
const scriptEnd = src.lastIndexOf('</script>')
const script = scriptStart >= 0 && scriptEnd > scriptStart ? src.slice(scriptStart, scriptEnd) : ''

function resetButtonClickExpr(): string {
  const anchor = '>重置默认</button>'
  const btn = src.indexOf(anchor)
  if (btn < 0) return ''
  const start = src.lastIndexOf('<button', btn)
  if (start < 0) return ''
  const seg = src.slice(start, btn)
  const c0 = seg.indexOf('@click="')
  if (c0 < 0) return ''
  const c1 = seg.indexOf('"', c0 + 8)
  return c1 > c0 ? seg.slice(c0 + 8, c1).trim() : ''
}

const clickExpr = resetButtonClickExpr()
const handlerName = clickExpr.indexOf('(') > 0 ? clickExpr.slice(0, clickExpr.indexOf('(')).trim() : ''

function hasDefinition(name: string): boolean {
  if (!name) return false
  return (
    script.indexOf('function ' + name + '(') >= 0 ||
    script.indexOf('const ' + name + ' =') >= 0 ||
    script.indexOf('let ' + name + ' =') >= 0 ||
    script.indexOf('var ' + name + ' =') >= 0
  )
}

describe('R607b ProcessDesigner reset button binding', () => {
  it('captures the reset button click target from source', () => {
    expect(src.indexOf('>重置默认</button>')).toBeGreaterThan(0)
    expect(clickExpr.length).toBeGreaterThan(0)
    expect(handlerName.length).toBeGreaterThan(0)
  })

  it('bound handler is defined in script (RED on father: resetRules undefined)', () => {
    // Father: handlerName = resetRules, script has no such definition -> fails below.
    expect(handlerName).not.toBe('resetRules')
    expect(hasDefinition(handlerName)).toBe(true)
  })

  it('rebinds to the existing resetConnectionRules implementation', () => {
    expect(handlerName).toBe('resetConnectionRules')
  })

  it('sibling rule helpers, default-rule body and modal anchors are intact', () => {
    const anchors = [
      'function resetConnectionRules(',
      'function isAllowed(',
      'function toggleRule(',
      'function saveRules(',
      'connectionRules.value = rules',
      'const showRulesModal = ref(false)',
      'showRulesModal.value = false',
    ]
    for (const a of anchors) expect(script).toContain(a)
    expect(src).toContain('class="rr-actions"')
    expect(src).toContain('💾 保存规则')
    expect(src).toContain('🔗 规则')
  })
})
