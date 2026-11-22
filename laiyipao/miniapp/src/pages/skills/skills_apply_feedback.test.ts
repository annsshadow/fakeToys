// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R10 真实缺陷：技能推荐两步确认在写入成功后立刻关闭遮罩，
 * 而成功文案只存在于遮罩内部，玩家根本看不到「已替换」。
 * 本测试要求成功态留在遮罩内直到玩家主动关闭，并给出真实的写入结果。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'

import Skills from './skills.vue'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeSkill } from '../../test/fixtures'
import { fetchRecommendations } from '@/pages/_shared/r6Recommendation'
import { fetchRecommendationImpact } from '@/pages/_shared/r7ImpactApply'

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
  request: vi.fn(),
}))

vi.mock('@/pages/_shared/r6Recommendation', async (importOriginal) => {
  const orig = await importOriginal<typeof import('@/pages/_shared/r6Recommendation')>()
  return { ...orig, fetchRecommendations: vi.fn() }
})

vi.mock('@/pages/_shared/r7ImpactApply', async (importOriginal) => {
  const orig = await importOriginal<typeof import('@/pages/_shared/r7ImpactApply')>()
  return { ...orig, fetchRecommendationImpact: vi.fn() }
})

vi.mock('@/pages/_shared/r7GameplayCausal', async (importOriginal) => {
  const orig = await importOriginal<typeof import('@/pages/_shared/r7GameplayCausal')>()
  return {
    ...orig,
    fetchLatestBattleCausal: vi.fn(async () => ({ text: '没有战报：先打一场战斗', hasHistory: false, source: 'battle_records' })),
    normalizeBattleCausal: vi.fn(() => ({ text: '没有战报：先打一场战斗', hasHistory: false, source: 'battle_records' })),
  }
})

const card = () => ({
  status: 'api' as const,
  context: 'skills',
  badge: 'AI' as const,
  headline: 'AI 推荐使用重炮弹',
  engineLabel: '统计模型 · stats rec-v1',
  evidence: [],
  basis: ['依据：battle_records'],
  cta: { label: '应用推荐', route: '', params: { id: 4, replace: 1 } },
})

const impact = () => ({
  kind: 'skill' as const,
  candidateId: 4,
  replacedId: 1,
  candidateName: '重炮弹',
  replacedName: '燃烧弹',
  baseline: { attackPermille: 100, armorPermille: 0, estimatedDamagePerWave: 1200, power: 0, ratingTotal: 100 },
  candidate: { attackPermille: 150, armorPermille: 0, estimatedDamagePerWave: 1800, power: 10, ratingTotal: 120 },
  changes: {
    attackPermilleDelta: 50,
    armorPermilleDelta: 0,
    damagePerWaveDelta: 600,
    damagePerWavePct: 50,
    powerDelta: 10,
    ratingTotalDelta: 20,
    starRateObserved: null,
    starProbabilityEstimate: null,
  },
  method: '按真实战斗每波伤害',
  starRateNote: '星级概率无可信模型，未做推测',
  evidence: [],
  basis: ['依据：battle_records'],
  configVersion: 'fixture0000000001',
  battleCount: 3,
  windowDays: 7,
  generatedAt: '',
})

beforeEach(() => {
  installUniMock()
  vi.clearAllMocks()
})
afterEach(() => { vi.unstubAllGlobals() })

describe('R10 技能推荐应用反馈', () => {
  it('确认写入后成功提示仍可见，遮罩等到玩家主动关闭才消失', async () => {
    const cfg = makeConfig({
      skills: [makeSkill({ id: 1, name: '燃烧弹' }), makeSkill({ id: 4, name: '重炮弹', code: 'heavy' })],
    })
    vi.mocked(fetchRecommendations).mockResolvedValue(card())
    vi.mocked(fetchRecommendationImpact).mockResolvedValue({ card: card(), impact: impact(), error: '' })
    const { wrapper, store } = mountPage(Skills, (s) => {
      s.loggedIn = true
      s.config = cfg
      s.loadout = [1, 2, 3, 0]
    })
    const load = vi.spyOn(store, 'loadLoadout').mockResolvedValue(undefined as never)
    const persist = vi.spyOn(store, 'persistLoadout').mockResolvedValue(true)

    await flushPromises()
    await wrapper.find('.r7-impact .r7-btn.primary').trigger('click')
    await flushPromises()
    expect(wrapper.find('.r7-compare-mask').exists()).toBe(true)

    await wrapper.find('.r7-compare .r7-btn.primary').trigger('click')
    await flushPromises()

    expect(load).toHaveBeenCalled()
    expect(persist).toHaveBeenCalledWith([4, 2, 3, 0])
    expect(wrapper.find('.r7-compare-mask').exists()).toBe(true)
    expect(wrapper.find('.apply-done').exists()).toBe(true)
    expect(wrapper.find('.apply-done').text()).toContain('已替换为「重炮弹」')
    expect(wrapper.find('.r7-compare .r7-btn.primary').text()).toContain('完成')
  })
})
