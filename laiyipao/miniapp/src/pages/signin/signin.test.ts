// @vitest-environment jsdom
/**
 * 签到页（signin.vue）组件测试。
 * 覆盖：7 天奖励表的分支（第 3/7 天加钻、第 7 天加体力）、任务列表三态按钮、
 * 签到的成功 / 已签 / 失败 / 重复点击守卫、任务领取、兑换码的空值 / 成功 / 失败。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Signin from './signin.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  fetchTasks: vi.fn(),
  signIn: vi.fn(),
  claimTask: vi.fn(),
  redeem: vi.fn(),
  fetchSignInCalendar: vi.fn(),
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
  mockApi.fetchConfig.mockResolvedValue({} as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchTasks.mockResolvedValue({
    items: [
      { id: 1, name: '通关 3 关', progress: 3, target: 3, done: true, claimed: false, reward: { coin: 500, gem: 10 } },
      { id: 2, name: '打一场战斗', progress: 0, target: 1, done: false, claimed: false, reward: { coin: 200 } },
      { id: 3, name: '已领任务', progress: 1, target: 1, done: true, claimed: true, reward: { keys: 1 } },
    ],
  })
  // 缺省：日历拉取失败 → 回落本地缺省公式（保持既有用例的展示值不变）
  mockApi.fetchSignInCalendar.mockRejectedValue(new Error('offline'))
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('signin.vue 签到页', () => {
  it('挂载后拉取任务并渲染 7 天奖励表（第 3/7 天的钻与体力分支）', async () => {
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(mockApi.fetchTasks).toHaveBeenCalledWith('daily')
    const days = wrapper.findAll('.day')
    expect(days.length).toBe(7)
    expect(days[0]!.text()).toContain('1000 金')
    expect(days[2]!.text()).toContain('60 钻') // day 3 → gem 20*3
    expect(days[6]!.text()).toContain('140 钻') // day 7 → gem 20*7
    expect(days[6]!.text()).toContain('50 体') // day 7 → energy 50
    // 任务按钮三态：领取 / 未完成 / 已领
    expect(wrapper.text()).toContain('领取')
    expect(wrapper.text()).toContain('未完成')
    expect(wrapper.text()).toContain('已领')
    // 奖励文案（currencyName 的金/钻/钥匙分支）
    expect(wrapper.text()).toContain('金 500 · 钻 10')
    expect(wrapper.text()).toContain('钥匙 1')
  })

  it('第 135 轮：预览奖励由服务端日历驱动，不是本地硬编码公式', async () => {
    // 服务端日历：第 1 天给 77 钻（与本地缺省 { coin: 1000 } 完全不同）。
    mockApi.fetchSignInCalendar.mockResolvedValue({
      days: [
        { day_index: 1, reward: { gem: 77 } },
        { day_index: 2, reward: { coin: 10 } },
        { day_index: 3, reward: { coin: 1000 } },
        { day_index: 4, reward: { coin: 1000 } },
        { day_index: 5, reward: { coin: 1000 } },
        { day_index: 6, reward: { coin: 1000 } },
        { day_index: 7, reward: { coin: 1000 } },
      ],
    } as any)
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    const days = wrapper.findAll('.day')
    // 第 1 天：必须是服务端的 77 钻（本地公式会显示 1000 金）
    expect(days[0]!.text()).toContain('77 钻')
    expect(days[0]!.text()).not.toContain('1000 金')
    // 第 2 天：服务端 10 金（本地公式会是 2000 金）
    expect(days[1]!.text()).toContain('10 金')
    expect(days[1]!.text()).not.toContain('2000 金')
  })

  it('第 135 轮：日历缺失/失败 → 回落本地缺省公式（不阻断页面）', async () => {
    // beforeEach 默认 fetchSignInCalendar reject → 回落本地公式
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    const days = wrapper.findAll('.day')
    expect(days[0]!.text()).toContain('1000 金') // 本地 day1 = coin 1000
  })

  it('任务列表为空 → 显示"暂无任务"（items 缺失时 ?? [] 回退）', async () => {
    mockApi.fetchTasks.mockResolvedValue({} as any)
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.text()).toContain('暂无任务')
  })

  it('任务加载失败 → toast 提示', async () => {
    mockApi.fetchTasks.mockRejectedValue(new Error('任务拉取失败'))
    mountPage(Signin)
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '任务拉取失败', icon: 'none' })
  })

  it('签到成功：记录第 N 天、展示奖励、刷新钱包', async () => {
    mockApi.signIn.mockResolvedValue({ already: false, day_index: 4, reward: { coin: 4000 } })
    const { wrapper, store } = mountPage(Signin)
    await flushPromises()
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()
    expect(mockApi.signIn).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('签到成功，获得 金 4000')
    // refreshWallet 被签到流程调用（coin 变为 1）
    expect(store.wallet.coin).toBe(1)
    expect(um.mock.showToast).not.toHaveBeenCalled()
  })

  it('已签到过（res.already）→ 按钮变为"今日已签到"且点击不再请求', async () => {
    mockApi.signIn.mockResolvedValue({ already: true })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()
    expect(wrapper.text()).toContain('今日已签到')
    expect(wrapper.text()).toContain('今日已签') // lastResult 文案

    const calls = mockApi.signIn.mock.calls.length
    await wrapper.find('.btn-primary').trigger('click') // canSign=false 的点击守卫
    await flushPromises()
    expect(mockApi.signIn).toHaveBeenCalledTimes(calls)
  })

  it('签到接口失败 → canSign 置 false 并把错误展示出来', async () => {
    mockApi.signIn.mockRejectedValue(new Error('服务端错误'))
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()
    expect(wrapper.text()).toContain('服务端错误')
  })

  it('领取任务成功 → toast 奖励并重新拉取任务、刷新钱包', async () => {
    mockApi.claimTask.mockResolvedValue({ reward: { coin: 500 } })
    const { wrapper, store } = mountPage(Signin)
    await flushPromises()
    const fetchCalls = mockApi.fetchTasks.mock.calls.length
    await wrapper.findAll('.task .btn')[0]!.trigger('click')
    await flushPromises()
    expect(mockApi.claimTask).toHaveBeenCalledWith(1)
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '已领取 金 500', icon: 'none' })
    expect(mockApi.fetchTasks).toHaveBeenCalledTimes(fetchCalls + 1)
    expect(store.wallet.coin).toBe(1)
  })

  it('领取任务失败 → toast 错误信息', async () => {
    mockApi.claimTask.mockRejectedValue(new Error('领取失败'))
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    await wrapper.findAll('.task .btn')[0]!.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '领取失败', icon: 'none' })
  })

  it('奖励数据的兜底分支：reward 为 null 与未知货币键都按原文/空渲染', async () => {
    mockApi.fetchTasks.mockResolvedValue({
      items: [
        { id: 9, name: '空奖励任务', progress: 0, target: 1, done: false, claimed: false, reward: null },
        { id: 10, name: '未知货币任务', progress: 0, target: 1, done: true, claimed: false, reward: { vip_point: 3 } },
        { id: 11, name: '体力任务', progress: 0, target: 1, done: false, claimed: false, reward: { energy: 5 } },
      ],
    } as any)
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.text()).toContain('vip_point 3') // currencyName 未知键 → 原文
    expect(wrapper.text()).toContain('体 5') // energy 键
    // reward ?? {} 分支：null 奖励渲染为空串而不是崩溃
    expect(wrapper.text()).not.toContain('undefined')
  })

  it('兑换码：空输入直接返回；成功后清空输入并刷新钱包', async () => {
    mockApi.redeem.mockResolvedValue({ reward: { gem: 100 } })
    const { wrapper, store } = mountPage(Signin)
    await flushPromises()
    const redeemBtn = wrapper.findAll('.btn').find((b) => b.text() === '兑换')!
    await redeemBtn.trigger('click') // 空 code → 早退
    expect(mockApi.redeem).not.toHaveBeenCalled()

    const input = wrapper.find('input')
    ;(input.element as HTMLInputElement).value = ' LYP2026 '
    await input.trigger('input')
    await redeemBtn.trigger('click')
    await flushPromises()
    // 输入被 trim 后提交
    expect(mockApi.redeem).toHaveBeenCalledWith('LYP2026')
    expect(wrapper.text()).toContain('兑换成功：钻 100')
    expect((wrapper.find('input').element as HTMLInputElement).value).toBe('')
    expect(store.wallet.coin).toBe(1) // refreshWallet 生效
  })

  it('兑换码失败 → 错误信息展示在输入框下方', async () => {
    mockApi.redeem.mockRejectedValue(new Error('兑换码无效'))
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    const input = wrapper.find('input')
    ;(input.element as HTMLInputElement).value = 'BAD'
    await input.trigger('input')
    const redeemBtn = wrapper.findAll('.btn').find((b) => b.text() === '兑换')!
    await redeemBtn.trigger('click')
    await flushPromises()
    expect(wrapper.text()).toContain('兑换码无效')
  })
})
