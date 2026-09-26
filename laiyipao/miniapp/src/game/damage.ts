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

/**
 * 结构性反通胀保证：攻击力侧在反应伤害里的上限。
 *
 * 不变式（I-1 的核心）：反应伤害中攻击力贡献的占比 ≤ w。
 *
 *   A·M / (E + A·M) ≤ w   ⟺   A·M ≤ wE/(1-w)   ⟺   A ≤ wE/((1-w)·M)
 *
 * 三个参数：
 *   w            反应的攻击力权重（千分比，≤ MAX_REACTION_ATTACK_WEIGHT=300）
 *   elemPortion  元素侧的绝对伤害（与养成完全无关）
 *   multPermille 反应倍率（千分比，1000 = 无加成）
 *
 * ⚠️ multPermille 必须参与上限计算。少了它，倍率就成了绕过反通胀保证的后门：
 * 专精树的 reaction_mult 节点、装备契合度、防线装置「特斯拉栅格」全部只写不读，
 * 正是因为读不到地方 —— 而一旦读进来又不管上限，玩家堆一个节点就能让
 * 反应伤害里的攻击力占比突破 30%，整套设计的根基被破坏。
 *
 * multPermille = 1000（无加成）时本函数与旧式 `wE/(1-w)` 逐位相同，
 * 所以默认配置下既有契约向量与平衡数据都不受影响。
 */
function reactionAttackCap(
  w: bigint,
  elemPortion: bigint,
  multPermille: bigint,
): bigint {
  if (w >= PERMILLE) return elemPortion * PERMILLE
  // multPermille <= 0 时不加成，等价于零倍率（攻击侧完全无效）
  if (multPermille <= 0n) return 0n
  const base = mulDiv(w, elemPortion, PERMILLE - w)
  return mulDiv(base, PERMILLE, multPermille)
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
    //
    // ⚠️ 上限要**除以 reactionMultPermille**，这不是随手加的除法。
    //
    // I-1 的反通胀不变式是：反应伤害中攻击力贡献的占比 ≤ w。
    //   占比 = A / (E + A) ≤ w   ⟺   A(1-w) ≤ wE   ⟺   A ≤ wE/(1-w)
    // 原来 A = attackPortion（未乘 M），所以 cap = wE/(1-w) 成立。
    //
    // 现在攻击侧要乘上 M（reactionMultPermille/1000，专精树/装备/特斯拉栅格
    // 给的加成），于是要约束的是 **A·M**：
    //   A·M/(E + A·M) ≤ w   ⟺   A·M ≤ wE/(1-w)   ⟺   **A ≤ wE/((1-w)·M)**
    //
    // 直接把 M 乘进伤害而不收紧 cap，会让占比随 M 上升而突破 30% ——
    // 那等于"堆一个专精节点就绕过了反通胀保证"，是整套设计的根基被破坏。
    //
    // M = 1000‰（无加成）时 cap 与原式**逐位相同**：
    //   mulDiv(mulDiv(w,E,1000-w), 1000, 1000) = wE/(1000-w) ✓
    // 所以默认配置下所有既有契约向量与平衡数据都不受影响。
    const cap = reactionAttackCap(w, elemSide, att.reactionMultPermille)
    let attackRaw = mulDiv(d, k, PERMILLE)
    attackRaw = elemDiv(attackRaw, t)
    if (attackRaw > cap) attackRaw = cap
    // 收紧后的上限再乘 M —— 上限约束的是「乘完 M 之后的攻击力贡献」
    attackPortion = mulDiv(attackRaw, att.reactionMultPermille, PERMILLE)
  }

  // 5) 抗性只作用于反应伤害
  if (reactionKey) {
    // ⚠️ 这里用「falsy 即未指定」而不是 `??`。
    //
    // Go 侧的写法是 `if resElement == "" { resElement = dominantElement(def) }`，
    // 也就是**空串会回退到守方主元素**。而 `??` 只对 null/undefined 回退，
    // 传 `''` 时会得到 `resistOf('')` = 0（视为无抗性）。
    //
    // 两端对**同一个空值**给出不同抗性 → 反应伤害不同 → replayHash 不同
    // → I-6 把正常对局判成伪造。
    //
    // 字段类型是 `Element | undefined`，所以 `''` 按类型不该出现；
    // 但"不该出现"和"出现时两端行为一致"是两件事：契约向量经 JSON 往返、
    // DB 里的关卡行被手改、灰度中的新客户端，都可能送来一个类型上
    // "不可能"的值。README 约束 4 说的正是这一类「同名不同义」隐患。
    const resElement = input.reactionElement || def.dominantElement()
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
