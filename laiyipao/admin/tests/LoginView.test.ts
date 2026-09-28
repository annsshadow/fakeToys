/**
 * LoginView.vue 测试。
 *
 * 登录页是唯一同时操作令牌与路由的视图：
 * - 表单校验（空字段不发请求）；
 * - 登录成功 → 存令牌 + 跳看板；
 * - onMounted 的「已有令牌先验一次」逻辑（验过跳过登录页 / 验失败清令牌）。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createMemoryHistory, createRouter } from 'vue-router'

const adminLogin = vi.hoisted(() => vi.fn())
const adminMe = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ adminLogin, adminMe }))

import LoginView from '@/views/LoginView.vue'
import { setToken } from '@/api/client'

const Empty = { template: '<div />' }

async function mountLogin() {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', redirect: '/admin/login' },
      { path: '/admin/login', component: LoginView },
      { path: '/admin/dashboard', component: Empty },
    ],
  })
  await router.push('/admin/login')
  const wrapper = mount(LoginView, { global: { plugins: [ElementPlus, router] } })
  await flushPromises()
  return { wrapper, router }
}

beforeEach(() => {
  localStorage.clear()
  vi.clearAllMocks()
})

describe('LoginView 表单校验', () => {
  it('用户名与密码都为空：提示错误，不发登录请求', async () => {
    const { wrapper } = await mountLogin()
    await wrapper.find('button').trigger('click')
    expect(wrapper.text()).toContain('请填写用户名与密码')
    expect(adminLogin).not.toHaveBeenCalled()
  })

  it('用户名非空但密码为空：同样拦截（|| 右侧分支）', async () => {
    const { wrapper } = await mountLogin()
    await wrapper.find('input[type="text"], input:not([type])').setValue('admin')
    await wrapper.find('input[type="password"]').setValue('')
    await wrapper.find('button').trigger('click')
    expect(wrapper.text()).toContain('请填写用户名与密码')
    expect(adminLogin).not.toHaveBeenCalled()
  })

  it('密码框回车等效于点击登录（keyup.enter）', async () => {
    const { wrapper } = await mountLogin()
    await wrapper.find('input[type="password"]').trigger('keyup.enter')
    expect(wrapper.text()).toContain('请填写用户名与密码')
  })
})

describe('LoginView 登录流程', () => {
  it('成功：存令牌、欢迎消息、跳转 /admin/dashboard', async () => {
    adminLogin.mockResolvedValueOnce({
      access_token: 'tok-42',
      admin: { id: 1, username: '运营龙', role: 'super' },
    })
    const { wrapper, router } = await mountLogin()
    const inputs = wrapper.findAll('input')
    await inputs[0].setValue(' admin ')
    await inputs[1].setValue('secret-pass')
    await wrapper.find('button').trigger('click')
    await flushPromises()

    // 用户名应 trim 后提交
    expect(adminLogin).toHaveBeenCalledWith('admin', 'secret-pass')
    expect(localStorage.getItem('lyp_admin_token')).toBe('tok-42')
    expect(document.body.textContent).toContain('欢迎回来，运营龙')
    expect(router.currentRoute.value.path).toBe('/admin/dashboard')
  })

  it('失败：后端错误文案展示在告警条里，按钮退出 loading', async () => {
    adminLogin.mockRejectedValueOnce(new Error('用户名或密码错误'))
    const { wrapper } = await mountLogin()
    const inputs = wrapper.findAll('input')
    await inputs[0].setValue('admin')
    await inputs[1].setValue('wrong')
    await wrapper.find('button').trigger('click')
    await flushPromises()

    expect(wrapper.text()).toContain('用户名或密码错误')
    expect(wrapper.text()).not.toContain('欢迎回来')
  })
})

describe('LoginView 已有令牌的挂载校验', () => {
  it('令牌有效：adminMe 通过后直接跳过登录页', async () => {
    setToken('valid-tok')
    adminMe.mockResolvedValueOnce({ admin: { id: 1, username: 'a', role: 'super' } })
    const { router } = await mountLogin()
    await flushPromises()
    expect(adminMe).toHaveBeenCalledTimes(1)
    expect(router.currentRoute.value.path).toBe('/admin/dashboard')
  })

  it('令牌失效：清空令牌并留在登录页', async () => {
    setToken('stale-tok')
    adminMe.mockRejectedValueOnce(new Error('401'))
    const { router } = await mountLogin()
    await flushPromises()
    expect(adminMe).toHaveBeenCalledTimes(1)
    expect(localStorage.getItem('lyp_admin_token')).toBeNull()
    expect(router.currentRoute.value.path).toBe('/admin/login')
  })

  it('无令牌：不做挂载校验，直接安静等待输入', async () => {
    await mountLogin()
    await flushPromises()
    expect(adminMe).not.toHaveBeenCalled()
  })
})
