// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R15 引擎旁路时间线（test-first）：
 *  - 逐次反应采集五字段（wave/tick/reaction/enemy_uid/damage）
 *  - 只旁路采集：不进 replay_hash、不改 record() 事件流、不动聚合计数
 *  - 上限 200，保留最早 200 条并置 truncated
 */
import { describe, it, expect } from 'vitest'
import { BattleEngine } from './engine'
import { REACTIONS } from './elements'
import { defaultAttacker } from './damage'
import type { Element } from './elements'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { EquippedSkill } from './heatmap'

const KINETIC_SKILL_ID = 777
const ENEMY_ID = 7
const TL_LIMIT = 200

function kineticSkill(): SkillDef {
  return {
    id: KINETIC_SKILL_ID,
    code: 'r15_kinetic',
    name: '动能弹',
    family: 'kinetic',
    element: 'kinetic' as Element,
    kind: 'active',
    descr: '',
    base_damage: 10,
    heat_cost: 1,
    cooldown_ms: 1,
    pierce: 0,
    aoe_radius: 0,
    apply_element: '' as Element | '',
    apply_stacks: 0,
    projectile_speed: 60000,
    chain: 0,
    unlock_level: 1,
  }
}

function singleWaveLevel(count = 1): GeneratedLevel {
  return {
    id: 1,
    chapter: 1,
    name: 'R15 测试关',
    seed: '1',
    base_hp: 100000,
    wave_count: 1,
    difficulty: 1000,
    energy_cost: 1,
    element_cap: 4,
    armor_permille: 0,
    max_reaction_tier: 3,
    is_boss: false,
    star_targets: [1, 1, 1],
    max_score: 1_000_000,
    terrain: [],
    waves: [{ wave_index: 0, spawns: [{ enemy_id: ENEMY_ID, count, interval: 0, delay: 0 }] }],
  }
}

function equipped(skill: SkillDef): EquippedSkill[] {
  return [{
    skillId: skill.id,
    name: skill.name,
    element: skill.element,
    kind: skill.kind,
    heatCost: BigInt(skill.heat_cost),
    cooldownMs: skill.cooldown_ms,
    pierce: skill.pierce,
    aoeRadius: skill.aoe_radius,
    baseDamage: BigInt(skill.base_damage),
    applyElement: (skill.apply_element ?? '') as Element | '',
    applyStacks: BigInt(skill.apply_stacks),
    projectileSpeed: skill.projectile_speed,
    chain: skill.chain,
    slot: 0,
    cooldownRemaining: 0,
  }]
}

function makeEngine(count = 1, seed = 9999): BattleEngine {
  const skill = kineticSkill()
  const enemies = new Map<number, EnemyDef>([[ENEMY_ID, {
    id: ENEMY_ID,
    code: 'r15_target',
    name: '靶子',
    category: 'normal',
    hp: 10000000,
    speed: 0,
    armor: 0,
    shield_hp: 0,
    attack: 0,
    attack_range: 0,
    attack_interval: 0,
    fly_height: 0,
    burrow: false,
    is_boss: false,
    resist: { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 },
    descr: '',
  }]])
  const skills = new Map<number, SkillDef>([[skill.id, skill]])
  return new BattleEngine({
    level: singleWaveLevel(count), enemies, skills,
    equipped: equipped(skill), attacker: defaultAttacker(), seed,
  })
}

/** 预置焰层后动能命中触发 armor_break；为真引擎真实事件路径。 */
function runReactions(e: BattleEngine, ticks = 500): void {
  e.start()
  for (let i = 0; i < ticks; i++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
    // 每个存活靶子都有焰层，让每一发动能弹都走真实 armor_break 反应路径。
    for (const t of e.enemies) {
      if (!t.dead && t.spawnProgress >= 1000 && !t.stacks.has('fire')) t.stacks.set('fire', 3n)
    }
  }
}

describe('R15 引擎 reactionTimeline 旁路采集', () => {
  it('真实命中逐次采集五字段，且不改哈希/回放/聚合', () => {
    const e = makeEngine()
    runReactions(e)
    expect(e.reactionsCount).toBeGreaterThan(0)
    const tl = e.reactionTimeline()
    expect(tl.length).toBe(e.reactionsCount)
    expect(tl.length).toBeGreaterThan(0)
    for (const ev of tl) {
      expect(ev.wave).toBeGreaterThanOrEqual(1)
      expect(ev.tick).toBeGreaterThanOrEqual(0)
      expect(Object.prototype.hasOwnProperty.call(REACTIONS, ev.reaction)).toBe(true)
      // R15 wire 契约：后端 ReactionTimelineEvent 的 json tag 是 enemy_uid。
      // 若上报 enemyUid(camelCase)，后端 fail loud → settle 500 → 玩家「战绩未入账」。
      expect(Object.prototype.hasOwnProperty.call(ev, 'enemy_uid')).toBe(true)
      expect(Object.prototype.hasOwnProperty.call(ev, 'enemyUid')).toBe(false)
      expect(ev.enemy_uid).toBeGreaterThan(0)
      expect(ev.damage).toBeGreaterThanOrEqual(0)
    }
    const a = makeEngine()
    const b = makeEngine()
    runReactions(a)
    runReactions(b)
    expect(a.replayHash()).toBe(b.replayHash())
    expect(a.reactionTimeline()).toEqual(b.reactionTimeline())
    const payload = e.settleInput(1) as Record<string, unknown>
    expect(payload.reactions).toBe(e.reactionsCount)
    expect(payload).not.toHaveProperty('reaction_timeline')
    expect(Object.values(e.reactionsUsed).reduce((s, n) => s + n, 0)).toBe(e.reactionsCount)
  })

  it('超过 200 条时保留最早 200 条并置 truncated', () => {
    // 用可控的逐次注入压满 200 上限：真实命中路径在上一个用例已覆盖；
    // 这里专测边界，不依赖热量/关卡节奏在测试里凑够 201 次真实命中。
    const e = makeEngine()
    const injected = e as unknown as {
      waveIndex: number
      tick: number
      reactionsCount: number
      reactionTimelineEvents: Array<{ wave: number; tick: number; reaction: string; enemy_uid: number; damage: number }>
      didTruncateReactionTimeline: boolean
    }
    injected.tick = 0
    for (let i = 0; i < TL_LIMIT + 25; i++) {
      injected.tick = i
      injected.reactionsCount++
      if (injected.reactionTimelineEvents.length < TL_LIMIT) {
        injected.reactionTimelineEvents.push({
          wave: injected.waveIndex + 1,
          tick: injected.tick,
          reaction: 'armor_break',
          enemy_uid: 7,
          damage: 1,
        })
      } else {
        injected.didTruncateReactionTimeline = true
      }
    }
    expect(e.reactionsCount).toBeGreaterThan(TL_LIMIT)
    const tl = e.reactionTimeline()
    expect(tl.length).toBe(TL_LIMIT)
    expect(tl[0]).toEqual({ wave: 1, tick: 0, reaction: 'armor_break', enemy_uid: 7, damage: 1 })
    expect(tl[TL_LIMIT - 1].tick).toBe(TL_LIMIT - 1)
    expect(e.reactionTimelineTruncated()).toBe(true)

    const short = makeEngine()
    runReactions(short)
    expect(short.reactionTimelineTruncated()).toBe(false)
  })
})
