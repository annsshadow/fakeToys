// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from 'vitest'
import vectors from '@vectors/formula_vectors.json'
import { Defender } from './damage'
import { applyReactionSplash } from './engine'
import type { Element } from './elements'

/**
 * t41 反应溅射的跨端绝对值契约（TS 侧）。
 * 与 server/internal/domain/reaction_splash_contract_test.go 读同一份 JSON。
 * 任何一端把 splash 塞回 resolveHit，25% 就会变成 10%，这里立刻红。
 */
const cases = vectors.reaction_splash.cases

describe('反应溅射跨端契约（t41）', () => {
  it('向量非空（否则本文件会静默变成空转）', () => {
    expect(cases.length, 'formula_vectors.json 里 reaction_splash.cases 是空的').toBeGreaterThan(0)
  })

  for (const c of cases) {
    it(c.name, () => {
      const def = new Defender(BigInt(c.hp), BigInt(c.shield), BigInt(c.armor_permille))
      const got = applyReactionSplash(def, BigInt(c.splash))
      expect(
        {
          damage: got.damage,
          hp: def.hp,
          shield: def.shield,
          shieldBroken: got.shieldBroken,
          killed: got.killed,
        },
        c.name,
      ).toEqual({
        damage: BigInt(c.expected.damage),
        hp: BigInt(c.expected.hp),
        shield: BigInt(c.expected.shield),
        shieldBroken: c.expected.shield_broken,
        killed: c.expected.killed,
      })
    })
  }

  it('纯函数不写元素层数（crit/attacker 根本不在签名里）', () => {
    const def = new Defender(10000n, 0n, 0n)
    def.stacks.set('fire' as Element, 3n)
    def.stacks.set('lightning' as Element, 2n)
    const got = applyReactionSplash(def, 1000n)
    expect(got.damage).toBe(1000n)
    expect(def.hp).toBe(9000n)
    expect(def.stacks.get('fire' as Element)).toBe(3n)
    expect(def.stacks.get('lightning' as Element)).toBe(2n)
  })
})
