// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R225 dynamic-form required semantics contract (WCAG 3.3.2 / 4.1.2).
 *
 * XformRuntime is the unified renderer for workflow forms. A required field
 * currently only shows a visual asterisk (<span class="required">*</span>);
 * its control carries neither `required` nor `aria-required`, so screen-reader
 * users cannot tell which fields are mandatory. This contract pins that every
 * control driven by module.required exposes the state, and that the decorative
 * asterisk is hidden from assistive technology.
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'XformRuntime.vue'), 'utf8')

/** From an anchor, take the full opening tag of the next matching element. */
function tagAfter(anchor: string, tag: string): string {
  const anchorIdx = source.indexOf(anchor)
  expect(anchorIdx, 'anchor missing: ' + anchor).toBeGreaterThan(-1)
  const start = source.lastIndexOf('<' + tag, anchorIdx)
  expect(start, 'opening tag missing: ' + tag).toBeGreaterThan(-1)
  const end = source.indexOf('>', start)
  expect(end).toBeGreaterThan(start)
  return source.slice(start, end + 1)
}

describe('XformRuntime required-field semantics (R225 / WCAG 3.3.2, 4.1.2)', () => {
  it('keeps the template anchors so the contract cannot silently no-op', () => {
    expect(source).toContain("kind(module) === 'textarea'")
    expect(source).toContain("kind(module) === 'select'")
    expect(source).toContain("kind(module) === 'checkboxes'")
    expect(source).toContain("kind(module) !== 'display'")
    expect(source).toContain('class="required"')
  })

  it('hides the decorative asterisk from screen readers', () => {
    const star = source.match(/<span v-if="module\.required" class="required"[^>]*>/)?.[0] ?? ''
    expect(star).toMatch(/aria-hidden="true"/)
  })

  it('drives textarea required state from module.required', () => {
    expect(tagAfter("kind(module) === 'textarea'", 'textarea')).toMatch(/:required="module\.required \|\| undefined"/)
  })

  it('drives select required state from module.required', () => {
    expect(tagAfter("kind(module) === 'select'", 'select')).toMatch(/:required="module\.required \|\| undefined"/)
  })

  it('drives plain input required state from module.required', () => {
    expect(tagAfter("kind(module) !== 'display'", 'input')).toMatch(/:required="module\.required \|\| undefined"/)
  })

  it('expresses a required checkbox group via role=group + aria-required', () => {
    const group = tagAfter("kind(module) === 'checkboxes'", 'div')
    expect(group).toMatch(/role="group"/)
    expect(group).toMatch(/:aria-required="module\.required \? 'true' : undefined"/)
  })
})
