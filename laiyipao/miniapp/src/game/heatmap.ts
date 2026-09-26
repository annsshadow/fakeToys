/**
 * 插槽与热量（I-2）。
 *
 * 解决的问题：原作局内"三选一"是无脑增益。这里给选择装上代价，
 * 让三选一变成风险管理。
 *
 * 规则：
 *  - 4 个主动插槽 + 1 个被动插槽
 *  - 每个波次间隙抽 3 张手牌（1 技能 / 1 属性 / 1 机制）
 *  - 每波可免费弃 1 张，弃牌回 +1 热量
 *  - 热量上限 100，超限进入"过热"（2s 无法释放）
 *  - 但过热期间元素层数不清除 → 高热量爆发是可行策略而非纯惩罚
 */

import type { Element } from './elements'
import type { BattleRng } from './lcg'

export const ACTIVE_SLOTS = 4
export const PASSIVE_SLOT = 4
export const TOTAL_SLOTS = 5

export const HEAT_MAX = 100n
export const OVERHEAT_DURATION_MS = 2000

/**
 * 热量自然衰减（每秒）。
 *
 * ⚠️ 没有衰减是不能玩的：上限 100、单发 20 热量意味着 5 次就过热，
 * 玩家被迫停手 2 秒。5 波次、30+ 敌人的关卡会变成"打 5 下等 2 秒"的循环，
 * 战术空间被完全压死。
 *
 * 有了衰减后：
 *   20 热量技能每 0.8s 一次 → 25/s 消耗 - 10/s 恢复 = 净 15/s，约 7s 过热
 *   40 热量技能每 2.0s 一次 → 20/s 消耗 - 10/s 恢复 = 净 10/s，约 10s 过热
 *   高热量大招因此成为"爆发而非持续"的选择，I-2 的取舍意图得以成立。
 */
export const HEAT_DECAY_PER_SEC = 10n
/** 每波免费弃牌次数 */
export const DISCARD_PER_WAVE = 1
/** 弃一张牌返还的热量 */
export const DISCARD_HEAT_REFUND = 1n

export type CardKind = 'skill' | 'attribute' | 'mechanic'
export type CardRarity = 'common' | 'rare' | 'epic'

/** 手牌 */
export interface Card {
  id: string
  kind: CardKind
  rarity: CardRarity
  name: string
  descr: string
  element?: Element
  /** skill 卡：技能 id 与阶数 */
  skillId?: number
  tier?: number
  /** attribute 卡的效果 */
  effect?: AttributeEffect
  /** mechanic 卡的效果 */
  mechanic?: MechanicEffect
}

export interface AttributeEffect {
  kind: 'attack' | 'element_coef' | 'crit' | 'heat_cap' | 'element_cap' | 'armor'
  /** 千分比增量 */
  value: bigint
}

export interface MechanicEffect {
  kind: 'pierce_bonus' | 'chain_bonus' | 'aoe_bonus' | 'free_discard' | 'overheat_guard'
  value: number
}

/** 已装备的技能 */
export interface EquippedSkill {
  skillId: number
  name: string
  element: Element
  kind: 'active' | 'passive'
  heatCost: bigint
  cooldownMs: number
  pierce: number
  aoeRadius: number
  baseDamage: bigint
  applyElement: Element | ''
  applyStacks: bigint
  projectileSpeed: number
  chain: number
  slot: number
  /** 冷却剩余毫秒 */
  cooldownRemaining: number
}

/** 热量系统 */
export class HeatSystem {
  heat: bigint = 0n
  overheated: boolean = false
  overheatRemaining: number = 0
  capBonus: bigint = 0n
  maxHeatThisBattle: bigint = 0n

  /**
   * 衰减的定点余数（单位：千分之一热量）。
   *
   * 存在的原因见 update() 里的说明：每 tick 的名义衰减量
   * (10/s × 50ms) 只有 0.5 点，整数运算会把它截成 0。
   * 把不足 1 点的部分留在这里，下一 tick 继续累积。
   *
   * 恒在 [0, 1000) 区间，不存在溢出或无界增长。
   */
  private decayRemainder: bigint = 0n

  get cap(): bigint {
    return HEAT_MAX + this.capBonus
  }

  /**
   * 释放技能。返回是否成功。
   *
   * 语义：热量**允许正好到上限**，超过则拒绝。
   * 因此 tryCast(cap) 应当成功，随后 checkOverheat() 判定为过热 ——
   * "用尽最后一点热量后被迫过热"正是 I-2 想要的代价设计。
   */
  tryCast(cost: bigint): boolean {
    if (this.overheated) return false
    if (this.heat + cost > this.cap) return false
    this.heat += cost
    if (this.heat > this.maxHeatThisBattle) this.maxHeatThisBattle = this.heat
    return true
  }

  /** 弃牌返还热量 */
  refundHeat(n: bigint): void {
    this.heat = this.heat - n < 0n ? 0n : this.heat - n
  }

  /** 超过上限则进入过热 */
  checkOverheat(): boolean {
    if (!this.overheated && this.heat >= this.cap) {
      this.overheated = true
      this.overheatRemaining = OVERHEAT_DURATION_MS
      return true
    }
    return false
  }

  /**
   * 强制进入过热（无视 heat 是否到 cap）。
   *
   * 存在的唯一理由：消除「放不出技能又触发不了过热」的吸收态。
   * tryCast 允许正好到 cap，checkOverheat 要求 >= cap，
   * 于是 heat ∈ (cap-最便宜技能, cap) 时既放不出也过热不了。
   * 自然衰减（update 里的余数累加）会最终把它拉出来，
   * 但那要等 0.1~0.8 秒且期间完全没有反馈；更要紧的是，
   * 任何让衰减失效的改动（把它写回 `(rate*dt)/1000`）都会让这个状态变成永久的。
   * 引擎在每 tick 的自动循环前调用 isStuck 判断来兜底。
   *
   * 副作用与正常过热一致：结束时热量清零。
   */
  forceOverheat(): boolean {
    if (this.overheated) return false
    this.overheated = true
    this.overheatRemaining = OVERHEAT_DURATION_MS
    return true
  }

  /**
   * 被「隔热护罩」免疫一次过热：清空热量并退出过热状态。
   *
   * ⚠️ 这里**不能**用 `if (!this.overheated) return false` 做前置检查。
   * 引擎有两条进入 enterOverheat 的路径：
   *   1. 正常开火打满 cap 后 checkOverheat() → overheated 已是 true
   *   2. 热量死区兜底（heatStuck）→ **过热还没开始**，overheated 仍是 false
   * 而死区恰恰是护罩最该起作用的场景（热量卡住、动不了）。
   * 带前置检查会让第 2 条路径下护盾完全无效。
   *
   * 另一个必须做的是**清空热量**：若只把 overheated 置 false，
   * 下一 tick 的死区兜底会再次判定「放不出技能」并再次调用
   * enterOverheat —— 变成「每 tick 白烧一层护盾」的空转，
   * 直到护盾耗尽才真的过热。清空的语义也说得通：护罩替你泄压了。
   */
  absorbOverheat(): void {
    this.overheated = false
    this.overheatRemaining = 0
    this.heat = 0n
  }

  /**
   * 当前是否已无法释放任何给定成本的技能。
   *
   * 供引擎做死区兜底判断。costs 为空时返回 false ——
   * 「没有任何技能」与「技能都放不下」是两回事，
   * 前者不该触发过热。
   */
  isStuckFor(costs: bigint[]): boolean {
    if (costs.length === 0) return false
    for (const c of costs) {
      if (c <= 0n) return false
      if (this.heat + c <= this.cap) return false
    }
    return true
  }

  update(dtMs: number): void {
    if (this.overheated) {
      this.overheatRemaining -= dtMs
      if (this.overheatRemaining <= 0) {
        this.overheated = false
        this.overheatRemaining = 0
        // 过热结束：热量清空，但元素层数不清除（I-2 明确的设计）
        this.heat = 0n
      }
      return
    }
    // 自然衰减。
    //
    // ⚠️ 必须用「余数累加」，不能写成 `(rate * dtMs) / 1000`。
    // 曾经的写法是 `(10n * 50n) / 1000n` = **0n** ——
    // 整数除法把每个 tick 的衰减彻底截断，衰减恒为 0。
    // 于是热量只增不减、只能靠过热清零；而过热又要求 heat >= cap。
    // 两者叠加出一个吸收态：heat 停在 [cap-最便宜技能+1, cap-1] 时
    // 所有技能都放不出、checkOverheat 又永不触发，战斗永久停摆。
    //
    // 另一种解法是把 heat 内部放大 10 倍再存，但 heat 是跨模块的公开字段
    // （引擎索敌、渲染 HUD、结算上报 heat_max 都直接读它），
    // 改单位要动一大片。余数累加把改动局限在这一个方法里，效果完全等价：
    // 每 tick 累积 10*50 = 500，攒够 1000 才减 1 点，
    // 于是 1 秒（20 tick）恰好减 10 点，与 HEAT_DECAY_PER_SEC 的定义一致。
    //
    // decayRemainder 恒 < 1000，不会溢出；
    // 它是 tick 序的确定性函数（重放时每 tick 同样的 dt 得到同样的余数），
    // 因此**不需要**进回放哈希。
    if (this.heat > 0n) {
      this.decayRemainder += HEAT_DECAY_PER_SEC * BigInt(Math.round(dtMs))
      const decay = this.decayRemainder / 1000n
      this.decayRemainder -= decay * 1000n
      this.heat = this.heat > decay ? this.heat - decay : 0n
    }
  }

  /**
   * 冷却自然恢复（仅在未过热时）。
   *
   * ⚠️ 参数类型是 Pick 而不是完整 EquippedSkill。
   * 本方法只读 cooldownMs / 只写 cooldownRemaining，
   * 声明完整类型会强迫调用方与测试方构造 13 个无关字段 ——
   * 测试里只能写 `as never` 或造一堆假数据，两者都会掩盖真正的类型错误。
   * 依赖面越窄越好改。
   */
  tickCooldown(
    dtMs: number,
    skills: Pick<EquippedSkill, 'cooldownMs' | 'cooldownRemaining'>[],
  ): void {
    if (this.overheated) return
    for (const s of skills) {
      if (s.cooldownRemaining > 0) {
        s.cooldownRemaining = Math.max(0, s.cooldownRemaining - dtMs)
      }
    }
  }

  /** 复制状态（用于防线值守的快照） */
  snapshot(): { heat: string; overheated: boolean } {
    return { heat: this.heat.toString(), overheated: this.overheated }
  }
}

/** 手牌管理 */
export class CardDeck {
  hand: Card[] = []
  discardsLeft: number = DISCARD_PER_WAVE

  newWave(): void {
    this.discardsLeft = DISCARD_PER_WAVE
  }

  setHand(cards: Card[]): void {
    this.hand = cards
  }

  get size(): number {
    return this.hand.length
  }

  take(id: string): Card | null {
    const idx = this.hand.findIndex((c) => c.id === id)
    if (idx < 0) return null
    return this.hand.splice(idx, 1)[0]
  }

  /** 弃牌。返回是否成功（次数已用完则失败） */
  discard(id: string): { ok: boolean; refund: bigint } {
    if (this.discardsLeft <= 0) return { ok: false, refund: 0n }
    const card = this.take(id)
    if (!card) return { ok: false, refund: 0n }
    this.discardsLeft--
    return { ok: true, refund: DISCARD_HEAT_REFUND }
  }

  /**
   * 移除手牌但不消耗弃牌次数。
   *
   * skipCards 需要它：玩家"跳过"不是"弃牌"，不应消耗每波一次的弃牌机会 ——
   * 否则"跳过"与"逐张弃掉"是同一个操作，DISCARD_PER_WAVE 的设计就失去意义。
   */
  drop(id: string): boolean {
    return this.take(id) !== null
  }
}

/** 技能牌池：按已解锁技能生成候选 */
export function rollSkillCards(
  rng: BattleRng,
  unlocked: Array<{
    id: number
    name: string
    element: Element
    kind: 'active' | 'passive'
  }>,
  count: number,
): Card[] {
  const out: Card[] = []
  const actives = unlocked.filter((s) => s.kind === 'active')
  if (actives.length === 0) return out
  const used = new Set<number>()
  for (let i = 0; i < count; i++) {
    let pick = actives[rng.intn(actives.length)]
    let guard = 0
    while (used.has(pick.id) && guard++ < actives.length) {
      pick = actives[rng.intn(actives.length)]
    }
    used.add(pick.id)
    out.push({
      id: `skill_${pick.id}_${i}`,
      kind: 'skill',
      rarity: 'common',
      name: pick.name,
      descr: `装备或升格 ${pick.name}`,
      element: pick.element,
      skillId: pick.id,
      tier: 1,
    })
  }
  return out
}

const ATTRIBUTE_POOL: Array<Omit<Card, 'id' | 'kind'>> = [
  {
    rarity: 'common',
    name: '锋锐训练',
    descr: '面板攻击力 +15%',
    effect: { kind: 'attack', value: 150n },
  },
  {
    rarity: 'common',
    name: '元素聚焦',
    descr: '元素系数 +20%',
    effect: { kind: 'element_coef', value: 200n },
  },
  {
    rarity: 'rare',
    name: '锐利视线',
    descr: '暴击率 +8%',
    effect: { kind: 'crit', value: 80n },
  },
  {
    rarity: 'rare',
    name: '散热涂层',
    descr: '热量上限 +20',
    effect: { kind: 'heat_cap', value: 20n },
  },
  {
    rarity: 'epic',
    name: '元素容器',
    descr: '单元素层数上限 +1',
    effect: { kind: 'element_cap', value: 1n },
  },
  {
    rarity: 'common',
    name: '加固工事',
    descr: '防线护甲 +10%',
    effect: { kind: 'armor', value: 100n },
  },
]

const MECHANIC_POOL: Array<Omit<Card, 'id' | 'kind'>> = [
  {
    rarity: 'common',
    name: '强化弹头',
    descr: '所有技能穿透 +1',
    mechanic: { kind: 'pierce_bonus', value: 1 },
  },
  {
    rarity: 'rare',
    name: '连锁导引',
    descr: '所有技能弹射 +1',
    mechanic: { kind: 'chain_bonus', value: 1 },
  },
  {
    rarity: 'common',
    name: '扩张装药',
    descr: '所有技能溅射范围 +15%',
    mechanic: { kind: 'aoe_bonus', value: 15 },
  },
  {
    rarity: 'epic',
    name: '快速装填',
    descr: '每波弃牌次数 +1',
    mechanic: { kind: 'free_discard', value: 1 },
  },
  {
    rarity: 'epic',
    name: '隔热护罩',
    descr: '过热时免疫一次过热',
    mechanic: { kind: 'overheat_guard', value: 1 },
  },
]

export function rollAttributeCards(rng: BattleRng, count: number): Card[] {
  const out: Card[] = []
  for (let i = 0; i < count; i++) {
    const tpl = ATTRIBUTE_POOL[rng.intn(ATTRIBUTE_POOL.length)]
    out.push({ ...tpl, id: `attr_${i}_${rng.intn(1000)}`, kind: 'attribute' })
  }
  return out
}

export function rollMechanicCards(rng: BattleRng, count: number): Card[] {
  const out: Card[] = []
  for (let i = 0; i < count; i++) {
    const tpl = MECHANIC_POOL[rng.intn(MECHANIC_POOL.length)]
    out.push({ ...tpl, id: `mech_${i}_${rng.intn(1000)}`, kind: 'mechanic' })
  }
  return out
}

/** 每波抽 3 张：1 技能 + 1 属性 + 1 机制（I-2 的固定构成） */
export function rollWaveCards(
  rng: BattleRng,
  unlocked: Array<{ id: number; name: string; element: Element; kind: 'active' | 'passive' }>,
): Card[] {
  return [
    ...rollSkillCards(rng, unlocked, 1),
    ...rollAttributeCards(rng, 1),
    ...rollMechanicCards(rng, 1),
  ]
}
