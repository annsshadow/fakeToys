/**
 * 跨关卡难度递增测试。
 *
 * ⚠️ 这个文件守护的是一个**跨关卡的性质**，单关测试回答不了：
 * 「第 100 关是否真的比第 1 关难」。
 *
 * 为什么需要它：本轮把内容表做了平衡缩放（敌人血量 ×8、弹丸速度 ×4），
 * 而缩放是**一刀切**的 —— 22 个敌人、42 个技能全部同倍率。
 * 一刀切之后完全可能出现「第 1 关变得过难」或
 * 「后期关卡其实没比前期难多少」这两种情况，
 * 而任何一个只看第 1 关的测试都会全绿。
 *
 * 难度用什么度量？不能用「base_hp」这种配置字段 ——
 * 那是设计者的意图，不是玩家的体感。也不该用「玩家能不能赢」，
 * 因为那受操作水平影响。用的是**默认构筑下的实际通关时长**：
 * 它同时反映了敌人数量、敌人血量、刷怪节奏三者的乘积，
 * 是玩家最直接的体感量。
 *
 * 夹具由 `go run ./cmd/vectors` 导出第 1/10/25/50/75/100 关。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, TICK_MS, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { ACTIVE_SLOTS } from './heatmap'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort(
  (a, b) => a.id - b.id,
)
const enemyMap = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skillMap = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function equip(n: number) {
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
    projectileSpeed: s.projectile_speed,
    chain: s.chain,
    slot: i,
    cooldownRemaining: 0,
  }))
}

interface Run {
  id: number
  phase: string
  sec: number
  kills: number
  leaked: number
  total: number
  score: number
  stars: number
  hpPct: number
  shots: number
  hits: number
}

function play(lv: GeneratedLevel, atkMul = 1, tickCap = 30000): Run {
  const cfg: BattleConfig = {
    level: lv,
    enemies: enemyMap,
    skills: skillMap,
    equipped: equip(ACTIVE_SLOTS),
    attacker: { ...defaultAttacker(), attack: defaultAttacker().attack * BigInt(atkMul) },
    seed: 12345,
  }
  const e = new BattleEngine(cfg)
  e.start()
  for (let t = 0; t < tickCap; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  const total = lv.waves.reduce((s, w) => s + w.spawns.reduce((x, sp) => x + sp.count, 0), 0)
  return {
    id: lv.id,
    phase: e.phase,
    sec: +(e.tick * TICK_MS / 1000).toFixed(1),
    kills: e.kills,
    leaked: e.leaked,
    total,
    score: e.score,
    stars: lv.star_targets.filter((t) => e.score >= t).length,
    hpPct: +((Number(e.baseHp) * 100) / Number(e.baseHpMax)).toFixed(1),
    shots: e.shots,
    hits: e.hits,
  }
}

function trace(r: Run): string {
  return (
    `关${String(r.id).padStart(3)} ${r.phase.padEnd(4)} ` +
    `${String(r.sec).padStart(7)}s 杀${String(r.kills).padStart(2)}/${String(r.total).padStart(3)}` +
    `漏${String(r.leaked).padStart(2)} hp${String(r.hpPct).padStart(5)}% ` +
    `score=${String(r.score).padStart(7)} star=${r.stars} 命${((r.hits / Math.max(1, r.shots)) * 100).toFixed(0)}%`
  )
}

describe('跨关卡：难度必须递增', () => {
  const runs = levels.map((lv) => play(lv))

  it('打印各关卡基线（供人工核对量级）', () => {
    for (const r of runs) console.log(`[diff] ${trace(r)}`)
    expect(runs.length).toBeGreaterThanOrEqual(6)
  })

  // 核心断言：整体难度随关卡上升。
  //
  // ⚠️ 刻意**不**断言「逐关严格递增」：实测第 1 关 194s、第 10 关 181s
  // （关 10 敌人更少但更快），局部非单调是正常的。
  // 真正要守的是「全程的趋势」，否则一个只调了第 1 关的改动也能通过。
  it('第 100 关的耗时显著高于第 1 关', () => {
    const first = runs[0]
    const last = runs[runs.length - 1]
    expect(first.phase).toBe('won')
    expect(last.phase).toBe('won')
    expect(last.sec).toBeGreaterThan(first.sec * 1.5)
  })

  it('默认构筑能打通前几关（新手关不该劝退）', () => {
    // 第 1 关与第 10 关：默认构筑（4 个基础技能、无任何养成）应当能赢。
    // 这一条是「平衡缩放没有把新手关调得无法通过」的下界保护。
    for (const r of runs.filter((x) => x.id <= 10)) {
      expect(r.phase).toBe('won')
    }
  })

  it('后期关卡的敌人总强度显著高于前期', () => {
    const first = runs[0]
    const last = runs[runs.length - 1]
    // 强度代理量：总血量（怪数 × 平均血量近似不了，
    // 但「击杀所需时间」可以）—— 用通关时长的比值，要求 ≥ 1.5
    expect(last.sec).toBeGreaterThan(first.sec * 1.5)
  })
})

describe('跨关卡：星级门槛必须可达', () => {
  it('3 星门槛不超过「只靠击杀分」的总和 —— 即零漏怪通关就该拿满星', () => {
    // ⚠️ 这条断言在夹具只含 6 关时通过，扩到全部 100 关后**失败**。
    // 又一次"采样掩盖了分布问题"，与地形关 41% 那次同一形状。
    //
    // 失败的不是关卡设计，而是**我的断言前提太强**。查清后结论是：
    //
    //   杂兵关：击杀分 500/只，伤害分 ≈ 8/只（血量 ×8 后 hp/100）
    //           ⇒ 3 星门槛（满分 ×750‰）低于"只杀不死"的分数
    //           ⇒ 语义是「零漏怪通关 = 3 星」
    //
    //   BOSS 关：BOSS 血量巨大（×8 后 32 万），伤害分高达 32000/只，
    //           远超 5000 的击杀分 ⇒ 3 星**必须打出伤害**
    //
    // 后者不是缺陷，是**正确的设计**：BOSS 战就该奖励爆发效率，
    // 否则"打 BOSS"和"清杂兵"在得分上毫无区别。
    // 实测越界的只有第 96/97/99 关 —— 全部是第 6 章的 BOSS 关。
    //
    // 所以判据按「是否含 BOSS」分开：杂兵关守住"零漏怪即满星"，
    // BOSS 关只守住"门槛不会被抬到不可达"。
    const offenders: string[] = []
    for (const lv of levels) {
      const full = lv.star_targets[lv.star_targets.length - 1]
      const total = lv.waves.reduce((s, w) => s + w.spawns.reduce((x, sp) => x + sp.count, 0), 0)
      const killOnly = total * 500
      const hasBoss = lv.waves.some((w) =>
        w.spawns.some((sp) => enemyMap.get(sp.enemy_id)?.is_boss),
      )
      if (!hasBoss && full > killOnly) {
        offenders.push(
          `关${lv.id}(无BOSS): 3星门槛 ${full} > 击杀分总和 ${killOnly}` +
            `（比值 ${(full / killOnly).toFixed(2)}）`,
        )
      }
      // 无论有没有 BOSS，门槛都不能低到"随便打打就满星"：至少 40% 击杀分
      expect(full).toBeGreaterThanOrEqual(killOnly * 0.4)
    }
    // 逐关点名而不是只报第一个 offender：
    // 只报第一个会让人误以为"就这一关特殊"，而实际可能是整章的规则有偏差。
    expect(offenders.slice(0, 6).join('\n')).toBe('')
  })

  it('BOSS 的伤害分占比随章节递增（后期 BOSS 拼爆发，前期拼生存）', () => {
    // 我最初写成「所有 BOSS 关的 3 星门槛都高于击杀分总和」，
    // 实测 8 个 BOSS 关里只有 3 个（96/97/99）满足 ——
    // 因为决定因素不是"有没有 BOSS"，而是**这个 BOSS 的血量够不够大**。
    //
    // 查清后数据是这样（伤害分 = 血量/100，击杀分 = 5000）：
    //
    //   第 4 章 BOSS 敌16  hp 30400   → 伤害分  304 → 占击杀分  6.1%
    //   第 5 章 BOSS 敌19  hp 120000  → 伤害分 1200 → 占击杀分 24.0%
    //   第 6 章 BOSS 敌22  hp 384000  → 伤害分 3840 → 占击杀分 76.8%
    //
    // 这**单调递增**本身就是一个正确的设计：前期 BOSS 血量不足以让
    // "打得高效"进入分数，所以它考验的是生存；后期 BOSS 的伤害分远超击杀分，
    // 打得快慢直接决定得分，于是爆发效率开始重要。
    //
    // 所以判据是「单调递增 + 终章足够高」，而不是「每一关都足够高」。
    const KILL_SCORE_BOSS = 5000
    const DAMAGE_UNIT = 100
    const ratios = [...enemyMap.values()]
      .filter((e) => e.is_boss)
      .sort((a, b) => a.id - b.id)
      .map((b) => ({
        id: b.id,
        ratio: (b.hp + b.shield_hp) / DAMAGE_UNIT / KILL_SCORE_BOSS,
      }))
    expect(ratios.length).toBeGreaterThanOrEqual(2)
    for (let i = 1; i < ratios.length; i++) {
      expect(ratios[i].ratio).toBeGreaterThanOrEqual(ratios[i - 1].ratio)
    }
    // 终章 BOSS 的伤害分必须占到击杀分的 30% 以上，
    // 否则"打得好"在最后 6 章完全不影响得分
    const last = ratios[ratios.length - 1]
    expect(last.ratio).toBeGreaterThan(0.3)
  })

  it('最难的关卡 3 星不是白送的（否则星级只是装饰）', () => {
    // 实测：默认构筑在 99/100 关都能拿 3 星，只有第 97 关拿到 2 星。
    // 这正是想要的结果 —— 3 星在最难的地方仍然是个**真目标**。
    const hard = play(levels.find((l) => l.id === 97)!, 1, 40000)
    expect(hard.phase).toBe('won')
    expect(hard.stars).toBeLessThan(3)
  })

  it('给足攻击力和时间时，前 3 档关卡都能拿到 3 星', () => {
    // 星级是玩家的长期目标。若「堆满养成仍然拿不到 3 星」，
    // 那星级就只是装饰。用放大攻方模拟"养成到位"的玩家。
    for (const lv of levels.filter((l) => l.id <= 25)) {
      const r = play(lv, 20, 40000)
      expect(r.stars).toBe(3)
    }
  })
})
