// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R34：Wiki 关卡机制字段的确定性展示格式化，纯函数、无副作用。 */

export const MECHANISM_EMPTY = '—'

function positiveInt(v: unknown): number | null {
  return typeof v === 'number' && Number.isInteger(v) && v > 0 ? v : null
}

/** 仅严格 true 视为首领，其余一律空态（避免把无标记误读为有此机制）。 */
export function bossText(v: unknown): string {
  return v === true ? '是' : MECHANISM_EMPTY
}

const TERRAIN_LABELS: Record<string, string> = {
  oil_drum: '油桶',
  collapse_wall: '塌墙',
  charge_tower: '蓄能塔',
}

/** 按 kind 去重计数；已知三类映射中文，其余合法 kind 原样；按首次出现顺序「、」连接。 */
export function terrainText(terrain: unknown): string {
  if (!Array.isArray(terrain)) return MECHANISM_EMPTY
  const order: string[] = []
  const counts = new Map<string, number>()
  for (const item of terrain) {
    if (item === null || typeof item !== 'object') continue
    const kind = (item as Record<string, unknown>).kind
    if (typeof kind !== 'string' || kind === '') continue
    if (!counts.has(kind)) {
      counts.set(kind, 0)
      order.push(kind)
    }
    counts.set(kind, (counts.get(kind) ?? 0) + 1)
  }
  if (order.length === 0) return MECHANISM_EMPTY
  return order.map((kind) => `${TERRAIN_LABELS[kind] ?? kind}×${counts.get(kind)}`).join('、')
}

/** 千分号 ‰，绝不冒充百分比。 */
export function armorPermilleText(v: unknown): string {
  const n = positiveInt(v)
  return n === null ? MECHANISM_EMPTY : `${n}‰`
}

/** 正整数原值，不加千分位与推断词。 */
export function maxScoreText(v: unknown): string {
  const n = positiveInt(v)
  return n === null ? MECHANISM_EMPTY : String(n)
}

export interface LevelMechanicsRow {
  is_boss?: unknown
  terrain?: unknown
  armor_permille?: unknown
  max_score?: unknown
}

export const LEVEL_AI_CTA = '查看关卡一览 →'

/** 由公开 cfg.levels 确定性聚合；空数组返回诚实空态。 */
export function buildLevelMechanicsAi(levels: readonly LevelMechanicsRow[]): string {
  const total = levels.length
  let boss = 0
  let withTerrain = 0
  const kinds = new Set<string>()
  for (const lv of levels) {
    if (lv.is_boss === true) boss++
    if (terrainText(lv.terrain) !== MECHANISM_EMPTY) withTerrain++
    if (Array.isArray(lv.terrain)) {
      for (const item of lv.terrain) {
        if (item === null || typeof item !== 'object') continue
        const kind = (item as Record<string, unknown>).kind
        if (typeof kind === 'string' && kind !== '') kinds.add(kind)
      }
    }
  }
  return `共 ${total} 关：首领关 ${boss} 个，含地形关 ${withTerrain} 个，地形种类 ${kinds.size} 种`
}
