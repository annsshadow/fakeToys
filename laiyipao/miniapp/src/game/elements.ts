/**
 * 元素与反应链（I-1 的数据定义）。
 *
 * ⚠️ 本文件与 server/internal/domain/elements.go 必须逐行等价。
 * 元素顺序（ELEMENT_ORDER）影响覆盖率计算与哈希，改动必须同步两处。
 */

export type Element = 'fire' | 'ice' | 'lightning' | 'corrosion' | 'kinetic'

/** 固定顺序，覆盖率计算与哈希都依赖它 */
export const ELEMENT_ORDER: readonly Element[] = [
  'fire',
  'ice',
  'lightning',
  'corrosion',
  'kinetic',
] as const

export const ELEMENT_NAME: Record<Element, string> = {
  fire: '焰',
  ice: '冰',
  lightning: '电',
  corrosion: '毒',
  kinetic: '动能',
}

/** 元素主题色，与后台 CSS 变量保持一致 */
export const ELEMENT_COLOR: Record<Element, string> = {
  fire: '#ff6b35',
  ice: '#58a6ff',
  lightning: '#ffd33d',
  corrosion: '#7ee787',
  kinetic: '#c9a7ff',
}

export type ReactionKey =
  | 'steam_burst'
  | 'overheat'
  | 'burn_cloud'
  | 'superconduct'
  | 'flash_freeze'
  | 'corrosion_spread'
  | 'armor_break'

/** 固定顺序 */
export const REACTION_ORDER: readonly ReactionKey[] = [
  'steam_burst',
  'overheat',
  'burn_cloud',
  'superconduct',
  'flash_freeze',
  'corrosion_spread',
  'armor_break',
] as const

export interface ReactionSpec {
  key: ReactionKey
  name: string
  /** 反应基础系数（千分比） */
  baseCoef: number
  /** 反应伤害中由攻击力贡献的比例（千分比）—— 硬性 ≤ 300 */
  attackWeightPct: number
  /** 附加状态时长（毫秒），0 表示无 */
  statusDurationMs: number
  /** 溅射半径（像素），0 表示单体 */
  aoeRadius: number
  /** 是否驱散护盾 */
  dispelShield: boolean
  /** 对目标的受击伤害放大（千分比），0 表示无 */
  amplifyPct: number
  /** 表现层用：简短说明 */
  descr: string
}

export const REACTIONS: Record<ReactionKey, ReactionSpec> = {
  steam_burst: {
    key: 'steam_burst',
    name: '蒸汽爆发',
    baseCoef: 60,
    attackWeightPct: 300,
    statusDurationMs: 0,
    aoeRadius: 120,
    dispelShield: true,
    amplifyPct: 0,
    descr: '范围伤害并驱散护盾',
  },
  overheat: {
    key: 'overheat',
    name: '过热',
    baseCoef: 55,
    attackWeightPct: 300,
    statusDurationMs: 1500,
    aoeRadius: 90,
    dispelShield: false,
    amplifyPct: 0,
    descr: '爆炸并眩晕',
  },
  burn_cloud: {
    key: 'burn_cloud',
    name: '燃烧云',
    baseCoef: 40,
    attackWeightPct: 250,
    statusDurationMs: 0,
    aoeRadius: 100,
    dispelShield: false,
    amplifyPct: 0,
    descr: '生成持续火区',
  },
  superconduct: {
    key: 'superconduct',
    name: '超导',
    baseCoef: 50,
    attackWeightPct: 250,
    statusDurationMs: 4000,
    aoeRadius: 0,
    dispelShield: false,
    amplifyPct: 600,
    descr: '受击伤害 +60%',
  },
  flash_freeze: {
    key: 'flash_freeze',
    name: '急速冻结',
    baseCoef: 45,
    attackWeightPct: 250,
    statusDurationMs: 2000,
    aoeRadius: 0,
    dispelShield: false,
    amplifyPct: 300,
    descr: '冻结并提高受击伤害',
  },
  corrosion_spread: {
    key: 'corrosion_spread',
    name: '腐蚀扩散',
    baseCoef: 35,
    attackWeightPct: 200,
    statusDurationMs: 0,
    aoeRadius: 150,
    dispelShield: false,
    amplifyPct: 0,
    descr: '把元素层数传播给周围敌人',
  },
  armor_break: {
    key: 'armor_break',
    name: '破甲击退',
    baseCoef: 30,
    attackWeightPct: 300,
    statusDurationMs: 0,
    aoeRadius: 0,
    dispelShield: false,
    amplifyPct: 0,
    descr: '击退并削减护甲',
  },
}

/**
 * 反应查表：已附着元素 A + 新施加元素 B → 反应。
 *
 * 反应不可交换：谁先谁后决定结果。动能不能作为"已附着元素"
 * 触发新反应（表中 kinetic 的键不含空），它只能后手打出破甲。
 */
const REACTION_TABLE: Record<Element, Partial<Record<Element, ReactionKey>>> = {
  fire: {
    ice: 'steam_burst',
    lightning: 'overheat',
    corrosion: 'burn_cloud',
    kinetic: 'armor_break',
  },
  ice: {
    fire: 'steam_burst',
    lightning: 'superconduct',
    corrosion: 'flash_freeze',
    kinetic: 'armor_break',
  },
  lightning: {
    fire: 'overheat',
    ice: 'superconduct',
    corrosion: 'corrosion_spread',
    kinetic: 'armor_break',
  },
  corrosion: {
    fire: 'burn_cloud',
    ice: 'flash_freeze',
    lightning: 'corrosion_spread',
    kinetic: 'armor_break',
  },
  kinetic: {
    fire: 'armor_break',
    ice: 'armor_break',
    lightning: 'armor_break',
    corrosion: 'armor_break',
  },
}

/** 返回"已附着元素 + 新施加元素"触发的反应；无反应返回 null */
export function lookupReaction(
  existing: Element | '' | undefined,
  incoming: Element | '' | undefined,
): ReactionKey | null {
  if (!existing || !incoming || existing === incoming) return null
  const byIncoming = REACTION_TABLE[incoming]
  if (!byIncoming) return null
  return byIncoming[existing] ?? null
}

/** 该元素搭配能触发的全部反应（用于构筑评分） */
export function reachableReactions(elements: Set<Element>): Set<ReactionKey> {
  const out = new Set<ReactionKey>()
  for (const existing of ELEMENT_ORDER) {
    if (!elements.has(existing)) continue
    for (const incoming of ELEMENT_ORDER) {
      // incoming 也必须在搭配里：反应需要先后两种元素**都能打出**，
      // 只带一种元素的构筑触发不了任何反应。不限定 incoming 会让
      // 单元素也被算出反应，构筑评分虚高。
      if (!elements.has(incoming)) continue
      const k = lookupReaction(existing, incoming)
      if (k) out.add(k)
    }
  }
  return out
}

export function isElement(s: string): s is Element {
  return (ELEMENT_ORDER as readonly string[]).includes(s)
}
