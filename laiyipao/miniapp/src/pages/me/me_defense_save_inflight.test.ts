// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 125 轮 laiyipao 真 bug（R275）：me.vue 的防线保存 / 护盾切换缺少在途守卫。
 *
 * saveDefense() 与 toggleShield() 都直接 POST /defenses/save 写同一条防线记录，
 * 且按钮没有 disabled。服务端是非幂等 upsert（每次重算快照哈希、刷新 updated_at），
 * 双点会发两次请求；交叉点时两次的 shield_hours 不同（0 vs 24），
 * 后到的那次会把先到的护盾状态覆盖 —— 玩家点了「开启护盾」却可能没开成。
 * 本测试让首次请求挂起，在途期间再点一次，断言只提交一次。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
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
  deleteMyDefense: vi.fn(),
  challengeDefense: vi.fn(),
  fetchLevel: vi.fn(),
  fetchMyEquipment: vi.fn(),
  fetchMastery: vi.fn(),
  updateNickname: vi.fn(),
  myChallengeHistory: vi.fn(),
  myDefenseChallenges: vi.fn(),
  fetchMyStars: vi.fn(),
  myBattleHistory: vi.fn(),
  myPurchases: vi.fn(),
  myRedemptions: vi.fn(),
  myOrders: vi.fn(),
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
    id: 5,
    name: '我的防线',
    owner_name: '对手',
    power: 100,
    element_coverage: 3,
    wins: 1,
    losses: 2,
    snapshot_hash: 'abcdef1234567890',
    snapshot: { skills: [1] },
    can_challenge: true,
    challenge_blocked: '',
    ...over,
  } as DefenseView
}

let releaseSave: ((v: unknown) => void) | null = null

beforeEach(() => {
  installUniMock()
  releaseSave = null
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
  mockApi.fetchMyEquipment.mockResolvedValue({
    items: [{ id: 1, code: 'w_p1', name: '炮', slot: 'weapon', tier: 1, element: 'kinetic',
      descr: '', base_armor: 0, base_bonus_pct: 50, unlock_level: 1, owned: true,
      equipped: true, level: 1, star: 1 }],
  } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11], families: [] } as any)
  mockApi.fetchLevel.mockResolvedValue(makeLevel({ id: 1, waves: [{ wave_index: 0, spawns: [] }] }) as any)
  mockApi.fetchDefenses.mockResolvedValue({
    mine: defenseView({ shielded_until: '', my_attempts_today: 0 } as any),
    candidates: [], attempt_limit: 3,
  } as any)
  // 首次 saveDefense 挂起，构造「写请求在途」窗口；每次调用都换一个 resolver。
  mockApi.saveDefense.mockImplementation((() => new Promise((resolve) => {
    releaseSave = resolve
  })) as any)
  mockApi.myChallengeHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myDefenseChallenges.mockResolvedValue({ items: [] } as any)
  mockApi.fetchMyStars.mockResolvedValue({ stars: {} } as any)
  mockApi.myBattleHistory.mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockResolvedValue({ items: [] } as any)
})

afterEach(async () => {
  releaseSave?.({ defense: defenseView() })
  await flushPromises()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me.vue 防线保存/护盾在途守卫（R275）', () => {
  it('保存防线请求在途时重复点击保存按钮不会重复提交', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()

    const saveBtn = () => wrapper.findAll('.btn').find((b) => b.text().includes('保存当前构筑为防线'))!
    expect(saveBtn()).toBeTruthy()

    await saveBtn().trigger('click')
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledTimes(1)
    expect(releaseSave).not.toBeNull()

    // 在途期间再点保存：无守卫时会再发一次 POST /defenses/save
    await saveBtn().trigger('click')
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledTimes(1)

    releaseSave?.({ defense: defenseView() })
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledTimes(1)
  })

  it('保存请求在途时点击护盾按钮不会交叉提交覆盖护盾状态', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()

    const saveBtn = () => wrapper.findAll('.btn').find((b) => b.text().includes('保存当前构筑为防线'))!
    const shieldBtn = () => wrapper.findAll('.btn').find((b) => b.text().includes('24h 护盾'))!
    expect(saveBtn()).toBeTruthy()
    expect(shieldBtn()).toBeTruthy()

    await saveBtn().trigger('click')
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledTimes(1)
    // 第一次提交 shield_hours=0（保存防线）
    expect(mockApi.saveDefense).toHaveBeenCalledWith(expect.objectContaining({ shield_hours: 0 }))

    // 在途期间点开盾：两个函数写同一条 defense 记录，无共享守卫时会交叉提交 shield_hours=24
    await shieldBtn().trigger('click')
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledTimes(1)

    releaseSave?.({ defense: defenseView() })
    await flushPromises()
  })
})
