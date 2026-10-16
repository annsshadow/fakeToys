/**
 * AdminLayout 的登出入口（第 155 轮）测试。
 *
 * 登出按钮是**服务端真切断**的入口，不只是清 localStorage。
 * 判据落在「点了按钮之后 adminLogout 被调用 + 令牌被清 + 跳回登录页」，
 * 不落在「localStorage 没了」（那只是 1/3）。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { describe, expect, it, vi, beforeEach } from 'vitest'
import { createMemoryHistory, createRouter } from 'vue-router'

const adminLogout = vi.hoisted(() => vi.fn())
const setToken = vi.hoisted(() => vi.fn())
const invalidateTokenVerification = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ adminLogout }))
vi.mock('@/api/client', () => ({
  setToken,
  invalidateTokenVerification,
}))

import AdminLayout from '@/layouts/AdminLayout.vue'

const Empty = { template: '<div />' }

beforeEach(() => {
  adminLogout.mockReset()
  setToken.mockReset()
  invalidateTokenVerification.mockReset()
})

function mountLayout() {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/admin/dashboard', component: Empty },
      { path: '/admin/login', component: Empty },
    ],
  })
  const wrapper = mount(AdminLayout, {
    global: { plugins: [ElementPlus, router] },
  })
  return { wrapper, router }
}

describe('AdminLayout 登出入口', () => {
  it('顶栏有「登出」按钮', async () => {
    const { wrapper, router } = mountLayout()
    await router.push('/admin/dashboard')
    await flushPromises()
    const btn = wrapper.find('.logout-btn')
    expect(btn.exists()).toBe(true)
    expect(btn.text()).toContain('登出')
  })

  it('点击登出：调用服务端登出 + 清令牌 + 作废验真缓存 + 跳登录页', async () => {
    adminLogout.mockResolvedValue({ revoked_sessions: 1 })
    const { wrapper, router } = mountLayout()
    await router.push('/admin/dashboard')
    await flushPromises()
    await wrapper.find('.logout-btn').trigger('click')
    await flushPromises()

    expect(adminLogout).toHaveBeenCalledTimes(1)
    expect(setToken).toHaveBeenCalledWith('')
    expect(invalidateTokenVerification).toHaveBeenCalled()
    expect(router.currentRoute.value.path).toBe('/admin/login')
  })

  it('服务端登出失败（网络错误）时仍清本地令牌并跳转（不阻塞登出体验）', async () => {
    adminLogout.mockRejectedValue(new Error('net'))
    const { wrapper, router } = mountLayout()
    await router.push('/admin/dashboard')
    await flushPromises()
    await wrapper.find('.logout-btn').trigger('click')
    await flushPromises()

    // 服务端吊销失败 ≠ 前端不能清本地：仍要清 + 跳，
    // 否则运营会卡在「点了没反应」。
    expect(setToken).toHaveBeenCalledWith('')
    expect(router.currentRoute.value.path).toBe('/admin/login')
  })
})
