// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 论坛主题列表的加载结果（R105）。
 *
 * 页面原有 `catch { rows.value = [] }` 把「网络/服务端失败」和「真实没有主题」
 * 折叠成同一个空数组，用户只看到「论坛暂无主题」——失败被静默吞掉。
 * 这里把失败与空结果分开返回：失败保留上一次成功的主题并附可见错误文案，
 * 页面据此渲染错误提示与「重试」，只有 `ok && rows.length === 0` 才是真空。
 */

export interface BbsTopicsResponse {
  data?: unknown
}

export type BbsLoadOutcome<T> = { ok: true; rows: T[] } | { ok: false; rows: T[]; error: string }

/** 从任意异常里取可展示文案；非 Error 也给出非空提示，避免再次静默。 */
export function describeBbsError(error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `主题加载失败：${detail}`
}

export async function loadBbsTopics<T>(
  fetchAll: () => Promise<BbsTopicsResponse | undefined>,
  previousRows: readonly T[] = [],
): Promise<BbsLoadOutcome<T>> {
  try {
    const resp = await fetchAll()
    const rows = Array.isArray(resp?.data) ? (resp.data as T[]) : []
    return { ok: true, rows }
  } catch (error) {
    // 失败时保留旧数据：下拉刷新失败不该顺手清空用户正在看的列表。
    return { ok: false, rows: [...previousRows], error: describeBbsError(error) }
  }
}
