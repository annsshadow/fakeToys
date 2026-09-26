/**
 * 伤害结算 —— 与 server/internal/domain/damage.go 逐行等价。
 *
 * 三段式伤害模型：
 *   直接伤害     = D × 0.40 × (1-护甲)              吃养成
 *   元素层数伤害 = E × 层数 × 元素系数 / 40           只吃元素投入
 *   反应伤害     = (E × 层数 × k × 阶 × 系数 + 攻击侧) × (1-抗性)
 *
 * 反通胀不变量（结构性保证）：
 *   attackSide = min(D × k × 阶,  w × elemSide / (1-w))
 *   ⇒ attackSide / (elemSide + attackSide) ≤ w ≤ 30%
 *
 * 这条不变量意味着：堆金币/等级永远无法替代正确的元素搭配。
 * server 侧的 TestReactionAttackRatioStaysUnderCap 断言同一性质。
 */

import {
  PERMILLE,
  DIRECT_WEIGHT,
  ELEMENT_PER_STACK_BASE,
  ELEMENT_TICK_DIVISOR,
  MAX_REACTION_ATTACK_WEIGHT,
  MAX_RESIST,
  mulDiv,
  clampInt64,
  applyArmor,
  minBig,
} from './fixed'
import {
  ELEMENT_ORDER,
  type Element,
  type ReactionKey,
  REACTIONS,
  lookupReaction,
} from './elements'
import type { BattleRng } from './lcg'

/** 攻方养成属性快照 */
export interface Attacker {
  /** 面板攻击力（千分比叠加） */
  attack: bigint
  /** 暴击率（千分比） */
  critPermille: bigint
  /** 暴击倍率（千分比，1500 = 150%） */
  critMultiplierPermille: bigint
  /** 反应伤害倍率（千分比） */
  reactionMultPermille: bigint
  /** 单元素层数上限 */
  elementCap: bigint
  /** 反应等级 1..3（技能阶数） */
  reactionTier: bigint
  /** 元素层数系数（千分比） */
  elementCoefPermille: bigint
}

/** 新号默认属性，与 Go 的 DefaultAttacker 一致 */
export function defaultAttacker(): Attacker {
  return {
    attack: 100n,
    critPermille: 50n,
    critMultiplierPermille: 1500n,
    reactionMultPermille: 1000n,
    elementCap: 3n,
    reactionTier: 1n,
    elementCoefPermille: 1000n,
  }
}

/** 守方状态 */
export class Defender {
  hp: bigint
  shield: bigint
  armorPermille: bigint
  resist: Map<Element, bigint>
  stacks: Map<Element, bigint>

  constructor(hp: bigint, shield: bigint, armorPermille: bigint) {
    this.hp = hp
    this.shield = shield
    this.armorPermille = armorPermille
    this.resist = new Map()
    this.stacks = new Map()
  }

  resistOf(e: Element): bigint {
    const v = this.resist.get(e)
    if (v === undefined) return 0n
    return clampInt64(v, -MAX_RESIST, MAX_RESIST)
  }

  stacksOf(e: Element): bigint {
    return this.stacks.get(e) ?? 0n
  }

  /** 施加元素层数（受上限约束），返回是否真的增加了 */
  applyElement(e: Element, add: bigint, cap: bigint): boolean {
    if (!e || add <= 0n) return false
    const cur = this.stacksOf(e)
    let next = cur + add
    if (next > cap) next = cap
    if (next === cur) return false
    this.stacks.set(e, next)
    return true
  }

  clearElements(): void {
    this.stacks = new Map()
  }

  totalStacks(): bigint {
    let total = 0n
    for (const e of ELEMENT_ORDER) total += this.stacksOf(e)
    return total
  }

  /** 层数最多的元素（相同时取 ELEMENT_ORDER 中靠前者） */
  dominantElement(): Element | '' {
    let best: Element | '' = ''
    let bestStacks = 0n
    for (const e of ELEMENT_ORDER) {
      const s = this.stacksOf(e)
      if (s > bestStacks) {
        best = e
        bestStacks = s
      }
    }
    return best
  }
}

/**
 * 一次命中的输入。
 *
 * applyStacks 允许省略（按 0 处理）—— 多数测试只想验证伤害与反应，
 * 不必关心层数。省略时结算仍完全确定。
 */
export interface HitInput {
  skillDamage: bigint
  skillElement: Element | ''
  applyStacks?: bigint
  forceReaction?: ReactionKey
  reactionElement?: Element
  /** 0..9999 的确定性滚点，来自 PRNG */
  roll: number
}

/** 一次命中的结果 */
export interface HitResult {
  directDamage: bigint
  elementDamage: bigint
  reactionDamage: bigint
  reactionElemPortion: bigint
  reactionAttackPortion: bigint
  totalDamage: bigint
  reaction: ReactionKey | ''
  resistAppliedPermille: bigint
  crit: boolean
  shieldBroken: boolean
  killed: boolean
  dispelShield: boolean
  statusDurationMs: number
}

/** 决定本次命中触发哪条反应 */
export function resolveReaction(def: Defender, input: HitInput): ReactionKey | null {
  if (input.forceReaction) return input.forceReaction
  const dominant = def.dominantElement()
  if (!dominant || def.stacksOf(dominant) === 0n) return null
  return lookupReaction(dominant, input.skillElement)
}

/** 结构性保证：攻击力侧的上限 = w/(1-w) × 元素侧 */
function reactionAttackCap(w: bigint, elemPortion: bigint): bigint {
  if (w >= PERMILLE) return elemPortion * PERMILLE
  return mulDiv(w, elemPortion, PERMILLE - w)
}

/**
 * 完整命中结算。全项目唯一伤害入口。
 * 计算顺序本身影响结果，必须与服务端一致。
 */
export function resolveHit(att: Attacker, def: Defender, input: HitInput): HitResult {
  const res: HitResult = {
    directDamage: 0n,
    elementDamage: 0n,
    reactionDamage: 0n,
    reactionElemPortion: 0n,
    reactionAttackPortion: 0n,
    totalDamage: 0n,
    reaction: '',
    resistAppliedPermille: PERMILLE,
    crit: false,
    shieldBroken: false,
    killed: false,
    dispelShield: false,
    statusDurationMs: 0,
  }

  // 1) 基准伤害：攻击力以千分比叠加，避免除法
  let d = mulDiv(input.skillDamage, PERMILLE + att.attack, PERMILLE)
  if (d < 0n) d = 0n

  // 2) 直接伤害：唯一的养成主来源
  let direct = applyArmor(mulDiv(d, DIRECT_WEIGHT, PERMILLE), def.armorPermille)

  // 3) 元素层数持续伤害：同样受技能元素抗性约束（否则是绕过抗性矩阵的后门）
  const stacks = def.totalStacks()
  let elementDmg = 0n
  if (stacks > 0n && input.skillElement) {
    elementDmg = ELEMENT_PER_STACK_BASE * stacks
    elementDmg = mulDiv(elementDmg, att.elementCoefPermille, PERMILLE)
    elementDmg = elementDmg / ELEMENT_TICK_DIVISOR
    elementDmg = mulDiv(elementDmg, PERMILLE - def.resistOf(input.skillElement), PERMILLE)
    if (elementDmg < 0n) elementDmg = 0n
  }

  // 4) 反应伤害
  const reactionKey = resolveReaction(def, input)
  let reactionDmg = 0n
  let attackPortion = 0n

  if (reactionKey) {
    const spec = REACTIONS[reactionKey]
    const k = BigInt(spec.baseCoef)
    const w = BigInt(spec.attackWeightPct)
    const t = att.reactionTier

    // 元素侧：绝对伤害基准 × 层数 × 反应系数 × 阶 × 元素系数，与养成完全无关
    let elemSide = ELEMENT_PER_STACK_BASE * stacks
    elemSide = mulDiv(elemSide, k, PERMILLE)
    elemSide = elemDiv(elemSide, t)
    elemSide = mulDiv(elemSide, att.elementCoefPermille, PERMILLE)
    res.reactionElemPortion = elemSide

    // 攻击力侧：吃养成，但有结构性上限
    let attackRaw = mulDiv(d, k, PERMILLE)
    attackRaw = elemDiv(attackRaw, t)
    const cap = reactionAttackCap(w, elemSide)
    if (attackRaw > cap) attackRaw = cap
    attackPortion = attackRaw
  }

  // 5) 抗性只作用于反应伤害
  if (reactionKey) {
    const resElement = input.reactionElement ?? def.dominantElement()
    const resist = resElement ? def.resistOf(resElement) : 0n
    res.reaction = reactionKey
    res.resistAppliedPermille = PERMILLE - resist
    const spec = REACTIONS[reactionKey]
    res.dispelShield = spec.dispelShield
    res.statusDurationMs = spec.statusDurationMs
    const total = res.reactionElemPortion + attackPortion
    reactionDmg = mulDiv(total, PERMILLE - resist, PERMILLE)
    attackPortion = mulDiv(attackPortion, PERMILLE - resist, PERMILLE)
  }

  // 6) 暴击作用于直接伤害 + 元素层数伤害（不作用于反应伤害，否则抗性会被绕开）
  let crit = false
  let critPart = direct + elementDmg
  if (att.critPermille > 0n && BigInt(input.roll) >= PERMILLE - att.critPermille) {
    crit = true
    critPart = mulDiv(critPart, att.critMultiplierPermille, PERMILLE)
  }
  direct = critPart

  const total = direct + reactionDmg

  // 7) 落伤害：先扣护盾（可被反应驱散），再扣血
  if (total > 0n) {
    if (res.dispelShield) def.shield = 0n
    let remaining = total
    if (def.shield > 0n) {
      const absorbed = minBig(def.shield, remaining)
      def.shield -= absorbed
      remaining -= absorbed
      if (def.shield === 0n) res.shieldBroken = true
    }
    if (remaining > 0n) {
      def.hp -= remaining
      if (def.hp <= 0n) {
        def.hp = 0n
        res.killed = true
      }
    }
  }

  res.directDamage = direct
  res.elementDamage = elementDmg
  res.reactionDamage = reactionDmg
  res.reactionAttackPortion = attackPortion
  res.totalDamage = total
  res.crit = crit
  return res
}

/** 乘以"阶数"这种小整数，避免走 mulDiv 引入除法截断差异 */
function elemDiv(v: bigint, tier: bigint): bigint {
  if (tier === 0n) return 0n
  return v * tier
}

/**
 * 结算后施加元素层数。
 * ⚠️ 顺序很重要：必须先结算伤害/反应，再施加新层数，
 * 否则"本次施加的元素"会立刻成为反应链的已附着元素，导致自我触发。
 */
export function applyElementStacks(
  att: Attacker,
  def: Defender,
  input: HitInput,
): void {
  const add = input.applyStacks ?? 0n
  if (!input.skillElement || add <= 0n) return
  def.applyElement(input.skillElement, add, att.elementCap)
}

/** 反应伤害中攻击力贡献的占比（千分比） */
export function reactionAttackRatio(r: HitResult): bigint {
  if (r.reactionDamage <= 0n) return 0n
  return mulDiv(r.reactionAttackPortion, PERMILLE, r.reactionDamage)
}

/** 反通胀不变量的运行时断言。开发期调用，生产可跳过 */
export function assertAntiInflation(r: HitResult): boolean {
  return reactionAttackRatio(r) <= MAX_REACTION_ATTACK_WEIGHT
}

/** 便捷：用 BattleRng 生成 roll 并结算 */
export function hitWithRng(
  att: Attacker,
  def: Defender,
  input: Omit<HitInput, 'roll'>,
  rng: BattleRng,
): HitResult {
  return resolveHit(att, def, { ...input, roll: rng.roll() })
}
