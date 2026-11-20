// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { beforeEach, describe, expect, it, vi } from 'vitest'
import { cachedRequest, clearRequestCache } from './requestCache'

function deferred<T>() {
  let resolve!: (value: T) => void
  let reject!: (error: unknown) => void
  const promise = new Promise<T>((res, rej) => {
    resolve = res
    reject = rej
  })
  return { promise, resolve, reject }
}

beforeEach(() => {
  clearRequestCache()
  vi.useRealTimers()
})

describe('cachedRequest：只读请求缓存与在途合并', () => {
  it('TTL 内复用同一 key 的结果，不再打网络', async () => {
    const loader = vi.fn().mockResolvedValue({ rows: [1] })
    await cachedRequest('a', loader, { ttlMs: 1000 })
    const second = await cachedRequest('a', loader, { ttlMs: 1000 })
    expect(loader).toHaveBeenCalledTimes(1)
    expect(second).toEqual({ rows: [1] })
  })

  it('过期后重新加载', async () => {
    vi.useFakeTimers()
    const loader = vi.fn().mockResolvedValue(1)
    await cachedRequest('ttl', loader, { ttlMs: 10 })
    await vi.advanceTimersByTimeAsync(11)
    await cachedRequest('ttl', loader, { ttlMs: 10 })
    expect(loader).toHaveBeenCalledTimes(2)
  })

  it('并发调用合并为一个在途请求', async () => {
    const gate = deferred<number>()
    const loader = vi.fn().mockReturnValue(gate.promise)
    const first = cachedRequest('inflight', loader)
    const second = cachedRequest('inflight', loader)
    gate.resolve(42)
    await expect(Promise.all([first, second])).resolves.toEqual([42, 42])
    expect(loader).toHaveBeenCalledTimes(1)
  })

  it('force 绕过缓存与在途请求，显式重试拿到新响应', async () => {
    const loader = vi.fn().mockResolvedValueOnce('old').mockResolvedValueOnce('new')
    expect(await cachedRequest('force', loader)).toBe('old')
    expect(await cachedRequest('force', loader, { force: true })).toBe('new')
    expect(loader).toHaveBeenCalledTimes(2)
  })

  it('失败不缓存且原样抛出，下次可重试', async () => {
    const loader = vi.fn().mockRejectedValueOnce(new Error('network')).mockResolvedValueOnce('ok')
    await expect(cachedRequest('error', loader)).rejects.toThrow('network')
    expect(await cachedRequest('error', loader)).toBe('ok')
    expect(loader).toHaveBeenCalledTimes(2)
  })

  it('clearRequestCache 支持按 key 清理与全量清理', async () => {
    const loader = vi.fn().mockImplementation((key: string) => Promise.resolve(key))
    await cachedRequest('x', () => loader('x'))
    await cachedRequest('y', () => loader('y'))
    clearRequestCache('x')
    await cachedRequest('x', () => loader('x'))
    expect(loader).toHaveBeenCalledTimes(3)
    clearRequestCache()
    await cachedRequest('y', () => loader('y'))
    expect(loader).toHaveBeenCalledTimes(4)
  })
})
