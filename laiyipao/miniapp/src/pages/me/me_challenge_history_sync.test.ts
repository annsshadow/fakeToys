// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 95 轮 laiyipao 真 bug（R245）：me.vue 挑战成功后未重拉挑战历史。
 * 服务端在 challengeDefense 成功时向 defense_challenges 写入挑战者的新记录，
 * 但页面只在 onShow 拉取挑战历史；挑战是页内动作，不会重新进入页面，
 * 因此「记录 → 挑战历史」卡片会一直显示进入页面时的旧快照（含统计）。
 * 本测试让 onShow 建立旧快照，随后在页内完成一次成功挑战，
 * 断言挑战历史与统计都按服务端新返回刷新（条数与胜场）。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
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

function outcome(over: Partial<ChallengeOutcome> = {}): ChallengeOutcome {
  return {
    won: true,
    report: { seed: '42', won: true, duration_ms: 1000, hp_left_pct: 0, replay_hash: 'aaaabbbb' },
    stats: { kills: 3, leaked: 0, waves: 2, score: 100, durationMs: 1000, hpLeftPct: 0 },
    ...over,
  } as ChallengeOutcome
}

function historyItem(over: Partial<api.ChallengeHistoryItem> = {}): api.ChallengeHistoryItem {
  return {
    id: 1,
    defense_id: 5,
    defense_name: '旧防线',
    owner_nickname: '旧对手',
    owner_power: 90,
    won: false,
    duration_ms: 900,
    hp_left_pct: 100,
    loot: {},
    created_at: '2026-11-01 10:00:00',
    ...over,
  } as api.ChallengeHistoryItem
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
    config: makeConfig({ levels: [makeLevel()], skills: [makeSkill({ id: 1, apply_element: '' })] }),
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] })
  mockApi.fetchMyEquipment.mockResolvedValue({
    items: [
      {
        id: 1,
        code: 'w_p1',
        name: '制式炮管',
        slot: 'weapon',
        tier: 1,
        element: 'kinetic',
        descr: '',
        base_armor: 0,
        base_bonus_pct: 50,
        unlock_level: 1,
        owned: true,
        equipped: true,
        level: 1,
        star: 1,
      },
    ],
  })
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11], families: [] })
  mockApi.fetchDefenses.mockResolvedValue({
    mine: defenseView({ shielded_until: '2099-01-02T00:00:00Z', my_attempts_today: 2 } as any),
    candidates: [defenseView()],
    attempt_limit: 5,
  } as any)
  mockApi.saveDefense.mockResolvedValue({ defense: defenseView({ snapshot_hash: 'feedface1234' }) } as any)
  mockApi.challengeDefense.mockResolvedValue({ stolen: { coin: 10, gem: 2, keys: 1, energy: 3, vip: 9 } })
  mockApi.fetchLevel.mockResolvedValue(makeLevel({ id: 1, waves: [{ wave_index: 0, spawns: [] }] }) as any)
  runChallengeMock.mockReturnValue(outcome())

  // 挑战历史：onShow 建立旧快照；挑战成功后服务端应返回新增一条的新快照。
  mockApi.myChallengeHistory.mockReset()
    .mockResolvedValueOnce({ items: [historyItem()] } as any)
    .mockResolvedValue({
      items: [
        historyItem(),
        historyItem({
          id: 2,
          defense_id: 5,
          defense_name: '新防线',
          owner_nickname: '新对手',
          won: true,
          loot: { coin: 10 },
          created_at: '2026-11-19 00:00:00',
        }),
      ],
    } as any)
  mockApi.myDefenseChallenges.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.fetchMyStars.mockReset().mockResolvedValue({ stars: {} } as any)
  mockApi.myBattleHistory.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockReset().mockResolvedValue({ items: [] } as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me.vue 挑战成功后同步挑战历史（R245）', () => {
  it('页内挑战成功后挑战历史卡片与统计按服务端最新返回刷新', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()
    // 进入页面时由 onShow 建立旧快照：1 条、0 胜 1 负。
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.myChallengeHistory).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('1 条')
    expect(wrapper.text()).toContain('旧防线 · 旧对手')

    // 页内完成一次成功挑战：服务端写入新记录，页面应重拉历史。
    await wrapper.findAll('.cand .btn')[0]!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()

    expect(mockApi.challengeDefense).toHaveBeenCalledTimes(1)
    // 核心行为断言：服务端新历史必须反映到页内，否则恒停在旧快照。
    expect(mockApi.myChallengeHistory).toHaveBeenCalledTimes(2)
    expect(wrapper.text()).toContain('2 条')
    expect(wrapper.text()).toContain('新防线 · 新对手')
    // 统计随 challengeHistory 重算：由 0 胜 1 负变为 1 胜 1 负。
    expect(wrapper.text()).toContain('1 胜 1 负')
  })
})
