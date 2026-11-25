// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R41：Wiki 章节元数据的确定性展示格式化，纯函数、无副作用。 */
import { MECHANISM_EMPTY } from './levelMechanicsText'

export { MECHANISM_EMPTY }

function positiveInt(v: unknown): number | null {
  return typeof v === 'number' && Number.isInteger(v) && v > 0 ? v : null
}

/** 合法正整数区间用 en-dash（U+2013），其余一律空态。 */
export function rangeText(start: unknown, end: unknown): string {
  const a = positiveInt(start)
  const b = positiveInt(end)
  if (a === null || b === null) return MECHANISM_EMPTY
  return `${a}–${b}关`
}

const TERRAIN_LABELS: Record<string, string> = {
  oil_drum: '油桶',
  collapse_wall: '塌墙',
  charge_tower: '蓄能塔',
  tidal_gate: '潮汐闸门',
  rotor_vane: '旋翼风扇',
}

/** 五种已知地形映射同值中文名，未知合法 kind 原样返回，非法值空态。 */
export function terrainKindText(kind: unknown): string {
  if (typeof kind !== 'string' || kind === '') return MECHANISM_EMPTY
  return TERRAIN_LABELS[kind] ?? kind
}

/** 首领敌人 id 是正整数，显示为 #N；布尔值一律空态。 */
export function bossText(bossEnemyId: unknown): string {
  const n = positiveInt(bossEnemyId)
  return n === null ? MECHANISM_EMPTY : `#${n}`
}

export interface ChapterMechanicsRow {
  id?: unknown
  name?: unknown
  start_level?: unknown
  end_level?: unknown
  terrain_kind?: unknown
  boss_enemy_id?: unknown
}

export const CHAPTER_AI_BASIS = '依据：公开 /api/v1/config 的 chapters（与服务端 domain.AllChapters 同源）；章节 terrain_kind 为配置基值，实际地形触发以结算 terrain_used 为准'
export const CHAPTER_AI_HONESTY = '章节 terrain_kind 是配置基值，与关卡 terrain_used（参与 replay_hash）不同概念，不得混同权威战斗数据；示例值不硬编码，差异以公开 /api/v1/config 为准'
export const CHAPTER_AI_CTA = '查看章节一览 →'

/** 由公开 cfg.chapters 确定性聚合；空数组返回诚实空态。 */
export function buildChapterMechanicsAi(chapters: unknown): string {
  const rows: readonly ChapterMechanicsRow[] = Array.isArray(chapters) ? chapters as readonly ChapterMechanicsRow[] : []
  const kinds = new Set<string>()
  let boss = 0
  for (const row of rows) {
    if (row === null || typeof row !== 'object') continue
    if (positiveInt(row.boss_enemy_id) !== null) boss++
    const kind = row.terrain_kind
    if (typeof kind === 'string' && kind !== '') kinds.add(kind)
  }
  return `共 ${rows.length} 章：含首领 ${boss} 章，地形 ${kinds.size} 种`
}
