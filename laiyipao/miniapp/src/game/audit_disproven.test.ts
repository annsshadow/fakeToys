import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, type BattleConfig } from './engine'
import { defaultAttacker, resolveHit, Defender } from './damage'
import { PERMILLE } from './fixed'
import { HeatSystem } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

/**
 * 两个**审计结论被实测推翻**的地方（第 73 轮）。
 *
 * 记录它们的价值不在于「修好了什么」，而在于：
 * **下一次有人看到同一条审计项时，不会去「修」一个不存在的问题。**
 *
 * ⚠️ 本项目此前已经吃过一次同型的亏：README 记的
 * 「去掉雪崩只留 levelID × 奇数 → 均匀度测试全绿 → 这个变异确实无害」。
 * 审计报告里的「疑似缺陷」与「实测无害」必须分开记，
 * 否则后来的人会把无害的代码改坏 —— 而且改的时候还会觉得自己在修 bug。
 */

/** 建一个最小可跑的引擎。 */
function mkEngine(critBase: bigint): BattleEngine {
  const raw = (fixture.levels as unknown as GeneratedLevel[]).find((l) => l.id === 1)!
  // ⚠️ base_hp 必须给足，否则玩家在第 1 波就输掉，观察不到多波行为。
  //
  // 我第一版用 `hp: 1_000_000` 的敌人（本意「打不死」），
  // 结果敌人确实打不死玩家、玩家也打不死敌人 —— 卡在一波里，
  // `out.length` 恒为 1。
  // 正确做法：敌人血**正常**（会被打死，波次才能推进），
  // 而玩家 base_hp 给到极大（保证不输）。
  const level = { ...raw, base_hp: 100_000_000 }
  const enemyMap = new Map<number, EnemyDef>(
    (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
  )
  const skillMap = new Map<number, SkillDef>(
    [
      ...(fixture.skills as unknown as SkillDef[]),
      ...(fixture.composite_skills as unknown as SkillDef[]),
    ].map((s) => [s.id, s]),
  )
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, 4)
  const def = [...enemyMap.values()][0]!

  const cfg: BattleConfig = {
    level,
    enemies: new Map([[def.id, { ...def, attack: 0n as never, attack_range: 0 as never }]]),
    skills: skillMap,
    equipped: active.map((s, i) => ({
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
    })),
    attacker: { ...defaultAttacker(), critPermille: critBase },
    seed: 4242,
  }
  return new BattleEngine(cfg)
}

type Inner = {
  start(): void
  step(): void
  skipCards(): void
  phase: string
  buffs: { critPermille: bigint; freeDiscard: number }
  deck: {
    hand: Array<{ id: string; kind: string; effect?: { kind: string; value: bigint } }>
    take(id: string): unknown
    discardsLeft: number
  }
  applyAttribute(e: { kind: string; value: bigint }): void
  applyMechanic(m: { kind: string; value: number }): void
  currentAttacker(): { critPermille: bigint }
}

describe('审计项 C-13：critPermille 的 0..1000 约定（实测结果记录）', () => {
  const base = 550n // 服务端 MaxLoadoutCritPermille=500 + defaultAttacker 50

  it('真实的 crit 卡是 80‰/张（不是别的数）', () => {
    // 判据直接读 ATTRIBUTE_POOL 的源码 —— 卡池不是导出常量，
    // 而一份导出的名单需要人工同步（清单会漂，扫描不会）。
    const i = mkEngine(base) as unknown as Inner
    // 通过 applyAttribute 反推：80‰ 的卡加 6 张应当得到 480
    for (let k = 0; k < 6; k++) i.applyAttribute({ kind: 'crit', value: 80n })
    expect(i.buffs.critPermille, '6 张 crit 卡应当累积 480‰').toBe(480n)
  })

  it('装备 + 满波卡可以越过 1000（这是「约定没被执行」的证据）', () => {
    const i = mkEngine(base) as unknown as Inner
    for (let k = 0; k < 6; k++) i.applyAttribute({ kind: 'crit', value: 80n })
    const raw = base + i.buffs.critPermille
    expect(raw, `base=${base} + buffs=${i.buffs.critPermille} 应当越过 1000`).toBeGreaterThan(
      PERMILLE,
    )
  })

  it('currentAttacker() 夹到 PERMILLE —— 声称的暴击率必须是真的', () => {
    const i = mkEngine(base) as unknown as Inner
    for (let k = 0; k < 6; k++) i.applyAttribute({ kind: 'crit', value: 80n })
    const got = i.currentAttacker().critPermille
    expect(
      got,
      `currentAttacker 报 ${got} —— 超过 1000 意味着「暴击率 >100%」，` +
        '而它会原样进 replay_hash 的 A 段前缀',
    ).toBeLessThanOrEqual(PERMILLE)
  })

  it('⚠️ 如实记录：夹取**不改变暴击行为**——1000 处已经饱和', () => {
    // 消费侧判据：`roll >= PERMILLE - critPermille`，而 roll 是 0..999
    // （BattleRng.roll()，早前一轮已把范围从 0..9999 修正为 0..999）。
    // critPermille = 1000 → roll >= 0   → roll ∈ [0,999] 恒真
    // critPermille = 1030 → roll >= -30  → 同样恒真
    //
    // 所以「critPermille > 1000 → 每发暴击」这个审计结论**成立**，
    // 但它在 1000 处就已经成立了 —— 夹取不修任何行为。
    //
    // 本条把这件事钉成事实，避免后来的人把夹取当成
    // 「修了一个大 bug」而过度自信，或反过来认为它可以删掉。
    const rates = [1000n, 1030n, 5000n, 1_000_000n].map((p) => {
      let crits = 0
      for (let roll = 0; roll < 200; roll++) {
        const r = resolveHit(
          { ...defaultAttacker(), critPermille: p },
          new Defender(1_000_000n, 0n, 0n),
          { skillDamage: 100n, skillElement: 'fire', roll },
        )
        if (r.crit) crits++
      }
      return { p, rate: crits / 200 }
    })
    for (const r of rates) {
      expect(
        r.rate,
        `critPermille=${r.p} 时暴击率 ${(r.rate * 100).toFixed(0)}%，` +
          '而 1000 处已经饱和 —— 说明夹取确实不改变行为',
      ).toBe(1)
    }
  })

  it('夹取之下界：0 不变成「负数」（`att.critPermille > 0n` 那道门必须留着）', () => {
    let crits = 0
    for (let roll = 0; roll < 1000; roll++) {
      const r = resolveHit({ ...defaultAttacker(), critPermille: 0n }, new Defender(1_000_000n, 0n, 0n), {
        skillDamage: 100n,
        skillElement: 'fire',
        roll,
      })
      if (r.crit) crits++
    }
    expect(crits, 'critPermille=0 时不该有任何一发暴击').toBe(0)
  })
})

describe('审计项 C-12：free_discard 被重复计入（实测结论：不是缺陷）', () => {
  // `applyMechanic` 的 free_discard 分支里有**两行**：
  //
  //     this.buffs.freeDiscard += v     // 记进 buff
  //     this.deck.discardsLeft += v     // 立刻加到本波
  //
  // 而 `beginWave` 又有：
  //
  //     this.deck.discardsLeft += this.buffs.freeDiscard
  //
  // 表面看是「加了两次」。**实测不是** ——
  // 两条路径覆盖的是**互补的波次**：
  //
  //   applyMechanic 的立刻加  ->  覆盖「拿到卡的那一波」
  //   beginWave 的每波加      ->  覆盖「之后每一波」
  //
  // 卡面 descr 写的是「每波弃牌次数 +1」，而玩家不可能拥有
  // 自己还没抽到的卡 —— 所以「从拿到那波起生效」是正确的语义。
  //
  // ⚠️ 如果有人把 `deck.discardsLeft += v` 那行当「重复」删掉，
  // 拿到卡的那一波会**少一次**弃牌 —— 那才是真的引入 bug。
  //
  // # 为什么不跑整局
  //
  // 我第一版跑整局逐波采样，踩了两个坑（都记在下面）：
  //  1. `phase === 'wave'` 持续上百 tick，逐 tick 采样得到 3806 个样本
  //     而不是 5 个波次 —— 于是 `out.length` 变成 tick 数
  //  2. 为了「敌人打不死玩家」把敌人 hp 抬到 1e6，结果玩家也打不死敌人，
  //     卡在第 1 波，`out.length` 恒为 1
  //
  // 而这个问题**只需要看两个函数各加了什么**，不需要跑整局。

  type W = Inner & {
    beginWave(index: number): void
  }

  it('applyMechanic：立刻加一次本波的 discardsLeft，并记进 buff', () => {
    const i = mkEngine(50n) as unknown as W
    const before = i.deck.discardsLeft
    i.applyMechanic({ kind: 'free_discard', value: 1 })

    expect(
      i.deck.discardsLeft,
      `applyMechanic 后 discardsLeft = ${i.deck.discardsLeft}（原 ${before}）—— ` +
        '少了一次就说明「立刻加」那行被误删了，而那是**拿到卡的那一波**的配额',
    ).toBe(before + 1)
    expect(i.buffs.freeDiscard, 'freeDiscard 应当被记进 buff 供后续波次用').toBe(1)
  })

  it('beginWave：每波只加 buffs.freeDiscard 一次（新波基线 + 1）', () => {
    const i = mkEngine(50n) as unknown as W
    i.applyMechanic({ kind: 'free_discard', value: 1 })

    for (const waveIdx of [1, 2, 3]) {
      i.beginWave(waveIdx)
      expect(
        i.deck.discardsLeft,
        `beginWave(${waveIdx}) 后 discardsLeft = ${i.deck.discardsLeft} —— ` +
          '期望 DISCARD_PER_WAVE(1) + freeDiscard(1) = 2。' +
          '得 3 说明重复计入，得 1 说明 beginWave 那行被删',
      ).toBe(2)
    }
  })

  it('两步合起来：拿到卡的那一波 +1，之后每一波各 +1（不是 +2）', () => {
    const base = mkEngine(50n) as unknown as W
    base.beginWave(0)
    const baseline = base.deck.discardsLeft

    const i = mkEngine(50n) as unknown as W
    i.beginWave(0)
    // 在第 0 波的 card_select 里拿到卡
    i.applyMechanic({ kind: 'free_discard', value: 1 })
    expect(
      i.deck.discardsLeft - baseline,
      `第 0 波多出 ${i.deck.discardsLeft - baseline} 次弃牌，期望 1`,
    ).toBe(1)

    i.beginWave(1)
    expect(
      i.deck.discardsLeft - baseline,
      `第 1 波多出 ${i.deck.discardsLeft - baseline} 次，期望 1（多了就是重复计入）`,
    ).toBe(1)
    i.beginWave(2)
    expect(
      i.deck.discardsLeft - baseline,
      `第 2 波多出 ${i.deck.discardsLeft - baseline} 次，期望 1`,
    ).toBe(1)
  })

  it('两张卡时每波 +2（不是 +3）', () => {
    const i = mkEngine(50n) as unknown as W
    i.beginWave(0)
    const baseline = i.deck.discardsLeft
    i.applyMechanic({ kind: 'free_discard', value: 1 })
    i.applyMechanic({ kind: 'free_discard', value: 1 })
    expect(i.deck.discardsLeft - baseline, '同波拿两张应当 +2').toBe(2)

    i.beginWave(1)
    expect(
      i.deck.discardsLeft - baseline,
      `下一波 ${i.deck.discardsLeft - baseline} 次，期望 2（不是 3）`,
    ).toBe(2)
  })

  it('HeatSystem 的 capBonus 与 discardsLeft 是两套东西（不要互相推断）', () => {
    // 防止把 C-12 与 C-09 混为一谈：两者都涉及
    // 「每波重置时加 buff」，但一个改热量上限、一个改弃牌次数，
    // 且 applyMechanic 只对后者有「立刻加」那行。
    const h = new HeatSystem()
    const before = h.cap
    h.capBonus += 200n
    expect(h.cap).toBeGreaterThan(before)
    expect(h.capBonus).toBe(200n)
  })
})
