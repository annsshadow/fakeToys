/**
 * 平衡基准测量 —— 跑真实关卡，输出可比较的指标。
 *
 * ⚠️ 这个文件不是测试，是一个**测量工具**。
 * 定位：任何涉及数值的改动（伤害公式、关卡生成、敌人血量、星级门槛）
 * 前后都跑一遍，拿到的是可比数字而不是"感觉变强了"。
 *
 * 之前的三次误判都源于缺这个：
 *   - 「热量上限压制攻击力收益」→ 实际是单位误读
 *   - 「夹具太弱」→ 实际是夹具温和度掩盖了粗糙缺陷
 *   - 「attack 边际收益低」→ 部分成立（敌人血量太低），但没量化过
 *
 * 用法：npx vitest run --config vitest.config.ts src/game/balance.probe.test.ts
 * 输出会以 [bal] 开头打印到控制��。
 */
import { describe, it } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, TICK_MS, type BattleConfig } from './engine'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { ACTIVE_SLOTS } from './heatmap'
import { resolveHit, Defender, defaultAttacker } from './damage'

/** 探针只跑第 1 关（它要的是可控的对照实验，不是难度梯度）。 */
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

function equip(n: number, projMul = 1) {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, n)
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
    projectileSpeed: Math.round(s.projectile_speed * projMul),
    chain: s.chain,
    slot: i,
    cooldownRemaining: 0,
  }))
}

interface Metrics {
  phase: string
  ticks: number
  sec: number
  shots: number
  hits: number
  hitRate: number
  kills: number
  leaked: number
  score: number
  stars: number
  hpPct: number
  reactions: number
  maxHeat: bigint
  total: number
}

function measure(
  opts: {
    nSkills?: number
    atkMul?: number
    crit?: number
    ticks?: number
    seed?: number
    /** 敌人血量倍率。用于不改内容表就做参数实验。 */
    hpMul?: number
    /** 敌人移速倍率。速度影响漏怪率与通关时长。 */
    speedMul?: number
    /** 弹丸速度倍率。当前内容表 45~80 单位/秒，跨 900 单位要 18 秒。 */
    projMul?: number
    /** 跑哪一关。缺省为第 1 关（它要的是可控的对照实验）。 */
    level?: GeneratedLevel
  } = {},
): Metrics {
  const hpMul = opts.hpMul ?? 1
  const speedMul = opts.speedMul ?? 1
  const projMul = opts.projMul ?? 1
  const enemies = new Map<number, EnemyDef>()
  for (const [id, d] of enemyMap) {
    enemies.set(id, {
      ...d,
      hp: Math.round(d.hp * hpMul),
      shield_hp: Math.round(d.shield_hp * hpMul),
      speed: Math.round(d.speed * speedMul),
    })
  }
  const skills = new Map<number, SkillDef>()
  for (const [id, s] of skillMap) {
    skills.set(id, { ...s, projectile_speed: Math.round(s.projectile_speed * projMul) })
  }
  const cfg: BattleConfig = {
    level: opts.level ?? level,
    enemies,
    skills,
    equipped: equip(opts.nSkills ?? ACTIVE_SLOTS, projMul),
    attacker: {
      ...defaultAttacker(),
      attack: defaultAttacker().attack * BigInt(opts.atkMul ?? 1),
      critPermille: BigInt(opts.crit ?? 50),
    },
    seed: opts.seed ?? 12345,
  }
  const e = new BattleEngine(cfg)
  e.start()
  const limit = opts.ticks ?? 20000
  for (let t = 0; t < limit; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return {
    phase: e.phase,
    ticks: e.tick,
    sec: +(e.tick * TICK_MS / 1000).toFixed(1),
    shots: e.shots,
    hits: e.hits,
    hitRate: e.shots > 0 ? +(e.hits / e.shots).toFixed(3) : 0,
    kills: e.kills,
    leaked: e.leaked,
    score: e.score,
    stars: calcStars(e.score),
    hpPct: +(Number(e.baseHp) * 100 / Number(e.baseHpMax)).toFixed(1),
    reactions: e.reactionsCount,
    maxHeat: e.heat.maxHeatThisBattle,
    total: e.totalEnemies,
  }
}

function calcStars(score: number): number {
  let s = 0
  for (const t of level.star_targets) if (score >= t) s++
  return s
}

const totalEnemies = level.waves.reduce(
  (sum, w) => sum + w.spawns.reduce((s, sp) => s + sp.count, 0),
  0,
)

function line(tag: string, m: Metrics): string {
  return (
    `[bal] ${tag.padEnd(26)} ${m.phase.padEnd(5)} ` +
    `${String(m.sec).padStart(6)}s shots=${String(m.shots).padStart(4)} ` +
    `hit%=${(m.hitRate * 100).toFixed(0).padStart(3)} ` +
    `kills=${String(m.kills).padStart(2)}/${totalEnemies} leak=${String(m.leaked).padStart(2)} ` +
    `score=${String(m.score).padStart(6)} star=${m.stars} hp=${String(m.hpPct).padStart(5)}% ` +
    `react=${String(m.reactions).padStart(3)} heat=${m.maxHeat}`
  )
}

/**
 * 对单次命中做离线测量，返回三段伤害的构成。
 *
 * 放在引擎之外直接调 resolveHit，是为了拿到**分段**数值 ——
 * hit 事件只有 damage 总数，拆不出「多少来自元素层数、多少来自反应」。
 */
function sampleHit(enemyHp: bigint) {
  const def = new Defender(enemyHp, 0n, 0n)
  // 挂 2 层焰元素，让元素段与反应段都有输入
  def.applyElement('fire', 2n, 3n) // 2 层，上限 3 层
  const res = resolveHit(defaultAttacker(), def, {
    skillDamage: 100n,
    skillElement: 'fire',
    forceReaction: 'steam_burst',
    roll: 500,
  })
  const first = enemyMap.values().next().value as EnemyDef
  return {
    direct: res.directDamage,
    element: res.elementDamage,
    reaction: res.reactionDamage,
    total: res.totalDamage,
    enemyHp: first.hp,
  }
}

describe('平衡基准', () => {
  it('默认构筑', () => {
    console.log(line('默认(4技能,seed12345)', measure()))
  })

  it('技能数的影响（热量机制）', () => {
    for (const n of [1, 2, 3, 4]) {
      console.log(line(`${n} 技能`, measure({ nSkills: n })))
    }
  })

  it('攻击力的边际收益', () => {
    for (const mul of [1, 2, 5, 10, 50, 200, 1000]) {
      console.log(line(`attack x${mul}`, measure({ atkMul: mul })))
    }
  })

  it('暴击率（当前 10x 单位错下的实际表现）', () => {
    for (const c of [0, 50, 130, 250, 500, 1000]) {
      console.log(line(`critPermille=${c}`, measure({ crit: c })))
    }
  })

  it('多种子稳定性', () => {
    for (const seed of [1, 42, 12345, 99999]) {
      console.log(line(`seed=${seed}`, measure({ seed })))
    }
  })

  // 这一组是**参数实验**：不改内容表，只在探针里缩放敌人血量，
  // 观察各成长维度的边际收益是否随血量厚度恢复。
  //
  // 背景：当前内容表敌人 hp 70~260，而 attack=100 时单发直接伤害约 44。
  // 于是「多打一发」没有意义（2~3 发就死），导致：
  //   - 攻击力 ×200 之后完全饱和（再高也没用）
  //   - 暴击率 0→1000‰ 只差 0.02%（反正都能杀死，分数大头是固定击杀分）
  //   - 元素层数来不及叠加就死人，反应次数随攻击力单调归零
  // 也就是**整套养成维度都被压平了**，I-1/I-2 的设计意图感受不到。
  // 上一组实验推翻了一个假设：血量翻倍时击杀数几乎不变，
  // 说明「多打一发」并不是瓶颈。这组测**伤害构成**来定位真正的瓶颈。
  //
  // 要回答三个问题：
  //   1. 一次命中实际打多少伤害？与敌人 hp 相比是"一枪一个"还是"磨半天"？
  //   2. 直接伤害 / 元素段 / 反应段 各占多少？
  //      若反应段占了大头，说明玩法实际是"堆元素"而不是"堆攻击"——
  //      那才是 I-1 想要的方向，但需要确认它不是靠某个 bug 刷出来的。
  //   3. 每只敌人平均要挨多少发？发数远大于 1 才说明血量有意义。
  it('伤害构成诊断', () => {
    for (const hpMul of [1, 2, 3, 5]) {
      const enemies = new Map<number, EnemyDef>()
      for (const [id, d] of enemyMap) {
        enemies.set(id, { ...d, hp: Math.round(d.hp * hpMul) })
      }
      const cfg: BattleConfig = {
        level,
        enemies,
        skills: skillMap,
        equipped: equip(ACTIVE_SLOTS),
        attacker: defaultAttacker(),
        seed: 12345,
      }
      const e = new BattleEngine(cfg)
      e.start()
      for (let t = 0; t < 12000; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        e.step()
      }
      // hit 事件不携带分段伤害，改用累计：击杀数 × 敌人 hp 不足以反推，
      // 所以直接对单次命中做一次离线测量。
      const sample = sampleHit(10n)
      console.log(
        `[bal-dmg] hp x${String(hpMul).padEnd(2)} ` +
          `| 单次命中 直接=${sample.direct} 元素=${sample.element} 反应=${sample.reaction} ` +
          `合计=${sample.total} | 敌人hp=${sample.enemyHp} ` +
          `| 需要的命中次数≈${(Number(sample.enemyHp) / Math.max(1, Number(sample.total))).toFixed(2)} ` +
          `| 对局 hits=${e.hits} kills=${e.kills}`,
      )
    }
  })

  // 命中率 15% 的根因假设：弹丸速度 45~80 单位/秒，跨越 900 单位场地要
  // 约 18 秒，而弹丸瞄的是**发射瞬间**的敌人位置。敌人 18 秒走 540 单位，
  // 弹丸必然落在目标身后 —— 85% 的输出被浪费。
  //
  // 这一组只动弹丸速度，观察命中率与通关表现。
  it('弹丸速度对命中率的影响', () => {
    for (const projMul of [1, 2, 4, 8, 16]) {
      const m = measure({ projMul })
      console.log(
        `[bal-p] 弹丸 x${String(projMul).padEnd(3)} ` +
          `| 命中 ${(m.hitRate * 100).toFixed(0).padStart(3)}% ` +
          `| 时长 ${String(m.sec).padStart(6)}s ${m.phase.padEnd(4)} ` +
          `| 杀${String(m.kills).padStart(2)}漏${String(m.leaked).padStart(2)} ` +
          `| score=${String(m.score).padStart(6)} star=${m.stars} ` +
          `| hp=${String(m.hpPct).padStart(5)}% react=${String(m.reactions).padStart(3)}`,
      )
    }
  })

  // 命中率先修好之后，再回头看血量厚度的影响 —— 上一轮的结论
  //（"血量不是瓶颈"）是在命中率 15% 的前提下得出的，那个前提本身就是错的。
  it('命中率修好后的血量厚度影响', () => {
    for (const hpMul of [1, 2, 3, 5, 8]) {
      const base = measure({ projMul: 8, hpMul })
      const atk2 = measure({ projMul: 8, hpMul, atkMul: 2 })
      const crit0 = measure({ projMul: 8, hpMul, crit: 0 })
      const crit1000 = measure({ projMul: 8, hpMul, crit: 1000 })
      console.log(
        `[bal-p8] hp x${String(hpMul).padEnd(2)} ` +
          `| 命中 ${(base.hitRate * 100).toFixed(0).padStart(3)}% ` +
          `| ${String(base.sec).padStart(6)}s ${base.phase.padEnd(4)} ` +
          `| 杀${String(base.kills).padStart(2)}漏${String(base.leaked).padStart(2)} ` +
          `| react=${String(base.reactions).padStart(3)} ` +
          `| atk(x2)+${((atk2.score - base.score) / Math.max(1, base.score) * 100).toFixed(1).padStart(5)}% ` +
          `| 暴击(0->满)+${((crit1000.score - crit0.score) / Math.max(1, crit0.score) * 100).toFixed(1).padStart(5)}%`,
      )
    }
  })

  // 目标区间搜索。
  //
  // 两端都是硬约束，缺一不可：
  //   下界（太软）：全清零漏、hp 满剩余 → 分数被击杀分锁死，
  //                 攻击力与暴击的边际收益必然为 0（实测确实为 0.0%）。
  //   上界（太硬）：默认构筑打不过 → 「默认配置应该能通关」这条冒烟断言会红。
  //
  // 所以要找的是：**默认构筑能赢，但血量剩余 20~60%、漏 2~8 只怪**的区间 ——
  // 那才是「有压力、成长有梯度、且不会让人挫败」的位置。
  for (const projMul of [2, 4, 8]) {
    it(`搜索可玩区间 projMul=${projMul}`, () => {
      for (const hpMul of [1, 2, 4, 8, 16, 32]) {
        const base = measure({ projMul, hpMul })
        const atk2 = measure({ projMul, hpMul, atkMul: 2 })
        const atk4 = measure({ projMul, hpMul, atkMul: 4 })
        const c0 = measure({ projMul, hpMul, crit: 0 })
        const c1000 = measure({ projMul, hpMul, crit: 1000 })
        const atkGain = (atk2.score - base.score) / Math.max(1, base.score)
        const atkGain4 = (atk4.score - base.score) / Math.max(1, base.score)
        const critGain = (c1000.score - c0.score) / Math.max(1, c0.score)
        // 健康度标记：won + 血量 20~60% + 漏 1~10
        const healthy =
          base.phase === 'won' && base.hpPct >= 15 && base.hpPct <= 70 && base.leaked >= 1
        console.log(
          `[bal-grid] p${projMul} hp${String(hpMul).padEnd(2)} ` +
            `| ${String(base.sec).padStart(6)}s ${base.phase.padEnd(4)} ` +
            `| 命${(base.hitRate * 100).toFixed(0).padStart(3)}% ` +
            `| 杀${String(base.kills).padStart(2)}漏${String(base.leaked).padStart(2)} ` +
            `| hp${String(base.hpPct).padStart(5)}% react${String(base.reactions).padStart(3)} ` +
            `| atk(x2)${(atkGain * 100).toFixed(1).padStart(6)}% ` +
            `atk(x4)${(atkGain4 * 100).toFixed(1).padStart(6)}% ` +
            `crit${(critGain * 100).toFixed(1).padStart(6)}% ` +
            (healthy ? ' <= 健康' : ''),
        )
      }
    })
  }

  // 漏怪伤害修好（不再拿敌人血量当伤害）之后的定档扫描。
  //
  // 修好之前，18 个组合没有一个落在健康区间：血量低时全清零漏（成长收益 0%），
  // 血量高时漏一只即败（只剩"从失败到成功"的跳变）。
  // 现在要找的是同时满足三条的那一档：
  //   1) 默认构筑能通关（冒烟测试依赖）
  //   2) 血量剩余 20~60%、漏 1~10 只（有压力，不会让人觉得白打）
  //   3) 攻击力与暴击的边际收益都 > 3%（成长维度有梯度）
  it('定档扫描（漏怪伤害修复后）', () => {
    for (const projMul of [4]) {
      for (const hpMul of [16, 20, 24, 28, 32, 40, 48]) {
        const base = measure({ projMul, hpMul })
        const atk2 = measure({ projMul, hpMul, atkMul: 2 })
        const c0 = measure({ projMul, hpMul, crit: 0 })
        const c1000 = measure({ projMul, hpMul, crit: 1000 })
        const atkGain = (atk2.score - base.score) / Math.max(1, base.score)
        const critGain = (c1000.score - c0.score) / Math.max(1, c0.score)
        const win = base.phase === 'won'
        const pressured = base.hpPct >= 15 && base.hpPct <= 70 && base.leaked >= 1
        const gradient = atkGain > 0.03 && critGain > 0.03
        const ok = win && pressured && gradient
        console.log(
          `[bal-fix] p${projMul} hp${String(hpMul).padEnd(3)} ` +
            `| ${String(base.sec).padStart(6)}s ${base.phase.padEnd(4)} ` +
            `| 命${(base.hitRate * 100).toFixed(0).padStart(3)}% ` +
            `| 杀${String(base.kills).padStart(2)}漏${String(base.leaked).padStart(2)} ` +
            `| hp${String(base.hpPct).padStart(4)}% react${String(base.reactions).padStart(3)} ` +
            `| atk${(atkGain * 100).toFixed(1).padStart(6)}% crit${(critGain * 100).toFixed(1).padStart(6)}% ` +
            `| ${win ? 'W' : 'L'}${pressured ? 'P' : '-'}${gradient ? 'G' : '-'}` +
            (ok ? '  <= 选定' : ''),
        )
      }
    }
  })

  // 全 100 关的难度曲线。
  //
  // ⚠️ 夹具曾经只导出第 1/10/25/50/75/100 关，于是**第 25~75 关从未被
  // 任何东西跑过** —— 50 关是黑盒。平衡一旦在那段出问题（敌人血量跳档、
  // 关卡从"全清"直接跳到"打不过"），不会有任何测试变红。
  //
  // 代价：跑 100 场完整战斗。逐位守恒，没有近似。
  // 采样 100 关全跑太慢时按 step 抽样，但**每章至少取 3 关**，
  // 否则会漏掉章内的跳变。
  it('全 100 关难度曲线', () => {
    const all = fixture.levels as unknown as GeneratedLevel[]
    const rows: string[] = []
    let lastWon = true
    let regressions = 0
    for (const lv of all.slice().sort((a, b) => a.id - b.id)) {
      const m = measure({ level: lv })
      rows.push(
        `[bal-curve] 关${String(lv.id).padStart(3)} ` +
          `${m.phase.padEnd(4)} ${String(m.sec).padStart(6)}s ` +
          `| 怪${String(m.total).padStart(3)} 杀${String(m.kills).padStart(3)}` +
          `漏${String(m.leaked).padStart(3)} hp${String(m.hpPct).padStart(5)}% ` +
          `| 命${(m.hitRate * 100).toFixed(0).padStart(3)}% ` +
          `react${String(m.reactions).padStart(3)} ` +
          `heat${String(m.maxHeat).padStart(3)} ` +
          `score=${String(m.score).padStart(6)} star=${m.stars}`,
      )
      // 回归：上一关能过、这一关过不了，且不是首次
      if (lastWon && m.phase === 'lost' && lv.id > 1) regressions++
      lastWon = m.phase === 'won'
    }
    for (const r of rows) console.log(r)
    // 只报告，不在这里断言 —— 断言放在 difficulty.test.ts（单元测试要快且确定）
    console.log(
      `[bal-curve] 汇总：失败 ${rows.filter((r) => r.includes(' lost ')).length} 关，` +
        `难度回归点 ${regressions} 处`,
    )
  }, 300_000)

  // I-4 地形在真实对局里的**实际触发率**。
  //
  // ⚠️ 上一轮把 5 类地形的机制全修好了（param 标定、删死路径、位置判定），
  // 并给每一类补了单元测试。但单元测试证明的是"机制能工作"，
  // **不是"它在真实对局里会被触发"**。
  //
  // 这两件事的差距在本项目已经吃过一次亏：掩体（collapse_wall）的
  // `onHit` 单元测试全绿，但它的唯一充能源在真实对局里永远走不到，
  // 于是第 22/33/34 关死锁 100% 的时间。
  //
  // 所以这里统计：全 100 关跑完，每类地形**真正触发**（collapsed / burning /
  // 状态改变）多少次。触发数为 0 的类别说明它在真实对局里仍是死代码。
  it('地形生效率（5 类 × 100 关，按真实效果度量）', () => {
    const all = fixture.levels as unknown as GeneratedLevel[]
    type Stat = {
      placed: number
      effective: number
      levels: Set<number>
      /** 未生效者的峰值充能：区分「从没靠近」与「靠近但不够」 */
      maxCharge: number
      placedButIdle: string[]
    }
    const stats = new Map<string, Stat>()
    for (const kind of [
      'oil_drum',
      'tidal_gate',
      'rotor_vane',
      'collapse_wall',
      'charge_tower',
    ]) {
      stats.set(kind, {
        placed: 0,
        effective: 0,
        levels: new Set(),
        maxCharge: 0,
        placedButIdle: [],
      })
    }

    // ⚠️ 度量口径换过两次，每次都是因为**度量本身错了**：
    //
    //  1) 第一版用 `t.triggered` 标志 → 得出「油桶/潮汐闸/风障 0%」
    //     查代码才发现潮汐闸与风障**从不设 triggered**：
    //     它们是**持续型**机制（一直挡 / 一直偏转），
    //     用「一次性事件标志」度量持续型机制，必然得到假的 0%。
    //
    //  2) 第二版改成「状态动过就算」→ 潮汐闸/风障 100%，
    //     但那是**假阳性**：状态动过只说明 update 跑了，
    //     不说明它影响了任何东西。地形放在敌人走廊外时，
    //     潮汐闸照样每 2 秒切一次状态，却一次弹丸都没挡到。
    //
    // 现在的口径是**可观测的实际效果**：
    //     油桶   → 真的进入 burning（那意味着真的被火焰命中）
    //     掩体   → 真的 collapsed（真的被打崩）
    //     蓄能塔 → charge 真的涨过（真的有击杀喂给它）
    //     潮汐闸 → **真的挡掉过一发弹丸**
    //     风障   → **真的改过一发弹丸的速度**
    //
    // 挡与偏转通过包一层 updateProjectiles 观测 —— 它是地形影响弹丸的唯一入口。
    const dead: string[] = []
    for (const lv of all) {
      if (lv.terrain.length === 0) continue
      const cfg: BattleConfig = {
        level: lv,
        enemies: enemyMap,
        skills: skillMap,
        equipped: equip(ACTIVE_SLOTS),
        attacker: defaultAttacker(),
        seed: 12345,
      }
      const e = new BattleEngine(cfg)
      type T = {
        kind: string
        state: string
        charge: number
        x: number
        y: number
        param: number
      }
      const terrains = (e as unknown as { terrains: T[] }).terrains
      for (const t of terrains) stats.get(t.kind)!.placed++

      // ⚠️ 观测方式换过两次，每次都是**观测本身错了**：
      //
      //  1) 包一层 `updateProjectiles` 数"被挡掉的弹丸"
      //     → 潮汐闸误判为 0%：它挡的是**敌人**不是弹丸，
      //       `blocksProjectile()` 只对 collapse_wall 返回 true
      //  2) 包一层 `updateProjectiles` 比"速度变了"
      //     → 风障误判为 0%：偏转发生在 `updateTerrain` 里，
      //       而我在 `updateProjectiles` 前后取快照 ——
      //       取到的**已经是被改过的速度**，差值恒为 0
      //  3) 战斗**结束后**查"有没有敌人被拦在闸门右侧"
      //     → 又一次误判：赢下战斗时敌人已全部死亡
      //
      // 三次都是同一个形状：**观测点选错了，于是观测不到东西，
      // 而观测不到被读成"机制没生效"**。
      //
      // 现在的做法是逐 tick 观测引擎的公开状态，不碰任何私有方法：
      //   - 弹丸：记住上一 tick 的速度，本 tick 比对（任何来源的改变都能发现）
      //   - 敌人：只要"活着 && x >= 闸门 x && |y 差| < 60"就记为被拦
      const lastVel = new Map<number, { vx: bigint; vy: bigint }>()
      const deflected = new Set<string>()
      const heldEnemy = new Set<string>()
      type E = { dead: boolean; x: bigint; y: bigint }
      type P = { uid: number; vx: bigint; vy: bigint }
      const view = e as unknown as { enemies: E[]; projectiles: P[] }

      e.start()
      for (let t = 0; t < 20000; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        e.step()

        // 1) 偏转：与**上一 tick** 的速度比对
        for (const p of view.projectiles) {
          const prev = lastVel.get(p.uid)
          if (prev && (prev.vx !== p.vx || prev.vy !== p.vy)) deflected.add('rotor_vane')
          lastVel.set(p.uid, { vx: p.vx, vy: p.vy })
        }
        // 2) 拦敌：敌人被钉在闸门右侧且与闸门同一条线
        for (const t2 of terrains) {
          if (t2.kind !== 'tidal_gate') continue
          const gx = BigInt(t2.x * 1000)
          const gy = BigInt(t2.y * 1000)
          for (const en of view.enemies) {
            if (en.dead || en.x < gx) continue
            const dy = en.y - gy
            if (dy > -60000n && dy < 60000n) heldEnemy.add('tidal_gate')
          }
        }
      }

      for (const t of terrains) {
        let ok = false
        if (t.kind === 'oil_drum') ok = t.state === 'burning'
        else if (t.kind === 'collapse_wall') ok = t.state === 'collapsed'
        else if (t.kind === 'charge_tower') ok = t.charge > 0
        else if (t.kind === 'tidal_gate') ok = heldEnemy.has('tidal_gate')
        else if (t.kind === 'rotor_vane') ok = deflected.has('rotor_vane')
        const s = stats.get(t.kind)!
        if (!ok) {
          // 峰值充能：区分「从没靠近过」（充能 0）与
          // 「靠近过但累积不够」（充能 > 0 但不到 param）——
          // 这两种情况的修法完全不同。
          s.maxCharge = Math.max(s.maxCharge, t.charge)
          s.placedButIdle.push(`关${lv.id} charge=${t.charge}/${t.param} @y=${t.y}`)
        }
        if (ok) {
          s.effective++
          s.levels.add(lv.id)
        }
      }
    }
    const rows: string[] = []
    for (const [kind, s] of stats) {
      const rate = s.placed > 0 ? (s.effective / s.placed) * 100 : 0
      rows.push(
        `[bal-ter] ${kind.padEnd(14)} 放置 ${String(s.placed).padStart(3)} 生效 ` +
          `${String(s.effective).padStart(3)} (${rate.toFixed(0).padStart(3)}%) ` +
          `覆盖 ${s.levels.size} 关`,
      )
      if (s.placed > 0 && s.effective === 0) dead.push(kind)
    }
    for (const r of rows) console.log(r)
    for (const [kind, s] of stats) {
      if (s.effective === 0 && s.placed > 0) {
        console.log('[bal-ter]   ' + kind + ' 未生效: ' + s.placedButIdle.join(' | '))
      }
    }
    // 有放置但零效果 = 机制在真实对局里是死代码
    expect(dead.join(',')).toBe('')
  }, 600_000)

  // 卡牌 / 装备的加成在真实对局里**有没有被触发过**。
  //
  // ⚠️ 与 I-4 地形同源的怀疑：`applyAttribute` 与 `applyMechanic`
  // 都实现了 6 / 5 个 kind，代码看起来完整。但——
  //   - 内容表的卡到底带不带 attribute / mechanic？
  //   - `applyCard` 到底走不走这两个分支？
  //   - 装备的加成到底有没有进战斗？
  // 这三件事单元测试都不回答（它们直接调 `applyMechanic`，
  // 绕过了"卡是否真的带效果"与"是否真的被取用"）。
  //
  // 判据：跑真实关卡、真实抽卡，然后看 10 个 buff 字段里
  // 哪些**真的从零变过**。全零 = 那些加成是纯装饰。
  it('卡牌/装备加成的实际触发率', () => {
    const lv = (fixture.levels as unknown as GeneratedLevel[]).find((l) => l.id === 50)!
    const peak = new Map<string, number>()
    const cardKinds = new Map<string, number>()
    let cardsTaken = 0
    let heatCapPeak = 0n

    for (const seed of [12345, 777, 20250101, 42, 99999]) {
      const e = new BattleEngine({
        level: lv,
        enemies: enemyMap,
        skills: skillMap,
        equipped: equip(ACTIVE_SLOTS),
        attacker: defaultAttacker(),
        seed,
      })
      e.start()
      const eng = e as unknown as {
        buffs: Record<string, bigint | number>
        heat: { capBonus: bigint }
        deck: { hand: { id: string; kind: string; mechanic?: { kind: string }; effect?: { kind: string } }[] }
        takeCard(id: string): unknown
      }
      for (let t = 0; t < 20000; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') {
          // 真实取牌：走 takeCard（含 applyCard → applyAttribute/applyMechanic）
          const hand = eng.deck.hand
          if (hand.length > 0) {
            const c = hand[Math.floor((t % 7) / 3) % hand.length]
            const k = c.mechanic?.kind
              ? `mechanic:${c.mechanic.kind}`
              : c.effect
                ? `attr:${c.effect.kind}`
                : `${c.kind}:none`
            cardKinds.set(k, (cardKinds.get(k) || 0) + 1)
            eng.takeCard(c.id)
            cardsTaken++
          } else {
            e.skipCards()
          }
        }
        e.step()
        // ⚠️ 必须**逐 tick 记录峰值**，不能只看战斗结束时的值。
        //
        // 我第一版在战斗结束后采样，得到「overheatGuard 从未被触发」——
        // 而实际上那张卡被取了 4 次。原因是 overheatGuard **触发后即被消耗**
        // （Round 7 修的「隔热护罩」语义就是用掉一层），
        // 于是结束时它已经回到 0。
        //
        // 消耗型资源用"结束值"度量，必然得到假的 0。
        // 这与地形度量踩的是同一类坑：**度量点选错 → 观测不到 → 误判为没生效。**
        for (const [k, v] of Object.entries(eng.buffs)) {
          const cur = Number(v)
          if (cur > (peak.get(k) ?? 0)) peak.set(k, cur)
        }
        // 热量上限**不住在 Buffs 里** —— 它在 HeatMeter 上
        // （`get cap() { return HEAT_MAX + this.capBonus }`）。
        // Buffs 里曾经有一份只写不读的镜像，已删；所以这里必须单独采样，
        // 否则删掉镜像之后这个加成就会"从统计里消失"，
        // 而它其实是生效的。
        const hc = eng.heat.capBonus
        if (hc > heatCapPeak) heatCapPeak = hc
      }
    }
    const seen = new Set([...peak.entries()].filter(([, v]) => v !== 0).map(([k]) => k))
    if (heatCapPeak > 0n) seen.add('heatCapBonus')
    console.log(
      `[bal-buff] 取牌 ${cardsTaken} 次，卡面效果分布: ` +
        [...cardKinds.entries()].map(([k, v]) => `${k}x${v}`).join(' '),
    )
    console.log(`[bal-buff] 实际被改变的 buff: ${[...seen].sort().join(', ') || '（全部为 0）'}`)

    // 期望：pierce/chain/aoe/free_discard/overheatGuard（机制卡）
    //      + attack/elementCoef/crit/heatCap/elementCap/armor（属性卡）
    //
    // heatCapBonus 住在 HeatMeter 而不是 Buffs（Buffs 里那份只写不读的
    // 镜像已删），上面单独采样后并进 seen，所以它仍在这个名单里。
    const expected = [
      'pierceBonus',
      'chainBonus',
      'aoeBonus',
      'freeDiscard',
      'overheatGuard',
      'attackPermille',
      'elementCoefPermille',
      'critPermille',
      'heatCapBonus',
      'elementCapBonus',
      'armorPermille',
    ]
    const dead = expected.filter((x) => !seen.has(x))
    console.log(`[bal-buff] 从未被触发的加成: ${dead.join(', ') || '（无）'}`)
    expect(dead.join(',')).toBe('')
  }, 300_000)

  // 血量厚度对成长维度的影响（参数实验）
  it('血量厚度对成长维度的影响（参数实验）', () => {
    for (const hpMul of [1, 2, 5, 10, 20]) {
      const base = measure({ hpMul })
      const atk2 = measure({ hpMul, atkMul: 2 })
      const atk5 = measure({ hpMul, atkMul: 5 })
      const crit0 = measure({ hpMul, crit: 0 })
      const crit1000 = measure({ hpMul, crit: 1000 })
      console.log(
        `[bal] hp x${String(hpMul).padEnd(3)} ` +
          `| 时长 ${String(base.sec).padStart(6)}s ${base.phase.padEnd(4)} ` +
          `| 命中 ${(base.hitRate * 100).toFixed(0).padStart(3)}% ` +
          `| 杀${String(base.kills).padStart(2)}漏${String(base.leaked).padStart(2)} ` +
          `| react ${String(base.reactions).padStart(3)} ` +
          `| atk增益(x2) ${((atk2.score - base.score) / Math.max(1, base.score) * 100).toFixed(1).padStart(6)}% ` +
          `atk增益(x5) ${((atk5.score - base.score) / Math.max(1, base.score) * 100).toFixed(1).padStart(6)}% ` +
          `| 暴击增益(0->满暴) ${((crit1000.score - crit0.score) / Math.max(1, crit0.score) * 100).toFixed(1).padStart(6)}%`,
      )
    }
  })
})
