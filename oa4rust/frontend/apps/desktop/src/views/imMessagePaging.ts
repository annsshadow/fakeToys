// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * IM「消息分页」诊断入口的纯逻辑（R106）。
 *
 * 旧实现固定请求 `/message/list/paging/1/size/20` 且丢弃响应，只弹「操作已提交」，
 * 页码不进请求、结果没人用。这里把「页码拼进 URL + 归一化结果 + 是否还有下一页」
 * 变成可测函数，页面按返回结果渲染当前页。
 */

export const IM_PAGING_PAGE_SIZE = 20

export interface ImPagingRow {
  id: string
  content: string
  sender: string
  createTime: string
}

export interface ImPagingPage {
  page: number
  size: number
  rows: ImPagingRow[]
  hasNext: boolean
}

export function buildImPagingPath(page: number, size = IM_PAGING_PAGE_SIZE): string {
  const safePage = Math.max(1, Math.floor(page))
  const safeSize = Math.max(1, Math.floor(size))
  return `/api/message/assemble/communicate/message/list/paging/${safePage}/size/${safeSize}`
}

type UnknownRecord = Record<string, unknown>

function pickRows(resp: unknown): unknown[] {
  const data = (resp as { data?: unknown } | undefined)?.data
  if (Array.isArray(data)) return data
  const nested = data as UnknownRecord | undefined
  if (Array.isArray(nested?.list)) return nested.list as unknown[]
  if (Array.isArray(nested?.data)) return nested.data as unknown[]
  return []
}

function text(value: unknown): string {
  return typeof value === 'string' ? value : ''
}

export function normalizePagingRows(resp: unknown): ImPagingRow[] {
  return pickRows(resp).flatMap((raw) => {
    const row = raw as UnknownRecord | null | undefined
    const id = text(row?.id)
    if (!id) return []
    return [
      {
        id,
        content: text(row?.content),
        sender: text(row?.sender),
        createTime: text(row?.createTime ?? row?.create_time),
      },
    ]
  })
}

export async function fetchImPagingPage(
  fetchPage: (path: string) => Promise<unknown>,
  page: number,
  size = IM_PAGING_PAGE_SIZE,
): Promise<ImPagingPage> {
  const safePage = Math.max(1, Math.floor(page))
  const safeSize = Math.max(1, Math.floor(size))
  const resp = await fetchPage(buildImPagingPath(safePage, safeSize))
  const rows = normalizePagingRows(resp)
  return { page: safePage, size: safeSize, rows, hasNext: rows.length >= safeSize }
}
