// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R278：me 页防线加载的并发序号守卫（行为级回归）。
 *
 * loadDefenses 有三个入口：onMounted、onShow（Promise.all 内）、写后重拉。
 * 弱网下先发的旧请求失败走 catch 会无条件清空 myDefense/candidates，
 * 若它晚于后发的新请求返回，就把已加载好的防线清空（用户看到「没有防线」）。
 * 用序号守卫丢弃陈旧响应，与同文件 battleDetailReqSeq / 其他页 loadSeq 同构。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from '../pages/me/me.vue'
import * as api from '@/api/client'
import { installUniMock } from './uni-mock'
import { triggerUniHook } from './uni-app-stub'
import { mountPage } from './page'
import { makeConfig, makeLevel, makeSkill } from './fixtures'

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
  fetchMyEquipment: vi.fn(),
  fetchMastery: vi.fn(),
  fetchDefenses: vi.fn(),
  saveDefense: vi.fn(),
  deleteMyDefense: vi.fn(),
  challengeDefense: vi.fn(),
  fetchLevel: vi.fn(),
  fetchMyStars: vi.fn(),
  myChallengeHistory: vi.fn(),
  myDefenseChallenges: vi.fn(),
  myBattleHistory: vi.fn(),
  myBattleDetail: vi.fn(),
  myPurchases: vi.fn(),
  myRedemptions: vi.fn(),
  myOrders: vi.fn(),
  changePassword: vi.fn(),
  upgradeAccount: vi.fn(),
  updateNickname: vi.fn(),
  logout: vi.fn(),
}))

vi.mock('@/game/defense', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/game/defense')>()
  return { runChallenge: vi.fn(), validateSnapshot: vi.fn(() => ({ ok: true })), isShieldActive: actual.isShieldActive }
})

const mockApi = vi.mocked(api, true)

function defenseView(id: number, name: string) {
  return {
    id,
    user_id: id,
    name,
    power: 100 + id,
    element_coverage: 3,
    wins: 2,
    losses: 1,
    snapshot_hash: 'hash' + id,
    shielded_until: '',
    my_attempts_today: 0,
    can_challenge: true,
  } as any
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig({ levels: [makeLevel()], skills: [makeSkill({ id: 1, apply_element: '' })] }),
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 0, gem: 0, energy: 100, keys: 0, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] })
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 0, nodes: [], families: [] } as any)
  mockApi.fetchMyStars.mockResolvedValue({ stars: {} } as any)
  mockApi.myChallengeHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myDefenseChallenges.mockResolvedValue({ items: [] } as any)
  mockApi.myBattleHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockResolvedValue({ items: [] } as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me 页防线加载并发序号守卫（R278）', () => {
  it('旧防线请求的失败晚到时，不得清空后发新请求已加载的防线', async () => {
    let rejectOld!: (e: Error) => void
    const oldPending = new Promise<ReturnType<typeof api.fetchDefenses>>((_, rej) => { rejectOld = rej })
    // 第一次（onMounted）挂起，第二次（onShow）立即返回有防线的快照
    mockApi.fetchDefenses
      .mockImplementationOnce(() => oldPending as any)
      .mockImplementationOnce(() => Promise.resolve({
        mine: defenseView(1, '我的防线'),
        candidates: [defenseView(2, '对手甲'), defenseView(3, '对手乙')],
        attempt_limit: 3,
      } as any) as any)

    const { wrapper } = mountPage(Me, (st) => {
      st.loggedIn = true
      st.isGuest = false
    })
    // onMounted 触发第一发（挂在半空）
    await flushPromises()
    expect(mockApi.fetchDefenses).toHaveBeenCalledTimes(1)

    // onShow 触发第二发，新请求立即返回并渲染出防线与 2 个候选
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.fetchDefenses).toHaveBeenCalledTimes(2)
    expect(wrapper.findAll('.cand')).toHaveLength(2)

    // 旧请求（第一发）的失败迟到 —— 不得清空新数据
    rejectOld(new Error('stale network error'))
    await flushPromises()
    expect(wrapper.findAll('.cand')).toHaveLength(2)
    expect(wrapper.text()).toContain('我的防线')
  })
})
