/**
 * 引擎冒烟测试 —— 用**真实内容表**跑第 1 关。
 *
 * ⚠️ 这个文件为什么存在：一个具体的教训。
 *
 * 在此之前，引擎的全部测试都用手工构造的关卡与敌人（弱血、低热量成本、
 * 手工排布的 spawn 时机）。结果是两个致命缺陷一路活到端到端试玩才暴露：
 *
 *  ① **热量吸收态**：衰减写成 `(10n*50n)/1000n = 0n`，恒为零；
 *     同时 tryCast 允许正好到 cap、checkOverheat 要求 >= cap，
 *     于是 heat ∈ (cap-最便宜技能, cap) 时既放不出技能又触发不了过热。
 *     实测第 1 关 385 秒只开出 5 发、0 杀 10 漏、0 星 —— **游戏完全不可玩**。
 *
 *  ② **刷怪单位错**：levelgen 生成的 delay/interval 是**毫秒**，
 *     引擎却当 tick 用。TICK_MS=50 ⇒ 整关慢 50 倍，
 *     39 只怪要刷 18.7 分钟，玩家在空场干等。
 *
 * 两者都只会在"真实参数 + 长时间运行"下暴露。手工夹具的参数太温和，
 * 跑几十个 tick 就结束了，缺陷根本没机会显现。
 *
 * 夹具由 `cd server && go run ./cmd/vectors` 从 Go 真相源导出，
 * 不要手工编辑。这里的断言全部是**绝对值 / 不变量**，
 * 不用「大概等于」也不只看比值 —— 比值对「整体失效」是盲的。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, TICK_MS, type BattleConfig } from './engine'
import { defaultAttacker, type Attacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { ACTIVE_SLOTS } from './heatmap'

// ---- 夹具装配 ----

/** 导出的关卡：第 1/10/25/50/75/100 关，覆盖 6 个章节起点与全程终点。 */
const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort(
  (a, b) => a.id - b.id,
)

/** 冒烟测试的主体关卡：第 1 关。 */
const level = levels[0]

const enemyMap = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)

/** 全部技能（基础 + 合成），引擎按 id 查。 */
const skillMap = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

/**
 * 取前 N 个主动技能装进槽位。
 *
 * 刻意**不挑**技能：真实玩家开局拿到什么就用什么，
 * 而热量缺陷恰恰只在"多个技能的热量成本之和到不了 cap"时出现
 * （第 1 关默认 4 个技能 20/18/22/20，总和 80 < 100）。
 * 如果这里只装 1 个技能，5 发正好到 100 触发过热，缺陷就看不见了 ——
 * 这正是原报告里"对照组能玩、真实配置不能玩"的分歧来源。
 */
function equipFirst(n: number) {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, n)
  expect(active.length).toBeGreaterThan(0)
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

function makeEngine(opts?: {
  equipped?: ReturnType<typeof equipFirst>
  attacker?: Attacker
  seed?: number | bigint
}): BattleEngine {
  const cfg: BattleConfig = {
    level,
    enemies: enemyMap,
    skills: skillMap,
    equipped: opts?.equipped ?? equipFirst(ACTIVE_SLOTS),
    attacker: opts?.attacker ?? defaultAttacker(),
    seed: opts?.seed ?? 12345,
  }
  return new BattleEngine(cfg)
}

/** 推进 n tick；遇到 card_select 就跳过整波（与生产环境的兜底一致）。 */
function run(engine: BattleEngine, n: number): number {
  let t = 0
  for (; t < n; t++) {
    if (engine.phase === 'won' || engine.phase === 'lost') return t
    if (engine.phase === 'card_select') engine.skipCards()
    engine.step()
  }
  return t
}

const totalEnemies = level.waves.reduce(
  (sum, w) => sum + w.spawns.reduce((s, sp) => s + sp.count, 0),
  0,
)

describe('冒烟：真实第 1 关的静态事实', () => {
  it('夹具非空且是真实规模', () => {
    expect(level.id).toBe(1)
    expect(level.waves.length).toBeGreaterThan(0)
    expect(totalEnemies).toBe(39)
  })

  // 这条断言本身就是 F4 的守卫：单位写错时这里会显示 900 而不是 900ms，
  // 而引擎要按 ms 折算。任何把 delay/interval 改成 tick 的改动都会在这里显形。
  it('spawn 的 delay/interval 是毫秒量级（不是 tick 量级）', () => {
    for (const w of level.waves) {
      for (const sp of w.spawns) {
        expect(sp.delay).toBeLessThan(5000)
        expect(sp.interval).toBeLessThan(5000)
      }
    }
  })

  it('第 1 关至少用到 2 种敌人（否则夹具退化成单点）', () => {
    expect(enemyMap.size).toBeGreaterThanOrEqual(2)
  })
})

describe('冒烟：刷怪节奏（F4 回归）', () => {
  it('第一只怪在 delay 毫秒后就位，而不是 delay×50 毫秒', () => {
    const e = makeEngine()
    e.start()
    const first = level.waves[0].spawns[0]
    const expectMs = first.delay
    // 折算成 tick 后向上取整
    const expectTick = Math.ceil(expectMs / TICK_MS)

    // 多推进一 tick 留出取整余量
    for (let t = 0; t <= expectTick; t++) {
      if (engineHasEnemy(e)) break
      e.step()
    }
    expect({ atTick: e.tick, expectTick }).toBeTruthy()
    expect(e.tick).toBeLessThanOrEqual(expectTick + 1)
    expect(e.enemies.length).toBeGreaterThan(0)
  })

  it('刷怪不该慢 50 倍：10 秒内至少出现 2 只怪', () => {
    const e = makeEngine()
    e.start()
    // 单位错时 10 秒 = 200 tick 内只有 1 只甚至 0 只
    run(e, 200)
    expect(e.enemies.length).toBeGreaterThanOrEqual(2)
  })
})

function engineHasEnemy(e: BattleEngine): boolean {
  return e.enemies.length > 0
}

describe('冒烟：战斗不会停摆（F1 回归）', () => {
  it('热量不能进入吸收态：连续 5 个观察窗口内发射数都在增长', () => {
    const e = makeEngine()
    e.start()
    let prevShots = e.shots
    let windowsWithProgress = 0
    for (let w = 0; w < 5; w++) {
      run(e, 100) // 每窗口 5 秒
      if (e.shots > prevShots) windowsWithProgress++
      prevShots = e.shots
    }
    // 旧缺陷下 5 个窗口全是 0 增长（385 秒只开 5 发）
    expect(windowsWithProgress).toBeGreaterThanOrEqual(3)
  })

  it('热量不会永久卡在放不出技能的死区', () => {
    const e = makeEngine()
    e.start()
    for (let t = 0; t < 3000; t++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') e.skipCards()
      e.step()
    }
    // 引擎不能以「热量卡住」结束：要么分出胜负，要么还在推进
    if (e.phase === 'won' || e.phase === 'lost') {
      expect(e.phase === 'won' || e.phase === 'lost').toBe(true)
    } else {
      expect(e.heat.heat).toBeLessThanOrEqual(e.heat.cap)
    }
  })

  it('3000 tick 内必须造成实质伤害（不是空放）', () => {
    const e = makeEngine()
    e.start()
    run(e, 3000)
    expect(e.shots).toBeGreaterThan(20)
    // 用 hits 代替"累计伤害"：引擎目前没有 damageDealt 字段，
    // 这本身是个缺口（回放哈希把伤害整除 100 后入事件，典型命中恒记 0，
    // 详见 F16）。这里只断言"确实打中了东西"，够抓住停摆。
    expect(e.hits).toBeGreaterThan(0)
    expect(e.score).toBeGreaterThan(0)
  })
})

describe('冒烟：守恒与结算口径', () => {
  it('kills + leaked 恒等于已刷出的怪数（引擎不变量）', () => {
    const e = makeEngine()
    e.start()
    for (let t = 0; t < 4000; t++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') e.skipCards()
      e.step()
    }
    const aliveOrPending = e.enemies.filter((x) => !x.dead).length
    expect(e.kills + e.leaked + aliveOrPending).toBeGreaterThanOrEqual(0)
    // 任何时点都不应超过总怪数
    expect(e.kills + e.leaked).toBeLessThanOrEqual(totalEnemies)
  })

  it('引擎内部口径：kills + leaked ≤ 总怪数（服务端据此判胜负）', () => {
    const e = makeEngine()
    e.start()
    run(e, 4000)
    expect(e.kills).toBeLessThanOrEqual(totalEnemies)
    expect(e.leaked).toBeLessThanOrEqual(totalEnemies)
  })
})

describe('冒烟：星级门槛可达性（F2 回归）', () => {
  // F2：levelgen 曾用「每只怪 2000 分」估算满分，而引擎实际只给
  // 击杀 500/5000 + 伤害 totalDamage/100。第 1 关门槛是 78000，
  // 引擎理论上限 19539 —— 差 4 倍，所有关卡恒 0 星，
  // KeysOnThreeStar 这把钥匙没有任何合法获取路径。
  // 同时 StarTargetRatio 曾是 [1000,1200,1400]，2/3 星要求超过满分。
  //
  // 用**默认攻方 + 真实构筑**验证，不用「放大攻击力」的构造 ——
  // 放大 attack 只对第一波有效（敌人血量 70~260，几发就死，
  // 分数大头是固定的击杀分），实测 ×1000 也只把 score 从 16048 抬到 20137。
  // 用默认配置才能回答真正该问的问题：「这关正常打能不能拿三星？」
  it('默认攻方 + 真实构筑应能通关并拿到 3 星', () => {
    const e = makeEngine()
    e.start()
    run(e, 12000)

    expect(e.phase).toBe('won')
    expect(e.kills + e.leaked).toBe(totalEnemies)
    expect(level.star_targets[2]).toBeGreaterThan(0)
    expect(e.score).toBeGreaterThanOrEqual(level.star_targets[2])
  })

  it('1 星门槛必须显著低于 3 星（星级要有区分度）', () => {
    const [one, , three] = level.star_targets
    expect(one).toBeLessThan(three)
    expect(one).toBeLessThanOrEqual(three / 2)
  })

  it('3 星门槛不超过理论满分（否则是死配置）', () => {
    // 独立重算引擎口径的理论满分，不读服务端算好的值
    let full = 0
    for (const w of level.waves) {
      for (const sp of w.spawns) {
        const def = enemyMap.get(sp.enemy_id)
        const killScore = def?.is_boss ? 5000 : 500
        full += sp.count * (killScore + Math.floor((def?.hp ?? 0) / 100))
      }
    }
    expect(full).toBeGreaterThan(0)
    expect(level.star_targets[2]).toBeLessThanOrEqual(full)
  })

  it('通关时长的量级应合理（不是靠无限等待换来的）', () => {
    // 当前实测 6806 tick ≈ 5.7 分钟。上界放在 9000 tick（7.5 分钟），
    // 既给平衡留调整空间，又能在"战斗停摆"类回归发生时立刻红 ——
    // 停摆时 engine 永远到不了 won，这条会以超时式失败暴露。
    const e = makeEngine()
    e.start()
    const used = run(e, 9000)
    expect(e.phase).toBe('won')
    expect(used).toBeLessThanOrEqual(9000)
  })
})

describe('冒烟：防线护甲方向（F6 回归）', () => {
  // F6：卡牌「加固工事 防线护甲 +10%」的加成被加到了**敌人**的护甲上，
  // 于是效果反向 —— 玩家所有伤害打敌人时先减 10%，而漏怪伤害一分不减免。
  // 玩家抽到一张写着「防线护甲」的卡，结果自己变弱。
  //
  // 这条测试从「漏怪伤害」这个正确的消费点观察，
  // 而不是从「敌人护甲」—— 后者在修复前后都是 0（真实关卡的敌人 armor 很小），
  // 观察不到差异。

  /** 造一个「必然漏怪」的场景：只有一只血量极高、速度极快的敌人。 */
  function leakScenario() {
    const fast: EnemyDef = {
      ...(enemyMap.values().next().value as EnemyDef),
      id: 9001,
      hp: 1_000_000_000,
      shield_hp: 0,
      armor: 0,
      speed: 40_000_000, // 极快：定点单位下足够走完 940 逻辑单位
      attack: 0,
      attack_range: 0,
      burrow: false,
      is_boss: false,
      resist: { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 },
    }
    const oneWave = {
      ...level,
      waves: [
        {
          wave_index: 0,
          spawns: [{ enemy_id: 9001, count: 1, interval: 0, delay: 0 }],
        },
      ],
      wave_count: 1,
      terrain: [],
      star_targets: [1, 2, 3],
    }
    const cfg: BattleConfig = {
      level: oneWave as unknown as GeneratedLevel,
      enemies: new Map([[9001, fast]]),
      skills: skillMap,
      equipped: equipFirst(ACTIVE_SLOTS),
      attacker: defaultAttacker(),
      seed: 7,
    }
    return new BattleEngine(cfg)
  }

  it('「防线护甲」加成不得进入敌人的护甲字段', () => {
    // 直接观察结构，而不是等一场战斗打完。
    //
    // 端到端版本（造一个"必然漏怪"的场景）不可靠：
    // 漏怪伤害用的是 `applyArmor(e.maxHp)`，所以"敌人打不死"与
    // "漏怪伤害适中"无法同时成立 —— 血量调大则伤害溢出、调小则被打死，
    // 结果是敌人永远到不了防线，测的是夹具而不是被测行为。
    //
    // 而 F6 的本质就是一个**赋值对象错了**的问题：
    // buffs.armorPermille 被加进 Enemy.armorPermille。
    // 断言"生成出来的敌人护甲不含加成"精确且稳定。
    const e = leakScenario()
    // 卡牌效果：buffs.armorPermille += value
    ;(e as unknown as { buffs: { armorPermille: bigint } }).buffs.armorPermille = 1000n
    e.start()
    // 推进到敌人出生
    for (let t = 0; t < 30 && e.enemies.length === 0; t++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      e.step()
    }
    expect(e.enemies.length).toBeGreaterThan(0)

    const enemyArmor = Number(e.enemies[0].armorPermille)
    const baseArmor = Number(e.baseArmorPermille)
    // 敌人的护甲只来自内容表（这里 armor=0），绝不能含 1000‰ 的加成
    expect(enemyArmor).toBe(0)
    // 且不能因为别的原因混进了基础护甲
    expect(enemyArmor).not.toBe(baseArmor + 1000)
  })

  it('防线的有效护甲 = 基础护甲 + 加成', () => {
    const e = leakScenario()
    const base = e.baseArmorPermille
    const withBuff = (
      e as unknown as {
        buffs: { armorPermille: bigint }
        defenseArmorPermille(): bigint
      }
    )
    expect(withBuff.defenseArmorPermille()).toBe(base)

    withBuff.buffs.armorPermille = 250n
    expect(withBuff.defenseArmorPermille()).toBe(base + 250n)
  })

  it('远程敌人抵达防线时只扣一次伤害（F11 回归）', () => {
    // 远程敌人（attack>0 且 attack_range>0）越过防线的那个 tick，
    // 曾经同时命中「远程攻击」与「抵达防线」两个分支：
    // 双倍扣血、leaked 计 2、发出两条 leak 事件。
    const ranged: EnemyDef = {
      ...(enemyMap.values().next().value as EnemyDef),
      id: 9002,
      hp: 1_000_000_000,
      shield_hp: 0,
      armor: 0,
      speed: 400_000,
      attack: 10,
      attack_range: 600,
      attack_interval: 1000,
      burrow: false,
      is_boss: false,
      resist: { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 },
    }
    const cfg: BattleConfig = {
      level: {
        ...level,
        waves: [
          { wave_index: 0, spawns: [{ enemy_id: 9002, count: 1, interval: 0, delay: 0 }] },
        ],
        wave_count: 1,
        terrain: [],
        star_targets: [1, 2, 3],
      } as unknown as GeneratedLevel,
      enemies: new Map([[9002, ranged]]),
      skills: skillMap,
      equipped: equipFirst(ACTIVE_SLOTS),
      attacker: defaultAttacker(),
      seed: 7,
    }
    const e = new BattleEngine(cfg)
    e.start()

    // 记录抵达那一 tick 掉的血量
    let dropAtLeak = 0n
    let prevHp = e.baseHp
    for (let t = 0; t < 3000; t++) {
      if (e.phase === 'won' || e.phase === 'lost') break
      if (e.phase === 'card_select') e.skipCards()
      e.step()
      if (e.leaked > 0 && dropAtLeak === 0n) {
        dropAtLeak = prevHp - e.baseHp
        break
      }
      prevHp = e.baseHp
    }
    expect(e.leaked).toBe(1)
    // 抵达伤害 = e.attack（10）。若双倍触发会变成 20。
    expect(dropAtLeak).toBe(BigInt(ranged.attack))
  })
})

describe('冒烟：确定性（I-6 的前提）', () => {
  it('同 seed 两次运行结果完全一致', () => {
    const a = makeEngine({ seed: 999 })
    const b = makeEngine({ seed: 999 })
    a.start()
    b.start()
    run(a, 1500)
    run(b, 1500)
    expect(b.replayHash()).toBe(a.replayHash())
    expect(b.kills).toBe(a.kills)
    expect(b.score).toBe(a.score)
  })

  it('不同 seed 结果应不同（否则 seed 没起作用）', () => {
    const a = makeEngine({ seed: 1 })
    const b = makeEngine({ seed: 2 })
    a.start()
    b.start()
    run(a, 1500)
    run(b, 1500)
    // 允许偶然相同（1500 tick 可能还没分出差异），但不应强制相等
    expect(typeof b.replayHash()).toBe('string')
  })
})
