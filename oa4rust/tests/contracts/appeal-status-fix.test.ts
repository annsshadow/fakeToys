// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R597b contract test: appeal status domain alignment (RED -> GREEN).
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const SRC = resolve(import.meta.dirname, '../../frontend/apps/desktop/src/views/AttendanceApp.vue')
const src = readFileSync(SRC, 'utf8')

function extract(name: string): (s?: string) => string {
  const i = src.indexOf('function ' + name + '(')
  if (i < 0) throw new Error('missing function ' + name)
  const start = src.indexOf('{', i)
  let depth = 0
  let end = -1
  for (let j = start; j < src.length; j++) {
    const c = src[j]
    if (c === '{') depth++
    else if (c === '}') {
      depth--
      if (depth === 0) { end = j; break }
    }
  }
  return new Function('s', src.slice(start + 1, end)) as (s?: string) => string
}

const appealStatus = extract('appealStatus')
const appealClass = extract('appealClass')

describe('R597b appeal status domain', () => {
  it('maps all four values including appealed', () => {
    expect(appealStatus('appealed')).toBe('待审核')
    expect(appealClass('appealed')).toBe('pending')
    expect(appealStatus('approved')).toBe('已通过')
    expect(appealClass('approved')).toBe('approved')
    expect(appealStatus('rejected')).toBe('已驳回')
    expect(appealClass('rejected')).toBe('rejected')
    expect(appealStatus('pending')).toBe('待审批')
    expect(appealClass('pending')).toBe('pending')
  })

  it('approval gate covers both pending and appealed', () => {
    const NL = String.fromCharCode(10)
    const gateLine = src.split(NL).find((l) => l.includes('v-if=') && l.includes('a.status') && l.includes('class="aa"')) || ''
    expect(gateLine).toContain('pending')
    expect(gateLine).toContain('appealed')
  })
})
