// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端会话列表加载（R110，R113 收敛到统一口径）。
 *
 * 旧实现 `catch { rows.value = [] }` 把网络/服务端失败折叠成「暂无会话」：
 * 用户看不出是「真的没有会话」还是「请求失败」，且 onShow 每次刷新失败都会
 * 清空正在看的列表。失败保留旧数据、给出可见文案统一由 `utils/loadList` 负责。
 */

import { type ListResponse, type LoadOutcome, loadList } from '@/utils/loadList'

export type ConversationLoadOutcome<T> = LoadOutcome<T>

export type ConversationListResponse = ListResponse

export async function loadConversations<T>(
  fetchAll: () => Promise<ConversationListResponse | undefined>,
  previousRows: readonly T[] = [],
): Promise<ConversationLoadOutcome<T>> {
  return loadList<T>(fetchAll, { previousRows, label: '会话加载失败' })
}
