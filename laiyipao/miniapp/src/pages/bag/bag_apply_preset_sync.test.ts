// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 94 轮：应用阵型预设后的页内局部状态同步。
 *
 * doApplyPreset 把技能槽/装备/宝石都写到了服务端，但页面自己的两份局部状态
 * 没有跟着走：
 *   1. selected（出战槽 UI 与热量预算的数据源）—— persistLoadout 成功后
 *      store.loadout 已是新值，selected 却还停在挂载时的旧快照；
 *      同页的 toggleEquip/clearSlot 成功后都有 selected.value = [...store.loadout]。
 *   2. myEquipments（装备行的已装备标记 / 穿戴贡献 / 换装建议的数据源）——
 *      saveEquippedEquipment 的响应带着整组新 items，被直接丢弃；
 *      同页的 equip/unequip 都写回 myEquipments.value = res.items。
 * bag.vue 只有 onMounted 没有 onShow，页面存活期间不会自动重拉 ——
 * 玩家应用预设后看到的是旧槽位与旧装备状态，与服务端事实不一致。
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

/** /me/equipment 响应：id1 武器已装配，id7 胸甲已拥有未装配（与 bag.test.ts 同形）。 */
function equipmentFixture() {
  return {
    items: [
      {
        id: 1, code: 'w_p1', name: '制式炮管', slot: 'weapon', tier: 1, element: 'kinetic',
        descr: '军方量产的基础炮管。攻击力 +50‰。',
        base_armor: 0, base_bonus_pct: 50, unlock_level: 1,
        owned: true, equipped: true, level: 1, star: 1,
      },
      {
        id: 7, code: 'a_p1', name: '拼接胸甲', slot: 'armor', tier: 1, element: 'kinetic',
        descr: '用废旧钢板拼成，聊胜于无。护甲 +35‰，攻击力 +25‰。',
        base_armor: 35, base_bonus_pct: 25, unlock_level: 6,
        owned: true, equipped: false, level: 1, star: 1,
      },
    ],
  }
}

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
    composite_skills: [],
    equipment: [],
    gems: [],
  })
}

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: bagConfig(), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({
    user_id: 7,
    build: { skills: owned(1, 2, 3, 5) },
    build_rating: null,
    power: 9,
  } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] })
  mockApi.fetchGems.mockResolvedValue({ gems: [] })
  mockApi.fetchMyEquipment.mockResolvedValue(equipmentFixture())
  // 服务端整组替换后回带新 items（真实契约：PUT /me/equipment → {items}）
  mockApi.saveEquippedEquipment.mockImplementation((ids: number[]) =>
    Promise.resolve({
      items: equipmentFixture().items.map((e) => ({ ...e, equipped: ids.includes(e.id) })),
    }),
  )
  mockApi.saveLoadout.mockImplementation((ids: number[]) => Promise.resolve({ skill_ids: ids }))
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('bag.vue 应用预设局部同步（第 94 轮）', () => {
  it('应用预设后出战槽显示预设技能，而不是挂载时的旧快照', async () => {
    mockApi.fetchPresets.mockResolvedValue({
      presets: [{
        id: 9, name: '进攻套', is_current: true, created_at: 'x',
        slots: { skills: [3, 5], equipment: [1], gems: [] },
      }],
    })
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    // 挂载时槽位来自 fetchLoadout [1,2]（补 0 到 4 槽）
    const namesBefore = wrapper.findAll('.slot').map((s) => s.find('.slot-name').exists() ? s.find('.slot-name').text() : s.text())
    expect(namesBefore).toEqual(['燃烧弹', '冰霜弹', '空槽', '空槽'])

    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    await flushPromises()

    // 写服务端成功（流程本身没走失败分支）
    expect(mockApi.saveLoadout).toHaveBeenCalledWith([3, 5, 0, 0])
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '已应用预设', icon: 'none' })

    // 槽位 UI 必须跟上服务端已保存的新槽位
    const namesAfter = wrapper.findAll('.slot').map((s) => s.find('.slot-name').exists() ? s.find('.slot-name').text() : s.text())
    expect(namesAfter).toEqual(['穿甲弹', '奇异弹', '空槽', '空槽'])
  })

  it('应用预设后装备行与穿戴贡献按服务端回带的 items 更新', async () => {
    mockApi.fetchPresets.mockResolvedValue({
      presets: [{
        id: 9, name: '防御套', is_current: true, created_at: 'x',
        // 技能槽不变，只换装备：制式炮管(1) → 拼接胸甲(7)
        slots: { skills: [1, 2], equipment: [7], gems: [] },
      }],
    })
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    const rowOf = (name: string) => wrapper.findAll('.equip-row').find((r) => r.text().includes(name))!
    expect(rowOf('制式炮管').text()).toContain('已装备')
    expect(rowOf('拼接胸甲').text()).not.toContain('已装备')
    expect(wrapper.find('.equip-contrib').text()).toContain('护甲 +0‰ · 攻击 +50‰')

    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    await flushPromises()

    expect(mockApi.saveEquippedEquipment).toHaveBeenCalledWith([7])
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '已应用预设', icon: 'none' })

    // 服务端已整组替换：炮管卸下、胸甲穿上 —— 页内标记与贡献必须一致
    expect(rowOf('拼接胸甲').text()).toContain('已装备')
    expect(rowOf('制式炮管').text()).not.toContain('已装备')
    expect(wrapper.find('.equip-contrib').text()).toContain('护甲 +35‰ · 攻击 +25‰')
  })
})
