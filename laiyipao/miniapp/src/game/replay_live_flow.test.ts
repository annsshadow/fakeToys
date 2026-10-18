import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, TICK_MS, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

/**
 * 真实玩法（带思考时间、全决策模式）必须可被重放复现 —— 第 159 轮 P0 的回归闸。
 *
 * # 事故（10-16 enforce 上线后 100% 拒发）
 *
 * 线上两笔真实结算全被拒（`mismatched=2/rejected=2`），根因是两条
 * 与「合成测试」擦肩而过的路径：
 *
 *  1. **思考时间进哈希**：`step()` 无条件 `tick++ / elapsedMs += TICK_MS`，
 *     包括玩家盯着三选一界面犹豫的每一格。事件 tick 是 replayHash 的
 *     输入 —— 于是「真人犹豫 2 秒」与「零等待重放」的时间线必然错开，
 *     哪怕只犹豫 50ms（=1 格）。
 *     旧测试全部在**同一 tick 内**连点完成选牌，恰好绕开了这个差异。
 *
 *  2. **一波连点多张**：旧引擎「取 1 张后仍停在 card_select」，UI 里
 *     连点两张都生效；而 card_picks 每波只记一项 —— 第二张在重放里
 *     必然对不上。旧测试偏偏只用「取 1 张」或「取空 3 张」，前者
 *     因为记录项恰好是取的牌而偶然对上，后者因为记满而侥幸过关。
 *
 * 本文件把这两条路径变成**显式断言**：
 *  - 等待期间 `tick / elapsedMs` 必须纹丝不动（时钟冻结）；
 *  - 四种真实决策模式（纯取 / 弃+取 / 弃末张+取 / 弃+跳）带随机思考
 *    时间跑完整局，重放哈希与原局必须一致；
 *  - 取牌后第二张必须点不动（phase 已离开 card_select）。
 */

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

function mkEngine(seed = 12345): BattleEngine {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  const cfg: BattleConfig = {
    level,
    enemies: enemyMap,
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
    attacker: defaultAttacker(),
    seed,
  }
  return new BattleEngine(cfg)
}

type Inner = {
  phase: string
  waveIndex: number
  tick: number
  elapsedMs: number
  deck: { hand: { id: string }[]; discardsLeft: number }
  cardPicks: number[]
  replay: { t: number; type: string; a: number; b: number; c: number }[]
  takeCard(id: string): unknown
  discardCard(id: string): unknown
  skipCards(): void
  setReplayScript(picks: number[] | null): void
  hasReplayScript(): boolean
  start(): void
  step(): unknown
  replayHash(): string
}

/**
 * 模拟真人一整局：每波先「思考」wait 格（这些 step 必须什么都不推进），
 * 再做一次完整决策（取牌/跳过都会结束本波）。
 *
 * 重放侧（isReplay）不得干预：applyReplayDecision 由 step() 内部驱动。
 */
function playWithThink(
  eng: BattleEngine,
  thinkTicks: (wave: number) => number,
  onSelect: (i: Inner, wave: number) => void,
): { hash: string; picks: number[]; elapsedMs: number } {
  const i = eng as unknown as Inner
  const isReplay = i.hasReplayScript()
  i.start()
  let decidedWave = -1
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (i.phase === 'won' || i.phase === 'lost') break
    if (i.phase === 'card_select' && !isReplay && decidedWave !== i.waveIndex) {
      decidedWave = i.waveIndex
      const wait = thinkTicks(i.waveIndex)
      for (let w = 0; w < wait; w++) i.step()
      onSelect(i, i.waveIndex)
      // 只弃了牌没取/跳时，对应真实玩家点「直接下一波」
      if (i.phase === 'card_select') i.skipCards()
    }
    i.step()
  }
  return { hash: eng.replayHash(), picks: [...i.cardPicks], elapsedMs: i.elapsedMs }
}

describe('真实玩法重放闭环（第 159 轮 P0 回归）', () => {
  it('选牌等待期间战斗时钟冻结，决策事件落在同一 tick', () => {
    const eng = mkEngine()
    const i = eng as unknown as Inner
    i.start()
    while (i.phase !== 'card_select') i.step()

    const t0 = i.tick
    const e0 = i.elapsedMs
    const n0 = i.replay.length
    // 「玩家盯着界面犹豫 500 格」（10 秒）—— 旧实现在这里 tick+500
    for (let k = 0; k < 500; k++) i.step()
    expect(i.phase, '等待期间跳出了选牌阶段').toBe('card_select')
    expect(i.tick, '思考时间推进了 tick —— 真人犹豫会把哈希冲散，线上 100% 拒发就是这么来的').toBe(t0)
    expect(i.elapsedMs, '思考时间进了 elapsedMs —— 结算时长会被虚报').toBe(e0)

    // 决策：取一张 → 立刻开波；取牌事件与开波事件都必须落在冻结的 t0
    i.takeCard(i.deck.hand[0]!.id)
    expect(i.phase).toBe('wave')
    const added = i.replay.slice(n0)
    expect(added.length, `取牌应恰好产生 2 条记录（card + wave），实际 ${JSON.stringify(added)}`).toBe(2)
    expect(added[0]!.t).toBe(t0)
    expect(added[1]!.t).toBe(t0)
    expect(added[0]!.type).toBe('card')
    expect(added[1]!.type).toBe('wave')
  })

  it('取牌后第二张点不动（一波恰好一项 card_picks）', () => {
    const eng = mkEngine()
    const i = eng as unknown as Inner
    i.start()
    while (i.phase !== 'card_select') i.step()

    const first = i.deck.hand[0]!.id
    const second = i.deck.hand[1]!.id
    expect(i.takeCard(first)).not.toBeNull()
    // 旧实现在这里返回第二张卡（phase 仍是 card_select）——
    // 而 card_picks 只记一项，重放必然对不上
    expect(i.takeCard(second), '取牌后 phase 已离开 card_select，第二张必须点不动').toBeNull()
    expect(i.cardPicks.length).toBe(1)
  })

  it('带思考时间的完整对局（四种决策模式轮转）可重放且时长一致', () => {
    const think = (wave: number) => [0, 37, 3, 100, 1][wave % 5]!

    const decide = (i: Inner, wave: number) => {
      const hand = () => i.deck.hand
      switch (wave % 4) {
        case 0: // 不弃直接取中间那张
          i.takeCard(hand()[1 % hand().length]!.id)
          break
        case 1: // 弃首张再取剩余首张
          i.discardCard(hand()[0]!.id)
          if (hand().length > 0) i.takeCard(hand()[0]!.id)
          else i.skipCards()
          break
        case 2: // 弃末张再取剩余首张
          i.discardCard(hand()[hand().length - 1]!.id)
          if (hand().length > 0) i.takeCard(hand()[0]!.id)
          else i.skipCards()
          break
        default: // 弃首张后跳过整波
          i.discardCard(hand()[0]!.id)
          i.skipCards()
          break
      }
    }

    const original = playWithThink(mkEngine(), think, decide)
    expect(
      original.picks.length,
      '原局应记录了每波一次决策（关卡 1 共 5 波 → 4 次）',
    ).toBe(4)
    // 决策模式轮转覆盖了三种负编码 + 普通取牌，防止「恰好全是 0/1/2」
    // 让本用例变成空转
    expect(original.picks.some((p) => p < 0), '四种模式里应有负编码（弃牌）').toBe(true)
    expect(original.picks.some((p) => p >= 0), '四种模式里应有普通取牌').toBe(true)

    const replay = mkEngine()
    ;(replay as unknown as Inner).setReplayScript(original.picks)
    const replayed = playWithThink(replay, () => 0, () => {})

    expect(
      replayed.picks,
      '重放重建的 card_picks 必须与原局逐项一致 —— 不一致说明 applyReplayDecision 与 recordPick 语义漂了',
    ).toEqual(original.picks)
    expect(
      replayed.hash,
      '带思考时间的真实玩法重放哈希与原局不同 —— I-6 会拒发。' +
        '根因候选：时钟冻结失效 / 弃牌掩码未复现 / 一波消费了多项决策',
    ).toBe(original.hash)
    expect(replayed.elapsedMs, '时长必须按战斗时间一致（思考时间不进 elapsedMs）').toBe(
      original.elapsedMs,
    )
  })

  it('结算时长按 tick 推导，且不含思考时间（10 秒犹豫不虚报）', () => {
    const original = playWithThink(
      mkEngine(),
      () => 200, // 每波犹豫 200 格 = 10 秒
      (i) => i.takeCard(i.deck.hand[0]!.id),
    )
    const noWait = playWithThink(
      mkEngine(),
      () => 0,
      (i) => i.takeCard(i.deck.hand[0]!.id),
    )
    expect(original.elapsedMs, '犹豫 10 秒 × 4 波不该进结算时长').toBe(noWait.elapsedMs)
    expect(original.hash).toBe(noWait.hash)
    // 时长与 tick 的换算关系锁定（服务端按同一口径推 duration_ms）
    expect(original.elapsedMs % TICK_MS).toBe(0)
  })
})
