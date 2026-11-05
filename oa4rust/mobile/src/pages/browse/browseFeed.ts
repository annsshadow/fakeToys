// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端只读浏览列表加载（R112，R113 收敛到统一口径）。
 *
 * 旧实现 `catch { rows.value = [] }` 把请求失败折叠成「暂无数据」：用户分不清
 * 「这个页签确实没有内容」还是「加载失败」，onShow 刷新失败还会清空正在看的列表。
 * 页签切换属于用户主动换数据源，不沿用旧页签的行；失败/空态判定统一由
 * `utils/loadList` 负责。
 */

import { type ListResponse, type LoadOutcome, loadList } from '@/utils/loadList'

export type BrowseRow = Record<string, unknown>

export type BrowseLoadOutcome = LoadOutcome<BrowseRow>

export type BrowseListResponse = ListResponse

export async function loadBrowseRows(
  fetchList: () => Promise<BrowseListResponse | undefined>,
  previousRows: readonly BrowseRow[] = [],
): Promise<BrowseLoadOutcome> {
  return loadList<BrowseRow>(fetchList, { previousRows, label: '加载失败' })
}
