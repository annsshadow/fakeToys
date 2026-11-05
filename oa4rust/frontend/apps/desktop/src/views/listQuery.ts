// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { type QueryClient, type QueryKey, useQueryClient } from '@tanstack/vue-query'
import { type ComputedRef, computed, type Ref } from 'vue'

/** 失败文案：`${label}：${detail}`，detail 取异常 message，缺失时回落「未知错误」。 */
export function describeListError(label: string, error: unknown): string {
  const message = (error as { message?: unknown } | null | undefined)?.message
  const detail = typeof message === 'string' && message.trim() ? message.trim() : '未知错误'
  return `${label}：${detail}`
}

export interface ListErrorOptions {
  /** useQuery 返回的 isError。 */
  isError: Ref<boolean>
  /** useQuery 返回的 error。 */
  error: Ref<unknown>
  queryKey: QueryKey
  /** 失败提示前缀，如「模板加载失败」。 */
  label: string
  /** 仅测试注入：不传时走 useQueryClient（页面运行时行为不变）。 */
  client?: QueryClient
}

export interface ListErrorState {
  /** 空串表示无错误；非空即可展示（错误态必须优先于空态）。 */
  loadError: ComputedRef<string>
  reload: () => void
}

/**
 * 列表刷新统一入口（R131）。
 *
 * TanStack 的 `invalidateQueries` 默认 `cancelRefetch: true`：只要该 query 已有
 * data，正在进行的请求会被**取消并重新发起**。于是「刷新按钮连点 5 次」= 5 个请求，
 * 前 4 个服务端白做（实测同一 queryKey 5 次触发 → 5 次 queryFn 调用）。
 * 关掉 cancelRefetch 后，在途请求会被复用：5 次触发 → 1 次请求；请求结束后的下一次
 * 刷新仍会正常发起新请求（不会把刷新吞掉）。
 */
export function reloadListQueries(qc: QueryClient, queryKey: QueryKey): Promise<void> {
  return qc.invalidateQueries({ queryKey }, { cancelRefetch: false })
}

/**
 * 桌面列表页统一失败口径（R115）。
 *
 * 页面仍负责「loading / 空 / 有数据」三者的渲染顺序，但失败不再折叠成空态：
 * `loadError` 非空时先显示错误与重试，`items.length === 0` 只在 `loadError` 为空时才算真正的空。
 */
export function useListError(options: ListErrorOptions): ListErrorState {
  const qc = options.client ?? useQueryClient()
  const loadError = computed(() => (options.isError.value ? describeListError(options.label, options.error.value) : ''))
  const reload = (): void => {
    void reloadListQueries(qc, options.queryKey)
  }
  return { loadError, reload }
}
