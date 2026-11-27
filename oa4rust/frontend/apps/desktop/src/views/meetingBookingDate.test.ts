// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R610b contract: MeetingApp bookingDate default must use the UTC+8 business-day helper,
// so during 00:00-08:00 Beijing time it shows business today, not UTC yesterday.
// Father: ref(new Date().toISOString().slice(0, 10)) -> 2025-12-31 at 2025-12-31T16:30Z (RED).
// Fix: ref(businessTodayYMD()) -> 2026-01-01 (GREEN).

import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { businessTodayYMD } from '../utils/date'

const here = path.dirname(fileURLToPath(import.meta.url))
const src = readFileSync(path.join(here, 'MeetingApp.vue'), 'utf8')

// Extract the bookingDate initializer expression from the component source.
function bookingDateInitializer(): string {
  const m = src.match(/const\s+bookingDate\s*=\s*ref\((.+)\)/)
  return m ? m[1].trim() : ''
}
const initExpr = bookingDateInitializer()

// Evaluate the real initializer at a fixed system time; only businessTodayYMD is injected.
function evalInitializerAt(iso: string): string {
  vi.useFakeTimers()
  vi.setSystemTime(new Date(iso))
  return new Function('businessTodayYMD', `return (${initExpr})`)(businessTodayYMD)
}

afterEach(() => vi.useRealTimers())

describe('R610b MeetingApp bookingDate UTC+8 business day', () => {
  it('captures the bookingDate initializer from source', () => {
    expect(initExpr.length).toBeGreaterThan(0)
  })

  it('early morning Beijing 00:30 shows business today, not UTC yesterday (RED on father)', () => {
    expect(evalInitializerAt('2025-12-31T16:30:00Z')).toBe('2026-01-01')
  })

  it('cross-month boundary: Beijing 2026-02-01 01:00', () => {
    expect(evalInitializerAt('2026-01-31T17:00:00Z')).toBe('2026-02-01')
  })

  it('normal UTC+8 daytime 10:00 stays the same day', () => {
    expect(evalInitializerAt('2026-06-15T02:00:00Z')).toBe('2026-06-15')
  })

  it('source guard: initializer uses businessTodayYMD, not toISOString.slice', () => {
    expect(initExpr).toContain('businessTodayYMD()')
    expect(initExpr).not.toContain('toISOString')
  })
})
