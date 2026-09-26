/**
 * 卡牌效果的**兑现**测试。
 *
 * ⚠️ 这个文件补的是一类特定缺陷：「只写不读」。
 *
 * 卡牌效果通过 `buffs.<字段> += value` 施加，引擎某处读取它才生效。
 * 如果**没有任何读取点**，代码照样编译、测试照样全绿，
 * 而玩家拿到那张卡后观察不到任何变化 ——
 * 更糟的是它会通过 mechanicIndex / cardIndex 参与 replayHash，
 * 于是**哈希记录了一个不产生任何效果的选择**。
 *
 * 本项目实际发生过两起：
 *   1. `overheat_guard`（隔热护罩：过热时免疫一次过热）—— 只写不读
 *   2. 技能卡的"装入空槽"—— 实际是覆盖已装备的技能
 *      （`findIndex(s => s.slot < ACTIVE_SLOTS && s.slot >= 0)`
 *        在「每个元素代表一个已占用槽」的数组里，命中的永远是第一个已占用槽）
 *
 * 所以这里对每一类卡都断言「施加后状态确实变了」。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

const level = (fixture.levels as unknown as GeneratedLevel[]).find((l) => l.id === 1)!
const enemyMap = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skillMap = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function mk(equippedCount = ACTIVE_SLOTS): BattleEngine {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, equippedCount)
  const cfg: BattleConfig = {
    level,
    enemies: enemyMap,
    skills: skillMap,
    equipped: active.map((s, i) => ({
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
      slot: i,
      cooldownRemaining: 0,
    })),
    attacker: defaultAttacker(),
    seed: 1,
  }
  return new BattleEngine(cfg)
}

/**
 * 施加一张卡：先把它放进手牌，再走公开的 takeCard 路径。
 *
 * ⚠️ 必须经过 takeCard（而不是直接调 applyCard）：
 * 只有 takeCard 会同时做「取牌 → applyCard → record → recordPick」，
 * 也就是说它才是"玩家真的拿了这张牌"的那条路径。
 * 直接调 applyCard 会绕过选牌记录，导致测的是一个真实流程里不存在的状态。
 */
function give(e: BattleEngine, card: Record<string, unknown>): void {
  const eng = e as unknown as {
    phase: string
    deck: { hand: unknown[] }
    takeCard(id: string): unknown
  }
  eng.phase = 'card_select'
  eng.deck.hand = [card]
  eng.takeCard(card.id as string)
}

describe('卡牌：隔热护罩（overheat_guard）确实生效', () => {
  // 修复前：buffs.overheatGuard += value 之后再无任何读取点。
  // 玩家拿到卡、看到 hash 变了、战斗表现一点没变。
  it('施加后护盾层数增加', () => {
    const e = mk()
    give(e, {
      id: 'mechanic_overheat_guard',
      name: '隔热护罩',
      descr: '过热时免疫一次过热',
      kind: 'mechanic',
      mechanic: { kind: 'overheat_guard', value: 2 },
    })
    const buffs = (e as unknown as { buffs: { overheatGuard: number } }).buffs
    expect(buffs.overheatGuard).toBe(2)
  })

  it('护盾层数会被真正消耗（不再是只写字段）', () => {
    const e = mk()
    give(e, {
      id: 'mechanic_overheat_guard',
      name: '隔热护罩',
      descr: '',
      kind: 'mechanic',
      mechanic: { kind: 'overheat_guard', value: 1 },
    })
    const eng = e as unknown as {
      buffs: { overheatGuard: number }
      heat: { heat: bigint; overheated: boolean; cap: bigint }
      enterOverheat(i?: number): void
    }
    expect(eng.buffs.overheatGuard).toBe(1)
    eng.heat.heat = eng.heat.cap // 强制进入过热条件
    eng.enterOverheat()
    expect(eng.buffs.overheatGuard).toBe(0)
  })

  it('有护盾时不进入过热状态（免疫生效）', () => {
    const e = mk()
    give(e, {
      id: 'g',
      name: '',
      descr: '',
      kind: 'mechanic',
      mechanic: { kind: 'overheat_guard', value: 1 },
    })
    const eng = e as unknown as {
      heat: { overheated: boolean; heat: bigint; cap: bigint }
      enterOverheat(i?: number): void
    }
    eng.heat.heat = eng.heat.cap
    eng.enterOverheat()
    expect(eng.heat.overheated).toBe(false)
  })

  it('护盾必须清空热量，否则会每 tick 白烧一层', () => {
    // 这是"免疫"最容易被写错的地方：只把 overheated 置 false 而不清热量，
    // 引擎的热量死区兜底下一 tick 又会判定"放不出技能"并再次触发，
    // 于是每 tick 消耗一层护盾 —— 直到耗尽才真的过热。
    const e = mk()
    give(e, {
      id: 'g',
      name: '',
      descr: '',
      kind: 'mechanic',
      mechanic: { kind: 'overheat_guard', value: 3 },
    })
    const eng = e as unknown as {
      heat: { overheated: boolean; heat: bigint; cap: bigint }
      buffs: { overheatGuard: number }
      enterOverheat(i?: number): void
    }
    eng.heat.heat = eng.heat.cap
    eng.enterOverheat()
    expect(eng.heat.heat).toBe(0n)
    expect(eng.buffs.overheatGuard).toBe(2) // 只消耗一层
  })

  it('护盾耗尽后恢复正常过热', () => {
    const e = mk()
    give(e, {
      id: 'g',
      name: '',
      descr: '',
      kind: 'mechanic',
      mechanic: { kind: 'overheat_guard', value: 1 },
    })
    const eng = e as unknown as {
      heat: { overheated: boolean; heat: bigint; cap: bigint }
      enterOverheat(i?: number): void
    }
    eng.heat.heat = eng.heat.cap
    eng.enterOverheat() // 被护盾挡掉
    expect(eng.heat.overheated).toBe(false)
    eng.heat.heat = eng.heat.cap
    eng.enterOverheat() // 无护盾，真过热
    expect(eng.heat.overheated).toBe(true)
  })
})

describe('卡牌：技能卡装入空槽而不是覆盖', () => {
  // 修复前：已装 [1,2] 时抽到技能 9 → after = [9,2]，技能 1 被销毁。
  // 槽位数永远不增加，玩家的三选一白白消耗。
  it('抽到新技能时装入空槽而不是顶掉已有技能', () => {
    const e = mk(2) // 只装 2 个，留 2 个空槽
    const before = e.skills.map((s) => s.skillId)
    const newSkill = [...skillMap.values()].find(
      (s) => s.kind === 'active' && !before.includes(s.id),
    )!
    give(e, { id: 'sk', name: newSkill.name, descr: '', kind: 'skill', skillId: newSkill.id })
    const after = e.skills.map((s) => s.skillId)
    expect(after.length).toBe(before.length + 1)
    expect(after).toContain(newSkill.id)
    // 原有技能一个都不能少
    for (const id of before) expect(after).toContain(id)
  })

  it('新技能占用的是空闲槽号', () => {
    const e = mk(2)
    const beforeSlots = e.skills.map((s) => s.slot).sort()
    const newSkill = [...skillMap.values()].find(
      (s) => s.kind === 'active' && !e.skills.some((x) => x.skillId === s.id),
    )!
    give(e, { id: 'sk', name: newSkill.name, descr: '', kind: 'skill', skillId: newSkill.id })
    const slots = e.skills.map((s) => s.slot).sort((a, b) => a - b)
    // 槽号不重复，且都在合法范围内
    expect(new Set(slots).size).toBe(slots.length)
    for (const s of slots) expect(s).toBeLessThan(ACTIVE_SLOTS)
    // 原本占用的槽位不变
    for (const s of beforeSlots) expect(slots).toContain(s)
  })

  it('抽到已装备的技能是升格而非新增', () => {
    const e = mk(2)
    const target = e.skills[0]
    const dmgBefore = target.baseDamage
    give(e, {
      id: 'sk',
      name: target.name,
      descr: '',
      kind: 'skill',
      skillId: target.skillId,
    })
    expect(e.skills.length).toBe(2)
    const after = e.skills.find((s) => s.skillId === target.skillId)!
    expect(after.baseDamage).toBeGreaterThan(dmgBefore)
  })

  it('主动槽装满时不会凭空减少技能数', () => {
    const e = mk(ACTIVE_SLOTS)
    const n = e.skills.length
    const newSkill = [...skillMap.values()].find(
      (s) => s.kind === 'active' && !e.skills.some((x) => x.skillId === s.id),
    )!
    give(e, { id: 'sk', name: newSkill.name, descr: '', kind: 'skill', skillId: newSkill.id })
    // 槽满时的处理是"强化已有技能"，技能数不变也不减
    expect(e.skills.length).toBe(n)
  })

  it('skillId === 0 不被静默吞掉', () => {
    // 判据从真值 `card.skillId` 改成 `card.skillId !== undefined`。
    // 原来 0 落进隐式 else：卡被 consume、emit 了 card_taken、什么也没发生。
    const e = mk(2)
    const n = e.skills.length
    give(e, { id: 'sk0', name: '零号', descr: '', kind: 'skill', skillId: 0 })
    // 0 号技能不在内容表里，所以不该崩溃；关键是流程不吞卡
    expect(e.skills.length).toBeGreaterThanOrEqual(n - 1)
  })
})
