/**
 * R40 Wiki 评分权重（rating_weights）展示文案。
 *
 * 与 R33 敌人机制助手同构：全部入参为 unknown，公开 config 的
 * rating_weights 即使缺字段或带脏值，表格也只能显示破折号，
 * 绝不把 NaN / undefined / [object Object] 泄漏给玩家。
 * 这里只做展示格式化，不参与任何评分计算或结算。
 */

import { MECHANISM_EMPTY } from '@/enemyMechanicsText'

// 同源复用 R33 的破折号（—），本模块不写第二份字面量。
export { MECHANISM_EMPTY }

/**
 * 权威五键固定顺序：逐字对齐 power.go RatingWeights 的 json tag
 * 与 types.gen.ts 的 RatingWeights；顺序不得按 Object.entries 漂移。
 */
export const RATING_KEYS = [
  'element_coverage',
  'reaction_coverage',
  'mastery_done',
  'equipment_synergy',
  'mechanic_depth',
] as const

const RATING_LABELS = new Map<string, string>([
  ['element_coverage', '元素覆盖'],
  ['reaction_coverage', '反应链'],
  ['mastery_done', '专精投入'],
  ['equipment_synergy', '装备契合'],
  ['mechanic_depth', '机制深度'],
])

/** 五键 → 中文；其它合法非空字符串逐字「（未知维度#key）」；空/全空白/非字符串一律 —。 */
export function ratingLabel(k: unknown): string {
  if (typeof k !== 'string' || k.trim() === '') return MECHANISM_EMPTY
  return RATING_LABELS.get(k) ?? `（未知维度#${k}）`
}

/** 只接受有限非负整数（含 0），直显原值；绝不换算 %/‰，绝不追加单位。 */
export function ratingValue(v: unknown): string {
  if (typeof v !== 'number' || !Number.isInteger(v) || v < 0) return MECHANISM_EMPTY
  return String(v)
}

/**
 * AI 结论：RATING_KEYS 五键中 ratingValue 非 — 的键数，模板逐字「权重 N/5 项齐备」。
 * 非法入参（null/数组/标量）按 0 项齐备处理；输出确定性，无占位符残留。
 */
export function buildRatingMechanicsAi(weights: unknown): string {
  let present = 0
  if (weights !== null && typeof weights === 'object' && !Array.isArray(weights)) {
    const record = weights as Record<string, unknown>
    for (const key of RATING_KEYS) {
      if (ratingValue(record[key]) !== MECHANISM_EMPTY) present += 1
    }
  }
  return `权重 ${present}/5 项齐备`
}

export const RATING_AI_BASIS =
  '依据：公开 /api/v1/config 的 rating_weights，与服务端 RatingWeights / DefaultRatingWeights 同源；仅展示配置基值'

export const RATING_AI_HONESTY =
  '评分权重是配置基值，不是权威结算；实际收益与边界以游戏内为准；权威回放以 replay_hash 为准'

export const RATING_AI_CTA = '查看构筑评分 →'
