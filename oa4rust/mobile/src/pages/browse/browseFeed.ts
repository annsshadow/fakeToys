// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端只读浏览列表加载（R112）。
 *
 * 旧实现 `catch { rows.value = [] }` 把请求失败折叠成「暂无数据」：用户分不清
 * 「这个页签确实没有内容」还是「加载失败」，onShow 刷新失败还会清空正在看的列表。
 * 页签切换属于用户主动换数据源，此时不清空旧错误但也不沿用旧页签的行。
 */

export type BrowseRow = Record<string, unknown>

export type BrowseLoadOutcome = { ok: true; rows: BrowseRow[] } | { ok: false; rows: BrowseRow[]; error: string }

export interface BrowseListResponse {
  data?: unknown
}

/** 页面展示用的错误文案：非 Error 也给非空提示，不许静默。 */
export function describeBrowseError(error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `加载失败：${detail}`
}

export async function loadBrowseRows(
  fetchList: () => Promise<BrowseListResponse | undefined>,
  previousRows: readonly BrowseRow[] = [],
): Promise<BrowseLoadOutcome> {
  try {
    const resp = await fetchList()
    const rows = Array.isArray(resp?.data) ? (resp.data as BrowseRow[]) : []
    return { ok: true, rows }
  } catch (error) {
    // 同页签刷新失败保留旧数据；失败文案保证可见，不当成空列表。
    return { ok: false, rows: [...previousRows], error: describeBrowseError(error) }
  }
}
