// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 签到页「服务端日历长度驱动周期展示」回归（行为测试）。
 *
 * 服务端 SignInStatus 用 len(SignInCalendar) 作为周期长度，SignInCalendar
 * 返回几天，玩家就能签几天。前端此前把「7 天」写死：天数格子恒 7、进度条
 * 分母恒 7，与服务端契约脱节 —— 运营把日历调成 14 天后，玩家永远看不到
 * 第 8~14 天、进度条在 57% 就封顶。本测试从组件行为层面钉住：
 *   1) 天数格子数量、进度条分母、完成文案都随服务端日历长度变化；
 *   2) 日历未加载（离线/失败）时才回落到 7 天缺省。
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
  offlineRewardStatus: vi.fn(),
  claimOfflineReward: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

function calendarDays(n: number) {
  return Array.from({ length: n }, (_, i) => ({ day_index: i + 1, reward: { coin: 100 } }))
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: {} as any, serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchTasks.mockResolvedValue({ items: [] })
  mockApi.fetchSignInStatus.mockResolvedValue({ claimed_count: 0, signed_today: false, can_sign: true })
  mockApi.offlineRewardStatus.mockResolvedValue({ coin: 0, elapsed_hours: 0, cap_hours: 12, last_claim_at: '' })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('signin.vue 周期长度跟随服务端日历', () => {
  it('服务端日历 14 天 → 渲染 14 个格子，签满 7 天时进度条是 50%（不是写死的 100%）', async () => {
    mockApi.fetchSignInCalendar.mockResolvedValue({ days: calendarDays(14) } as any)
    mockApi.fetchSignInStatus.mockResolvedValue({ claimed_count: 7, signed_today: false, can_sign: true })
    const { wrapper } = mountPage(Signin)
    await flushPromises()

    const days = wrapper.findAll('.day')
    expect(days.length).toBe(14)
    expect((wrapper.find('.streak-fill').attributes('style') || '')).toContain('width: 50%')
    // 今日应该落在第 8 天（claimed+1），且仍可领取（服务端 can_sign=true）
    expect(days[7]!.classes()).toContain('today')
    expect(wrapper.text()).toContain('领取今日奖励')
  })

  it('服务端日历 14 天 → 签满 14 天时按钮文案用真实周期天数', async () => {
    mockApi.fetchSignInCalendar.mockResolvedValue({ days: calendarDays(14) } as any)
    mockApi.fetchSignInStatus.mockResolvedValue({ claimed_count: 14, signed_today: false, can_sign: false })
    const { wrapper } = mountPage(Signin)
    await flushPromises()

    expect(wrapper.findAll('.day').length).toBe(14)
    expect(wrapper.text()).toContain('14 天签到已完成')
    expect(wrapper.text()).not.toContain('七日签到已完成')
  })

  it('日历未加载（离线/失败）时仍回落 7 天缺省，保持既有行为', async () => {
    mockApi.fetchSignInCalendar.mockRejectedValue(new Error('offline'))
    const { wrapper } = mountPage(Signin)
    await flushPromises()
    expect(wrapper.findAll('.day').length).toBe(7)
  })
})
