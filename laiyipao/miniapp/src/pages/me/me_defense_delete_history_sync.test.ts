// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 103 轮 laiyipao 真 bug（R246）：me.vue 删除防线后未重拉「被挑战记录」。
 *
 * 服务端 DeleteMyDefense 直接 DELETE defenses，defense_challenges 随外键
 * ON DELETE CASCADE 一起消失（progression.go:811 注释明确）。但页面
 * onDeleteMyDefense 成功后只调 loadDefenses()，没有重拉
 * loadDefenseChallengeHistory()；这是页内动作（不触发 onShow），
 * 于是「被挑战记录」卡片会一直显示进入页面时的旧快照，
 * 哪怕服务端权威态里这些记录已经不存在了。
 *
 * 本测试让 onShow 建立含 1 条旧记录的快照，页内删除防线后，
 * 断言被挑战历史按服务端新返回（空）刷新。
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
    validateSnapshot: vi.fn((c: DefenseView) => ({ ok: !String(c.snapshot_hash).startsWith('bad') })),
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

function defenseChallengeItem(over: Record<string, unknown> = {}): Record<string, unknown> {
  return {
    id: 11,
    defense_id: 5,
    defense_name: '我的防线',
    challenger_nickname: '老挑战者',
    won: true,
    duration_ms: 1000,
    hp_left_pct: 0,
    loot: { coin: 1 },
    created_at: '2026-11-01 10:00:00',
    ...over,
  }
}

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
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
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] })
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11], families: [] })
  mockApi.fetchDefenses.mockResolvedValue({
    mine: defenseView({ shielded_until: '', my_attempts_today: 0 } as any),
    candidates: [],
    attempt_limit: 5,
  } as any)
  mockApi.saveDefense.mockResolvedValue({ defense: defenseView() } as any)
  mockApi.deleteMyDefense.mockResolvedValue({ ok: true } as any)
  mockApi.fetchLevel.mockResolvedValue(makeLevel({ id: 1, waves: [{ wave_index: 0, spawns: [] }] }) as any)
  // 被挑战历史：onShow 建立旧快照；删除防线后服务端 CASCADE 清空，应返回空。
  mockApi.myDefenseChallenges.mockReset()
    .mockResolvedValueOnce({ items: [defenseChallengeItem()] } as any)
    .mockResolvedValue({ items: [] } as any)
  mockApi.myChallengeHistory.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.fetchMyStars.mockReset().mockResolvedValue({ stars: {} } as any)
  mockApi.myBattleHistory.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockReset().mockResolvedValue({ items: [] } as any)
  // 删除防线必须二次确认：这里模拟玩家点了「删除」。
  um.mock.showModal.mockImplementation((opts: any) => {
    opts.success?.({ confirm: true, cancel: false })
  })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me.vue 删除防线后同步被挑战记录（R246）', () => {
  it('页内删除防线后被挑战记录按服务端最新返回清空', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()

    expect(mockApi.myDefenseChallenges).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('被挑战记录')
    expect(wrapper.text()).toContain('1 条')
    expect(wrapper.text()).toContain('老挑战者')

    const delBtn = wrapper.findAll('.btn').find((b) => b.text().includes('删除防线'))
    expect(delBtn).toBeDefined()
    await delBtn!.trigger('click')
    await new Promise((r) => setTimeout(r, 30))
    await flushPromises()

    expect(mockApi.deleteMyDefense).toHaveBeenCalledTimes(1)
    // 核心行为断言：服务端已随防线删除清空记录，页内必须重拉并去掉旧条目。
    expect(mockApi.myDefenseChallenges).toHaveBeenCalledTimes(2)
    expect(wrapper.text()).not.toContain('老挑战者')
  })
})
