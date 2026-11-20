// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')

describe('BBSForum detail race contract', () => {
  it('guards detail backfill and extras with a shared sequence', () => {
    const seqDecl = source.indexOf('let detailSeq = 0')
    expect(seqDecl).toBeGreaterThan(-1)

    const openStart = source.indexOf('async function openTopic(topic: Topic): Promise<void> {')
    expect(openStart).toBeGreaterThan(seqDecl)
    const openSlice = source.slice(openStart, source.indexOf('function formatContent', openStart))
    const seqTake = openSlice.indexOf('const seq = ++detailSeq')
    const backfill = openSlice.indexOf('viewingTopic.value = { ...topic, ...full')
    const guard = openSlice.indexOf('if (seq !== detailSeq) return')
    expect(seqTake).toBeGreaterThan(-1)
    expect(backfill).toBeGreaterThan(seqTake)
    expect(guard).toBeGreaterThan(seqTake)
    expect(guard).toBeLessThan(backfill)
    expect(openSlice).toContain('loadTopicExtras(targetId, seq)')

    const extrasStart = source.indexOf('async function loadTopicExtras(')
    expect(extrasStart).toBeGreaterThan(openStart)
    const extrasSlice = source.slice(extrasStart, source.indexOf('async function toggleMod', extrasStart))
    const extrasGuard = extrasSlice.indexOf('if (seq !== detailSeq) return')
    expect(extrasGuard).toBeGreaterThan(-1)
    const afterGuard = extrasSlice.slice(extrasGuard)
    expect(afterGuard).toContain('topicPics.value =')
    expect(afterGuard).toContain('replyGate.value =')
    expect(afterGuard).toContain('topicMeta.value =')
  })

  it('invalidates in-flight loads when the detail modal is closed', () => {
    expect(source).toContain('@click.self="closeTopic()"')
    expect(source).toContain('@click="closeTopic()"')

    const closeStart = source.indexOf('function closeTopic(): void {')
    expect(closeStart).toBeGreaterThan(-1)
    const closeSlice = source.slice(closeStart, closeStart + 200)
    expect(closeSlice).toContain('detailSeq++')
    expect(closeSlice).toContain('viewingTopic.value = null')
    expect(source).not.toContain('viewingTopic = null')
  })
})
