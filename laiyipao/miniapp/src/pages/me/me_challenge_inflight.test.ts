// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 123 轮 laiyipao 真 bug（R273）：me.vue 挑战防线缺少在途守卫。
 *
 * `challenge()` 把 simulating 仅用于按钮文案，未做 return 守卫；
 * 模拟完成、challengeDefense 请求仍在途时（simulating 仍置位），
 * 再点另一个/同一个目标会再发一次 challengeDefense —— 服务端每次
 * 上报都记 attempts+1 并发放窃取，双点即双倍消耗每日挑战次数。
 * 本测试让首次上报挂起，在途期间再点一次，断言只上报一次。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel, makeSkill } from '../../test/fixtures'
import type { ChallengeOutcome, DefenseView } from '@/game/defense'

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
    validateSnapshot: vi.fn((c: DefenseView) => ({ ok: !String(c.snapshot_hash).startsWith('bad') })),
    isShieldActive: actual.isShieldActive,
  }
})

const mockApi = vi.mocked(api, true)
const defenseModule = await import('@/game/defense')
const runChallengeMock = vi.mocked(defenseModule.runChallenge)

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

function outcome(): ChallengeOutcome {
  return {
    won: true,
    report: { seed: '42', won: true, duration_ms: 1000, hp_left_pct: 0, replay_hash: 'aaaabbbb' },
    stats: { kills: 3, leaked: 0, waves: 2, score: 100, durationMs: 1000, hpLeftPct: 0 },
  } as ChallengeOutcome
}

let releaseChallenge: ((v: unknown) => void) | null = null

beforeEach(() => {
  installUniMock()
  releaseChallenge = null
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
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
    items: [
      {
        id: 1, code: 'w_p1', name: '制式炮管', slot: 'weapon', tier: 1, element: 'kinetic',
        descr: '', base_armor: 0, base_bonus_pct: 50, unlock_level: 1, owned: true,
        equipped: true, level: 1, star: 1,
      },
    ],
  } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11], families: [] } as any)
  mockApi.fetchDefenses.mockResolvedValue({
    mine: defenseView({ shielded_until: '', my_attempts_today: 0 } as any),
    candidates: [
      defenseView({ id: 5, name: '防线A', owner_name: '甲' }),
      defenseView({ id: 6, name: '防线B', owner_name: '乙' }),
    ],
    attempt_limit: 5,
  } as any)
  mockApi.saveDefense.mockResolvedValue({ defense: defenseView() } as any)
  // 首次 challengeDefense 挂起，用于构造「请求在途」窗口
  mockApi.challengeDefense.mockImplementation((() => new Promise((resolve) => {
    releaseChallenge = resolve
  })) as any)
  mockApi.fetchLevel.mockResolvedValue(makeLevel({ id: 1, waves: [{ wave_index: 0, spawns: [] }] }) as any)
  runChallengeMock.mockReturnValue(outcome())

  mockApi.myChallengeHistory.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myDefenseChallenges.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.fetchMyStars.mockReset().mockResolvedValue({ stars: {} } as any)
  mockApi.myBattleHistory.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockReset().mockResolvedValue({ items: [] } as any)
})

afterEach(async () => {
  releaseChallenge?.({ stolen: {} })
  await flushPromises()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me.vue 挑战防线在途守卫（R273）', () => {
  it('首次挑战上报在途时再次点击不会重复上报服务端', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()

    const buttons = wrapper.findAll('.cand .btn')
    expect(buttons.length).toBe(2)

    // 第一次挑战：等 30ms 延迟回调跑完，challengeDefense 已进入在途（挂起）
    await buttons[0]!.trigger('click')
    await flushPromises()
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(mockApi.challengeDefense).toHaveBeenCalledTimes(1)
    expect(releaseChallenge).not.toBeNull()

    // 在途期间再点另一个目标：无守卫时会再发一次 challengeDefense
    await buttons[1]!.trigger('click')
    await flushPromises()
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(mockApi.challengeDefense).toHaveBeenCalledTimes(1)
  })
})
