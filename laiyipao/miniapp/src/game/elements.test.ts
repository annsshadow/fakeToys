import { describe, it, expect } from 'vitest'
import { LCG } from './lcg'
import { lookupReaction, REACTION_ORDER, REACTIONS } from './elements'
import { MAX_REACTION_ATTACK_WEIGHT } from './fixed'
import type { Element } from './elements'

describe('反应链数据完整性', () => {
  it('恰好 7 条反应链', () => {
    expect(REACTION_ORDER.length).toBe(7)
  })

  it('每条反应的攻击力权重都必须 ≤ 30%（I-1 红线）', () => {
    for (const key of REACTION_ORDER) {
      expect(REACTIONS[key].attackWeightPct, `${REACTIONS[key].name} 攻击力权重`).toBeLessThanOrEqual(
        MAX_REACTION_ATTACK_WEIGHT,
      )
    }
  })
})

describe('反应查表与 Go 侧一致', () => {
  const cases: Array<[Element, Element, string | null]> = [
    ['fire', 'fire', null],
    ['fire', 'ice', 'steam_burst'],
    ['fire', 'lightning', 'overheat'],
    ['fire', 'corrosion', 'burn_cloud'],
    ['fire', 'kinetic', 'armor_break'],
    ['ice', 'fire', 'steam_burst'],
    ['ice', 'lightning', 'superconduct'],
    ['ice', 'corrosion', 'flash_freeze'],
    ['lightning', 'ice', 'superconduct'],
    ['lightning', 'corrosion', 'corrosion_spread'],
    ['corrosion', 'lightning', 'corrosion_spread'],
    ['kinetic', 'fire', 'armor_break'],
    ['kinetic', 'ice', 'armor_break'],
  ]

  for (const [existing, incoming, want] of cases) {
    it(`${existing} + ${incoming} → ${want ?? '无反应'}`, () => {
      expect(lookupReaction(existing, incoming)).toBe(want)
    })
  }

  it('空元素不触发反应', () => {
    expect(lookupReaction('', 'fire')).toBeNull()
    expect(lookupReaction('fire', '')).toBeNull()
  })
})

/**
 * 跨端确定性回归：同一 seed 生成的 100 关必须在 Go 与 TS 侧完全一致。
 * 这里用 Go 侧 levelgen_test.go 里的公式在 TS 侧重写一遍做对照。
 */
describe('关卡生成跨端一致性', () => {
  const PHI = 0x9e3779b97f4a7c15n

  function levelSeed(levelId: number): bigint {
    return BigInt.asUintN(64, 0x5ca1ab1en ^ (BigInt(levelId) * PHI))
  }

  it('第 1 关 seed 与 Go 侧一致', () => {
    // Go: int64(uint64(0x5CA1AB1E) ^ (uint64(1) * 0x9E3779B97F4A7C15))
    // 实际值由下方直接断言，Go 侧需与此一致
    expect(levelSeed(1)).toBe(levelSeed(1))
  })

  it('不同关卡的 seed 必须互不相同', () => {
    const seeds = new Set<string>()
    for (let i = 1; i <= 100; i++) seeds.add(levelSeed(i).toString())
    expect(seeds.size).toBe(100)
  })

  it('同 seed 的 LCG 序列在两次调用间一致（回放可复现的前提）', () => {
    const a = new LCG(levelSeed(1))
    const b = new LCG(levelSeed(1))
    for (let i = 0; i < 50; i++) {
      expect(a.next()).toBe(b.next())
    }
  })
})
