// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 102 轮：穿戴/卸下装备后的 store 写后失同步。
 *
 * equip()/unequip() 只把 saveEquippedEquipment 回带的 items 写回页内
 * myEquipments，没有刷新 Pinia store。而：
 *   - store.build / power / attacker 来自 /me（装备的 base_armor/base_bonus_pct
 *     会被服务端 attackerFromParts 读进攻方），
 *   - store.equipment / equippedEquipmentIds 来自 /me/equipment。
 * 于是本会话内穿戴/卸下后，战力面板与已装备 id 仍是旧值 ——
 * 同页的宝石/升级/熔炼/应用预设写后都调了 refreshProfile，唯独这两处漏了。
 *
 * 判据用可观察行为：写服务端成功后，store.power 必须跟到服务端新值，
 * store.equippedEquipmentIds 必须跟到新的已装备集合。业务逻辑不变则失败。
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

/** /me/equipment 视图：按「当前已装备 id 集合」回显，初始只穿武器(1)。 */
function equipmentFixture(equippedIds: number[] = [1]) {
  const defs = [
    { id: 1, code: 'w_p1', name: '制式炮管', slot: 'weapon', tier: 1, element: 'kinetic',
      descr: '', base_armor: 0, base_bonus_pct: 50, unlock_level: 1, level: 1, star: 1 },
    { id: 7, code: 'a_p1', name: '拼接胸甲', slot: 'armor', tier: 1, element: 'kinetic',
      descr: '', base_armor: 35, base_bonus_pct: 25, unlock_level: 6, level: 1, star: 1 },
  ]
  return { items: defs.map((d) => ({ ...d, owned: true, equipped: equippedIds.includes(d.id) })) }
}

function owned(...ids: number[]): Record<string, { id: number; level: number; slot: number }> {
  return Object.fromEntries(ids.map((id) => [String(id), { id, level: 1, slot: -1 }]))
}

function attackerRaw(attack: number) {
  return {
    attack, crit_permille: 0, crit_multiplier_permille: 1500,
    reaction_mult_permille: 1000, element_cap: 10, reaction_tier: 1,
    element_coef_permille: 0, heat_cap_permille: 0, armor_permille: 0,
    mechanic_permille: 0,
  }
}

function bagConfig(): GameConfig {
  return makeConfig({
    skills: [
      makeSkill({ id: 1, name: '燃烧弹', element: 'fire' as any, family: 'flame' }),
      makeSkill({ id: 2, name: '冰霜弹', element: 'ice' as any, family: 'frost' }),
    ],
    composite_skills: [], equipment: [], gems: [],
  })
}

/** 服务端当前已装备集合：写前为挂载态 [1]，写后回显最后一次提交。 */
function serverEquipped(): number[] {
  const calls = mockApi.saveEquippedEquipment.mock.calls
  return calls.length ? (calls[calls.length - 1]![0] as number[]) : [1]
}

/** 挂载时战力 9；写发生后服务端据新装备重算战力。 */
function serverPower(afterPower: number): number {
  return mockApi.saveEquippedEquipment.mock.calls.length ? afterPower : 9
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: bagConfig(), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] })
  mockApi.fetchGems.mockResolvedValue({ gems: [] })
  mockApi.fetchMyEquipment.mockImplementation(() => Promise.resolve(equipmentFixture(serverEquipped())))
  mockApi.saveEquippedEquipment.mockImplementation((ids: number[]) =>
    Promise.resolve({ items: equipmentFixture(ids).items }),
  )
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('bag.vue 穿戴/卸下写后同步（第 102 轮）', () => {
  it('穿戴后 store 战力与已装备 id 跟随服务端刷新', async () => {
    mockApi.fetchMe.mockImplementation(() => Promise.resolve({
      user_id: 7,
      build: { skills: owned(1, 2), attacker: attackerRaw(serverPower(42)) },
      build_rating: null,
      power: serverPower(42),
    } as any))

    const { wrapper, store } = mountPage(Bag)
    await flushPromises()
    expect(store.power).toBe(9)

    const armorRow = wrapper.findAll('.equip-row').find((r) => r.text().includes('拼接胸甲'))!
    await armorRow.findAll('.btn').find((b) => b.text() === '装备')!.trigger('click')
    await flushPromises()
    await flushPromises()

    expect(mockApi.saveEquippedEquipment).toHaveBeenCalled()
    expect(store.power).toBe(42)
    expect(store.equippedEquipmentIds).toEqual([1, 7])
  })

  it('卸下后 store 战力与已装备 id 跟随服务端刷新', async () => {
    mockApi.fetchMe.mockImplementation(() => Promise.resolve({
      user_id: 7,
      build: { skills: owned(1, 2), attacker: attackerRaw(serverPower(0)) },
      build_rating: null,
      power: serverPower(0),
    } as any))

    const { wrapper, store } = mountPage(Bag)
    await flushPromises()
    expect(store.power).toBe(9)

    const weaponRow = wrapper.findAll('.equip-row').find((r) => r.text().includes('制式炮管'))!
    await weaponRow.findAll('.btn').find((b) => b.text() === '卸下')!.trigger('click')
    await flushPromises()
    await flushPromises()

    expect(mockApi.saveEquippedEquipment).toHaveBeenCalled()
    expect(store.power).toBe(0)
    expect(store.equippedEquipmentIds).toEqual([])
  })
})
