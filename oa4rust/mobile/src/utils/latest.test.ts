// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { createLatestGate } from './latest'

describe('列表请求只认最新一次（R151）', () => {
  it('新请求让旧令牌立即失效', () => {
    const gate = createLatestGate()
    const first = gate.begin()
    expect(gate.isLatest(first)).toBe(true)
    const second = gate.begin()
    expect(gate.isLatest(first)).toBe(false)
    expect(gate.isLatest(second)).toBe(true)
  })

  it('令牌递增，未 begin 过的令牌永不算最新', () => {
    const gate = createLatestGate()
    expect(gate.begin()).toBe(1)
    expect(gate.isLatest(0)).toBe(false)
    expect(gate.begin()).toBe(2)
    expect(gate.begin()).toBe(3)
    expect(gate.isLatest(2)).toBe(false)
    expect(gate.isLatest(3)).toBe(true)
  })

  it('模拟慢网晚到：先发的请求回来时已被后发请求取代', async () => {
    const gate = createLatestGate()
    const landed: string[] = []
    const respond = async (label: string, delayMs: number, token: number) => {
      await new Promise((resolve) => setTimeout(resolve, delayMs))
      if (gate.isLatest(token)) landed.push(label)
    }
    const oldToken = gate.begin()
    const newToken = gate.begin()
    // 旧请求先发但后到（100ms），新请求后发先到（10ms）。
    await Promise.all([respond('旧页签', 100, oldToken), respond('新页签', 10, newToken)])
    expect(landed).toEqual(['新页签'])
  })
})
