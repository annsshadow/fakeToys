// @vitest-environment jsdom
/**
 * 登录页（login.vue）组件测试。
 *
 * 覆盖：用户名回填、口令**不**预填、空表单守卫、登录成功后的跳转、
 * 登录失败的错误展示。
 *
 * # 为什么「口令不预填」也要测
 *
 * 预填口令等于把口令打进前端产物 —— 那份产物是公开可下载的，
 * 于是运营超管的口令（它同时是玩家测试账号的口令）就公开了。
 * 这条断言是防止有人「为了方便」把口令写回默认值。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Login from './login.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  loadLastUsername: vi.fn(() => ''),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  mockApi.loadLastUsername.mockReturnValue('')
  // 登录成功后 store 还会拉配置与档案 —— 这几个必须有返回值，
  // 否则它们抛错会把「登录成功」变成「登录失败」。
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig(),
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 30, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('login.vue 登录页', () => {
  /** 触发 onLoad 并等 DOM 更新（回填是响应式的）。 */
  async function fireOnLoad(wrapper: ReturnType<typeof mountPage>['wrapper']) {
    triggerUniHook(wrapper.vm, 'onLoad')
    await flushPromises()
  }

  it('用户名回填上次用过的；口令始终为空', async () => {
    mockApi.loadLastUsername.mockReturnValue('admin')
    const { wrapper } = mountPage(Login)
    await flushPromises()
    await fireOnLoad(wrapper)

    const inputs = wrapper.findAll('input')
    expect(inputs.length).toBe(2)
    // 用户名回填
    expect((inputs[0].element as HTMLInputElement).value).toBe('admin')
    // ⚠️ 口令**不预填** —— 前端产物是公开的，预填等于公开口令。
    expect((inputs[1].element as HTMLInputElement).value).toBe('')
  })

  it('没有上次用户名时用 admin 作为默认值（用户名不是秘密）', async () => {
    mockApi.loadLastUsername.mockReturnValue('')
    const { wrapper } = mountPage(Login)
    await flushPromises()
    await fireOnLoad(wrapper)

    const inputs = wrapper.findAll('input')
    expect((inputs[0].element as HTMLInputElement).value).toBe('admin')
  })

  it('空表单点登录 → 提示补全，不发请求', async () => {
    const { wrapper } = mountPage(Login)
    await flushPromises()
    await fireOnLoad(wrapper)

    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()

    expect(mockApi.login).not.toHaveBeenCalled()
    expect(wrapper.text()).toContain('请填写用户名与口令')
  })

  it('登录成功 → 跳回首页（清栈）', async () => {
    mockApi.login.mockResolvedValue({
      access_token: 'acc',
      refresh_token: 'ref',
      expires_at: '2099-01-01T00:00:00Z',
      user: { id: 7, nickname: 'admin', avatar_url: '', is_guest: false, status: 1 },
    })
    const { wrapper, store } = mountPage(Login)
    await flushPromises()
    await fireOnLoad(wrapper)

    const inputs = wrapper.findAll('input')
    await inputs[0].setValue('admin')
    await inputs[1].setValue('the-password')
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()

    expect(mockApi.login).toHaveBeenCalledWith('admin', 'the-password')
    expect(store.loggedIn).toBe(true)
    // reLaunch（不是 navigateTo）：登录页是闸门，回栈里不能留着它。
    expect(um.mock.reLaunch).toHaveBeenCalledWith({ url: '/pages/index/index' })
  })

  it('登录失败 → 显示服务端给的错误文案', async () => {
    mockApi.login.mockRejectedValue(new Error('用户名或密码错误'))
    const { wrapper, store } = mountPage(Login)
    await flushPromises()
    await fireOnLoad(wrapper)

    const inputs = wrapper.findAll('input')
    await inputs[0].setValue('admin')
    await inputs[1].setValue('wrong')
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()

    expect(store.loggedIn).toBe(false)
    expect(wrapper.text()).toContain('用户名或密码错误')
    expect(um.mock.reLaunch).not.toHaveBeenCalled()
  })
})
