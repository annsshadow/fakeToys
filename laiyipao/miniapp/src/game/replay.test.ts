/**
 * 重放引擎（I-6 客户端闭环）的行为契约。
 *
 * 这些用例锁住三件事：
 *  1. 同输入必然同哈希（重放的前提）
 *  2. 缺构筑 / 缺 attacker 时**明确报错**，而不是给一个必然不匹配的哈希
 *     （后者会让 I-6 变成制造假警报的噪音源）
 *  3. 篡改种子或构筑必然改变哈希 —— 否则"证伪"根本不成立
 */
import { describe, it, expect } from 'vitest'
import { replay, equippedFromSnapshot, prettyHash, prettyDuration } from './replay'
import { BattleEngine } from './engine'
import type { ReplayInfo, BuildSnapshot } from './replay'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

const zeroResist = { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 }

const ENEMIES: EnemyDef[] = [
  {
    id: 1, code: 'wanderer', name: '游荡者', category: 'normal',
    hp: 60, speed: 40000, armor: 0, shield_hp: 0, attack: 8, attack_range: 0,
    attack_interval: 0, fly_height: 0, burrow: false, is_boss: false,
    resist: zeroResist, descr: '',
  },
]

function skill(id: number, element: Element, name = `S${id}`): SkillDef {
  return {
    id, code: `s${id}`, name, family: 'flame', element, kind: 'active', descr: '',
    base_damage: 100, heat_cost: 20, cooldown_ms: 400, pierce: 1, aoe_radius: 60,
    apply_element: element, apply_stacks: 2, projectile_speed: 90000, chain: 0,
    unlock_level: 1,
  }
}

const SKILLS: SkillDef[] = [skill(1, 'fire', '燃烧弹'), skill(2, 'ice', '干冰弹')]

function level(): GeneratedLevel {
  return {
    id: 1, chapter: 1, name: '重放测试', seed: '1', base_hp: 800, wave_count: 2,
    difficulty: 1000, energy_cost: 6, element_cap: 3, armor_permille: 0,
    max_reaction_tier: 2, is_boss: false, star_targets: [100, 200, 300],
    // 理论满分：结算裁剪的上界锚定在它上面（不是 star_targets[2]）
    max_score: 500,
    terrain: [],
    waves: [
      { wave_index: 0, spawns: [{ enemy_id: 1, count: 2, interval: 60, delay: 0 }] },
      { wave_index: 1, spawns: [{ enemy_id: 1, count: 2, interval: 60, delay: 0 }] },
    ],
  }
}

function build(over: Partial<BuildSnapshot> = {}): BuildSnapshot {
  return {
    skills: {
      '1': { id: 1, name: '燃烧弹', family: 'flame', element: 'fire', kind: 'active', level: 1, slot: 0 },
      '2': { id: 2, name: '干冰弹', family: 'frost', element: 'ice', kind: 'active', level: 1, slot: 1 },
    },
    skill_ids: ['fire', 'ice'],
    equipment: {},
    mastery_nodes: [],
    attacker: {
      attack: 300,
      crit_permille: 50,
      crit_multiplier_permille: 1500,
      reaction_mult_permille: 1000,
      element_cap: 3,
      reaction_tier: 2,
      element_coef_permille: 1200,
    heat_cap_permille: 150,
    armor_permille: 200,
    mechanic_permille: 100,
    },
    ...over,
  }
}

function info(over: Partial<ReplayInfo> = {}): ReplayInfo {
  return {
    battle_id: 1,
    level_id: 1,
    seed: '1234567890123456789',
    level: level(),
    expected_hash: '0000000000000000',
    created_at: '2026-01-01T00:00:00Z',
    build: build(),
    ...over,
  }
}

const deps = {
  level: level(),
  enemies: new Map(ENEMIES.map((e) => [e.id, e])),
  skills: new Map(SKILLS.map((s) => [s.id, s])),
}

describe('重放：确定性', () => {
  it('同输入两次重放产出完全相同的哈希', () => {
    const a = replay(info(), deps)
    const b = replay(info(), deps)
    expect(a.error).toBeUndefined()
    expect(a.computedHash).toMatch(/^[0-9a-f]{16}$/)
    expect(b.computedHash).toBe(a.computedHash)
  })

  it('统计结果在两次重放间一致（kills/leaked/score）', () => {
    const a = replay(info(), deps)
    const b = replay(info(), deps)
    expect(b.stats).toEqual(a.stats)
  })

  it('重放会真的跑完整局（不是空转）', () => {
    const r = replay(info(), deps)
    expect(r.stats.kills + r.stats.leaked).toBeGreaterThan(0)
    expect(r.stats.durationMs).toBeGreaterThan(0)
  })
})

describe('重放：篡改必然被发现（I-6 的核心保证）', () => {
  it('换种子 → 哈希改变', () => {
    const a = replay(info({ seed: '1234567890123456789' }), deps)
    const b = replay(info({ seed: '9876543210987654321' }), deps)
    expect(b.computedHash).not.toBe(a.computedHash)
  })

  it('改攻方攻击力 → 哈希改变', () => {
    const a = replay(info(), deps)
    const b = replay(
      info({
        build: build({
          attacker: {
            attack: 9999, crit_permille: 50, crit_multiplier_permille: 1500,
            reaction_mult_permille: 1000, element_cap: 3, reaction_tier: 2,
            element_coef_permille: 1200,
    heat_cap_permille: 150,
    armor_permille: 200,
    mechanic_permille: 100,
          },
        }),
      }),
      deps,
    )
    expect(b.computedHash).not.toBe(a.computedHash)
  })

  it('换构筑技能 → 哈希改变', () => {
    const a = replay(info(), deps)
    const b = replay(
      info({
        build: build({
          skills: {
            '1': { id: 1, name: '燃烧弹', family: 'flame', element: 'fire', kind: 'active', level: 1, slot: 0 },
          },
        }),
      }),
      deps,
    )
    expect(b.computedHash).not.toBe(a.computedHash)
  })

  it('expected_hash 不同时 matched 必须为 false（这是"证伪"路径）', () => {
    const r = replay(info({ expected_hash: 'ffffffffffffffff' }), deps)
    expect(r.computedHash).not.toBe('ffffffffffffffff')
    expect(r.matched).toBe(false)
  })

  it('expected_hash 与重放一致时 matched 为 true', () => {
    const first = replay(info(), deps)
    const second = replay(info({ expected_hash: first.computedHash }), deps)
    expect(second.matched).toBe(true)
  })
})

describe('重放：选牌决策复现（I-6 闭环的最后一环）', () => {
  /** 跑一局并在每个选牌阶段选第 idx 张，产出原局的 cardPicks */
  function playWithPicks(seed: string, picksPerWave: number[]): { hash: string; picks: number[] } {
    const ri = info({ seed })
    const equipped = equippedFromSnapshot(ri.build, deps.skills)
    const engine = new BattleEngine({
      level: ri.level, enemies: deps.enemies, skills: deps.skills, equipped,
      attacker: {
        attack: BigInt(ri.build.attacker!.attack),
        critPermille: BigInt(ri.build.attacker!.crit_permille),
        critMultiplierPermille: BigInt(ri.build.attacker!.crit_multiplier_permille),
        reactionMultPermille: BigInt(ri.build.attacker!.reaction_mult_permille),
        elementCap: BigInt(ri.build.attacker!.element_cap),
        reactionTier: BigInt(ri.build.attacker!.reaction_tier),
        elementCoefPermille: BigInt(ri.build.attacker!.element_coef_permille),
        // 三项本轮新增。夹具里给了非零值（heat 150 / armor 200 / mechanic 100），
        // 于是这些用例同时覆盖"重放时新字段也必须逐位还原"——
        // 漏掉任何一项，重放算出的 heat.capBonus / 防线护甲 / 卡面数值就不同，
        // 哈希随之失配。
        heatCapPermille: BigInt(ri.build.attacker!.heat_cap_permille ?? 0),
        armorPermille: BigInt(ri.build.attacker!.armor_permille ?? 0),
        mechanicPermille: BigInt(ri.build.attacker!.mechanic_permille ?? 0),
      },
      seed: BigInt(seed),
    })
    engine.start()
    let wave = 0
    for (let i = 0; i < 20000; i++) {
      const phase: string = engine.phase
      if (phase === 'won' || phase === 'lost') break
      if (phase === 'card_select') {
        const want = picksPerWave[wave] ?? -1
        if (want >= 0 && want < engine.deck.hand.length) {
          engine.takeCard(engine.deck.hand[want].id)
        } else {
          engine.skipCards()
        }
        wave++
        continue
      }
      engine.step()
    }
    return { hash: engine.replayHash(), picks: [...engine.cardPicks] }
  }

  it('不同选牌产生不同结果（说明选牌确实影响战斗）', () => {
    const a = playWithPicks('1234567890123456789', [0])
    const b = playWithPicks('1234567890123456789', [2])
    expect(b.hash, '选第 0 张与选第 2 张不应产生相同结果').not.toBe(a.hash)
  })

  it('把 card_picks 回传后，重放能复现原局（含玩家真实选牌）', () => {
    // 关键用例：玩家在第 0 波选了第 2 张牌，重放时必须选同一张。
    // 若重放固定走 skipCards（修复前的行为），这里必然失败。
    const original = playWithPicks('1234567890123456789', [2])
    expect(original.picks.length, '应记录到选牌决策').toBeGreaterThan(0)
    expect(original.picks[0], '首波应记录为索引 2').toBe(2)

    const outcome = replay(
      info({
        seed: '1234567890123456789',
        card_picks: original.picks,
        // expected_hash 必须是原局真实哈希。否则 matched 恒为 false，
        // 那测的是默认值而不是重放能力 —— 是个看起来在跑的假断言。
        expected_hash: original.hash,
      }),
      deps,
    )
    expect(outcome.error).toBeUndefined()
    expect(
      outcome.computedHash,
      '带 card_picks 的重放应与原局一致 —— 选牌决策未被复现',
    ).toBe(original.hash)
    expect(outcome.matched).toBe(true)
  })

  it('跳过整波（-1）也能被复现', () => {
    const original = playWithPicks('1234567890123456789', [-1])
    expect(original.picks[0]).toBe(-1)
    const outcome = replay(info({ seed: '1234567890123456789', card_picks: original.picks }), deps)
    expect(outcome.error).toBeUndefined()
    expect(outcome.computedHash).toBe(original.hash)
  })

  it('card_picks 越界时回退为跳过整波，不中断重放', () => {
    const outcome = replay(
      info({ seed: '1234567890123456789', card_picks: [99, 99, 99] }),
      deps,
    )
    expect(outcome.error).toBeUndefined()
    expect(outcome.computedHash).toMatch(/^[0-9a-f]{16}$/)
  })

  it('card_picks 缺失（旧记录）仍能跑完，只是无法复现选牌', () => {
    const ri = info({ seed: '1234567890123456789' })
    const outcome = replay(ri, deps)
    expect(outcome.error).toBeUndefined()
    expect(outcome.computedHash).toMatch(/^[0-9a-f]{16}$/)
  })
})

describe('重放：前置条件缺失必须明确报错', () => {
  it('缺 level → 报错', () => {
    const r = replay(info({ level: undefined as any }), deps)
    expect(r.error).toContain('缺少关卡数据')
    expect(r.matched).toBe(false)
  })

  it('缺 build → 报错，且不给哈希', () => {
    const r = replay(info({ build: undefined as any }), deps)
    expect(r.error).toContain('构筑快照')
    expect(r.computedHash).toBe('')
  })

  it('缺 attacker → 报错（而不是用默认值算出假哈希）', () => {
    const b = build()
    delete (b as any).attacker
    const r = replay(info({ build: b }), deps)
    expect(r.error).toContain('attacker')
    expect(r.computedHash).toBe('')
  })

  it('全部技能未入槽（slot 全为 -1）→ 报错', () => {
    const b = build({
      skills: {
        '1': { id: 1, name: '燃烧弹', family: 'flame', element: 'fire', kind: 'active', level: 1, slot: -1 },
        '2': { id: 2, name: '干冰弹', family: 'frost', element: 'ice', kind: 'active', level: 1, slot: -1 },
      },
    })
    const r = replay(info({ build: b }), deps)
    expect(r.error).toContain('没有')
    expect(r.computedHash).toBe('')
  })

  it('种子格式非法 → 报错', () => {
    const r = replay(info({ seed: 'not-a-number' }), deps)
    expect(r.error).toContain('种子格式非法')
  })
})

describe('equippedFromSnapshot', () => {
  it('按 slot 升序返回，跳过 slot < 0 的技能', () => {
    const b = build({
      skills: {
        '2': { id: 2, name: '干冰弹', family: 'frost', element: 'ice', kind: 'active', level: 1, slot: 1 },
        '1': { id: 1, name: '燃烧弹', family: 'flame', element: 'fire', kind: 'active', level: 1, slot: 0 },
        '3': { id: 3, name: '未装备', family: 'flame', element: 'fire', kind: 'active', level: 1, slot: -1 },
      },
    })
    const out = equippedFromSnapshot(b, new Map(SKILLS.map((s) => [s.id, s])))
    expect(out).toHaveLength(2)
    expect(out[0].skillId).toBe(1)
    expect(out[1].skillId).toBe(2)
  })

  it('技能已下架时跳过而不是抛错', () => {
    const b = build({
      skills: {
        '1': { id: 1, name: '燃烧弹', family: 'flame', element: 'fire', kind: 'active', level: 1, slot: 0 },
        '99': { id: 99, name: '已下架', family: 'x', element: 'fire', kind: 'active', level: 1, slot: 1 },
      },
    })
    const out = equippedFromSnapshot(b, new Map(SKILLS.map((s) => [s.id, s])))
    expect(out).toHaveLength(1)
  })

  it('skills 为空对象时返回空数组', () => {
    const b = build({ skills: {} })
    expect(equippedFromSnapshot(b, new Map())).toEqual([])
  })
})

describe('显示辅助', () => {
  it('prettyHash 按 4 位分组', () => {
    expect(prettyHash('cbf29ce484222325')).toBe('CBF2 9CE4 8422 2325')
  })

  it('prettyHash 对空串安全', () => {
    expect(prettyHash('')).toBe('')
  })

  it('prettyDuration 输出 m:ss', () => {
    expect(prettyDuration(0)).toBe('0:00')
    expect(prettyDuration(65_000)).toBe('1:05')
    expect(prettyDuration(600_000)).toBe('10:00')
  })
})
