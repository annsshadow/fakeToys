/**
 * 热量系统（I-2）的行为契约。
 *
 * ⚠️ 这个文件的存在理由是一个具体事故。
 * 热量衰减曾经写成 `(HEAT_DECAY_PER_SEC * dtMs) / 1000n`
 * = (10n * 50n) / 1000n = **0n**，即衰减恒为零。
 * 由于本文件当时不存在，没有任何测试覆盖 update() 的非过热分支，
 * 这个缺陷一路活到端到端试玩才暴露：
 * 第 1 关 385 秒只开出 5 发、0 杀 10 漏、0 星 —— 游戏完全不可玩。
 *
 * 所以这里的第一原则是：**每个分支都要有断言**，
 * 尤其是不靠"端点取值"就能测的那些（衰减、余数、死区）。
 */
import { describe, it, expect } from 'vitest'
import {
  HeatSystem,
  HEAT_MAX,
  HEAT_DECAY_PER_SEC,
  OVERHEAT_DURATION_MS,
  DISCARD_PER_WAVE,
  DISCARD_HEAT_REFUND,
  ACTIVE_SLOTS,
  TOTAL_SLOTS,
  PASSIVE_SLOT,
  CardDeck,
  type EquippedSkill,
} from './heatmap'

const TICK = 50

function newHeat(): HeatSystem {
  return new HeatSystem()
}

describe('热量：上限与释放', () => {
  it('初始热量为 0，未过热', () => {
    const h = newHeat()
    expect(h.heat).toBe(0n)
    expect(h.overheated).toBe(false)
  })

  it('cap = HEAT_MAX + 加成', () => {
    const h = newHeat()
    expect(h.cap).toBe(HEAT_MAX)
    h.capBonus = 50n
    expect(h.cap).toBe(HEAT_MAX + 50n)
  })

  it('释放扣热量并记录本局峰值', () => {
    const h = newHeat()
    expect(h.tryCast(20n)).toBe(true)
    expect(h.heat).toBe(20n)
    expect(h.tryCast(30n)).toBe(true)
    expect(h.heat).toBe(50n)
    expect(h.maxHeatThisBattle).toBe(50n)
  })

  // 语义：允许**正好**到上限，超过才拒。
  // 这条是 I-2 代价设计的基础 —— "用尽最后一点热量后被迫过热"。
  it('热量正好到 cap 时允许释放', () => {
    const h = newHeat()
    expect(h.tryCast(HEAT_MAX)).toBe(true)
    expect(h.heat).toBe(HEAT_MAX)
  })

  it('超过 cap 的释放被拒，且不扣热量', () => {
    const h = newHeat()
    expect(h.tryCast(HEAT_MAX + 1n)).toBe(false)
    expect(h.heat).toBe(0n)
  })

  it('过热期间拒绝一切释放', () => {
    const h = newHeat()
    h.tryCast(HEAT_MAX)
    expect(h.checkOverheat()).toBe(true)
    expect(h.tryCast(1n)).toBe(false)
  })
})

describe('热量：弃牌返还', () => {
  it('返还后热量下调，不为负', () => {
    const h = newHeat()
    h.tryCast(20n)
    h.refundHeat(5n)
    expect(h.heat).toBe(15n)
    h.refundHeat(1000n)
    expect(h.heat).toBe(0n)
  })
})

describe('热量：过热', () => {
  it('达到 cap 才进入过热', () => {
    const h = newHeat()
    h.tryCast(HEAT_MAX - 1n)
    expect(h.checkOverheat()).toBe(false)
    expect(h.overheated).toBe(false)
  })

  it('过热持续 OVERHEAT_DURATION_MS 后结束并清空热量', () => {
    const h = newHeat()
    h.tryCast(HEAT_MAX)
    h.checkOverheat()
    expect(h.overheated).toBe(true)

    // 逐 tick 推进，注意最后不足一整 tick 的余数也要能正确结束
    let elapsed = 0
    while (h.overheated && elapsed < OVERHEAT_DURATION_MS * 3) {
      h.update(TICK)
      elapsed += TICK
    }
    expect(h.overheated).toBe(false)
    expect(h.heat).toBe(0n)
  })

  it('过热期间不衰减、不再触发第二次过热', () => {
    const h = newHeat()
    h.tryCast(HEAT_MAX)
    h.checkOverheat()
    expect(h.checkOverheat()).toBe(false) // 已在过热中
    h.update(TICK)
    expect(h.heat).toBe(HEAT_MAX) // 期间不清空
  })

  // 过热结束时元素层数不清除 —— I-2 明确的设计：过热惩罚是热量，不是元素。
  it('过热结束只清热量（元素层数由引擎另行维护）', () => {
    const h = newHeat()
    h.tryCast(HEAT_MAX)
    h.checkOverheat()
    for (let t = 0; t < OVERHEAT_DURATION_MS / TICK + 2; t++) h.update(TICK)
    expect(h.heat).toBe(0n)
  })
})

describe('热量：自然衰减（回归重点）', () => {
  // 这一条直接对应事故：`(10n * 50n) / 1000n === 0n` 让衰减完全失效。
  // 断言用**绝对值**而不是比值 —— 衰减整体乘 10 或恒为 0 都会被抓住。
  it('1 秒应衰减恰好 HEAT_DECAY_PER_SEC 点', () => {
    const h = newHeat()
    h.tryCast(80n)
    expect(h.heat).toBe(80n)
    const perSec = 1000 / TICK
    for (let i = 0; i < perSec; i++) h.update(TICK)
    expect(h.heat).toBe(80n - HEAT_DECAY_PER_SEC)
  })

  it('单个 tick 的衰减不足 1 点时不能被截断为零', () => {
    // 每 tick 名义衰减 0.5 点。跑 1 tick 后热量可能不变（余数不够），
    // 但跑满 2 秒必须精确减 20 —— 若回到旧的截断写法，这里会是 80。
    const h = newHeat()
    h.tryCast(80n)
    for (let i = 0; i < 40; i++) h.update(TICK) // 2 秒
    expect(h.heat).toBe(80n - 2n * HEAT_DECAY_PER_SEC)
  })

  it('衰减率与秒数严格成正比（多档验证）', () => {
    for (const secs of [1, 2, 5, 10]) {
      const h = newHeat()
      h.tryCast(HEAT_MAX)
      for (let i = 0; i < secs * (1000 / TICK); i++) h.update(TICK)
      expect(h.heat).toBe(HEAT_MAX - HEAT_DECAY_PER_SEC * BigInt(secs))
    }
  })

  it('余数机制保证长时间衰减不累积误差', () => {
    // 3 秒 = 60 tick。若实现是"每 tick 独立截断"，
    // 60 tick 会衰减 0 而不是 30。
    const h = newHeat()
    h.tryCast(HEAT_MAX)
    for (let i = 0; i < 60; i++) h.update(TICK)
    expect(h.heat).toBe(HEAT_MAX - 30n)
  })

  it('热量为 0 时不衰减（也不产生负值）', () => {
    const h = newHeat()
    for (let i = 0; i < 100; i++) h.update(TICK)
    expect(h.heat).toBe(0n)
  })

  it('衰减不会把热量减成负数', () => {
    const h = newHeat()
    h.tryCast(1n)
    for (let i = 0; i < 10; i++) h.update(TICK)
    expect(h.heat).toBe(0n)
    expect(h.heat >= 0n).toBe(true)
  })
})

describe('热量：死区兜底（吸收态回归）', () => {
  // 事故的第二个必要条件：heat ∈ (cap-最便宜技能, cap) 时
  // 放不出技能、也触发不了过热。现在由 isStuckFor/forceOverheat 兜住。

  it('够得着最便宜技能时不算被困', () => {
    const h = newHeat()
    h.heat = 80n // 80 + 20 = 100 <= cap，放得下
    expect(h.isStuckFor([20n])).toBe(false)
  })

  it('热量高到连最便宜技能都放不下时判定被困', () => {
    const h = newHeat()
    h.heat = 85n // 85 + 20 = 105 > 100
    expect(h.isStuckFor([20n, 18n])).toBe(true)
  })

  it('任一技能放得下就不算被困', () => {
    const h = newHeat()
    h.heat = 85n
    expect(h.isStuckFor([20n, 10n])).toBe(false) // 10 点的技能还放得下
  })

  it('零热量成本的技能不算被困（否则会被永久过热锁死）', () => {
    const h = newHeat()
    h.heat = 100n
    expect(h.isStuckFor([0n])).toBe(false)
  })

  it('没有技能时不算被困', () => {
    const h = newHeat()
    h.heat = HEAT_MAX
    expect(h.isStuckFor([])).toBe(false)
  })

  it('forceOverheat 无视热量直接进入过热', () => {
    const h = newHeat()
    h.heat = 85n
    expect(h.heat >= h.cap).toBe(false) // 确实没到 cap
    expect(h.forceOverheat()).toBe(true)
    expect(h.overheated).toBe(true)
  })

  it('已在过热时 forceOverheat 返回 false（不重置持续时间）', () => {
    const h = newHeat()
    h.forceOverheat()
    h.update(TICK)
    expect(h.forceOverheat()).toBe(false)
    expect(h.overheatRemaining).toBe(OVERHEAT_DURATION_MS - TICK)
  })

  it('被解救后热量归零，能重新释放技能', () => {
    const h = newHeat()
    h.heat = 85n
    h.forceOverheat()
    for (let t = 0; t < OVERHEAT_DURATION_MS / TICK + 2; t++) h.update(TICK)
    expect(h.heat).toBe(0n)
    expect(h.tryCast(20n)).toBe(true)
  })
})

describe('冷却恢复', () => {
  // tickCooldown 只读 cooldownMs / 写 cooldownRemaining，
  // 所以夹具只需要这两个字段 —— 但类型要用 Pick 而不是 as never，
  // 否则整个数组被推成 never，属性访问直接报错（vitest 能过、tsc 报错）。
  type Coolable = Pick<EquippedSkill, 'cooldownMs' | 'cooldownRemaining'>
  const mkSkill = (cd: number, rem: number): Coolable => ({
    cooldownMs: cd,
    cooldownRemaining: rem,
  })

  it('冷却随时间递减到 0', () => {
    const h = newHeat()
    const s = mkSkill(1000, 1000)
    for (let i = 0; i < 20; i++) h.tickCooldown(TICK, [s])
    expect(s.cooldownRemaining).toBe(0)
  })

  it('冷却不足一 tick 的余数被保留（不丢进度）', () => {
    const h = newHeat()
    const s = mkSkill(100, 75) // 1.5 tick
    h.tickCooldown(TICK, [s]) // 减 50
    expect(s.cooldownRemaining).toBe(25)
    h.tickCooldown(TICK, [s]) // 再减 50 → 归零，不为负
    expect(s.cooldownRemaining).toBe(0)
  })

  it('过热期间冷却不恢复', () => {
    const h = newHeat()
    h.forceOverheat()
    const s = mkSkill(1000, 500)
    for (let i = 0; i < 20; i++) h.tickCooldown(TICK, [s])
    expect(s.cooldownRemaining).toBe(500)
  })
})

describe('手牌与弃牌', () => {
  it('常量自洽：主动槽 4 + 被动槽 1 = 总槽 5', () => {
    expect(ACTIVE_SLOTS).toBe(4)
    expect(PASSIVE_SLOT).toBe(4)
    expect(TOTAL_SLOTS).toBe(5)
  })

  it('每波免费弃牌次数为 1，返还 1 点热量', () => {
    expect(DISCARD_PER_WAVE).toBe(1)
    expect(DISCARD_HEAT_REFUND).toBe(1n)
  })
})

describe('牌堆', () => {
  const mkCard = (id: string) => ({ id, name: id, descr: '', kind: 'skill' as const, rarity: 'common' as const })

  it('setHand 后手牌数正确，size 与 hand.length 一致', () => {
    const d = new CardDeck()
    expect(d.size).toBe(0)
    d.setHand([mkCard('a'), mkCard('b')])
    expect(d.size).toBe(2)
    expect(d.hand.length).toBe(2)
  })

  it('取走手牌后该牌不在手中', () => {
    const d = new CardDeck()
    d.setHand([mkCard('a'), mkCard('b')])
    const target = d.hand[0].id
    const got = d.take(target)
    expect(got).not.toBeNull()
    expect(got!.id).toBe(target)
    expect(d.hand.find((c) => c.id === target)).toBeUndefined()
    expect(d.size).toBe(1)
  })

  it('取不存在的牌返回 null 而不抛异常', () => {
    const d = new CardDeck()
    d.setHand([mkCard('a')])
    expect(d.take('nope')).toBeNull()
    expect(d.size).toBe(1) // 手牌不受影响
  })

  it('弃牌消耗次数并返还热量', () => {
    const d = new CardDeck()
    d.setHand([mkCard('a'), mkCard('b')])
    const r = d.discard('a')
    expect(r.ok).toBe(true)
    expect(r.refund).toBe(DISCARD_HEAT_REFUND)
    expect(d.discardsLeft).toBe(DISCARD_PER_WAVE - 1)
  })

  it('弃牌次数用尽后 discard 返回 ok:false 且不返还', () => {
    const d = new CardDeck()
    d.setHand([mkCard('a')])
    d.discard('a') // 用掉唯一一次
    d.setHand([mkCard('b')])
    const r = d.discard('b')
    expect(r.ok).toBe(false)
    expect(r.refund).toBe(0n)
    expect(d.size).toBe(1) // 牌还在手上
  })

  it('newWave 重置弃牌次数', () => {
    const d = new CardDeck()
    d.setHand([mkCard('a')])
    d.discard('a')
    expect(d.discardsLeft).toBe(0)
    d.newWave()
    expect(d.discardsLeft).toBe(DISCARD_PER_WAVE)
  })

  it('drop 取牌但不消耗弃牌次数（与 discard 的区别）', () => {
    const d = new CardDeck()
    d.setHand([mkCard('a')])
    expect(d.drop('a')).toBe(true)
    expect(d.discardsLeft).toBe(DISCARD_PER_WAVE) // 未消耗
    expect(d.size).toBe(0)
    expect(d.drop('a')).toBe(false) // 已取走
  })
})
