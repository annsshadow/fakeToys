// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment jsdom
/**
 * createO2App 生命周期与认证守卫行为测试。
 *
 * 意图：createO2App 是 SDK 的组装入口，必须保证——
 * 1. 容器不存在时给出可定位的错误（而不是静默空应用）；
 * 2. 默认安装认证守卫：未登录用户被重定向到 Login 且保留 redirect 回跳参数；
 * 3. meta.requiresAuth=false 的公开路由不受守卫拦截；
 * 4. 已登录用户（who 返回带 unique 的用户）不被拦截；
 * 5. authGuard:false 时不安装守卫（公开路由保护可被显式关闭）；
 * 6. theme 选项应用到 document；getO2App/getO2Router 返回实际实例。
 *
 * fetch 按 URL 返回 who 结果，其余 URL 不会命中。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { createO2App, getO2App, getO2Router } from './app.js'
import { themeProvider } from './theme.js'

type WhoState = 'guest' | 'user'
let whoState: WhoState = 'guest'

// jsdom 不提供 fetch/Response；api.ts 只消费 ok/status/statusText/json()，
// 因此按 URL 返回最小对象即可（不依赖全局 Response 构造器）。
function fakeFetch(url: string, init?: RequestInit): Promise<unknown> {
  void init
  const path = new URL(url, 'http://localhost:3000').pathname
  if (path === '/api/authentication/who') {
    if (whoState === 'guest') {
      return Promise.resolve({
        ok: false,
        status: 401,
        statusText: 'Unauthorized',
        json: async () => ({ success: false, data: null }),
      })
    }
    return Promise.resolve({
      ok: true,
      status: 200,
      statusText: 'OK',
      json: async () => ({ success: true, data: { unique: 'alice', name: 'Alice' } }),
    })
  }
  return Promise.resolve({
    ok: true,
    status: 200,
    statusText: 'OK',
    json: async () => ({ success: true }),
  })
}

const loginComponent = { template: '<div>login</div>' }
const dashboardComponent = { template: '<div>dash</div>' }
const publicComponent = { template: '<div>pub</div>' }
const routes = [
  { path: '/login', name: 'Login', component: loginComponent },
  { path: '/dashboard', name: 'Dashboard', component: dashboardComponent },
  {
    path: '/public',
    name: 'Public',
    component: publicComponent,
    meta: { requiresAuth: false },
  },
]

describe('createO2App lifecycle', () => {
  beforeEach(() => {
    whoState = 'guest'
    vi.stubGlobal('fetch', fakeFetch)
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    document.body.innerHTML = ''
  })

  it('throws a locatable error when the container does not exist', () => {
    expect(() => createO2App('#missing', { routes })).toThrow('Container not found: #missing')
  })

  it('redirects unauthenticated users to Login with a redirect query', async () => {
    const root = document.createElement('div')
    root.id = 'root1'
    document.body.appendChild(root)

    const { router } = createO2App(root, { routes })
    await router.isReady()

    await router.push('/dashboard')
    expect(router.currentRoute.value.name).toBe('Login')
    expect(router.currentRoute.value.query).toEqual({ redirect: '/dashboard' })

    // 返回的是同一批活实例
    expect(getO2App()).toBeDefined()
    expect(getO2Router()).toBe(router)
  })

  it('leaves public routes untouched for unauthenticated users', async () => {
    const root = document.createElement('div')
    root.id = 'root2'
    document.body.appendChild(root)

    const { router } = createO2App(root, { routes })
    await router.isReady()

    await router.push('/public')
    expect(router.currentRoute.value.name).toBe('Public')
  })

  it('does not intercept authenticated users on guarded routes', async () => {
    whoState = 'user'
    const root = document.createElement('div')
    root.id = 'root3'
    document.body.appendChild(root)

    const { router } = createO2App(root, { routes })
    await router.isReady()

    await router.push('/dashboard')
    expect(router.currentRoute.value.name).toBe('Dashboard')
  })

  it('installs no guard when authGuard is explicitly false', async () => {
    const root = document.createElement('div')
    root.id = 'root4'
    document.body.appendChild(root)

    const { router } = createO2App(root, { routes, authGuard: false, theme: 'light' })
    await router.isReady()

    await router.push('/dashboard')
    // 无守卫：未登录也能到达受保护路由
    expect(router.currentRoute.value.name).toBe('Dashboard')

    // theme 选项落地到 document
    expect(document.documentElement.getAttribute('data-theme')).toBe('light')
    expect(themeProvider.theme.value).toBe('light')
  })
})
