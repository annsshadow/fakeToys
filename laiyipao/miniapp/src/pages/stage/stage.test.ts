// @vitest-environment jsdom
/**
 * 选关页（stage.vue）组件测试。
 * 覆盖：章节切换、关卡选择与详情卡、startLabel 四种文案、startBattle 的
 * 守卫分支（未选关 / 未解锁 / 体力不足 / 正常导航）、onLoad query 解析、
 * 卡关诊断的"可用 / unclear / 失败"三分支。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Stage from './stage.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  diagnose: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  // 带地形与 BOSS 标记的关卡，驱动关卡格 class 分支
  mockApi.fetchConfig.mockResolvedValue(
    makeConfig({
      levels: [
        makeLevel({ id: 1, chapter: 1 }),
        makeLevel({ id: 2, chapter: 1, terrain: [{ kind: 'oil_drum', x: 600, y: 600, param: 50 }] }),
        makeLevel({ id: 3, chapter: 2, is_boss: true }),
        makeLevel({ id: 4, chapter: 2, terrain: [{ kind: 'alien_kind' as any, x: 1, y: 1, param: 1 }] }),
      ],
    }),
  )
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 30, keys: 4 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.diagnose.mockResolvedValue({
    stage: 'mid_game',
    confidence: 80,
    title: '反应链不足',
    detail: '技能元素单一',
    suggestions: ['补一种元素'],
  })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('stage.vue 选关页', () => {
  it('挂载后默认选中可解锁关（第 1 关），详情卡与开始按钮齐全，诊断可用时展示', async () => {
    const { wrapper } = mountPage(Stage)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onLoad', {})
    await flushPromises()
    expect(mockApi.diagnose).toHaveBeenCalledWith(1, 2)
    expect(wrapper.text()).toContain('第 1 关 · 测试关')
    expect(wrapper.text()).toContain('防线血量')
    expect(wrapper.text()).toContain('1000‰') // difficulty
    expect(wrapper.text()).toContain('1★ 100') // star_targets
    expect(wrapper.find('.btn-primary').text()).toBe('开始战斗（消耗 6 体力）')
    // 诊断卡
    expect(wrapper.text()).toContain('卡关诊断')
    expect(wrapper.text()).toContain('置信度 80%')
    expect(wrapper.text()).toContain('补一种元素')
    // 章节徽标：第 2 关地形、BOSS 关在第 2 章
    expect(wrapper.find('.level-badge.terrain').exists()).toBe(true)
    expect(wrapper.text()).not.toContain('BOSS')
  })

  it('诊断返回 unclear → 不展示诊断卡；诊断失败 → 静默不阻塞', async () => {
    mockApi.diagnose.mockResolvedValue({ stage: 'unclear' })
    const { wrapper } = mountPage(Stage)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onLoad', {})
    await flushPromises()
    expect(wrapper.text()).not.toContain('卡关诊断')

    mockApi.diagnose.mockRejectedValue(new Error('诊断不可用'))
    triggerUniHook(wrapper.vm, 'onLoad') // query 缺省（undefined）→ level 0 → 默认选中
    await flushPromises()
    expect(um.mock.showToast).not.toHaveBeenCalled() // 不阻塞选关
  })

  it('switchChapter：点击章节 chip 切换关卡集合并重置选中与诊断', async () => {
    const { wrapper } = mountPage(Stage)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onLoad', {})
    await flushPromises()
    const chips = wrapper.findAll('.chapter-chip')
    expect(chips.length).toBe(2)
    await chips[1]!.trigger('click')
    await flushPromises()
    // 第 2 章的两关，且带 BOSS/地形徽标
    const cells = wrapper.findAll('.level-cell')
    expect(cells.length).toBe(2)
    expect(cells[0]!.classes()).toContain('boss')
    expect(cells[1]!.classes()).toContain('terrain')
    expect(wrapper.text()).not.toContain('卡关诊断') // diagnoseInfo 被重置
  })

  it('enter：点击关卡格选中并显示该关详情（含地形名与未知地形回退）', async () => {
    const { wrapper } = mountPage(Stage)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onLoad', {})
    await flushPromises()
    await wrapper.findAll('.chapter-chip')[1]!.trigger('click')
    await wrapper.findAll('.level-cell')[1]!.trigger('click') // 第 4 关（未知地形）
    await flushPromises()
    expect(wrapper.text()).toContain('第 4 关 · 测试关')
    expect(wrapper.text()).toContain('alien_kind') // TERRAIN_NAME 未命中 → 原文
  })

  it('startLabel：未选关 / 未解锁 / 体力不足 三种文案', async () => {
    const { wrapper, store } = mountPage(Stage)
    await flushPromises()
    // 未选关（query 指向不存在的关 → selected 为 null）
    triggerUniHook(wrapper.vm, 'onLoad', { level: '99' })
    await flushPromises()
    expect(wrapper.find('.btn-primary').text()).toBe('未选择关卡')
    // 诊断此时回退用 unlockedLevel
    expect(mockApi.diagnose).toHaveBeenCalledWith(1, 2)

    // selected 为 null 时点击开始：守卫直接返回，不 toast 不导航
    await wrapper.find('.btn-primary').trigger('click')
    expect(um.mock.showToast).not.toHaveBeenCalled()
    expect(um.mock.navigateTo).not.toHaveBeenCalled()

    // 未解锁：进入第 2 关之后 maxStage 停在 1 → 第 3 关锁定
    triggerUniHook(wrapper.vm, 'onLoad', { level: '3' })
    await flushPromises()
    expect(wrapper.find('.btn-primary').text()).toBe('第 3 关未解锁')

    // 体力不足（第 1 关消耗 6，体力清零）
    store.wallet.energy = 0
    triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
    await flushPromises()
    expect(wrapper.find('.btn-primary').text()).toBe('体力不足')
  })

  it('startBattle：不可进入 + 体力不足 → toast；不可进入 + 体力充足 → 静默返回', async () => {
    const { wrapper, store } = mountPage(Stage)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onLoad', { level: '3' }) // 第 3 关未解锁
    await flushPromises()

    store.wallet.energy = 0
    await wrapper.find('.btn-primary').trigger('click')
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '体力不足', icon: 'none' })
    expect(um.mock.navigateTo).not.toHaveBeenCalled()

    // 体力充足但未解锁：静默 return，无 toast 无导航
    store.wallet.energy = 30
    await wrapper.find('.btn-primary').trigger('click')
    expect(um.mock.navigateTo).not.toHaveBeenCalled()
    expect(um.mock.showToast).toHaveBeenCalledTimes(1)
  })

  it('startBattle：可进入 → 携带关卡号跳转战斗页（即使体力为 0，也由服务端裁定）', async () => {
    const { wrapper, store } = mountPage(Stage)
    await flushPromises()
    store.setMaxStage(1) // unlockedLevel = 2
    triggerUniHook(wrapper.vm, 'onLoad', { level: '2' })
    await flushPromises()
    store.wallet.energy = 0
    await wrapper.find('.btn-primary').trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/battle/battle?level=2' })
  })

  it('onMounted：未登录时自动登录', async () => {
    mountPage(Stage)
    await flushPromises()
    expect(mockApi.guestLogin).toHaveBeenCalledTimes(1)
  })
})
