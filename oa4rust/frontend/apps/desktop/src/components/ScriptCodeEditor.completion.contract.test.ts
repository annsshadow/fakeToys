// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R605b contract: xscriptCompletion must use context.matchBefore, not an undeclared word.

import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const NL = String.fromCharCode(10)
const here = path.dirname(fileURLToPath(import.meta.url))
const src = readFileSync(path.join(here, 'ScriptCodeEditor.vue'), 'utf8')

function sliceBlock(text: string, startTag: string, open: string, closeTag: string): string {
  const s = text.indexOf(startTag)
  if (s < 0) return ''
  const o = text.indexOf(open, s)
  if (o < 0) return ''
  const c = text.indexOf(closeTag, o)
  if (c < 0) return ''
  return text.slice(o, c + closeTag.length)
}

const arrText = sliceBlock(src, 'const XSCRIPT_COMPLETIONS = ', '[', `${NL}]`)
const fnBody = (() => {
  const f = src.indexOf('function xscriptCompletion(')
  if (f < 0) return ''
  const o = src.indexOf('{', f)
  const c = src.indexOf(`${NL}}`, o)
  if (o < 0 || c < 0) return ''
  return src.slice(o + 1, c)
})()

const COMPLETIONS: { label: string; type?: string; detail?: string; apply?: string }[] = arrText
  ? Function(`return ${arrText}`)()
  : []
// Inject only the captured body; a bare `word` reference throws at call time (RED on father).
const INVOKE = Function('XSCRIPT_COMPLETIONS', 'context', fnBody)

type Match = { from: number; text: string } | null
function ctx(match: Match) {
  return { matchBefore: () => match }
}
type Result = { from: number; options: unknown; validFor: RegExp } | null

const EXPECTED_LABELS = [
  'this',
  'workContext',
  'effectivePerson',
  'organization',
  'resources',
  'applications',
  'console',
  'JSON',
  'Promise',
  'include',
  'print',
  'getValue',
  'setValue',
  'require',
]

describe('R605b ScriptCodeEditor xscriptCompletion completion source', () => {
  it('source exposes XSCRIPT_COMPLETIONS and an extracted function body', () => {
    expect(arrText.length).toBeGreaterThan(0)
    expect(fnBody.length).toBeGreaterThan(0)
  })

  it('XSCRIPT_COMPLETIONS has exactly 14 items in the expected order', () => {
    expect(COMPLETIONS).toHaveLength(14)
    expect(COMPLETIONS.map((c) => c.label)).toEqual(EXPECTED_LABELS)
  })

  it('with a match returns from/options/validFor and never throws ReferenceError', () => {
    // RED on father: bare word throws ReferenceError before any assertion below.
    const result = INVOKE(COMPLETIONS, ctx({ from: 3, text: 'wo' })) as Result
    expect(result).not.toBeNull()
    if (!result) return
    expect(result.from).toBe(3)
    expect(result.options).toBe(COMPLETIONS)
    expect(result.validFor).toBeInstanceOf(RegExp)
    expect(result.validFor.test('workContext')).toBe(true)
    expect(result.validFor.test('a-b')).toBe(false)
  })

  it('with no match returns null', () => {
    expect(INVOKE(COMPLETIONS, ctx(null))).toBeNull()
  })
})
