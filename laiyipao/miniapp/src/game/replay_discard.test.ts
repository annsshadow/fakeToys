import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, PICK_DISCARD_SKIP_BASE, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

/**
 * 「先弃牌再取牌」必须能被重放复现（第 66 轮）。
 *
 * # 缺陷：card_picks 只记取牌，弃牌的副作用无人复现
 *
 * `card_picks` 每波只记**一个**整数（`recordPick` 只在
 * `length <= waveIndex` 时 push，即每波第一次有效选择）。
 *
 * 原局里玩家若**先弃牌再取牌**，弃牌会做三件事：
 *   1. `this.deck.discard(id)` —— 扣 `discardsLeft`、把手牌移除
 *   2. `this.heat.refundHeat(r.refund)` —— **回充热量**
 *   3. `this.record(this.tick, 'card', cardIndex(card), 1)` —— **写入回放事件流**
 *
 * 重放侧 `applyReplayDecision` 只按脚本取牌：
 *   - 不复现那次 `record(..., 1)` → 事件流少一条 → **哈希必然不同**
 *   - 不复现那次 `refundHeat` → 热量差 1 → 后续开火序列整体偏移
 *
 * 后果：该局的 replayHash 与记录不符 → I-6 判**伪造**。
 * 而 `discardCard` 与 `takeCard` 一样只判 `phase === 'card_select'`，
 * 「先弃后取」是 UI 与 API 都允许的合法操作。
 *
 * # 为什么原有测试全绿
 *
 * `replay.test.ts` 与 `i6.e2e.test.ts` 的 card_picks 往返用例都是
 * 「取牌」或「跳过」两种形态 —— **没有一条覆盖「弃牌 + 取牌」**。
 *
 * # 判据
 *
 * 不是「discardCard 被调用过」，而是**同一份 card_picks 重放出的
 * replayHash 必须与原局相同**。只有这个能回答「能不能复现」。
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
  deck: { hand: { id: string; kind: string }[]; discardsLeft: number }
  heat: { heat: bigint }
  cardPicks: number[]
  replay: unknown[]
  takeCard(id: string): unknown
  discardCard(id: string): unknown
  finishCardSelect(): void
  setReplayScript(picks: number[]): void
  hasReplayScript(): boolean
  replayHash(): string
  skipCards(): void
  start(): void
  step(): void
}

/**
 * 跑完整局，onSelect 决定每波怎么选牌。
 *
 * ⚠️⚠️ `onSelect` 必须**每波只调用一次**，不能每 tick 都调。
 *
 * 我第一版写成「每 tick 检查 phase==='card_select' 就决策一次」，
 * 于是引擎在 takeCard 后手牌未空时不进下一波，下一 tick 又进选牌阶段
 * → 每波取牌 2~3 次。
 *
 * 那个模式**真实玩家做不到**（UI 上「取 1 张」就点下一步），
 * 而且它超出了 `card_picks` 的协议能力：服务端强制
 * `len(card_picks) <= 波数`，即「每波一项」。
 *
 * 实测 symptom：原局 card 事件是
 *   401/30/1  401/506/0  **402/902/0**
 * 而重放只能复现前两条 —— 第 3 条是同波的第 2 次取牌，
 * 协议里没有任何位置能记它。
 *
 * 这个「每波一项」的假设**是既有的、有服务端校验强制的**，
 * 本文件守的就是这个语义下的可复现性。
 */
function play(eng: BattleEngine, onSelect: (e: Inner) => void): { hash: string; picks: number[] } {
  const i = eng as unknown as Inner
  const isReplay = i.hasReplayScript()
  i.start()
  let decidedThisWave = -1
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (i.phase === 'won' || i.phase === 'lost') break
    // ⚠️ 重放时**不能**干预选牌：applyReplayDecision 由 step() 内部驱动，
    // 这里再调 onSelect / finishCardSelect 就是双重驱动。
    if (i.phase === 'card_select' && !isReplay) {
      if (decidedThisWave !== i.waveIndex) {
        decidedThisWave = i.waveIndex
        onSelect(i)
      } else {
        // 玩家离开选牌界面：丢弃剩余手牌，进入下一波。
        i.finishCardSelect()
      }
    }
    i.step()
  }
  return { hash: eng.replayHash(), picks: [...i.cardPicks] }
}

describe('弃牌可被重放复现（第 66 轮）', () => {
  it('「先弃 1 张再取 1 张」的重放哈希必须与原局一致', () => {
    // 原局：每波先弃第一张，再取第二张
    const original = play(mkEngine(), (i) => {
      const hand = i.deck.hand
      if (hand.length > 0) i.discardCard(hand[0]!.id)
      const after = i.deck.hand
      if (after.length > 0) i.takeCard(after[0]!.id)
      else i.skipCards()
    })

    expect(original.picks.length, '原局应记录了选牌').toBeGreaterThan(0)

    // 重放：只喂 card_picks，不告诉它「原来弃过牌」
    const replay = mkEngine()
    ;(replay as unknown as Inner).setReplayScript(original.picks)
    const replayed = play(replay, () => {
      /* 重放侧由 applyReplayDecision 驱动，这里不干预 */
    })

    expect(
      replayed.hash,
      '重放哈希与原局不同 —— I-6 会把这局判成伪造。' +
        '根因：card_picks 每波只记一个整数，弃牌的 record 事件与热量回充都没被复现',
    ).toBe(original.hash)
  })

  it('「弃一张后跳过整波」的可复现性（弃牌 + skipCards 组合）', () => {
    // 弃掉第一张，然后把剩下的整波跳过 —— 这会走 `skipCards` → `recordPick(-1)`。
    // 弃牌的 record 事件与 refundHeat 仍然要被复现，所以这是最容易失配的组合。
    const original = play(mkEngine(), (i) => {
      if (i.deck.hand.length > 0) i.discardCard(i.deck.hand[0]!.id)
      i.skipCards()
    })

    const replay = mkEngine()
    ;(replay as unknown as Inner).setReplayScript(original.picks)
    const replayed = play(replay, () => {})

    expect(replayed.hash).toBe(original.hash)
  })

  it('「弃+跳过」的编码会被真实复现（discard 的热量回充必须发生）', () => {
    // ⚠️ 变异测试的一条**如实记录**：把
    //   `if (want <= -PICK_DISCARD_SKIP_BASE)` 改成 `if (false)`
    // 之后，12 条守卫**全部通过**。
    //
    // 我追查了原因，结论是**这个变异无害**，不是守卫漏了：
    //   -6 落入「弃+取」分支，`rest = -(want + 3) = 3`
    //   而每波只有 3 张牌（索引 0..2），`hand[3]` 不存在
    //   → 走 `this.skipCards(); continue`
    //
    // 两条分支的实际动作都是「discardCard(hand[0]) 然后 skipCards()」，
    // **完全等价**。所以任何行为断言都抓不到它 ——
    // 能抓到的只有「源码里有没有这个分支」这种文本扫描，
    // 而本项目的判据是「改坏会不会红」，文本扫描恰好是那种
    // 「看起来在守着某样东西、实际什么也没守」的假守卫。
    //
    // 这与 README 记的「去掉雪崩只留 levelID × 奇数 → 均匀度测试全绿」
    // 是同一类：变异确实无害，如实记录，不粉饰。
    //
    // 那么这两个分支该不该合并？**不合并** ——
    // 它们的区别会在 `rollWaveCards` 改成每波 4 张时显现：
    // 那时 `rest = 3` 落在合法区间内，「弃+跳」的编码就会被
    // 当成「弃+取第 3 张」。合并等于把这个坑埋到那一天才炸。
    // `card_picks_contract.test.ts` 的「区间不相交」守卫盯着这件事。
    //
    // 这条用例守的是**行为契约本身**（discard 的 refundHeat 真的发生），
    // 那是无论分支怎么合并都必须成立的东西。
    const eng = mkEngine()
    const i = eng as unknown as Inner
    i.setReplayScript([-PICK_DISCARD_SKIP_BASE])
    i.start()
    let guard = 0
    while (i.phase !== 'card_select' && guard++ < 4000) i.step()
    expect(i.phase).toBe('card_select')

    const before = i.heat.heat
    i.step() // applyReplayDecision 在 step 开头跑
    expect(
      i.heat.heat,
      `重放「弃+跳」没有回充热量（before=${before}）—— ` +
        '说明 discard 没被执行，退化成纯跳过，热量与原局差 1',
    ).not.toBe(before)
  })

it('弃过牌再跳过用独立编码（-6..-8），不与「弃+取」（-3..-5）重叠', () => {
    // 上一条的判据是哈希；这条钉住**编码取值域本身**。
    //
    // 我第一版写的是「弃过牌再跳过记 -1」，那是错的：
    // -1 无法表达「弃过牌」，重放侧不会复现那次 discard 的 record 事件
    // 与热量变化 → 哈希失配。实测那条用例红，且与「弃+取」的编码域重叠。
    const original = play(mkEngine(), (i) => {
      if (i.deck.hand.length > 0) i.discardCard(i.deck.hand[0]!.id)
      i.skipCards()
    })
    expect(original.picks.length).toBeGreaterThan(0)
    for (const p of original.picks) {
      expect(
        p,
        `picks=${JSON.stringify(original.picks)} —— 「弃+跳」应落在 [-8,-6]，` +
          '且不得与「弃+取」的 [-5,-3] 重叠（BASE 相差 3 > 每波最大手牌数 2）',
      ).toBeLessThanOrEqual(-6)
      expect(p).toBeGreaterThanOrEqual(-8)
    }
  })

  it('弃牌确实改变了热量（否则上面两条守卫可能是空转）', () => {
    // 守卫的判据是哈希；这条确认「弃牌」不是无副作用的空操作。
    // 若 discardCard 哪天不再动热量，上面的哈希守卫会失去意义，
    // 这条会提醒换一种触发机制。
    //
    // ⚠️ 断言用「不相等」而不是「变大」：`refundHeat` 的实现是
    // `heat = heat - n < 0 ? 0 : heat - n` —— 它**降低**热量
    // （函数名叫 refund 但语义是减），且热量为 0 时减了也不变。
    // 断言「变大」会把这个语义写反，让守卫变成假绿。
    const eng = mkEngine()
    const i = eng as unknown as Inner
    i.start()
    let guard = 0
    while (i.phase !== 'card_select' && guard++ < 4000) i.step()
    expect(i.phase, '未能进入选牌阶段').toBe('card_select')

    // 弃牌次数必须 > 0，否则 deck.discard 直接失败、什么都不发生。
    // ⚠️ 先断言这个再断言热量 —— 否则「热量没变」可能只是因为
    // 弃牌根本没执行，而我把它读成「弃牌不再有副作用」。
    expect(i.deck.discardsLeft, '弃牌次数已耗尽，本用例测不到任何东西').toBeGreaterThan(0)

    // 手动加一点热量，保证「减 1」可观测
    i.heat.heat = 50n
    const before = i.heat.heat
    const discarded = i.discardCard(i.deck.hand[0]!.id)
    expect(discarded, 'discardCard 返回 null —— 弃牌未生效').not.toBeNull()
    expect(
      i.heat.heat,
      `弃牌没有改变热量（before=${before}）—— 说明本文件测的「弃牌副作用」已不存在，需换触发机制`,
    ).not.toBe(before)
  })

  it('弃牌会写入回放事件流（这是哈希失配的直接来源）', () => {
    const eng = mkEngine()
    const i = eng as unknown as Inner
    i.start()
    let guard = 0
    while (i.phase !== 'card_select' && guard++ < 4000) i.step()

    const before = i.replay.length
    i.discardCard(i.deck.hand[0]!.id)
    expect(i.replay.length, '弃牌没有写回放事件 —— 与上一条互为印证').toBeGreaterThan(before)
  })
})