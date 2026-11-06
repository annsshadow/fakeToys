// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { ApiError } from '@oa4rust/sdk'
import { MutationObserver, QueryClient, QueryObserver } from '@tanstack/vue-query'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ref } from 'vue'
import { shouldRetryQuery, useListError } from './listQuery'

interface Outcome {
  calls: number
  loadError: string
}

/**
 * 用真实 QueryClient 注入失败（假后端）：queryFn 每次都 reject 指定错误，
 * 推进假时钟走完默认退避（1s、2s），统计真实尝试次数与用户看到的文案。
 */
async function runFailure(queryKey: string, error: unknown, settleMs = 5000): Promise<Outcome> {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: shouldRetryQuery, gcTime: Infinity, staleTime: 0 } },
  })
  let calls = 0
  const isError = ref(false)
  const failure = ref<unknown>(null)
  const observer = new QueryObserver(client, {
    queryKey: ['r133', queryKey],
    queryFn: () => {
      calls += 1
      return Promise.reject(error)
    },
  })
  const unsubscribe = observer.subscribe((result) => {
    isError.value = result.isError
    failure.value = result.error
  })
  await vi.advanceTimersByTimeAsync(0)
  await vi.advanceTimersByTimeAsync(settleMs)
  const { loadError } = useListError({
    isError,
    error: failure,
    queryKey: ['r133', queryKey],
    label: '加载失败',
    client,
  })
  unsubscribe()
  return { calls, loadError: loadError.value }
}

describe('R133 弱网/超时重试策略（假后端注入失败）', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  it('5xx 属于可能恢复：最多 3 次尝试，失败后给出明确文案', async () => {
    const out = await runFailure('server-500', new ApiError('HTTP 500', 500))
    expect(out.calls).toBe(3)
    expect(out.loadError).toContain('加载失败：HTTP 500')
  })

  it('403 不重试：第 1 次失败就停并显示文案（不把等待拖成 3 次）', async () => {
    const out = await runFailure('forbidden', new ApiError('HTTP 403', 403))
    expect(out.calls).toBe(1)
    expect(out.loadError).toContain('加载失败：HTTP 403')
  })

  it('超时（status 0）不重试：一次 30s 即失败，不会被重试拖成 90s', async () => {
    const out = await runFailure('timeout', new ApiError('请求超时', 0))
    expect(out.calls).toBe(1)
    expect(out.loadError).toContain('加载失败：请求超时')
  })

  it('瞬时网络错误（TypeError）会重试，共 3 次；30s 内不再无限重试', async () => {
    const queryKey = 'offline'
    const client = new QueryClient({
      defaultOptions: { queries: { retry: shouldRetryQuery, gcTime: Infinity, staleTime: 0 } },
    })
    let calls = 0
    const observer = new QueryObserver(client, {
      queryKey: ['r133', queryKey],
      queryFn: () => {
        calls += 1
        return Promise.reject(new TypeError('Failed to fetch'))
      },
    })
    const unsubscribe = observer.subscribe(() => {})
    await vi.advanceTimersByTimeAsync(5000)
    expect(calls).toBe(3)
    // 断网超过 30s 也不会有第 4 次尝试（没有无限重试）
    await vi.advanceTimersByTimeAsync(30_000)
    expect(calls).toBe(3)
    unsubscribe()
  })

  it('重试期间不提前显示错误文案，最终失败才显示', async () => {
    const client = new QueryClient({
      defaultOptions: { queries: { retry: shouldRetryQuery, gcTime: Infinity, staleTime: 0 } },
    })
    const isError = ref(false)
    const failure = ref<unknown>(null)
    const observer = new QueryObserver(client, {
      queryKey: ['r133', 'retrying'],
      queryFn: () => Promise.reject(new ApiError('HTTP 500', 500)),
    })
    const unsubscribe = observer.subscribe((result) => {
      isError.value = result.isError
      failure.value = result.error
    })
    const { loadError } = useListError({
      isError,
      error: failure,
      queryKey: ['r133', 'retrying'],
      label: '加载失败',
      client,
    })
    await vi.advanceTimersByTimeAsync(0)
    expect(loadError.value).toBe('')
    await vi.advanceTimersByTimeAsync(5000)
    expect(loadError.value).toContain('加载失败：HTTP 500')
    unsubscribe()
  })

  it('非幂等写请求不自动重试：网络失败 1 次就停，避免重复提交', async () => {
    const client = new QueryClient({ defaultOptions: { mutations: { retry: 0 } } })
    let calls = 0
    const observer = new MutationObserver(client, {
      mutationFn: () => {
        calls += 1
        return Promise.reject(new TypeError('Failed to fetch'))
      },
    })
    await observer.mutate().catch(() => undefined)
    await vi.advanceTimersByTimeAsync(3000)
    expect(calls).toBe(1)
  })

  it('策略表：超时与 4xx 不重试，5xx/408/429/网络错误重试', () => {
    expect(shouldRetryQuery(0, new ApiError('请求超时', 0))).toBe(false)
    expect(shouldRetryQuery(0, new ApiError('HTTP 403', 403))).toBe(false)
    expect(shouldRetryQuery(0, new ApiError('HTTP 404', 404))).toBe(false)
    expect(shouldRetryQuery(0, new ApiError('HTTP 408', 408))).toBe(true)
    expect(shouldRetryQuery(0, new ApiError('HTTP 429', 429))).toBe(true)
    expect(shouldRetryQuery(0, new ApiError('HTTP 500', 500))).toBe(true)
    expect(shouldRetryQuery(0, new TypeError('Failed to fetch'))).toBe(true)
    expect(shouldRetryQuery(2, new ApiError('HTTP 500', 500))).toBe(false)
  })
})
