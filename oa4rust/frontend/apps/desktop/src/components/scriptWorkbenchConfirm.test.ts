// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const NL = String.fromCharCode(10)
const source = readFileSync(resolve(import.meta.dirname, 'ScriptWorkbench.vue'), 'utf8')
const script = source.slice(source.indexOf('<script setup'), source.indexOf('</script>'))

/** Return the source of a locally declared confirmMsg wrapper, if any. */
function localConfirmSource(): string | null {
  const marker = 'function confirmMsg'
  const at = script.indexOf(marker)
  if (at === -1) return null
  const end = script.indexOf(`${NL}}`, at)
  expect(end, 'unclosed local confirmMsg').toBeGreaterThan(at)
  return script.slice(at, end + 2)
}

/**
 * Resolve the confirmMsg binding that the destructive delete action really
 * uses. A local wrapper is evaluated with no window, reproducing the
 * non-browser shell path: it returns a plain boolean. When no local wrapper
 * exists, confirmMsg must be imported from ../utils/toast, whose accessible
 * dialog is asynchronous and is represented here by a Promise.
 */
function resolveConfirmationBinding(): (message: string) => unknown {
  const local = localConfirmSource()
  if (local) {
    const js = local.replace(': string', '').replace(': boolean', '')
    return new Function('window', `${js}; return confirmMsg`)(undefined) as (message: string) => unknown
  }
  const importAt = script.indexOf("from '../utils/toast'")
  expect(importAt, 'confirmMsg must come from the accessible ../utils/toast helper').toBeGreaterThan(-1)
  const lineStart = script.lastIndexOf(NL, importAt)
  const importLine = script.slice(lineStart, script.indexOf(NL, importAt))
  expect(importLine, 'confirmMsg is not imported from ../utils/toast').toContain('confirmMsg')
  return () => Promise.resolve(false)
}

/**
 * R199: the shared script designer workbench is routed at three paths
 * (CMS/portal/process). Its delete action used a local native window.confirm
 * wrapper, shadowing the project's accessible async dialog. Native confirm has
 * no role=dialog / focus trap / Esc handling / focus restore, and silently
 * returns false outside a window shell, so the delete never runs there.
 */
describe('ScriptWorkbench delete uses accessible async confirmation (R199)', () => {
  it('the confirmation binding is async (Promise), not a native boolean confirm', () => {
    const result = resolveConfirmationBinding()('确定删除该脚本？')
    expect(
      result,
      'accessible dialog is async so it can manage focus and keyboard; native window.confirm returns a boolean',
    ).toBeInstanceOf(Promise)
  })

  it('defines no local native confirm wrapper', () => {
    expect(localConfirmSource(), 'no local confirmMsg wrapper allowed').toBeNull()
    expect(script.includes('window.confirm'), 'no window.confirm allowed').toBe(false)
  })
})
