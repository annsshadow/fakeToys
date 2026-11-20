// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * me.vue 战报详情「剩余血量」单位行为测试。
 *
 * 服务端契约（economy.go / domain/battle.go / engine.ts）：
 *   GET /me/battles/:id 的 hp_left 来自 battle_records.hp_left，是**绝对剩余
 *   防线血量** —— 引擎 settleInput 上报 Number(baseHp)，domain.validateHPLeft
 *   用「0 ≤ hp_left ≤ 该关初始血量 base_hp」校验，量纲是血量点（可达数百上千）。
 *   真正的百分比是另一个字段 hp_left_pct（defense_challenges，0–100），
 *   BattleDetail 响应里根本没有它。
 * 因此详情里把 hp_left 渲染成「500%」是把绝对血量误标成百分比，用户可感知错误。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel, makeSkill } from '../../test/fixtures'
import type { DefenseView } from '@/game/defense'

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
  fetchDefenses: vi.fn(),
  saveDefense: vi.fn(),
  challengeDefense: vi.fn(),
  deleteMyDefense: vi.fn(),
  changePassword: vi.fn(),
  logout: vi.fn(),
  updateNickname: vi.fn(),
  upgradeAccount: vi.fn(),
  fetchLevel: vi.fn(),
  fetchMyEquipment: vi.fn(),
  fetchMastery: vi.fn(),
  myChallengeHistory: vi.fn(),
  myDefenseChallenges: vi.fn(),
  myBattleHistory: vi.fn(),
  myPurchases: vi.fn(),
  myRedemptions: vi.fn(),
  myOrders: vi.fn(),
  fetchMyStars: vi.fn(),
  myBattleDetail: vi.fn(),
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

function defenseView(over: Partial<DefenseView> = {}): DefenseView {
  return {
    id: 5, name: '我的防线', owner_name: '对手', power: 100,
    element_coverage: 3, wins: 1, losses: 2,
    snapshot_hash: 'abcdef1234567890', snapshot: { skills: [1] },
    can_challenge: true, challenge_blocked: '', ...over,
  } as DefenseView
}

beforeEach(() => {
  installUniMock()
  vi.clearAllMocks()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig({ levels: [makeLevel()], skills: [makeSkill({ id: 1, apply_element: '' })] }),
    serverTimeMs: 0,
  } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] } as any)
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 0, nodes: [], families: [] } as any)
  mockApi.fetchDefenses.mockResolvedValue({ mine: defenseView(), candidates: [], attempt_limit: 3 } as any)
  mockApi.fetchLevel.mockResolvedValue(makeLevel({ id: 1, waves: [] }) as any)
  mockApi.myChallengeHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myDefenseChallenges.mockResolvedValue({ items: [] } as any)
  mockApi.myBattleHistory.mockResolvedValue({
    items: [{
      id: 42, level_id: 3, result: 'win', stars: 3, score: 100,
      kills: 10, leaked: 0, replay_hash: 'abc', created_at: '2026-11-01T10:00:00Z',
    }],
    next_cursor: 0,
  } as any)
  mockApi.myPurchases.mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockResolvedValue({ items: [] } as any)
  mockApi.fetchMyStars.mockResolvedValue({ total: 0 } as any)
  // 绝对剩余血量 500（远超 100）—— 该关 base_hp 可达数百上千。
  mockApi.myBattleDetail.mockResolvedValue({
    id: 42, level_id: 3, result: 'win', stars: 3, score: 100,
    kills: 10, leaked: 0, hp_left: 500, wave_reached: 8,
    duration_ms: 15000, shots: 100, hits: 90, reactions: 7, heat_max: 85,
    elements_used: { fire: 3 }, reactions_used: {}, terrain_used: [],
    loot: { coin: 10 }, replay_hash: 'abc', created_at: '2026-11-01T10:00:00Z',
  } as any)
})

afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })

describe('me.vue 战报详情剩余血量单位', () => {
  it('hp_left 是绝对血量：显示纯数字，不追加百分号（500 不能显示成 500%）', async () => {
    const { wrapper } = mountPage(Me, (st) => { st.loggedIn = true; st.isGuest = false })
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()

    const detailBtn = wrapper.findAll('.link').find((t) => t.text() === '详情')
    expect(detailBtn, '应渲染战报「详情」入口').toBeTruthy()
    await detailBtn!.trigger('click')
    await flushPromises()

    const card = wrapper.findAll('.card').find((c) => c.text().includes('剩余血量'))
    expect(card, '点击详情后应展开单局详情').toBeTruthy()
    const kv = card!.findAll('.kv').find((k) => k.text().includes('剩余血量'))
    expect(kv, '应渲染「剩余血量」字段').toBeTruthy()
    // 绝对血量 500 必须原样呈现；服务端 hp_left 不是百分比，
    // 真百分比字段 hp_left_pct 才带 % 语义且不在 BattleDetail 里。
    expect(kv!.text()).toContain('500')
    expect(kv!.text()).not.toMatch(/%/)
  })
})
