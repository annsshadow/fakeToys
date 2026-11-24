// @vitest-environment jsdom
/**
 * R25 单局战报反应伤害主力聚合卡：渲染侧 RED→GREEN 测试。
 * 纯前端派生：count 取权威 reactions_used，events 不冒充次数；诚实空态与 AI 四判据。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Battles from './battles.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(), fetchConfigIfVersion: vi.fn(), fetchWallet: vi.fn(), fetchMe: vi.fn(),
  fetchLoadout: vi.fn(), saveLoadout: vi.fn(), setTokenInternal: vi.fn(), hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}), loadProfile: vi.fn(() => null), saveProfile: vi.fn(), login: vi.fn(),
  saveLastUsername: vi.fn(), startBattle: vi.fn(), settleBattle: vi.fn(), abandonBattle: vi.fn(),
  myBattleHistory: vi.fn(), myBattleDetail: vi.fn(),
}))
const mockApi = vi.mocked(api, true)

const row = {
  id: 9, result: 'win', stars: 3, level_id: 1, score: 15000,
  kills: 39, leaked: 0, created_at: '2026-11-23T16:00:00Z',
}
function detailWith(over: Record<string, unknown> = {}): Record<string, unknown> {
  return {
    id: 9, level_id: 1, result: 'win', stars: 3, score: 15000,
    kills: 39, leaked: 0, hp_left: 300, wave_reached: 5, duration_ms: 70000,
    shots: 300, hits: 260, reactions: 3, heat_max: 70,
    elements_used: { fire: 90, lightning: 80 },
    reactions_used: { steam_burst: 7, overheat: 3 },
    terrain_used: [], loot: { coin: 100 }, replay_hash: 'r25-report',
    created_at: '2026-11-23T16:00:00Z',
    ...over,
  }
}
async function openDetail(over: Record<string, unknown> = {}) {
  mockApi.myBattleHistory.mockResolvedValue({ items: [row], next_cursor: 0 } as any)
  mockApi.myBattleDetail.mockResolvedValue(detailWith(over) as any)
  const { wrapper } = mountPage(Battles)
  await flushPromises()
  await wrapper.find('.battle-row').trigger('click')
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: { reactions: [
      { key: 'steam_burst', name: '蒸汽爆发' }, { key: 'overheat', name: '过热' },
    ] } as any,
    serverTimeMs: 0,
  })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [] })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
})
afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })

describe('battles.vue R25 单局反应伤害主力聚合卡', () => {
  it('按reaction分组聚合伤害/占比降序，count取权威reactions_used而非events条数', async () => {
    const wrapper = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1',
        events: [
          { wave: 1, tick: 1, reaction: 'steam_burst', enemy_uid: 11, damage: 40 },
          { wave: 2, tick: 2, reaction: 'overheat', enemy_uid: 22, damage: 30 },
          { wave: 2, tick: 5, reaction: 'steam_burst', enemy_uid: 33, damage: 20 },
        ],
        truncated: false, total: 3,
      },
    })
    const card = wrapper.find('.r25-reaction-impact')
    expect(card.exists()).toBe(true)
    const rows = wrapper.findAll('.r25-impact-row')
    expect(rows.length).toBe(2)
    expect(rows.map(r => r.attributes('data-r25-reaction'))).toEqual(['steam_burst', 'overheat'])
    const steam = rows[0]!.text()
    expect(steam).toContain('蒸汽爆发')
    expect(steam).toContain('60.0')
    expect(steam).toContain('66.7%')
    expect(steam).toContain('权威7次')
    expect(steam).toContain('事件2条')
    expect(rows[1]!.text()).toContain('33.3%')
    wrapper.unmount()
  })

  it('旧战报无时间线/空数组/结构异常 → 逐字空态，不编造主力', async () => {
    for (const over of [{}, { reaction_timeline: { events: [] } }, { reaction_timeline: { events: 'broken' } }]) {
      const wrapper = await openDetail(over)
      expect(wrapper.find('.r25-impact-empty').text()).toBe('本局无逐次反应伤害记录')
      expect(wrapper.findAll('.r25-impact-row').length).toBe(0)
      wrapper.unmount()
    }
  })

  it('truncated 提示 + AI 四判据 + 逐字依据行与诚实红线', async () => {
    const wrapper = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1',
        events: [
          { wave: 1, tick: 1, reaction: 'steam_burst', enemy_uid: 11, damage: 50 },
          { wave: 1, tick: 2, reaction: 'overheat', enemy_uid: 22, damage: 50 },
        ],
        truncated: true, total: 201,
      },
    })
    expect(wrapper.find('.r25-truncated-note').text()).toBe('已截断，仅统计前 200 条')
    const note = wrapper.find('.r25-ai-note')
    expect(note.exists()).toBe(true)
    expect(note.text()).toMatch(/AI/)
    expect(note.text()).toContain('蒸汽爆发')
    expect(note.text()).toContain('50.0%')
    expect(note.text()).toContain('依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准')
    expect(note.text()).toContain('下一局优先凑齐该元素组合')
    expect(note.text()).toContain('辅助展示非伤害权威,权威以 replay_hash 为准')
    wrapper.unmount()
  })
})
