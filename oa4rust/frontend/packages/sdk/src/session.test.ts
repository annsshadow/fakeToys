// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createPinia, setActivePinia } from 'pinia'
import { describe, expect, it, vi } from 'vitest'

// Behavioral (state-machine) tests for the cookie-only session store. Source-string
// assertions in auth.test.ts prove "no token handling"; these prove "what the store
// does with current-user / login / logout / refresh". The store keeps a module-level
// single-flight init promise, so each test gets a fresh module instance.
async function loadSession() {
  const sessionModule = await import('./session.js')
  const apiModule = await import('./api.js')
  return { sessionModule, api: apiModule.api }
}

describe('session store (cookie-only)', () => {
  async function freshStore() {
    vi.resetModules()
    setActivePinia(createPinia())
    const { sessionModule, api } = await loadSession()
    const getSpy = vi.spyOn(api, 'get')
    const postSpy = vi.spyOn(api, 'post')
    return { store: sessionModule.useSession(), getSpy, postSpy, api }
  }

  it('restores an authenticated user from current-user on init', async () => {
    const { store, getSpy } = await freshStore()
    getSpy.mockResolvedValue({ data: { unique: 'u1', name: 'Alice' } } as never)
    await store.init()
    expect(getSpy).toHaveBeenCalledWith('/api/authentication/who', { requireAuth: false })
    expect(store.isAuthenticated).toBe(true)
    expect(store.state.user?.unique).toBe('u1')
  })

  it('treats a current-user failure as initialized+anonymous (no throw)', async () => {
    const { store, getSpy } = await freshStore()
    getSpy.mockRejectedValue(new Error('401'))
    await store.init()
    expect(store.isAuthenticated).toBe(false)
    expect(store.state.initialized).toBe(true)
    expect(store.state.user).toBeNull()
  })

  it('init(true) forces a re-read and picks up a changed user', async () => {
    const { store, getSpy } = await freshStore()
    getSpy.mockResolvedValue({ data: { unique: 'u2' } } as never)
    await store.init(true)
    expect(getSpy).toHaveBeenCalledTimes(1)
    expect(store.state.user?.unique).toBe('u2')
    getSpy.mockResolvedValue({ data: { unique: 'u3' } } as never)
    await store.init(true)
    expect(getSpy).toHaveBeenCalledTimes(2)
    expect(store.state.user?.unique).toBe('u3')
  })

  it('logout clears the local user even when the request fails', async () => {
    const { store, getSpy, postSpy } = await freshStore()
    getSpy.mockResolvedValue({ data: { unique: 'u1' } } as never)
    await store.init()
    expect(store.isAuthenticated).toBe(true)
    postSpy.mockRejectedValue(new Error('network'))
    await store.logout()
    expect(store.state.user).toBeNull()
    expect(store.isAuthenticated).toBe(false)
  })

  it('login posts the credential contract and resolves the user from current-user', async () => {
    const { store, getSpy, postSpy } = await freshStore()
    postSpy.mockResolvedValue({} as never)
    getSpy.mockResolvedValue({ data: { unique: 'u9' } } as never)
    await store.login('it-login', 'pw')
    // No token is read from or returned in the login JSON; identity comes from the
    // server-authoritative current-user call that follows.
    expect(postSpy).toHaveBeenCalledTimes(1)
    expect(store.state.user?.unique).toBe('u9')
    expect(store.isAuthenticated).toBe(true)
  })

  it('a second init() reuses the finished first read instead of hammering the endpoint', async () => {
    const { store, getSpy } = await freshStore()
    getSpy.mockResolvedValue({ data: { unique: 'u1' } } as never)
    await store.init()
    await store.init()
    expect(getSpy).toHaveBeenCalledTimes(1)
    expect(store.state.user?.unique).toBe('u1')
  })

  it('login() throws AuthenticationError when the post did not establish a user', async () => {
    const { store, getSpy, postSpy } = await freshStore()
    postSpy.mockResolvedValue({} as never)
    getSpy.mockResolvedValue({ data: null } as never)
    await expect(store.login('it-login', 'pw')).rejects.toThrow('Login did not create a session')
  })

  it('refresh() asks the client to refresh the session then re-reads the current user', async () => {
    const { store, getSpy, api } = await freshStore()
    const refreshSpy = vi.spyOn(api, 'refreshSession').mockResolvedValue(undefined)
    getSpy.mockResolvedValue({ data: { unique: 'u42' } } as never)
    await store.refresh()
    expect(refreshSpy).toHaveBeenCalledTimes(1)
    expect(store.state.user?.unique).toBe('u42')
  })

  it('switchUser() posts the target unique and resolves under the new identity', async () => {
    const { store, getSpy, postSpy } = await freshStore()
    postSpy.mockResolvedValue({} as never)
    getSpy.mockResolvedValue({ data: { unique: 'bob' } } as never)
    const user = await store.switchUser('bob')
    expect(postSpy).toHaveBeenCalledWith(
      '/api/authentication/switchuser',
      { targetUnique: 'bob' },
      {
        discardResponse: true,
      },
    )
    expect(user.unique).toBe('bob')
  })

  it('switchUser() throws AuthenticationError when no user is established afterwards', async () => {
    const { store, getSpy, postSpy } = await freshStore()
    postSpy.mockResolvedValue({} as never)
    getSpy.mockResolvedValue({ data: null } as never)
    await expect(store.switchUser('bob')).rejects.toThrow('User switch did not create a session')
  })

  it('a 401 current-user resets the stored user through the failure handler', async () => {
    // 语义：会话失效（后端 401 且非 refresh 请求）时，store 必须经
    // setAuthenticationFailureHandler 把本地用户清掉，让视图回退到登录态而不是
    // 拿着过期 user 继续渲染。
    const { store, api } = await freshStore()
    vi.stubGlobal(
      'fetch',
      vi.fn((input: string) => {
        const u = String(input)
        if (u.includes('/who')) {
          return Promise.resolve(new Response(JSON.stringify({ success: false }), { status: 401 }))
        }
        return Promise.resolve(new Response(JSON.stringify({ success: true, data: {} }), { status: 200 }))
      }),
    )
    await store.init()
    expect(store.state.user).toBeNull()
    expect(store.state.initialized).toBe(true)
    expect(store.isAuthenticated).toBe(false)
    vi.unstubAllGlobals()
  })
})
