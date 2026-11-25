// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R42：Wiki 关卡表所属章节列与章节×关卡交叉引用的确定性展示格式化，纯函数、无副作用。 */
import { MECHANISM_EMPTY } from './levelMechanicsText'

export { MECHANISM_EMPTY }

function positiveInt(v: unknown): number | null {
  return typeof v === 'number' && Number.isInteger(v) && v > 0 ? v : null
}

/**
 * chapter 非合法正整数 → 空态破折号；合法时在 chapters 中按 id 严格相等查名字。
 * 只接受 id 为合法正整数且 name 为非空字符串的章节条目，其余安全回退未知章节。
 */
export function levelChapterText(chapter: unknown, chapters: unknown): string {
  const id = positiveInt(chapter)
  if (id === null) return MECHANISM_EMPTY
  if (Array.isArray(chapters)) {
    for (const item of chapters) {
      if (item === null || typeof item !== 'object') continue
      const row = item as Record<string, unknown>
      if (positiveInt(row.id) !== id) continue
      if (typeof row.name !== 'string' || row.name === '') continue
      return row.name
    }
  }
  return `未知章节#${id}`
}

export const LEVEL_CHAPTER_AI_BASIS = '依据：公开 /api/v1/config 的 levels.chapter 与 chapters（与服务端 LevelSummary.chapter / domain.ChapterOf 同源）'
export const LEVEL_CHAPTER_AI_HONESTY = '章节是配置归属，不是地形使用 terrain_used，也不参与 replay_hash；实际战斗数据以结算为准，差异以公开 /api/v1/config 为准'
export const LEVEL_CHAPTER_AI_CTA = '查看章节一览 →'

/**
 * 逐字模板 共N关，覆盖M章。N 为 levels 数组条数（含脏条目）；
 * M 为 levels 中合法正整数 chapter 去重数，与 chapters 参数无关（仅保留 API 对称）。
 */
export function buildLevelChapterAi(levels: unknown, chapters: unknown): string {
  void chapters
  const rows: readonly unknown[] = Array.isArray(levels) ? levels : []
  const ids = new Set<number>()
  for (const row of rows) {
    if (row === null || typeof row !== 'object') continue
    const id = positiveInt((row as Record<string, unknown>).chapter)
    if (id !== null) ids.add(id)
  }
  return `共${rows.length}关，覆盖${ids.size}章`
}
