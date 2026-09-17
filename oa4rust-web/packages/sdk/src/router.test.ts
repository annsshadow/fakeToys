// @vitest-environment node
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

vi.mock('./api.js', () => ({
  api: { get: vi.fn() },
}))

import { api } from './api.js'
import { authRoutes, checkAuth, getAppParams, openApp, setRouter, useRouter } from './router'

interface FakeRouter {
  push: (path: string) => void
  replace: (path: string) => void
  currentRoute: { value: { query: Record<string, string> } }
}

function fakeRouter(query: Record<string, string> = {}): FakeRouter {
  return {
    push: vi.fn() as never,
    replace: vi.fn() as never,
    currentRoute: { value: { query } },
  }
}

let router: FakeRouter

beforeEach(() => {
  router = fakeRouter()
  setRouter(router as never)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.restoreAllMocks()
})

describe('useRouter', () => {
  it('throws a helpful error when no router is installed yet', () => {
    setRouter(null as never)
    expect(() => useRouter()).toThrow('Call createO2App() first')
  })
})

describe('openApp', () => {
  it('pushes /app/{id} with URLSearchParams-encoded query params', async () => {
    await openApp('bbs', { tab: 'forum', q: 'a b&c' })
    expect(router.push).toHaveBeenCalledTimes(1)
    // URLSearchParams 把空格编码为 "+"（application/x-www-form-urlencoded 语义）。
    expect(router.push).toHaveBeenCalledWith('/app/bbs?tab=forum&q=a+b%26c')
    expect(router.replace).not.toHaveBeenCalled()
  })

  it('omits the query string when no params are given', async () => {
    await openApp('bbs')
    expect(router.push).toHaveBeenCalledWith('/app/bbs')
  })

  it('replace option navigates without polluting the history stack', async () => {
    await openApp('bbs', undefined, { replace: true })
    expect(router.replace).toHaveBeenCalledWith('/app/bbs')
    expect(router.push).not.toHaveBeenCalled()
  })

  it('newWindow option delegates to window.open and skips the router', async () => {
    const openSpy = vi.fn()
    vi.stubGlobal('window', { open: openSpy, location: { protocol: 'https:', host: 'oa.example' } })
    await openApp('bbs', { x: '1' }, { newWindow: true })
    expect(openSpy).toHaveBeenCalledTimes(1)
    expect(openSpy.mock.calls[0][0]).toBe('/app/bbs?x=1')
    expect(openSpy.mock.calls[0][1]).toBe('_blank')
    expect(router.push).not.toHaveBeenCalled()
  })
})

describe('getAppParams', () => {
  it('returns the current route query cast to the requested type', () => {
    setRouter(fakeRouter({ flag: 'x_pp_ann' }) as never)
    const params = getAppParams<{ flag: string }>()
    expect(params.flag).toBe('x_pp_ann')
  })
})

describe('authRoutes', () => {
  it('pins the legacy auth paths the login screen and OAuth callbacks rely on', () => {
    expect(authRoutes).toEqual({
      login: '/login',
      callback: '/oauth/callback/:platform',
      error: '/oauth/error',
      sso: '/sso',
    })
  })
})

describe('checkAuth', () => {
  it('resolves true when /who returns a user', async () => {
    vi.mocked(api.get).mockResolvedValue({ success: true, data: { unique: 'u-1' } as never })
    await expect(checkAuth()).resolves.toBe(true)
  })

  it('resolves false when /who has no session (null user)', async () => {
    vi.mocked(api.get).mockResolvedValue({ success: true, data: null })
    await expect(checkAuth()).resolves.toBe(false)
  })

  it('resolves false (not throws) on a network/auth failure', async () => {
    vi.mocked(api.get).mockRejectedValue(new Error('HTTP 401'))
    await expect(checkAuth()).resolves.toBe(false)
  })
})
