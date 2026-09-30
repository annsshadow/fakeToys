/**
 * main.ts + App.vue 引导测试。
 *
 * main.ts 是纯启动胶水（createApp → 三件套 → mount），单独抽函数测是伪命题，
 * 因此直接在 jsdom 里执行它：断言真实挂载出了登录页（无令牌时守卫落点）。
 * App.vue 只提供路由出口，用真实路由渲染两种落点验证 <router-view /> 生效。
 */
import { flushPromises } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
vi.mock('@/api', () => ({
  adminLogin: vi.fn(),
  adminMe: vi.fn(),
  fetchDashboard: vi.fn().mockResolvedValue({
    users: { total: 0, guests: 0, wechat: 0, banned: 0, new_today: 0 },
    battles: { total: 0, today: 0, wins: 0, avg_duration_ms: 0 },
    progression: { avg_max_stage: 0, avg_power: 0, stage_100_clears: 0 },
    economy: { coin_in: 0, coin_out: 0, gem_in: 0, gem_out: 0, orders_paid: 0, orders_pending: 0 },
    verification: { checked: 0, matched: 0, mismatched: 0 },
    reaction_usage: [],
    daily_active: [],
    stage_funnel: [],
  }),
  fetchLevels: vi.fn(),
}))

// jsdom 无 canvas，真实 echarts 会在动画帧里崩溃；本文件只关心路由/挂载，不关心图表内容
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(() => ({ setOption: vi.fn(), dispose: vi.fn() })),
}))

describe('main.ts 启动引导', () => {
  it('挂载后 #app 内渲染出登录页（无令牌 → 守卫重定向到 /admin/login）', async () => {
    document.body.innerHTML = '<div id="app"></div>'
    localStorage.clear()
    await import('../src/main')
    await flushPromises()
    // 登录页是懒加载 chunk，覆盖率并行模式下解析可能慢于一轮微任务，轮询等待
    await vi.waitFor(
      () => {
        const app = document.querySelector('#app')
        expect(app?.children.length ?? 0).toBeGreaterThan(0)
        expect(app?.textContent ?? '').toContain('运营后台')
        expect(app?.textContent ?? '').toContain('登录')
      },
      { timeout: 15000 },
    )
  })
})

describe('App.vue 根组件', () => {
  it('提供路由出口：当前路由的组件被真实渲染', async () => {
    const { default: App } = await import('@/App.vue')
    const { router } = await import('@/router')
    const { mount } = await import('@vue/test-utils')
    const { setToken } = await import('@/api/client')
    const { default: ElementPlus } = await import('element-plus')

    localStorage.clear()
    // ⚠️ 必须装 Element Plus，与 `main.ts` 的 `app.use(ElementPlus)` 一致。
    //
    // 不装的话这一处会刷出 22 条「Failed to resolve component: el-*」警告。
    // 而这批噪声是有代价的：图标那个真缺陷（10 个图标全空、`Reading` 未解析）
    // 就一直**混在这类警告里**，没人分得清哪个是真问题。
    // 噪声归零之后，新的未解析组件警告才重新具备信号价值。
    const wrapper = mount(App, { global: { plugins: [router, ElementPlus] } })
    await flushPromises()
    // 无令牌：守卫把初始导航带到登录页
    await vi.waitFor(() => expect(wrapper.text()).toContain('默认账号'), { timeout: 15000 })

    // 有令牌：进入后台壳，看板真实渲染（fetchDashboard 已 mock）
    setToken('tok')
    await router.push('/admin/dashboard')
    await flushPromises()
    await vi.waitFor(() => expect(wrapper.text()).toContain('数据看板'), { timeout: 15000 })
    wrapper.unmount()
    localStorage.clear()
  })
})
