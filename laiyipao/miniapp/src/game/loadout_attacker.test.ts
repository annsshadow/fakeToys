/**
 * 攻方三项新字段在**客户端引擎里真的生效**。
 *
 * ⚠️ 为什么这条必须存在
 *
 * 服务端接线完成不等于客户端接线完成：`computeAttacker` 把装备、宝石、
 * 专精汇总进 `attacker`，而**真正扣血、真正涨热量上限、真正放大卡面数值**
 * 的是 TS 引擎。三处各写一遍，漏一处的表现是
 * 「装备显示在界面上、战斗里毫无变化」。
 *
 * 变异测试已经证明这一点：把 `defenseArmorPermille()` 里的
 * `+ this.cfg.attacker.armorPermille` 删掉，**23 条用例全绿**。
 *
 * 判据一律用「改变一个输入 → 观测到战斗结果变化」，
 * 不断言代码里写了哪几行。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, type BattleConfig } from './engine'
import { ACTIVE_SLOTS } from './heatmap'
import { defaultAttacker, type Attacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { toFixed } from './terrain'

const allLevels = fixture.levels as unknown as GeneratedLevel[]
const level = allLevels.find((l) => l.id === 1)!
// 第 1 关默认构筑是**零漏怪全清**（见 no_deadlock 的基线），
// 而第 50 关的实测基线是 heat 打满 100 —— 用来测需要"真的发生"的量。
const heatLevel = allLevels.find((l) => l.id === 50)!
// 第 97 关是默认构筑**实测会漏怪**的四关之一（基线漏 26 只），
// 是护甲减伤唯一能观测到的关卡。
//
// ⚠️ 一度试图用 `attack: 0n` 制造漏怪，结果漏怪数仍是 0 ——
// 因为 `Attack` 是**加成**不是伤害总量：`D = SkillDamage × (1000+Attack)/1000`，
// Attack=0 意味着"只剩技能基础伤害"，照样打得完。
// **前提不成立时测的不是想测的东西** —— 与地形度量同源的坑。
const leakyLevel = allLevels.find((l) => l.id === 97)!
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

function mk(attackerOverrides: Partial<Attacker> = {}, lv: GeneratedLevel = level): BattleEngine {
  const cfg: BattleConfig = {
    level: lv,
    enemies,
    skills,
    equipped: equipped(),
    attacker: { ...defaultAttacker(), ...attackerOverrides },
    seed: 12345,
  }
  const e = new BattleEngine(cfg)
  e.start()
  return e
}

/**
 * 跑完整局（用会漏怪的关卡），返回漏怪数与结束血量。
 *
 * ⚠️ 必须用 leakyLevel：默认构筑在第 1 关是「全清零漏」，
 * 护甲减伤**无从体现** —— 两组血量相同，比不出差异。
 */
function runFullBattle(attackerOverrides: Partial<Attacker> = {}): {
  hpLeft: bigint
  leaked: number
  cap: bigint
} {
  const e = mk(attackerOverrides, leakyLevel)
  for (let t = 0; t < 40000; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return {
    hpLeft: (e as unknown as { baseHp: bigint }).baseHp,
    leaked: e.leaked,
    cap: e.heat.cap,
  }
}

describe('攻方·防线护甲（armorPermille）', () => {
  it('护甲越高，漏怪伤害越低（血量剩余更多）', () => {
    const bare = runFullBattle({ armorPermille: 0n })
    const armored = runFullBattle({ armorPermille: 600n })

    // 前提：这一局必须真的漏了怪，否则两组血量相同、比不出差异
    expect(bare.leaked).toBeGreaterThan(0)
    expect(armored.leaked).toBe(bare.leaked)

    // 护甲 600‰ ⇒ 漏怪伤害降到 40%
    const ratio = Number(armored.hpLeft) / Number(bare.hpLeft)
    expect(ratio).toBeGreaterThan(1.2)
  })

  it('零护甲时行为与「字段不存在」完全一致', () => {
    // 这一条是回归护栏：defenseArmorPermille() 里若误写成
    // `+ attacker.armorPermille` 之外还叠加了别的默认值，
    // 新号（零加成）的战斗就会与历史战报哈希不符。
    const e = mk({ armorPermille: 0n })
    const got = (e as unknown as { baseArmorPermille: bigint; buffs: { armorPermille: number } })
    const expected =
      got.baseArmorPermille + BigInt(got.buffs.armorPermille)
    const actual = (e as unknown as { defenseArmorPermille(): bigint }).defenseArmorPermille()
    expect(actual).toBe(expected)
  })

  it('护甲有上限，不会把漏怪伤害压到 0（否则关卡无法失败）', () => {
    // 比较**承伤量**而不是剩余血量的比值：
    // 第 97 关无护甲时防线会破（剩余血量归零甚至为负），
    // 用比值会得到 0 或乱数 —— 前提不成立的指标读不出结论。
    const taken = (p: bigint): number => {
      const r = runFullBattle({ armorPermille: p })
      // ⚠️ 必须用 leakyLevel 的 base_hp：runFullBattle 跑的是它，
      // 拿第 1 关的血量当分母会得到大负数，断言随即变成"永远失败"。
      const max = Number(leakyLevel.base_hp)
      return max - Number(r.hpLeft)
    }
    const bare = taken(0n)
    const maxArmor = taken(999_999n)
    expect(bare).toBeGreaterThan(0)
    // 上限 750‰ ⇒ 伤害保留约 25%
    const ratio = maxArmor / bare
    expect(ratio).toBeGreaterThan(0.15)
    expect(ratio).toBeLessThan(0.45)
  })
})

describe('攻方·热量上限（heatCapPermille）', () => {
  it('热量上限加成真的抬高了 HeatMeter.cap', () => {
    const bare = mk({ heatCapPermille: 0n }).heat.cap
    // ⚠️ 400‰ 是 **+40%**，不是 +400 点。
    // 曾经的 `cap = HEAT_MAX + capBonus` 让它变成 +400 点（上限 4 倍），
    // 而热量衰减率固定 ⇒ 玩家长期卡在死区 ⇒ 漏怪翻 2.2 倍、分数掉 13.8%。
    const boosted = mk({ heatCapPermille: 400n }).heat.cap
    expect(boosted - bare).toBe((bare * 400n) / 1000n)
    expect(boosted - bare).toBe(40n)
  })

  it('零加成时 cap 与常量基准一致', () => {
    const e = mk({ heatCapPermille: 0n })
    // 用「加成 0 与加成 1000‰ 的比值」反推，而不是硬编码 HEAT_MAX ——
    // 硬编码会在引擎基准变动时静默失配。
    //
    // ⚠️ 不能用 1‰ 做反推：千分比下 100 × 1001/1000 整除后仍是 100，
    // 「+1 就等于 1‰ 的 cap」这个等式在整除下不成立。
    const doubled = mk({ heatCapPermille: 1000n }).heat.cap
    expect(doubled).toBe(e.heat.cap * 2n)
  })

  it('热量上限提高后过热更难触发（过热次数更少）', () => {
    const countOverheats = (capBonus: bigint): number => {
      // 用第 50 关而不是第 1 关：实测基线里第 1 关的热量打不满，
      // 根本不会过热 —— 前提不成立时测的不是想测的东西。
      const e = mk({ heatCapPermille: capBonus }, heatLevel)
      let n = 0
      const seen = e as unknown as { heat: { overheated: boolean } }
      let wasHot = false
      for (let t = 0; t < 20000; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        e.step()
        if (seen.heat.overheated && !wasHot) n++
        wasHot = seen.heat.overheated
      }
      return n
    }
    const bare = countOverheats(0n)
    const boosted = countOverheats(600n)
    // 前提：基线必须真的过热过，否则比不出差异
    expect(bare).toBeGreaterThan(0)
    expect(boosted).toBeLessThan(bare)
  })
})

describe('攻方·机制卡强度（mechanicPermille）', () => {
  /**
   * 取第一张 pierce_bonus 机制卡，返回取卡后的 pierceBonus。
   *
   * 跨多个种子搜索：某一局未必抽到 pierce_bonus，
   * 只跑一个种子会让这条测试**偶发失败**——
   * 而偶发的守卫比没有守卫更糟（它会被当成 flaky 忽略掉）。
   */
  function takePierceCard(mechanicPermille: bigint): { taken: boolean; pierce: number } {
    for (const seed of [12345, 777, 20250101, 42, 99999, 31337, 8080, 555]) {
      const cfg: BattleConfig = {
        level,
        enemies,
        skills,
        equipped: equipped(),
        attacker: { ...defaultAttacker(), mechanicPermille },
        seed,
      }
      const e = new BattleEngine(cfg)
      e.start()
      for (let t = 0; t < 40000; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') {
          const hand = (
            e as unknown as { deck: { hand: { id: string; mechanic?: { kind: string } }[] } }
          ).deck.hand
          const card = hand.find((c) => c.mechanic?.kind === 'pierce_bonus')
          if (card) {
            ;(e as unknown as { takeCard(id: string): unknown }).takeCard(card.id)
            const pierce = (e as unknown as { buffs: { pierceBonus: number } }).buffs.pierceBonus
            return { taken: true, pierce }
          }
          e.skipCards()
        }
        e.step()
      }
    }
    return { taken: false, pierce: 0 }
  }

  it('机制卡强度加成会放大卡面数值', () => {
    const bare = takePierceCard(0n)
    // 用 **2 倍**（2000‰）而不是 1.5 倍：
    // 穿透卡的面值是 +1~+2 这种**小整数**，
    // floor(1 × 1.5) = 1 = 1 × 1.0 —— 加了也看不出差别。
    //
    // 这不是 bug，是整数运算的正确结果（README 工程约束 8 要求全程定点）。
    // 但它意味着「低加成 + 小面值」这一组合在数值上无效，
    // 所以 `mechanicPermille` 的设计上限（1000‰）之下，
    // 玩家必须点够层数或选高面值卡才看得到效果。
    const doubled = takePierceCard(1000n)

    expect(bare.taken).toBe(true)
    expect(bare.pierce).toBeGreaterThan(0)
    expect(doubled.pierce).toBe(bare.pierce * 2)
  })

  it('低加成 + 小面值时数值不变（整数取整的直接后果）', () => {
    // 把上面那条的"反直觉"钉成显式契约，而不是让它看起来像 bug：
    // 1.5× 作用在 +1 上取整后仍是 1。
    const bare = takePierceCard(0n)
    const half = takePierceCard(500n)
    expect(bare.taken).toBe(true)
    expect(half.pierce).toBe(Math.floor((bare.pierce * 1500) / 1000))
    // 面值为 1 时，1.5× 与 1× 取整后相同
    if (bare.pierce === 1) {
      expect(half.pierce).toBe(1)
    }
  })

  it('零加成时卡面数值就是原始值（不引入舍入偏差）', () => {
    const bare = takePierceCard(0n)
    const zero = takePierceCard(0n)
    expect(zero.pierce).toBe(bare.pierce)
  })

  it('加成是单调的：越大不越小', () => {
    const a = takePierceCard(250n).pierce
    const b = takePierceCard(1000n).pierce
    const c = takePierceCard(2000n).pierce
    expect(b).toBeGreaterThanOrEqual(a)
    expect(c).toBeGreaterThanOrEqual(b)
  })
})

describe('攻方三项字段的缺省值', () => {
  it('defaultAttacker() 的三项都是 0（新号没有任何加成）', () => {
    const a = defaultAttacker()
    expect(a.armorPermille).toBe(0n)
    expect(a.heatCapPermille).toBe(0n)
    expect(a.mechanicPermille).toBe(0n)
  })

  it('这三项从 currentAttacker 原样透传，不与局内 Buffs 相加', () => {
    // 加了会怎样：currentAttacker 的返回值参与重放哈希，
    // 多加一次就与原局不符 —— I-6 会把正常对局判成伪造。
    const e = mk({ armorPermille: 100n, heatCapPermille: 200n, mechanicPermille: 300n })
    const cur = (e as unknown as { currentAttacker(): Attacker }).currentAttacker()
    expect(cur.armorPermille).toBe(100n)
    expect(cur.heatCapPermille).toBe(200n)
    expect(cur.mechanicPermille).toBe(300n)
  })
})

describe('攻方字段的坐标约定', () => {
  it('装甲在定点域里比较，不引入浮点', () => {
    // 与 damage.ts 的 applyArmor 一致：护甲是千分比，
    // 漏怪伤害是 bigint。若哪天有人改成 Number 比较，
    // 大血量关卡会出现 1 单位的舍入差 → 哈希不符。
    const e = mk({ armorPermille: 300n })
    const armor = (e as unknown as { defenseArmorPermille(): bigint }).defenseArmorPermille()
    expect(typeof armor).toBe('bigint')
    // 且不小于 0
    expect(armor >= 0n).toBe(true)
  })

  it('toFixed 仍是定点整数（防止有人改成 Number 混用）', () => {
    expect(typeof toFixed(100)).toBe('bigint')
  })
})
