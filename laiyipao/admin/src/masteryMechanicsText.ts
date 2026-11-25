// R37：Wiki 专精树节点字段的**纯显示辅助**。
// 入参一律 unknown；脏值/0/负数/小数统一显示 MECHANISM_EMPTY，
// 绝不把 NaN / undefined / [object Object] 泄漏给玩家。
// 口径只对齐 power.go 的 EvaluateMastery 配置基值，不参与任何战斗结算。

export { MECHANISM_EMPTY } from '@/enemyMechanicsText'
import { MECHANISM_EMPTY } from '@/enemyMechanicsText'

const KIND_LABELS: Record<string, string> = {
  element_cap: '元素层数上限',
  reaction_mult: '反应倍率',
  heat_cap: '热量上限',
  extra_slot: '额外插槽',
  skill_damage: '技能伤害',
  crit: '暴击',
  armor: '护甲',
  mechanic: '机制改造',
}

function isPositiveInteger(v: unknown): v is number {
  return typeof v === 'number' && Number.isInteger(v) && v > 0
}

/** 权威八种效果名；未知合法字符串原样显示（不伪造中文名）。 */
export function kindText(kind: unknown): string {
  if (typeof kind !== 'string' || kind.length === 0) return MECHANISM_EMPTY
  return KIND_LABELS[kind] ?? kind
}

/**
 * valueText 按 power.go 语义展示配置基值：
 * - heat_cap 的配置值已经是服务端 v*10 的结果，直接 +N‰，禁止二次乘 10；
 * - extra_slot 恒 +1 槽（配置 value 不是槽数）；
 * - element_cap 是绝对层数；
 * - 其余权威 kind 为千分比；
 * - 未知合法 kind 原值 + 诚实单位说明，绝不冒充百分比。
 */
export function valueText(kind: unknown, value: unknown): string {
  if (!isPositiveInteger(value)) return MECHANISM_EMPTY
  const label = kindText(kind)
  if (label === MECHANISM_EMPTY) return MECHANISM_EMPTY
  if (kind === 'heat_cap') return `+${value}‰`
  if (kind === 'extra_slot') return '+1 槽'
  if (kind === 'element_cap') return `+${value}层`
  if (kind === 'reaction_mult' || kind === 'skill_damage' || kind === 'crit' || kind === 'armor' || kind === 'mechanic') {
    return `+${value}‰`
  }
  return `${value}（单位以游戏内结算为准）`
}

/** 空族 + 0 层 = 无前置；其余组合必须族非空且层为有限正整数。 */
export function prereqText(prereq_family: unknown, prereq_layer: unknown): string {
  const family = typeof prereq_family === 'string' && prereq_family.length > 0 ? prereq_family : null
  if (family === null && prereq_layer === 0) return '无前置'
  if (family !== null && isPositiveInteger(prereq_layer)) return `前置：${family} 第${prereq_layer}层`
  return MECHANISM_EMPTY
}

export interface MasteryMechanicsFamily {
  family: unknown
  name?: unknown
  nodes: readonly {
    name: unknown
    kind?: unknown
    value?: unknown
    prereq_family?: unknown
    prereq_layer?: unknown
  }[]
}

interface MasteryMechanicsNode {
  name: unknown
  kind?: unknown
  prereq_family?: unknown
  prereq_layer?: unknown
}

function asFamily(f: unknown): MasteryMechanicsFamily | null {
  if (typeof f !== 'object' || f === null || Array.isArray(f)) return null
  const family = (f as MasteryMechanicsFamily).family
  return typeof family === 'string' && family.length > 0 ? (f as MasteryMechanicsFamily) : null
}

function asNode(n: unknown): MasteryMechanicsNode | null {
  if (typeof n !== 'object' || n === null || Array.isArray(n)) return null
  const name = (n as MasteryMechanicsNode).name
  return typeof name === 'string' && name.length > 0 ? (n as MasteryMechanicsNode) : null
}

export const MASTERY_AI_BASIS = '依据：公开 /api/v1/config 的 mastery_families（与服务端 MasteryFamily / MasteryNode 同源）；效果按 power.go 口径展示，实际以引擎结算为准'
export const MASTERY_AI_HONESTY = '辅助展示配置基值，非权威战斗数据；实际效果以引擎结算为准，权威回放以 replay_hash 为准'
export const MASTERY_AI_CTA = '查看专精树 →'

/** 由公开 cfg.mastery_families 确定性聚合，不依赖焦点行。 */
export function buildMasteryMechanicsAi(families: readonly MasteryMechanicsFamily[]): string {
  let countFamilies = 0
  let nodes = 0
  let gated = 0
  let slots = 0
  for (const raw of families) {
    const family = asFamily(raw)
    if (!family) continue
    countFamilies += 1
    for (const rawNode of family.nodes ?? []) {
      const node = asNode(rawNode)
      if (!node) continue
      nodes += 1
      if (!prereqText(node.prereq_family, node.prereq_layer).startsWith('无前置')) gated += 1
      if (node.kind === 'extra_slot') slots += 1
    }
  }
  return `共 ${countFamilies} 系：${nodes} 个节点，含前置 ${gated} 个，额外插槽 ${slots} 个`
}
