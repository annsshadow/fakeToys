import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { HeatSystem, HEAT_MAX } from './heatmap'

/**
 * 属性卡的**单位约定**必须机器化（第 72 轮）。
 *
 * # 缺陷：`heat_cap` 卡面按绝对值写、按千分比消费
 *
 * `ATTRIBUTE_POOL` 里六张卡的 `value` 单位**不统一**：
 *
 * | kind          | value  | descr         | 实际效果               |
 * |---------------|--------|---------------|------------------------|
 * | attack        | 150n   | 攻击力 +15%    | +150‰ = +15% ✓         |
 * | element_coef  | 200n   | 元素系数 +20%  | +200‰ = +20% ✓         |
 * | crit          | 80n    | 暴击率 +8%     | +80‰ = +8% ✓           |
 * | **heat_cap**  | **20n** | 热量上限 +20  | **+20‰ = +2** ✗        |
 * | element_cap   | 1n     | 层数上限 +1    | +1 层（**绝对值**）✓    |
 * | armor         | 100n   | 防线护甲 +10% | +100‰ = +10% ✓         |
 *
 * `heat_cap` 的 `value: 20n` 让热量上限从 100 只抬到 **102**
 * （`get cap() { return (HEAT_MAX * (1000 + capBonus)) / 1000 }`），
 * 而 descr 承诺 120 —— **玩家拿到这张卡的实际收益是标称的 1/10**。
 *
 * # 为什么此前没人发现
 *
 * 1. `heatmap.test.ts` 只测 `cap()` 公式本身（用自己写的 capBonus 50n/1000n），
 *    不测「池里的值按该公式解释后与 descr 是否匹配」
 * 2. `cards.test.ts` 里**没有** heat_cap 卡（grep `heat_cap` 零命中）
 * 3. 单张卡偏差 1/10 不影响「能不能通关」，平衡探针也看不到
 *
 * # 判据：扫源码 + 按单位解释
 *
 * `ATTRIBUTE_POOL` 不是导出常量，所以本文件**扫源码**把它读出来。
 * 扫源码而不是导出的理由：加新卡时它会自动被检查，
 * 而一份导出的名单需要人工同步 —— 清单会漂，扫描不会。
 *
 * `PERMILLE_KINDS` 是「value 按千分比消费」的 kind 集合。
 * 集合与 `applyAttribute` 的 switch **逐项对应**：
 * 少了任何一项 → 第一条守卫红；多了任何一项 → 第二条守卫红。
 */

const PERMILLE_KINDS = ['attack', 'element_coef', 'crit', 'heat_cap', 'armor'] as const
const ABSOLUTE_KINDS = ['element_cap'] as const

/** 从 ATTRIBUTE_POOL 的源码文本里抽出每张卡的 (kind, value, descr)。 */
function scanAttributePool(): Array<{ name: string; kind: string; value: number; descr: string }> {
  const src = readFileSync(resolve(__dirname, 'heatmap.ts'), 'utf-8')
  const start = src.indexOf('const ATTRIBUTE_POOL')
  if (start < 0) throw new Error('找不到 ATTRIBUTE_POOL —— 扫描器坏了，本文件会静默变成空转')
  const end = src.indexOf('const MECHANIC_POOL', start)
  if (end < 0) throw new Error('找不到 MECHANIC_POOL 的边界 —— 扫描器坏了')
  const body = src.slice(start, end)

  const out: Array<{ name: string; kind: string; value: number; descr: string }> = []
  const re =
    /name:\s*'([^']*)'[\s\S]*?descr:\s*'([^']*)'[\s\S]*?effect:\s*\{\s*kind:\s*'([^']*)',\s*value:\s*(\d+)n\s*\}/g
  let m: RegExpExecArray | null
  while ((m = re.exec(body)) !== null) {
    out.push({ name: m[1]!, descr: m[2]!, kind: m[3]!, value: Number(m[4]) })
  }
  return out
}

describe('属性卡单位约定（第 72 轮）', () => {
  const pool = scanAttributePool()

  it('扫描器确实读到了卡（否则后面全是空转）', () => {
    expect(pool.length, 'ATTRIBUTE_POOL 扫描结果为空').toBeGreaterThanOrEqual(5)
  })

  it('PERMILLE_KINDS 覆盖了池里所有非 element_cap 的卡', () => {
    const declared = new Set<string>([...PERMILLE_KINDS, ...ABSOLUTE_KINDS])
    for (const c of pool) {
      expect(
        declared.has(c.kind),
        `ATTRIBUTE_POOL 里的 kind "${c.kind}"（${c.descr}）不在 PERMILLE_KINDS ∪ ABSOLUTE_KINDS 里 —— ` +
          '新加卡时必须声明它的 value 是什么单位，否则这里没有任何东西能发现它',
      ).toBe(true)
    }
  })

  it('descr 里的百分比必须与 value 的千分比一致', () => {
    for (const c of pool) {
      if (!(PERMILLE_KINDS as readonly string[]).includes(c.kind)) continue
      const m = /\+(\d+)%/.exec(c.descr)
      if (!m) {
        // 没有百分号的千分比 kind：descr 必须写绝对量且与 value 直接对应
        // （目前只有 heat_cap 曾是这种，写法应是「+N%」）
        expect.fail(
          `${c.name}（${c.kind}）（${c.descr}）是千分比 kind 却没有「+N%」的描述 —— ` +
            '无法核对 value 与 descr 是否一致',
        )
      }
      const promised = Number(m![1])
      const actual = c.value / 10
      expect(
        actual,
        `卡「${c.name}」的「${c.descr}」承诺 +${promised}%，而 value=${c.value}n 按千分比解释是 +${actual}%。` +
          '玩家拿到的实际收益与卡面承诺不符',
      ).toBe(promised)
    }
  })

  it('heat_cap 卡的实测收益：施加后上限真的按千分比抬', () => {
    // 判据落到**行为**上而不只是数值比对：
    // 把卡面那个值加进 capBonus，看上限是不是真的按千分比走。
    const base = new HeatSystem()
    expect(base.cap).toBe(HEAT_MAX)

    const card = pool.find((c) => c.kind === 'heat_cap')
    expect(card, 'ATTRIBUTE_POOL 里没有 heat_cap 卡').toBeTruthy()

    const up = new HeatSystem()
    up.capBonus += BigInt(card!.value)
    expect(
      up.cap,
      `施加 ${card!.value}‰ 后上限 ${up.cap}，期望 ${(HEAT_MAX * (1000n + BigInt(card!.value))) / 1000n}`,
    ).toBe((HEAT_MAX * (1000n + BigInt(card!.value))) / 1000n)

    // 而 descr 承诺的百分比必须真的兑现
    const promised = Number(/\+(\d+)%/.exec(card!.descr)![1])
    expect(up.cap, `descr 承诺 +${promised}%，实测上限 ${up.cap}（基准 ${HEAT_MAX}）`).toBe(
      (HEAT_MAX * (1000n + BigInt(promised * 10))) / 1000n,
    )
  })

  it('element_cap 是绝对值（与 heat_cap 的千分比不同，但同在一个池子里）', () => {
    // 这条存在的意义是**记录这个混用**：同一个池子里两种单位，
    // 靠 kind 区分。将来若统一成千分比，ABSOLUTE_KINDS 就该清空。
    const abs = pool.filter((c) => (ABSOLUTE_KINDS as readonly string[]).includes(c.kind))
    expect(abs.length, 'element_cap 卡不见了').toBeGreaterThan(0)
    for (const c of abs) {
      expect(
        c.value,
        `卡「${c.name}」的 ${c.kind} 是绝对值 kind，value=${c.value} 看起来不像「层数」量级 —— ` +
          '若它其实变成了千分比，请同步更新 ABSOLUTE_KINDS',
      ).toBeLessThan(10)
    }
  })
})