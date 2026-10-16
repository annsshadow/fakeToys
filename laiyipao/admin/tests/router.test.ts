/**
 * router/index.ts 测试。
 *
 * 路由表是「侧边栏 ↔ 页面」的唯一契约，守卫是后台安全的第一道门：
 * - 无令牌访问任何后台页 → 必须带 redirect 参数跳登录（登录后跳回原目标）；
 * - afterEach 兜底清理「导航途中令牌消失」的脏状态。
 */
import { flushPromises } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'

// 替身化 api 层：路由测试不关心 HTTP，但要保留真实的 getToken/setToken/
// confirmTokenValid（守卫依赖它们，且「验真」这一环必须走真代码）。
vi.mock('@/api/client', async (importOriginal) => {
  const mod = await importOriginal<typeof import('@/api/client')>()
  return { ...mod, api: { get: vi.fn(), post: vi.fn(), put: vi.fn(), del: vi.fn() } }
})

import { invalidateTokenVerification, setToken } from '@/api/client'
import { router } from '@/router'

const TOKEN_KEY = 'lyp_admin_token'

/**
 * 让守卫的令牌验真（GET /admin/me）返回指定状态码。
 *
 * ⚠️ 为什么路由测试要打桩 fetch：第 154 轮起守卫不再只看
 * 「localStorage 里有没有字符串」，而是真的问一次 `/admin/me`。
 * 不桩的话 jsdom 里会去连 http://localhost/api/v1 —— 网络失败
 * 被 `confirmTokenValid` 判为「令牌不可用」，于是每一个「有令牌应放行」
 * 的用例都会跳登录页，失败信息还指向路由表而不是网络。
 */
function stubMe(status: number) {
  const body =
    status >= 200 && status < 300
      ? JSON.stringify({ admin: { id: 1, username: 'a', role: 'super' } })
      : JSON.stringify({ error: { code: 'unauthorized', message: 'no' } })
  const spy = vi.fn().mockResolvedValue({
    ok: status >= 200 && status < 300,
    status,
    text: () => Promise.resolve(body),
  })
  vi.stubGlobal('fetch', spy)
  return spy
}

const ALL_PAGE_NAMES = [
  'login',
  'dashboard',
  'levels',
  'skills',
  'equipment',
  'users',
  'battles',
  'recomputes',
  'defenses',
  'economy',
  'content',
  'wiki',
] as const

beforeEach(async () => {
  localStorage.clear()
  // 验真结论缓存在模块内存里，跨用例会残留 —— 每个用例从「没验过」出发。
  invalidateTokenVerification()
  // 默认：令牌验得过。需要验真失败的用例自行覆盖。
  stubMe(200)
  // 回到登录页，保证每个用例从已知状态出发
  await router.push('/admin/login')
  localStorage.clear()
})

describe('路由表完整性', () => {
  it.each(ALL_PAGE_NAMES)('具名路由 %s 能解析出组件（懒加载 chunk 存在）', (name) => {
    const resolved = router.resolve({ name })
    expect(resolved.matched.length).toBeGreaterThan(0)
    expect(resolved.matched.at(-1)?.components?.default).toBeDefined()
  })

  it('/ 重定向到 /admin/dashboard（登录态下）', async () => {
    setToken('tok')
    await router.push('/')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/dashboard')
  })

  it('未知路径（catch-all）重定向到 /admin/dashboard，而不是白屏', async () => {
    setToken('tok')
    await router.push('/definitely/not/exist')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/dashboard')
  })

  it('admin 子路由挂在 /admin 布局下，wiki 在布局外（全宽文档站）', () => {
    const admin = router.resolve('/admin/levels')
    expect(admin.matched.map((m) => m.path)).toEqual(['/admin', '/admin/levels'])
    const wiki = router.resolve('/wiki')
    expect(wiki.matched.map((m) => m.path)).toEqual(['/wiki'])
  })

  it('每个页面路由都带 title 元信息（顶栏标题依赖它）', () => {
    for (const name of ALL_PAGE_NAMES) {
      const resolved = router.resolve({ name })
      expect(resolved.meta.title, `路由 ${name} 缺少 meta.title`).toBeTruthy()
    }
  })

  it('所有路由的组件都可解析：懒加载 chunk 存在且默认导出有效', async () => {
    // 说明：vue-router 在导航后会用已解析组件替换懒加载工厂，因此这里
    // 对「仍是函数的」真实执行工厂，对「已是组件的」验证非空 —— 两者都必须有效。
    let lazyCount = 0
    for (const rec of router.getRoutes()) {
      const comp = rec.components?.default as unknown
      if (!comp) continue
      if (typeof comp === 'function') {
        const mod = await (comp as () => Promise<{ default?: unknown }>)()
        expect(mod?.default, `路由 ${rec.path} 的懒加载组件缺默认导出`).toBeTruthy()
        lazyCount++
      } else {
        expect(comp, `路由 ${rec.path} 的组件为空`).toBeTruthy()
      }
    }
    // 至少覆盖剩余未导航过的懒加载工厂（skills/equipment/... /wiki 等）
    expect(lazyCount).toBeGreaterThan(5)
  })
})

describe('全局守卫 beforeEach', () => {
  it('无令牌访问后台页 → 跳登录，并通过 redirect 参数记住目标', async () => {
    await router.push('/admin/users')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/login')
    expect(router.currentRoute.value.query.redirect).toBe('/admin/users')
  })

  it('无令牌访问文档站 /wiki 也要求登录（config 公开但看板数据不公开）', async () => {
    await router.push('/wiki')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/login')
    expect(router.currentRoute.value.query.redirect).toBe('/wiki')
  })

  it('登录页本身无令牌也可进入（否则死循环）', async () => {
    await router.push('/admin/login')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/login')
    expect(router.currentRoute.value.query.redirect).toBeUndefined()
  })

  it('有令牌且验真通过时放行所有页面', async () => {
    setToken('tok')
    await router.push('/admin/economy')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/economy')
  })

  // ── 第 154 轮：令牌存在 ≠ 令牌有效 ──────────────────────────────

  it('伪造令牌（localStorage 里塞任意字符串）不得进入后台壳', async () => {
    // 复现路径：浏览器控制台执行
    //   localStorage.setItem('lyp_admin_token','x')
    // 原守卫只看真值就放行 —— 组件照样加载、11 个视图全部可达，
    // 只是每个请求 401。它不是服务端越权（requireAdmin 每次查库），
    // 但「放行一个已知无效的会话」本身就是守卫的失职。
    setToken('forged-token')
    stubMe(401)
    await router.push('/admin/levels')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/login')
    expect(router.currentRoute.value.query.redirect).toBe('/admin/levels')
  })

  it('验真失败时令牌被清掉（不是只跳页，凭证不留磁盘）', async () => {
    setToken('expired-token')
    stubMe(401)
    await router.push('/admin/users')
    await flushPromises()
    expect(localStorage.getItem(TOKEN_KEY)).toBeNull()
  })

  it('验真结论在同一次会话内缓存：连续导航只问一次 /admin/me', async () => {
    // 若每次导航都验，11 个视图切换 = 11 次额外请求 ——
    // 缓存不是优化细节，是让这个守卫本身可用。
    setToken('tok')
    const spy = stubMe(200)
    await router.push('/admin/levels')
    await flushPromises()
    await router.push('/admin/skills')
    await flushPromises()
    await router.push('/admin/economy')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/economy')
    expect(spy).toHaveBeenCalledTimes(1)
  })

  it('/wiki 不额外验真：它读的是公开 /config，不该因令牌过期闪断', async () => {
    // ⚠️ 这条钉住一个**刻意的不对称**：/admin/* 验真，/wiki 不验。
    // wiki 的正文来自公开端点 /api/v1/config；只有右上角的使用统计
    // 来自 /admin/dashboard（失败时被 catch 成空）。
    // 让令牌过期的运营看不了公开文档，收益为零而代价是打断阅读。
    setToken('ancient-token')
    const spy = stubMe(401)
    await router.push('/wiki')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/wiki')
    expect(spy).not.toHaveBeenCalled()
  })
})

describe('全局守卫 afterEach（令牌失效兜底）', () => {
  it('导航结束时若已不在登录页且令牌消失 → 清理令牌残留', async () => {
    setToken('tok')
    // 模拟真实场景：导航发起时令牌还在（beforeEach 放行），
    // 随后被 api 层的 401 处理清掉 —— afterEach 必须把脏状态清干净
    const stop = router.beforeEach(() => {
      localStorage.removeItem('lyp_admin_token')
      return true
    })
    try {
      await router.push('/admin/dashboard')
      await flushPromises()
      expect(router.currentRoute.value.path).toBe('/admin/dashboard')
      expect(localStorage.getItem('lyp_admin_token')).toBeNull()
    } finally {
      stop()
    }
  })

  it('在登录页上不触发清理（登录页本身就不需要令牌）', async () => {
    setToken('stale-token')
    await router.push('/admin/login')
    await flushPromises()
    expect(localStorage.getItem('lyp_admin_token')).toBe('stale-token')
  })

  it('令牌健在的正常导航不清理', async () => {
    setToken('tok')
    await router.push('/admin/dashboard')
    await flushPromises()
    expect(localStorage.getItem('lyp_admin_token')).toBe('tok')
  })
})
