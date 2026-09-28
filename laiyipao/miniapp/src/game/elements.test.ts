import { describe, it, expect } from 'vitest'
import { LCG } from './lcg'
import {
  lookupReaction,
  reachableReactions,
  isElement,
  ELEMENT_ORDER,
  REACTION_ORDER,
  REACTIONS,
  type Element,
} from './elements'
import { MAX_REACTION_ATTACK_WEIGHT } from './fixed'
import levelSeeds from '@vectors/level_seeds.json'

describe('反应链数据完整性', () => {
  it('恰好 7 条反应链', () => {
    expect(REACTION_ORDER.length).toBe(7)
  })

  it('每条反应的攻击力权重都必须 ≤ 30%（I-1 红线）', () => {
    for (const key of REACTION_ORDER) {
      expect(REACTIONS[key].attackWeightPct, `${REACTIONS[key].name} 攻击力权重`).toBeLessThanOrEqual(
        MAX_REACTION_ATTACK_WEIGHT,
      )
    }
  })
})

describe('反应查表与 Go 侧一致', () => {
  const cases: Array<[Element, Element, string | null]> = [
    ['fire', 'fire', null],
    ['fire', 'ice', 'steam_burst'],
    ['fire', 'lightning', 'overheat'],
    ['fire', 'corrosion', 'burn_cloud'],
    ['fire', 'kinetic', 'armor_break'],
    ['ice', 'fire', 'steam_burst'],
    ['ice', 'lightning', 'superconduct'],
    ['ice', 'corrosion', 'flash_freeze'],
    ['lightning', 'ice', 'superconduct'],
    ['lightning', 'corrosion', 'corrosion_spread'],
    ['corrosion', 'lightning', 'corrosion_spread'],
    ['kinetic', 'fire', 'armor_break'],
    ['kinetic', 'ice', 'armor_break'],
  ]

  for (const [existing, incoming, want] of cases) {
    it(`${existing} + ${incoming} → ${want ?? '无反应'}`, () => {
      expect(lookupReaction(existing, incoming)).toBe(want)
    })
  }

  it('空元素不触发反应', () => {
    expect(lookupReaction('', 'fire')).toBeNull()
    expect(lookupReaction('fire', '')).toBeNull()
  })

  it('未知的施加元素（网络脏值）安全回退为无反应，而非抛错或误算', () => {
    // 为什么钉这条分支：incoming 的静态类型是 Element，但它来自网络
    // ——构筑快照 / 关卡行经 JSON 往返、灰度期新元素下发到旧客户端，
    // 都可能送来一个类型上"不存在"的元素名。此时 REACTION_TABLE[incoming]
    // 为 undefined（`!byIncoming` 分支），必须回退成 null；若放行，
    // 下游会拿着 undefined 继续算反应伤害并静默出错（与 damage.ts 里
    // 「同名不同义」隐患同源）。这正是 lookupReaction 第一道空表守卫。
    const bogus = 'plasma' as unknown as Element
    // incoming 未知：命中 `if (!byIncoming) return null`
    expect(lookupReaction('fire', bogus)).toBeNull()
    // existing 未知：走到二级查表 `byIncoming[existing] ?? null`，
    // 同样不得抛错，回退 null
    expect(lookupReaction(bogus, 'fire')).toBeNull()
  })
})

describe('reachableReactions（构筑评分用）', () => {
  it('单一元素无法触发任何反应（反应必须有先后两种元素）', () => {
    for (const e of ELEMENT_ORDER) {
      expect(reachableReactions(new Set([e])).size).toBe(0)
    }
  })

  it('焰 + 冰 只能出蒸汽爆发', () => {
    const out = reachableReactions(new Set<Element>(['fire', 'ice']))
    expect(out).toEqual(new Set(['steam_burst']))
  })

  it('全元素搭配恰好覆盖全部 7 条反应（覆盖率评分的满分前提）', () => {
    const out = reachableReactions(new Set(ELEMENT_ORDER))
    expect(out).toEqual(new Set(REACTION_ORDER))
  })

  it('动能在搭配里永远只贡献破甲击退', () => {
    // REACTION_TABLE 里 kinetic 作先手只映射 armor_break ——
    // 这是「动能不能作为已附着元素触发新反应」的读侧体现。
    const withKinetic = reachableReactions(new Set<Element>(['kinetic', 'fire', 'ice', 'lightning', 'corrosion']))
    expect(withKinetic.has('armor_break')).toBe(true)
    expect(withKinetic.size).toBe(REACTION_ORDER.length)
  })
})

describe('isElement（网络数据校验入口）', () => {
  it('合法元素名返回 true', () => {
    for (const e of ELEMENT_ORDER) expect(isElement(e)).toBe(true)
  })

  it('非法元素名返回 false 且类型收窄可用', () => {
    // 服务端 /config 的元素串经 JSON 往返，拼错一个字母就类型上
    // "不存在"。isElement 是把 unknown 收窄回 Element 的唯一闸门，
    // 它放行一个非法值，下游查表就会拿到 undefined 静默错下去。
    const bogus = ['flame', 'FIRE', '', 'fir e', 'poison'] as unknown as string[]
    for (const s of bogus) {
      expect(isElement(s)).toBe(false)
    }
  })
})

/**
 * 跨端确定性回归：同一 seed 生成的 100 关必须在 Go 与 TS 侧完全一致。
 * ⚠️ 曾经的实现是「在 TS 侧重写一遍 Go 的公式做对照」，那是三重自证（见下方说明）。
 */
/**
 * 关卡种子的跨端一致性。
 *
 * ⚠️ 这个 describe 块曾经是**假的**。
 *
 * 原实现是：
 *   const PHI = 0x9e3779b97f4a7c15n                       // 测试内本地复制的常量
 *   function levelSeed(id) { return ... ^ (id * PHI) }   // 测试内本地复制的公式
 *   expect(levelSeed(1)).toBe(levelSeed(1))               // 恒等式
 *
 * 三重自证：断言自己等于自己、常量抄了一份、公式也抄了一份。
 * 实测把 Go 侧的 0x9E3779B97F4A7C15 改掉，这条名叫「与 Go 侧一致」的
 * 用例照样全绿。
 *
 * 根本原因是 TS 端**没有关卡生成实现**（关卡由 Go 的 levelgen 现算，
 * 客户端只接收结果）。所以客户端能做的只有一件事：
 * 拿 Go 侧导出的**真值**字面量做断言。
 *
 * 真值来自 testdata/level_seeds.json（由 `go run ./cmd/vectors` 导出）。
 * 这才是跨端锁：改 Go 侧实现 -> -check 报过期 -> 客户端测试红。
 */
describe('关卡种子跨端一致性', () => {
  const seeds = levelSeeds as string[]

  it('契约文件覆盖 1..100 关', () => {
    expect(seeds.length).toBe(100)
    expect(seeds.every((s) => typeof s === 'string' && s.length > 0)).toBe(true)
  })

  it('100 个 seed 互不相同（否则两关的战斗序列会重复）', () => {
    expect(new Set(seeds).size).toBe(100)
  })

  it('第 1 关 seed 等于 Go 侧的真值（字面量，不重算）', () => {
    // ⚠️ 期望值是字面量，不是现算。
    // 写成 `expect(seed(1)).toBe(seed(1))` 就退化成恒等式；
    // 写成 `expect(seed(1)).toBe(recomputed)` 就退化成本地自证 ——
    // 两种都挡不住 Go 侧被改。
    expect(seeds[0]).toBe('-7046029255919282421')
    expect(seeds[1]).toBe('4354685563387073332')
    expect(seeds[2]).toBe('-2691343690999341279')
  })

  it('seed 能被 BigInt 解析（跨端字符串承载，无 2^53 失真）', () => {
    // 走 JSON number 会失真：-7046029255919282421 超过 2^53。
    // 所以契约文件与 /battle/token 的 seed 一样用字符串承载。
    for (const s of seeds) {
      expect(() => BigInt(s)).not.toThrow()
    }
    // 至少要有一个超过 2^53 的值，否则这条测试是空转
    const big = seeds.filter((s) => {
      const v = BigInt(s)
      return v > BigInt(Number.MAX_SAFE_INTEGER) || v < BigInt(Number.MIN_SAFE_INTEGER)
    })
    expect(big.length).toBeGreaterThan(0)
  })

  it('同 seed 的 LCG 序列可复现（回放验真的前提）', () => {
    const a = new LCG(BigInt(seeds[0]))
    const b = new LCG(BigInt(seeds[0]))
    for (let i = 0; i < 50; i++) {
      expect(a.next()).toBe(b.next())
    }
  })
})
