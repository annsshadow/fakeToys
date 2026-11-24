// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R20 战报页服务端全量效率趋势卡：test-first 行为守卫。
 * 真实响应 / 空态 / SUM=0 / AI 四判据四项缺一不可；G132 本地卡不删。
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
  fetchBattleEfficiency: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

const history = () => [
  {
    id: 1, result: 'win', stars: 3, level_id: 4, score: 1234,
    kills: 20, leaked: 0, created_at: '2026-11-24T10:00:00Z',
  },
]

const efficiency = () => ({
  window_days: 30,
  window_from: '2026-10-25',
  window_to: '2026-11-24',
  battles: 3,
  shots: 20,
  hits: 13,
  accuracy_pct: 65,
  avg_heat_max: 42.25,
  leaked_per_battle: 1.5,
  avg_duration_ms: 61234.5,
  buckets: [
    { bucket: '2026-11-23', battles: 1, shots: 8, hits: 4, accuracy_pct: 50 },
    { bucket: '2026-11-24', battles: 2, shots: 12, hits: 9, accuracy_pct: 75 },
  ],
  ai_insight: {},
})

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: {} as any, serverTimeMs: 0 } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [] })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.myBattleHistory.mockResolvedValue({ items: history() } as any)
  mockApi.myBattleDetail.mockResolvedValue({} as any)
  mockApi.fetchBattleEfficiency.mockResolvedValue(efficiency() as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('R20 battles.vue 服务端全量效率趋势卡', () => {
  it('真实数据：四 KPI / 按日趋势条 / 窗口与诚实红线全部落地', async () => {
    const { wrapper } = mountPage(Battles)
    await flushPromises()

    expect(mockApi.fetchBattleEfficiency).toHaveBeenCalledWith(30)
    const card = wrapper.find('.r20-efficiency-trend')
    expect(card.exists()).toBe(true)
    expect(card.attributes('role')).toBe('region')

    const row = wrapper.find('.r20-efficiency-row')
    expect(row.text()).toContain('命中率 65.0%')
    expect(row.text()).toContain('平均峰值热量 42.3')
    expect(row.text()).toContain('场均漏怪 1.5')
    expect(row.text()).toContain('场均时长 61234.5ms')

    const bars = wrapper.findAll('.r20-trend-bar')
    expect(bars.map((b) => b.attributes('data-r20-bucket'))).toEqual(['2026-11-23', '2026-11-24'])
    expect(card.text()).toContain('2026-10-25')
    expect(card.text()).toContain('2026-11-24')
    expect(card.text()).toContain('辅助诊断非权威结算依据，权威请看 replay_hash')
  })

  it('空态：battles=0 逐字显示无记录，不画趋势条不造 AI 结论', async () => {
    mockApi.fetchBattleEfficiency.mockResolvedValue({
      ...efficiency(),
      battles: 0, shots: 0, hits: 0, accuracy_pct: 0,
      avg_heat_max: 0, leaked_per_battle: 0, avg_duration_ms: 0, buckets: [],
    } as any)
    const { wrapper } = mountPage(Battles)
    await flushPromises()

    const empty = wrapper.find('.r20-efficiency-empty')
    expect(empty.exists()).toBe(true)
    expect(empty.text()).toContain('近 30 天无战斗记录')
    expect(empty.text().replace(/\s/g, '')).toContain('近30天无战斗记录')
    expect(empty.text()).toContain('2026-10-25')
    expect(wrapper.findAll('.r20-trend-bar').length).toBe(0)
    expect(wrapper.find('.r20-ai-note').exists()).toBe(false)
    expect(wrapper.find('.r20-efficiency-trend').text()).not.toMatch(/NaN|undefined/)
  })

  it('SUM=0：命中率归零且不产 NaN/Infinity', async () => {
    mockApi.fetchBattleEfficiency.mockResolvedValue({
      ...efficiency(),
      battles: 2, shots: 0, hits: 0, accuracy_pct: 0,
      buckets: [
        { bucket: '2026-11-23', battles: 1, shots: 0, hits: 0, accuracy_pct: 0 },
        { bucket: '2026-11-24', battles: 1, shots: 0, hits: 0, accuracy_pct: 0 },
      ],
    } as any)
    const { wrapper } = mountPage(Battles)
    await flushPromises()

    expect(wrapper.find('.r20-efficiency-row').text()).toContain('命中率 0.0%')
    expect(wrapper.find('.r20-efficiency-trend').text()).not.toMatch(/NaN|Infinity/)
    const bars = wrapper.findAll('.r20-trend-bar')
    expect(bars.length).toBe(2)
    expect(bars.every((b) => (b.find('.r20-trend-fill').attributes('style') || '').includes('width: 0%'))).toBe(true)
  })

  it('AI 四判据：AI 标识 / 真实结论 / 依据行 / 可执行 CTA', async () => {
    const { wrapper } = mountPage(Battles)
    await flushPromises()

    const ai = wrapper.find('.r20-ai-note')
    expect(ai.exists()).toBe(true)
    expect(ai.text()).toContain('AI')
    expect(ai.text()).toContain('65.0%')
    expect(ai.text()).toContain('数据来自全量battle_records近30天')
    expect(ai.text()).toContain('近 30 天 battle_records 的 shots/hits/heat_max 聚合')
    const cta = ai.find('[data-r20-cta]')
    expect(cta.text()).toBe('去战斗→')
    await cta.trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/stage/stage' })
  })
})
