/**
 * 401 处理守卫（本轮安全审计新增）。
 *
 * # 为什么这条必须存在
 *
 * `api/client.ts` 此前**没有**任何 401 分支，而 `router/index.ts` 的注释
 * 声称「真正的 401 检测在 api/client 抛出」。
 *
 * 那是一条**虚构的安全网**：令牌过期（后台 access token 默认 2h）
 * 或管理员被停用时，路由守卫看到 `getToken()` 仍为真而放行，
 * 每个视图反复吃 401，运营**永远不会被提示重新认证**。
 *
 * # 判据
 *
 * 断言的是**可观测后果**（令牌被清、跳回登录），不是「调用过某个函数」——
 * 后者能被一个空实现骗过。
 */

import { beforeEach, describe, expect, it, vi } from 'vitest'

const TOKEN_KEY = 'lyp_admin_token'

describe('api client 401 处理', () => {
  beforeEach(() => {
    localStorage.clear()
    window.location.hash = ''
    vi.restoreAllMocks()
  })

  function mockFetch(status: number, body: unknown = { error: { code: 'unauthorized', message: 'nope' } }) {
    const spy = vi.fn().mockResolvedValue({
      ok: status >= 200 && status < 300,
      status,
      text: () => Promise.resolve(JSON.stringify(body)),
    })
    vi.stubGlobal('fetch', spy)
    return spy
  }

  it('业务端点 401 时清掉令牌并送回登录页', async () => {
    localStorage.setItem(TOKEN_KEY, 'expired-token')
    window.location.hash = '#/admin/dashboard'
    mockFetch(401)

    const { api, ApiError } = await import('@/api/client')
    await expect(api.get('/admin/me')).rejects.toBeInstanceOf(ApiError)

    expect(localStorage.getItem(TOKEN_KEY)).toBeNull()
    expect(window.location.hash).toBe('#/admin/login')
  })

  it('仍要抛出 ApiError —— 不能静默吞掉 401', async () => {
    localStorage.setItem(TOKEN_KEY, 'expired-token')
    mockFetch(401)

    const { api, ApiError } = await import('@/api/client')
    // 判据是「以 401 的错误抛出去」，不是「不抛」。
    // 一个 `return null` 的实现能让「不崩」绿，却会让每个调用方
    // 在 undefined 上炸 —— 症状更糟且更难定位。
    await expect(api.get('/admin/users')).rejects.toMatchObject({ status: 401 })
    expect(ApiError).toBeTruthy()
  })

  it('登录接口自己的 401 不跳转、不清令牌', async () => {
    // 「密码打错」必须显示成「密码打错」，
    // 而不是把用户踢回登录页让他看不出哪里错了。
    localStorage.setItem(TOKEN_KEY, 'stale-token')
    window.location.hash = '#/admin/login'
    mockFetch(401)

    const { api } = await import('@/api/client')
    await expect(api.post('/admin/login', { username: 'a', password: 'b' })).rejects.toBeTruthy()

    expect(localStorage.getItem(TOKEN_KEY)).toBe('stale-token')
    expect(window.location.hash).toBe('#/admin/login')
  })

  it('非 401 的错误不动令牌', async () => {
    localStorage.setItem(TOKEN_KEY, 'good-token')
    window.location.hash = '#/admin/dashboard'
    mockFetch(500, { error: { code: 'internal_error', message: 'boom' } })

    const { api } = await import('@/api/client')
    await expect(api.get('/admin/dashboard')).rejects.toBeTruthy()

    // 500 不是「你没登录」—— 把令牌清掉会让一次服务端抖动
    // 变成「所有人被踢出后台」。
    expect(localStorage.getItem(TOKEN_KEY)).toBe('good-token')
    expect(window.location.hash).toBe('#/admin/dashboard')
  })

  it('403 也不清令牌', async () => {
    // readonly 账号打写操作会拿到 403。那是**权限不足**，
    // 身份仍然有效 —— 清令牌会把「你被降权了」误报成「你掉线了」。
    localStorage.setItem(TOKEN_KEY, 'good-token')
    window.location.hash = '#/admin/users'
    mockFetch(403, { error: { code: 'readonly', message: 'no' } })

    const { api } = await import('@/api/client')
    await expect(api.post('/admin/users/1/ban', {})).rejects.toBeTruthy()

    expect(localStorage.getItem(TOKEN_KEY)).toBe('good-token')
  })

  it('网络错误（fetch 抛异常）不动令牌', async () => {
    localStorage.setItem(TOKEN_KEY, 'good-token')
    window.location.hash = '#/admin/dashboard'
    vi.stubGlobal(
      'fetch',
      vi.fn().mockRejectedValue(new Error('network down')),
    )

    const { api } = await import('@/api/client')
    await expect(api.get('/admin/dashboard')).rejects.toBeTruthy()

    // 服务端宕机 ≠ 令牌失效。把两者混起来的后果是
    // 「后端重启一次，后台全体掉线」。
    expect(localStorage.getItem(TOKEN_KEY)).toBe('good-token')
    expect(window.location.hash).toBe('#/admin/dashboard')
  })

  it('已在登录页时令牌照样清空', async () => {
    localStorage.setItem(TOKEN_KEY, 'expired-token')
    window.location.hash = '#/admin/login'
    mockFetch(401)

    const { api } = await import('@/api/client')
    await expect(api.get('/admin/dashboard')).rejects.toBeTruthy()

    // ⚠️ 诚实标注：这一条**实际守的是「令牌清空」**。
    //
    // 我原本把测试名写成「不重复改 hash」，但那一半在 jsdom 里
    // **观察不到** —— hash 本来就已经是登录页，重复赋同一个值
    // 不改变任何可观测状态。所以那个断言在「实现退化成无条件赋值」
    // 时照样会绿，而**名字却让人以为它有守卫**。
    //
    // 如实改名，而不是让测试名看起来比它实际守的更多。
    // 真要守「不重复改」需要路由跳转次数计数，那是另一个夹具的事。
    expect(localStorage.getItem(TOKEN_KEY)).toBeNull()
    expect(window.location.hash).toBe('#/admin/login')
  })
})