/**
 * 专精「额外插槽」——最后一批曾被接线的惰性节点。
 *
 * ## 之前为什么无效
 *
 * `MasteryEffect.ExtraSlots` 一直会被算出来，但：
 *  1. 客户端用**编译期常量** `ACTIVE_SLOTS = 4`，
 *     根本不读服务端 `/mastery` 下发的 `base_slots`（它压根没读）
 *  2. 服务端**不校验**玩家装了几个技能 —— `user_skill_slots` 照单全收
 *
 * 于是玩家花点数点出来的槽位在战斗里不存在，
 * 而「有几个槽位」完全由客户端说了算。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, type BattleConfig } from './engine'
import { ACTIVE_SLOTS, type EquippedSkill } from './heatmap'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

const level = (fixture.levels as unknown as GeneratedLevel[]).find((l) => l.id === 1)!
const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

/** 按 slot 顺序造 n 个主动技能。 */
function equippedAtSlots(slots: number[]): EquippedSkill[] {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
  return slots.map((slot, i) => {
    const s = active[i % active.length]
    return {
      skillId: s.id,
      name: s.name,
      element: s.element as Element,
      kind: s.kind,
      heatCost: BigInt(s.heat_cost),
      cooldownMs: s.cooldown_ms,
      pierce: s.pierce,
      aoeRadius: s.aoe_radius,
      baseDamage: BigInt(s.base_damage),
      applyElement: (s.apply_element ?? '') as Element | '',
      applyStacks: BigInt(s.apply_stacks),
      projectileSpeed: s.projectile_speed,
      chain: s.chain,
      slot,
      cooldownRemaining: 0,
    }
  })
}

function mk(activeSlots: number | undefined, slots: number[]): BattleEngine {
  const cfg: BattleConfig = {
    level,
    enemies,
    skills,
    equipped: equippedAtSlots(slots),
    attacker: defaultAttacker(),
    seed: 12345,
  }
  if (activeSlots !== undefined) cfg.activeSlots = activeSlots
  const e = new BattleEngine(cfg)
  e.start()
  return e
}

describe('专精·额外插槽', () => {
  it('缺省槽位数必须等于 ACTIVE_SLOTS（决定所有历史战报的重放哈希）', () => {
    // ⚠️ 这一条是本文件最重要的护栏。
    // 绝大多数战报是 4 槽的，缺省值一旦不是 4，那些战报全部重放不出原哈希，
    // 而 I-6 会把它们判成伪造。
    const e = mk(undefined, [0, 1, 2, 3])
    expect(e.activeSlots).toBe(ACTIVE_SLOTS)
    expect(ACTIVE_SLOTS).toBe(4)
  })

  it('显式传入时按传入值生效', () => {
    expect(mk(6, [0, 1, 2, 3, 4, 5]).activeSlots).toBe(6)
    expect(mk(5, [0, 1, 2, 3, 4]).activeSlots).toBe(5)
  })

  it('槽位开到 6 时，第 5/6 号槽位的技能真的会开火（否则仍然是惰性）', () => {
    // ⚠️ 关键判据：不是"字段读到了"，而是**那些槽位的技能真的参与了战斗**。
    const shotsAt = (activeSlots: number, slots: number[]): Set<number> => {
      const e = mk(activeSlots, slots)
      const fired = new Set<number>()
      for (let t = 0; t < 3000; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        for (const ev of e.step()) {
          if (ev.type === 'fire') fired.add(ev.slot)
        }
      }
      return fired
    }

    const four = shotsAt(4, [0, 1, 2, 3, 4, 5])
    // 4 槽时，slot 4/5 的技能一次都不该开火（引擎忽略超出的槽位）
    expect([...four].sort()).toEqual([0, 1, 2, 3])

    const six = shotsAt(6, [0, 1, 2, 3, 4, 5])
    // 6 槽时，slot 4/5 必须真的开火 —— 这才是「额外插槽」生效的证据
    expect([...six].sort()).toEqual([0, 1, 2, 3, 4, 5])
  })

  it('槽位数不影响攻方属性（它是构筑维度，不是养成维度）', () => {
    // 「额外插槽」不该顺带改变伤害/暴击等 —— 它只给一个位置。
    const four = mk(4, [0, 1, 2, 3])
    const six = mk(6, [0, 1, 2, 3, 4, 5])
    const a4 = (four as unknown as { currentAttacker(): unknown }).currentAttacker()
    const a6 = (six as unknown as { currentAttacker(): unknown }).currentAttacker()
    expect(a6).toEqual(a4)
  })
})
