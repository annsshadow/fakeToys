/**
 * 暴击率的**分布**测试。
 *
 * ⚠️ 这个文件补的是一个真实事故：
 * `roll` 曾经是万分比（0..9999）而判定阈值是千分比，
 * 于是 critPermille=50（标称 5%）实际产生 **90.5%** 暴击率。
 *
 * 为什么端点断言抓不到：既有测试只测 critPermille=1000（必暴）与
 * 0（必不暴）。这两个端点在 10× 错误下**都成立** ——
 * roll>=0 恒真、roll>=1000 对 roll∈[0,9999] 也有 90% 成立。
 * 两个端点同时正确，是 10× 错误能长期存活的原因。
 *
 * 所以这里必须用**分布**：穷举全部 roll，看实际发生率是否落在标称值附近。
 * 只有统计能区分"阈值对"和"阈值差一个数量级"。
 */
import { describe, it, expect } from 'vitest'
import { BattleRng } from './lcg'
import { resolveHit, Defender, defaultAttacker } from './damage'

/**
 * 在给定 roll 上跑一次判定，返回是否暴击。
 *
 * 签名是 resolveHit(att, def, input) —— 攻方与守方都是独立参数，
 * 不是塞在 input 里。
 *
 * HP 给得很大（1e6）是为了让这次判定**不杀不死**：
 * 击杀会改变后续的护盾扣除路径，而我们只关心暴击这一个布尔。
 * 守方无元素层数、无抗性、无护甲，把其它变量全部固定住。
 */
function critAt(roll: number, critPermille: bigint): boolean {
  const def = new Defender(1_000_000n, 0n, 0n)
  const res = resolveHit({ ...defaultAttacker(), critPermille }, def, {
    skillDamage: 100n,
    skillElement: 'fire',
    roll,
  })
  return res.crit
}

describe('暴击判定：量纲', () => {
  it('roll 必须是 0..999 的千分比', () => {
    const rng = new BattleRng(12345)
    for (let i = 0; i < 5000; i++) {
      const r = rng.roll()
      expect(Number.isInteger(r)).toBe(true)
      expect(r).toBeGreaterThanOrEqual(0)
      // ⚠️ 上界 999 是本次修复的核心。此前是 9999。
      expect(r).toBeLessThan(1000)
    }
  })

  it('roll 的分布覆盖整个 0..999 区间（不是偏在某个子区间）', () => {
    const rng = new BattleRng(4242)
    const seen = new Set<number>()
    for (let i = 0; i < 20000; i++) seen.add(rng.roll())
    // 20000 次抽样应覆盖绝大多数可能值；若 roll 仍是 0..9999，
    // 覆盖率会低到 20% 左右
    expect(seen.size).toBeGreaterThan(900)
  })
})

describe('暴击判定：阈值语义（端点）', () => {
  it('critPermille=1000：所有 roll 都暴击', () => {
    for (let r = 0; r < 1000; r += 37) {
      expect(critAt(r, 1000n)).toBe(true)
    }
  })

  it('critPermille=0：没有 roll 会暴击', () => {
    for (let r = 0; r < 1000; r += 37) {
      expect(critAt(r, 0n)).toBe(false)
    }
  })

  it('critPermille=500：roll >= 500 暴击，以下不暴击', () => {
    for (let r = 0; r < 500; r += 41) {
      expect(critAt(r, 500n)).toBe(false)
    }
    for (let r = 500; r < 1000; r += 41) {
      expect(critAt(r, 500n)).toBe(true)
    }
  })
})

describe('暴击率：分布（这条才是真正的守卫）', () => {
  // 穷举 1000 个 roll，统计实际暴击次数。
  // 期望：实际暴击率 ≈ critPermille / 1000，误差 < 1.5 个百分点。
  //
  // 修复前（roll 0..9999 而阈值按千分比算）：
  //   critPermille=50  → 实际 905/1000 = 90.5%（标称 5%）
  //   critPermille=1000 → 实际 100%（标称 100%）  ← 端点仍然正确！
  // 正是"端点对、中间全错"的形状，所以端点测试完全测不出来。
  for (const [permille, nominal] of [
    [50, 0.05],
    [130, 0.13],
    [250, 0.25],
    [500, 0.5],
    [750, 0.75],
  ] as const) {
    it(`标称 ${(nominal * 100).toFixed(0)}% 暴击率应实际等于该值`, () => {
      let crit = 0
      for (let r = 0; r < 1000; r++) {
        if (critAt(r, BigInt(permille))) crit++
      }
      const actual = crit / 1000
      expect(Math.abs(actual - nominal)).toBeLessThan(0.015)
    })
  }

  it('暴击率随 critPermille 严格单调递增（成长维度有梯度）', () => {
    const rates: number[] = []
    for (const p of [0, 100, 200, 400, 600, 800, 1000]) {
      let crit = 0
      for (let r = 0; r < 1000; r++) if (critAt(r, BigInt(p))) crit++
      rates.push(crit / 1000)
    }
    for (let i = 1; i < rates.length; i++) {
      expect(rates[i]).toBeGreaterThan(rates[i - 1])
    }
    // 首尾必须是真的 0% 与 100% —— 修复前中段全挤在 90%+，
    // 这条会在中途某处变成"几乎不增长"而失败
    expect(rates[0]).toBe(0)
    expect(rates[rates.length - 1]).toBe(1)
  })
})
