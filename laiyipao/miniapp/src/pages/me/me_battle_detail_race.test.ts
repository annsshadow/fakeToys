// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 88 轮：我的页「最近战报详情」竞态回归。
 *
 * 症状：战报 A 点「详情」发出 GET /me/battles/A，用户紧接着点战报 B
 * 的「详情」。若 A 的响应晚于 B 返回，旧的 A 详情会覆盖单一 battleDetail
 * ref，使 B 行下面显示 A 的掉落 / 命中 / 峰值热量（同一页里两处战报串味）。
 *
 * 与 stage.vue（第 69 轮）、rank.vue / signin.vue 的「请求序号守卫」同构：
 * 每次发起新请求递增序号，响应回来时序号不匹配就丢弃。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel, makeSkill } from '../../test/fixtures'

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
  return {
    runChallenge: vi.fn(),
    validateSnapshot: vi.fn(() => ({ ok: true })),
    isShieldActive: actual.isShieldActive,
  }
})

const mockApi = vi.mocked(api, true)

function historyItem(id: number) {
  return {
    id,
    level_id: id,
    result: 'win',
    stars: 3,
    score: 100,
    kills: 5,
    leaked: 0,
    replay_hash: 'hash' + id,
    created_at: '2026-11-18T00:00:00Z',
  }
}

function detail(id: number, hits: number, shots: number) {
  return {
    id,
    level_id: id,
    result: 'win',
    stars: 3,
    score: 100,
    kills: 5,
    leaked: 0,
    hp_left: 80,
    wave_reached: 3,
    duration_ms: 12000,
    shots,
    hits,
    reactions: 1,
    heat_max: 30,
    elements_used: { fire: hits },
    reactions_used: {},
    terrain_used: [],
    loot: { coin: 10 },
    replay_hash: 'hash' + id,
    created_at: '2026-11-18T00:00:00Z',
  }
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig({
      levels: [makeLevel()],
      skills: [makeSkill({ id: 1, apply_element: '' })],
    }),
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 0, gem: 0, energy: 100, keys: 0, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] })
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] })
  mockApi.fetchMastery.mockResolvedValue({ points: 0, nodes: [], families: [] })
  mockApi.fetchDefenses.mockResolvedValue({ mine: null, candidates: [], attempt_limit: 3 } as any)
  mockApi.fetchMyStars.mockResolvedValue({ stars: {} } as any)
  mockApi.myBattleHistory.mockResolvedValue({
    items: [historyItem(1), historyItem(2)],
    next_cursor: 0,
  })
  mockApi.myChallengeHistory.mockResolvedValue({ items: [] })
  mockApi.myDefenseChallenges.mockResolvedValue({ items: [] })
  mockApi.myPurchases.mockResolvedValue({ items: [] })
  mockApi.myRedemptions.mockResolvedValue({ items: [] })
  mockApi.myOrders.mockResolvedValue({ items: [] })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me.vue 战报详情竞态（第 88 轮）', () => {
  it('旧战报的慢响应返回后不得覆盖新战报的详情', async () => {
    let resolveOld!: (v: ReturnType<typeof detail>) => void
    const oldPending = new Promise<ReturnType<typeof detail>>((resolve) => {
      resolveOld = resolve
    })
    mockApi.myBattleDetail.mockImplementation((id: any) => {
      if (id === 1) return oldPending as any
      return Promise.resolve(detail(2, 22, 222)) as any
    })

    const { wrapper } = mountPage(Me, (st) => {
      st.loggedIn = true
      st.isGuest = false
    })
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()

    const links = wrapper.findAll('text').filter((t) => t.text() === '详情')
    expect(links).toHaveLength(2)

    // 先点战报 1：请求挂在半空（模拟慢网络）。
    await links[0].trigger('click')
    await flushPromises()
    expect(mockApi.myBattleDetail).toHaveBeenCalledWith(1)

    // 再点战报 2：新请求立即返回，详情应显示为 22/222。
    const linksAfterFirst = wrapper.findAll('text').filter((t) => t.text() === '详情')
    await linksAfterFirst[1].trigger('click')
    await flushPromises()
    expect(mockApi.myBattleDetail).toHaveBeenCalledWith(2)
    expect(wrapper.text()).toContain('22/222')

    // 战报 1 的迟到响应抵达 —— 旧值不得覆盖战报 2 的详情。
    resolveOld(detail(1, 11, 111))
    await flushPromises()
    expect(wrapper.text()).toContain('22/222')
    expect(wrapper.text()).not.toContain('11/111')
  })
})
