/**
 * 移动端会话存储（pinia）单测。
 *
 * mock 掉 authApi（@/services）与传输层的认证失败回调（@/services/http），
 * 钉死会话生命周期语义：init 拉 /who 恢复会话、login 后强制重新拉取、
 * 本地登出无条件清用户、认证失败回调把本地会话打回未登录。
 */
import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mocked = vi.hoisted(() => {
  return {
    who: vi.fn(),
    login: vi.fn(),
    logout: vi.fn(),
    refresh: vi.fn(),
    /** 传输层注册的认证失败回调（store setup 时注册一次）。 */
    authFailureHandler: null as (() => void) | null,
  }
})

vi.mock('@/services', () => ({
  authApi: {
    who: (...args: unknown[]) => mocked.who(...args),
    login: (...args: unknown[]) => mocked.login(...args),
    logout: (...args: unknown[]) => mocked.logout(...args),
    refresh: (...args: unknown[]) => mocked.refresh(...args),
  },
}))

vi.mock('@/services/http', () => ({
  setAuthenticationFailureHandler: (fn: (() => void) | null) => {
    mocked.authFailureHandler = fn
  },
}))

import { useSession, useSessionStore } from './session'

function makeUser(overrides: Record<string, unknown> = {}) {
  return { unique: 'u-1', name: '测试用户', ...overrides } as never
}

function ok<T>(data: T) {
  return Promise.resolve({ success: true, data })
}

beforeEach(() => {
  setActivePinia(createPinia())
  mocked.who.mockReset()
  mocked.login.mockReset()
  mocked.logout.mockReset()
  mocked.refresh.mockReset()
  mocked.authFailureHandler = null
})

describe('initial state', () => {
  it('starts unauthenticated with a pending init', () => {
    const session = useSession()
    expect(session.user).toBeNull()
    expect(session.isAuthenticated).toBe(false)
    expect(session.initialized).toBe(false)
    expect(session.loading).toBe(false)
  })
})

describe('init (session restore via /who)', () => {
  it('restores an authenticated user from /who and marks itself initialized', async () => {
    mocked.who.mockReturnValue(ok(makeUser()))
    const session = useSession()
    await session.init()
    expect(session.user).toMatchObject({ unique: 'u-1', name: '测试用户' })
    expect(session.isAuthenticated).toBe(true)
    expect(session.initialized).toBe(true)
    expect(session.loading).toBe(false)
  })

  it('is idempotent: a second init without force does not re-fetch /who', async () => {
    mocked.who.mockReturnValue(ok(makeUser()))
    const session = useSession()
    await session.init()
    await session.init()
    expect(mocked.who).toHaveBeenCalledTimes(1)
  })

  it('init(force) always re-fetches /who (login/refresh 依赖此语义)', async () => {
    mocked.who.mockReturnValue(ok(makeUser()))
    const session = useSession()
    await session.init()
    await session.init(true)
    expect(mocked.who).toHaveBeenCalledTimes(2)
  })

  it('a user payload without unique is treated as unauthenticated', async () => {
    mocked.who.mockReturnValue(ok({ unique: '', name: '匿名' } as never))
    const session = useSession()
    await session.init()
    expect(session.user).toBeNull()
    expect(session.isAuthenticated).toBe(false)
  })

  it('a /who failure (401 等) leaves the session unauthenticated without throwing', async () => {
    mocked.who.mockRejectedValue(new Error('HTTP 401'))
    const session = useSession()
    await expect(session.init()).resolves.toBeUndefined()
    expect(session.user).toBeNull()
    expect(session.isAuthenticated).toBe(false)
    expect(session.initialized).toBe(true)
  })
})

describe('login', () => {
  it('sends the credential payload, re-fetches /who, and returns the user', async () => {
    mocked.login.mockReturnValue(ok(undefined))
    mocked.who.mockReturnValue(ok(makeUser()))
    const session = useSession()
    const user = await session.login('admin', 'secret', 'cap-id', 'cap-ans')
    expect(mocked.login).toHaveBeenCalledWith({
      credential: 'admin',
      password: 'secret',
      captchaId: 'cap-id',
      captchaAnswer: 'cap-ans',
    })
    expect(user.unique).toBe('u-1')
  })

  it('throws when login "succeeds" but /who still returns no session', async () => {
    mocked.login.mockReturnValue(ok(undefined))
    mocked.who.mockRejectedValue(new Error('HTTP 401'))
    const session = useSession()
    await expect(session.login('admin', 'wrong')).rejects.toThrow('Login did not create a session')
    expect(session.isAuthenticated).toBe(false)
  })

  it('propagates a failed login request', async () => {
    mocked.login.mockRejectedValue(new Error('HTTP 500'))
    const session = useSession()
    await expect(session.login('admin', 'x')).rejects.toThrow('HTTP 500')
  })
})

describe('logout', () => {
  it('calls the server logout and clears the local user', async () => {
    mocked.who.mockReturnValue(ok(makeUser()))
    mocked.logout.mockReturnValue(ok(undefined))
    const session = useSession()
    await session.init()
    await session.logout()
    expect(mocked.logout).toHaveBeenCalledTimes(1)
    expect(session.user).toBeNull()
    expect(session.isAuthenticated).toBe(false)
  })

  it('clears the local user even when the server logout fails', async () => {
    mocked.who.mockReturnValue(ok(makeUser()))
    mocked.logout.mockRejectedValue(new Error('HTTP 503'))
    const session = useSession()
    await session.init()
    await expect(session.logout()).resolves.toBeUndefined()
    expect(session.user).toBeNull()
  })
})

describe('refresh', () => {
  it('renews the session token then re-fetches /who', async () => {
    mocked.who.mockReturnValue(ok(makeUser()))
    mocked.refresh.mockReturnValue(ok(undefined))
    const session = useSession()
    await session.init()
    await session.refresh()
    expect(mocked.refresh).toHaveBeenCalledTimes(1)
    expect(mocked.who).toHaveBeenCalledTimes(2)
  })
})

describe('transport auth-failure callback (401 after refresh)', () => {
  it('registers a failure handler when the store is created', () => {
    useSession()
    expect(mocked.authFailureHandler).toBeTypeOf('function')
  })

  it('the handler drops the local session back to unauthenticated', async () => {
    mocked.who.mockReturnValue(ok(makeUser()))
    const session = useSession()
    await session.init()
    expect(session.isAuthenticated).toBe(true)
    mocked.authFailureHandler!()
    expect(session.user).toBeNull()
    expect(session.isAuthenticated).toBe(false)
    expect(session.initialized).toBe(true)
  })
})

describe('useSession()', () => {
  it('is an alias of useSessionStore on the active pinia instance', () => {
    const a = useSession()
    const b = useSessionStore()
    expect(a).toBe(b)
  })
})
