// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R10 真实缺陷：装备推荐确认写入成功后立刻关闭遮罩，
 * 成功文案只在遮罩内，玩家看不到装备已替换。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'

import Equipment from './equipment.vue'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'
import { fetchRecommendations } from '@/pages/_shared/r6Recommendation'
import { fetchRecommendationImpact, applyEquipmentImpact } from '@/pages/_shared/r7ImpactApply'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  saveEquippedEquipment: vi.fn(async () => true),
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
  return { ...orig, fetchRecommendationImpact: vi.fn(), applyEquipmentImpact: vi.fn(async () => true) }
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
  context: 'equipment',
  badge: 'AI' as const,
  headline: 'AI 推荐使用母核碎片',
  engineLabel: '统计模型 · stats rec-v1',
  evidence: [],
  basis: ['依据：battle_records'],
  cta: { label: '应用推荐', route: '', params: { id: 4, replace: 1 } },
})

const impact = () => ({
  kind: 'equipment' as const,
  candidateId: 4,
  replacedId: 1,
  candidateName: '母核碎片',
  replacedName: '制式炮管',
  baseline: { attackPermille: 100, armorPermille: 0, estimatedDamagePerWave: 1200, power: 0, ratingTotal: 100 },
  candidate: { attackPermille: 150, armorPermille: 20, estimatedDamagePerWave: 1800, power: 10, ratingTotal: 120 },
  changes: {
    attackPermilleDelta: 50,
    armorPermilleDelta: 20,
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

describe('R10 装备推荐应用反馈', () => {
  it('确认写入后成功提示仍可见，遮罩等到玩家主动关闭才消失', async () => {
    const cfg = makeConfig({ equipment: [
      { id: 1, name: '制式炮管', element: 'kinetic', descr: '', slot: 'weapon', tier: 1 },
      { id: 4, name: '母核碎片', element: 'fire', descr: '', slot: 'weapon', tier: 3 },
    ] })
    vi.mocked(fetchRecommendations).mockResolvedValue(card())
    vi.mocked(fetchRecommendationImpact).mockResolvedValue({ card: card(), impact: impact(), error: '' })
    const { wrapper, store } = mountPage(Equipment, (s) => {
      s.loggedIn = true
      s.config = cfg
      s.equipment = [
        { id: 1, name: '制式炮管', slot: 'weapon', equipped: true } as never,
        { id: 4, name: '母核碎片', slot: 'weapon', equipped: false } as never,
      ]
    })
    const load = vi.spyOn(store, 'loadEquipment').mockResolvedValue(undefined as never)

    await flushPromises()
    await wrapper.find('.r7-impact .r7-btn.primary').trigger('click')
    await flushPromises()
    expect(wrapper.find('.r7-compare-mask').exists()).toBe(true)

    await wrapper.find('.r7-compare .r7-btn.primary').trigger('click')
    await flushPromises()

    expect(load).toHaveBeenCalled()
    expect(applyEquipmentImpact).toHaveBeenCalledWith(expect.any(Array), 4, 1, 'weapon')
    expect(wrapper.find('.r7-compare-mask').exists()).toBe(true)
    expect(wrapper.find('.apply-done').exists()).toBe(true)
    expect(wrapper.find('.apply-done').text()).toContain('已替换为「母核碎片」')
    expect(wrapper.find('.r7-compare .r7-btn.primary').text()).toContain('完成')
  })
})
