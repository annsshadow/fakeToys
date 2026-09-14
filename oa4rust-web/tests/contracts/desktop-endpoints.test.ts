import { readdirSync, readFileSync, statSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { REGISTERED_BACKEND_ROUTES } from './backend-registered-routes.fixture'

/**
 * Desktop endpoint contract guard.
 *
 * Every /jaxrs literal in apps/desktop/src is OK when ANY holds:
 *   (a) it matches a route the oa4rust backend registers (REGISTERED_BACKEND_ROUTES),
 *   (b) it is an explicitly acknowledged backend parity gap (KNOWN_BACKEND_GAPS), or
 *   (c) it is a bare base prefix (trailing /) used by a dynamic request helper.
 * A NEW unregistered, non-bare endpoint fails this test, catching drift at CI time
 * instead of as a silent 404. KNOWN_BACKEND_GAPS entries are legacy o2web surfaces the
 * Rust port has not implemented; they are documented here, not fabricated.
 */

const KNOWN_BACKEND_GAPS: string[] = [
  '/jaxrs/ai/assemble/control/ann/list',
  '/jaxrs/attendance/assemble/control/rule/create',
  '/jaxrs/attendance/assemble/control/statistics/list',
  '/jaxrs/cms/assemble/control/dict/list',
  '/jaxrs/cms/assemble/control/form/list',
  '/jaxrs/cms/assemble/control/view/list',
  '/jaxrs/cms/assemble/control/xform/list',
  '/jaxrs/cms/core/entity/column/list',
  '/jaxrs/cms/core/entity/column_manager/list',
  '/jaxrs/cms/core/entity/index/list',
  '/jaxrs/cms/core/entity/module/list',
  '/jaxrs/cms/core/entity/note/list',
  '/jaxrs/config/create',
  '/jaxrs/ftsearch/list',
  '/jaxrs/general/assemble/control/list',
  '/jaxrs/message/unread/count',
  '/jaxrs/organization/assemble/control/threemember/list',
  '/jaxrs/person/signature/save',
  '/jaxrs/personal/face/list',
  '/jaxrs/portal/assemble/designer',
  '/jaxrs/processplatform/assemble/designer',
  '/jaxrs/processplatform/assemble/surface/process_manager/list',
  '/jaxrs/query/assemble/designer/delete',
  '/jaxrs/query/assemble/designer/list',
  '/jaxrs/query/assemble/designer/save',
  '/jaxrs/query/assemble/surface/explorer/list',
  '/jaxrs/server/deploy/list',
]

const desktopSrcRoot = resolve(import.meta.dirname, '../../apps/desktop/src')

function collectPaths(): string[] {
  const files: string[] = []
  function walk(dir: string): void {
    for (const name of readdirSync(dir)) {
      const full = resolve(dir, name)
      const st = statSync(full)
      if (st.isDirectory()) walk(full)
      else if (/\.(ts|vue)$/.test(name) && !name.endsWith('.test.ts')) files.push(full)
    }
  }
  walk(desktopSrcRoot)
  const paths = new Set<string>()
  for (const file of files) {
    const text = readFileSync(file, 'utf8')
    for (const m of text.matchAll(/\/jaxrs\/[A-Za-z0-9_\-/{}$.]+/g)) paths.add(m[0])
  }
  return [...paths]
}

function segsUsed(p: string): string[] {
  return p
    .toLowerCase()
    .split('/')
    .filter(Boolean)
    .map((s) => (s.includes('${') || s.startsWith('{') ? '*' : s))
}
function segsPattern(p: string): string[] {
  return p
    .toLowerCase()
    .split('/')
    .filter(Boolean)
    .map((s) => (s.startsWith('{') ? '*' : s))
}
const segm = (a: string, b: string) => a === '*' || b === '*' || a === b

function hitsRegistered(path: string, dyn: boolean): boolean {
  const c = segsUsed(path)
  for (const p of REGISTERED_BACKEND_ROUTES) {
    const r = segsPattern(p)
    if (r.length === c.length && r.every((s, i) => segm(s, c[i]!))) return true
    if (dyn && r.length > c.length && r.slice(0, c.length).every((s, i) => segm(s, c[i]!))) return true
  }
  return false
}

function isKnownGap(path: string): boolean {
  const c = segsUsed(path)
  return KNOWN_BACKEND_GAPS.some((g) => {
    const r = segsUsed(g)
    return r.length === c.length && r.every((s, i) => segm(s, c[i]!))
  })
}

describe('desktop endpoints are registered or acknowledged gaps', () => {
  it('no unregistered, non-bare /jaxrs endpoint outside KNOWN_BACKEND_GAPS', () => {
    const flagged = collectPaths().filter((p) => {
      if (p.endsWith('/')) return false // bare dynamic base prefix
      const dyn = p.includes('${')
      return !hitsRegistered(p, dyn) && !isKnownGap(p)
    })
    expect(flagged, `unacknowledged unregistered endpoints: ${flagged.join(', ')}`).toEqual([])
  })
})
