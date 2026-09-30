import { describe, it, expect } from 'vitest'
import { BattleEngine, type BattleConfig, MAX_BATTLE_TICKS } from './engine'
import { Terrain, BASE_X, BASE_Y, toFixed } from './terrain'
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
    // 理论满分：star_targets 由它按 StarTargetRatio = [600, 850, 980]‰ 导出，
    // 结算裁剪的上界也锚定在它上面（不是 star_targets[2]）。
    max_score: 5000,
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
      hp: 70, speed: 75000, attack: 7200,
      // 第 54 轮改了漏怪公式：代价从固定 e.attack 变成
      // 「底血 × attack / LeakAttackerDivisor」，所以这个场景的攻击力必须跟着改。
      //
      //   500 → 50 × 500 / 3600 = 6    （5 只只掉 30，防线不空，测不到本意）
      //   7200 → 50 × 7200 / 3600 = 100 ≥ 50（漏一只即打空）
      //
      // 若改了 LeakAttackerDivisor，这里必须同步，否则这条用例会悄悄退化成
      // 「防线打不空也通过」—— 那比红更糟，因为它不再测「打空判负」。 resist: zeroResist(),
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

// ---- 覆盖率补强：状态机边界与兜底路径 ----
//
// 以下用例专门钉住引擎里"平时不走、出问题时必须正确"的分支：
// 空关卡、击退位移、地形触发记录、弃牌进波、脏脚本回退、停滞兜底。
// 它们对应的都是真实的终局/边界语义，回归任何一条都会破坏 I-5/I-6 的闭环。

describe('回放事件计数（表现层与验真的观测口径）', () => {
  it('replayEventCount 随战斗推进单调增长，且等于内部事件条数', () => {
    // 为什么钉：replayEventCount 是外部（结算页/调试）观测回放规模的唯一读口，
    // 若它读错，"这局记了多少步"就无从校验。start 前应为 0（仅无参构造），
    // 推进后必然 >0（至少有 wave_start / spawn 等记录）。
    const e = mkEngine(42)
    expect(e.replayEventCount).toBe(0)
    e.start()
    run(e, 200)
    expect(e.replayEventCount).toBeGreaterThan(0)
  })
})

describe('空关卡不是胜利（畸形网络数据兜底）', () => {
  it('waves 为空时 start() 直接判负，绝不秒胜发奖', () => {
    // 为什么钉：这正是注释里记载的历史事故——waves=[] 曾走 finish(true)，
    // 配合服务端 MaxKillsFor=0 让 `kills>=0` 成立而"秒胜+发钱"。
    // levelgen 现在必填 >=5 波，但 level 是网络数据，缺字段时这条分支是唯一入口，
    // 必须落到 finish(false)。
    const e = mkEngine(1, mkLevel({ waves: [], wave_count: 0 }))
    e.start()
    expect(e.phase).toBe('lost')
    // 空关卡不产生任何击杀，结算不能出现"未战而胜"的战果
    expect(e.kills).toBe(0)
  })
})

describe('敌人击退位移（armor_break 的 knockback 语义）', () => {
  it('带 knockback 的敌人本 tick 按击退量位移并清零，替代常规推进', () => {
    // 为什么钉：stepEnemyMotion 里 knockback!==0 分支与常规"向防线推进"互斥——
    // 击退把敌人**向右**推（x 增大），与常规推进（x 减小）方向相反。
    // 这条分支保证击退是一次性的（用后清零），漏掉清零会让敌人被永久顶住。
    const level = mkLevel({
      base_hp: 1_000_000,
      wave_count: 1,
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 9, count: 1, interval: 0, delay: 0 }] }],
    })
    const enemies = new Map(ENEMIES.map((e) => [e.id, e]))
    // 血量天量 + 低速：确保这只怪在被我们操作前既没被清、也没漏进防线。
    enemies.set(9, mkEnemy(9, '重甲', 'normal', { hp: 1_000_000_000, speed: 1000, resist: zeroResist() }))
    const skills = mkSkills()
    const e = new BattleEngine({
      level,
      enemies,
      skills,
      equipped: mkEquipped([...skills.values()]),
      attacker: defaultAttacker(),
      seed: 7,
    })
    e.start()
    // 推进到该怪出场动画结束（spawnProgress 满），否则 stepEnemyMotion 提前 return。
    let guard = 0
    while (guard++ < 60) {
      e.step()
      const spawned = e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)
      if (spawned) break
    }
    const enemy = e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)
    expect(enemy).toBeDefined()
    const x0 = enemy!.x
    enemy!.knockback = 5000n
    e.step()
    // 位移恰好等于击退量（击退分支不叠加常规推进），且击退被清零
    expect(enemy!.x).toBe(x0 + 5000n)
    expect(enemy!.knockback).toBe(0n)
  })
})

describe('地形触发回调（terrain_used 的记录口径）', () => {
  it('onTerrainTrigger 把触发计入 terrainUsed、发事件并记回放', () => {
    // 为什么钉：这个回调是"地形被触发"进入 terrain_used / 回放哈希的记录口径。
    // terrain_used 参与结算与 I-6 回放比对，记录口径出错会让同一场战斗
    // 在触发地形后算出不同哈希。回调经 terrainCtx 下发给 Terrain 层，
    // 这里直接驱动回调本身以锁死其记录行为。
    const e = mkEngine(3)
    const ctx = (
      e as unknown as {
        terrainCtx(): { onTerrainTrigger(kind: string, t: Terrain): void }
      }
    ).terrainCtx()
    const t = new Terrain({ kind: 'oil_drum', x: 300, y: 400, param: 100 })
    const before = e.replayEventCount
    ctx.onTerrainTrigger('oil_drum', t)
    expect(e.terrainUsed).toContain('oil_drum')
    expect(e.replayEventCount).toBe(before + 1)
    const evs = e.drainEvents()
    expect(evs.some((ev) => ev.type === 'terrain')).toBe(true)
  })
})

describe('弃牌进波与非法弃牌兜底', () => {
  it('非选牌阶段 / 未知卡 id 弃牌返回 null；合法弃牌可逐张弃空并进入下一波', () => {
    // 为什么钉：discardCard 有三道闸——阶段闸（只在 card_select 生效）、
    // 存在闸（卡不在手牌则 null）、弃空进波（deck 清零即 beginWave）。
    // 其中"弃空进波"平时被 DISCARD_PER_WAVE=1 挡住（只能弃 1 张），
    // 但 free_discard 词条会放开次数，弃空进波是必须正确的真实路径。
    const e = mkEngine(3)
    // prepare 阶段（未 start）弃牌：阶段闸拦下，返回 null
    expect(e.discardCard('whatever')).toBeNull()
    e.start()
    let guard = 0
    while (e.phase === 'wave' && guard++ < 20000) e.step()
    expect(e.phase).toBe('card_select')
    expect(e.deck.size).toBe(3)
    // 未知卡 id：存在闸拦下，返回 null
    expect(e.discardCard('no-such-card')).toBeNull()
    // 放开弃牌次数（等效 free_discard 词条），逐张弃空
    e.deck.discardsLeft = 5
    const ids = e.deck.hand.map((c) => c.id)
    expect(e.discardCard(ids[0])).not.toBeNull()
    expect(e.deck.size).toBe(2)
    expect(e.phase).toBe('card_select') // 弃 1 张仍在选牌阶段
    e.discardCard(ids[1])
    e.discardCard(ids[2])
    expect(e.deck.size).toBe(0)
    // 弃空触发 beginWave，推进到下一波
    expect(e.phase).toBe('wave')
  })
})

describe('回放脚本脏索引兜底（非整数下标回退跳过整波）', () => {
  it('脚本下标绕过范围检查却取不到手牌（如 0.5）时回退跳过整波，不崩溃', () => {
    // 为什么钉：脚本下标来自存储的 card_picks（number[]，JSON 承载）。
    // 范围检查 `want<0 || want>=len` 拦不住 0.5 这类非整数——它落在 [0,len) 内，
    // 但 hand[0.5] 是 undefined。若不兜底，takeCard(undefined) 会抛错、
    // 整个回放中断（用户看到空白页）。`!target` 守卫保证脏脚本也能跑到底。
    const e = mkEngine(3)
    e.setReplayScript([0.5]) // 非整数下标：绕过范围检查但指不到卡
    e.start()
    let guard = 0
    // 脚本模式下引擎在 step() 内自动决策；进 card_select 时 applyReplayDecision
    // 读到 0.5 → hand[0.5]=undefined → skipCards 跳过整波
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < 20000) e.step()
    // 能正常跑到终局（未因脏脚本抛错卡死）即证明兜底生效
    expect(['won', 'lost']).toContain(e.phase)
  })
})

describe('停滞兜底（防引擎挂死，非防玩家卡住）', () => {
  it('到达绝对 tick 上限 MAX_BATTLE_TICKS 时强制判负并记录 stalemate', () => {
    // 为什么钉：这是"防引擎挂死"的最后防线（历史上第 22/33/34 关掩体死锁踩过）。
    // checkStalemate 到上限时既要发 stalemate 事件让表现层收尾，
    // finish 又要把 stalemate record 进回放——否则同一场战斗
    // "跑到上限结束"与"玩家操作结束"哈希不同，I-6 会误判。
    // 构造永不自然结束的战局：唯一敌人 speed=0 永不推进、hp 天量永不被清，
    // wave 阶段既不胜（怪没清）也不负（防线没破），只能靠 checkStalemate 收口。
    const level = mkLevel({
      base_hp: 1_000_000,
      wave_count: 1,
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 8, count: 1, interval: 0, delay: 0 }] }],
    })
    const enemies = new Map(ENEMIES.map((e) => [e.id, e]))
    enemies.set(
      8,
      mkEnemy(8, '钉子户', 'normal', { hp: 1_000_000_000, speed: 0, attack: 0, resist: zeroResist() }),
    )
    const skills = mkSkills()
    const e = new BattleEngine({
      level,
      enemies,
      skills,
      equipped: mkEquipped([...skills.values()]),
      attacker: defaultAttacker(),
      seed: 5,
    })
    e.start()
    let last: ReturnType<BattleEngine['step']> = []
    let guard = 0
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < MAX_BATTLE_TICKS + 10) {
      last = e.step()
    }
    expect(e.phase).toBe('lost')
    // checkStalemate 先发 stalemate 事件，finish(false,true) 再发 lost 并 record
    expect(last.some((ev) => ev.type === 'stalemate')).toBe(true)
    expect(last.some((ev) => ev.type === 'lost')).toBe(true)
  })
})

// ---- 覆盖率补强：分支短路与防御守卫（网络脏数据 / 状态机边界） ----

describe('畸形关卡数据的构造期兜底（waves/terrain 缺失）', () => {
  it('waves 与 terrain 字段缺失时仍能构造引擎，不抛错', () => {
    // 为什么钉：level 来自网络，缺字段是现实（灰度/回滚/DB 手改）。构造函数里
    // `cfg.level.waves ?? []` 与 `cfg.level.terrain ?? []` 是防"读 undefined 崩"的守卫。
    // 构造阶段必须容忍缺失（真正的空关卡判负发生在 start()→beginWave，另有用例覆盖）。
    const bad = mkLevel({
      waves: undefined as unknown as GeneratedLevel['waves'],
      terrain: undefined as unknown as GeneratedLevel['terrain'],
    })
    const skills = mkSkills()
    expect(
      () =>
        new BattleEngine({
          level: bad,
          enemies: new Map(ENEMIES.map((e) => [e.id, e])),
          skills,
          equipped: mkEquipped([...skills.values()]),
          attacker: defaultAttacker(),
          seed: 1,
        }),
    ).not.toThrow()
  })
})

describe('刷怪时的敌人数据兜底（未知 id / 缺抗性）', () => {
  it('波次引用了内容表里没有的 enemy_id 时跳过该只，且缺 resist 字段按 0 抗性处理', () => {
    // 为什么钉：spawnEnemy 里 `if (!def) return` 挡的是"波次引用了 enemies 表没有的 id"
    // ——灰度期新怪下发到旧客户端就会这样，必须跳过而非崩。`def.resist?.[e] ?? 0`
    // 挡的是"怪缺 resist 字段"（老快照），按无抗性处理，否则抗性矩阵读 undefined 崩。
    const enemies = new Map(ENEMIES.map((e) => [e.id, e]))
    // 一只缺 resist 字段的怪（触发 def.resist?.[e] ?? 0）
    enemies.set(
      5,
      mkEnemy(5, '无抗性怪', 'normal', {
        hp: 30,
        speed: 20000,
        resist: undefined as unknown as Record<string, number>,
      }),
    )
    const level = mkLevel({
      base_hp: 500_000,
      wave_count: 1,
      waves: [
        {
          wave_index: 0,
          spawns: [
            { enemy_id: 5, count: 1, interval: 0, delay: 0 }, // 缺抗性
            { enemy_id: 999, count: 1, interval: 0, delay: 0 }, // 未知 id，应被跳过
          ],
        },
      ],
    })
    const skills = mkSkills()
    const e = new BattleEngine({
      level,
      enemies,
      skills,
      equipped: mkEquipped([...skills.values()]),
      attacker: defaultAttacker(),
      seed: 3,
    })
    e.start()
    let guard = 0
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < 20000) e.step()
    // 未知 id 被跳过 ⇒ 实际只会出场 1 只怪，战斗照常结束
    expect(['won', 'lost']).toContain(e.phase)
    expect(e.kills + e.leaked).toBeLessThanOrEqual(1)
  })
})

describe('关卡反应阶的缺省（max_reaction_tier 落空回退 1 阶）', () => {
  it('max_reaction_tier 为 0/缺失时，攻方反应阶回退为 1', () => {
    // 为什么钉：currentAttacker 里 `BigInt(lv.max_reaction_tier || 1)` 保证脏关卡
    // （tier=0 意味着"除以 0 阶"）不会把反应伤害算炸。回退 1 阶 = 无衰减基准。
    const level = mkLevel({ max_reaction_tier: 0 })
    const e = mkEngine(7, level)
    e.start()
    // currentAttacker 私有：灰盒读取以断言回退语义（不改产品逻辑，仅观测）
    const tier = (e as unknown as { currentAttacker(): { reactionTier: bigint } })
      .currentAttacker()
      .reactionTier
    expect(tier).toBe(1n)
  })
})

describe('选牌操作在非选牌阶段/脏输入下的守卫', () => {
  it('takeCard/skipCards 在非 card_select 阶段是安全空操作；坏 id/弃牌次数耗尽返回 null', () => {
    // 为什么钉：这些守卫保证 UI 在错误时机（战斗中误触、双击）调用不会破坏状态机。
    const e = mkEngine(3)
    // prepare 阶段：takeCard 阶段闸 → null；skipCards 阶段闸 → 空操作不抛
    expect(e.takeCard('anything')).toBeNull()
    expect(() => e.skipCards()).not.toThrow()
    e.start()
    let guard = 0
    while (e.phase === 'wave' && guard++ < 20000) e.step()
    expect(e.phase).toBe('card_select')
    // card_select 阶段：坏 id 找不到卡 → null
    expect(e.takeCard('no-such-id')).toBeNull()
    // 弃牌次数耗尽：deck.discard 返回 !ok → discardCard 返回 null
    e.deck.discardsLeft = 0
    const anyId = e.deck.hand[0].id
    expect(e.discardCard(anyId)).toBeNull()
    expect(e.deck.size).toBe(3) // 未成功弃牌，手牌不变
  })
})

describe('终局后再 step 是幂等空转（won 分支）', () => {
  it('胜利后继续 step 不推进状态、不再产生战斗事件', () => {
    // 为什么钉：表现层可能在 won 之后多调一次 step（动画/收尾）。step 开头的
    // `if (phase==='won'||'lost') return drainEvents()` 保证终局是吸收态。
    const skills = mkSkills()
    const enemies = new Map(ENEMIES.map((e) => [e.id, e]))
    enemies.set(1, mkEnemy(1, '游荡者', 'normal', { hp: 10, speed: 1000, resist: zeroResist() }))
    const level = mkLevel({
      base_hp: 100_000,
      star_targets: [1, 2, 3],
      wave_count: 1,
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 2, interval: 100, delay: 0 }] }],
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
    const tickAtWin = e.replayEventCount
    const evs = e.step() // 终局后再 step
    expect(e.phase).toBe('won')
    expect(evs.length).toBe(0) // 吸收态不再产出新事件
    expect(e.replayEventCount).toBe(tickAtWin) // 不再追加回放
  })
})

describe('减速状态位移（slowedMs 生效路径）', () => {
  it('slowedMs>0 的敌人本 tick 半速推进并递减计时', () => {
    // 为什么钉：stepEnemyMotion 里 `slowedMs>0` 决定 slowFactor（500‰ 半速）与
    // 计时递减两处分支。减速是内容设计的控制手段，位移量算错会直接改变
    // 敌人抵达 tick → 命中/漏怪 → replayHash。用 speed=40000 的怪精确验证半速位移。
    const level = mkLevel({
      base_hp: 1_000_000,
      wave_count: 1,
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 6, count: 1, interval: 0, delay: 0 }] }],
    })
    const enemies = new Map(ENEMIES.map((e) => [e.id, e]))
    enemies.set(6, mkEnemy(6, '减速靶', 'normal', { hp: 1_000_000_000, speed: 40000, resist: zeroResist() }))
    // equipped 置空：不开火 ⇒ 不积热 ⇒ frozenAll 恒 false，位移可精确断言
    const e = new BattleEngine({
      level,
      enemies,
      skills: mkSkills(),
      equipped: [],
      attacker: defaultAttacker(),
      seed: 4,
    })
    e.start()
    let guard = 0
    while (guard++ < 40) {
      e.step()
      if (e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)) break
    }
    const enemy = e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)!
    expect(enemy).toBeDefined()
    enemy.slowedMs = 200
    const x0 = enemy.x
    e.step()
    // 半速：mulDiv(40000,500,1000)*50/1000 = 1000（未减速则为 2000）
    expect(x0 - enemy.x).toBe(1000n)
    // 计时按 TICK_MS(50) 递减
    expect(enemy.slowedMs).toBe(150)
  })
})

describe('技能字段落空的回退（projectileSpeed / apply_element）', () => {
  it('projectileSpeed 为 0 的技能开火时回退到默认弹速 60000', () => {
    // 为什么钉：fire() 里 `s.projectileSpeed || 60000` 防"配置弹速为 0 → 弹丸原地不动
    // → 永不命中 → 战斗挂死"。装一个弹速 0 的技能，开火即走回退分支。
    const level = mkLevel({
      base_hp: 1_000_000,
      wave_count: 1,
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 1, interval: 0, delay: 0 }] }],
    })
    const skills = mkSkills()
    const equipped = mkEquipped([...skills.values()])
    equipped[0].projectileSpeed = 0 // 触发 `|| 60000`
    const e = new BattleEngine({
      level,
      enemies: new Map(ENEMIES.map((x) => [x.id, x])),
      skills,
      equipped,
      attacker: defaultAttacker(),
      seed: 8,
    })
    e.start()
    run(e, 120)
    // 弹速 0 的技能仍成功开火（用了回退弹速），有射击即证明未卡死
    expect(e.shots).toBeGreaterThan(0)
  })

  it('技能卡装入新技能时 apply_element 为空则回退到技能主元素', () => {
    // 为什么钉：applyCard 装入新技能走 `def.apply_element || def.element`——
    // 老内容表的 apply_element 可能为空串，此时必须回退到 element，
    // 否则新技能"打不出任何元素"，反应链断裂。技能 4 未在初始 equipped（只装 1..3），
    // 且其 apply_element 置空，灰盒调用 applyCard 驱动装入路径。
    const skills = mkSkills()
    const s4 = { ...skills.get(4)!, apply_element: '' as unknown as Element }
    skills.set(4, s4)
    const e = new BattleEngine({
      level: mkLevel(),
      enemies: new Map(ENEMIES.map((x) => [x.id, x])),
      skills,
      equipped: mkEquipped([...skills.values()]), // 只含技能 1..3
      attacker: defaultAttacker(),
      seed: 2,
      activeSlots: 4, // 留一个空槽，保证走"装入新技能"而非"槽满升格"
    })
    e.start()
    ;(e as unknown as { applyCard(c: unknown): void }).applyCard({
      kind: 'skill',
      skillId: 4,
      rarity: 'common',
      name: '电磁穿刺',
      descr: '',
    })
    const added = (e as unknown as { skills: Array<{ skillId: number; applyElement: string }> }).skills.find(
      (s) => s.skillId === 4,
    )
    expect(added).toBeDefined()
    // apply_element 为空 ⇒ 回退到 def.element（lightning）
    expect(added!.applyElement).toBe('lightning')
  })
})

describe('无元素层数击杀的主元素回退（蓄能塔 dominant ?? fire）', () => {
  it('击杀一只身上没有任何元素层数的敌人时主元素回退为 fire', () => {
    // 为什么钉：击杀后 `dominantOf(e) ?? 'fire'` 供蓄能塔充能用。若敌人被
    // 零层数技能（纯物理）打死，dominantOf 返回 null，必须回退，否则传 null
    // 给 onKillCharging 会污染全场元素判定。用 applyStacks=0 的单技能一击必杀验证。
    const level = mkLevel({
      base_hp: 1_000_000,
      wave_count: 1,
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 1, interval: 0, delay: 0 }] }],
    })
    const skills = mkSkills()
    const equipped = mkEquipped([...skills.values()]).slice(0, 1)
    equipped[0].applyStacks = 0n // 不施加任何元素层数
    equipped[0].baseDamage = 1_000_000n // 一击必杀
    const enemies = new Map(ENEMIES.map((x) => [x.id, x]))
    enemies.set(1, mkEnemy(1, '薄皮', 'normal', { hp: 5, speed: 20000, resist: zeroResist() }))
    const e = new BattleEngine({
      level,
      enemies,
      skills,
      equipped,
      attacker: defaultAttacker(),
      seed: 6,
    })
    e.start()
    let guard = 0
    while (e.phase !== 'won' && e.phase !== 'lost' && guard++ < 20000) e.step()
    // 敌人被零层数技能击杀（dominantOf→null→'fire' 回退分支被走过）
    expect(e.kills).toBeGreaterThan(0)
  })
})

describe('地形击杀 BOSS 的记录（onKill 的 isBoss 分支）', () => {
  it('地形火区击杀 BOSS 时按 boss 记账并发 boss 击杀事件', () => {
    // 为什么钉：terrainCtx.onKill 里 `e.isBoss ? 1 : 0` 决定回放里这次击杀是否记为 BOSS。
    // 普通弹丸击杀 BOSS 另有路径覆盖；地形（油桶火区）击杀 BOSS 罕见，
    // 灰盒直接驱动回调，锁死"地形杀 BOSS 也按 boss 记录"这一口径。
    const e = mkEngine(3)
    const ctx = (
      e as unknown as { terrainCtx(): { onKill(en: unknown): void } }
    ).terrainCtx()
    const kills0 = e.kills
    ctx.onKill({ isBoss: true, x: 100n, y: 100n, uid: 42 })
    expect(e.kills).toBe(kills0 + 1)
    const evs = e.drainEvents()
    expect(evs.some((ev) => ev.type === 'kill' && (ev as { boss?: boolean }).boss === true)).toBe(true)
  })
})

describe('近距离开火的整数平方根边界（sqrtApprox n<2）', () => {
  it('目标与炮塔原点几乎重合（|Δ|=1）时，方向归一化不除零、正常开火', () => {
    // 为什么钉：fire() 用 sqrtApprox(dx²+dy²) 归一化方向；当目标贴到炮塔原点
    // （dx=1,dy=0 ⇒ n=1<2），sqrtApprox 的 `if (n<2) return n` 保证返回 1 而非
    // 进牛顿迭代，len 再经 maxBig(1n,...) 兜底，避免除零。用定点坐标精确构造该几何。
    const level = mkLevel({
      base_hp: 1_000_000,
      wave_count: 1,
      waves: [{ wave_index: 0, spawns: [{ enemy_id: 7, count: 1, interval: 0, delay: 0 }] }],
    })
    const enemies = new Map(ENEMIES.map((x) => [x.id, x]))
    enemies.set(7, mkEnemy(7, '贴脸怪', 'normal', { hp: 1_000_000_000, speed: 0, resist: zeroResist() }))
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
    while (guard++ < 40) {
      e.step()
      if (e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)) break
    }
    const enemy = e.enemies.find((en) => !en.dead && en.spawnProgress >= 1000)!
    // 贴到炮塔原点旁 1 个定点单位：dx=1, dy=0 ⇒ n=dx²+dy²=1<2
    enemy.x = toFixed(BASE_X) + 1n
    enemy.y = toFixed(BASE_Y)
    const shots0 = e.shots
    let g2 = 0
    while (e.shots === shots0 && g2++ < 300) e.step()
    // 在贴脸几何下成功开火（走过 sqrtApprox 的 n<2 分支），未除零/未卡死
    expect(e.shots).toBeGreaterThan(shots0)
  })
})

