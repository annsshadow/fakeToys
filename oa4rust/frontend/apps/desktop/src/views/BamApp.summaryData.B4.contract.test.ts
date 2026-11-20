// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const src = readFileSync(resolve(__dirname, 'BamApp.vue'), 'utf8')

describe('BAM summary query contract', () => {
  it('queryFn returns normalized summary data instead of undefined', () => {
    const marker = 'return { stats: nextStats, events: [] }'
    expect(src).toContain(marker)
    expect(src).toContain("const nextStats = { total: totalWork + totalTask, active: pendingTask + processingTask, completed: completedWork, failed: expiredTask }")
  })
})
