/**
 * LoginView.vue 测试。
 *
 * 登录页是唯一同时操作令牌与路由的视图：
 * - 表单校验（空字段不发请求）；
 * - 登录成功 → 存令牌 + 跳看板；
 * - onMounted 的「已有令牌先验一次」逻辑（验过跳过登录页 / 验失败清令牌）。
 *
 * ⚠️ 第 154 轮：挂载校验从「调用 `adminMe`」改成「调用 `confirmTokenValid`」，
 * 因此这一组用例的桩打在 **fetch** 上而不是 `@/api` 上 ——
 * 验真必须走真实的 client（含它的 401 清理），否则「验失败清令牌」
 * 那一半就成了对被 mock 掉的函数的断言，什么都不守。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createMemoryHistory, createRouter } from 'vue-router'

const adminLogin = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ adminLogin }))

import LoginView from '@/views/LoginView.vue'
import { invalidateTokenVerification, setToken } from '@/api/client'

const Empty = { template: '<div />' }

/** 桩掉 /admin/me 的响应（守卫与登录页共用同一条验真路径）。 */
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
  invalidateTokenVerification()
  vi.clearAllMocks()
  stubMe(200)
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
  it('令牌有效：/admin/me 通过后直接跳过登录页', async () => {
    setToken('valid-tok')
    const me = stubMe(200)
    const { router } = await mountLogin()
    await flushPromises()
    expect(me).toHaveBeenCalledTimes(1)
    expect(me.mock.calls[0][0]).toBe('/api/v1/admin/me')
    expect(router.currentRoute.value.path).toBe('/admin/dashboard')
  })

  it('令牌失效：401 → 清空令牌并留在登录页', async () => {
    setToken('stale-tok')
    stubMe(401)
    const { router } = await mountLogin()
    await flushPromises()
    expect(localStorage.getItem('lyp_admin_token')).toBeNull()
    expect(router.currentRoute.value.path).toBe('/admin/login')
  })

  it('无令牌：连一个请求都不发，安静等待输入', async () => {
    const me = stubMe(200)
    await mountLogin()
    await flushPromises()
    expect(me).not.toHaveBeenCalled()
  })
})

describe('LoginView 消费 ?redirect=（第 155 轮）', () => {
  // 守卫 toLogin(to) 把目标路径写进 ?redirect=，登录成功应跳回。
  // 但 redirect 是用户可改的 query 参数，必须限制在 /admin 前缀下 ——
  // 否则 /admin/login?redirect=https://evil.com 就是开放重定向。

  it('合法的 /admin 前缀 redirect：登录后跳回原目标', async () => {
    adminLogin.mockResolvedValueOnce({
      access_token: 'tok-r1',
      admin: { id: 1, username: 'a', role: 'admin' },
    })
    const { wrapper, router } = await mountWithQuery('/admin/login', { redirect: '/admin/levels' })
    await submitVia(wrapper)
    expect(router.currentRoute.value.path).toBe('/admin/levels')
  })

  it('外部 URL 的 redirect 被拒绝（开放重定向防护），落到看板', async () => {
    adminLogin.mockResolvedValueOnce({
      access_token: 'tok-r2',
      admin: { id: 1, username: 'a', role: 'admin' },
    })
    const { wrapper, router } = await mountWithQuery('/admin/login', {
      redirect: 'https://evil.example.com/admin/levels',
    })
    await submitVia(wrapper)
    expect(router.currentRoute.value.path).toBe('/admin/dashboard')
  })

  it('协议相对 //evil.com 的 redirect 同样被拒绝', async () => {
    adminLogin.mockResolvedValueOnce({
      access_token: 'tok-r3',
      admin: { id: 1, username: 'a', role: 'admin' },
    })
    const { wrapper, router } = await mountWithQuery('/admin/login', { redirect: '//evil.example.com' })
    await submitVia(wrapper)
    expect(router.currentRoute.value.path).toBe('/admin/dashboard')
  })
})

/** 填表并提交（登录页 input 全局唯一，直接操作）。 */
async function submitVia(wrapper: ReturnType<typeof mount>) {
  const inputs = wrapper.findAll('input')
  await inputs[0].setValue('admin')
  await inputs[1].setValue('secret')
  await wrapper.find('button').trigger('click')
  await flushPromises()
}

/** 带 query 挂一次登录页（返回 wrapper 与 router，供填表断言跳转）。 */
async function mountWithQuery(path: string, query: Record<string, string>) {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', redirect: '/admin/login' },
      { path: '/admin/login', component: LoginView },
      { path: '/admin/dashboard', component: Empty },
      { path: '/admin/levels', component: Empty },
    ],
  })
  await router.push({ path, query })
  const wrapper = mount(LoginView, { global: { plugins: [ElementPlus, router] } })
  await flushPromises()
  return { wrapper, router }
}
