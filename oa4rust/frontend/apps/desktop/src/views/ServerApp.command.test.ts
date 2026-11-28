// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')

describe('R624b ServerApp command execute real endpoint', () => {
  it('executeCommand posts command body to console/command/execute with failure-envelope check', () => {
    const s = read('ServerApp.vue')
    const body = s.match(/async function executeCommand\(\) \{[\s\S]*?\n\}/)?.[0] ?? ''
    expect(body).toContain("api.post('/api/console/command/execute'")
    expect(body).toContain('{ command: command.value }')
    expect(body).toContain('consoleWriteSucceeded(r)')
    expect(body).toContain('JSON.stringify(r.data, null, 2)')
    expect(body).not.toContain('/api/server/execute?command=')
  })
})
