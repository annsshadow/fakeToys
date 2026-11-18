// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
﻿// @vitest-environment jsdom
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
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  // 账号登录（测试环境共用一个账号）新增的 API 面：
  //   hasUsableToken —— 本地是否已有可用令牌（有就别再建游客号）
  //   loadProfile / saveProfile —— 身份摘要（昵称 + 是否游客）
  //   login / saveLastUsername —— 口令登录与用户名回填
  hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  fetchTasks: vi.fn(),
  signIn: vi.fn(),
  claimTask: vi.fn(),
  redeem: vi.fn(),
  fetchSignInCalendar: vi.fn(),
  fetchSignInStatus: vi.fn(),
  // 第六批 R1：离线收益入口迁到签到页
  offlineRewardStatus: vi.fn(),
  claimOfflineReward: vi.fn(),
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
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: {} as any, serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
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
  // 第六批 R1：离线收益缺省可领
  mockApi.offlineRewardStatus.mockResolvedValue({
    coin: 600, elapsed_hours: 2, cap_hours: 12, last_claim_at: '',
  })
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
    expect(wrapper.text()).toContain('金币 500 · 钻石 10')
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

  it('第 141 轮：初始态以服务端签到状态为准：已签 3 天且今日已签 → 第 4 天标 today、按钮禁用且文案「今日已签到」', async () => {
    mockApi.fetchSignInStatus.mockResolvedValue({ claimed_count: 3, signed_today: true, can_sign: false })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(mockApi.fetchSignInStatus).toHaveBeenCalledTimes(1)
    const days = wrapper.findAll('.day')
    expect(days[2]!.classes()).toContain('claimed') // 第 3 天已签
    expect(days[3]!.classes()).toContain('today') // 今天是第 4 天
    expect(days[3]!.classes()).not.toContain('claimed')
    expect(days[4]!.classes()).not.toContain('today')
    const btn = wrapper.find('.btn-primary')
    expect(btn.classes()).toContain('btn-disabled')
    expect(wrapper.text()).toContain('今日已签到')
    expect(wrapper.text()).not.toContain('领取今日奖励')
  })

  it('第 141 轮：周期已签满（7 天全签、今日未签）→ 文案是「七日签到已完成」而非「今日已签到」', async () => {
    mockApi.fetchSignInStatus.mockResolvedValue({ claimed_count: 7, signed_today: false, can_sign: false })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.text()).toContain('七日签到已完成')
    expect(wrapper.text()).not.toContain('今日已签到')
    expect(wrapper.findAll('.day')[6]!.classes()).toContain('claimed')
  })

  it('第 141 轮：状态拉取失败 → 保持缺省可点态（点了由 POST 的权威结果兜底），页面不崩', async () => {
    mockApi.fetchSignInStatus.mockRejectedValue(new Error('网络故障'))
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    const btn = wrapper.find('.btn-primary')
    expect(btn.classes()).not.toContain('btn-disabled')
    expect(wrapper.text()).toContain('领取今日奖励')
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
    expect(wrapper.text()).toContain('签到成功，获得 金币 4000')
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
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '已领取 金币 500', icon: 'none' })
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
    expect(wrapper.text()).toContain('体力 5') // energy 键（统一后的长名）
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
    expect(wrapper.text()).toContain('兑换成功：钻石 100')
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

  // ── R20 签到三态 + 任务进度完成态（2026-11-17 100 轮打磨 W 批）─────
  it('R20 签到三态：已签有对勾、今日脉冲、未来天降权', async () => {
    mockApi.fetchSignInStatus.mockResolvedValue({ claimed_count: 2, signed_today: false, can_sign: true })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    const days = wrapper.findAll('.day')
    // 第 1、2 天已签：绿框 + 对勾角标
    expect(days[0]!.classes()).toContain('claimed')
    expect(days[0]!.find('.day-check').exists()).toBe(true)
    expect(days[1]!.classes()).toContain('claimed')
    // 第 3 天今日：脉冲，但还没签（无对勾）
    expect(days[2]!.classes()).toContain('today')
    expect(days[2]!.classes()).not.toContain('future')
    expect(days[2]!.find('.day-check').exists()).toBe(false)
    // 第 4 天起未来：降权
    expect(days[3]!.classes()).toContain('future')
    expect(days[6]!.classes()).toContain('future')
  })

  it('R20 任务进度条完成态：done 任务渐变金 + 百分比', async () => {
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    const bars = wrapper.findAll('.task-progress')
    expect(bars.length).toBe(3)
    // fixture 第 1 条 done:true progress 3/3；第 2 条未完成
    expect(bars[0]!.classes()).toContain('done')
    expect(bars[1]!.classes()).not.toContain('done')
    const percents = wrapper.findAll('.task-percent')
    expect(percents[0]!.text()).toBe('100%')
    expect(percents[0]!.classes()).toContain('done')
    expect(percents[1]!.classes()).not.toContain('done')
  })

  // ── R34 成功反馈动效（X 批）────────────────────────────────────────
  it('R34 签到成功后刚点亮的那天短暂高亮', async () => {
    mockApi.signIn.mockResolvedValue({ already: false, day_index: 4, reward: { coin: 4000 } })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.findAll('.day')[3]!.classes()).not.toContain('claim-pop')

    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()
    const days = wrapper.findAll('.day')
    expect(days[3]!.classes()).toContain('claim-pop') // 刚签的第 4 天
    expect(days[2]!.classes()).not.toContain('claim-pop') // 已签的旧天不高亮
  })

  it('R34 领取任务后对应任务行短暂高亮', async () => {
    mockApi.claimTask.mockResolvedValue({ reward: { coin: 500 } })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.findAll('.task')[0]!.classes()).not.toContain('claim-pop')

    await wrapper.findAll('.task .btn')[0]!.trigger('click')
    await flushPromises()
    const tasks = wrapper.findAll('.task')
    expect(tasks[0]!.classes()).toContain('claim-pop')
    expect(tasks[1]!.classes()).not.toContain('claim-pop')
  })

  it('R40 连续进度条宽度随已签天数变化', async () => {
    mockApi.fetchSignInStatus.mockResolvedValue({ claimed_count: 2, signed_today: false, can_sign: true })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.find('.streak-fill').attributes('style')).toContain('width: 29%')
  })

  // ── 第六批 R1 离线收益与签到同页 ──────────────────────────────────
  it('R1 签到页展示离线收益卡片，点击可领取并反馈', async () => {
    mockApi.claimOfflineReward.mockResolvedValue({ coin: 600 })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.find('.offline-card').exists()).toBe(true)
    expect(wrapper.text()).toContain('离线收益')
    expect(wrapper.text()).toContain('600')
    expect(wrapper.text()).toContain('2.0 小时')
    const claimBtn = wrapper.find('.offline-claim')
    expect(claimBtn.exists()).toBe(true)
    await claimBtn.trigger('click')
    await flushPromises()
    expect(mockApi.claimOfflineReward).toHaveBeenCalledOnce()
    expect(wrapper.text()).toContain('领取成功：600 金币')
  })

  it('R1 无可领金币时领取按钮禁用', async () => {
    mockApi.offlineRewardStatus.mockResolvedValue({
      coin: 0, elapsed_hours: 0, cap_hours: 12, last_claim_at: '',
    })
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.find('.offline-claim').classes()).toContain('disabled')
    await wrapper.find('.offline-claim').trigger('click')
    expect(mockApi.claimOfflineReward).not.toHaveBeenCalled()
  })

  it('竞态：切到每周任务后，先发后到的每日任务响应不得覆盖列表', async () => {
    // 构造两个可控响应：daily 请求先发但慢到，weekly 请求后发但先到。
    type Res = { items: any[] }
    let resolveDaily!: (value: Res) => void
    let resolveWeekly!: (value: Res) => void
    const dailyPromise = new Promise<Res>((r) => { resolveDaily = r })
    const weeklyPromise = new Promise<Res>((r) => { resolveWeekly = r })
    const dailyItem = { id: 91, name: '每日-慢到任务', progress: 0, target: 1, done: false, claimed: false, reward: { coin: 1 } }
    const weeklyItem = { id: 92, name: '每周-当前任务', progress: 1, target: 1, done: false, claimed: false, reward: { coin: 2 } }
    mockApi.fetchTasks.mockImplementation((scope?: string) => {
      if (scope === 'daily') return dailyPromise as any
      if (scope === 'weekly') return weeklyPromise as any
      return Promise.resolve({ items: [] }) as any
    })

    const { wrapper } = mountPage(Signin)
    await flushPromises()
    // 初始 daily 请求已发出，尚未返回
    expect(mockApi.fetchTasks).toHaveBeenCalledWith('daily')

    // 用户切到「每周」：发出 weekly 请求（后发）
    const weeklyTab = wrapper.findAll('text').find((t) => t.text().startsWith('每周'))!
    await weeklyTab.trigger('click')
    expect(mockApi.fetchTasks).toHaveBeenCalledWith('weekly')

    // weekly 先返回 → 当前选中 tab 应展示每周任务
    resolveWeekly({ items: [weeklyItem] })
    await flushPromises()
    expect(wrapper.text()).toContain('每周-当前任务')

    // daily 后返回（陈旧响应）→ 不得覆盖当前选中的每周任务
    resolveDaily({ items: [dailyItem] })
    await flushPromises()
    expect(wrapper.text()).toContain('每周-当前任务')
    expect(wrapper.text()).not.toContain('每日-慢到任务')
  })

  it('签到成功后重拉任务列表：daily_signin 任务进度即时刷新（写后失同步）', async () => {
    // 服务端 SignIn 会 bumpTasks({signin:1}) 推进 daily_signin 任务；
    // 签到页必须随之重拉任务，否则本会话内仍停在 0/1「未完成」。
    let signedIn = false
    mockApi.fetchTasks.mockImplementation(() =>
      Promise.resolve({
        items: [
          signedIn
            ? { id: 4, name: '今日完成签到', progress: 1, target: 1, done: true, claimed: false, reward: { coin: 500 } }
            : { id: 4, name: '今日完成签到', progress: 0, target: 1, done: false, claimed: false, reward: { coin: 500 } },
        ],
      }),
    )
    mockApi.signIn.mockImplementation(() => {
      signedIn = true
      return Promise.resolve({ already: false, day_index: 1, reward: { coin: 1000 } })
    })

    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.findAll('.task .btn')[0]!.text()).toBe('未完成')
    expect(wrapper.find('.task').text()).toContain('0 / 1')

    const fetchBefore = mockApi.fetchTasks.mock.calls.length
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()

    expect(mockApi.fetchTasks.mock.calls.length).toBeGreaterThan(fetchBefore)
    expect(wrapper.findAll('.task .btn')[0]!.text()).toBe('领取')
    expect(wrapper.find('.task').text()).toContain('1 / 1')
  })
})
