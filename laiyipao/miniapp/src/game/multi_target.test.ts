import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, SPAWN_PROGRESS_FULL, TICK_MS, type BattleConfig } from './engine'
import { toFixed } from './terrain'
import { defaultAttacker } from './damage'
import type { EquippedSkill } from './heatmap'
import type { Enemy, EnemyDef, GeneratedLevel, Projectile, SkillDef } from './types'
import type { Element } from './elements'

/**
 * 一颗弹丸必须能命中**多个**敌人（第 65 轮）。
 *
 * # 缺陷 C-03：弹射目标进了 hitSet 却从未结算伤害
 *
 * `checkProjectileHit` 的弹射分支：
 *
 *     const next = <离弹丸最近的未命中敌人>
 *     if (next) {
 *       p.chainLeft--
 *       p.x = next.x; p.y = next.y
 *       p.hitSet.add(next.uid)     // ← 加进已命中集合
 *       continue                    // ← 只是数组游标前进
 *     }
 *
 * `continue` 不会回头去结算 `next`。而 `next` 已经被写进 `hitSet`，
 * 下一轮循环开头的 `if (p.hitSet.has(e.uid)) continue` 会跳过它。
 *
 * 结果：**弹射只消耗 chainLeft、只位移弹丸，目标一点伤害都吃不到**，
 * 并且被永久排除在这颗弹丸之外。连锁闪电 / 电弧弹这一整类「多目标」技能
 * 的定位完全失效 —— chainLeft 只是空转。对 aoeRadius=0 的纯 chain 技能
 * 等于单发技能。
 *
 * # 为什么从没有任何测试变红
 *
 * engine.test.ts 的夹具把 `chain: 0` 写死 —— 整条多目标路径被从测试矩阵里删掉了。
 * 而 balance.probe 断言的是「能不能通关 / 地形生效率」，
 * 机制整体失效时它照样绿。
 *
 * # 判据
 *
 * 不是「chain 属性存在」，不是「调用了某函数」，
 * 而是**一颗弹丸让 ≥2 个敌人掉血** —— 只有行为断言能回答这个。
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
  shots: number
  projectiles: Projectile[]
  hits: number
  skills: EquippedSkill[]
  /** 真实签名是 (enemyId, uid)，敌人被 push 进 this.enemies。 */
  spawnEnemy(enemyId: number, uid: number): void
  updateProjectiles(): void
}

/**
 * 构造一个只带一颗弹丸的引擎，敌人位置由调用方指定。
 *
 * 直接构造弹丸并调 updateProjectiles —— 因为要测的正是
 * 「一颗弹丸穿过多个敌人」这条路径，走完整波次会引入刷怪节奏的干扰。
 */
function mkEngine(opts: { chain: number; pierce: number; aoeRadius: number }): {
  eng: BattleEngine
  inner: EngineInternals
} {
  const base = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)[0]!

  const equipped: EquippedSkill[] = [
    {
      skillId: base.id,
      name: base.name,
      element: base.element as Element,
      kind: base.kind,
      heatCost: BigInt(base.heat_cost),
      cooldownMs: base.cooldown_ms,
      pierce: opts.pierce,
      aoeRadius: opts.aoeRadius,
      baseDamage: BigInt(base.base_damage),
      applyElement: (base.apply_element ?? '') as Element | '',
      applyStacks: BigInt(base.apply_stacks),
      projectileSpeed: 1_000_000, // 极快：一 tick 内飞完全程，便于单点观测
      chain: opts.chain,
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
  const eng = new BattleEngine(cfg)
  return { eng, inner: eng as unknown as EngineInternals }
}

/**
 * 放一只满血、已完成刷刷新的敌人，并定位到指定坐标。
 *
 * `spawnEnemy(enemyId, uid)` 自己会随机 y，所以这里显式覆写 x/y。
 * hp 抬到 100 万是为了让「掉血」可观测而不致死 —— 死了也掉血，
 * 但 `e.dead` 会让后续断言里「下一轮 hitSet 跳过它」这条路径更难分辨。
 */
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

/**
 * 在指定位置放一颗弹丸，朝 +x 高速飞行。
 *
 * vx 取一个 tick 位移 200 单位 —— 足够一步跨过测试里 100~150 单位的间距，
 * 又不至于一步飞出整个场。⚠️ 不能省 vx/vy：弹丸靠速度推进，
 * vx=0 时它永远不动，整条命中路径都走不到。
 */
/**
 * 让弹丸飞 N 个 tick。
 *
 * ⚠️ **必须跑多个 tick**，不能只调一次 updateProjectiles 就断言。
 *
 * 真实对局里一发弹丸要飞几十个 tick 才能穿过敌群。只跑一次时：
 *   - pierce 只会有 1 次命中机会 → 「pierce=3 打 4 只」必然失败，
 *     而那**不是缺陷**，是夹具把射程压成了一个 tick
 *   - 第一版夹具正是这样，pierce 用例红得莫名其妙，
 *     差点被我当成第三个缺陷记进去
 *
 * 射程取 200 单位/tick × 12 tick，足够穿过测试里 700 单位的跨度。
 */
function fly(eng: BattleEngine, ticks = 12): void {
  const inner = eng as unknown as EngineInternals
  for (let i = 0; i < ticks; i++) {
    if (inner.projectiles.length === 0) return
    inner.updateProjectiles()
  }
}

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

describe('一颗弹丸命中多个敌人（第 65 轮）', () => {
  it('弹射：chain=2 的弹丸必须打到 3 只敌人', () => {
    const { eng } = mkEngine({ chain: 2, pierce: 0, aoeRadius: 0 })

    // 三只敌人沿 x 轴排开，间距 40 单位 —— 大于 withinEnemy 的 28 单位命中半径，
    // 所以「主目标命中」靠弹丸位置，弹射目标靠 chain 跳转。
    const a = placeEnemy(eng, 200, 500, 1)
    const b = placeEnemy(eng, 400, 500, 2)
    const c = placeEnemy(eng, 600, 500, 3)

    fireAt(eng, 200, 500)
    fly(eng)

    const damaged = [a, b, c].filter((e) => e.hp < 1_000_000n)
    expect(
      damaged.length,
      `弹射只打到 ${damaged.length} 只敌人 —— chain 分支把目标塞进 hitSet 就 continue，` +
        '伤害从未结算，而下一轮 hitSet.has 会把它跳过',
    ).toBe(3)
  })

  it('弹射：chain=1 的弹丸必须打到 2 只敌人', () => {
    const { eng } = mkEngine({ chain: 1, pierce: 0, aoeRadius: 0 })
    const a = placeEnemy(eng, 200, 500, 1)
    const b = placeEnemy(eng, 500, 500, 2)

    fireAt(eng, 200, 500)
    fly(eng)

    expect([a, b].filter((e) => e.hp < 1_000_000n).length).toBe(2)
  })

  it('穿透：pierce=3 的弹丸必须打到全部 4 只敌人', () => {
    const { eng } = mkEngine({ chain: 0, pierce: 3, aoeRadius: 0 })
    const es = [
      placeEnemy(eng, 200, 500, 1),
      placeEnemy(eng, 300, 500, 2),
      placeEnemy(eng, 400, 500, 3),
      placeEnemy(eng, 500, 500, 4),
    ]
    fireAt(eng, 200, 500)
    fly(eng)
    expect(es.filter((e) => e.hp < 1_000_000n).length).toBe(4)
  })

  it('单目标：chain=0 且 pierce=0 只打 1 只（守「不过度命中」）', () => {
    const { eng } = mkEngine({ chain: 0, pierce: 0, aoeRadius: 0 })
    const a = placeEnemy(eng, 200, 500, 1)
    const b = placeEnemy(eng, 500, 500, 2)
    fireAt(eng, 200, 500)
    fly(eng)
    expect([a, b].filter((e) => e.hp < 1_000_000n).length).toBe(1)
  })

  it('hits ≤ shots：溅射/弹射的多次结算不得抬高 hits（跨端强约束）', () => {
    // 服务端 `ValidateSettle` 拒 `in.Hits > in.Shots`（ErrInvalidHitRate），
    // 所以这条是**跨端契约**，客户端违约就是真的 422。
    //
    // 第 65 轮修好弹射/溅射后顺手在两个分支里加了 hits++，
    // 平衡探针立刻打出 hit%=145% / 190% —— 一发弹丸命中 3 只就算 3 次命中，
    // 而分母 shots 只有 1。
    const { eng, inner } = mkEngine({ chain: 3, pierce: 3, aoeRadius: 300 })
    placeEnemy(eng, 200, 500, 1)
    placeEnemy(eng, 260, 500, 2)
    placeEnemy(eng, 320, 500, 3)
    placeEnemy(eng, 380, 500, 4)
    placeEnemy(eng, 440, 500, 5)

    fireAt(eng, 200, 500)
    fly(eng)

    expect(
      inner.hits,
      `hits=${inner.hits} > 1 —— 一发弹丸的多次结算被当成了多次开火，` +
        '服务端会以 ErrInvalidHitRate 拒掉这局（422）',
    ).toBeLessThanOrEqual(1)
  })

it('AOE：aoeRadius=200 时距命中点 150 单位的敌人必须吃到伤害', () => {
    // 这一条针对**另一个**缺陷（C-02）：
    // applyAoe 按 aoeRadius 正确选了目标，随后调 hitEnemy(p, e)，
    // 而 hitEnemy 第一行是 `withinEnemy(e, p.x, p.y)` —— p.x/p.y 就是命中点，
    // 半径只有 28 单位。于是 aoe_radius 声明的 200 被二次裁成 28。
    const { eng } = mkEngine({ chain: 0, pierce: 0, aoeRadius: 200 })
    const a = placeEnemy(eng, 200, 500, 1)
    const far = placeEnemy(eng, 350, 500, 2) // 距命中点 150 单位，< 200 但 > 28

    fireAt(eng, 200, 500)
    fly(eng)

    expect(a.hp).toBeLessThan(1_000_000n)
    expect(
      far.hp,
      'AOE 声明半径 200，但距命中点 150 的敌人没吃到伤害 —— ' +
        'hitEnemy 的 withinEnemy(28 单位) 把 applyAoe 选出的目标又裁掉了',
    ).toBeLessThan(1_000_000n)
  })
})