// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node

import { createPinia, setActivePinia } from 'pinia'
import { describe, expect, it, vi } from 'vitest'

// 竞态回归：登录 / SSO / OAuth / 切用户 / 刷新成功后都会调用 init(true) 表达
// 「身份已变，必须重新读取当前用户」。若此刻路由守卫触发的普通 init() 仍在
// 在途（慢网下 who 请求未返回），force 必须发起**新的** who 请求；复用旧请求
// 会拿到身份变更前的匿名结果，让页面把「已登录」误判成「未建立会话」。

type WhoResponse = { data: { unique: string } | null }

function deferred<T>() {
  let resolve!: (value: T) => void
  const promise = new Promise<T>((r) => {
    resolve = r
  })
  return { promise, resolve }
}

async function freshStore() {
  vi.resetModules()
  setActivePinia(createPinia())
  const sessionModule = await import('./session.js')
  const apiModule = await import('./api.js')
  const store = sessionModule.useSession()
  const getSpy = vi.spyOn(apiModule.api, 'get')
  return { store, getSpy }
}

describe('session init(force) 竞态', () => {
  it('在途 init() 期间调用 init(true) 必须重新读取 who，而非复用旧请求', async () => {
    const { store, getSpy } = await freshStore()

    // 第一次 who：路由守卫的普通 init()，慢请求，最终返回「未登录」。
    const first = deferred<WhoResponse>()
    getSpy.mockImplementationOnce(() => first.promise as never)
    const guardInit = store.init()

    // 第二次 who：服务端已建立会话，init(true) 强制重读，应返回已登录用户。
    const second = deferred<WhoResponse>()
    getSpy.mockImplementationOnce(() => second.promise as never)
    const forcedInit = store.init(true)

    // 旧守卫请求先返回匿名：既不能覆盖身份，也不能提前解锁仍在途的 force 请求。
    first.resolve({ data: null })
    await guardInit
    expect(store.state.loading).toBe(true)
    expect(store.state.user).toBeNull()

    // force 的新请求返回已登录身份。
    second.resolve({ data: { unique: 'logged-in' } })
    await forcedInit

    // force 必须真的发起第二次 who 读取（旧实现只调用一次 → RED）。
    expect(getSpy).toHaveBeenCalledTimes(2)
    // 最终身份必须来自被 force 触发的那次读取，而不是旧的匿名结果。
    expect(store.state.user?.unique).toBe('logged-in')
    expect(store.isAuthenticated).toBe(true)
    expect(store.state.loading).toBe(false)
  })
})
