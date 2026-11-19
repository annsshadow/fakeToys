// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync, statSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'

function collectVue(dir: string, out: string[]): string[] {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name)
    if (statSync(p).isDirectory()) collectVue(p, out)
    else if (name.endsWith('.vue')) out.push(p)
  }
  return out
}

describe('SFC structural tag balance (production build prerequisite)', () => {
  const root = join(__dirname)
  const files = collectVue(root, [])

  it('finds desktop .vue files', () => {
    expect(files.length).toBeGreaterThan(0)
  })

  it('every .vue has balanced <template>/<script>/<style> block tags', () => {
    const unbalanced: string[] = []
    for (const f of files) {
      const src = readFileSync(f, 'utf8')
      for (const tag of ['template', 'script', 'style']) {
        const open = (src.match(new RegExp('<' + tag + '[\\s>]', 'g')) || []).length
        const close = (src.match(new RegExp('</' + tag + '>', 'g')) || []).length
        if (open !== close) unbalanced.push(f + ' ' + tag + ' open=' + open + ' close=' + close)
      }
    }
    expect(unbalanced).toEqual([])
  })
})
