// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * me.vue 被挑战损失排行（topLooters）行为测试。
 *
 * 服务端契约（adminapi.go / my_defense_challenges_test.go）：
 *   won = 挑战者是否获胜。won=true 表示防线被攻破，此时 loot 才有偷取明细。
 * 排行标题「损失排行（防线被破 Top5）」只应统计防线被破的记录。
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
  mockApi.myBattleHistory.mockResolvedValue({ items: [], next_cursor: 0 } as any)
  mockApi.myPurchases.mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockResolvedValue({ items: [] } as any)
  mockApi.fetchMyStars.mockResolvedValue({ total: 0 } as any)
  mockApi.myBattleDetail.mockResolvedValue({ detail: null } as any)
})

afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })

describe('me.vue 被挑战损失排行', () => {
  it('只统计防线被破（won=true）的记录，按 loot 总和降序展示', async () => {
    // won=true = 挑战者获胜 = 防线被破（loot 有偷取明细）
    // won=false = 防线守住（loot 应为空）
    mockApi.myDefenseChallenges.mockResolvedValue({
      items: [
        { id: 1, defense_id: 5, defense_name: '铁壁', challenger_nickname: '攻破者甲',
          won: true, duration_ms: 1000, hp_left_pct: 0,
          loot: { coin: 500, gem: 10 }, created_at: '2026-11-01T10:00:00Z' },
        { id: 2, defense_id: 5, defense_name: '铁壁', challenger_nickname: '攻破者乙',
          won: true, duration_ms: 1200, hp_left_pct: 0,
          loot: { coin: 200 }, created_at: '2026-11-01T09:00:00Z' },
        { id: 3, defense_id: 5, defense_name: '铁壁', challenger_nickname: '守住者丙',
          won: false, duration_ms: 800, hp_left_pct: 100,
          loot: {}, created_at: '2026-11-01T08:00:00Z' },
      ],
    } as any)

    const { wrapper } = mountPage(Me, (st) => { st.loggedIn = true; st.isGuest = false })
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()

    const card = wrapper.findAll('.card').find((c) => c.text().includes('损失排行'))
    expect(card, '应渲染「损失排行」卡片').toBeTruthy()
    const rows = card!.findAll('.tl-row')
    // 防线被破的两名挑战者都应入榜，按 loot 总和降序；防线守住的丙不应入榜。
    expect(rows.length).toBe(2)
    expect(rows[0].text()).toContain('攻破者甲')
    expect(rows[0].text()).toContain('510')
    expect(rows[1].text()).toContain('攻破者乙')
    expect(rows[1].text()).toContain('200')
    expect(card!.text()).not.toContain('守住者丙')
  })

  it('统一时间轴：won=true 的被挑战记录应显示「防线被破」而非「防线守住」', async () => {
    mockApi.myDefenseChallenges.mockResolvedValue({
      items: [
        { id: 11, defense_id: 5, defense_name: '铁壁', challenger_nickname: '攻破者甲',
          won: true, duration_ms: 1000, hp_left_pct: 0,
          loot: { coin: 500 }, created_at: '2026-11-05T10:00:00Z' },
        { id: 12, defense_id: 5, defense_name: '铁壁', challenger_nickname: '挑战者乙',
          won: false, duration_ms: 800, hp_left_pct: 100,
          loot: {}, created_at: '2026-11-05T09:00:00Z' },
      ],
    } as any)

    const { wrapper } = mountPage(Me, (st) => { st.loggedIn = true; st.isGuest = false })
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()

    const timeline: { at: string; kind: string; text: string }[] = (wrapper.vm as any).unifiedTimeline
    const defenseEvents = timeline.filter((e) => e.kind === '被挑战')
    expect(defenseEvents.length).toBe(2)
    const breached = defenseEvents.find((e) => e.at === '2026-11-05T10:00:00Z')!
    const held = defenseEvents.find((e) => e.at === '2026-11-05T09:00:00Z')!
    expect(breached.text).toBe('防线被破')
    expect(held.text).toBe('防线守住')
  })
})