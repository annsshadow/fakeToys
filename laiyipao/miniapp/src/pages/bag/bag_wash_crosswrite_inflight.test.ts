// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 138 轮 laiyipao 真 bug（R287）：洗练 doWash 漏接 presetWriteBusy 共享写守卫。
 *
 * R276/R277 给写同一批 user_gems 资源的单入口（toggleEquip/clearSlot/equip/
 * unequip/doEquipGem/doUnequipGem）都接了 presetWriteBusy 共享守卫，但 doWash
 * 洗练同样写 user_gems.affixes，并用局部 ownedGems 旧快照 splice 就地回填。
 * 应用预设在途时点洗练（或反向：洗练在途时应用预设），会用旧快照交叉覆盖
 * 宝石词条/佩戴状态，玩家看到「应用了预设但宝石词条/佩戴不对」。
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
    ],
  }
}

const gem = (over: Record<string, unknown> = {}) => ({
  id: 7, gem_id: 3, code: 'gem_crit', name: '暴击石', descr: '',
  quality: 'gold', affixes: [{ affix: 'crit', value: 100 }], equipped_slot: '',
  ...over,
})

const preset = (over: Record<string, unknown> = {}) => ({
  id: 1,
  name: '套A',
  slots: { skills: [1], equipment: [1], gems: [7] },
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
let releaseWash: ((v: unknown) => void) | null = null

beforeEach(() => {
  installUniMock()
  releaseLoadout = null
  releaseWash = null
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: bagConfig(), serverTimeMs: 0 } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: { gem_wash_token: 1 } } as any)
  mockApi.fetchMe.mockResolvedValue({
    user_id: 7, build: { skills: owned(1, 2, 3) }, build_rating: null, power: 9,
  } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] } as any)
  mockApi.fetchGems.mockResolvedValue({ gems: [gem()] } as any)
  mockApi.fetchMyEquipment.mockResolvedValue(equipmentFixture() as any)
  mockApi.fetchPresets.mockResolvedValue({ presets: [preset()] } as any)
  mockApi.saveEquippedEquipment.mockResolvedValue({ items: [] } as any)
  mockApi.equipGem.mockResolvedValue({} as any)
  mockApi.unequipGem.mockResolvedValue({} as any)
  mockApi.washGem.mockResolvedValue({ gem: gem({ affixes: [{ affix: 'crit', value: 155 }] }) } as any)
  // 首个 saveLoadout 挂起，构造「应用预设在途」窗口；后续调用直返，
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
  // 首个 washGem 挂起，构造「洗练在途」窗口。
  let washCalls = 0
  mockApi.washGem.mockImplementation((((id: number) => {
    washCalls += 1
    if (washCalls === 1) {
      const d = deferred<any>()
      releaseWash = d.resolve
      return d.promise
    }
    return Promise.resolve({ gem: gem({ id, affixes: [{ affix: 'crit', value: 155 }] }) })
  })) as any)
})

afterEach(async () => {
  releaseLoadout?.({ skill_ids: [1] })
  releaseWash?.({ gem: gem() })
  await flushPromises()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('bag.vue 洗练写共享守卫（R287）', () => {
  vi.setConfig({ testTimeout: 15000 })

  it('应用预设在途时点洗练不会交叉写宝石', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)
    expect(releaseLoadout).not.toBeNull()

    // 在途期间点洗练 gem 7：doWash 应被共享守卫挡住，不发出 washGem。
    // 用 void 触发，避免基线缺陷下 doWash 卡在挂起的 washGem 上导致超时，
    // 从而给出明确的断言失败而非超时。
    void (wrapper.vm as any).doWash(gem())
    await flushPromises()
    expect(mockApi.washGem).not.toHaveBeenCalled()

    releaseLoadout?.({ skill_ids: [1] })
    await flushPromises()
  })

  it('洗练在途时点应用预设不会交叉写宝石', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    // 先点洗练 gem 7，挂起在 washGem（洗练在途）。
    // doWash 内部 await 这个挂起的 promise，不能 await 它本身，
    // 用 void 触发即可。
    void (wrapper.vm as any).doWash(gem())
    await flushPromises()
    expect(mockApi.washGem).toHaveBeenCalledTimes(1)
    expect(releaseWash).not.toBeNull()

    // 在途期间点应用预设：doApplyPreset 应被共享守卫挡住，不发 saveLoadout
    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).not.toHaveBeenCalled()

    releaseWash?.({ gem: gem() })
    await flushPromises()
  })
})
