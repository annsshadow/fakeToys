/**
 * 伤害结算的 TS 侧测试 —— 与 server/internal/domain/damage_test.go 断言同一批性质。
 *
 * 目的不是重复 Go 的测试，而是**证明两端在相同输入下产出相同数字**。
 * 若这些用例在 TS 侧也成立，两端同时改动时就会有一侧先红。
 */
import { describe, it, expect } from 'vitest'
import { Defender, resolveHit, applyElementStacks, defaultAttacker, reactionAttackRatio } from './damage'
import { MAX_REACTION_ATTACK_WEIGHT, ELEMENT_PER_STACK_BASE, PERMILLE } from './fixed'
import { ELEMENT_ORDER, REACTION_ORDER, type Element } from './elements'

function target(resist?: Partial<Record<Element, bigint>>, stacks = 0n): Defender {
  const d = new Defender(1_000_000_000n, 0n, 0n)
  if (resist) {
    for (const [k, v] of Object.entries(resist)) d.resist.set(k as Element, v)
  }
  if (stacks > 0n) {
    for (const e of ELEMENT_ORDER) d.stacks.set(e, stacks)
  }
  return d
}

describe('反通胀不变量（结构性保证）', () => {
  const cases = [
    { name: '低养成', attack: 100n, tier: 1n, stacks: 1n, resist: 0n },
    { name: '中养成', attack: 2000n, tier: 2n, stacks: 2n, resist: 0n },
    { name: '高养成', attack: 50000n, tier: 3n, stacks: 3n, resist: 0n },
    { name: '高养成+负抗', attack: 50000n, tier: 3n, stacks: 3n, resist: -500n },
    { name: '高养成+高抗', attack: 50000n, tier: 3n, stacks: 3n, resist: 500n },
    { name: '零层数', attack: 50000n, tier: 3n, stacks: 0n, resist: 0n },
    { name: '满层', attack: 50000n, tier: 3n, stacks: 8n, resist: 0n },
  ]

  for (const c of cases) {
    for (const reaction of REACTION_ORDER) {
      it(`${c.name} / ${reaction}：攻击力占比 ≤ 30%`, () => {
        const att = defaultAttacker()
        att.attack = c.attack
        att.reactionTier = c.tier
        att.elementCap = 64n

        const d = target({ fire: c.resist }, c.stacks)
        const res = resolveHit(att, d, {
          skillDamage: 1000n,
          skillElement: 'fire',
          forceReaction: reaction,
          roll: 9999,
        })
        if (res.reactionDamage <= 0n) return
        expect(reactionAttackRatio(res)).toBeLessThanOrEqual(MAX_REACTION_ATTACK_WEIGHT)
      })
    }
  }
})

describe('以弱胜强（与 Go 侧同一组数字）', () => {
  const HP = 1_000_000_000n
  const armor = 100n

  function newTarget(): Defender {
    const d = new Defender(HP, 0n, armor)
    d.resist.set('fire', 500n)
    d.resist.set('lightning', 500n)
    d.resist.set('ice', -300n)
    d.resist.set('corrosion', -300n)
    return d
  }

  function lowBuild(reaction: 'flash_freeze' | 'overheat', reactionElement: Element): bigint {
    const att = defaultAttacker()
    att.attack = 2000n
    att.reactionTier = 3n
    att.elementCoefPermille = 3000n
    att.elementCap = 64n
    att.critPermille = 0n

    const d = newTarget()
    let total = 0n
    for (let i = 0; i < 40; i++) {
      for (const e of ELEMENT_ORDER) d.stacks.set(e, 4n)
      const r = resolveHit(att, d, {
        skillDamage: 1000n,
        skillElement: 'corrosion',
        forceReaction: reaction,
        reactionElement,
        roll: 9999,
      })
      total += r.totalDamage
    }
    return total
  }

  function highBuildNoReaction(): bigint {
    const att = defaultAttacker()
    att.attack = 20000n
    att.reactionTier = 1n
    att.elementCoefPermille = 1000n
    att.elementCap = 64n
    att.critPermille = 0n

    const d = newTarget()
    let total = 0n
    for (let i = 0; i < 40; i++) {
      d.stacks = new Map([['fire' as Element, 4n]])
      const r = resolveHit(att, d, {
        skillDamage: 1000n,
        skillElement: 'fire',
        roll: 9999,
      })
      total += r.totalDamage
    }
    return total
  }

  it('低养成+正确方向必须赢过高养成无反应', () => {
    const low = lowBuild('flash_freeze', 'ice')
    const high = highBuildNoReaction()
    expect(low).toBeGreaterThan(high)
  })

  it('同养成下选对方向必须比选错方向高出至少 1.5 倍', () => {
    const correct = lowBuild('flash_freeze', 'ice')
    const wrong = lowBuild('overheat', 'fire')
    expect(correct).toBeGreaterThan((wrong * 3n) / 2n)
  })
})

describe('抗性必须真的起作用', () => {
  it('同一发打在高抗 / 无抗 / 负抗上，反应伤害依次递减', () => {
    const att = defaultAttacker()
    att.attack = 5000n
    att.reactionTier = 2n

    const hitWith = (resist: bigint): bigint => {
      const d = target({ fire: resist }, 6n)
      return resolveHit(att, d, {
        skillDamage: 1000n,
        skillElement: 'fire',
        forceReaction: 'overheat',
        roll: 9999,
      }).reactionDamage
    }
    const weak = hitWith(-500n)
    const neutral = hitWith(0n)
    const resisted = hitWith(500n)
    expect(weak).toBeGreaterThan(neutral)
    expect(neutral).toBeGreaterThan(resisted)
  })

  it('元素持续伤害同样受抗性约束（防止绕过抗性矩阵的后门）', () => {
    const att = defaultAttacker()
    att.elementCoefPermille = 3000n

    const tick = (resist: bigint): bigint => {
      const d = target({ fire: resist }, 20n)
      return resolveHit(att, d, {
        skillDamage: 0n,
        skillElement: 'fire',
        roll: 9999,
      }).elementDamage
    }
    const weak = tick(-500n)
    const resisted = tick(500n)
    expect(weak).toBeGreaterThan(resisted)
  })
})

describe('护盾与层数', () => {
  it('蒸汽爆发驱散护盾，其他反应不驱散', () => {
    const att = defaultAttacker()
    const run = (reaction: 'steam_burst' | 'overheat'): bigint => {
      const d = new Defender(10_000n, 5_000n, 0n)
      d.stacks.set('ice', 5n)
      resolveHit(att, d, {
        skillDamage: 500n,
        skillElement: 'fire',
        forceReaction: reaction,
        roll: 9999,
      })
      return d.shield
    }
    expect(run('steam_burst')).toBe(0n)
    expect(run('overheat')).toBeGreaterThan(0n)
  })

  it('层数受上限截断', () => {
    const att = defaultAttacker()
    att.elementCap = 3n
    const d = new Defender(10_000n, 0n, 0n)
    for (let i = 0; i < 10; i++) {
      applyElementStacks(att, d, { skillDamage: 0n, skillElement: 'fire', applyStacks: 1n, roll: 0 })
    }
    expect(d.stacksOf('fire')).toBe(3n)
  })

  it('层数越多反应伤害单调不减（弱玩家的成长方向）', () => {
    const att = defaultAttacker()
    att.attack = 1000n
    att.reactionTier = 3n
    att.elementCap = 64n
    const damageAt = (stacks: bigint): bigint => {
      const d = target({}, stacks)
      return resolveHit(att, d, {
        skillDamage: 1000n,
        skillElement: 'fire',
        forceReaction: 'overheat',
        roll: 9999,
      }).reactionDamage
    }
    let prev = -1n
    for (const s of [1n, 2n, 4n, 8n, 16n]) {
      const cur = damageAt(s)
      expect(cur).toBeGreaterThan(prev)
      prev = cur
    }
  })

  it('暴击不放大反应伤害（否则抗性会被绕开）', () => {
    const att = defaultAttacker()
    att.critPermille = 1000n // 必暴击
    att.critMultiplierPermille = 3000n
    const d = target({}, 6n)
    const r = resolveHit(att, d, {
      skillDamage: 1000n,
      skillElement: 'fire',
      forceReaction: 'overheat',
      roll: 0,
    })
    expect(r.crit).toBe(true)
    // 反应伤害应等于非暴击时的值
    const att2 = defaultAttacker()
    att2.critPermille = 0n
    const d2 = target({}, 6n)
    const r2 = resolveHit(att2, d2, {
      skillDamage: 1000n,
      skillElement: 'fire',
      forceReaction: 'overheat',
      roll: 0,
    })
    expect(r.reactionDamage).toBe(r2.reactionDamage)
  })
})

describe('元素持续伤害基准', () => {
  it('ELEMENT_PER_STACK_BASE 与 Go 侧一致（1200）', () => {
    expect(ELEMENT_PER_STACK_BASE).toBe(1200n)
  })
  it('PERMILLE 为 1000', () => {
    expect(PERMILLE).toBe(1000n)
  })
})
