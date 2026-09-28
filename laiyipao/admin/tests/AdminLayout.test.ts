/**
 * AdminLayout.vue 测试。
 *
 * 布局壳承担两个契约：
 * 1. 侧边栏 9 个入口分组与顺序（「路由表 ↔ 菜单」一一对应的另一半）；
 * 2. 顶栏标题跟随路由 meta.title，缺失时回退品牌名。
 * 使用独立内存路由，避免拉起真实子页面的数据加载。
 */
import { flushPromises, mount } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import { createMemoryHistory, createRouter, type RouteRecordRaw } from 'vue-router'

// 图标在产品代码里按名字解析（<component :is="item.icon" /> / <Reading />），
// 测试环境没有全局注册，这里按同名注册空壳，聚焦布局本身
function iconStub(name: string) {
  return { name, render: () => null }
}
const iconComponents = Object.fromEntries(
  ['DataLine', 'Grid', 'MagicStick', 'Briefcase', 'User', 'Aim', 'Shield', 'Coin', 'Bell', 'Reading'].map(
    (n) => [n, iconStub(n)],
  ),
)

const Empty = { template: '<div data-test="empty-page" />' }

function makeRouter(initial: string) {
  const routes: RouteRecordRaw[] = [
    { path: '/', redirect: initial },
    { path: '/wiki', component: Empty },
    { path: '/admin', component: Empty },
    { path: '/admin/dashboard', component: Empty, meta: { title: '数据看板' } },
    { path: '/admin/levels', component: Empty, meta: { title: '关卡配置' } },
    // 故意不带 meta.title：验证 currentTitle 的品牌名回退分支
    { path: '/no-title', component: Empty },
    { path: '/:pathMatch(.*)*', component: Empty },
  ]
  return createRouter({ history: createMemoryHistory(), routes })
}

async function mountLayout(initial: string) {
  const router = makeRouter(initial)
  const { default: AdminLayout } = await import('@/layouts/AdminLayout.vue')
  const wrapper = mount(AdminLayout, {
    global: { plugins: [router], components: iconComponents },
  })
  await router.push(initial)
  await flushPromises()
  return { wrapper, router }
}

describe('AdminLayout 侧边栏', () => {
  it('渲染 4 个分组共 9 个后台入口，顺序与产品信息架构一致', async () => {
    const { wrapper } = await mountLayout('/admin/dashboard')
    const groups = wrapper.findAll('.nav-group')
    expect(groups.map((g) => g.find('.nav-group-title').text())).toEqual([
      '运营概览',
      '游戏内容',
      '玩家与战斗',
      '商业化与运营',
    ])
    const items = wrapper.findAll('.nav-item')
    expect(items).toHaveLength(9)
    expect(items.map((i) => i.text())).toEqual([
      '数据看板',
      '关卡配置',
      '技能与配方',
      '装备宝石皮肤',
      '用户管理',
      '战斗记录与验真',
      '防线值守',
      '经济与商城',
      '公告与兑换码',
    ])
  })

  it('当前路由对应的菜单项高亮（active class 跟随 route.path）', async () => {
    const { wrapper, router } = await mountLayout('/admin/dashboard')
    const active = () => wrapper.findAll('.nav-item.active').map((i) => i.text())
    expect(active()).toEqual(['数据看板'])

    await router.push('/admin/levels')
    await flushPromises()
    expect(active()).toEqual(['关卡配置'])
  })

  it('侧边栏底部固定提供文档站入口 /wiki', async () => {
    const { wrapper } = await mountLayout('/admin/dashboard')
    const link = wrapper.find('.wiki-link')
    expect(link.attributes('href')).toBe('/wiki')
    expect(link.text()).toContain('玩家玩法文档站')
  })
})

describe('AdminLayout 顶栏', () => {
  it('标题取自路由 meta.title 并随路由切换', async () => {
    const { wrapper, router } = await mountLayout('/admin/dashboard')
    expect(wrapper.find('.topbar-title').text()).toBe('数据看板')
    await router.push('/admin/levels')
    await flushPromises()
    expect(wrapper.find('.topbar-title').text()).toBe('关卡配置')
  })

  it('路由没有 meta.title 时回退为品牌名「来一炮」', async () => {
    const { wrapper } = await mountLayout('/no-title')
    expect(wrapper.find('.topbar-title').text()).toBe('来一炮')
  })

  it('默认展示后端地址标签（VITE_API_BASE 未配置时的开发默认值）', async () => {
    const { wrapper } = await mountLayout('/admin/dashboard')
    expect(wrapper.find('.topbar-right').text()).toContain('API 127.0.0.1:8080')
  })

  it('配置了 VITE_API_BASE 时标签显示配置值', async () => {
    vi.stubEnv('VITE_API_BASE', 'https://ops.example.com/api/v1')
    vi.resetModules()
    try {
      const { default: AdminLayout } = await import('@/layouts/AdminLayout.vue')
      const router = makeRouter('/admin/dashboard')
      const wrapper = mount(AdminLayout, {
        global: { plugins: [router], components: iconComponents },
      })
      await router.push('/admin/dashboard')
      await flushPromises()
      expect(wrapper.find('.topbar-right').text()).toContain('API https://ops.example.com/api/v1')
      wrapper.unmount()
    } finally {
      vi.unstubAllEnvs()
      vi.resetModules()
    }
  })
})
