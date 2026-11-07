// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 工作台「待办审批」计数（R149）。
 *
 * 后端 `task/list/my/paging` 的 `count` 是**本页条数**而不是总数
 * （见 `processplatform_assemble_surface` 的 `task_list_my_paging_page_size_size_core`：
 * `let count = data.len()`）。旧实现固定请求 50 条并直接展示 count：
 * 待办 ≥51 条时工作台永远显示 50，用户既看不出还有更多，也没有入口知道真实量级。
 *
 * 这里改为「多取一条」的哨兵页：请求 51 条，若拿满 51 条就展示 `50+`。
 */

export const PENDING_PAGE_SIZE = 50
export const PENDING_PROBE_SIZE = PENDING_PAGE_SIZE + 1

export interface PendingBadge {
  text: string
  overflow: boolean
}

export function pendingBadge(total: number | null | undefined): PendingBadge {
  if (total === null || total === undefined || !Number.isFinite(total) || total < 0) {
    return { text: '–', overflow: false }
  }
  if (total >= PENDING_PROBE_SIZE) {
    return { text: `${PENDING_PAGE_SIZE}+`, overflow: true }
  }
  return { text: String(Math.floor(total)), overflow: false }
}
