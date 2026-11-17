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
    // R78 筛选结果计数
    expect(wrapper.find('.filter-count').text()).toContain('共 2 场')
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

describe('battles.vue 趋势胜负色必须与战报 result 一致', () => {
  /**
   * 服务端 result 只有 'win'/'lose' 两种值（game.go resultStr），
   * 而 stars 由 computeStars(score, targets) 独立按分数门槛算、与胜负无关
   * （battle.go）。所以 result='lose' 但 stars>0 是合法真实数据：
   * 分数够门槛、但防线失守被判负。
   *
   * 列表徽章/连胜/胜率都用 result==='win' 把它判成「负」，
   * 趋势图若用 stars>0 代替 result，就会把同一条「负」战报画成绿色胜利点 ——
   * 同一份数据在页面上给出互相矛盾的结论。
   */
  it('败局带星（result=lose, stars>0）的趋势点必须画成负色 trend-lose，与列表徽章一致', async () => {
    mockApi.myBattleHistory.mockResolvedValue({
      items: [
        {
          id: 1, result: 'win', stars: 3, level_id: 4, score: 1200,
          kills: 20, leaked: 0, created_at: '2026-11-17T10:00:00Z',
        },
        {
          id: 2, result: 'lose', stars: 2, level_id: 5, score: 900,
          kills: 10, leaked: 8, created_at: '2026-11-16T10:00:00Z',
        },
      ],
    } as any)
    const { wrapper } = mountPage(Battles)
    await flushPromises()

    // 列表徽章权威：items[1] 是「负」
    const badges = wrapper.findAll('.result-badge')
    expect(badges.length).toBe(2)
    expect(badges[1]!.text()).toBe('负')

    // 趋势圆点是时间正序（旧→新）：pts[0] 对应 items[1]（那条 lose+2星）
    const dots = wrapper.findAll('.trend-svg circle')
    expect(dots.length).toBe(2)
    // 同一条「负」战报在趋势图也必须是负色，不能是绿色胜利点
    expect(dots[0]!.classes()).toContain('trend-lose')
    expect(dots[0]!.classes()).not.toContain('trend-win')
    // 真正的胜场仍是胜利色
    expect(dots[1]!.classes()).toContain('trend-win')
  })
})
