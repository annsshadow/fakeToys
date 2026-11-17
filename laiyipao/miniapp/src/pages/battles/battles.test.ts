// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 战报页（battles.vue）R19 视觉分层测试。
 *
 * 此前该页只有源码扫描守卫，胜负色差这类「看得见」的改动无法被 DOM 断言捕获。
 * 本文件用真实挂载验证：胜/负行给不同类、结果徽章落地、空态有引导锚点。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Battles from './battles.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  myBattleHistory: vi.fn(),
  myBattleDetail: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

const history = () => [
  {
    id: 1, result: 'win', stars: 3, level_id: 4, score: 1234,
    kills: 20, leaked: 0, created_at: '2026-11-17T10:00:00Z',
  },
  {
    id: 2, result: 'lose', stars: 0, level_id: 5, score: 99,
    kills: 3, leaked: 12, created_at: '2026-11-16T10:00:00Z',
  },
]

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: {} as any, serverTimeMs: 0 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [] })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.myBattleHistory.mockResolvedValue({ items: history() } as any)
  mockApi.myBattleDetail.mockResolvedValue({} as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('battles.vue 战报行视觉分层（R19）', () => {
  it('胜/负行给不同类，结果徽章落地', async () => {
    const { wrapper } = mountPage(Battles)
    await flushPromises()
    const rows = wrapper.findAll('.battle-row')
    expect(rows.length).toBe(2)
    expect(rows[0]!.classes()).toContain('win')
    expect(rows[0]!.classes()).not.toContain('lose')
    expect(rows[1]!.classes()).toContain('lose')
    expect(rows[1]!.classes()).not.toContain('win')

    const badges = wrapper.findAll('.result-badge')
    expect(badges.length).toBe(2)
    expect(badges[0]!.classes()).toContain('win')
    expect(badges[0]!.text()).toBe('胜')
    expect(badges[1]!.classes()).toContain('lose')
    expect(badges[1]!.text()).toBe('负')
  })

  it('空态给引导锚点（不再是孤零零一行文字）', async () => {
    mockApi.myBattleHistory.mockResolvedValue({ items: [] } as any)
    const { wrapper } = mountPage(Battles)
    await flushPromises()
    expect(wrapper.find('.empty-state').exists()).toBe(true)
    expect(wrapper.find('.empty-glyph').exists()).toBe(true)
    expect(wrapper.text()).toContain('去关卡页挑一关开打')
  })
})
