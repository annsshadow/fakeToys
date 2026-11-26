// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/** R14 §C.1/C.3：战报建议必须来自服务端权威 CTA；空态不渲染按钮；确认前不写。 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Battles from './battles.vue'
import * as api from '@/api/client'
import { fetchAdviceAction, applyAdviceAction } from '@/pages/_shared/r14AdviceAction'
import { installUniMock } from '../../test/uni-mock'
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
const detail = { id: 1, level_id: 4, result: 'win', stars: 3, score: 1200, kills: 20, leaked: 0,
  reactions: 5, replay_hash: 'hash-abc', created_at: '2026-11-17T10:00:00Z',
  elements_used: { fire: 4, ice: 2 }, reactions_used: { steam_burst: 3 } } as any
beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({ access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 } } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: {} as any, serverTimeMs: 0 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [] })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.myBattleHistory.mockResolvedValue({ items: [{ id: 1, result: 'win', stars: 3, level_id: 4,
    score: 1200, kills: 20, leaked: 0, created_at: '2026-11-17T10:00:00Z' }] } as any)
  mockApi.myBattleDetail.mockResolvedValue(detail)
  mockFetch.mockResolvedValue(ready)
  mockApply.mockResolvedValue({ ok: true, error: '', doneName: '重炮弹' })
})
afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })
async function mountAndExpand() {
  const m = mountPage(Battles)
  await flushPromises()
  await m.wrapper.find('.battle-row').trigger('click')
  await flushPromises()
  return m
}
describe('battles.vue R14 建议一键执行', () => {
  it('保留原建议文案并在其下渲染权威 CTA；只回传战报元素，不拼 id', async () => {
    const { wrapper } = await mountAndExpand()
    expect(wrapper.find('.reaction-learning-label').text()).toBe('AI · 反应搭配建议')
    const cta = wrapper.find('#r14-advice-action-btn')
    expect(cta.exists()).toBe(true)
    expect(cta.attributes('data-r14-source')).toBe('battle')
    expect(cta.text()).toContain('重炮弹')
    const q = mockFetch.mock.calls[0]?.[0] as any
    expect(q.source).toBe('battle')
    expect(q.element).toBe('fire')
    expect(q).not.toHaveProperty('id')
    expect(q).not.toHaveProperty('replace')
  })
  it('空态不渲染按钮，只给诚实说明与下一步', async () => {
    mockFetch.mockResolvedValue(empty)
    const { wrapper } = await mountAndExpand()
    expect(wrapper.find('#r14-advice-action-btn').exists()).toBe(false)
    expect(wrapper.find('.r14-advice-empty').exists()).toBe(true)
    expect(wrapper.text()).toContain('当前没有可执行的替换')
  })
  it('首次点击只预览；确认后才写入，成功后卡片保留配置反馈', async () => {
    const { wrapper } = await mountAndExpand()
    await wrapper.find('#r14-advice-action-btn').trigger('click')
    await flushPromises()
    expect(wrapper.find('.r7-compare-mask').exists()).toBe(true)
    expect(wrapper.find('.r7-compare-title').text()).toContain('当前 vs 推荐')
    expect(mockApply).not.toHaveBeenCalled()
    const confirm = wrapper.findAll('.r7-btn.primary').find((b) => b.text().includes('确认替换'))
    expect(confirm).toBeTruthy()
    await confirm!.trigger('click')
    await flushPromises()
    expect(mockApply).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('已替换为')
    // 列表刷新会折叠详情（既有行为）；重新展开后卡片仍须保留服务端写入反馈
    if (!wrapper.find('.r14-advice').exists()) {
      await wrapper.find('.battle-row').trigger('click')
      await flushPromises()
    }
    expect(wrapper.find('.r14-advice').text()).toContain('已替换为')
  })
})