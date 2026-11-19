// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/** R291 回归测试，见 describe 内说明。 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from '../pages/me/me.vue'
import * as api from '@/api/client'
import { installUniMock } from './uni-mock'
import { mountPage } from './page'
import { makeConfig, makeLevel, makeSkill } from './fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(), fetchConfigIfVersion: vi.fn(), fetchWallet: vi.fn(), fetchMe: vi.fn(),
  fetchLoadout: vi.fn(), saveLoadout: vi.fn(), setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false), onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null), saveProfile: vi.fn(), login: vi.fn(), saveLastUsername: vi.fn(),
  fetchMyEquipment: vi.fn(), fetchMastery: vi.fn(), fetchDefenses: vi.fn(), saveDefense: vi.fn(),
  deleteMyDefense: vi.fn(), challengeDefense: vi.fn(), fetchLevel: vi.fn(), fetchMyStars: vi.fn(),
  myChallengeHistory: vi.fn(), myDefenseChallenges: vi.fn(), myBattleHistory: vi.fn(),
  myBattleDetail: vi.fn(), myPurchases: vi.fn(), myRedemptions: vi.fn(), myOrders: vi.fn(),
  changePassword: vi.fn(), upgradeAccount: vi.fn(), updateNickname: vi.fn(), logout: vi.fn(),
}))
const mockApi = vi.mocked(api, true)

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig({ levels: [makeLevel()], skills: [makeSkill({ id: 1, apply_element: '' })] }),
    serverTimeMs: 0,
  } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 0, gem: 0, energy: 100, keys: 0, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] })
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 0, nodes: [], families: [] } as any)
  mockApi.fetchDefenses.mockResolvedValue({ mine: null, candidates: [], attempt_limit: 3 } as any)
  mockApi.fetchMyStars.mockResolvedValue({ stars: {} } as any)
  mockApi.myChallengeHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myDefenseChallenges.mockResolvedValue({ items: [] } as any)
  mockApi.myBattleHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockResolvedValue({ items: [] } as any)
})
afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })

describe('防线保存前构筑加载失败必须中止（R291）', () => {
  async function clickSave() {
    const { wrapper } = mountPage(Me, (st) => { st.loggedIn = true; st.isGuest = false })
    await flushPromises()
    const btn = wrapper.findAll('.btn').find((b) => b.text().includes('保存当前构筑为防线'))
    expect(btn).toBeTruthy()
    await btn!.trigger('click')
    await flushPromises()
  }
  it('装备加载失败但专精成功时，不得提交残缺构筑快照', async () => {
    mockApi.fetchMyEquipment.mockRejectedValueOnce(new Error('装备网络抖动'))
    mockApi.fetchMastery.mockResolvedValueOnce({ points: 0, nodes: [1], families: [] } as any)
    await clickSave()
    expect(mockApi.saveDefense).not.toHaveBeenCalled()
  })
  it('专精加载失败但装备成功时，不得提交残缺构筑快照', async () => {
    mockApi.fetchMyEquipment.mockResolvedValueOnce({ items: [{ id: 1, equipped: true, slot: 'armor' }] } as any)
    mockApi.fetchMastery.mockRejectedValueOnce(new Error('专精网络抖动'))
    await clickSave()
    expect(mockApi.saveDefense).not.toHaveBeenCalled()
  })
})
