import { describe, it, expect } from 'vitest'
import vectors from '@vectors/formula_vectors.json'
import { PICK_CODE_BASE, PICK_CODE_MAX, PICK_MIN, encodeDiscardCode } from './engine'

/**
 * card_picks 编码的**跨端契约**（第 66 轮引入，第 159 轮重写）。
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

  it('code 基址与上界和契约向量一致', () => {
    expect(PICK_CODE_BASE).toBe(cp.code_base)
    expect(PICK_CODE_MAX).toBe(cp.code_max)
  })

  it('字面编码用例与契约向量一致', () => {
    // 本地重算一遍编码，确认与向量里写死的期望值逐条一致。
    // 判据不是「常量相等」而是「算出来的值等于向量」——
    // 后者能抓住「改了枚举但忘了同步向量」这种半途而废。
    const byName: Record<string, number> = {
      plain_pick_first: 0,
      plain_pick_last: cp.hand_slots - 1,
      plain_skip: cp.skip,
      // mask 位 i = 弃原手牌第 i 张；take = 取剩余（按原序）第几张，-1 = 不取
      discard_first_take_first: -(PICK_CODE_BASE + encodeDiscardCode(0b001, 0)),
      discard_last_take_first: -(PICK_CODE_BASE + encodeDiscardCode(0b100, 0)),
      discard_first_skip: -(PICK_CODE_BASE + encodeDiscardCode(0b001, -1)),
      discard_two_take_remaining: -(PICK_CODE_BASE + encodeDiscardCode(0b011, 0)),
      discard_two_skip: -(PICK_CODE_BASE + encodeDiscardCode(0b011, -1)),
      discard_all_skip: -(PICK_CODE_BASE + encodeDiscardCode(0b111, -1)),
    }
    for (const c of cp.cases) {
      expect(
        byName[c.name],
        `向量里出现未知用例 ${c.name} —— TS 侧还没有对应编码`,
      ).toBeTypeOf('number')
      expect(byName[c.name], `用例 ${c.name} 编码不符`).toBe(c.expected)
    }
    // 手推核对：枚举表里 mask=1/4/3/7 的区间起点分别是 4/12/10/19。
    // 这条把「枚举表」与「向量字面值」绑在一起，两边不一致时必红。
    expect(byName.discard_first_take_first, '弃0取剩余0 应落在 code=4').toBe(-7)
    expect(byName.discard_last_take_first, '弃2取剩余0 应落在 code=12').toBe(-15)
    expect(byName.discard_first_skip, '弃0不取 应落在 code=6').toBe(-9)
    expect(byName.discard_two_take_remaining, '弃0,1取剩余0 应落在 code=10').toBe(-13)
    expect(byName.discard_two_skip, '弃0,1不取 应落在 code=11').toBe(-14)
    expect(byName.discard_all_skip, '弃全部 应落在 code=19').toBe(-22)
  })

  it('所有合法编码都不低于 PICK_MIN（否则服务端会拒掉客户端自己的产物）', () => {
    // 最大的 code 是 mask=7 的 19 → -(3+19) = -22 = PICK_MIN
    expect(PICK_MIN).toBe(-(PICK_CODE_BASE + PICK_CODE_MAX))
    for (let mask = 0; mask < 8; mask++) {
      const lit = encodeDiscardCode(mask, -1)
      expect(-(PICK_CODE_BASE + lit)).toBeGreaterThanOrEqual(PICK_MIN)
    }
  })

  it('-2 落在空隙里（协议刻意留的，不是笔误）', () => {
    // -1 是「跳过」、-3 起是「弃过牌」，-2 夹在中间且无语义。
    // 服务端只比合法集合，所以 -2 会被拒 —— 那正是「值域不撞车」的实现。
    const isEncoded = (p: number) => {
      if (p >= 0) return p < cp.hand_slots
      if (p === cp.skip) return true
      return p <= -PICK_CODE_BASE && p >= PICK_MIN
    }
    expect(isEncoded(-1), '-1 应是合法的「跳过」').toBe(true)
    expect(isEncoded(-2), '-2 落在编码间隙，必须无语义').toBe(false)
    expect(isEncoded(-3), '-3 应是合法的负编码（mask=0 的冗余编码）').toBe(true)
    expect(isEncoded(-22), '-22 应是合法的负编码（弃全部）').toBe(true)
    expect(isEncoded(-23), '-23 低于下界，必须无语义').toBe(false)
  })
})
