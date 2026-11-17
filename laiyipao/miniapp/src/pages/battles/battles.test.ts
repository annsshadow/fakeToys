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

/**
 * 服务端契约：/me/battles 的 result 是**全量分页过滤**参数
 * （routes.go myBattleHistory → Svc.MyBattleHistoryPage(..., result, ...)，
 * economy.go `if result != "" { where += " AND result = $4" }`）。
 * 因此筛选「胜利」后点「加载更多」，后续页必须仍带 result=win；
 * 否则服务端回的是未过滤混合页，失败战报会混进胜利列表。
 */
describe('battles.vue 筛选下的加载更多必须沿用服务端 result 过滤', () => {
  const win = (id: number) => ({
    id,
    result: 'win',
    stars: 3,
    level_id: 4,
    score: 1000 + id,
    kills: 12,
    leaked: 0,
    created_at: '2026-11-17T10:00:00Z',
  })
  const lose = (id: number) => ({
    id,
    result: 'lose',
    stars: 0,
    level_id: 5,
    score: 10,
    kills: 1,
    leaked: 9,
    created_at: '2026-11-15T10:00:00Z',
  })

  it('胜利筛选下加载更多：请求带 result=win，列表不出现失败战报', async () => {
    // 模拟服务端：只有带 result='win' 才过滤；缺失则回混合页（当前 bug 现状）。
    mockApi.myBattleHistory.mockImplementation((_limit?: number, offset = 0, result?: string, cursor?: number) => {
      const allWins = Array.from({ length: 40 }, (_, i) => win(100 + i))
      const start = cursor ? allWins.findIndex((w) => w.id === cursor) + 1 : offset
      const page = allWins.slice(start, start + (offset || 20))
      const next = start + (offset || 20) < allWins.length ? page[page.length - 1].id : 0
      if (result === 'win') return Promise.resolve({ items: page, next_cursor: next } as any)
      // 未过滤的第二页混入一场失败，模拟服务端不过滤的结果
      const mixed = offset === 0 ? page : [...page.slice(0, 19), lose(999)]
      return Promise.resolve({ items: mixed, next_cursor: next } as any)
    })

    const { wrapper } = mountPage(Battles)
    await flushPromises()

    // 切到「胜利」：reload 带 result=win 重新拉第一页
    const winChip = wrapper.findAll('.filter-chip').find((c) => c.text() === '胜利')!
    await winChip.trigger('click')
    await flushPromises()

    // 筛选后应全是胜场，且满一页 → 出现「加载更多」
    const before = wrapper.findAll('.result-badge')
    expect(before.length).toBe(20)
    expect(before.every((b) => b.text() === '胜')).toBe(true)
    expect(wrapper.find('.load-more').exists()).toBe(true)

    await wrapper.find('.load-more').trigger('click')
    await flushPromises()

    // 行为断言：胜利筛选下翻页后仍不能出现「负」
    const badges = wrapper.findAll('.result-badge')
    expect(badges.length).toBe(40)
    expect(badges.filter((b) => b.text() === '负').length).toBe(0)

    // 请求契约断言：第二次分页请求必须把筛选条件带给服务端
    const last = mockApi.myBattleHistory.mock.calls.at(-1)!
    expect(last[2]).toBe('win')
  })
})
/**
 * R17 行为复现：offset 翻页在「翻页途中插入新战报」时会重复/漏行；
 * 服务端已提供 keyset 游标（next_cursor / ?cursor=），客户端必须用它翻页。
 * 初始 newest→oldest = [50..1]；第一页加载后插入 id=100；offset 第二页返回 [30..11]，
 * 与已加载页产生重复并漏掉本来紧接的旧行。
 */
describe('battles.vue 翻页途中插入新战报：offset 重复漏行，cursor 不重复不漏', () => {
  const row = (id: number) => ({
    id, result: 'win' as const, stars: 3, level_id: 4, score: 100 + id,
    kills: 10, leaked: 0, created_at: '2026-11-17T10:00:00Z',
  })

  it('加载更多后列表每场唯一、从新到旧连续，不重复也不漏', async () => {
    let inserted = false
    mockApi.myBattleHistory.mockImplementation(function (_limit = 20, offset = 0, result = '', cursor?: number) {
      if (result) return Promise.resolve({ items: [] } as any)
      const all = Array.from({ length: 50 }, (_, i) => row(50 - i))
      if (inserted) all.unshift(row(100))
      const nextCursorOf = (page: any[]) => (page.length >= _limit ? page[page.length - 1].id : 0)
      if (cursor && cursor > 0) {
        const idx = all.findIndex((it) => it.id === cursor)
        const page = all.slice(idx + 1, idx + 1 + _limit)
        return Promise.resolve({ items: page, next_cursor: nextCursorOf(page) } as any)
      }
      const page = all.slice(offset, offset + _limit)
      return Promise.resolve({ items: page, next_cursor: nextCursorOf(page) } as any)
    })

    const { wrapper } = mountPage(Battles)
    await flushPromises()
    inserted = true
    await wrapper.find('.load-more').trigger('click')
    await flushPromises()

    const ids = wrapper.findAll('.battle-row').map((r) =>
      Number((r.find('.battle-score').text() || '0').replace(/\D/g, '')) - 100,
    )
    expect(new Set(ids).size).toBe(ids.length)
    expect(ids).toEqual(Array.from({ length: 40 }, (_, i) => 50 - i))
  })
})

/**
 * 服务端契约（routes.go myBattleHistory / economy.go MyBattleHistoryPage）：
 * - `hasMore = len(rows) > limit`（先多取 1 行判断），因此**恰好满一页**（总数正好 20）
 *   时 hasMore=false，响应 next_cursor=0；
 * - next_cursor 只有在 hasMore 为真时才是上一页末行 id，否则恒为 0。
 * 所以「本页是否满载」不等于「是否还有下一页」：满 20 条但 next_cursor=0 时没有更多页。
 *
 * 旧实现把 hasMore 写成 items.length >= 20，于是总数恰好 20/40/60… 时，
 * 明明服务端说没有下一页，页面却一直挂着「加载更多」，点下去只会重复拉同一页。
 */
describe('battles.vue 末页恰好满一页时不得再显示加载更多（以 next_cursor 为准）', () => {
  it('满 20 条但服务端 next_cursor=0：隐藏加载更多，不重复请求', async () => {
    const rows = Array.from({ length: 20 }, (_, i) => ({
      id: 40 - i,
      result: 'win' as const,
      stars: 3,
      level_id: 4,
      score: 1000 + (40 - i),
      kills: 10,
      leaked: 0,
      created_at: '2026-11-17T10:00:00Z',
    }))
    // 模拟服务端真实响应：总数正好 20 = 满页但无下一页
    mockApi.myBattleHistory.mockImplementation(() =>
      Promise.resolve({ items: rows, next_cursor: 0 } as any),
    )

    const { wrapper } = mountPage(Battles)
    await flushPromises()

    const callsAfterLoad = mockApi.myBattleHistory.mock.calls.length
    expect(callsAfterLoad).toBe(1)
    expect(wrapper.findAll('.battle-row').length).toBe(20)

    // 行为断言：服务端已明确没有下一页，加载更多按钮不应出现
    expect(wrapper.find('.load-more').exists()).toBe(false)
  })
})
