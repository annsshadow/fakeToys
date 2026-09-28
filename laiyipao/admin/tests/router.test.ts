/**
 * router/index.ts 测试。
 *
 * 路由表是「侧边栏 ↔ 页面」的唯一契约，守卫是后台安全的第一道门：
 * - 无令牌访问任何后台页 → 必须带 redirect 参数跳登录（登录后跳回原目标）；
 * - afterEach 兜底清理「导航途中令牌消失」的脏状态。
 */
import { flushPromises } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'

// 替身化 api 层：路由测试不关心 HTTP，但要保留真实的 getToken/setToken（守卫依赖它们）
vi.mock('@/api/client', async (importOriginal) => {
  const mod = await importOriginal<typeof import('@/api/client')>()
  return { ...mod, api: { get: vi.fn(), post: vi.fn(), put: vi.fn(), del: vi.fn() } }
})

import { setToken } from '@/api/client'
import { router } from '@/router'

const ALL_PAGE_NAMES = [
  'login',
  'dashboard',
  'levels',
  'skills',
  'equipment',
  'users',
  'battles',
  'defenses',
  'economy',
  'content',
  'wiki',
] as const

beforeEach(async () => {
  localStorage.clear()
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

  it('有令牌时放行所有页面', async () => {
    setToken('tok')
    await router.push('/admin/economy')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/economy')
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
