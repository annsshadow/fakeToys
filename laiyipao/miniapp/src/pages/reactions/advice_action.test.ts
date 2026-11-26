// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/** R14 §C.2/C.3：图鉴每条按服务端权威元素取 CTA；空态无按钮；两步确认后才写。 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Reactions from './reactions.vue'
import * as api from '@/api/client'
import { fetchAdviceAction, applyAdviceAction, normalizeAdviceAction } from '@/pages/_shared/r14AdviceAction'
import { useGameStore } from '@/store/game'
import { installUniMock } from '../../test/uni-mock'
import { makeConfig } from '../../test/fixtures'
import { mountPage } from '../../test/page'
vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(), fetchConfigIfVersion: vi.fn(), fetchWallet: vi.fn(), fetchMe: vi.fn(),
  fetchLoadout: vi.fn(), saveLoadout: vi.fn(), setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false), onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null), saveProfile: vi.fn(), login: vi.fn(), saveLastUsername: vi.fn(),
  myBattleHistory: vi.fn(), myBattleDetail: vi.fn(), request: vi.fn(), saveEquippedEquipment: vi.fn(),
}))
vi.mock('@/pages/_shared/r14AdviceAction', async (importOriginal) => {
  const orig = await importOriginal<typeof import('@/pages/_shared/r14AdviceAction')>()
  return { ...orig, fetchAdviceAction: vi.fn(), applyAdviceAction: vi.fn() }
})
const ready = {
  badge: 'AI', element: 'fire', expectedReaction: '蒸汽爆发',
  basedOn: ['近 7 天 12 场，命中/波 3.1', '/config(v=7): fire 技能池'],
  kind: 'skill', candidateId: 21, replacedId: 11,
  candidateName: '重炮弹', replacedName: '燃烧弹',
  params: { kind: 'skill', id: 21, replace: 11 }, previewRoute: '', evidence: [], impact: null,
  status: 'ready', empty: null,
} as any
const empty = {
  ...ready, status: 'empty', kind: null, candidateId: 0, replacedId: 0, params: null, impact: null,
  empty: { code: 'NO_OWNED_CANDIDATE', headline: '当前没有可执行的替换',
    message: '你还没有同元素且可替换的已拥有技能，服务端不返回候选 id。',
    cta: '去关卡页打一场 →', route: '/pages/stage/stage' },
} as any
const mockApi = vi.mocked(api, true)
const mockFetch = vi.mocked(fetchAdviceAction, true)
const mockApply = vi.mocked(applyAdviceAction, true)
const reactions = [{ key: 'steam_burst', name: '蒸汽爆发', base_coef: 60, attack_weight_pct: 300,
  status_duration_ms: 0, aoe_radius: 120, spread_stacks: 0, dispel_shield: true, amplify_pct: 0,
  armor_shred_permille: 0, knockback: 0, descr: '驱散护盾并造成溅射' }] as any
beforeEach(() => {
  installUniMock()
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified',
    config: makeConfig({ reactions }), serverTimeMs: 0 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: { skills: {}, equipment: [], mastery_nodes: [] },
    build_rating: {}, power: 0, max_stage: 1, account: {} } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 1, energy: 1, keys: 1, tokens: {} })
  mockFetch.mockResolvedValue(ready)
  mockApply.mockResolvedValue({ ok: true, error: '', doneName: '重炮弹' })
})
afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })
async function mountReactions() {
  const m = mountPage(Reactions)
  await flushPromises()
  const store = useGameStore()
  await store.loadConfig()
  await store.loadLoadout()
  await flushPromises()
  return m
}
describe('reactions.vue R14 建议一键执行', () => {
  it('每条图鉴渲染权威 CTA，不硬编码候选 id', async () => {
    const { wrapper } = await mountReactions()
    const cta = wrapper.find('#r14-advice-action-btn')
    expect(cta.exists()).toBe(true)
    expect(cta.attributes('data-r14-source')).toBe('reactions')
    const q = mockFetch.mock.calls[0]?.[0] as any
    expect(q.source).toBe('reactions')
    expect(q.element).toBeTruthy()
    expect(q).not.toHaveProperty('id')
    expect(q).not.toHaveProperty('replace')
  })
  it('空态只显示诚实说明，不渲染按钮', async () => {
    mockFetch.mockResolvedValue(empty)
    const { wrapper } = await mountReactions()
    expect(wrapper.find('#r14-advice-action-btn').exists()).toBe(false)
    expect(wrapper.find('.r14-advice-empty').exists()).toBe(true)
  })
  it('服务端 expected_reaction 为空串时图鉴卡仍渲染可执行 CTA', async () => {
    const n = normalizeAdviceAction({
      schema_version: 'advice-action-v1', element: 'fire', expected_reaction: '',
      based_on: ['近 7 天 12 场，命中/波 3.1'], kind: 'skill',
      candidate_id: 21, replaced_id: 11, candidate_name: '重炮弹', replaced_name: '燃烧弹',
      params: { kind: 'skill', id: 21, replace: 11 }, empty_reason: null,
    }, 'reactions')
    expect(n?.status).toBe('ready')
    mockFetch.mockResolvedValue(n as any)
    const { wrapper } = await mountReactions()
    expect(wrapper.find('#r14-advice-action-btn').exists()).toBe(true)
    expect(wrapper.find('.r14-advice').text()).toContain('可提升当前构筑（服务端未给出目标反应）')
  })

  it('两步确认：预览不写、确认才写，卡片保留服务端反馈', async () => {
    const { wrapper } = await mountReactions()
    await wrapper.find('#r14-advice-action-btn').trigger('click')
    await flushPromises()
    expect(wrapper.find('.r7-compare-mask').exists()).toBe(true)
    expect(mockApply).not.toHaveBeenCalled()
    const confirm = wrapper.findAll('.r7-btn.primary').find((b) => b.text().includes('确认替换'))
    expect(confirm).toBeTruthy()
    await confirm!.trigger('click')
    await flushPromises()
    expect(mockApply).toHaveBeenCalledTimes(1)
    expect(wrapper.find('.r14-advice').text()).toContain('已替换为')
  })
})