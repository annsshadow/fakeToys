/**
 * 移动端导航守卫单测。
 *
 * uni-app 无 vue-router：受保护页在 onShow 调 ensureAuthenticated()，
 * 未认证必须以 reLaunch 跳登录页（tabBar 页不能用 navigateTo 覆盖栈）。
 * mock 掉 session store 与全局 uni 导航，钉死跳转语义。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'

const mocked = vi.hoisted(() => {
  return {
    session: {
      init: vi.fn(),
      isAuthenticated: false,
      user: null,
      loading: false,
      initialized: false,
      login: vi.fn(),
      logout: vi.fn(),
      refresh: vi.fn(),
    },
  }
})

vi.mock('@/store/session', () => ({
  useSession: () => mocked.session,
  useSessionStore: () => mocked.session,
}))

import { ensureAuthenticated, HOME_PAGE, LOGIN_PAGE, navigateHome } from './auth-guard'

function installUni() {
  const stub = { reLaunch: vi.fn(), switchTab: vi.fn(), navigateTo: vi.fn() }
  vi.stubGlobal('uni', stub)
  return stub
}

afterEach(() => {
  vi.unstubAllGlobals()
  mocked.session.init.mockReset()
  mocked.session.init.mockResolvedValue(undefined)
  mocked.session.isAuthenticated = false
  mocked.session.user = null
})

describe('ensureAuthenticated', () => {
  it('returns true for an authenticated session without any navigation', async () => {
    const uniStub = installUni()
    mocked.session.isAuthenticated = true
    await expect(ensureAuthenticated()).resolves.toBe(true)
    expect(mocked.session.init).toHaveBeenCalledTimes(1)
    expect(uniStub.reLaunch).not.toHaveBeenCalled()
    expect(uniStub.switchTab).not.toHaveBeenCalled()
  })

  it('an unauthenticated session is reLaunched to the login page and returns false', async () => {
    const uniStub = installUni()
    await expect(ensureAuthenticated()).resolves.toBe(false)
    expect(uniStub.reLaunch).toHaveBeenCalledTimes(1)
    expect(uniStub.reLaunch).toHaveBeenCalledWith({ url: LOGIN_PAGE })
    // reLaunch（而非 navigateTo）：清掉页面栈，避免登录后回退到受保护页循环。
    expect(uniStub.navigateTo).not.toHaveBeenCalled()
  })

  it('initializes the session before deciding (401 后 /who 恢复机会)', async () => {
    installUni()
    await ensureAuthenticated()
    expect(mocked.session.init).toHaveBeenCalledTimes(1)
  })
})

describe('navigateHome', () => {
  it('switches to the tabbed home page with switchTab', () => {
    const uniStub = installUni()
    navigateHome()
    expect(uniStub.switchTab).toHaveBeenCalledTimes(1)
    expect(uniStub.switchTab).toHaveBeenCalledWith({ url: HOME_PAGE })
    // tabBar 页只能 switchTab 进入（uni-app 约定）。
    expect(uniStub.reLaunch).not.toHaveBeenCalled()
  })
})

describe('page constants', () => {
  it('LOGIN_PAGE / HOME_PAGE are pinned to the registered pages.json routes', () => {
    expect(LOGIN_PAGE).toBe('/pages/login/login')
    expect(HOME_PAGE).toBe('/pages/index/index')
  })
})
