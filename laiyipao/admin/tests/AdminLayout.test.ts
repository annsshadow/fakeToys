/**
 * AdminLayout.vue 测试。
 *
 * 布局壳承担三个契约：
 * 1. 侧边栏 9 个入口分组与顺序（「路由表 ↔ 菜单」一一对应的另一半）；
 * 2. 顶栏标题跟随路由 meta.title，缺失时回退品牌名；
 * 3. **10 个图标都真的画出来了**。
 *
 * ## 关于第 3 条：这里原本有一份替身列表，等于把缺陷写成了「环境限制」
 *
 * 第一版这个文件注册了一份**硬编码的图标名空壳**：
 *
 *   const iconComponents = Object.fromEntries(
 *     ['DataLine','Grid','MagicStick','Briefcase','User','Aim','Shield','Coin','Bell','Reading']
 *       .map((n) => [n, { name: n, render: () => null }]))
 *   // 注释写：「图标在产品代码里按名字解析，测试环境没有全局注册，
 *   //          这里按同名注册空壳，聚焦布局本身」
 *
 * 但**产品里同样没有注册** —— `main.ts` 只 `app.use(ElementPlus)`
 * （那是组件，不是图标包），`vite.config.ts` 里只有 `vue()`，
 * 没配 devDeps 里那个 `unplugin-vue-components`。
 *
 * 实测（不注册任何图标直接挂载）：`el-icon` 外壳 10 个、**`svg` 0 个**，
 * 未解析的名字被降级成 `<data-line>` / `<reading>` 这种无意义自定义元素。
 * **侧边栏 10 个图标全是空的。**
 *
 * 那份替身列表让测试永远绿，代价是**永远抓不到这件事**：
 * 名字写错（列表里有 `Shield`，而图标包 293 个导出里根本没有 `Shield`）
 * 测试也不会红。
 *
 * 所以现在：① 不再注册替身，② 挂载时装真 Element Plus（与 `main.ts` 一致），
 * ③ 断言真的渲染出 10 个 `<svg>`。
 */
import { flushPromises } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import { createMemoryHistory, createRouter, type RouteRecordRaw } from 'vue-router'
import { mountEl } from './helpers'

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
  // ⚠️ 必须装真 Element Plus（`mountEl` 做的就是这个），与 `main.ts` 一致。
  // 不装的话 `el-icon` 自己也解析不了，图标守卫就退化成
  // 「el-icon 解析了吗」，而查不到图标本身。
  const wrapper = mountEl(AdminLayout, { global: { plugins: [router] } })
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

  it('10 个图标全部真的画出来了（9 个导航项 + 1 个文档站入口）', async () => {
    // 这条守的是「图标解析失败 → 退化成 <data-line>/<reading> 空元素」这个形状。
    //
    // 判据选 **`<svg>` 计数**而不是「有没有警告」：
    //   - Element Plus 的图标一律渲染成 `<svg>`，数量对得上就说明解析成功；
    //   - 警告在测试输出里是噪声（实测一次跑出 136 条），没人会去看；
    //   - 而「未解析的名字被降级成自定义元素」是**静默**的 ——
    //     页面照样能看（文字标签都在），只是图标位置空着。
    //
    // 顺带钉住「不许再有降级残留」：Vue 把未解析组件渲染成小写标签，
    // 修复前实测有 1 个 `<reading>`。
    const { wrapper } = await mountLayout('/admin/dashboard')

    expect(wrapper.findAll('.nav-item')).toHaveLength(9)
    expect(wrapper.findAll('.el-icon')).toHaveLength(10)
    // 9 个导航项 + 底部文档站链接各一个图标
    expect(wrapper.findAll('svg')).toHaveLength(10)
    // 未解析的组件名会被降级成小写自定义元素，这里必须一个都没有
    expect(wrapper.findAll('reading')).toHaveLength(0)
    expect(wrapper.findAll('data-line')).toHaveLength(0)
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
      const wrapper = mountEl(AdminLayout, { global: { plugins: [router] } })
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
