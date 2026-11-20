// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 140 轮：battles.vue 战报详情展开「加载中」竞态回归。
 *
 * detailLoading 原为全局单例 ref<boolean>：并行拉取两条战报详情时，
 * 先发请求的 finally 会把后发请求的「加载中」误清掉 ——
 * 用户展开战报 B 后，A 的迟到响应先回，B 仍在飞，
 * 展开区却既不显示「加载中」也没有详情，变成一块空白。
 * 本用例钉住：加载提示必须按战报 id 归属，互不串扰。
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
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  myBattleHistory: vi.fn(),
  myBattleDetail: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

const history = () => [
  { id: 1, result: 'win', stars: 3, level_id: 4, score: 1234, kills: 20, leaked: 0, created_at: '2026-11-17T10:00:00Z' },
  { id: 2, result: 'lose', stars: 0, level_id: 5, score: 99, kills: 3, leaked: 12, created_at: '2026-11-16T10:00:00Z' },
]

const detail = (id: number) => ({
  id, level_id: id, result: 'win', stars: 3, score: 100 + id,
  kills: 5, leaked: 0, hp_left: 80, wave_reached: 3, duration_ms: 12000,
  shots: id * 10, hits: id * 5, reactions: 1, heat_max: 30,
  elements_used: {}, reactions_used: {}, terrain_used: [], loot: { coin: 10 },
  replay_hash: 'hash' + id, created_at: '2026-11-18T00:00:00Z',
})

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
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('battles.vue 战报详情展开加载提示竞态（第 140 轮）', () => {
  it('先发的慢响应先回、后发的仍在飞时，后展开行必须继续显示加载中', async () => {
    let resolveA!: (v: any) => void
    let resolveB!: (v: any) => void
    const pendingA = new Promise<any>((res) => { resolveA = res })
    const pendingB = new Promise<any>((res) => { resolveB = res })
    mockApi.myBattleDetail.mockImplementation((id: any) => (id === 1 ? pendingA : pendingB) as any)

    const { wrapper } = mountPage(Battles)
    await flushPromises()
    const rows = wrapper.findAll('.battle-row')
    expect(rows.length).toBe(2)

    // 展开战报 1：其详情请求挂在半空，展开区显示「加载中」
    await rows[0]!.trigger('click')
    await flushPromises()
    expect(wrapper.find('.battle-detail').text()).toContain('加载中')

    // 紧接着展开战报 2：请求也挂在半空（模拟更慢的网络）
    await rows[1]!.trigger('click')
    await flushPromises()
    expect(wrapper.find('.battle-detail').text()).toContain('加载中')

    // 战报 1 的响应先回；此时展开的是战报 2，它仍在飞。
    resolveA(detail(1))
    await flushPromises()
    // 行为断言：后展开的战报 2 必须继续显示「加载中」，不能变空白。
    expect(wrapper.find('.battle-detail').exists()).toBe(true)
    expect(wrapper.find('.battle-detail').text()).toContain('加载中')

    // 战报 2 的响应最后回来，详情正常落地。
    resolveB(detail(2))
    await flushPromises()
    expect(wrapper.find('.battle-detail').text()).toContain('hash2')
    expect(wrapper.find('.battle-detail').text()).toContain('20 / 10')
  })
})
