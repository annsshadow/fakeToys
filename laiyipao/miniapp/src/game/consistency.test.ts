/**
 * 双端公式一致性检查 —— 读 server/testdata/formula_vectors.json，断言与 Go 侧相同的数字。
 *
 * 这是「两端公式必须一致」这条约定的可执行形态。
 * Go 侧对应 internal/domain/consistency_test.go。
 * 任一端单方面改动公式 → 两端中至少一方红。
 *
 * ⚠️ 改动 damage.ts / lcg.ts / fixed.ts / elements.ts 的公式时，
 * 必须同步 Go 侧并重新回填契约向量，否则本测试会失败（这正是它的目的）。
 */
import { describe, it, expect } from 'vitest'
import vectors from '@vectors/formula_vectors.json'
import { LCG, fnv1a64, hex16 } from './lcg'
import { Defender, resolveHit, defaultAttacker, reactionAttackRatio } from './damage'
import { ELEMENT_ORDER, type Element, type ReactionKey } from './elements'
import {
  PERMILLE,
  DIRECT_WEIGHT,
  ELEMENT_PER_STACK_BASE,
  ELEMENT_TICK_DIVISOR,
  MAX_RESIST,
  MAX_ARMOR,
  MAX_REACTION_ATTACK_WEIGHT,
} from './fixed'

describe('跨端契约：LCG', () => {
  it('必须与 Go 侧产出逐位相同的序列', () => {
    // 契约向量里 64 位值用字符串承载：JSON number 超过 2^53 后 JS 解析即失精，
    // 直接 BigInt(6954...) 会得到与 Go 不同的值，那是假通过。
    const l = new LCG(vectors.lcg.seed)
    for (const want of vectors.lcg.expected) {
      expect(l.next()).toBe(BigInt(want))
    }
  })
})

describe('跨端契约：FNV-1a 64', () => {
  for (const c of vectors.fnv1a64.vectors) {
    it(`FNV-1a(${JSON.stringify(c.input)}) = ${c.expected}`, () => {
      expect(hex16(fnv1a64(c.input))).toBe(c.expected)
    })
  }
})

describe('跨端契约：伤害结算', () => {
  for (const c of vectors.damage.cases) {
    it(c.intent ?? c.name, () => {
      const att = defaultAttacker()
      att.attack = BigInt(c.attacker.attack)
      att.reactionTier = BigInt(c.attacker.reaction_tier)
      att.elementCoefPermille = BigInt(c.attacker.element_coef_permille)
      att.elementCap = BigInt(c.attacker.element_cap)
      att.critPermille = 0n
      att.critMultiplierPermille = 1500n

      const def = new Defender(
        BigInt(c.defender.hp),
        0n,
        BigInt(c.defender.armor_permille),
      )
      for (const [k, v] of Object.entries(c.defender.resist ?? {})) {
        def.resist.set(k as Element, BigInt(v))
      }
      if (c.defender.stacks_per_element > 0) {
        for (const e of ELEMENT_ORDER) {
          def.stacks.set(e, BigInt(c.defender.stacks_per_element))
        }
      }

      const res = resolveHit(att, def, {
        skillDamage: BigInt(c.hit.skill_damage),
        skillElement: c.hit.skill_element as Element,
        forceReaction: (c.hit.force_reaction || undefined) as ReactionKey | undefined,
        reactionElement: (c.hit.reaction_element || undefined) as Element | undefined,
        roll: c.hit.roll,
      })

      const e = c.expect
      expect(res.reactionAttackPortion, 'reaction_attack_portion').toBe(
        BigInt(e.reaction_attack_portion),
      )
      expect(res.reactionElemPortion, 'reaction_elem_portion').toBe(BigInt(e.reaction_elem_portion))
      expect(res.reactionDamage, 'reaction_damage').toBe(BigInt(e.reaction_damage))
      expect(res.directDamage, 'direct_damage').toBe(BigInt(e.direct_damage))
      expect(res.elementDamage, 'element_damage').toBe(BigInt(e.element_damage))
      expect(res.totalDamage, 'total_damage').toBe(BigInt(e.total_damage))
      expect(res.crit, 'crit').toBe(e.crit)
      expect(res.killed, 'killed').toBe(e.killed)
    })
  }
})

describe('跨端契约：设计常量', () => {
  it('全部常量必须与 Go 侧一致', () => {
    const c = vectors.constants
    expect(ELEMENT_PER_STACK_BASE).toBe(BigInt(c.element_per_stack_base))
    expect(ELEMENT_TICK_DIVISOR).toBe(BigInt(c.element_tick_divisor))
    expect(DIRECT_WEIGHT).toBe(BigInt(c.direct_weight_permille))
    expect(PERMILLE).toBe(BigInt(c.permille))
    expect(MAX_RESIST).toBe(BigInt(c.max_resist_permille))
    expect(MAX_ARMOR).toBe(BigInt(c.max_armor_permille))
    expect(MAX_REACTION_ATTACK_WEIGHT).toBe(
      BigInt(vectors.balance_invariants.max_reaction_attack_weight_permille),
    )
  })
})

describe('跨端契约：平衡意图仍然成立', () => {
  const byName = (n: string) => vectors.damage.cases.find((c) => c.name === n)!
  const total = (n: string) => BigInt(byName(n).expect.total_damage)

  it('低养成+正确搭配 > 高养成+无反应（以弱胜强）', () => {
    if (!vectors.balance_invariants.low_correct_beats_high_no_reaction) return
    expect(total('low_invest_correct_element')).toBeGreaterThan(total('high_invest_no_reaction'))
  })

  it('同养成下正确方向比错误方向至少高 1.5 倍（抗性有牙齿）', () => {
    const min = vectors.balance_invariants.correct_direction_ratio_min
    const correct = total('low_invest_correct_element')
    const wrong = total('low_invest_wrong_element')
    expect(Number(correct) / Number(wrong)).toBeGreaterThanOrEqual(min)
  })

  it('契约向量自身必须仍满足反通胀不变量', () => {
    for (const c of vectors.damage.cases) {
      const e = c.expect
      if (e.reaction_damage <= 0) continue
      const ratio = (e.reaction_attack_portion * 1000) / e.reaction_damage
      expect(
        ratio,
        `${c.name}：攻击力占比 ${ratio}‰ 超过上限 ${vectors.balance_invariants.max_reaction_attack_weight_permille}‰`,
      ).toBeLessThanOrEqual(vectors.balance_invariants.max_reaction_attack_weight_permille)
    }
  })
})

describe('跨端契约：关卡生成', () => {
  it('关卡总数与地形关下限与 Go 侧一致', () => {
    expect(vectors.levelgen.levels).toBe(100)
    expect(vectors.levelgen.terrain_level_min).toBeGreaterThanOrEqual(20)
  })
})

// 顺带复核 reactionAttackRatio 的实现与契约向量自洽
describe('reactionAttackRatio 自洽性', () => {
  it('用契约向量反查占比应等于 reaction_attack_portion/reaction_damage', () => {
    for (const c of vectors.damage.cases) {
      const att = defaultAttacker()
      att.attack = BigInt(c.attacker.attack)
      att.reactionTier = BigInt(c.attacker.reaction_tier)
      att.elementCoefPermille = BigInt(c.attacker.element_coef_permille)
      att.elementCap = BigInt(c.attacker.element_cap)
      att.critPermille = 0n

      const def = new Defender(BigInt(c.defender.hp), 0n, BigInt(c.defender.armor_permille))
      for (const [k, v] of Object.entries(c.defender.resist ?? {})) {
        def.resist.set(k as Element, BigInt(v))
      }
      for (const e of ELEMENT_ORDER) def.stacks.set(e, BigInt(c.defender.stacks_per_element))

      const res = resolveHit(att, def, {
        skillDamage: BigInt(c.hit.skill_damage),
        skillElement: c.hit.skill_element as Element,
        forceReaction: (c.hit.force_reaction || undefined) as ReactionKey | undefined,
        reactionElement: (c.hit.reaction_element || undefined) as Element | undefined,
        roll: c.hit.roll,
      })
      expect(reactionAttackRatio(res), `${c.name} 占比`).toBeLessThanOrEqual(
        MAX_REACTION_ATTACK_WEIGHT,
      )
    }
  })
})
