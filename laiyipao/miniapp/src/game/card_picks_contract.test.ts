import { describe, it, expect } from 'vitest'
import vectors from '@vectors/formula_vectors.json'
import {
  PICK_DISCARD_BASE,
  PICK_DISCARD_SKIP_BASE,
  PICK_MIN,
} from './engine'

/**
 * card_picks 编码的**跨端契约**（第 66 轮）。
 *
 * 抄的是 `server/internal/domain/card_picks_contract_test.go` 的分工：
 * 那个文件盯 Go 侧，本文件盯 TS 侧，两端读**同一个** JSON。
 *
 * # 为什么非要跨端
 *
 * `card_picks` 的负值编码在两端各有一份边界：
 *
 *   Go：CardPickMin（internal/domain/battle_collections.go）—— 按它拒
 *   TS：PICK_MIN（src/game/engine.ts）—— 按它生成
 *
 * 漂了的后果是**最坏的那种**：客户端正常对局被服务端 422 拒掉，
 * 而客户端单测、Go 单测、服务端 e2e **全部照样绿** ——
 * 因为它们都不跑一次真实结算。
 *
 * 只有「两端对着同一份字面值」才抓得到。
 */

const cp = vectors.card_picks

describe('card_picks 跨端契约', () => {
  it('PICK_MIN 与契约向量一致', () => {
    expect(cp.cases.length, '向量被误删时必须红，否则这个测试会静默变成空转').toBeGreaterThan(0)
    expect(PICK_MIN, 'TS 侧 PICK_MIN 与契约向量不一致 —— 两端漂了的后果是客户端全绿但真机玩家被 422 拒').toBe(
      cp.min,
    )
  })

  it('两个 BASE 与契约向量一致', () => {
    expect(PICK_DISCARD_BASE).toBe(cp.discard_take_base)
    expect(PICK_DISCARD_SKIP_BASE).toBe(cp.discard_skip_base)
  })

  it('两个负编码区间不相交（BASE 之差 >= 每波手牌数）', () => {
    // ⚠️ 判据是 `>=`，不是 `>`：两个区间长度都是 hand_slots，
    // 相差恰好等于 hand_slots 时它们**恰好相邻**而仍不相交。
    // 写成 `>` 会把「恰好相邻」误判成「重叠」——
    // 这类「差一」的错误会让结论整个反过来。
    const gap = PICK_DISCARD_SKIP_BASE - PICK_DISCARD_BASE
    expect(
      gap,
      `两个 BASE 之差 ${gap} < 每波手牌数 ${cp.hand_slots} —— 编码区间会重叠，` +
        '同一个值被解释成两种操作。修法是拉开 BASE，不是改这条断言',
    ).toBeGreaterThanOrEqual(cp.hand_slots)
  })

  it('编码集合与契约向量里的字面期望值一致', () => {
    // 本地重算一遍编码，确认与向量里写死的期望值逐条一致。
    // 判据不是「常量相等」而是「算出来的值等于向量」——
    // 后者能抓住「改了 BASE 但忘了同步向量」这种半途而废。
    const byName: Record<string, number> = {
      plain_pick_first: 0,
      plain_skip: cp.skip,
      discard_then_take_first: -(PICK_DISCARD_BASE + 0),
      discard_then_skip: -PICK_DISCARD_SKIP_BASE,
    }
    for (const c of cp.cases) {
      expect(
        byName[c.name],
        `向量里出现未知用例 ${c.name} —— TS 侧还没有对应编码`,
      ).toBeTypeOf('number')
      expect(byName[c.name], `用例 ${c.name} 编码不符`).toBe(c.expected)
    }
  })

  it('所有合法编码都不低于 PICK_MIN（否则服务端会拒掉客户端自己的产物）', () => {
    const takeLo = -(PICK_DISCARD_BASE + cp.hand_slots - 1)
    const skipLo = -(PICK_DISCARD_SKIP_BASE + cp.hand_slots - 1)
    expect(takeLo).toBeGreaterThanOrEqual(PICK_MIN)
    expect(skipLo).toBeGreaterThanOrEqual(PICK_MIN)
  })

  it('-2 落在空隙里（协议刻意留的，不是笔误）', () => {
    // -1 是「跳过」、-3 是「弃+取」，-2 夹在中间且无语义。
    // 服务端只比合法集合，所以 -2 会被拒 —— 那正是「区间不相交」的实现。
    const isEncoded = (p: number) => {
      if (p >= 0) return p < cp.hand_slots
      if (p === cp.skip) return true
      const n = -p
      return (
        (n >= PICK_DISCARD_BASE && n < PICK_DISCARD_BASE + cp.hand_slots) ||
        (n >= PICK_DISCARD_SKIP_BASE && n < PICK_DISCARD_SKIP_BASE + cp.hand_slots)
      )
    }
    expect(isEncoded(-1), '-1 应是合法的「跳过」').toBe(true)
    expect(isEncoded(-2), '-2 落在两个编码区间之间的空隙，必须无语义').toBe(false)
    expect(isEncoded(-3), '-3 应是合法的「弃+取第 0 张」').toBe(true)
    expect(isEncoded(-6), '-6 应是合法的「弃+跳过」').toBe(true)
    expect(isEncoded(-9), '-9 低于下界，必须无语义').toBe(false)
  })
})