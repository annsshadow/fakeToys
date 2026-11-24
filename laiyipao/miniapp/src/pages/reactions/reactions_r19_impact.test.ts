// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R19 玩家反应伤害贡献卡渲染测试（test-first）。
 *
 * 防的是"能力存在但玩家看不到"：后端按 user_id 聚合的真实伤害/权威次数
 * 必须在 reactions.vue 落成可切换的贡献卡，且辅助分析绝不冒充权威结算。
 */
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import Reactions from './reactions.vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { makeConfig } from '../../test/fixtures'

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
  fetchMyReactionDamageImpact: vi.fn(),
}))

vi.mock('@/pages/_shared/r6Recommendation', async (importOriginal) => {
  const orig = await importOriginal<typeof import('@/pages/_shared/r6Recommendation')>()
  return { ...orig, fetchRecommendations: vi.fn() }
})

const mockApi = vi.mocked(api, true)

function reactionFixtures() {
  return [
    { key: 'steam_burst', name: '蒸汽爆发', base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0, aoe_radius: 120, spread_stacks: 0, dispel_shield: true, amplify_pct: 0, armor_shred_permille: 0, knockback: 0, descr: '' },
    { key: 'overheat', name: '过热', base_coef: 55, attack_weight_pct: 300, status_duration_ms: 1500, aoe_radius: 90, spread_stacks: 0, dispel_shield: false, amplify_pct: 0, armor_shred_permille: 0, knockback: 0, descr: '' },
  ]
}

function impactFixture() {
  return {
    window_days: 30,
    window_from: '2026-10-25',
    window_to: '2026-11-24',
    excluded_old_battles: 2,
    items: [
      { reaction: 'overheat', total_damage: 100, damage_share_pct: 10, events: 12, count: 20 },
      { reaction: 'steam_burst', total_damage: 605, damage_share_pct: 60.5, events: 4, count: 3 },
    ],
  }
}

async function mountPage(impact: unknown = impactFixture()) {
  const pinia = createPinia()
  setActivePinia(pinia)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig({ reactions: reactionFixtures() as never }), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 1, energy: 1, keys: 1, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({
    user_id: 7,
    build: { skills: {}, equipment: [], mastery_nodes: [] },
    build_rating: {},
    power: 0,
    max_stage: 1,
    account: { created_at: '2026-01-01T00:00:00Z', last_login_at: '2026-01-02T00:00:00Z', is_guest: false, nickname: '测试' },
  })
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] })
  if (impact === 'reject') mockApi.fetchMyReactionDamageImpact.mockRejectedValue(new Error('网络错误'))
  else mockApi.fetchMyReactionDamageImpact.mockResolvedValue(impact as never)
  const wrapper = mount(Reactions, { global: { plugins: [pinia] } })
  await flushPromises()
  const store = useGameStore()
  await store.loadConfig()
  await store.refreshProfile()
  await store.loadLoadout()
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  installUniMock()
  vi.clearAllMocks()
  mockApi.hasUsableToken.mockReturnValue(false)
  mockApi.loadProfile.mockReturnValue(null)
})

describe('R19 玩家反应伤害贡献卡', () => {
  it('双视图：默认次数视图显示权威次数，切伤害视图按占比降序', async () => {
    const wrapper = await mountPage()
    expect(wrapper.find('.r19-my-reaction-impact').exists(), '.r19-my-reaction-impact 必须存在').toBe(true)
    expect(wrapper.findAll('.r19-rdi-row').length).toBe(2)
    expect(wrapper.find('.r19-rdi-row[data-r19-reaction="overheat"]').exists()).toBe(true)
    expect(wrapper.text()).toContain('权威 20 次')
    const toggle = wrapper.findAll('.r19-view-toggle .r19-toggle-btn')
    expect(toggle.length).toBe(2)
    await toggle[1]!.trigger('click')
    const rows = wrapper.findAll('.r19-rdi-row')
    expect(rows[0]!.attributes('data-r19-reaction')).toBe('steam_burst')
    expect(rows[0]!.text()).toContain('60.5%')
    expect(wrapper.text()).not.toContain('NaN')
    await toggle[0]!.trigger('click')
    expect(wrapper.findAll('.r19-rdi-row')[0]!.attributes('data-r19-reaction')).toBe('overheat')
    wrapper.unmount()
  })

  it('AI四判据与诚实红线、窗口字段均来自真实数据', async () => {
    const wrapper = await mountPage()
    const note = wrapper.find('.r19-ai-note')
    expect(note.exists(), '.r19-ai-note 必须存在').toBe(true)
    expect(note.text()).toContain('AI')
    expect(note.text()).toContain('过热 触发 20 次')
    expect(note.text()).toContain('依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准')
    expect(note.text()).toContain('查看权威结算')
    expect(wrapper.text()).toContain('辅助分析非权威结算依据，权威请看 replay_hash')
    expect(wrapper.text()).toContain('2026-10-25')
    expect(wrapper.text()).toContain('2026-11-24')
    expect(wrapper.text()).toContain('已排除 2 条无逐次记录的旧战报')
    wrapper.unmount()
  })

  it('空态：items=[] 与 API 失败均走逐字空态，不显示 NaN/undefined', async () => {
    const empty = await mountPage({ ...impactFixture(), items: [], excluded_old_battles: 0 })
    expect(empty.find('.r19-rdi-empty').exists()).toBe(true)
    expect(empty.find('.r19-rdi-empty').text()).toBe('近 30 天无逐次反应伤害记录')
    expect(empty.findAll('.r19-rdi-row').length).toBe(0)
    expect(empty.text()).not.toContain('NaN')
    expect(empty.text()).not.toContain('undefined')
    expect(empty.text()).toContain('已排除 0 条无逐次记录的旧战报')
    empty.unmount()

    const failed = await mountPage('reject')
    expect(failed.find('.r19-rdi-empty').exists()).toBe(true)
    expect(failed.text()).not.toContain('NaN')
    failed.unmount()
  })

  it('占比为 0 时显示 0.0% 不产生 NaN', async () => {
    const wrapper = await mountPage({
      ...impactFixture(),
      items: [{ reaction: 'overheat', total_damage: 0, damage_share_pct: 0, events: 0, count: 5 }],
    })
    expect(wrapper.text()).toContain('0.0%')
    expect(wrapper.text()).not.toContain('NaN')
    wrapper.unmount()
  })
})
