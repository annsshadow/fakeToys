// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R290：me 页护盾开关的权威写与在途防线加载竞态（行为级回归）。
 *
 * toggleShield 成功写入服务端权威防线（新 shielded_until）后必须让在途的
 * loadDefenses 失效。若像旧实现那样只更新 shielded 却不递增 defensesReqSeq，
 * 弱网下先发的 loadDefenses 旧响应后到会用旧 shielded_until 覆盖刚生效的护盾态，
 * 玩家开了盾却仍显示未开（或反之）。与 R282 persistLoadout 同构。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
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
  return { runChallenge: vi.fn(), validateSnapshot: vi.fn(() => ({ ok: true })), isShieldActive: actual.isShieldActive }
})

const mockApi = vi.mocked(api, true)
const FUTURE = '2099-01-01T00:00:00Z'

function defenseView(shieldedUntil: string) {
  return {
    id: 1,
    user_id: 7,
    name: '我的防线',
    power: 100,
    element_coverage: 3,
    wins: 2,
    losses: 1,
    snapshot_hash: 'hash1',
    shielded_until: shieldedUntil,
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
  mockApi.fetchMastery.mockResolvedValue({ points: 0, nodes: [{ id: 1, level: 1 }], families: [] } as any)
  mockApi.fetchMyStars.mockResolvedValue({ stars: {} } as any)
  mockApi.myChallengeHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myDefenseChallenges.mockResolvedValue({ items: [] } as any)
  mockApi.myBattleHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockResolvedValue({ items: [] } as any)
  // 首次加载：有防线但未开盾
  mockApi.fetchDefenses.mockResolvedValue({ mine: defenseView(''), candidates: [], attempt_limit: 3 } as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me 页护盾权威写与在途防线加载竞态（R290）', () => {
  it('toggleShield 成功后，在途的旧 loadDefenses 响应不得回滚刚生效的护盾态', async () => {
    // onMounted 发起第一发 loadDefenses（挂起，旧快照：未开盾）
    let resolveOld!: (v: any) => void
    const oldPending = new Promise<any>((r) => { resolveOld = r })
    mockApi.fetchDefenses.mockImplementationOnce(() => oldPending as any)

    const { wrapper } = mountPage(Me, (st) => {
      st.loggedIn = true
      st.isGuest = false
    })
    await flushPromises()
    expect(mockApi.fetchDefenses).toHaveBeenCalledTimes(1)

    // 用户点开盾：saveDefense 返回权威防线（shielded_until 在未来）
    mockApi.saveDefense.mockResolvedValueOnce({ defense: defenseView(FUTURE) } as any)
    ;(wrapper.vm as any).toggleShield()
    await flushPromises()
    // 权威写已生效：页面显示「关闭 24h 护盾」（已开盾态）
    expect(wrapper.text()).toContain('关闭 24h 护盾')

    // 旧 loadDefenses 响应后到，携带开盾前的旧快照（未开盾）
    resolveOld({ mine: defenseView(''), candidates: [], attempt_limit: 3 })
    await flushPromises()
    // 不得回滚：仍显示已开盾
    expect(wrapper.text()).toContain('关闭 24h 护盾')
  })
})
