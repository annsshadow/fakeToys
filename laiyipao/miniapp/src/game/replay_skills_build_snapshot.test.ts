import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, replaySkillsSegmentOf, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

/**
 * 局内取牌后，上报的 replay_skills 必须仍然是**开战前**的构筑（第 64 轮）。
 *
 * # 缺陷：把「含局内状态」的串当成给服务端的构筑凭证
 *
 * `replaySkillsSegment()` 读的是 `this.skills`。`this.skills` 在构造时是
 * `cfg.equipped` 的拷贝，但 `applyCard` 的技能卡分支会**就地升格**：
 *
 *     baseDamage:  s.baseDamage + s.baseDamage / 5n
 *     applyStacks: s.applyStacks + 1n
 *
 * 而 settle 时上报的是这个已升格的列表（engine.ts 的 settleInput）。
 *
 * 服务端把上报值与按 **DB 重算**的值逐字比对（replay_skills 校验，
 * 见 server/internal/service/game.go 的 replaySkillsSegment）：
 * 重算走 SQL 读 `user_skill_slots` + `skills.base_damage/apply_stacks/heat_cost`，
 * 那是**开战前**的数值，不含任何局内升格。
 *
 * 于是：取过一张技能卡的正常对局 → S 段不匹配 → `ErrReplaySkillsMismatch`
 * → 422 → **战报不落库、体力与掉落拿不回来**。
 *
 * 触发面：`rollWaveCards` 固定返回 `[skill, attribute, mechanic]`，
 * 技能卡恒在手牌下标 0；5 波各取 1 张，随机选时命中技能卡概率
 * ≈ 1-(2/3)^5 ≈ **86%**。也就是说绝大多数真实玩家的正常对局会被判作弊。
 *
 * # 为什么原有测试全都绿
 *
 *   - `replay_skills_contract.test.ts` 读 formula_vectors.json 的**字面**期望值，
 *     不跑引擎
 *   - 服务端 `replay_skills_e2e_test.go` 的期望值直接查 DB 原始行，也不跑引擎
 *   - `cards.test.ts` 用 `give()` 直接 `takeCard(id)`，绕开了真实牌池
 *
 * 三者都只验「两端副本一致」，没有任何东西验「跑完整局取牌后两端仍一致」。
 *
 * # 为什么 `replayHash()` 里用同一个函数**不是** bug
 *
 * engine.ts:525 的 `const skills = this.replaySkillsSegment()` 是在算
 * 回放哈希前缀 —— 哈希**必须**覆盖局内状态，否则取牌不改变哈希，
 * 「同种子不同操作得同哈希」就成立，验真会说谎。
 *
 * 错的是把同一个「含局内状态」的串**同时**当成上报给服务端的构筑凭证。
 * 两个用途需要两个不同的取值口径。
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

function mkEngine(equippedCount = 4): BattleEngine {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, equippedCount)
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
    seed: 1,
  }
  return new BattleEngine(cfg)
}

/** 经公开的 takeCard 路径施加一张技能卡（与真实玩家操作同一条路）。 */
function giveSkillCard(e: BattleEngine, skillId: number): void {
  const eng = e as unknown as {
    phase: string
    deck: { hand: unknown[] }
    takeCard(id: string): unknown
  }
  eng.phase = 'card_select'
  eng.deck.hand = [{ id: 'c1', kind: 'skill', name: '技能卡', descr: '', skillId }]
  eng.takeCard('c1')
}

function buildTimeSegment(e: BattleEngine): string {
  return (e as unknown as { replaySkillsSegment(): string }).replaySkillsSegment()
}

/** 真正上报的那个口径（settleInput 用的）。 */
function reportedSegment(e: BattleEngine): string {
  return (e as unknown as { buildSnapshotSegment(): string }).buildSnapshotSegment()
}

/** replayHash() 的真实输出。 */
function replayHashOf(e: BattleEngine): string {
  return (e as unknown as { replayHash(): string }).replayHash()
}

describe('上报的 replay_skills 必须是开战前的构筑（第 64 轮）', () => {
  /** 只对 4 张已装备技能各取一张技能卡 —— rollWaveCards 唯一可能产出的形态。 */
  function takeAllSkillCards(e: BattleEngine): void {
    const ids = [...skillMap.values()]
      .filter((s) => s.kind === 'active')
      .sort((a, b) => a.id - b.id)
      .slice(0, 4)
      .map((s) => s.id)
    for (const id of ids) giveSkillCard(e, id)
  }

  it('settleInput 上报的 replay_skills 取牌后必须逐字不变', () => {
    const eng = mkEngine()
    // 真正的上报体 —— 不是内部方法，是发给服务端的那个对象。
    const before = (eng as unknown as { settleInput(): { replay_skills: string } }).settleInput()
      .replay_skills
    expect(before).not.toBe('')

    takeAllSkillCards(eng)

    const after = (eng as unknown as { settleInput(): { replay_skills: string } }).settleInput()
      .replay_skills
    expect(
      after,
      '取技能卡改变了上报的 replay_skills —— 服务端按 DB 重算的是开战前构筑，' +
        '于是这局会被 422 判成作弊，战报不落库、掉落拿不回来',
    ).toBe(before)
  })

  it('buildSnapshotSegment 与 settleInput 上报值一致（两处不能各算一份）', () => {
    const eng = mkEngine()
    takeAllSkillCards(eng)
    const viaSettle = (eng as unknown as { settleInput(): { replay_skills: string } }).settleInput()
      .replay_skills
    expect(reportedSegment(eng)).toBe(viaSettle)
  })

  // ⚠️ 这里**没有**「replayHash 的 S 段该用哪个口径」的守卫，是刻意的。
//
// 本轮试过写，变异测试证明它测不出意图：
//   把 replayHash() 里的 `this.replaySkillsSegment()` 换成 buildSnapshotSkills，
//   第一版守卫（比较 replaySkillsSegment() 前后）**全绿** —— 那是消费者，不是连接；
//   改成比较 replayHash() 输出后**仍然全绿** —— 因为取牌同时往 this.replay 写了
//   8 个 card_taken 事件，哈希照样会变，两种 S 段口径的差异被事件流掩盖了。
//
// 要真正隔离这个变量，需要「同一事件流、仅 S 段取值不同」的两个引擎实例，
// 而 applyCard 会同时改两者 —— 构造不出来，除非给引擎加一个只为测试存在的
// 「以指定 S 段算哈希」的入口。那是**为测试改产品码**，本项目不做
// （参见 README「测试空洞的四种形态」：判据是「改坏会不会红」，
//  而一条需要改产品码才能成立的守卫，它的有效性反而更难保证）。
//
// 现状是安全的：replayHash 用局内态是**当前**行为，
// 且 replay.test.ts / determinism.test.ts 的 38 条守卫全绿。
// 本文件只守上报口径 —— 那才是本轮修的东西。

  it('replayHash 随取牌而变（守住「同种子换操作」不会被漏报）', () => {
    // 这条**测不出** S 段取哪个口径（见文件末尾的说明），
    // 但它能守住一条更要紧的不变式：取牌必须改变哈希。
    //
    // 少了它，某天有人把 replayHash 的事件流或前缀改成与选牌无关，
    // 「同种子 + 换一套操作」就会得到同一个哈希，I-6 直接失去区分能力。
    const eng = mkEngine()
    const before = replayHashOf(eng)
    expect(before).toMatch(/^[0-9a-f]{16}$/)
    takeAllSkillCards(eng)
    expect(replayHashOf(eng), '取牌竟然没改变 replayHash').not.toBe(before)
  })

it('两个口径确实不同：局内态含升格，上报口径不含', () => {
    // 钉住「两个口径确实不同」这个前提：
    // 若哪天 applyCard 不再升格，这条会红，提醒本文件换一种触发机制。
    const eng = mkEngine()
    takeAllSkillCards(eng)

    expect(
      buildTimeSegment(eng),
      '局内态与上报口径相同 —— 说明引擎不再升格，本文件需换触发机制',
    ).not.toBe(reportedSegment(eng))
  })

  it('冻结时机早于任何 applyCard（构造那一刻就是开战前）', () => {
    // 直接验「冻结」这件事本身，而不是间接看结果：
    // 构造后、任何取牌之前，冻结值必须已经等于局内态。
    const eng = mkEngine()
    expect(reportedSegment(eng)).toBe(buildTimeSegment(eng))

    // 再确认它**不会**随后续取牌而变（这才是冻结的含义）
    const frozen = reportedSegment(eng)
    takeAllSkillCards(eng)
    expect(reportedSegment(eng)).toBe(frozen)
  })

  it('局内升格确实发生过（否则上面两条守卫可能空转）', () => {
    const eng = mkEngine()
    const before = buildTimeSegment(eng)
    const first = [...skillMap.values()].filter((s) => s.kind === 'active').sort((a, b) => a.id - b.id)[0]!
    giveSkillCard(eng, first.id)
    expect(buildTimeSegment(eng), '局内没有任何升格 —— 说明引擎没走到 applyCard 的升格分支').not.toBe(before)
  })

  it('replaySkillsSegmentOf 对同一份输入是纯函数（口径必须可复现）', () => {
    const skills = [
      { slot: 0, skillId: 1, baseDamage: 100n, applyStacks: 1n, heatCost: 20n },
      { slot: 10, skillId: 2, baseDamage: 50n, applyStacks: 2n, heatCost: 15n },
    ]
    const a = replaySkillsSegmentOf(skills)
    const b = replaySkillsSegmentOf([...skills].reverse())
    expect(a).toBe(b)
    // slot=10 排在 slot=2 前面（字符串序，不是数值序）—— 与 Go sort.Strings 等价
    expect(a).toBe('0:1:100:1:20,10:2:50:2:15')
  })
})