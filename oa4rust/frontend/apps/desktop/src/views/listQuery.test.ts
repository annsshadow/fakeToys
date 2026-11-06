// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { QueryClient, QueryObserver } from '@tanstack/vue-query'
import { describe, expect, it } from 'vitest'
import { ref } from 'vue'
import { reloadListQueries, useListError } from './listQuery'

/** 只用于把事件循环让出去，不做任何耗时断言。 */
const flush = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0))

function deferred(): { promise: Promise<undefined>; resolve: () => void } {
  let resolve!: () => void
  const promise = new Promise<undefined>((r) => {
    resolve = () => r(undefined)
  })
  return { promise, resolve }
}

interface Harness {
  client: QueryClient
  gate: { promise: Promise<undefined>; resolve: () => void }
  calls: () => number
  unsubscribe: () => void
}

/**
 * 模拟一次已加载过的列表：先写入 data（下拉刷新/刷新按钮都是在有数据时才连点），
 * 再挂一个 active observer，queryFn 由 gate 卡住，便于在「请求在途」时连续触发。
 */
function harness(queryKey: readonly unknown[]): Harness {
  const client = new QueryClient({ defaultOptions: { queries: { retry: false, gcTime: Infinity } } })
  client.setQueryData(queryKey, [{ id: 'seed' }])
  let count = 0
  const gate = deferred()
  const observer = new QueryObserver(client, {
    queryKey,
    queryFn: () => {
      count += 1
      return gate.promise.then(() => [{ id: `row-${count}` }])
    },
    staleTime: 5 * 60 * 1000,
  })
  const unsubscribe = observer.subscribe(() => {})
  return { client, gate, calls: () => count, unsubscribe }
}

describe('桌面列表刷新去重（R131）', () => {
  it('同一 queryKey 连点 5 次刷新，在途期间只发起 1 次请求', async () => {
    const qk = ['r131', 'reload']
    const h = harness(qk)
    try {
      for (let i = 0; i < 5; i += 1) void reloadListQueries(h.client, qk)
      await flush()
      expect(h.calls()).toBe(1)
    } finally {
      h.gate.resolve()
      await flush()
      h.unsubscribe()
    }
  })

  it('对照：未去重的 invalidateQueries 默认行为会发起 5 次请求（这就是基线）', async () => {
    const qk = ['r131', 'baseline']
    const h = harness(qk)
    try {
      for (let i = 0; i < 5; i += 1) void h.client.invalidateQueries({ queryKey: qk })
      await flush()
      expect(h.calls()).toBe(5)
    } finally {
      h.gate.resolve()
      await flush()
      h.unsubscribe()
    }
  })

  it('请求结束后再点刷新仍会发起新请求（去重只合并「在途」的重复触发）', async () => {
    const qk = ['r131', 'after-settled']
    const h = harness(qk)
    try {
      void reloadListQueries(h.client, qk)
      await flush()
      expect(h.calls()).toBe(1)

      h.gate.resolve()
      await flush()
      expect(h.calls()).toBe(1)

      void reloadListQueries(h.client, qk)
      await flush()
      expect(h.calls()).toBe(2)
    } finally {
      h.unsubscribe()
    }
  })

  it('同一页面两个组件订阅同一 queryKey 只发 1 次请求（缓存本身已按 key 合并）', async () => {
    const client = new QueryClient({ defaultOptions: { queries: { retry: false, gcTime: Infinity } } })
    const qk = ['r131', 'shared']
    let count = 0
    const gate = deferred()
    const options = {
      queryKey: qk,
      queryFn: () => {
        count += 1
        return gate.promise.then(() => [])
      },
    }
    const a = new QueryObserver(client, options)
    const b = new QueryObserver(client, options)
    const ua = a.subscribe(() => {})
    const ub = b.subscribe(() => {})
    gate.resolve()
    await flush()
    ua()
    ub()
    expect(count).toBe(1)
  })

  it('页签切换属于不同 queryKey：每个页签各 1 次，切回 staleTime 内不重发（不是重复请求）', async () => {
    const client = new QueryClient({ defaultOptions: { queries: { retry: false, gcTime: Infinity } } })
    const calls = { a: 0, b: 0 }
    const make = (name: 'a' | 'b') => {
      const observer = new QueryObserver(client, {
        queryKey: ['r131', 'tab', name],
        queryFn: () => {
          calls[name] += 1
          return Promise.resolve([])
        },
        staleTime: 5 * 60 * 1000,
      })
      return observer.subscribe(() => {})
    }
    const ua = make('a')
    ua()
    const ub = make('b')
    ub()
    const ua2 = make('a')
    await flush()
    ua2()
    expect(calls).toEqual({ a: 1, b: 1 })
  })

  it('页面实际调用的 useListError.reload 也走同一入口：5 次重试只发 1 次请求', async () => {
    const qk = ['r131', 'page-reload']
    const h = harness(qk)
    try {
      const { reload } = useListError({
        isError: ref(false),
        error: ref(null),
        queryKey: qk,
        label: '加载失败',
        client: h.client,
      })
      for (let i = 0; i < 5; i += 1) reload()
      await flush()
      expect(h.calls()).toBe(1)
    } finally {
      h.gate.resolve()
      await flush()
      h.unsubscribe()
    }
  })
})
