// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 127 轮 laiyipao 真 bug（R277）：bag.vue 穿戴写共享守卫缺口。
 *
 * R276 给四个「预设入口」加了 presetWriteBusy 共享守卫，但 doApplyPreset
 * 的整组写（saveLoadout → saveEquippedEquipment → equipGem/unequipGem）期间，
 * 技能槽 / 装备 / 宝石的**单入口**并不检查这个守卫：
 *   - toggleEquip / clearSlot → store.persistLoadout(saveLoadout)
 *   - equip / unequip         → api.saveEquippedEquipment
 *   - doEquipGem / doUnequipGem → api.equipGem / unequipGem
 * 应用预设在途时点这些入口，会用局部旧快照写同一资源，把预设已写好的部分覆盖掉，
 * 玩家看到「应用了预设但槽位/装备/宝石不对」。反向（单入口在途时应用预设）同理。
 * 本测试把首个写挂起构造在途窗口，交叉点击另一入口，断言不会发出第二次写。
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

function owned(...ids: number[]): Record<string, { id: number; level: number; slot: number }> {
  return Object.fromEntries(ids.map((id) => [String(id), { id, level: 1, slot: -1 }]))
}

function bagConfig(): GameConfig {
  return makeConfig({
    skills: [
      makeSkill({ id: 1, name: '燃烧弹', element: 'fire' as any, family: 'flame' }),
      makeSkill({ id: 2, name: '冰霜弹', element: 'ice' as any, family: 'frost' }),
      makeSkill({ id: 3, name: '穿甲弹', element: 'kinetic' as any, family: 'kinetic' }),
    ],
    composite_skills: [],
    equipment: [],
    gems: [],
  })
}

function equipmentFixture() {
  return {
    items: [
      { id: 1, code: 'w', name: '制式炮管', slot: 'weapon', tier: 1, element: 'kinetic', descr: '', base_armor: 0, base_bonus_pct: 50, unlock_level: 1, owned: true, equipped: true, level: 1, star: 1 },
      { id: 7, code: 'a', name: '拼接胸甲', slot: 'armor', tier: 1, element: 'kinetic', descr: '', base_armor: 35, base_bonus_pct: 25, unlock_level: 1, owned: true, equipped: false, level: 1, star: 1 },
    ],
  }
}

const preset = (over: Record<string, unknown> = {}) => ({
  id: 1,
  name: '套A',
  slots: { skills: [1], equipment: [1], gems: [99] },
  is_current: true,
  created_at: '2026-01-01T00:00:00Z',
  ...over,
})

function deferred<T>() {
  let resolve!: (v: T) => void
  const promise = new Promise<T>((res) => { resolve = res })
  return { promise, resolve }
}

let releaseLoadout: ((v: unknown) => void) | null = null

beforeEach(() => {
  installUniMock()
  releaseLoadout = null
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: bagConfig(), serverTimeMs: 0 } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({
    user_id: 7, build: { skills: owned(1, 2, 3) }, build_rating: null, power: 9,
  } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] } as any)
  mockApi.fetchGems.mockResolvedValue({ gems: [] } as any)
  mockApi.fetchMyEquipment.mockResolvedValue(equipmentFixture() as any)
  mockApi.fetchPresets.mockResolvedValue({ presets: [preset()] } as any)
  mockApi.saveEquippedEquipment.mockResolvedValue({ items: [] } as any)
  mockApi.equipGem.mockResolvedValue({} as any)
  mockApi.unequipGem.mockResolvedValue({} as any)
  mockApi.washGem.mockResolvedValue({} as any)
  // 首个 saveLoadout 挂起，构造「写技能槽在途」窗口；后续调用直返，
  // 这样即使 bug 存在（交叉写发生）也能给出明确断言失败而非超时。
  let loadoutCalls = 0
  mockApi.saveLoadout.mockImplementation((((ids: number[]) => {
    loadoutCalls += 1
    if (loadoutCalls === 1) {
      const d = deferred<any>()
      releaseLoadout = d.resolve
      return d.promise
    }
    return Promise.resolve({ skill_ids: ids })
  })) as any)
})

afterEach(async () => {
  releaseLoadout?.({ skill_ids: [1] })
  await flushPromises()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('bag.vue 穿戴写共享守卫（R277）', () => {
  vi.setConfig({ testTimeout: 15000 })
  it('应用预设在途时点技能槽装备不会交叉写', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)
    expect(releaseLoadout).not.toBeNull()

    // 在途期间点技能 3 的「装备」（走 toggleEquip → saveLoadout）：应被共享守卫挡住
    await (wrapper.vm as any).toggleEquip(3)
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)

    releaseLoadout?.({ skill_ids: [1] })
    await flushPromises()
  })

  it('应用预设在途时点装备穿戴不会交叉写', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)
    // 应用预设卡在第一步 saveLoadout，还没走到 saveEquippedEquipment
    expect(mockApi.saveEquippedEquipment).not.toHaveBeenCalled()

    // 在途期间点装备 7 的「穿戴」：应被共享守卫挡住
    await (wrapper.vm as any).equip(7)
    await flushPromises()
    expect(mockApi.saveEquippedEquipment).not.toHaveBeenCalled()

    releaseLoadout?.({ skill_ids: [1] })
    await flushPromises()
  })

  it('应用预设在途时点宝石佩戴不会交叉写', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)
    expect(mockApi.equipGem).not.toHaveBeenCalled()

    // 在途期间点佩戴一颗宝石：应被共享守卫挡住
    await (wrapper.vm as any).doEquipGem({ id: 99, equipped_slot: 0 })
    await flushPromises()
    expect(mockApi.equipGem).not.toHaveBeenCalled()

    releaseLoadout?.({ skill_ids: [1] })
    await flushPromises()
  })

  it('单入口在途时点应用预设不会交叉写技能槽', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    // 先点技能 3 装备，挂起在 saveLoadout（单入口在途）。
    // toggleEquip 内部 await 这个挂起的 promise，不能 await 它本身，
    // 否则测试会停在挂起窗口直到超时；用 void 触发即可。
    void (wrapper.vm as any).toggleEquip(3)
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)
    expect(releaseLoadout).not.toBeNull()

    // 在途期间点应用预设：应被共享守卫挡住，不再写 saveLoadout
    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)

    releaseLoadout?.({ skill_ids: [1] })
    await flushPromises()
  })
})
