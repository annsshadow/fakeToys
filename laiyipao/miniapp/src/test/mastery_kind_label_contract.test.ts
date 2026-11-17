// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 行为测试：专精「收益汇总」 chip 必须显示服务端契约里的中文属性名，
 * 而不是把 kind 的英文 key 原文漏给玩家。
 *
 * 服务端权威（laiyipao/server/internal/domain/power.go:62-79）：
 *   masteryLayerKinds 用到 element_cap / reaction_mult / heat_cap /
 *   skill_damage / crit / extra_slot / mechanic / armor；
 *   masteryKindNames 给每个 kind 配了中文名。
 * 配置接口下发的节点 kind 就是这些 key（GameConfig.mastery_families），
 * 所以前端汇总 chip 必须按这些 key 映射到中文名。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Mastery from '../pages/mastery/mastery.vue'
import * as api from '@/api/client'
import { installUniMock } from './uni-mock'
import { mountPage } from './page'
import { makeConfig } from './fixtures'
import type { MasteryFamily } from '@/game/types'

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
  fetchMastery: vi.fn(),
  allocateMastery: vi.fn(),
  resetMastery: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

/** 8 个服务端权威 kind，各给一个可辨认的 value。 */
function contractFamily(): MasteryFamily {
  const kinds = [
    'element_cap', 'reaction_mult', 'heat_cap', 'skill_damage',
    'crit', 'extra_slot', 'mechanic', 'armor',
  ]
  return {
    family: 'flame',
    name: '烈焰',
    nodes: kinds.map((kind, i) => ({
      id: 11 + i,
      family: 'flame',
      layer: 1,
      slot: i,
      name: kind,
      kind,
      value: (i + 1) * 10,
      prereq_family: '',
      prereq_layer: 0,
    })),
  }
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified', config: makeConfig({ mastery_families: [contractFamily()] }), serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 8, nodes: [11, 12, 13, 14, 15, 16, 17, 18] })
  mockApi.allocateMastery.mockResolvedValue({ points: 7, nodes: [11] })
  mockApi.resetMastery.mockResolvedValue({ reset_cleared: 8, points: 8, nodes: [], layer_limit: 2, base_slots: 4, total_slots: 4 })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('mastery.vue 收益汇总 · 服务端 kind 中文名', () => {
  it('已点亮节点按服务端 kind 显示中文属性名，不漏英文 key', async () => {
    const { wrapper } = mountPage(Mastery)
    await flushPromises()

    const text = wrapper.text()
    const labels = [
      '元素上限 +10', '反应倍率 +20', '热量上限 +30', '技能伤害 +40',
      '暴击率 +50', '额外插槽 +60', '机制改造 +70', '防线护甲 +80',
    ]
    for (const label of labels) {
      expect(text, '收益汇总应显示中文属性名：' + label).toContain(label)
    }
    // 关键回归：收益汇总 chip 绝不能把服务端 kind 的英文 key 原文显示给玩家。
    // 断言范围限定在 iv-chip：节点卡片的 name 用契约 key 是可接受的（那是 fixture 的简化），
    // 但汇总 chip 必须有中文 label。
    const chipText = wrapper.findAll('.iv-chip').map((c) => c.text()).join(' | ')
    expect(chipText, '收益汇总 chip 应非空').not.toBe('')
    for (const key of ['element_cap', 'reaction_mult', 'heat_cap', 'skill_damage', 'extra_slot']) {
      expect(chipText, '收益汇总 chip 不应出现英文 key 原文：' + key).not.toContain(key)
    }
  })
})
