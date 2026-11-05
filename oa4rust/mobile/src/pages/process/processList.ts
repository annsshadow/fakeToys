// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端流程列表分页（R109）。
 *
 * 旧实现三个页签都固定 `xxxList(1, 50)`，且 `catch { rows.value = [] }`：
 * ① 超过 50 条的待办/已办/我发起既加载不到，也没有「还有更多」提示；
 * ② 请求失败被折叠成「暂无待办 / 暂无已办」，用户看不出是网络问题还是真的空。
 * 这里把页码/大小、是否还有下一页、按 id 去重合并、失败文案变成可测函数。
 */

export const PROCESS_PAGE_SIZE = 50

export type ProcessTab = 'pending' | 'done' | 'started'

export interface ProcessPage<T> {
  page: number
  size: number
  rows: T[]
  hasMore: boolean
}

export type ProcessLoader = (tab: ProcessTab, page: number, size: number) => Promise<{ data?: unknown } | undefined>

export async function loadProcessPage<T>(
  load: ProcessLoader,
  tab: ProcessTab,
  page: number,
  size = PROCESS_PAGE_SIZE,
): Promise<ProcessPage<T>> {
  const safePage = Math.max(1, Math.floor(page))
  const safeSize = Math.max(1, Math.floor(size))
  const resp = await load(tab, safePage, safeSize)
  const data = resp?.data
  const rows = Array.isArray(data) ? (data as T[]) : []
  return { page: safePage, size: safeSize, rows, hasMore: rows.length >= safeSize }
}

/**
 * 追加下一页并按 id 去重。分页基于 OFFSET，办理任务会让相邻页重叠，
 * 去重是必需的；无 id 的行无法判重，原样保留。
 */
export function mergeProcessRows<T extends { id?: unknown }>(current: readonly T[], incoming: readonly T[]): T[] {
  const seen = new Set<string>()
  for (const row of current) {
    const id = row?.id
    if (typeof id === 'string' && id) seen.add(id)
  }
  const appended: T[] = []
  for (const row of incoming) {
    const id = row?.id
    if (typeof id === 'string' && id) {
      if (seen.has(id)) continue
      seen.add(id)
    }
    appended.push(row)
  }
  return [...current, ...appended]
}

/** 页面展示用的错误文案：非 Error 也给非空提示，不许静默。 */
export function describeProcessError(prefix: string, error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `${prefix}：${detail}`
}
