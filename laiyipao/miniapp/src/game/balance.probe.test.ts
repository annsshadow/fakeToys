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
    level,
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
