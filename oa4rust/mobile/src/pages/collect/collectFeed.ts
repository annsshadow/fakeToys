// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端收藏列表加载（R111）。
 *
 * 旧实现 `catch { rows.value = [] }` 把请求失败折叠成「暂无收藏」：用户分不清
 * 「确实没有收藏」还是「加载失败」，且 onShow 刷新失败会清空正在看的列表。
 * 这里把过滤/映射与失败保留旧数据都变成可测函数。
 */

export interface CollectRow {
  id: string
  title: string
  url: string
}

export type CollectLoadOutcome = { ok: true; rows: CollectRow[] } | { ok: false; rows: CollectRow[]; error: string }

export interface CollectListResponse {
  data?: unknown
}

/** 页面展示用的错误文案：非 Error 也给非空提示，不许静默。 */
export function describeCollectError(error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `收藏加载失败：${detail}`
}

function titleOf(row: Record<string, unknown>): string {
  const v = row.title
  return typeof v === 'string' && v ? v : '（无标题）'
}

function urlOf(row: Record<string, unknown>): string {
  const v = row.url
  return typeof v === 'string' ? v : ''
}

/**
 * 后端 collect_list 返回全员收藏（与桌面 CollectApp 同口径），按本人过滤；
 * 无 id 的行无法删除也无法去重，直接丢弃。
 */
export function mapCollectRows(data: unknown, personId: unknown): CollectRow[] {
  if (!Array.isArray(data)) return []
  return (data as Record<string, unknown>[])
    .filter((r) => personId != null && (r.personId === personId || r.person_id === personId))
    .map((r) => ({
      id: typeof r.id === 'string' ? r.id : '',
      title: titleOf(r),
      url: urlOf(r),
    }))
    .filter((r) => r.id)
}

export async function loadCollectRows(
  fetchAll: () => Promise<CollectListResponse | undefined>,
  personId: unknown,
  previousRows: readonly CollectRow[] = [],
): Promise<CollectLoadOutcome> {
  try {
    const resp = await fetchAll()
    return { ok: true, rows: mapCollectRows(resp?.data, personId) }
  } catch (error) {
    // 刷新失败保留旧收藏：不能让一次网络抖动把用户正在看的列表抹成「暂无收藏」。
    return { ok: false, rows: [...previousRows], error: describeCollectError(error) }
  }
}
