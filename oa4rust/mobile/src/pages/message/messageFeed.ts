// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端会话列表加载（R110）。
 *
 * 旧实现 `catch { rows.value = [] }` 把网络/服务端失败折叠成「暂无会话」：
 * 用户看不出是「真的没有会话」还是「请求失败」，且 onShow 每次刷新失败都会
 * 清空正在看的列表。这里把失败保留旧数据、给出可见文案，变成可测函数。
 */

export type ConversationLoadOutcome<T> = { ok: true; rows: T[] } | { ok: false; rows: T[]; error: string }

export interface ConversationListResponse {
  data?: unknown
}

/** 页面展示用的错误文案：非 Error 也给非空提示，不许静默。 */
export function describeConversationError(error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `会话加载失败：${detail}`
}

export async function loadConversations<T>(
  fetchAll: () => Promise<ConversationListResponse | undefined>,
  previousRows: readonly T[] = [],
): Promise<ConversationLoadOutcome<T>> {
  try {
    const resp = await fetchAll()
    const rows = Array.isArray(resp?.data) ? (resp.data as T[]) : []
    return { ok: true, rows }
  } catch (error) {
    // 刷新失败保留旧会话：不能让用户正在看的列表被一次网络抖动抹掉。
    return { ok: false, rows: [...previousRows], error: describeConversationError(error) }
  }
}
