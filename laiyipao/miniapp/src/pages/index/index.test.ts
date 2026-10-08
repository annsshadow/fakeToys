// @vitest-environment jsdom
/**
 * 首页（index.vue）组件测试。
 *
 * mock 掉 @/api/client 与全局 uni，把页面放进真实 Pinia + jsdom 里挂载，
 * 驱动模板三态（登录失败 / 配置加载中 / 正常）与全部交互分支：
 * onMounted 自动登录、onShow 刷新档案、菜单与主行动的导航参数。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount, flushPromises, type VueWrapper } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import Index from './index.vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { makeConfig } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>
let wrapper: VueWrapper<any>
let pinia: ReturnType<typeof createPinia>

function mountPage() {
  pinia = createPinia()
  setActivePinia(pinia)
  wrapper = mount(Index, { global: { plugins: [pinia] } })
  return wrapper
}

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '游客7', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfig.mockResolvedValue(makeConfig())
  mockApi.fetchWallet.mockResolvedValue({ coin: 10, gem: 20, energy: 30, keys: 40 })
  mockApi.fetchMe.mockResolvedValue({
    user_id: 7,
    build: null,
    build_rating: { total: 42, element_coverage: 5, reaction_coverage: 7, mastery_done: 8, equipment_synergy: 6, mechanic_depth: 4, weaknesses: ['缺少冰系'] },
    power: 9,
  } as any)
})

afterEach(() => {
  wrapper?.unmount()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('index.vue 首页', () => {
  it('挂载时未登录 → 自动登录并渲染正常态（资源栏 / 五维 / 菜单 / 元素说明）', async () => {
    mountPage()
    await flushPromises()
    const store = useGameStore()
    expect(store.loggedIn).toBe(true)
    expect(wrapper.text()).toContain('游客7')
    expect(wrapper.text()).toContain('游客账号')
    expect(wrapper.text()).toContain('金 10')
    // buildRating 已下发：五维与短板建议出现
    expect(wrapper.text()).toContain('构筑评分')
    expect(wrapper.text()).toContain('5/5')
    expect(wrapper.text()).toContain('搭配建议')
    expect(wrapper.text()).toContain('缺少冰系')
    // 第 138 轮：五维必须全（此前漏 mechanic_depth 只有 4 条）
    expect(wrapper.findAll('.dim-row').length).toBe(5)
    expect(wrapper.text()).toContain('机制深度')
    expect(wrapper.text()).toContain('4/8') // mechanic_depth=4 → 4/8
    // 装备契合分母是 18（服务端上界），不是误写的 6
    expect(wrapper.text()).toContain('6/18')
    // 7 个功能入口
    expect(wrapper.findAll('.grid-item').length).toBe(7)
    // 5 种元素说明
    expect(wrapper.findAll('.elem-chip').length).toBe(5)
  })

  it('登录失败 → 错误卡片 + 重试按钮；点击重试重新登录', async () => {
    mockApi.guestLogin.mockRejectedValue(new Error('无法连接服务器'))
    mountPage()
    await flushPromises()
    expect(wrapper.text()).toContain('无法连接服务器')
    expect(wrapper.text()).toContain('cd server && go run ./cmd/api')

    mockApi.guestLogin.mockResolvedValue({
      access_token: 'a',
      refresh_token: 'r',
      expires_at: '',
      user: { id: 1, nickname: 'n', avatar_url: '', is_guest: true, status: 1 },
    })
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()
    // 重试成功后回到正常态
    expect(wrapper.text()).toContain('构筑评分')
    expect(mockApi.guestLogin).toHaveBeenCalledTimes(2)
  })

  it('configLoading 中 → 显示加载文案（v-else-if 分支）', async () => {
    // 先把 store 置为「已登录 + 配置加载中」再挂载，避免 onMounted 的
    // 登录流程在 flush 时把 configLoading 复位成 false
    pinia = createPinia()
    setActivePinia(pinia)
    const store = useGameStore()
    store.loggedIn = true
    store.configLoading = true
    wrapper = mount(Index, { global: { plugins: [pinia] } })
    await flushPromises()
    expect(wrapper.text()).toContain('正在加载游戏配置…')
  })

  it('微信账号（isGuest=false）→ 顶栏显示微信账号（isGuest 三元分支）', async () => {
    pinia = createPinia()
    setActivePinia(pinia)
    const store = useGameStore()
    store.loggedIn = true
    store.isGuest = false
    store.nickname = '微信玩家'
    wrapper = mount(Index, { global: { plugins: [pinia] } })
    await flushPromises()
    expect(wrapper.text()).toContain('微信账号')
    expect(wrapper.text()).toContain('微信玩家')
  })

  it('buildRating 缺失 → 五维显示默认 0 且没有短板建议（?? 分支）', async () => {
    mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
    mountPage()
    await flushPromises()
    expect(wrapper.text()).toContain('0/5')
    expect(wrapper.text()).toContain('0/7')
    expect(wrapper.text()).toContain('0/8')
    expect(wrapper.find('.weakness-box').exists()).toBe(false)
  })

  it('equipment_synergy 超过 18 → 进度条宽度夹到 100%', async () => {
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: null,
      build_rating: { total: 1, element_coverage: 0, reaction_coverage: 0, mastery_done: 0, equipment_synergy: 99, mechanic_depth: 0, weaknesses: [] },
      power: 9,
    } as any)
    mountPage()
    await flushPromises()
    const fills = wrapper.findAll('.bar-fill')
    expect(fills.length).toBe(5) // 第 138 轮起五维齐全（原 4）
    // 第 4 维（索引 3）是装备契合：99/18 → 550% → 夹到 100%
    expect(fills[3]!.attributes('style')).toContain('width: 100%')
  })

  it('点击功能入口 → uni.navigateTo 携带对应路径', async () => {
    mountPage()
    await flushPromises()
    await wrapper.findAll('.grid-item')[2]!.trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/mastery/mastery' })
  })

  it('点击主行动卡 → 跳转下一关（unlockedLevel = maxStage + 1）', async () => {
    mountPage()
    await flushPromises()
    const store = useGameStore()
    store.setMaxStage(5)
    await flushPromises()
    await wrapper.find('.action-card').trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/stage/stage?level=6' })
  })

  it('onShow 且已登录 → 刷新玩家档案', async () => {
    mountPage()
    await flushPromises()
    expect(mockApi.fetchWallet).toHaveBeenCalledTimes(1)
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.fetchWallet).toHaveBeenCalledTimes(2)
  })

  it('onShow 但未登录 → 不发档案请求', async () => {
    mockApi.guestLogin.mockImplementation(() => new Promise(() => {})) // 挂起，loggedIn 停留 false
    mountPage()
    await flushPromises()
    expect(mockApi.fetchWallet).not.toHaveBeenCalled()
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.fetchWallet).not.toHaveBeenCalled()
  })
})
