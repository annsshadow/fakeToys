import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import {
  BattleEngine,
  MAX_BATTLE_TICKS,
  PICK_CODE_BASE,
  PICK_MIN,
  decodeDiscardCode,
  encodeDiscardCode,
  type BattleConfig,
} from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

/**
 * 「先弃牌再取牌」必须能被重放复现（第 66 轮引入，第 159 轮重写）。
 *
 * # 第 66 轮版的核心缺陷（第 159 轮修复）
 *
 * 旧编码 `-(3+handIdx)` / `-(6+handIdx)` 只记「有没有弃过」：
 *
 *   1. **弃的是哪张表达不了** —— 重放侧固定弃 `hand[0]`。
 *      线上 UI 允许弃任意一张：弃第 2/3 张的正常玩家，
 *      重放弃错牌 → 事件流里那张牌不一样 → 哈希失配 → I-6 判伪造。
 *   2. **弃多于一张表达不了** —— 「快速装填」卡（每波弃牌次数 +1）
 *      让 discardsLeft > 1 成为正常玩法，旧编码下无法复现。
 *
 * 新编码把弃牌集合写成**手牌位掩码**（见 `PICK_CODE_BASE` 的枚举表）：
 * 一个 code 同时携带「弃了原手牌的哪几张」+「取剩余第几张 / 跳过」。
 * 弃牌不再写 record 事件 —— 事件流里只留 take/skip 的结果，
 * 于是**弃牌顺序也不进哈希**（旧实现下点击顺序会改变哈希，
 * 而点击顺序本来就不是语义）。
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
  buffs: { freeDiscard: number }
  heat: { heat: bigint }
  cardPicks: number[]
  replay: unknown[]
  takeCard(id: string): unknown
  discardCard(id: string): unknown
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
 * ⚠️ 第 159 轮起「取牌/跳过」都会**立刻**结束本波，所以 onSelect 每波
 * 只会被调用一次；保险起见仍保留「同一波不重复决策」的守卫，
 * 且 onSelect 结束时若还停在 card_select（只弃了牌没取），
 * 由 harness 补一次 skipCards —— 对应真实玩家点「直接下一波」。
 */
function play(eng: BattleEngine, onSelect: (e: Inner) => void): { hash: string; picks: number[] } {
  const i = eng as unknown as Inner
  const isReplay = i.hasReplayScript()
  i.start()
  let decidedWave = -1
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (i.phase === 'won' || i.phase === 'lost') break
    // ⚠️ 重放时**不能**干预选牌：applyReplayDecision 由 step() 内部驱动，
    // 这里再调 onSelect / skipCards 就是双重驱动。
    if (i.phase === 'card_select' && !isReplay && decidedWave !== i.waveIndex) {
      decidedWave = i.waveIndex
      onSelect(i)
      // onSelect 只弃牌不取牌时停在 card_select —— 玩家会点「直接下一波」结束
      if (i.phase === 'card_select') i.skipCards()
    }
    i.step()
  }
  return { hash: eng.replayHash(), picks: [...i.cardPicks] }
}

describe('弃牌可被重放复现（第 66 轮引入，第 159 轮重写）', () => {
  it('「先弃第一张再取一张」的重放哈希必须与原局一致', () => {
    const original = play(mkEngine(), (i) => {
      const hand = i.deck.hand
      if (hand.length > 0) i.discardCard(hand[0]!.id)
      const after = i.deck.hand
      if (after.length > 0) i.takeCard(after[0]!.id)
      else i.skipCards()
    })

    expect(original.picks.length, '原局应记录了选牌').toBeGreaterThan(0)

    const replay = mkEngine()
    ;(replay as unknown as Inner).setReplayScript(original.picks)
    const replayed = play(replay, () => {})

    expect(
      replayed.hash,
      '重放哈希与原局不同 —— I-6 会把这局判成伪造。' +
        '根因候选：弃牌掩码没有被重放侧复现（热量回充/手牌变化）',
    ).toBe(original.hash)
  })

  it('「弃第 3 张再取第 1 张」可复现（旧编码的盲区：重放固定弃 hand[0]）', () => {
    // ⚠️ 这是线上真实玩家最常踩的失配之一：
    // 旧编码只记「有没有弃过」，重放侧永远弃 hand[0]；
    // 玩家弃了第 3 张时，重放弃错牌 → 哈希失配 → 结算被拒。
    const original = play(mkEngine(), (i) => {
      const hand = i.deck.hand
      if (hand.length > 0) i.discardCard(hand[hand.length - 1]!.id)
      const after = i.deck.hand
      if (after.length > 0) i.takeCard(after[0]!.id)
      else i.skipCards()
    })

    expect(original.picks.length).toBeGreaterThan(0)
    // 弃原手牌第 2 张（mask=0b100=4）、取剩余第 0 张 → code = 12 + 0
    // mask=4 的区间起点 = count(0)+count(1)+count(2)+count(3) = 4+3+3+2 = 12
    expect(original.picks[0], '编码应携带「弃了哪张」').toBe(-(PICK_CODE_BASE + 12))

    const replay = mkEngine()
    ;(replay as unknown as Inner).setReplayScript(original.picks)
    const replayed = play(replay, () => {})
    expect(replayed.hash).toBe(original.hash)
  })

  it('「弃一张后跳过整波」可复现（mask + 不取）', () => {
    const original = play(mkEngine(), (i) => {
      if (i.deck.hand.length > 0) i.discardCard(i.deck.hand[0]!.id)
      i.skipCards()
    })
    expect(original.picks.length).toBeGreaterThan(0)
    // 弃原手牌第 0 张（mask=1）、不取：code = count(0) + (count(1)-1) = 4 + 2 = 6
    expect(original.picks[0]).toBe(-(PICK_CODE_BASE + 6))

    const replay = mkEngine()
    ;(replay as unknown as Inner).setReplayScript(original.picks)
    const replayed = play(replay, () => {})
    expect(replayed.hash).toBe(original.hash)
  })

  it('「一次弃两张（快速装填）后取牌」可复现（旧编码表达不了）', () => {
    // 真实玩法：先取到「快速装填」（freeDiscard=1），之后每波可弃 2 张。
    // ⚠️ 弃牌额度来自**战斗状态**（buffs.freeDiscard → beginWave 时进
    // deck.discardsLeft），不来自 card_picks —— 所以两端必须同源。
    // 之前在这里给单侧 `deck.discardsLeft = 2`，重放侧没有额度、
    // 第二次弃牌静默失败 → 哈希失配（这正是本条用例的第一版红因）。
    const withFreeDiscard = (e: BattleEngine): BattleEngine => {
      ;(e as unknown as Inner).buffs.freeDiscard = 1
      return e
    }
    const original = play(withFreeDiscard(mkEngine()), (i) => {
      const hand = i.deck.hand
      if (hand.length > 0) i.discardCard(hand[0]!.id)
      const after1 = i.deck.hand
      if (after1.length > 0) i.discardCard(after1[0]!.id)
      const after2 = i.deck.hand
      if (after2.length > 0) i.takeCard(after2[0]!.id)
      else i.skipCards()
    })
    expect(original.picks.length).toBeGreaterThan(0)
    // 弃原手牌 0、1（mask=0b011=3）、取剩余第 0 张：
    // mask=3 的区间起点 = count(0)+count(1)+count(2) = 4+3+3 = 10 → code = 10
    expect(original.picks[0]).toBe(-(PICK_CODE_BASE + 10))

    const replay = withFreeDiscard(mkEngine())
    ;(replay as unknown as Inner).setReplayScript(original.picks)
    const replayed = play(replay, () => {})
    expect(replayed.hash).toBe(original.hash)
  })

  it('弃牌顺序不进哈希（先弃 A 再弃 B = 先弃 B 再弃 A）', () => {
    // 第 159 轮起弃牌不写 record 事件，掩码编码存的是**集合**：
    // 两种点击顺序的最终状态（热量、手牌）相同，哈希必须相同。
    // 旧实现下点击顺序写进事件流 → 哈希不同 → 换一种点击顺序的玩家
    // 永远无法被重放。
    const playTwoDiscards = (first: number, second: number): string => {
      return play(mkEngine(), (i) => {
        i.deck.discardsLeft = 2
        i.discardCard(i.deck.hand[first]!.id)
        // 第一次移除后下标会移位：按**剩余手牌重新定位**第二张
        const remaining = i.deck.hand
        i.discardCard(remaining[second]!.id)
        const left = i.deck.hand
        if (left.length > 0) i.takeCard(left[0]!.id)
        else i.skipCards()
      }).hash
    }
    // 顺序 A：先弃 0 号、再弃（现在的）0 号 → 集合 {0,1}
    // 顺序 B：先弃 1 号、再弃（现在的）0 号 → 集合 {1,0}
    expect(playTwoDiscards(0, 0)).toBe(playTwoDiscards(1, 0))
  })

  it('弃牌确实改变了热量（否则上面几条守卫可能是空转）', () => {
    const eng = mkEngine()
    const i = eng as unknown as Inner
    i.start()
    let guard = 0
    while (i.phase !== 'card_select' && guard++ < 4000) i.step()
    expect(i.phase, '未能进入选牌阶段').toBe('card_select')

    // 弃牌次数必须 > 0，否则 deck.discard 直接失败、什么都不发生。
    expect(i.deck.discardsLeft, '弃牌次数已耗尽，本用例测不到任何东西').toBeGreaterThan(0)

    i.heat.heat = 50n
    const before = i.heat.heat
    const discarded = i.discardCard(i.deck.hand[0]!.id)
    expect(discarded, 'discardCard 返回 null —— 弃牌未生效').not.toBeNull()
    expect(
      i.heat.heat,
      `弃牌没有改变热量（before=${before}）—— 说明本文件测的「弃牌副作用」已不存在，需换触发机制`,
    ).not.toBe(before)
  })

  it('弃牌不再写回放事件流（第 159 轮反转：顺序不进哈希的前提）', () => {
    const eng = mkEngine()
    const i = eng as unknown as Inner
    i.start()
    let guard = 0
    while (i.phase !== 'card_select' && guard++ < 4000) i.step()

    const before = i.replay.length
    i.discardCard(i.deck.hand[0]!.id)
    expect(
      i.replay.length,
      '弃牌竟然写了回放事件 —— 「弃哪张/顺序」会重新进哈希，' +
        '而重放侧只存集合、无法复现点击顺序',
    ).toBe(before)
  })
})

describe('弃牌掩码编码（第 159 轮）', () => {
  it('encode/decode 往返一致，且覆盖全部 20 个 code', () => {
    const seen = new Set<number>()
    for (let mask = 0; mask < 8; mask++) {
      const remaining = 3 - ((mask & 1) + ((mask >> 1) & 1) + ((mask >> 2) & 1))
      // 取剩余每一张 + 不取（-1）
      for (let take = 0; take < remaining; take++) {
        const code = encodeDiscardCode(mask, take)
        expect(decodeDiscardCode(code), `mask=${mask} take=${take}`).toEqual({ mask, take })
        seen.add(code)
      }
      const skipCode = encodeDiscardCode(mask, -1)
      expect(decodeDiscardCode(skipCode), `mask=${mask} take=-1`).toEqual({ mask, take: -1 })
      seen.add(skipCode)
    }
    // 枚举表共 4+3+3+2+3+2+2+1 = 20 个 code
    expect(seen.size).toBe(20)
    expect(Math.max(...seen)).toBe(19)
  })

  it('code 越界返回 null（非法值不产生动作）', () => {
    expect(decodeDiscardCode(-1)).toBeNull()
    expect(decodeDiscardCode(20)).toBeNull()
    expect(decodeDiscardCode(1.5)).toBeNull()
  })

  it('所有可产出的编码都不低于 PICK_MIN（否则服务端会拒掉客户端自己的产物）', () => {
    expect(PICK_MIN).toBe(-(PICK_CODE_BASE + 19))
    expect(-(PICK_CODE_BASE + 19)).toBeGreaterThanOrEqual(PICK_MIN)
  })
})
