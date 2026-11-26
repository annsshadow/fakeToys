// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R35 Wiki 宝石图鉴纯展示 helper。
 *
 * 权威数据源是 Wiki 实际请求的公开 /api/v1/config（gems / gem_qualities），
 * 与服务端 SeedGem / GemQualities 同源。这里只做格式化展示，
 * 不碰 roll / 结算 / replay_hash。所有入参 unknown，非法一律归空态破折号。
 */

/** 统一空值：0 / 空 / 非数字 / 非有限 / 非法结构一律显示它。 */
export const MECHANISM_EMPTY = '—'

/** 权威 attr → 中文名。未知但非空字符串原样返回，不伪造。 */
const ATTR_LABEL: Record<string, string> = {
  attack: '攻击',
  element_coef: '元素系数',
  crit: '暴击率',
  reaction_mult: '反应倍率',
  element_cap: '元素层数上限',
  heat_cap: '热量上限',
  armor: '护甲',
  skill_damage: '技能伤害',
}

export function attrText(v: unknown): string {
  if (typeof v !== 'string' || v === '') return MECHANISM_EMPTY
  return ATTR_LABEL[v] ?? v
}

interface QualityLike {
  name: unknown
  affix_count: unknown
  mult: unknown
}

function isQualityLike(v: unknown): v is QualityLike {
  return typeof v === 'object' && v !== null && !Array.isArray(v)
}

function positiveInt(v: unknown): number | null {
  return typeof v === 'number' && Number.isInteger(v) && v > 0 ? v : null
}

/**
 * 品质展示：name · N 条 · M‰。
 * mult 是千分比系数（1000 → 1000‰），绝不换算成百分比。
 */
export function qualityText(v: unknown): string {
  if (!isQualityLike(v)) return MECHANISM_EMPTY
  if (typeof v.name !== 'string' || v.name === '') return MECHANISM_EMPTY
  const count = positiveInt(v.affix_count)
  const mult = positiveInt(v.mult)
  if (count === null || mult === null) return MECHANISM_EMPTY
  return `${v.name} · ${count} 条 · ${mult}‰`
}

/** 词条随机说明（诚实边界硬门禁，逐字）。 */
export const GEM_AFFIX_NOTE = '词条随机，具体数值以游戏内为准'

/** AI 依据行（逐字）。 */
export const GEM_AI_BASIS = '依据：公开 /api/v1/config 的 gems 与 gem_qualities（与服务端 SeedGem / GemQualities 同源）；词条随机，具体数值以游戏内为准'

/** AI 诚实红线（逐字）。 */
export const GEM_AI_HONESTY = '辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准'

/** AI CTA（逐字）。 */
export const GEM_AI_CTA = '查看宝石图鉴 →'

/** 由公开配置确定性聚合：种类数 / 档数 / 词条条数范围。 */
export function buildGemMechanicsAi(gems: readonly unknown[], qualities: readonly unknown[]): string {
  const counts: number[] = []
  for (const q of qualities) {
    if (!isQualityLike(q)) continue
    const n = positiveInt(q.affix_count)
    if (n !== null) counts.push(n)
  }
  let range: string
  if (counts.length === 0) {
    range = MECHANISM_EMPTY
  } else {
    const min = Math.min(...counts)
    const max = Math.max(...counts)
    range = min === max ? String(min) : `${min}–${max}`
  }
  return `共 ${gems.length} 种宝石，${qualities.length} 档品质，词条 ${range} 条`
}
