// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端会话历史分页（R108）。
 *
 * 旧实现固定 `msgHistory(conv, 1, 200)`：超过 200 条的会话，更早消息既加载不到、
 * 也没有任何「还有更多」提示 —— 静默截断。这里把页码/大小、是否还有更早、
 * 以及「新一页按时间正序插入顶部」变成可测函数，页面据此提供加载与到底反馈。
 */

export const CHAT_HISTORY_PAGE_SIZE = 200

export interface ChatHistoryPage<T> {
  page: number
  size: number
  /** 原样返回后端顺序（最新在前）。 */
  messages: T[]
  hasMore: boolean
}

export type ChatHistoryLoader = (
  conversationId: string,
  page: number,
  size: number,
) => Promise<{ data?: unknown } | undefined>

function rowsOf<T>(resp: { data?: unknown } | undefined): T[] {
  const data = resp?.data
  return Array.isArray(data) ? (data as T[]) : []
}

export async function loadChatHistoryPage<T>(
  load: ChatHistoryLoader,
  conversationId: string,
  page: number,
  size = CHAT_HISTORY_PAGE_SIZE,
): Promise<ChatHistoryPage<T>> {
  const safePage = Math.max(1, Math.floor(page))
  const safeSize = Math.max(1, Math.floor(size))
  const messages = rowsOf<T>(await load(conversationId, safePage, safeSize))
  return { page: safePage, size: safeSize, messages, hasMore: messages.length >= safeSize }
}

/**
 * 把更早一页（后端顺序：新→旧）按时间正序插到当前列表**顶部**；按 id 去重。
 * 分页基于 OFFSET，新消息插入会让相邻页重叠，去重是必需的。
 */
export function mergeOlderMessages<T extends { id: string }>(current: readonly T[], olderDesc: readonly T[]): T[] {
  const seen = new Set<string>()
  for (const message of current) {
    if (message?.id) seen.add(message.id)
  }
  const prepend: T[] = []
  for (let i = olderDesc.length - 1; i >= 0; i--) {
    const message = olderDesc[i]
    const id = message?.id
    if (!id || seen.has(id)) continue
    seen.add(id)
    prepend.push(message)
  }
  return [...prepend, ...current]
}

/** 页面展示用的错误文案：非 Error 也给非空提示，不许静默。 */
export function describeChatError(prefix: string, error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `${prefix}：${detail}`
}
