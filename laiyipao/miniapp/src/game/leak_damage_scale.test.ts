/**
 * 漏怪伤害的**量级实测** —— 解释「为什么整个防御属性层买不到东西」。
 *
 * ## 结论（2026-09-29 实测，本文件的断言就是这些数字的来源）
 *
 * 扫前 40 关，默认构筑，固定 seed：
 *
 * | 项 | 数值 |
 * |---|---|
 * | 漏怪次数 | **19**，只发生在 5/40 个关卡 |
 * | 护甲 0‰ 总掉血 | **574**（单关最大 286） |
 * | 护甲 750‰ 总掉血 | **142**（单关最大 71） |
 * | 前 40 关最大底血 | 2360 |
 * | 40 关总掉血占总血量 | 574 / 63776 = **0.90%** |
 *
 * 护甲从 0 到吃满封顶，只把 574 降到 142 —— **省 432 点，占总血量 0.68%**。
 * 这就是「护甲、装备护甲、专精 armor 节点、宝石 armor 词条全部买不到东西」
 * 的直接原因（`armor_shape.test.ts` 从另一个角度独立测到同一个 432）。
 *
 * ## 机制：两类漏怪，只有一类会痛
 *
 * `engine.ts` 的 `leakDamage` 有两条分支：
 *
 * ```ts
 * if (e.attack > 0n) return applyArmor(e.attack, ...)   // 有攻击力的：按攻击力结算
 * return applyArmor(this.breachDamage, ...)              // 无攻击力的：底血 / LeakDamageDivisor
 * ```
 *
 * 种子里 21 种敌人中 **14 种 `attack === 0`**（走兜底）、**7 种 `attack > 0`**。
 * 兜底分支按设计要给「无攻击力的杂兵」一个**与底血挂钩**的推进伤害，
 * 单次约等于底血的 1/10（第 1 关 100 点，第 40 关 236 点）—— 那是很痛的。
 *
 * **但前 40 关里漏掉的 19 只，拆开是这样的**：
 *
 * | | 次数 | 单次伤害 | 合计 |
 * |---|---|---|---|
 * | 有攻击力的输出位（走 `e.attack` 分支） | **17** | 约 6 点 | 约 100 |
 * | 零攻击力的杂兵（走 `breachDamage` 兜底） | **2** | 约 236 点（底血/10） | 约 472 |
 * | 合计 | 19 | 平均 30 点 | **574** |
 *
 * **2 次杂兵漏怪贡献了约 82% 的伤害，而 17 次输出位只贡献约 18%。**
 *
 * 所以问题不是「兜底分支是死代码」，而是：
 *
> > **会痛的敌人太稀疏。** 20 次漏怪里只有 2 只杂兵漏到线。
> >
> > 而护甲减的是**每一次**漏怪的伤害，那 17 次每人 6 点的小伤害
> > 把它的效果彻底摊平了 —— 574 里有 472 来自那两次，剩下 102 被护甲
> > 按比例削减，看着就只剩 142。
>
> 这也解释了为什么「护甲强不强」和「这局痛不痛」几乎脱钩：
> **决定痛感的是杂兵漏不漏，而不是输出位漏几个。**
 *
 * ## ⚠️ 我在这件事上连错两次，都记在这里
 *
 * ### 第一次（第 45 轮）：用反推的数字下结论
 *
 * 我把现象解释成「19 次漏怪倒推单次约 45 点、每关底血约 1594、
 * **一次漏怪只掉 2.8%**」。三个数**全是错的**，而且错法一致 —— 都是反推：
 *
 * | 我写的 | 实测 |
 * |---|---|
 * | 底血约 1,594 | 底血 1000 → **2,360**（第 40 关） |
 * | 单次约 45 点 | 19 次共 574 → 平均 **30** 点 |
 * | 一次掉 2.8% | 40 关**总共**掉 0.90% |
 *
 * ### 第二次（第 50 轮）：读了不存在的字段，然后当成结构性发现
 *
 * 我写了个探针统计「有多少种敌人 `base_damage === 0`」，
 * 得到「0 种」，于是宣布「**`breachDamage` 兜底分支是死代码，一行都没执行过**」，
 * 还把它当成一条「有价值的结构性发现」写进了文件头。
 *
 * **`EnemyDef` 上根本没有 `base_damage` 这个字段**（真名是 `attack`）。
 * `undefined === 0` 为假，于是 21 种敌人**全部**落进「> 0」桶。
 *
 * 改成读 `attack` 之后，真实数字是 **14 种为 0** —— 与我的结论**正好相反**：
 * 兜底分支非但不是死代码，它才是设计上的主要分支。
 *
 * > 一个不存在的字段不会报错，它只会让每一个值都变成 `undefined`，
 * > 而 `undefined` 恰好满足我随手写的那个判据。
> > **「得到一个干净的结果」不等于「结果正确」** ——
> > 尤其是当判据里出现了一个你没验证过的字段名。
 *
 * 两次的共同教训：**先确认字段与语义真实存在，再解释现象。**
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { equippedFromSnapshot, type BuildSnapshot } from './replay'
import { ACTIVE_SLOTS } from './heatmap'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>((fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]))
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function buildSnapshot(): BuildSnapshot {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  const map: Record<string, unknown> = {}
  active.forEach((s, i) => {
    map[String(s.id)] = {
      id: s.id, name: s.name, family: 't', element: s.element, kind: s.kind, slot: i, level: 1,
    }
  })
  return { skills: map, attacker: { attack: 0 }, active_slots: ACTIVE_SLOTS } as unknown as BuildSnapshot
}

interface Measured {
  leaks: number
  damage: bigint
  levelsWithDamage: number
  totalBaseHp: bigint
  /** 「单次掉血 ≥ 底血 8%」的漏怪次数 —— 即走了 breachDamage 兜底分支的那些 */
  breachHits: number
  /** 其余的：attack > 0 的敌人，只掉自己那点攻击力 */
  attackerHits: number
}

/** 跑指定关卡，统计漏怪次数、总掉血，并按「掉血量级」把两类漏怪分开。 */
function measure(ids: number[], armorPermille: bigint): Measured {
  let leaks = 0
  let damage = 0n
  let levelsWithDamage = 0
  let totalBaseHp = 0n
  let breachHits = 0
  let attackerHits = 0
  for (const id of ids) {
    const level = levels.find((l) => l.id === id)!
    const baseHp = BigInt(level.base_hp)
    totalBaseHp += baseHp
    const cfg: BattleConfig = {
      level,
      enemies,
      skills,
      equipped: equippedFromSnapshot(buildSnapshot(), skills),
      attacker: { ...defaultAttacker(), armorPermille },
      seed: 12345,
      activeSlots: ACTIVE_SLOTS,
    }
    const e = new BattleEngine(cfg)
    e.start()
    for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') e.skipCards()
      e.step()
    }
    leaks += e.leaked
    const drop = e.baseHpMax - e.baseHp
    damage += drop
    if (drop > 0n) levelsWithDamage++
    // 单关内按比例拆：breachDamage = base_hp / 10，取「≥ 8%」为兜底的判据
    if (e.leaked > 0 && drop > 0n) {
      const perLeak = Number(drop) / e.leaked
      const breach = Number(baseHp) / 10
      if (perLeak >= breach * 0.8) breachHits += e.leaked
      else attackerHits += e.leaked
    }
  }
  return { leaks, damage, levelsWithDamage, totalBaseHp, breachHits, attackerHits }
}

const first40 = levels.slice(0, 40).map((l) => l.id)
const noArmor = measure(first40, 0n)

/** 跑一关，返回它有没有掉血。用来找出「值得用别的护甲再跑一遍」的关卡。 */
function leaksAt(id: number): boolean {
  const cfg: BattleConfig = {
    level: levels.find((l) => l.id === id)!,
    enemies,
    skills,
    equipped: equippedFromSnapshot(buildSnapshot(), skills),
    attacker: defaultAttacker(),
    seed: 12345,
    activeSlots: ACTIVE_SLOTS,
  }
  const e = new BattleEngine(cfg)
  e.start()
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return e.baseHpMax > e.baseHp
}

// ⚠️ 第二趟**只跑有掉血的关卡**，不是把 40 关重跑一遍。
//
// 满封顶护甲那组只用来取两个数：总掉血 142，以及与 0‰ 的差 432。
// 而掉血只发生在 5 个关卡上 —— 跑那 35 个零掉血的关卡纯属浪费。
//
// 这与第 35 轮「消除 16 次重复全量扫描」是同一条道理：
// **重复的全量扫描是纯成本**，除非它在测别的东西。
//
// ⚠️ 但收益要**实测**。我第一版在这里写的是「全量墙钟 48s → 约 37s」——
// 那是**外推**，不是测量。实测：
//
//   单独跑本文件   19s   → 2.5s
//   miniapp 全量   47.9s → 46.7s   （只省 1.2s）
//
// 全量几乎没变 —— 并行执行时本文件不是瓶颈，
// 它那 19 秒本来就和别的文件重叠掉了。
//
// > 单独一个文件变快，不等于整个套件变快。
// > 这与 R47「用 dist 总量代替首屏」同类：**拿容易测的量代替真正关心的量。**
const fullArmor = measure(first40.filter(leaksAt), 750n)
const enemyTypes = [...enemies.values()]

describe('漏怪伤害的量级', () => {
  it('前提守卫：敌人表非空，且确实分两类（零攻击力 / 有攻击力）', () => {
    // 这一条是为了钉住我踩过两次的坑。
    //
    // 第 50 轮我读了一个**不存在的字段** `base_damage`，
    // 于是 21 种敌人全部落进「> 0」桶，我据此宣布「兜底分支是死代码」。
    // 真实分布是 14 种为 0、7 种大于 0 —— **结论正好相反**。
    //
    // 所以这里显式断言「两类都非空」。若哪天真的只剩一类，
    // 这条会红 —— 那时才是「某一类消失」，而不是「我读错了字段」。
    const zero = enemyTypes.filter((d) => d.attack === 0)
    const nonzero = enemyTypes.filter((d) => d.attack > 0)
    expect(enemyTypes.length, '敌人表是空的 —— 下面的量级断言毫无意义').toBeGreaterThan(10)
    expect(zero.length, '没有任何零攻击力的敌人 —— 兜底分支失去对象').toBeGreaterThan(0)
    expect(nonzero.length, '没有任何有攻击力的敌人 —— 按攻击力结算那条分支成了死代码').toBeGreaterThan(0)
  }, 300000)

  it('零攻击力敌人占多数（14/21），它们走的是「底血除以 10」的兜底', () => {
    // 这是「会痛的敌人」那一类。它们的单次漏怪代价约等于底血的 1/10。
    const zero = enemyTypes.filter((d) => d.attack === 0)
    expect(
      zero.length,
      `零攻击力敌人实测 ${zero.length}/${enemyTypes.length} 种，` +
        '与「14 种」不符 —— 内容表变了，兜底分支的覆盖面也变了',
    ).toBe(14)
  }, 300000)

  it('满封顶护甲只把 40 关总掉血从 574 降到 142', () => {
    // 与 armor_shape.test.ts 的独立测量交叉验证：那边算的是
    // 「40 关合计剩血之差」，这边算的是「直接累加每次掉血」，
    // 两条独立路径得到同一个 432。
    expect(noArmor.leaks).toBe(19)
    expect(noArmor.damage).toBe(574n)
    expect(fullArmor.damage).toBe(142n)
    expect(noArmor.damage - fullArmor.damage).toBe(432n)
  }, 300000)

  it('19 次漏怪里只有 2 次走了兜底，但它们贡献了约 82% 的伤害', () => {
    // 本文件真正的结论，也是 R45「护甲买不到东西」现象的根因。
    //
    // 第一次写这条时我断言「兜底分支一次都没被走到」（= 0），
    // 实测是 **2** —— 又是一个我凭印象填的数。改正。
    //
    // 拆开看更说明问题：
    //   17 次输出位漏怪，每次约 6 点（它们只掉自己的攻击力）
    //    2 次杂兵漏怪，每次约 236 点（底血 / 10）
    //   → 2 次占了 574 里的约 472，即 **82%**
    //
    // 所以「漏怪不痛」不是因为漏得少，而是因为**痛的那两次太稀疏**：
    // 20 次漏怪里只有 2 只杂兵漏到线。
    // 而护甲减的是**每一次**漏怪的伤害，它的作用被那 17 次小伤害摊薄了。
    expect(
      noArmor.breachHits,
      `前 40 关走 breachDamage 兜底的漏怪实测 ${noArmor.breachHits} 次（基线 2）—— ` +
        '若杂兵开始更多地漏到线，「痛但稀疏」这个结构就变了',
    ).toBe(2)
    expect(noArmor.attackerHits).toBe(17)
    expect(noArmor.breachHits + noArmor.attackerHits).toBe(noArmor.leaks)
  }, 300000)

  it('漏怪只发生在 5 个关卡，总掉血占总血量 0.90%', () => {
    // 「防御属性层为什么是死的」的直接证据。
    // 阈值 2%：若平衡改动让漏怪真的有分量，这条会红 ——
    // 那是一次有意的改动，需要连带重测平衡守卫。
    expect(noArmor.levelsWithDamage).toBe(5)
    const permille = Number((noArmor.damage * 10000n) / noArmor.totalBaseHp)
    expect(permille, `总掉血占比 ${permille / 100}%，超过 2% 阈值`).toBeLessThan(200)
  }, 300000)

  it('单次漏怪的平均伤害远小于杂兵兜底值', () => {
    // 19 次共 574 → 平均 30；而底血除以 10 在第 40 关是 236。差 8 倍。
    const avg = Number(noArmor.damage) / noArmor.leaks
    const maxBaseHp = Math.max(...first40.map((id) => levels.find((l) => l.id === id)!.base_hp))
    expect(avg).toBeLessThan(maxBaseHp / 10)
    expect(avg, `平均单次漏怪伤害 ${avg.toFixed(1)}，超过 60 阈值（实测 30.2）`).toBeLessThan(60)
  }, 300000)
})
