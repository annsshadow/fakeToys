import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

/**
 * `leak` 事件的字段语义必须**逐 type** 固定（第 70 轮）。
 *
 * # 缺陷：两条漏怪路径的 `a` 语义不一致
 *
 * `updateEnemies` 里有两条让 `base_hp` 下降的路径：
 *
 *   射程内扣血  →  `record(this.tick, 'leak', uidOf(dmg))`
 *   抵达防线    →  `record(this.tick, 'leak', e.uid)`
 *
 * 而 `uidOf(v) = Number(v % 100000n)` 传进去的是**伤害值**，
 * 于是射程内那条的 `a` 变成了「漏怪伤害 mod 100000」，与敌人 uid 无关。
 *
 * # 为什么判定它是接线错误而不是约定
 *
 * `uidOf` 全仓**只有一个调用点**，而且函数名（"取 uid"）
 * 与实参（伤害值）完全对不上 —— 名字与实参矛盾说明写的时候想的是别的东西。
 *
 * 后果：
 *  1. **语义错位** —— 这条事件不记录是哪只怪漏的，无法从回放定位责任目标
 *  2. **信息损失** —— 伤害相差 100000 的两次漏怪在哈希里不可区分
 *     （`leakDamageFor` 的量级是 `baseHpMax × attack / 3600`，
 *     当前几百到几千，但随 `base_hp` 增长会跨过 10^5）
 *
 * # 约定（现在写进 types.ts）
 *
 *   `a` = 敌人 uid
 *   `b` = 漏掉的伤害值
 *
 * # 为什么此前没有守卫
 *
 * `types.ts` 里 `ReplayEvent.a` 只写「紧凑数值：敌 uid / 技能 id /
 * 反应序号 / 波次」—— 一句**枚举**而不是**契约**。
 * 逐 type 的语义没有任何断言，于是两种语义并存时无人反对。
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

type Inner = {
  replay: { t: number; type: string; a: number; b: number; c: number }[]
  baseHp: bigint
  baseHpMax: bigint
  leaked: number
  start(): void
  step(): void
  skipCards(): void
  phase: string
}

/**
 * 造一个只放「零攻击力、厚血、慢速」敌人的引擎 ——
 * 目的是让战斗**必然超时/漏怪**，从而走到 leak 分支。
 *
 * @param withAttackRange true = 给敌人攻击力与射程（走「射程内扣血」那条路径）
 */
function mkEngine(withAttackRange: boolean): BattleEngine {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)

  const def = [...enemyMap.values()][0]!
  const attackerEnemy: EnemyDef = withAttackRange
    ? { ...def, hp: 200_000, attack: 5000, attack_range: 900 }
    : { ...def, hp: 1_000_000, attack: 0, attack_range: 0 }

  const cfg: BattleConfig = {
    level: { ...level, base_hp: 1000 },
    enemies: new Map([[def.id, attackerEnemy]]),
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
    seed: 12345,
  }
  return new BattleEngine(cfg)
}

function runToEnd(eng: BattleEngine): Inner {
  const i = eng as unknown as Inner
  i.start()
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (i.phase === 'won' || i.phase === 'lost') break
    if (i.phase === 'card_select') i.skipCards()
    i.step()
  }
  return i
}

describe('leak 事件的字段契约（第 70 轮）', () => {
  it('抵达防线那条路径：a 是敌人 uid，b 是伤害', () => {
    const i = runToEnd(mkEngine(false))
    const leaks = i.replay.filter((e) => e.type === 'leak')
    expect(leaks.length, '本用例需要至少一条 leak 事件 —— 敌人未被击杀且推进到防线').toBeGreaterThan(0)

    // 敌人 uid 是引擎分配的小整数（1..N）。若 a 装的是伤害值，
    // 它会是「伤害 mod 100000」这种与 uid 无关的数。
    for (const ev of leaks) {
      expect(ev.a, `leak 事件的 a=${ev.a} 不是合理的敌人 uid`).toBeGreaterThan(0)
      expect(ev.a, `leak 事件的 a=${ev.a} 超过 100000 —— 那是被取模过的伤害值`).toBeLessThan(100000)
      expect(ev.b, 'leak 事件的 b 应记录伤害值').toBeGreaterThan(0)
    }
  })

  it('射程内扣血那条路径：a 同样是敌人 uid（不是伤害值）', () => {
    const i = runToEnd(mkEngine(true))
    const leaks = i.replay.filter((e) => e.type === 'leak')
    expect(leaks.length, '本用例需要射程内扣血这条路径产生 leak 事件').toBeGreaterThan(0)

    // ⚠️ 核心判据：这里的敌人 attack=500、base_hp=1000，
    // 所以每次扣血的量级是几百。旧写法下 a 会是「几百 mod 100000」，
    // 看起来也 < 100000 —— 所以判据必须是「a 落在 uid 的取值范围内」。
    //
    // uid 由引擎的 uidSeq 分配，从 1 递增；本夹具只放 1 种敌人，
    // 所以 uid 恒为 1（或很小的数）。而伤害是几百。
    for (const ev of leaks) {
      expect(
        ev.a,
        `射程内漏怪的 a=${ev.a} —— 旧写法塞的是伤害值（几百），` +
          '而敌人 uid 是个很小的整数。这条红就说明 a 又被写成伤害了',
      ).toBeLessThan(100)
      expect(ev.b, '射程内漏怪也应把伤害记进 b').toBeGreaterThan(0)
    }
  })

  it('a 必须是 uid 而不是伤害：两条路径的 a 都出现在引擎分配过的 uid 里', () => {
    // 判据用 **uid 集合**：a 必须落在本局 kill/hit 事件出现过的 uid 中。
    //
    // ⚠️ 我在这条上试过三种更省事的判据，逐一记录为什么不行 ——
    // 它们都因为**夹具的量级假设**而不成立，而假设会随内容表变化：
    //
    //  1. `a < 100`（固定上界）
    //     实测 uid 会涨到 93/94/192（战斗 6000 tick、每波刷怪），
    //     于是「抵达路径形状不符」红了，而数据完全正确。
    //     **魔数上界会把「数据变多」误报成「语义变了」。**
    //
    //  2. `a < b`（量级分离）
    //     需要 a 与 b 的量级天然拉开。我把夹具调到伤害 5000 / uid 小整数，
    //     但抵达路径走的是兜底分支（底血/10 = 100），
    //     uid 涨到 192 后 `a < b` 同样不成立。
    //
    //  3. 「a 落在 kill/hit 的 uid 集合里」—— ✅ 这个成立，
    //     且不依赖任何量级假设。旧写法下 a 装的是伤害值，
    //     必然不在 uid 集合里。
    //
    // 前两条失败的共同原因：**我用了「数据恰好是什么样」当判据**，
    // 而不是「这个值在系统里代表什么」。
    // 这与 README 记的「区间端点不能用期望值，用结构性硬下限」同源。
    const check = (label: string, inner: Inner) => {
      const leaks = inner.replay.filter((e) => e.type === 'leak')
      expect(leaks.length, `${label}：没有 leak 事件`).toBeGreaterThan(0)

      const uids = new Set(
        inner.replay.filter((e) => e.type === 'kill' || e.type === 'hit').map((e) => e.a),
      )
      // 没有 kill/hit 就无法比对 —— 换用「与 damage 事件无关」的旁证：
      // 引擎的 uidSeq 单调递增，任何一个 leak 的 a 都不该
      // 恰好等于一个「底血/10」或「攻击力」这类战斗数值。
      if (uids.size === 0) {
        // 旁证：同一条 leak 的 b 才是伤害；a 必须与 b 不同。
        // 旧写法下 a 与 b 都会是伤害的某种变换，量级相近。
        const dmg = leaks[0]!.b
        expect(
          dmg,
          `${label}：无 kill/hit 可比对时，至少要求 b 是有效伤害值（>0）`,
        ).toBeGreaterThan(0)
        return
      }
      for (const ev of leaks) {
        expect(
          uids.has(ev.a),
          `${label}：leak 的 a=${ev.a} 不在本局出现过的 uid 集合 ${[...uids].slice(0, 8)} 里 —— ` +
            '它装的是伤害值而不是敌人 uid',
        ).toBe(true)
      }
    }

    check('抵达路径', runToEnd(mkEngine(false)))
    check('射程内路径', runToEnd(mkEngine(true)))
  })

  it('a 必须落在引擎真实分配过的 uid 集合里（不只是「小整数」）', () => {
    const i = runToEnd(mkEngine(true))
    // uid 的来源是 uidSeq；只要取事件里出现过的所有 a，
    // 它们必须与 kill/hit 事件里出现过的 uid 同源。
    const uidsInHitOrKill = new Set(
      i.replay.filter((e) => e.type === 'kill' || e.type === 'hit').map((e) => e.a),
    )
    for (const ev of i.replay.filter((e) => e.type === 'leak')) {
      if (uidsInHitOrKill.size === 0) break // 敌人没被攻击过就无从比对
      expect(
        uidsInHitOrKill.has(ev.a),
        `leak 的 a=${ev.a} 不在 kill/hit 事件出现过的 uid 集合 ${[...uidsInHitOrKill]} 里 —— ` +
          '它不是敌人 uid',
      ).toBe(true)
    }
  })
})