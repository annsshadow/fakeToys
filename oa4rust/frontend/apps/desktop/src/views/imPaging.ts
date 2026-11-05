// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * IM 消息分页的纯逻辑：页码进 URL、按 id 去重、按「本页是否满」判到底。
 *
 * 为什么单独抽出来：`IMChat.vue` 里 page 曾经只自增、请求固定第 1 页，
 * 结果「加载更多」既不换内容也没有提示。把 URL 拼装和合并放在这里，
 * 测试才能直接构造 120 条消息、连续拉 3 页并断言无重复与到底。
 */
export const IM_MESSAGE_PAGE_SIZE = 50

export interface MessagePageItem {
  id: string
}

export interface MessagePage<T extends MessagePageItem> {
  items: T[]
  page: number
  size: number
  /** 本页返回满 size 时认为可能还有下一页；不足一页即为到底。 */
  hasMore: boolean
}

type PostLike = (path: string, body?: unknown) => Promise<unknown>

/** 页码必须真正进 URL —— 这是「假翻页」的核心回归点。 */
export function buildMessageListPath(page: number, size = IM_MESSAGE_PAGE_SIZE): string {
  const safePage = Math.max(1, Math.floor(page))
  const safeSize = Math.max(1, Math.floor(size))
  return `/api/message/assemble/communicate/im/msg/list/${safePage}/size/${safeSize}`
}

/** 合并分页结果：按 id 去重，保留已有顺序，把新页追加在末尾（更旧的消息）。 */
export function mergeMessagePages<T extends MessagePageItem>(current: readonly T[], incoming: readonly T[]): T[] {
  const seen = new Set(current.map((item) => item.id))
  const appended: T[] = []
  for (const item of incoming) {
    if (!item?.id || seen.has(item.id)) continue
    seen.add(item.id)
    appended.push(item)
  }
  return [...current, ...appended]
}

/**
 * 取一页消息。post 由调用方注入（组件传 api.post，测试传内存假后端），
 * 这样「页码没进请求」会直接表现为三页拿到同一批数据，测试必红。
 */
export async function fetchMessagePage<T extends MessagePageItem>(
  post: PostLike,
  conversationId: string,
  page: number,
  size = IM_MESSAGE_PAGE_SIZE,
): Promise<MessagePage<T>> {
  const resp = (await post(buildMessageListPath(page, size), { conversationId })) as { data?: T[] } | undefined
  const items = Array.isArray(resp?.data) ? resp.data : []
  return {
    items,
    page,
    size,
    hasMore: items.length >= size,
  }
}
