/**
 * 诊断：元素系数曲线的非单调性。
 *
 * ## 现象
 *
 * 100 关实测（可比口径），elementCoef 只改这一个字段：
 *
 *   elementCoef‰   漏怪    反应      分数
 *      0(基准)      486    14987    2801869
 *     400         1296    21180(+41%) 2663797(-4.9%)
 *     800          639    16356(+9%)  2757017(-1.6%)
 *    1200          452    14056(-6%)  2805978(+0.1%)
 *    1800          319    11833(-21%) 2840389(+1.4%)
 *
 * 反应次数在 400‰ 涨 41%，而漏怪翻 2.7 倍、分数掉 4.9%。
 * 「反应更多但打得更差」是自相矛盾的，除非**反应构成变了**。
 *
 * ## 假设
 *
 * H1「反应更频繁但更弱」：元素 DoT 提高 → 敌人死得更快 →
 *     攒不够层数触发高阶反应 → 低阶反应占比上升。
 *     若成立，`ReactionsUsed` 里高阶键的占比会下降。
 *
 * H2「漏怪统计口径问题」：与上一次「跨胜负不可比」同类。
 *     若成立，两档的胜负分布会不同。
 *
 * 本文件是**诊断脚本**（跑完保留为探针），判据是分布而不是单一汇总值。
 */
import { describe, it } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { defaultAttacker, type Attacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import { REACTIONS } from './elements'
import type { Element } from './elements'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function equipped() {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  return active.map((s, i) => ({
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
  }))
}

interface Probe {
  leak: number
  kills: number
  score: number
  reactions: number
  won: number
  lost: number
  /** 反应类型 → 次数 */
  byType: Map<string, number>
  /** 反应强度档（按 baseCoef 分桶）→ 次数 */
  byTier: Map<string, number>
}

function probe(tier: Partial<Attacker>): Probe {
  const out: Probe = {
    leak: 0,
    kills: 0,
    score: 0,
    reactions: 0,
    won: 0,
    lost: 0,
    byType: new Map(),
    byTier: new Map(),
  }
  for (const lv of levels) {
    const cfg: BattleConfig = {
      level: lv,
      enemies,
      skills,
      equipped: equipped(),
      attacker: { ...defaultAttacker(), ...tier },
      seed: 12345,
    }
    const e = new BattleEngine(cfg)
    e.start()
    let pick = 0
    for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') {
        const hand = (
          e as unknown as { deck: { hand: { id: string }[] }; takeCard(id: string): unknown }
        ).deck.hand
        if (hand.length > 0) {
          ;(e as unknown as { takeCard(id: string): unknown }).takeCard(hand[pick++ % hand.length].id)
        } else {
          e.skipCards()
        }
      }
      for (const ev of e.step()) {
        if (ev.type === 'reaction') {
          out.reactions++
          out.byType.set(ev.reaction, (out.byType.get(ev.reaction) ?? 0) + 1)
          const coef = REACTIONS[ev.reaction]?.baseCoef ?? 0
          const bucket = coef >= 60 ? 'strong(>=60)' : 'weak(<60)'
          out.byTier.set(bucket, (out.byTier.get(bucket) ?? 0) + 1)
        }
      }
    }
    out.leak += e.leaked
    out.kills += e.kills
    out.score += e.score
    if (e.phase === 'won') out.won++
    else if (e.phase === 'lost') out.lost++
  }
  return out
}

// ── 漏怪逐关观测：共享且带缓存 ─────────────────────────────────
//
// ⚠️ 这个函数**不能**和上面的 `probe` 合并 —— 两者口径不同：
//
//   probe(tier)      **真的取牌**（固定轮转），记反应构成 + 漏怪
//   leakedPerLevel()  `skipCards()`，                只记漏怪
//
// 差别不是笔误。`tier_impact.test.ts` 里记着同一个坑：
// 「一开始用 skipCards()，结果 mechanicPermille 测出来是完全无效 ——
// 而它其实生效，只是没有任何卡被取过，**没有东西可放大**」。
// 也就是说 `skipCards()` 下测出来的量属于另一个体系。
//
// 而本文件那个 1200‰ 封顶的结论正是**建立在这套 skipCards 口径**上的
// （见下面「守卫」那个 describe 的注释）。所以改口径 = 改结论，
// 数值会变、依据会失效。**保持现状，不要「顺手统一」。**
//
// 原来这里有**两份逐字相同的 20 行 `perLevel` 局部函数**（分别在
// 「决定性检验」与「封顶守卫」两个 `it` 里），外加 5 次全量扫描
// ——而互不相同的只有 3 份构筑。
const LEAK_TIERS = {
  base: {},
  c400: { elementCoefPermille: 400n },
  c1200: { elementCoefPermille: 1200n },
} satisfies Record<string, Partial<Attacker>>

const leakedCache = new Map<string, Map<number, number>>()

/** 返回的 Map 是**共享对象**，只读；需要改先 `new Map(...)` 复制。 */
function leakedPerLevel(key: keyof typeof LEAK_TIERS): Map<number, number> {
  let m = leakedCache.get(key)
  if (!m) {
    m = new Map<number, number>()
    for (const lv of levels) {
      const cfg: BattleConfig = {
        level: lv,
        enemies,
        skills,
        equipped: equipped(),
        attacker: { ...defaultAttacker(), ...LEAK_TIERS[key] },
        seed: 12345,
      }
      const e = new BattleEngine(cfg)
      e.start()
      for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        e.step()
      }
      m.set(lv.id, e.leaked)
    }
    leakedCache.set(key, m)
  }
  return m
}

describe('诊断：元素系数非单调', () => {
  it('对比 0‰ 与 400‰ 的反应构成、胜负分布', () => {
    const a = probe({})
    const b = probe({ elementCoefPermille: 400n })
    const c = probe({ elementCoefPermille: 1200n })

    const dump = (name: string, p: Probe) => {
      console.log(`[bal-ecoef] ${name}`)
      console.log(`[bal-ecoef]     won=${p.won} lost=${p.lost}`)
      console.log(`[bal-ecoef]     杀=${p.kills} 漏=${p.leak} 反应=${p.reactions} 分=${p.score}`)
      const tiers = [...p.byTier.entries()].sort((x, y) => x[0].localeCompare(y[0]))
      for (const [tier, n] of tiers) {
        console.log(
          `[bal-ecoef]     tier${tier}: ${n} (${((n / p.reactions) * 100).toFixed(1)}%)`,
        )
      }
      const types = [...p.byType.entries()].sort((x, y) => y[1] - x[1])
      for (const [k, n] of types) {
        console.log(
          `[bal-ecoef]     ${k}: ${n} (${((n / p.reactions) * 100).toFixed(1)}%)`,
        )
      }
    }
    dump('elementCoef=0‰（基准）', a)
    dump('elementCoef=400‰', b)
    dump('elementCoef=1200‰', c)

    // H2 优先排除：两档的胜负分布必须一致，否则漏怪数不可比。
    console.log(
      `[bal-ecoef] H2 判定：0‰ (won${a.won}/lost${a.lost}) vs ` +
        `400‰ (won${b.won}/lost${b.lost}) —— ` +
        (a.won === b.won && a.lost === b.lost ? '分布一致，非口径问题' : '分布不同，漏怪不可比！'),
    )

    // H1：若高阶反应占比在 400‰ 下降而低阶上升，则假设成立。
    const share = (p: Probe, tier: string): number => {
      const n = p.byTier.get(tier) ?? 0
      return p.reactions > 0 ? n / p.reactions : 0
    }
    for (const tier of ['strong(>=60)', 'weak(<60)']) {
      console.log(
        `[bal-ecoef] ${tier} 占比：0‰=${(share(a, tier) * 100).toFixed(1)}%  ` +
          `400‰=${(share(b, tier) * 100).toFixed(1)}%  ` +
          `1200‰=${(share(c, tier) * 100).toFixed(1)}%`,
      )
    }
  }, 600_000)

  it('决定性检验：漏怪增量集中在少数关（共振）还是均匀分布（系统性）', () => {
    // 「集中在少数关」= 系统的共振点（某些波次的节奏与 DoT 周期同步）
    // 「均匀分布」= 系统性的效率损失（那是真缺陷）
    //
    // 判据：前 5 关贡献了增量的多少比例。> 60% 判为共振。
    const base = leakedPerLevel('base')
    for (const [name, key] of [
      ['400‰', 'c400'],
      ['1200‰', 'c1200'],
    ] as const) {
      const m = leakedPerLevel(key)
      const deltas = levels
        .map((lv) => ({ id: lv.id, d: (m.get(lv.id) ?? 0) - (base.get(lv.id) ?? 0) }))
        .filter((x) => x.d > 0)
        .sort((x, y) => y.d - x.d)
      const total = deltas.reduce((s, x) => s + x.d, 0)
      const top = deltas.slice(0, 5)
      const share = total > 0 ? top.reduce((s, x) => s + x.d, 0) / total : 0
      console.log(`[bal-ecoef] ${name} 相对基准的漏怪增量`)
      console.log(`[bal-ecoef]     变多的关卡数 ${deltas.length}/${levels.length}  总增量 ${total}`)
      console.log(`[bal-ecoef]     前 5 关贡献 ${(share * 100).toFixed(0)}%`)
      console.log(`[bal-ecoef]     前 5: ${top.map((x) => `关${x.id}+${x.d}`).join(' ')}`)
    }
  }, 900_000)
})

describe('守卫：元素系数封顶必须避开惩罚区间', () => {
  // ## 结论（本轮诊断得出，已写进 README）
  //
  // 元素系数曲线的**惩罚区间在 200~800‰**：
  //
  //   400‰   63/100 关漏怪变多，总增量 +878，前 5 关只占 40% ⇒ **系统性**
  //   1200‰   6/100 关漏怪变多，总增量  +29，前 5 关占 97%  ⇒ 可忽略
  //
  // 「系统性」这个判定很重要：它排除了「某些波次节奏与 DoT 周期共振」这种
  // 局部解释 —— 那是调参能绕过的；系统性的意思是**这条成长线在 400‰ 就是净损失**。
  //
  // 最可能的机制：元素 DoT 是**不消耗热量的旁路伤害**，
  // 而它打在一个"已经走到防线附近"的敌人身上时就浪费掉了。
  // 关卡总血量固定，于是这份浪费就表现为**更多漏怪**。
  // 直接伤害不会有这个问题，因为它由玩家的射击节奏驱动、打得更早。
  //
  // ## 为什么不修机制，只卡封顶
  //
  // 改 DoT 的结算时机属于**核心元素系统的重设计**，回归风险高；
  // 而封顶 1200‰ 已经让惩罚消失（增量 +29，落在 6 关上）。
  // 性价比上，封顶是正确的选择。
  //
  // 本守卫的作用是**防止有人把封顶调进惩罚区间**。
  it('在封顶值（1200‰）下，漏怪增量可忽略', () => {
    const base = leakedPerLevel('base')
    const capped = leakedPerLevel('c1200')
    let worse = 0
    let totalDelta = 0
    for (const lv of levels) {
      const d = (capped.get(lv.id) ?? 0) - (base.get(lv.id) ?? 0)
      if (d > 0) worse++
      totalDelta += Math.max(0, d)
    }
    // 400‰ 时是「63 关 / +878」，1200‰ 应当远低于此
    expect(worse).toBeLessThanOrEqual(12)
    expect(totalDelta).toBeLessThanOrEqual(80)
  }, 900_000)
})
