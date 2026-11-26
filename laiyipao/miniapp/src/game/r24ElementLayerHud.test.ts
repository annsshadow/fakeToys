// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R24 引擎只读敌人元素层快照（test-first）：
 *  - elementLayerHud() 返回当前存活敌人的只读副本（uid/defId/name/hp/maxHp/stacks）
 *  - 仅读 this.enemies 与 stacks：调用前后 replayHash/buildSnapshotSegment/score/step 不变
 *  - stacks 只列层数>0 的元素，并 clamp 到当前 element_cap
 */
import { describe, it, expect } from 'vitest'
import { BattleEngine } from './engine'
import { ELEMENT_ORDER, type Element } from './elements'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { EquippedSkill } from './heatmap'

const KINETIC_SKILL_ID = 777
const ENEMY_ID = 7

function kineticSkill(): SkillDef {
  return {
    id: KINETIC_SKILL_ID,
    code: 'r24_kinetic',
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
    name: 'R24 测试关',
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
    code: 'r24_target',
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

/** 推进到至少一个敌人出生（真引擎 tick） */
function spawnOne(e: BattleEngine): void {
  e.start()
  for (let i = 0; i < 300; i++) {
    if (e.enemies.length > 0 && e.enemies[0].spawnProgress >= 1000) return
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  throw new Error('R24 fixture: enemy did not spawn')
}

describe('R24 engine.elementLayerHud 只读敌人元素层快照', () => {
  it('真引擎 tick 后返回存活敌人快照，字段齐备且 stacks 只列正数层', () => {
    const e = makeEngine()
    spawnOne(e)
    const target = e.enemies[0]
    target.stacks.set('fire', 2n)
    target.stacks.set('ice', 0n)
    const hud = e.elementLayerHud()
    expect(hud.length).toBe(1)
    const row = hud[0]
    expect(row.uid).toBe(target.uid)
    expect(row.defId).toBe(ENEMY_ID)
    expect(row.name).toBe('靶子')
    expect(row.hp).toBe(Number(target.hp))
    expect(row.maxHp).toBe(Number(target.maxHp))
    expect(row.stacks.fire).toBe(2)
    expect(row.stacks.ice).toBeUndefined()
    expect(row.elementCap).toBe((e as unknown as { elementLayerHud(): Array<{ elementCap: number }> }).elementLayerHud()[0].elementCap)
  })

  it('snapshot 是深拷贝：外部改 row.stacks 不回写引擎内部 Map', () => {
    const e = makeEngine()
    spawnOne(e)
    const target = e.enemies[0]
    target.stacks.set('fire', 3n)
    const hud = e.elementLayerHud()
    hud[0].stacks.fire = 99
    hud[0].name = '被篡改'
    expect(target.stacks.get('fire')).toBe(3n)
    expect(e.enemies[0].name).toBe('靶子')
    expect(e.elementLayerHud()[0].stacks.fire).toBe(3)
  })

  it('stacks 层数 clamp 到 elementCap；空场返回空数组', () => {
    const e = makeEngine()
    spawnOne(e)
    const cap = (e as unknown as { elementLayerHud(): Array<{ elementCap: number }> }).elementLayerHud()[0].elementCap
    e.enemies[0].stacks.set('lightning', BigInt(cap + 10))
    expect(e.elementLayerHud()[0].stacks.lightning).toBe(cap)
    const fresh = makeEngine()
    expect(fresh.elementLayerHud()).toEqual([])
  })

  it('调用快照前后 replayHash/buildSnapshotSegment/score/phase 不变且不推进 step', () => {
    const e = makeEngine()
    spawnOne(e)
    e.enemies[0].stacks.set('fire', 2n)
    const before = {
      hash: e.replayHash(),
      seg: e.buildSnapshotSegment(),
      score: e.score,
      phase: e.phase,
      waveIndex: (e as unknown as { waveIndex: number }).waveIndex,
      tick: (e as unknown as { tick: number }).tick,
    }
    const a = e.elementLayerHud()
    const b = e.elementLayerHud()
    expect(e.replayHash()).toBe(before.hash)
    expect(e.buildSnapshotSegment()).toBe(before.seg)
    expect(e.score).toBe(before.score)
    expect(e.phase).toBe(before.phase)
    expect((e as unknown as { waveIndex: number }).waveIndex).toBe(before.waveIndex)
    expect((e as unknown as { tick: number }).tick).toBe(before.tick)
    expect(a).toEqual(b)
    const payload = e.settleInput(1) as Record<string, unknown>
    expect(payload.score).toBe(before.score)
    expect(payload.replay_hash).toBe(before.hash)
    expect(payload).not.toHaveProperty('element_layer_hud')
  })

  it('确定性：同 seed 同 stacks 的两引擎快照相等，ELEMENT_ORDER 结构稳定', () => {
    const a = makeEngine()
    const b = makeEngine()
    spawnOne(a)
    spawnOne(b)
    for (const el of ELEMENT_ORDER) {
      a.enemies[0].stacks.set(el, 1n)
      b.enemies[0].stacks.set(el, 1n)
    }
    expect(a.elementLayerHud()).toEqual(b.elementLayerHud())
  })
})
