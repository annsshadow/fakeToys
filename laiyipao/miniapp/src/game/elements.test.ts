import { describe, it, expect } from 'vitest'
import { LCG } from './lcg'
import { lookupReaction, REACTION_ORDER, REACTIONS } from './elements'
import { MAX_REACTION_ATTACK_WEIGHT } from './fixed'
import type { Element } from './elements'
import levelSeeds from '@vectors/level_seeds.json'

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
 * ⚠️ 曾经的实现是「在 TS 侧重写一遍 Go 的公式做对照」，那是三重自证（见下方说明）。
 */
/**
 * 关卡种子的跨端一致性。
 *
 * ⚠️ 这个 describe 块曾经是**假的**。
 *
 * 原实现是：
 *   const PHI = 0x9e3779b97f4a7c15n                       // 测试内本地复制的常量
 *   function levelSeed(id) { return ... ^ (id * PHI) }   // 测试内本地复制的公式
 *   expect(levelSeed(1)).toBe(levelSeed(1))               // 恒等式
 *
 * 三重自证：断言自己等于自己、常量抄了一份、公式也抄了一份。
 * 实测把 Go 侧的 0x9E3779B97F4A7C15 改掉，这条名叫「与 Go 侧一致」的
 * 用例照样全绿。
 *
 * 根本原因是 TS 端**没有关卡生成实现**（关卡由 Go 的 levelgen 现算，
 * 客户端只接收结果）。所以客户端能做的只有一件事：
 * 拿 Go 侧导出的**真值**字面量做断言。
 *
 * 真值来自 testdata/level_seeds.json（由 `go run ./cmd/vectors` 导出）。
 * 这才是跨端锁：改 Go 侧实现 -> -check 报过期 -> 客户端测试红。
 */
describe('关卡种子跨端一致性', () => {
  const seeds = levelSeeds as string[]

  it('契约文件覆盖 1..100 关', () => {
    expect(seeds.length).toBe(100)
    expect(seeds.every((s) => typeof s === 'string' && s.length > 0)).toBe(true)
  })

  it('100 个 seed 互不相同（否则两关的战斗序列会重复）', () => {
    expect(new Set(seeds).size).toBe(100)
  })

  it('第 1 关 seed 等于 Go 侧的真值（字面量，不重算）', () => {
    // ⚠️ 期望值是字面量，不是现算。
    // 写成 `expect(seed(1)).toBe(seed(1))` 就退化成恒等式；
    // 写成 `expect(seed(1)).toBe(recomputed)` 就退化成本地自证 ——
    // 两种都挡不住 Go 侧被改。
    expect(seeds[0]).toBe('-7046029255919282421')
    expect(seeds[1]).toBe('4354685563387073332')
    expect(seeds[2]).toBe('-2691343690999341279')
  })

  it('seed 能被 BigInt 解析（跨端字符串承载，无 2^53 失真）', () => {
    // 走 JSON number 会失真：-7046029255919282421 超过 2^53。
    // 所以契约文件与 /battle/token 的 seed 一样用字符串承载。
    for (const s of seeds) {
      expect(() => BigInt(s)).not.toThrow()
    }
    // 至少要有一个超过 2^53 的值，否则这条测试是空转
    const big = seeds.filter((s) => {
      const v = BigInt(s)
      return v > BigInt(Number.MAX_SAFE_INTEGER) || v < BigInt(Number.MIN_SAFE_INTEGER)
    })
    expect(big.length).toBeGreaterThan(0)
  })

  it('同 seed 的 LCG 序列可复现（回放验真的前提）', () => {
    const a = new LCG(BigInt(seeds[0]))
    const b = new LCG(BigInt(seeds[0]))
    for (let i = 0; i < 50; i++) {
      expect(a.next()).toBe(b.next())
    }
  })
})
