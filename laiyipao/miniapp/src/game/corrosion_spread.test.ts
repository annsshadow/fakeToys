import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, SPAWN_PROGRESS_FULL, TICK_MS, type BattleConfig } from './engine'
import { toFixed } from './terrain'
import { defaultAttacker } from './damage'
import type { EquippedSkill } from './heatmap'
import type { Enemy, EnemyDef, GeneratedLevel, Projectile, SkillDef } from './types'
import type { Element } from './elements'

/**
 * corrosion_spread 的层数传播（2026-10-20，DECISIONS D2）。
 *
 * # 拍板口径（四条，全部在这里钉住）
 *
 * 1. 触发时把 `spreadStacks`（1）层腐蚀**直接写入**半径 `aoeRadius`（120px）
 *    内其他敌人的层数；
 * 2. **不造成伤害** —— 只写层数，不产生 hit / kill 事件、不消耗 RNG；
 * 3. **不再触发新反应** —— 直接写 stacks，不走 applyElement 的反应判定
 *    （否则每个被传播的敌人都可能立即触发新反应，传播链指数级扩散）；
 * 4. **clamp 到 elementCap** —— 与 applyElement 同一上限。
 *
 * # 为什么必须有一条行为用例
 *
 * 契约文件 `reaction_specs.json` 只能锁**数值**（aoe_radius / spread_stacks），
 * 锁不住「消费方式」：把传播实现成「走 applyElement」或「顺手算伤害」，
 * 数值全对、契约全绿，但机制语义完全不同（前者会连锁、后者白送伤害）。
 */

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

interface EngineInternals {
  enemies: Enemy[]
  projectiles: Projectile[]
  skills: EquippedSkill[]
  reactionsCount: number
  spawnEnemy(enemyId: number, uid: number): void
  updateProjectiles(): void
}

/**
 * 构造一个只带一颗「毒」弹丸的引擎。
 *
 * 元素必须是腐蚀（corrosion）：已附着「电」的敌人被毒命中才会配对出
 * corrosion_spread（`REACTION_TABLE.corrosion.lightning`）。
 */
function mkEngine(): BattleEngine {
  const poisonSkill = [...skillMap.values()]
    .filter((s) => s.kind === 'active' && s.element === 'corrosion')
    .sort((a, b) => a.id - b.id)[0]!

  const equipped: EquippedSkill[] = [
    {
      skillId: poisonSkill.id,
      name: poisonSkill.name,
      element: poisonSkill.element as Element,
      kind: poisonSkill.kind,
      heatCost: BigInt(poisonSkill.heat_cost),
      cooldownMs: poisonSkill.cooldown_ms,
      pierce: 0,
      aoeRadius: 0,
      baseDamage: BigInt(poisonSkill.base_damage),
      applyElement: (poisonSkill.apply_element ?? '') as Element | '',
      applyStacks: BigInt(poisonSkill.apply_stacks),
      projectileSpeed: 1_000_000,
      chain: 0,
      slot: 0,
      cooldownRemaining: 0,
    },
  ]

  const cfg: BattleConfig = {
    level,
    enemies: enemyMap,
    skills: skillMap,
    equipped,
    attacker: defaultAttacker(),
    seed: 1,
  }
  return new BattleEngine(cfg)
}

/** 放一只满血、已完成刷新的敌人，并定位到指定坐标（py 单位）。 */
function placeEnemy(eng: BattleEngine, x: number, y: number, uid: number): Enemy {
  const def = [...enemyMap.values()][0]!
  const inner = eng as unknown as EngineInternals
  inner.spawnEnemy(def.id, uid)
  const e = inner.enemies[inner.enemies.length - 1]!
  e.x = BigInt(x * 1000)
  e.y = BigInt(y * 1000)
  e.hp = 1_000_000n
  e.maxHp = 1_000_000n
  e.shield = 0n
  e.spawnProgress = SPAWN_PROGRESS_FULL
  return e
}

/** 在指定位置放一颗弹丸，朝 +x 高速飞行。 */
function fireAt(eng: BattleEngine, x: number, y: number, damage = 5_000n): void {
  const inner = eng as unknown as EngineInternals
  const s = inner.skills[0]!
  inner.projectiles.push({
    uid: 9000 + inner.projectiles.length,
    skillId: s.skillId,
    element: s.element,
    x: toFixed(x),
    y: toFixed(y),
    vx: toFixed(20) * BigInt(TICK_MS),
    vy: 0n,
    damage,
    applyStacks: s.applyStacks,
    aoeRadius: s.aoeRadius,
    chainLeft: s.chain,
    pierceLeft: s.pierce,
    hitSet: new Set<number>(),
    dead: false,
    trailLen: 3,
    slot: 0,
  })
}

/** 让弹丸飞若干 tick（首子步即命中主目标并熄灭）。 */
function fly(eng: BattleEngine, ticks = 12): void {
  const inner = eng as unknown as EngineInternals
  for (let i = 0; i < ticks; i++) {
    if (inner.projectiles.length === 0) return
    inner.updateProjectiles()
  }
}

describe('corrosion_spread 层数传播（2026-10-20）', () => {
  it('半径 120 内的邻居获得 1 层腐蚀，半径外不动', () => {
    const eng = mkEngine()
    const a = placeEnemy(eng, 100, 500, 1) // 主目标
    const b = placeEnemy(eng, 200, 500, 2) // 距 A 100px < 120
    const c = placeEnemy(eng, 400, 500, 3) // 距 A 300px > 120
    a.stacks.set('lightning', 1n) // 预置电层 → 毒弹命中触发 corrosion_spread

    fireAt(eng, 0, 500)
    fly(eng)

    // A 的腐蚀层来自技能施加（apply_element），不是传播
    expect(a.stacks.get('corrosion') ?? 0n).toBeGreaterThan(0n)
    // B 获得恰好 1 层（spread_stacks）
    expect(b.stacks.get('corrosion') ?? 0n).toBe(1n)
    // C 在半径外，不动
    expect(c.stacks.get('corrosion') ?? 0n).toBe(0n)
  })

  it('传播不造成任何伤害（只写层数）', () => {
    const eng = mkEngine()
    const a = placeEnemy(eng, 100, 500, 1)
    const b = placeEnemy(eng, 200, 500, 2)
    a.stacks.set('lightning', 1n)
    const bHp = b.hp
    const bShield = b.shield

    fireAt(eng, 0, 500)
    fly(eng)

    expect(b.hp).toBe(bHp)
    expect(b.shield).toBe(bShield)
  })

  it('传播不再触发新反应（邻居不会连锁爆开）', () => {
    const eng = mkEngine()
    const a = placeEnemy(eng, 100, 500, 1)
    const b = placeEnemy(eng, 200, 500, 2)
    a.stacks.set('lightning', 1n)
    // B 也预置电层：若传播走 applyElement，B 收到腐蚀层会**立即**再触发一次
    // corrosion_spread —— 传播链就会指数级扩散。
    b.stacks.set('lightning', 1n)

    fireAt(eng, 0, 500)
    fly(eng)

    const inner = eng as unknown as EngineInternals
    expect(inner.reactionsCount, '只应有 A 那一次反应，传播不得引发新反应').toBe(1)
  })

  it('层数 clamp 到 elementCap（不越界）', () => {
    const eng = mkEngine()
    const a = placeEnemy(eng, 100, 500, 1)
    const b = placeEnemy(eng, 200, 500, 2)
    a.stacks.set('lightning', 1n)
    // ⚠️ cap 的权威是**关卡字段** `lv.element_cap`（第 1 关是 2），
    // `defaultAttacker().elementCap`（3）只是关卡没配时的 fallback ——
    // 用后者写断言会得到「3 层却 clamp 到 2」的假失败（我踩过一次）。
    const cap = (eng as unknown as { currentAttacker(): { elementCap: bigint } })
      .currentAttacker().elementCap
    b.stacks.set('corrosion', cap) // 已满层

    fireAt(eng, 0, 500)
    fly(eng)

    expect(b.stacks.get('corrosion')).toBe(cap)
  })
})
