// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 98 轮：熔炼（doSynthesize）成功后的出战槽位局部同步。
 *
 * 服务端熔炼会删除两种材料技能，并清掉 user_skill_slots 里指向它们的槽位
 * （synthesis.go 第 6 步）。但 bag.vue 的 doSynthesize 成功后只 refreshProfile，
 * 没重拉 /me/loadout，也没同步页内 selected —— 出战槽仍显示已被消耗的材料技能
 * （幽灵槽位），与服务端权威态不一致。这与 R244 doApplyPreset 的失同步同族。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Bag from './bag.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeSkill } from '../../test/fixtures'
import type { GameConfig } from '@/api/client'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  synthesizeSkills: vi.fn(),
  upgradeSkill: vi.fn(),
  fetchGems: vi.fn(),
  washGem: vi.fn(),
  equipGem: vi.fn(),
  unequipGem: vi.fn(),
  fetchPresets: vi.fn(),
  saveCurrentPreset: vi.fn(),
  deletePreset: vi.fn(),
  setCurrentPreset: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  fetchMyEquipment: vi.fn(),
  saveEquippedEquipment: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

function owned(...ids: number[]): Record<string, { id: number; level: number; slot: number }> {
  return Object.fromEntries(ids.map((id) => [String(id), { id, level: 1, slot: -1 }]))
}

function bagConfig(): GameConfig {
  return makeConfig({
    skills: [
      makeSkill({ id: 1, name: '燃烧弹', element: 'fire' as any, family: 'flame' }),
      makeSkill({ id: 2, name: '冰霜弹', element: 'ice' as any, family: 'frost' }),
      makeSkill({ id: 3, name: '穿甲弹', element: 'kinetic' as any, family: 'kinetic' }),
      makeSkill({ id: 5, name: '奇异弹', element: 'bogus' as any, family: 'xyz' }),
    ],
    composite_skills: [makeSkill({ id: 101, name: '蒸汽弹', element: 'fire' as any })],
    recipes: [{ output: 101, a: 1, b: 2, out_tier: 2 }],
    equipment: [],
    gems: [],
  })
}

/** 服务端状态：熔炼前材料 1/2 在槽位；熔炼后材料被删、槽位被清、产物 101 发放。 */
let skillsOwned: Record<string, { id: number; level: number; slot: number }>
let loadoutIds: number[]

beforeEach(() => {
  installUniMock()
  skillsOwned = owned(1, 2, 3, 5)
  loadoutIds = [1, 2]
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: bagConfig(), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockImplementation(() =>
    Promise.resolve({
      user_id: 7,
      build: { skills: skillsOwned },
      build_rating: null,
      power: 9,
    } as any),
  )
  mockApi.fetchLoadout.mockImplementation(() => Promise.resolve({ skill_ids: [...loadoutIds] }))
  mockApi.fetchGems.mockResolvedValue({ gems: [] })
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] })
  mockApi.synthesizeSkills.mockImplementation((a: number, b: number) => {
    // 模拟服务端事务：删材料、清槽位、发产物
    expect([a, b]).toEqual([1, 2])
    skillsOwned = owned(3, 5, 101)
    loadoutIds = [0, 0]
    return Promise.resolve({ output: 101, build: { skills: skillsOwned } })
  })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

function slotNames(wrapper: ReturnType<typeof mountPage>['wrapper']): string[] {
  return wrapper.findAll('.slot').map((s) =>
    s.find('.slot-name').exists() ? s.find('.slot-name').text() : s.text(),
  )
}

describe('bag.vue 熔炼后出战槽同步（第 98 轮）', () => {
  it('熔炼消耗的材料在出战槽里时，成功后槽位应跟随服务端清空而不是显示幽灵技能', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    // 挂载时：服务端 loadout [1,2]，两个材料技能都在出战槽
    expect(slotNames(wrapper)).toEqual(['燃烧弹', '冰霜弹', '空槽', '空槽'])

    const row = wrapper.findAll('.recipe-row').find((r) =>
      r.text().includes('燃烧弹') && r.text().includes('冰霜弹'),
    )!
    const link = row.findAll('.link').find((l) => l.text() === '熔炼')!
    await link.trigger('click')
    await flushPromises()
    await flushPromises()

    // 写服务端成功
    expect(mockApi.synthesizeSkills).toHaveBeenCalledWith(1, 2)
    // 服务端已把两个材料的槽位清空（权威 loadout = [0,0,0,0]），
    // 页内出战槽必须跟着清空，不能继续显示已被消耗的技能
    expect(slotNames(wrapper)).toEqual(['空槽', '空槽', '空槽', '空槽'])
  })
})
