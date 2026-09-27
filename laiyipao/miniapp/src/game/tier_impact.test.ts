/**
 * 攻方加成（装备/宝石/专精）对**关卡可完成性**的影响。
 *
 * ⚠️ 为什么在接线之后立刻要测这个
 *
 * 接线本身只证明「数值进去了」。玩家真正关心的是：
 * 满配之后是不是**赢得更轻松**？还是**赢得过头、关卡失去意义**？
 *
 * 一个更危险的失败模式：**加成把默认构筑推过某个阈值**，
 * 于是 100 关里有一批从「有压力」变成「闭眼过」——
 * 而这类变化不会让任何测试变红（关卡仍然 won）。
 *
 * 判据用**同一批关卡**在三个档位下各跑一遍：
 *   裸装（零加成）→ 典型（专精中等）→ 满配（封顶）
 * 关注两件事：
 *   1. 加成确实单调地让战斗更好（否则接线可能仍是半通）
 *   2. 满配下**没有出现新的失败**（封顶失控会毁掉关卡）
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { defaultAttacker, type Attacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
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

/**
 * 跑一局。
 *
 * ⚠️ **必须真的取牌**（而不是 `skipCards()`）。
 * 一开始用 `skipCards()`，结果 `mechanicPermille` 测出来是"完全无效"——
 * 而它其实生效，只是没有任何卡被取过，**没有东西可放大**。
 * 「观测不到」被读成「没生效」：本项目第六次同形状的坑。
 */
function play(lv: GeneratedLevel, tier: Partial<Attacker>) {
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
        // 固定轮转选牌：与档位无关，保证各档抽到同一批卡
        const c = hand[pick % hand.length]
        pick++
        ;(e as unknown as { takeCard(id: string): unknown }).takeCard(c.id)
      } else {
        e.skipCards()
      }
    }
    e.step()
  }
  return {
    phase: e.phase,
    sec: (e.tick * 50) / 1000,
    kills: e.kills,
    leaked: e.leaked,
    reactions: e.reactionsCount,
    total: e.totalEnemies,
    score: e.score,
    stars: lv.star_targets.filter((t) => e.score >= t).length,
  }
}

// 三个档位取自服务端的封顶（service/loadout_attacker.go）。
//
// 用**封顶值**而不是"典型玩家值"：封顶一旦失效，第一个受害者就是它。
//
// ⚠️ `attack` 的满配值是 **1000‰ 而非 3000‰**，这是实测决定的：
// 扫描 0/300/600/1000/1500/2000/3000‰ 的结果（100 关、可比口径）——
//   击杀数几乎不变（5262 → 5275）
//   分数几乎不变（790188 → 807665，+0.6%）
//   **反应次数单调下降**（15064 → 11279，-25%）
// 也就是说堆攻击力**买不到分数，却直接压制反应**，
// 而反应正是 I-1 的核心创新。所以封顶砍到 1000‰，
// 把成长预算留给元素系数/反应倍率（那些方向是正的）。
const TIERS = {
  bare: {},
  mid: { attack: 1200n, elementCoefPermille: 600n, critPermille: 200n, heatCapPermille: 400n },
  max: {
    attack: 1000n,
    elementCoefPermille: 1200n,
    critPermille: 500n,
    reactionMultPermille: 800n,
    heatCapPermille: 1000n,
    armorPermille: 750n,
    mechanicPermille: 1000n,
  },
} satisfies Record<string, Partial<Attacker>>

function median(xs: number[]): number {
  const s = xs.slice().sort((a, b) => a - b)
  return s[Math.floor(s.length / 2)]
}

describe('攻方加成对关卡的影响（全 100 关）', () => {
  it('加成确实让战斗更快（否则说明接线仍是半通）', () => {
    const secs = (tier: Partial<Attacker>) => median(levels.map((lv) => play(lv, tier).sec))
    const bare = secs(TIERS.bare)
    const mid = secs(TIERS.mid)
    const max = secs(TIERS.max)
    console.log(`[bal-tier] 通关时长中位数：裸装 ${bare}s → 中配 ${mid}s → 满配 ${max}s`)
    expect(mid).toBeLessThan(bare)
    expect(max).toBeLessThan(mid)
  })

  it('满配下没有任何一关变成失败（封顶失控会毁掉关卡）', () => {
    const failed = levels
      .map((lv) => ({ id: lv.id, r: play(lv, TIERS.max) }))
      .filter((x) => x.r.phase !== 'won')
    // 统一写法：空列表 join 出来就是空串。|| '（无）' 会让空列表变成'（无）'
    // 再去比 ''，于是**永远失败** —— 一条恒红的守卫比没有守卫更糟。
    expect(failed.map((x) => `关${x.id}(${x.r.phase})`).join('\n')).toBe('')
  })

  it('满配下没有任何一关触发停滞兜底', () => {
    const stuck = levels
      .map((lv) => ({ id: lv.id, r: play(lv, TIERS.max) }))
      .filter((x) => x.r.sec * 20 >= MAX_BATTLE_TICKS)
    expect(stuck.map((x) => `关${x.id}(${x.r.sec}s)`).join('\n')).toBe('')
  })

  it('分数不会因为满配而**明显**下降（升级装备不该让排行变差）', () => {
    // ⚠️ 容差 2%，而不是严格 `>=`。
    //
    // 分数是**整数**（击杀分 + 伤害分，伤害分 = totalDamage/100），
    // 而 totalDamage 取决于过杀时序 —— 攻击更高时敌人死得更快，
    // 最后一发的过杀量就不同。实测差异落在 0.01% ~ 1% 区间，
    // 属于整数粒度 + 过杀时序的噪声，不是"升级反而掉分"。
    //
    // 曾经用严格 `>=`，结果 2 关差 1~2 点就红 ——
    // 那是一条**永远会红**的守卫，会被当成 flaky 忽略掉，比没有守卫更糟。
    //
    // 2% 这个阈值不是随手取的：收紧
    // `MaxLoadoutElementCoefPermille`（1800 → 1200）**之前**，
    // 同一条断言捕捉到的是 891 点（3.9%）与 3 星掉 2 星 —— 那是真问题，已修。
    // 也就是说 2% 仍能抓住那类回归，只是放过了 1% 量级的噪声。
    const TOL = 0.02
    const regress: string[] = []
    for (const lv of levels) {
      const b = play(lv, TIERS.bare)
      const x = play(lv, TIERS.max)
      const drop = (b.score - x.score) / Math.max(1, b.score)
      if (drop > TOL) {
        regress.push(
          `关${lv.id}: ${b.score} → ${x.score}（-${(drop * 100).toFixed(2)}%）`,
        )
      }
    }
    expect(regress.slice(0, 8).join('\n')).toBe('')
  })

  it('星级不会因为满配而下降', () => {
    const regress: string[] = []
    for (const lv of levels) {
      const b = play(lv, TIERS.bare)
      const x = play(lv, TIERS.max)
      if (x.stars < b.stars) regress.push(`关${lv.id}: ${b.stars}星 → ${x.stars}星`)
    }
    expect(regress.slice(0, 8).join('\n')).toBe('')
  })
})

describe('热量上限是千分比而不是绝对值（单位回归守卫）', () => {
  // ⚠️ 守的是一个**真实踩过的单位错**：
  // `cap = HEAT_MAX + capBonus` 把千分比当绝对值加，
  // 于是 1000‰ 让上限变成 1100 而不是 200 —— 漏怪翻 2.2 倍、分数掉 13.8%。
  // 「提高热量上限」是**有利属性**，出现反向曲线只能是单位错了。
  const capWith = (permille: bigint): bigint => {
    const e = new BattleEngine({
      level: levels[0],
      enemies,
      skills,
      equipped: equipped(),
      attacker: { ...defaultAttacker(), heatCapPermille: permille },
      seed: 1,
    })
    return e.heat.cap
  }

  it('1000‰ 让上限翻倍，而不是变成 11 倍', () => {
    const bare = capWith(0n)
    const doubled = capWith(1000n)
    expect(doubled).toBe(bare * 2n)
    // 反向断言：旧写法会得到 bare + 1000
    expect(doubled).not.toBe(bare + 1000n)
  })

  it('200‰ / 500‰ 分别是 1.2 倍 / 1.5 倍', () => {
    const bare = capWith(0n)
    expect(capWith(200n)).toBe((bare * 1200n) / 1000n)
    expect(capWith(500n)).toBe((bare * 1500n) / 1000n)
  })

  it('上限随加成单调不减', () => {
    let prev = 0n
    for (const p of [0n, 100n, 500n, 1000n, 2000n, 5000n]) {
      const c = capWith(p)
      expect(c).toBeGreaterThanOrEqual(prev)
      prev = c
    }
  })

  it('提高上限让过热次数减少（有利属性该有的方向）', () => {
    const overheatCount = (cap: bigint): number => {
      const e = new BattleEngine({
        level: levels[0],
        enemies,
        skills,
        equipped: equipped(),
        attacker: { ...defaultAttacker(), heatCapPermille: cap },
        seed: 12345,
      })
      e.start()
      let n = 0
      let wasHot = false
      for (let t = 0; t < 40000; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        e.step()
        const hot = e.heat.overheated
        if (hot && !wasHot) n++
        wasHot = hot
      }
      return n
    }
    const bare = overheatCount(0n)
    const raised = overheatCount(1000n)
    expect(bare).toBeGreaterThan(0)
    expect(raised).toBeLessThan(bare)
  })
})
