// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R17 引擎旁路采集契约（test-first）：
 * replay() 在 runToEnd 后同帧取 engine.reactionTimeline()，
 * 绝不改 computedHash / matched 任何计算；旧记录缺字段仍照常重放。
 */
import { describe, it, expect } from 'vitest'
import { replay } from './replay'
import type { ReplayInfo, BuildSnapshot } from './replay'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { REACTIONS } from './elements'

const zeroResist = { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 }
const ENEMIES: EnemyDef[] = [{
  id: 1, code: 'wanderer', name: '游荡者', category: 'normal',
  hp: 60, speed: 40000, armor: 0, shield_hp: 0, attack: 8, attack_range: 0,
  attack_interval: 0, fly_height: 0, burrow: false, is_boss: false,
  resist: zeroResist, descr: '',
}]
function skill(id: number, element: Element): SkillDef {
  return {
    id, code: 's' + id, name: 'S' + id, family: 'flame', element, kind: 'active', descr: '',
    base_damage: 100, heat_cost: 20, cooldown_ms: 400, pierce: 1, aoe_radius: 60,
    apply_element: element, apply_stacks: 2, projectile_speed: 90000, chain: 0, unlock_level: 1,
  }
}
const SKILLS = [skill(1, 'fire'), skill(2, 'ice')]
function level(): GeneratedLevel {
  return {
    id: 1, chapter: 1, name: 'R17 重放', seed: '1', base_hp: 800, wave_count: 2,
    difficulty: 1000, energy_cost: 6, element_cap: 3, armor_permille: 0,
    max_reaction_tier: 2, is_boss: false, star_targets: [100, 200, 300], max_score: 500,
    terrain: [],
    waves: [
      { wave_index: 0, spawns: [{ enemy_id: 1, count: 2, interval: 60, delay: 0 }] },
      { wave_index: 1, spawns: [{ enemy_id: 1, count: 2, interval: 60, delay: 0 }] },
    ],
  }
}
function build(): BuildSnapshot {
  return {
    skills: {
      '1': { id: 1, name: '燃烧弹', family: 'flame', element: 'fire', kind: 'active', level: 1, slot: 0 },
      '2': { id: 2, name: '干冰弹', family: 'frost', element: 'ice', kind: 'active', level: 1, slot: 1 },
    },
    skill_ids: ['fire', 'ice'], equipment: {}, mastery_nodes: [],
    attacker: {
      attack: 300, crit_permille: 50, crit_multiplier_permille: 1500,
      reaction_mult_permille: 1000, element_cap: 3, reaction_tier: 2, element_coef_permille: 1200,
      heat_cap_permille: 150, armor_permille: 200, mechanic_permille: 100,
    },
  }
}
function info(over: Partial<ReplayInfo> = {}): ReplayInfo {
  return {
    battle_id: 1, level_id: 1, seed: '1234567890123456789', level: level(),
    expected_hash: '0000000000000000', created_at: '2026-01-01T00:00:00Z', build: build(), ...over,
  }
}
const deps = {
  level: level(),
  enemies: new Map(ENEMIES.map((e) => [e.id, e])),
  skills: new Map(SKILLS.map((s) => [s.id, s])),
}

describe('R17 replay() 逐次反应序列旁路采集', () => {
  it('返回 reactionTimeline 与截断标记，且五字段合法可复现', () => {
    const a = replay(info(), deps)
    const b = replay(info(), deps)
    expect(a.error).toBeUndefined()
    expect(Array.isArray(a.reactionTimeline)).toBe(true)
    expect(typeof a.reactionTimelineTruncated).toBe('boolean')
    expect(b.reactionTimeline).toEqual(a.reactionTimeline)
    for (const ev of a.reactionTimeline) {
      expect(Number.isInteger(ev.wave) && ev.wave >= 1).toBe(true)
      expect(Number.isInteger(ev.tick) && ev.tick >= 0).toBe(true)
      expect(typeof ev.reaction === 'string' && ev.reaction.length > 0).toBe(true)
      expect(Object.prototype.hasOwnProperty.call(REACTIONS, ev.reaction)).toBe(true)
      expect(Object.prototype.hasOwnProperty.call(ev, 'enemy_uid')).toBe(true)
      expect(Number.isInteger(ev.enemy_uid)).toBe(true)
      expect(Number.isFinite(ev.damage)).toBe(true)
    }
  })

  it('reaction_timeline 不进哈希也不改 matched：篡改入参序列哈希不变', () => {
    const base = replay(info(), deps)
    const tampered = replay(info({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1',
        events: [{ wave: 99, tick: 99, reaction: 'overheat', enemy_uid: 99, damage: 99999 }],
        truncated: true, total: 1,
      },
    }), deps)
    expect(tampered.computedHash).toBe(base.computedHash)
    expect(tampered.matched).toBe(base.matched)
    expect(tampered.stats).toEqual(base.stats)
    expect(tampered.reactionTimeline).toEqual(base.reactionTimeline)
  })

  it('缺构筑/缺关卡的 error 路径也返回空序列，不抛异常', () => {
    const r = replay(info({ level: undefined as unknown as GeneratedLevel }), deps)
    expect(r.error).toBeTruthy()
    expect(r.reactionTimeline).toEqual([])
    expect(r.matched).toBe(false)
  })
})
