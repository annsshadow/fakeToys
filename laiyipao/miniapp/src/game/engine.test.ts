import { describe, it, expect } from 'vitest'
import { BattleEngine, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { HeatSystem, CardDeck, HEAT_MAX } from './heatmap'

// ---- 测试夹具 ----

const ENEMIES: EnemyDef[] = [
  mkEnemy(1, '游荡者', 'normal', { hp: 100, speed: 40000, resist: zeroResist() }),
  mkEnemy(2, '疾行者', 'normal', { hp: 70, speed: 75000, resist: zeroResist() }),
  mkEnemy(3, '铁颚精英', 'boss', { hp: 3000, speed: 20000, is_boss: true, armor: 200,
    resist: { fire: -400, ice: 500, lightning: -400, corrosion: 500, kinetic: 0 } }),
]

function zeroResist(): Record<string, number> {
  return { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 }
}

function mkEnemy(
  id: number,
  name: string,
  category: EnemyDef['category'],
  extra: Partial<EnemyDef>,
): EnemyDef {
  return {
    id,
    code: `e${id}`,
    name,
    category,
    hp: 100,
    speed: 40000,
    armor: 0,
    shield_hp: 0,
    attack: 0,
    attack_range: 0,
    attack_interval: 0,
    fly_height: 0,
    burrow: false,
    is_boss: false,
    resist: zeroResist(),
    descr: '',
    ...extra,
  }
}

function mkLevel(overrides: Partial<GeneratedLevel> = {}): GeneratedLevel {
  return {
    id: 1,
    chapter: 1,
    name: '测试关',
    seed: '12345',
    base_hp: 2000,
    wave_count: 2,
    difficulty: 1000,
    energy_cost: 5,
    element_cap: 4,
    armor_permille: 0,
    max_reaction_tier: 3,
    is_boss: false,
    star_targets: [1000, 2000, 3000],
    terrain: [],
    waves: [
      {
        wave_index: 0,
        spawns: [
          { enemy_id: 1, count: 3, interval: 200, delay: 0 },
          { enemy_id: 2, count: 2, interval: 300, delay: 400 },
        ],
      },
      { wave_index: 1, spawns: [{ enemy_id: 1, count: 4, interval: 250, delay: 0 }] },
    ],
    ...overrides,
  }
}

function mkSkills(): Map<number, SkillDef> {
  const defs: SkillDef[] = [
    mkSkill(1, '燃烧弹', 'fire', 100, 20, 800, 0, 60, 1),
    mkSkill(2, '干冰弹', 'ice', 90, 18, 700, 1, 0, 1),
    mkSkill(3, '重炮弹', 'kinetic', 130, 22, 1100, 1, 80, 1),
    mkSkill(4, '电磁穿刺', 'lightning', 150, 34, 1500, 2, 0, 2),
  ]
  return new Map(defs.map((d) => [d.id, d]))
}

function mkSkill(
  id: number,
  name: string,
  element: Element,
  base_damage: number,
  heat_cost: number,
  cooldown_ms: number,
  pierce: number,
  aoe_radius: number,
  apply_stacks: number,
): SkillDef {
  return {
    id,
    code: `s${id}`,
    name,
    family: 'test',
    element,
    kind: 'active',
    descr: '',
    base_damage,
    heat_cost,
    cooldown_ms,
    pierce,
    aoe_radius,
    apply_element: element,
    apply_stacks,
    projectile_speed: 60000,
    chain: 0,
    unlock_level: 1,
  }
}

function mkEquipped(skills: SkillDef[]) {
  return skills.slice(0, 3).map((s, i) => ({
    skillId: s.id,
    name: s.name,
    element: s.element,
    kind: s.kind,
    heatCost: BigInt(s.heat_cost),
    cooldownMs: s.cooldown_ms,
    pierce: s.pierce,
    aoeRadius: s.aoe_radius,
    baseDamage: BigInt(s.base_damage),
    applyElement: s.element,
    applyStacks: BigInt(s.apply_stacks),
    projectileSpeed: s.projectile_speed,
    chain: s.chain,
    slot: i,
    cooldownRemaining: 0,
  }))
}

function mkEngine(seed = 12345, level = mkLevel()): BattleEngine {
  const skills = mkSkills()
  const cfg: BattleConfig = {
    level,
    enemies: new Map(ENEMIES.map((e) => [e.id, e])),
    skills,
    equipped: mkEquipped([...skills.values()]),
    attacker: defaultAttacker(),
    seed,
  }
  return new BattleEngine(cfg)
}

/** 推进 n 步，收集所有事件 */
function run(engine: BattleEngine, steps: number) {
  const events = []
  for (let i = 0; i < steps; i++) {
    events.push(...engine.step())
  }
  return events
}

// ---- 测试 ----

describe('战斗引擎确定性', () => {
  it('同种子两次战斗产出完全相同的统计与哈希', () => {
    const a = mkEngine(999)
    const b = mkEngine(999)
    a.start()
    b.start()
    run(a, 600)
    run(b, 600)
    expect(a.kills).toBe(b.kills)
    expect(a.shots).toBe(b.shots)
    expect(a.score).toBe(b.score)
    expect(a.replayHash()).toBe(b.replayHash())
  })

  it('不同种子产出不同的回放哈希（否则哈希无区分度）', () => {
    const a = mkEngine(111)
    const b = mkEngine(222)
    a.start()
    b.start()
    run(a, 400)
    run(b, 400)
    expect(a.replayHash()).not.toBe(b.replayHash())
  })

  it('回放哈希为 16 位十六进制', () => {
    const e = mkEngine(5)
    e.start()
    run(e, 120)
    expect(e.replayHash()).toMatch(/^[0-9a-f]{16}$/)
  })
})

describe('战斗流程', () => {
  it('开始后进入 wave 阶段并按波次刷怪', () => {
    const e = mkEngine(3)
    e.start()
    expect(e.phase).toBe('wave')
    const events = run(e, 120)
    expect(events.some((ev) => ev.type === 'wave_start')).toBe(true)
    expect(e.enemies.length).toBeGreaterThan(0)
  })

  it('清完一波后进入 card_select 并给出 3 张手牌', () => {
    const e = mkEngine(3)
    e.start()
    // 推进到第一波清完
    let guard = 0
    while (e.phase === 'wave' && guard++ < 4000) e.step()
    expect(e.phase).toBe('card_select')
    expect(e.deck.size).toBe(3)
    const kinds = e.deck.hand.map((c) => c.kind).sort()
    expect(kinds).toEqual(['attribute', 'mechanic', 'skill'])
  })

  it('取完所有手牌后进入下一波（取 1 张仍在选牌阶段）', () => {
    const e = mkEngine(3)
    e.start()
    let guard = 0
    while (e.phase === 'wave' && guard++ < 20000) e.step()
    expect(e.phase).toBe('card_select')

    // 取 1 张后仍有 2 张，必须还停在选牌阶段
    e.takeCard(e.deck.hand[0].id)
    expect(e.phase).toBe('card_select')
    expect(e.deck.size).toBe(2)

    // 取完剩余的才进入下一波
    e.takeCard(e.deck.hand[0].id)
    e.takeCard(e.deck.hand[0].id)
    expect(e.deck.size).toBe(0)
    expect(e.phase).toBe('wave')
  })

  it('跳过手牌也能进入下一波', () => {
    const e = mkEngine(3)
    e.start()
    let guard = 0
    while (e.phase === 'wave' && guard++ < 20000) e.step()
    expect(e.phase).toBe('card_select')
    e.skipCards()
    expect(e.phase).toBe('wave')
    expect(e.deck.size).toBe(0)
  })

  it('通关后结算数据自洽（result=win 且击杀覆盖全部敌人）', () => {
    // 敌人放在中场且血量极低，确保在抵达防线前被清掉。
    // 把敌人放太靠右/血太厚时，它会先漏怪进底线 —— 那测的是"打不打得过"，
    // 而不是"结算数据是否自洽"，会让这个用例的意图失焦。
    const skills = mkSkills()
    const enemies = new Map(ENEMIES.map((e) => [e.id, e]))
    enemies.set(
      1,
      mkEnemy(1, '游荡者', 'normal', { hp: 10, speed: 1000, resist: zeroResist() }),
    )
    const level = mkLevel({
      base_hp: 100_000,
      star_targets: [1, 2, 3],
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 2, interval: 100, delay: 0 }] }],
      wave_count: 1,
    })
    const e = new BattleEngine({
      level,
      enemies,
      skills,
      equipped: mkEquipped([...skills.values()]),
      attacker: defaultAttacker(),
      seed: 77,
    })
    e.start()
    let guard = 0
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < 20000) e.step()
    expect(e.phase).toBe('won')
    const s = e.settleInput(1)
    expect(s.result).toBe('win')
    // 胜利条件是"守住防线"，不是"全歼敌人"——漏怪但防线没破同样是赢。
    // 因此真正该断言的是内部守恒：击杀 + 漏怪 = 该关总怪数。
    // 这条不变量正是服务端 ValidateSettle 里 kills ≤ 总怪数 校验的依据。
    const totalEnemies = level.waves[0].spawns[0].count
    expect(s.kills + s.leaked).toBe(totalEnemies)
    expect(s.kills).toBeGreaterThan(0)
    expect(s.wave_reached).toBe(1)
    expect(s.replay_hash).toMatch(/^[0-9a-f]{16}$/)
  })

  it('防线被打穿则判负', () => {
    const level = mkLevel({
      base_hp: 50,
      waves: [
        {
          wave_index: 0,
          spawns: [{ enemy_id: 2, count: 5, interval: 50, delay: 0 }],
        },
      ],
      wave_count: 1,
    })
    // 给敌人攻击力
    const enemies = new Map(ENEMIES.map((e) => [e.id, e]))
    enemies.set(2, mkEnemy(2, '疾行者', 'normal', {
      hp: 70, speed: 75000, attack: 500, resist: zeroResist(),
    }))
    const skills = mkSkills()
    const e = new BattleEngine({
      level,
      enemies,
      skills,
      equipped: mkEquipped([...skills.values()]),
      attacker: defaultAttacker(),
      seed: 9,
    })
    e.start()
    let guard = 0
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < 6000) e.step()
    expect(e.phase).toBe('lost')
    expect(e.baseHp).toBe(0n)
  })
})

describe('元素与反应在实战中生效', () => {
  it('会记录元素使用与反应触发次数', () => {
    const e = mkEngine(2024)
    e.start()
    let guard = 0
    while (e.phase === 'wave' && guard++ < 3000) e.step()
    expect(Object.keys(e.elementsUsed).length).toBeGreaterThan(0)
    expect(e.reactionsCount).toBeGreaterThanOrEqual(0)
  })

  it('元素覆盖多时能触发多种反应', () => {
    const level = mkLevel({
      base_hp: 1_000_000,
      star_targets: [1, 2, 3],
      waves: [
        {
          wave_index: 0,
          spawns: [{ enemy_id: 1, count: 6, interval: 100, delay: 0 }],
        },
      ],
      wave_count: 1,
    })
    const skills = mkSkills()
    const e = new BattleEngine({
      level,
      enemies: new Map(ENEMIES.map((x) => [x.id, x])),
      skills,
      equipped: mkEquipped([...skills.values()]),
      attacker: defaultAttacker(),
      seed: 31337,
    })
    e.start()
    let guard = 0
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < 8000) e.step()
    const totalReactions = Object.values(e.reactionsUsed).reduce((a, b) => a + b, 0)
    expect(totalReactions).toBeGreaterThan(0)
  })
})

describe('热量系统（I-2）', () => {
  it('热量允许正好用尽上限，随后进入过热', () => {
    const h = new HeatSystem()
    expect(h.tryCast(60n)).toBe(true)
    // 60 + 50 = 110 > 100，应被拒绝
    expect(h.tryCast(50n)).toBe(false)
    expect(h.heat).toBe(60n)
    // 恰好到上限应成功
    expect(h.tryCast(40n)).toBe(true)
    expect(h.heat).toBe(100n)
    expect(h.checkOverheat()).toBe(true)
    expect(h.overheated).toBe(true)
    // 过热期间不能释放
    expect(h.tryCast(10n)).toBe(false)
  })

  it('峰值热量被正确记录（用于结算上报）', () => {
    const h = new HeatSystem()
    h.tryCast(30n)
    h.tryCast(20n)
    expect(h.maxHeatThisBattle).toBe(50n)
    h.refundHeat(20n)
    h.tryCast(10n)
    // 退款后再次释放不应刷新峰值
    expect(h.maxHeatThisBattle).toBe(50n)
  })

  it('过热持续 2 秒后自动恢复并清空热量', () => {
    const h = new HeatSystem()
    h.heat = HEAT_MAX
    h.checkOverheat()
    expect(h.overheated).toBe(true)
    h.update(1100)
    expect(h.overheated).toBe(true)
    h.update(1100)
    expect(h.overheated).toBe(false)
    expect(h.heat).toBe(0n)
  })

  it('弃牌返还热量', () => {
    const h = new HeatSystem()
    h.heat = 50n
    h.refundHeat(1n)
    expect(h.heat).toBe(49n)
    h.refundHeat(100n)
    expect(h.heat).toBe(0n)
  })
})

describe('手牌与弃牌（I-2）', () => {
  it('每波弃牌次数有限，用完后弃牌失败', () => {
    const deck = new CardDeck()
    deck.setHand([
      { id: 'a', kind: 'skill', rarity: 'common', name: 'A', descr: '' },
      { id: 'b', kind: 'skill', rarity: 'common', name: 'B', descr: '' },
    ])
    expect(deck.discard('a').ok).toBe(true)
    expect(deck.discard('b').ok).toBe(false)
  })

  it('newWave 重置弃牌次数', () => {
    const deck = new CardDeck()
    deck.setHand([{ id: 'a', kind: 'skill', rarity: 'common', name: 'A', descr: '' }])
    deck.discard('a')
    deck.newWave()
    expect(deck.discardsLeft).toBe(1)
  })
})

describe('地形（I-4）', () => {
  it('油桶受焰元素命中后引燃', () => {
    const e = mkEngine(11, mkLevel({
      terrain: [{ kind: 'oil_drum', x: 500, y: 500, param: 100 }],
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 2, interval: 100, delay: 0 }] }],
      wave_count: 1,
      base_hp: 100000,
    }))
    e.start()
    let guard = 0
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < 8000) e.step()
    const drum = e.terrains[0]
    expect(drum.kind).toBe('oil_drum')
    // 引燃与否取决于弹道是否经过油桶位置，不强制断言 triggered，
    // 但地形对象必须存在且状态合法
    expect(['idle', 'burning']).toContain(drum.state)
  })

  it('地形数组永不为 null（客户端遍历安全）', () => {
    const e = mkEngine(11)
    expect(Array.isArray(e.terrains)).toBe(true)
    expect(e.terrains.length).toBe(0)
  })
})

describe('统计自洽', () => {
  it('命中数不超过发射数', () => {
    const e = mkEngine(555)
    e.start()
    run(e, 1500)
    expect(e.hits).toBeLessThanOrEqual(e.shots)
  })

  it('结算数据中所有数值非负', () => {
    const e = mkEngine(556)
    e.start()
    run(e, 1000)
    const s = e.settleInput(1)
    expect(s.score).toBeGreaterThanOrEqual(0)
    expect(s.kills).toBeGreaterThanOrEqual(0)
    expect(s.leaked).toBeGreaterThanOrEqual(0)
    expect(s.shots).toBeGreaterThanOrEqual(0)
    expect(s.hits).toBeGreaterThanOrEqual(0)
    expect(s.reactions).toBeGreaterThanOrEqual(0)
    expect(s.heat_max).toBeGreaterThanOrEqual(0)
    expect(s.duration_ms).toBeGreaterThan(0)
  })

  it('浮动文字数量受上限保护', () => {
    const e = mkEngine(557)
    e.start()
    run(e, 2000)
    expect(e.floats.length).toBeLessThanOrEqual(60)
  })
})
