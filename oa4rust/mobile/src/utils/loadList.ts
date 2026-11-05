// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 列表加载的统一口径（R113 横切收敛）。
 *
 * 统一约定：
 * - 失败：保留上一次成功的数据，并给出非空错误文案（首个请求失败则 rows=[]）。
 * - 真实空：只有请求成功且返回空集合才算「暂无数据」。
 * - 渲染：loading → error → empty → list，错误分支恒先于空态。
 *
 * 各页只保留自己的业务映射（例如收藏页「只留本人、丢弃无 id」），
 * 不得各自复制一份 try/catch 与空态判定。
 */

export interface ListResponse {
  data?: unknown
}

export type LoadOutcome<T> = { ok: true; rows: T[] } | { ok: false; rows: T[]; error: string }

export type ListViewState = 'loading' | 'error' | 'empty' | 'list'

/** 页面展示用的错误文案：非 Error 也给非空提示，不许静默。 */
export function describeLoadError(label: string, error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `${label}：${detail}`
}

/** 三分支判定：错误优先于空态，首屏加载优先于两者。 */
export function listState(input: { loading: boolean; loaded: boolean; count: number; error: string }): ListViewState {
  if (input.loading && !input.loaded) return 'loading'
  if (input.error && input.count === 0) return 'error'
  if (input.count === 0) return 'empty'
  return 'list'
}

function asRows<T>(data: unknown): T[] {
  return Array.isArray(data) ? (data as T[]) : []
}

export async function loadList<T>(
  fetchList: () => Promise<ListResponse | undefined>,
  options: {
    previousRows?: readonly T[]
    label?: string
    map?: (data: unknown) => T[]
  } = {},
): Promise<LoadOutcome<T>> {
  const { previousRows = [], label = '加载失败', map = asRows<T> } = options
  try {
    const resp = await fetchList()
    return { ok: true, rows: map(resp?.data) }
  } catch (error) {
    // 刷新失败保留旧数据：一次网络抖动不能把用户正在看的列表抹成空态。
    return { ok: false, rows: [...previousRows], error: describeLoadError(label, error) }
  }
}
