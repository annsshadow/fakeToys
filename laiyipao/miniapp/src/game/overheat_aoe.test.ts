import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, SPAWN_PROGRESS_FULL, TICK_MS, type BattleConfig } from './engine'
import { toFixed } from './terrain'
import { defaultAttacker } from './damage'
import type { EquippedSkill } from './heatmap'
import type { Enemy, EnemyDef, GeneratedLevel, Projectile, SkillDef } from './types'
import type { Element } from './elements'

/**
 * overheat 的爆炸溅射（第 160 轮）。
 *
 * # 口径（与 steam_burst 同管线，半径读各自表项）
 *
 * - 命中敌人为圆心、半径 90px 内的存活邻居受 `reactionDmg >> 2`（25%）
 * - 眩晕只作用主目标，**不随溅射扩散**
 * - 溅射不施加元素层数、不二次触发反应
 *
 * # 为什么这条必须存在
 *
 * `engine_pure_vectors.json` 里唯一覆盖 overheat 溅射的是 `card_multiwave`
 * 场景，而那份向量由 TS 实现自己生成 —— 半径若写错（比如误用 steam 的
 * 120），重新生成演一遍就会把错误「洗白」进契约文件。
 * 本用例把邻居摆在两条半径之间（100px）：90 打不到、120 打得到，
 * 「半径读的是哪条反应」当场可分辨。
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
  /** 真实签名是 (enemyId, uid)，敌人被 push 进 this.enemies。 */
  spawnEnemy(enemyId: number, uid: number): void
  updateProjectiles(): void
}

/**
 * 构造一个只带一颗「火」弹丸的引擎：走真实投射体路径触发 overheat。
 *
 * 元素必须是火 —— 已附着「电」的敌人被火命中才会配对出 overheat
 * （`REACTION_TABLE.fire.lightning`）。
 */
function mkEngine(): BattleEngine {
  const fireSkill = [...skillMap.values()]
    .filter((s) => s.kind === 'active' && s.element === 'fire')
    .sort((a, b) => a.id - b.id)[0]!

  const equipped: EquippedSkill[] = [
    {
      skillId: fireSkill.id,
      name: fireSkill.name,
      element: fireSkill.element as Element,
      kind: fireSkill.kind,
      heatCost: BigInt(fireSkill.heat_cost),
      cooldownMs: fireSkill.cooldown_ms,
      pierce: 0,
      aoeRadius: 0,
      baseDamage: BigInt(fireSkill.base_damage),
      applyElement: (fireSkill.apply_element ?? '') as Element | '',
      applyStacks: BigInt(fireSkill.apply_stacks),
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

/** 在指定位置放一颗弹丸，朝 +x 高速飞行（每子步 20 单位）。 */
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

describe('overheat 爆炸溅射（第 160 轮）', () => {
  it('半径 90 内的邻居吃 25% 伤害、不眩晕、不回传元素', () => {
    const eng = mkEngine()
    const inner = eng as unknown as EngineInternals
    const origin = placeEnemy(eng, 300, 500, 1)
    const near = placeEnemy(eng, 350, 500, 2) // 50px
    const far = placeEnemy(eng, 400, 500, 3) // 100px：steam 120 覆盖、overheat 90 不覆盖

    // 已附着「电」+ 命中「火」→ overheat（REACTION_TABLE.fire.lightning）
    origin.stacks.set('lightning', 1n)

    fireAt(eng, 300, 500)
    fly(eng)

    expect(inner.reactionsCount, '主目标命中应触发 overheat；溅射不得二次触发反应').toBe(1)
    expect(origin.hp, '主目标必须被命中').toBeLessThan(1_000_000n)
    expect(origin.stunnedMs, 'overheat 的眩晕应作用在主目标').toBeGreaterThan(0)
    expect(near.hp, '50px 邻居应吃到 25% 溅射伤害').toBeLessThan(1_000_000n)
    expect(near.stunnedMs, '眩晕只作用主目标，不随溅射扩散（第 160 轮口径）').toBe(0)
    expect(
      far.hp,
      '100px 在 overheat 半径（90）之外 —— 吃到伤害说明半径误用了 steam_burst 的 120',
    ).toBe(1_000_000n)
    expect(near.stacks.get('fire') ?? 0n, '溅射不施加元素层数').toBe(0n)
  })
})
