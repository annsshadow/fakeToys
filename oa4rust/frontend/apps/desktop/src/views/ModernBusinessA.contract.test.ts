// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * ModernBusinessA contract: directory / file / personal pages consume shared ModernUI
 * classes (hero -> section -> card grid) while keeping every existing data-testid.
 * Structural assertions only; CSS lives in components/admin/ModernUI.vue.
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const views = resolve(import.meta.dirname, '.')
const read = (file: string) => readFileSync(resolve(views, file), 'utf8')

const org = read('OrgDirectoryApp.vue')
const file = read('FileManager.vue')
const personal = read('Personal.vue')

/** Only the <template> section, so script api paths do not count. */
function template(src: string): string {
  const start = src.indexOf('<template>')
  const end = src.indexOf('</template>', start)
  expect(start, 'missing <template>').toBeGreaterThanOrEqual(0)
  return src.slice(start, end)
}

const testids = (src: string): string[] => Array.from(src.matchAll(/data-testid="([^"]+)"/g), (m) => m[1])

describe('ModernBusinessA three-page template structure contract', () => {
  describe('shared ModernUI classes (hero -> section -> grid)', () => {
    it.each([
      ['directory', org],
      ['files', file],
      ['personal', personal],
    ])('%s uses .modern-page root class', (label, src) => {
      const tpl = template(src)
      expect(tpl, `${label} missing modern-page root`).toMatch(/class="[^"]*\bmodern-page\b/)
    })

    it.each([
      ['directory', org],
      ['files', file],
      ['personal', personal],
    ])('%s uses .modern-hero for the title block', (label, src) => {
      const tpl = template(src)
      expect(tpl, `${label} missing modern-hero`).toMatch(/class="[^"]*\bmodern-hero\b/)
    })

    it.each([
      ['directory', org],
      ['files', file],
      ['personal', personal],
    ])('%s uses .modern-section + .modern-grid/.modern-card', (label, src) => {
      const tpl = template(src)
      expect(tpl, `${label} missing modern-section`).toMatch(/class="[^"]*\bmodern-section\b/)
      expect(tpl, `${label} missing modern-grid/card`).toMatch(/class="[^"]*\bmodern-(?:grid|card)\b/)
    })

    it('each page keeps exactly one h1 title', () => {
      for (const [label, src] of [
        ['directory', org],
        ['files', file],
        ['personal', personal],
      ] as const) {
        const h1 = template(src).match(/<h1\b/g) ?? []
        expect(h1.length, `${label} h1 count should be 1, got ${h1.length}`).toBe(1)
      }
    })
  })

  describe('legacy data-testid preserved', () => {
    it('directory keeps key testids', () => {
      const ids = testids(template(org))
      for (const id of [
        'org-directory-business-view',
        'org-directory-header',
        'org-directory-subtitle',
        'org-search-section',
        'org-search-form',
        'org-search-input',
        'org-search-button',
        'org-tree-section',
        'org-department-tree',
        'org-department-node',
        'org-person-section',
        'org-person-reload',
        'org-person-meta',
        'org-person-list',
        'org-person-empty',
        'org-person-pager',
      ]) {
        expect(ids, `directory missing data-testid=${id}`).toContain(id)
      }
    })

    it('files keeps list / shared / recycle testids', () => {
      const ids = testids(template(file))
      for (const id of ['files-mine-list', 'files-shared-list', 'files-recycle-list']) {
        expect(ids, `files missing data-testid=${id}`).toContain(id)
      }
    })

    it('personal keeps key testids', () => {
      const ids = testids(template(personal))
      for (const id of [
        'personal-business-view',
        'personal-header',
        'personal-profile',
        'personal-avatar',
        'personal-password',
        'personal-signature',
        'personal-security',
        'personal-empower',
      ]) {
        expect(ids, `personal missing data-testid=${id}`).toContain(id)
      }
    })
  })

  describe('modern empty/error states and no /api/ exposure', () => {
    it('directory empty state uses modern-empty', () => {
      expect(template(org)).toMatch(/class="[^"]*\bmodern-empty\b/)
    })

    it('no page template exposes /api/ paths', () => {
      for (const [label, src] of [
        ['directory', org],
        ['files', file],
        ['personal', personal],
      ] as const) {
        expect(template(src), `${label} template exposes /api/ path`).not.toMatch(/\/api\//)
      }
    })
  })
})
