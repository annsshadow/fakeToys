// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端会议列表分页（R149）。
 *
 * 旧实现「我的会议」固定 `meetingApi.applied(1, 50)`：第 51 条以后的会议
 * 既加载不到，页面也没有任何「还有更多」提示。这里把每页大小、是否还有下一页
 * 与按 id 去重合并抽成可测函数。
 */

export const MEETING_PAGE_SIZE = 50

export function meetingHasMore(rowsLength: number, size = MEETING_PAGE_SIZE): boolean {
  return rowsLength >= Math.max(1, Math.floor(size))
}

/**
 * 追加下一页并按 id 去重。分页基于 OFFSET，接受/拒绝会议会让相邻页重叠，
 * 去重是必需的；无 id 的行无法判重，原样保留。
 */
export function mergeMeetingRows<T extends { id?: unknown }>(current: readonly T[], incoming: readonly T[]): T[] {
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
