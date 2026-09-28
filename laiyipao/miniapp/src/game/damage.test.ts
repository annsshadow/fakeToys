/**
 * 伤害结算的 TS 侧测试 —— 与 server/internal/domain/damage_test.go 断言同一批性质。
 *
 * 目的不是重复 Go 的测试，而是**证明两端在相同输入下产出相同数字**。
 * 若这些用例在 TS 侧也成立，两端同时改动时就会有一侧先红。
 */
import { describe, it, expect } from 'vitest'
import {
  Defender,
  resolveHit,
  applyElementStacks,
  assertAntiInflation,
  hitWithRng,
  defaultAttacker,
  reactionAttackRatio,
  type HitResult,
} from './damage'
import { MAX_REACTION_ATTACK_WEIGHT, ELEMENT_PER_STACK_BASE, PERMILLE } from './fixed'
import { ELEMENT_ORDER, REACTION_ORDER, type Element } from './elements'
import { BattleRng } from './lcg'

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

describe('Defender 层数簿记的边界', () => {
  it('clearElements 清空全部层数（过热结算 / 波次重置的出口）', () => {
    // clearElements 不在命中路径上，缺少测试时「清了但没清干净」
    // （比如只清了 fire 留下 ice）不会红 —— 而那会改变反应链走向。
    const d = target({}, 3n)
    expect(d.totalStacks()).toBeGreaterThan(0n)
    expect(d.dominantElement()).not.toBe('')
    d.clearElements()
    expect(d.totalStacks()).toBe(0n)
    expect(d.dominantElement()).toBe('')
    for (const e of ELEMENT_ORDER) expect(d.stacksOf(e)).toBe(0n)
  })

  it('applyElement 拒绝空元素与非正增量', () => {
    const d = new Defender(1000n, 0n, 0n)
    // 空元素：类型上不该出现，但 JSON 往返可能送来 ''。
    // 静默记账一个空键层会让 totalStacks 与 dominantElement 说谎。
    expect(d.applyElement('' as Element, 1n, 4n)).toBe(false)
    // 非正增量：0 层施加必须显式失败而不是把 Map 洗成 0 键。
    expect(d.applyElement('fire', 0n, 4n)).toBe(false)
    expect(d.applyElement('fire', -1n, 4n)).toBe(false)
    expect(d.totalStacks()).toBe(0n)
  })
})

describe('抗通胀的运行时断言与便捷入口', () => {
  it('assertAntiInflation：占比 ≤ 300‰ 时放行', () => {
    // 这是反通胀不变式的运行时守卫（开发期调用，生产可跳过）。
    // 没有测试的话，改坏 reactionAttackRatio 的分母它也不会红。
    const att = defaultAttacker()
    const r = resolveHit(att, target({}, 4n), {
      skillDamage: 1000n,
      skillElement: 'fire',
      forceReaction: 'overheat',
      roll: 9999,
    })
    expect(assertAntiInflation(r)).toBe(true)
  })

  it('assertAntiInflation：手工构造越权结果时必须失败', () => {
    // 攻击侧占比 301‰ —— 只超过红线 1‰ 也要拦。上限是硬约束，
    // 不是「大致 30%」。
    const bad = {
      reactionDamage: 1000n,
      reactionAttackPortion: 301n,
    } as HitResult
    expect(assertAntiInflation(bad)).toBe(false)

    const good = { reactionDamage: 1000n, reactionAttackPortion: 300n } as HitResult
    expect(assertAntiInflation(good)).toBe(true)
  })

  it('hitWithRng 与 resolveHit(同 roll) 逐位一致', () => {
    // hitWithRng 是「便捷入口」：它只做一件事 —— 用 rng.roll() 补全
    // HitInput 再转发。两个入口分叉 = 同一战斗两种结算，I-6 直接失效。
    const att = defaultAttacker()
    const mk = () => new Defender(1_000_000n, 0n, 0n)
    const input = { skillDamage: 1000n, skillElement: 'fire' as Element, applyStacks: 2n }

    const r1 = hitWithRng(att, mk(), input, new BattleRng(20240101))
    const rng2 = new BattleRng(20240101)
    const r2 = resolveHit(att, mk(), { ...input, roll: rng2.roll() })
    expect(r1).toEqual(r2)
  })
})

describe('畸形输入的钳制（契约向量经 JSON 往返可能送来越界值）', () => {
  it('攻击力低到使基准伤害为负时钳到 0，不产生负伤害', () => {
    // PERMILLE + attack = -1000 → d = -1000。若没有 d<0 钳制，
    // 负的"直接伤害"会在第 7 步变成给守方**加血**。
    const att = defaultAttacker()
    att.attack = -2000n
    const d = new Defender(1_000_000n, 0n, 0n)
    const r = resolveHit(att, d, { skillDamage: 1000n, skillElement: '', roll: 9999 })
    expect(r.directDamage).toBe(0n)
    expect(r.totalDamage).toBe(0n)
    expect(d.hp).toBe(1_000_000n)
  })

  it('元素系数为负时元素持续伤害钳到 0', () => {
    // elementCoefPermille = -5000 → 乘出负的 elementDmg。
    // 负层数伤害同样等于给守方回血。
    const att = defaultAttacker()
    att.elementCoefPermille = -5000n
    const d = target({ fire: 0n }, 4n)
    const r = resolveHit(att, d, { skillDamage: 0n, skillElement: 'fire', roll: 9999 })
    expect(r.elementDamage).toBe(0n)
    expect(r.totalDamage).toBe(0n)
    // target() 造的守方 HP = 1e9（见文件顶部 helper），伤害钳 0 后血量不变。
    expect(d.hp).toBe(1_000_000_000n)
  })

  it('reactionTier 为 0 时不炸除法，反应伤害为 0', () => {
    // elemDiv 直接做 `v / tier`，bigint 除 0 会抛 RangeError。
    // reactionTier 来自关卡数据（max_reaction_tier），0 是合法的畸形值。
    const att = defaultAttacker()
    att.reactionTier = 0n
    const r = resolveHit(att, target({}, 4n), {
      skillDamage: 1000n,
      skillElement: 'fire',
      forceReaction: 'overheat',
      roll: 9999,
    })
    expect(r.reactionDamage).toBe(0n)
    expect(r.reactionElemPortion).toBe(0n)
    // 直接伤害不受影响
    expect(r.directDamage).toBeGreaterThan(0n)
  })
})

describe('applyElementStacks 的缺省契约', () => {
  it('省略 applyStacks 按 0 处理（HitInput 契约：多数测试不关心层数）', () => {
    // 接口注释明确「applyStacks 允许省略（按 0 处理）」。这是
    // `?? 0n` 的回退路径 —— 若它丢了，省略字段的调用方会直接崩。
    const att = defaultAttacker()
    const d = new Defender(1_000_000n, 0n, 0n)
    applyElementStacks(att, d, { skillDamage: 0n, skillElement: 'fire', roll: 0 })
    expect(d.stacksOf('fire')).toBe(0n)
  })

  it('skillElement 为空串时不施加任何层数', () => {
    const att = defaultAttacker()
    const d = new Defender(1_000_000n, 0n, 0n)
    applyElementStacks(att, d, {
      skillDamage: 0n,
      skillElement: '',
      applyStacks: 5n,
      roll: 0,
    })
    expect(d.totalStacks()).toBe(0n)
  })
})
