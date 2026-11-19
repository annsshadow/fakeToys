// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 轻量请求缓存 / 在途合并工具（移动端性能收敛）。
 *
 * 场景：uni-app 页面 onShow 反复触发的只读端点（公告、字典、统计等）。
 * 语义：
 * - TTL 内命中直接复用，不重复打网络；
 * - 同一 key 有多个并发调用时合并为一个在途请求（不含 force）；
 * - force=true 绕过缓存与在途请求，用于用户显式重试 / 刷新；
 * - 失败不写缓存、原样抛出，让 loadList 保留旧数据的既有语义不被破坏。
 *
 * 仅做内存缓存（页面级生命周期），不持久化、不引入依赖；
 * 写端点禁止使用，避免读到过期状态。
 */

interface CacheEntry<T> {
  value: T
  expiresAt: number
}

const store = new Map<string, CacheEntry<unknown>>()
const inflight = new Map<string, Promise<unknown>>()

export interface CacheOptions {
  /** 缓存有效期（毫秒），默认 30 秒。 */
  ttlMs?: number
  /** 强制刷新：忽略缓存与在途请求，发起新请求。 */
  force?: boolean
}

export async function cachedRequest<T>(
  key: string,
  loader: () => Promise<T>,
  options: CacheOptions = {},
): Promise<T> {
  const ttl = options.ttlMs ?? 30_000
  if (!options.force) {
    const hit = store.get(key)
    if (hit && hit.expiresAt > Date.now()) return hit.value as T
    const running = inflight.get(key)
    if (running) return running as Promise<T>
  }

  const task = loader().then(
    (value) => {
      store.set(key, { value, expiresAt: Date.now() + ttl })
      return value
    },
  )
  const tracked = task.finally(() => {
    // force 请求会覆盖旧在途项；只清自己对应的那条，避免误删后来的请求。
    if (inflight.get(key) === tracked) inflight.delete(key)
  })
  inflight.set(key, tracked)
  return tracked
}

/** 清理缓存（不传 key 清空全部）。不取消已经在途的请求。 */
export function clearRequestCache(key?: string): void {
  if (key === undefined) {
    store.clear()
    return
  }
  store.delete(key)
}
