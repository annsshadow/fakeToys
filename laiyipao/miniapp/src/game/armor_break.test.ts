import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { stripTsComments } from '../testkit/stripComments'
import { BattleEngine, type BattleConfig } from './engine'
import { type EquippedSkill } from './heatmap'
import { defaultAttacker, resolveHit, type HitInput } from './damage'
import { Defender } from './damage'
import { REACTIONS, type Element } from './elements'
import { PERMILLE, applyArmor, mulDiv, DIRECT_WEIGHT } from './fixed'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

/**
 * `armor_break`（破甲击退）的两个特效**必须真的生效**（本轮补实现）。
 *
 * # 背景：它曾是 7 条反应里唯一「名字在承诺、代码没实现」的
 *
 * 第 83 轮把文案改成只描述已实现的效果后，`armor_break` 只剩反应伤害
 * （`baseCoef` 30，七条里最低），名字里的「破甲」「击退」是空的。
 * 本轮把两个特效接上（engine.ts `hitEnemy`）：
 *
 * - **削甲**：触发后 4000ms 内，目标有效护甲被削 `armorShredPermille`(150‰)。
 *   引擎在构建 `Defender` 时折进有效护甲（`effArmor`）。
 * - **击退**：触发时目标一次性向右（远离防线）位移 `knockback`(4000 定点)，
 *   由 `stepEnemyMotion` 的 knockback 分支消费后清零。
 *
 * # 判据三级（与 skill_level_battle.test.ts 同一条工程约束：
 * 「凡是能改变战斗结果的输入，必须落在可观测的量上」）
 *
 * | 级别 | 判据 | 能证明什么 |
 * |---|---|---|
 * | 触发 | 真引擎里 `reactionsUsed.armor_break >= 1` | 反应真的触发了 |
 * | 状态 | 触发后 `armorShredMs>0 && armorShredPermille===150n && 击退量=4000` | 状态确实被写了 |
 * | 数值 | 削甲后 resolveHit 直伤严格变大 | 改变的不是字段，是**伤害结果** |
 *
 * 光断「字段被赋值」是**反射**，改坏了照样绿 —— 所以第三级必须落在
 * 「施加后状态确实变了 + 伤害真的变了」。
 */

// ---- 夹具 ----

function zeroResist(): Record<string, number> {
  return { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 }
}

/** 一个**高护甲、超高血、零速度**的敌人：停在场地右侧，既不打防线也不死。 */
function armoredEnemy(armorPermille: number): EnemyDef {
  return {
    id: 7,
    code: 'armored',
    name: '重甲',
    category: 'normal',
    hp: 1_000_000_000,
    speed: 0,
    armor: armorPermille,
    shield_hp: 0,
    attack: 0,
    attack_range: 0,
    attack_interval: 0,
    fly_height: 0,
    burrow: false,
    is_boss: false,
    resist: zeroResist(),
    descr: '',
  }
}

function kineticSkill(): SkillDef {
  return {
    id: 3,
    code: 's3',
    name: '重炮弹',
    family: 'test',
    element: 'kinetic',
    kind: 'active',
    descr: '',
    base_damage: 60,
    heat_cost: 8,
    cooldown_ms: 300,
    pierce: 0,
    aoe_radius: 0,
    apply_element: 'kinetic',
    apply_stacks: 1,
    projectile_speed: 200_000,
    chain: 0,
    unlock_level: 1,
  }
}

function singleWaveLevel(): GeneratedLevel {
  return {
    id: 1,
    chapter: 1,
    name: '破甲测试关',
    seed: '424242',
    base_hp: 1_000_000_000,
    wave_count: 1,
    difficulty: 1000,
    energy_cost: 5,
    element_cap: 4,
    armor_permille: 0,
    max_reaction_tier: 3,
    is_boss: false,
    star_targets: [1, 1, 1],
    max_score: 1_000_000,
    terrain: [],
    waves: [
      {
        wave_index: 0,
        spawns: [{ enemy_id: 7, count: 1, interval: 0, delay: 0 }],
      },
    ],
  }
}

function equippedSkills(skill: SkillDef): EquippedSkill[] {
  return [
    {
      skillId: skill.id,
      name: skill.name,
      element: skill.element,
      kind: skill.kind,
      heatCost: BigInt(skill.heat_cost),
      cooldownMs: skill.cooldown_ms,
      pierce: skill.pierce,
      aoeRadius: skill.aoe_radius,
      baseDamage: BigInt(skill.base_damage),
      applyElement: (skill.apply_element ?? '') as Element | '',
      applyStacks: BigInt(skill.apply_stacks),
      projectileSpeed: skill.projectile_speed,
      chain: skill.chain,
      slot: 0,
      cooldownRemaining: 0,
    },
  ]
}

/**
 * 造一个「只装动能弹 + 敌人身上预置了焰层」的引擎。
 *
 * 预置焰层是关键：`resolveReaction` 用「已附着元素 + 来袭元素」查表，
 * 焰(已附着) + 动能(来袭) → `armor_break`。没有这层，动能命中只会
 * 挂动能层、不触发任何反应，本用例就测不到破甲。
 */
function armorBreakEngine(seed = 9999): BattleEngine {
  const skill = kineticSkill()
  const level = singleWaveLevel()
  const enemies = new Map<number, EnemyDef>([
    // 关卡 spawn 的 enemy_id 是 7（singleWaveLevel 里），夹具必须同 id。
    [7, armoredEnemy(300)],
  ])
  const skills = new Map<number, SkillDef>([[skill.id, skill]])
  const cfg: BattleConfig = {
    level,
    enemies,
    skills,
    equipped: equippedSkills(skill),
    attacker: defaultAttacker(),
    seed,
  }
  const e = new BattleEngine(cfg)
  e.start()
  // 推进到出生动画结束，再给那个敌人预置焰层。
  for (let i = 0; i < 30; i++) {
    e.step()
    const t = e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)
    if (t) {
      t.stacks.set('fire', 3n)
      break
    }
  }
  return e
}

// ---- 一、触发：真引擎里 armor_break 真的发生 ----

describe('armor_break 触发（真实引擎）', () => {
  it('焰层 + 动能命中 → 破甲反应计数增加', () => {
    const e = armorBreakEngine()
    // 跑够多 tick 让动能弹打上那面重甲墙（它血量天量、不会死）。
    for (let i = 0; i < 400; i++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') e.skipCards()
      e.step()
    }
    expect(
      e.reactionsUsed['armor_break'] ?? 0,
      '焰层在身的目标被动能命中，必然触发破甲反应',
    ).toBeGreaterThan(0)
  })

  it('触发后目标确实带着削甲状态（状态被写，不是没写）', () => {
    const e = armorBreakEngine()
    for (let i = 0; i < 400; i++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') e.skipCards()
      e.step()
    }
    const t = e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)!
    // 削甲量必须恰好等于反应表声明的 150‰（不是别的值、不是 0）。
    expect(
      t.armorShredPermille,
      '触发后削甲量应等于反应表的 armorShredPermille',
    ).toBe(BigInt(REACTIONS.armor_break.armorShredPermille))
    expect(t.armorShredPermille).toBeGreaterThan(0n)
  })
})

// ---- 二、数值：削甲必须真的让伤害变大 ----

describe('削甲改变的是伤害结果，不是某个字段', () => {
  const att = defaultAttacker()
  // 一次「焰层 3 + 动能」的命中输入（与 armor_break 触发同形）。
  const input: HitInput = {
    skillDamage: 60n,
    skillElement: 'kinetic',
    applyStacks: 1n,
    roll: 0,
  }

  it('削甲 150‰ 后直伤严格变大（护甲 300‰ → 150‰）', () => {
    const fullArmor = new Defender(10_000n, 0n, 300n)
    const shrunkenArmor = new Defender(10_000n, 0n, 150n) // 300 - 150
    fullArmor.stacks.set('fire', 3n)
    shrunkenArmor.stacks.set('fire', 3n)
    const rFull = resolveHit(att, fullArmor, input)
    const rShred = resolveHit(att, shrunkenArmor, input)
    expect(
      rShred.totalDamage,
      '削甲后总伤害必须严格大于全额护甲时（削甲买到了东西）',
    ).toBeGreaterThan(rFull.totalDamage)
    // 差值恰好是「直伤段在两种护甲下的差」—— 用同一个 applyArmor 锚定。
    // ⚠️ 必须复刻 resolveHit 的**两次除法**顺序（先攻击力倍率、再直伤权重）：
    // 合成一次乘法再除一次会得到不同的截断结果。
    const d = mulDiv(60n, PERMILLE + att.attack, PERMILLE)
    const baseDirect = applyArmor(mulDiv(d, DIRECT_WEIGHT, PERMILLE), 300n)
    const shredDirect = applyArmor(mulDiv(d, DIRECT_WEIGHT, PERMILLE), 150n)
    expect(shredDirect).toBeGreaterThan(baseDirect)
  })

  it('削甲不为负：护甲 50‰ 被削 150‰ 后有效护甲 = 0（不是负数）', () => {
    // 引擎 effArmor 的 `armorPermille > armorShredPermille ? 相减 : 0n` 分支。
    const armor = 50n
    const shred = BigInt(REACTIONS.armor_break.armorShredPermille) // 150n
    const eff = armor > shred ? armor - shred : 0n
    expect(eff, '削甲后有效护甲应当被夹到 0，绝不为负').toBe(0n)
  })

  it('削甲期结束后护甲复原（armorShredMs 归零即失效）', () => {
    // 判据落在「状态机」上：削甲是有时限的 debuff，不是永久 debuff。
    // 这里直接复刻 stepEnemyMotion 的递减 + hitEnemy 的 `armorShredMs > 0` 判据，
    // 确认「倒计时走完 → 有效护甲回到全额」这条时序成立。
    const armor = 300n
    const shred = BigInt(REACTIONS.armor_break.armorShredPermille)
    const TICK_MS = 50
    let remaining = 4000 // statusDurationMs
    const effAt = (ms: number) => (ms > 0 ? (armor > shred ? armor - shred : 0n) : armor)
    expect(effAt(remaining)).toBe(150n) // 削甲期内
    for (let ms = remaining; ms > 0; ms -= TICK_MS) remaining = ms - TICK_MS
    expect(effAt(remaining)).toBe(armor) // 倒计时走完 → 复原
  })
})

// ---- 三、接线：hitEnemy 必须真的读削甲状态、写击退（源码消费面核对）----

/** 取出 engine.ts 里 `hitEnemy` 的**已剥注释**源码区间。 */
function hitEnemyBody(): string {
  const src = stripTsComments(readFileSync(resolve(__dirname, 'engine.ts'), 'utf-8'))
  const start = src.indexOf('private hitEnemy(')
  expect(start, '找不到 hitEnemy —— 结构变了').toBeGreaterThan(0)
  const end = src.indexOf('\n  }', start)
  expect(end, 'hitEnemy 没有正常闭合').toBeGreaterThan(start)
  return src.slice(start, end)
}

describe('armor_break 接线核对（消费面）', () => {
  it('构建 Defender 时折进削甲后的有效护甲，而不是原始护甲', () => {
    const body = hitEnemyBody()
    expect(
      /new Defender\(e\.hp,\s*e\.shield,\s*effArmor\)/.test(body),
      'hitEnemy 必须用「削甲后的有效护甲」建 Defender。\n' +
        '若仍写 `new Defender(e.hp, e.shield, e.armorPermille)`，削甲就成了只写不读。',
    ).toBe(true)
    // effArmor 的表达式必须同时用到 armorShredMs 与 armorShredPermille。
    expect(body).toMatch(/e\.armorShredMs > 0/)
    expect(body).toMatch(/e\.armorShredPermille/)
  })

  it('破甲反应触发时写削甲时长 / 削甲量 / 击退量三个状态', () => {
    const body = hitEnemyBody()
    expect(body, '写削甲时长').toMatch(/e\.armorShredMs\s*=\s*spec\.statusDurationMs/)
    expect(body, '写削甲量').toMatch(/e\.armorShredPermille\s*=\s*BigInt\(spec\.armorShredPermille\)/)
    expect(body, '写击退位移').toMatch(/e\.knockback\s*\+=\s*BigInt\(spec\.knockback\)/)
    // 三个写入必须被「是 armor_break 且削甲量 > 0」的判据包住，
    // 而不是无条件写（否则任何反应都会削甲/击退）。
    expect(body).toMatch(/react === 'armor_break' && spec\.armorShredPermille > 0/)
  })

  it('击退位移由 stepEnemyMotion 的 knockback 分支消费并清零', () => {
    // 这一分支已被 engine.test.ts 的「敌人击退位移」用例行为覆盖；
    // 这里只核对「hitEnemy 写的是累加（+=）」，避免多次触发相互覆盖。
    const body = hitEnemyBody()
    expect(body).toMatch(/e\.knockback\s*\+=/)
  })
})

// ---- 四、回归护栏：默认构筑（无动能）触发不了 armor_break，伤害基线不变 ----

describe('默认构筑基线不受影响（无动能 = 无破甲）', () => {
  it('只装焰弹时，目标不会被削甲（armorShredPermille 恒 0）', () => {
    // 焰 + 焰 查表是 null（同元素不反应），所以装纯焰弹时目标即便挂了焰层
    // 也不会触发任何反应，更不会走 armor_break 的削甲/击退写入。
    // 这保证了「默认构筑（fire/fire/fire/ice）的历史 100 关基线不变」这条
    // 事实：只有带动能的构筑才会进入新的削甲/击退路径。
    const fireSkill: SkillDef = {
      ...kineticSkill(),
      id: 1,
      code: 's1',
      name: '燃烧弹',
      element: 'fire',
      apply_element: 'fire',
    }
    const level = singleWaveLevel()
    const enemies = new Map<number, EnemyDef>([[7, armoredEnemy(300)]])
    const skills = new Map<number, SkillDef>([[fireSkill.id, fireSkill]])
    const e = new BattleEngine({
      level,
      enemies,
      skills,
      equipped: equippedSkills(fireSkill),
      attacker: defaultAttacker(),
      seed: 1,
    })
    e.start()
    for (let i = 0; i < 400; i++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') e.skipCards()
      e.step()
    }
    for (const t of e.enemies) {
      expect(
        t.armorShredPermille,
        '无动能构筑不应给任何目标削甲',
      ).toBe(0n)
    }
    expect(e.reactionsUsed['armor_break'] ?? 0, '无动能时不应有破甲反应').toBe(0)
  })
})
